use crate::{
    core::{
        error::AppError,
        presence,
        state::{SessionId, SharedState},
    },
    features::ws::services::identity::{handle_connect, handle_disconnect},
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use entity::user;
use futures::{SinkExt, StreamExt};
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use shared::{
    data::{ChannelId, GuildId, UserId},
    ws::{ClientMessage, ServerMessage, rtc::RtcClientEvents},
};
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| {
        if let Err(e) = socket_handler(socket, state).await {
            tracing::error!("WebSocket dropped: {e}");
        }
    })
}

pub async fn socket_handler(socket: WebSocket, state: SharedState) -> Result<(), AppError> {
    let session_id = SessionId(Uuid::new_v4());

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let current_user_id = loop {
        match ws_receiver.next().await {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(ClientMessage::Identify { token }) => {
                    let mut conn = state.redis.sessions.get().await?;
                    let redis_key = format!("session:{token}");

                    let user_id_result: Option<String> = conn.get(&redis_key).await.unwrap_or(None);

                    match user_id_result {
                        Some(raw_id) => {
                            let user_id = UserId(Uuid::parse_str(&raw_id)?);
                            handle_connect(&state, user_id, session_id, tx.clone()).await?;
                            break user_id;
                        }
                        None => {
                            send_error(&tx, "Invalid or expired session token");
                            send_task.abort();
                            return Ok(());
                        }
                    }
                }
                _ => {
                    send_error(&tx, "Must Identify first");
                }
            },
            Some(Ok(Message::Ping(data))) => {
                let _ = tx.send(Message::Pong(data));
            }
            _ => {
                send_task.abort();
                return Ok(());
            }
        }
    };

    let mut current_voice_channel: Option<ChannelId> = None;

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg {
                        ClientMessage::SetPresence { presence } => {
                            if let Err(e) = crate::core::presence::set_presence(
                                &state.redis.presence,
                                &current_user_id.0,
                                &presence,
                            )
                            .await
                            {
                                tracing::error!("Failed to update presence: {}", e);
                                continue;
                            }

                            if let Ok(Some(user_model)) =
                                user::Entity::find_by_id(current_user_id.0)
                                    .one(&state.db)
                                    .await
                            {
                                let public_user = shared::data::user::UserPublic {
                                    id: current_user_id,
                                    profile: crate::core::mappers::FromDomain::from_domain(
                                        user_model,
                                    ),
                                    presence,
                                };

                                let event = ServerMessage::User(
                                    shared::ws::user::UserServerEvents::PresenceUpdate {
                                        user: Box::new(public_user),
                                    },
                                );

                                if let Ok(guild_ids) = entity::guild_member::Entity::find()
                                    .select_only()
                                    .column(entity::guild_member::Column::GuildId)
                                    .filter(
                                        entity::guild_member::Column::UserId.eq(current_user_id.0),
                                    )
                                    .into_tuple::<uuid::Uuid>()
                                    .all(&state.db)
                                    .await
                                {
                                    for g_id in guild_ids {
                                        let _ = crate::core::broadcast::to_guild(
                                            &state.redis.messages,
                                            &shared::data::GuildId(g_id),
                                            &event,
                                        )
                                        .await;
                                    }
                                }

                                if let Ok(friend_ids) =
                                    crate::core::cache::friends::get_cached_friends(
                                        &state,
                                        current_user_id,
                                    )
                                    .await
                                {
                                    let _ = crate::core::broadcast::to_friends(
                                        &state.redis.messages,
                                        &friend_ids,
                                        &event,
                                    )
                                    .await;
                                }
                            }
                        }
                        ClientMessage::Rtc(rtc_event) => match rtc_event {
                            RtcClientEvents::JoinVoice { channel_id } => {
                                current_voice_channel = Some(channel_id);

                                let mut conn = state.redis.presence.get().await.unwrap();
                                let _: () = deadpool_redis::redis::cmd("SADD")
                                    .arg(format!("voice_channel:{}", channel_id.0))
                                    .arg(current_user_id.0.to_string())
                                    .query_async(&mut conn)
                                    .await
                                    .unwrap_or(());

                                let event = ServerMessage::Rtc(
                                    shared::ws::rtc::RtcServerEvents::UserJoinedVoice {
                                        user_id: current_user_id,
                                    },
                                );

                                if let Ok(Some(channel)) =
                                    entity::channel::Entity::find_by_id(channel_id.0)
                                        .one(&state.db)
                                        .await
                                {
                                    let guild_id = GuildId(channel.guild_id);
                                    let _ = crate::core::broadcast::to_guild(
                                        &state.redis.messages,
                                        &guild_id,
                                        &event,
                                    )
                                    .await;
                                } else if let Ok(members) =
                                    entity::dm_channel_member::Entity::find()
                                        .filter(
                                            entity::dm_channel_member::Column::ChannelId
                                                .eq(channel_id.0),
                                        )
                                        .all(&state.db)
                                        .await
                                {
                                    for member in members {
                                        if member.user_id != current_user_id.0 {
                                            let _ = crate::core::broadcast::to_user(
                                                &state.redis.messages,
                                                &UserId(member.user_id),
                                                &event,
                                            )
                                            .await;
                                        }
                                    }
                                }
                            }
                            RtcClientEvents::LeaveVoice => {
                                if let Some(channel_id) = current_voice_channel {
                                    cleanup_voice_state(&state, channel_id, current_user_id).await;
                                    current_voice_channel = None;
                                }
                            }
                            RtcClientEvents::SdpOffer { sdp } => {
                                if let Some(channel_id) = current_voice_channel {
                                    match state
                                        .rtc
                                        .accept_offer(channel_id, current_user_id, sdp, tx.clone())
                                        .await
                                    {
                                        Ok(answer_sdp) => {
                                            let response = ServerMessage::Rtc(
                                                shared::ws::rtc::RtcServerEvents::SdpAnswer {
                                                    sdp: answer_sdp,
                                                },
                                            );
                                            if let Ok(json) = serde_json::to_string(&response) {
                                                let _ = tx.send(axum::extract::ws::Message::Text(
                                                    json.into(),
                                                ));
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to process SDP Offer: {}", e)
                                        }
                                    }
                                } else {
                                    send_error(
                                        &tx,
                                        "Must send JoinVoice before sending an SDP Offer.",
                                    );
                                }
                            }
                            RtcClientEvents::SdpAnswer { sdp } => {
                                let _ = state.rtc.accept_answer(current_user_id, sdp).await;
                            }
                            RtcClientEvents::IceCandidate { candidate } => {
                                if let Some(channel_id) = current_voice_channel {
                                    let _ = state
                                        .rtc
                                        .add_ice_candidate(channel_id, current_user_id, candidate)
                                        .await;
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = tx.send(Message::Pong(data));
            }
            Ok(Message::Close(_)) | Err(_) => {
                break;
            }
            _ => {}
        }
    }

    let heartbeat_state = state.clone();
    let heartbeat_user_id = current_user_id;

    let heartbeat_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

        loop {
            interval.tick().await;
            let _ = presence::refresh_presence(&heartbeat_state.redis.presence, &heartbeat_user_id)
                .await;
        }
    });

    while let Some(_res) = ws_receiver.next().await {}

    if let Some(channel_id) = current_voice_channel {
        cleanup_voice_state(&state, channel_id, current_user_id).await;
    }

    heartbeat_task.abort();
    send_task.abort();
    handle_disconnect(&state, current_user_id, session_id).await;

    Ok(())
}

fn send_error(tx: &tokio::sync::mpsc::UnboundedSender<Message>, msg: &str) {
    let payload = ServerMessage::Error {
        message: msg.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&payload) {
        let _ = tx.send(Message::Text(json.into()));
    }
}

async fn cleanup_voice_state(state: &SharedState, channel_id: ChannelId, user_id: UserId) {
    state.rtc.remove_user_from_room(&channel_id, &user_id);

    if let Ok(mut conn) = state.redis.presence.get().await {
        let _: () = deadpool_redis::redis::cmd("SREM")
            .arg(format!("voice_channel:{}", channel_id.0))
            .arg(user_id.0.to_string())
            .query_async(&mut conn)
            .await
            .unwrap_or(());
    }

    let event = ServerMessage::Rtc(shared::ws::rtc::RtcServerEvents::UserLeftVoice { user_id });

    if let Ok(Some(channel)) = entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await
    {
        let guild_id = GuildId(channel.guild_id);
        let _ = crate::core::broadcast::to_guild(&state.redis.messages, &guild_id, &event).await;
    } else if let Ok(members) = entity::dm_channel_member::Entity::find()
        .filter(entity::dm_channel_member::Column::ChannelId.eq(channel_id.0))
        .all(&state.db)
        .await
    {
        for member in members {
            if member.user_id != user_id.0 {
                let _ = crate::core::broadcast::to_user(
                    &state.redis.messages,
                    &UserId(member.user_id),
                    &event,
                )
                .await;
            }
        }
    }
}
