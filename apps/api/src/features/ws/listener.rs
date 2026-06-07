use crate::core::{cache::guild_members::get_cached_guild_members, state::SharedState};
use axum::extract::ws::Message;
use futures_util::StreamExt;
use shared::data::{GuildId, UserId};
use uuid::Uuid;

pub async fn redis_listener(state: SharedState) {
    let redis_url = std::env::var("REDIS_MESSAGES_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6381/1".to_string());

    tokio::spawn(async move {
        tracing::info!("Starting Redis Pub/Sub listener...");

        let client = match redis::Client::open(redis_url) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to parse Redis URL: {e}");
                return;
            }
        };

        let mut pubsub = match client.get_async_pubsub().await {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Failed to connect to Redis PubSub: {e}");
                return;
            }
        };

        if let Err(e) = pubsub.psubscribe(&["channel:*", "guild:*", "user:*"]).await {
            tracing::error!("Redis PSUBSCRIBE failed: {e}");
            return;
        }

        tracing::info!("Redis listener successfully subscribed to 'channel:*'");

        let mut message_stream = pubsub.on_message();

        while let Some(msg) = message_stream.next().await {
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to parse Redis payload: {e}");
                    continue;
                }
            };

            let channel_name = msg.get_channel_name();
            let ws_message = Message::Text(payload.clone().into());

            if channel_name.starts_with("user:")
                && let Some(id_str) = channel_name.strip_prefix("user:")
                && let Ok(target_id) = Uuid::parse_str(id_str)
            {
                dispatch_to_user(&state, UserId(target_id), ws_message).await;
            } else if channel_name.starts_with("guild:")
                && let Some(id_str) = channel_name.strip_prefix("guild:")
                && let Ok(target_id) = Uuid::parse_str(id_str)
            {
                dispatch_to_guild(&state, GuildId(target_id), ws_message).await;
            }
        }

        tracing::warn!("Redis listener stream ended unexpectedly!");
    });
}

async fn dispatch_to_user(state: &SharedState, user_id: UserId, msg: Message) {
    if let Some(user_sessions) = state.active_sessions.get(&user_id) {
        let sessions = user_sessions.read().unwrap();

        for (_, session) in sessions.iter() {
            let _ = session.tx.send(msg.clone());
        }
    }
}
async fn dispatch_to_guild(state: &SharedState, guild_id: GuildId, msg: Message) {
    let members = match get_cached_guild_members(state, guild_id).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to fetch guild members for broadcast: {e}");
            return;
        }
    };

    for user_id in members {
        if state.active_sessions.contains_key(&user_id) {
            dispatch_to_user(state, user_id, msg.clone()).await;
        }
    }
}
