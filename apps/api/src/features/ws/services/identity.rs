use crate::core::{
    broadcast,
    cache::friends::get_cached_friends,
    error::AppError,
    mappers::FromDomain,
    presence,
    state::{SessionId, SharedState, Tx, WsSession},
};
use axum::extract::ws::Message;
use entity::{guild_member, user};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use shared::{
    data::{
        GuildId, UserId,
        user::{Status, UserPresence, UserProfile, UserPublic},
    },
    ws::{ServerMessage, user::UserServerEvents},
};
use uuid::Uuid;

pub async fn handle_connect(
    state: &SharedState,
    user_id: UserId,
    session_id: SessionId,
    tx: Tx,
) -> Result<(), AppError> {
    let session_map = state.active_sessions.entry(user_id).or_default();
    session_map.write().unwrap().insert(
        session_id,
        WsSession {
            tx: tx.clone(),
            user_id,
        },
    );

    let ready_msg = ServerMessage::User(UserServerEvents::Ready);
    let _ = tx.send(Message::Text(serde_json::to_string(&ready_msg)?.into()));

    let (user_model_opt, guild_ids) = tokio::try_join!(
        user::Entity::find_by_id(user_id.0).one(&state.db),
        guild_member::Entity::find()
            .select_only()
            .column(guild_member::Column::GuildId)
            .filter(guild_member::Column::UserId.eq(user_id.0))
            .into_tuple::<Uuid>()
            .all(&state.db)
    )?;

    let user_model = user_model_opt.ok_or(AppError::NotFound)?;

    let current_presence = presence::get_presence(&state.redis.presence, &user_id.0).await?;
    let new_presence = if current_presence.status == Status::Offline {
        UserPresence {
            status: Status::Online,
            preset: None,
        }
    } else {
        current_presence
    };

    presence::set_presence(&state.redis.presence, &user_id.0, &new_presence).await?;

    let online_msg = ServerMessage::User(UserServerEvents::PresenceUpdate {
        user: Box::new(UserPublic {
            id: user_id,
            profile: UserProfile::from_domain(user_model),
            presence: new_presence,
        }),
    });

    for id in guild_ids {
        broadcast::to_guild(&state.redis.messages, &GuildId(id), &online_msg).await?;
    }

    if let Ok(friend_ids) = get_cached_friends(state, user_id).await {
        let _ = broadcast::to_friends(&state.redis.messages, &friend_ids, &online_msg).await;
    }

    Ok(())
}

pub async fn handle_disconnect(state: &SharedState, user_id: UserId, session_id: SessionId) {
    let mut is_offline = false;

    if let Some(user_sessions_entry) = state.active_sessions.get(&user_id) {
        let mut session_map = user_sessions_entry.write().unwrap();

        session_map.remove(&session_id);

        if session_map.is_empty() {
            is_offline = true;
        }
    }

    if is_offline {
        state.active_sessions.remove(&user_id);

        let offline_presence = UserPresence {
            status: Status::Offline,
            preset: None,
        };

        if let Err(e) =
            presence::set_presence(&state.redis.presence, &user_id.0, &offline_presence).await
        {
            tracing::error!("Failed to set offline presence for {}: {}", user_id.0, e);
        }

        let (user_res, guilds_res) = tokio::join!(
            user::Entity::find_by_id(user_id.0).one(&state.db),
            guild_member::Entity::find()
                .select_only()
                .column(guild_member::Column::GuildId)
                .filter(guild_member::Column::UserId.eq(user_id.0))
                .into_tuple::<Uuid>()
                .all(&state.db)
        );

        if let Ok(Some(user_model)) = user_res {
            let offline_msg = ServerMessage::User(UserServerEvents::PresenceUpdate {
                user: Box::new(UserPublic {
                    id: user_id,
                    profile: UserProfile::from_domain(user_model),
                    presence: offline_presence,
                }),
            });

            if let Ok(guild_ids) = guilds_res {
                for id in guild_ids {
                    let _ = broadcast::to_guild(&state.redis.messages, &GuildId(id), &offline_msg)
                        .await;
                }
            }

            if let Ok(friend_ids) = get_cached_friends(state, user_id).await {
                let _ =
                    broadcast::to_friends(&state.redis.messages, &friend_ids, &offline_msg).await;
            }
        }
    }
}
