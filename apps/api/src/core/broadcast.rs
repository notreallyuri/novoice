use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::Serialize;
use shared::data::{ChannelId, GuildId, UserId};

use crate::core::error::AppError;

pub async fn to_channel<T: Serialize>(
    redis_pool: &Pool,
    channel_id: &ChannelId,
    payload: &T,
) -> Result<(), AppError> {
    let mut conn = redis_pool.get().await?;
    let msg = serde_json::to_string(payload)?;
    let _: () = conn
        .publish(format!("channel:{}", channel_id.0), msg)
        .await?;

    Ok(())
}

pub async fn to_guild<T: Serialize>(
    redis_pool: &Pool,
    guild_id: &GuildId,
    payload: &T,
) -> Result<(), AppError> {
    let mut conn = redis_pool.get().await?;
    let msg = serde_json::to_string(payload)?;
    let _: () = conn.publish(format!("guild:{}", guild_id.0), msg).await?;

    Ok(())
}

pub async fn to_user<T: Serialize>(
    redis_pool: &Pool,
    user_id: &UserId,
    payload: &T,
) -> Result<(), AppError> {
    let mut conn = redis_pool.get().await?;
    let msg = serde_json::to_string(payload)?;
    let _: () = conn.publish(format!("user:{}", user_id.0), msg).await?;

    Ok(())
}

pub async fn to_friends<T: Serialize>(
    redis_pool: &Pool,
    friend_ids: &[UserId],
    payload: &T,
) -> Result<(), AppError> {
    if friend_ids.is_empty() {
        return Ok(());
    }

    let mut conn = redis_pool.get().await?;
    let msg = serde_json::to_string(payload)?;

    let mut pipeline = redis::pipe();

    for friend_id in friend_ids {
        pipeline
            .cmd("PUBLISH")
            .arg(format!("user:{}", friend_id.0))
            .arg(&msg)
            .ignore();
    }

    let _: () = pipeline.query_async(&mut conn).await?;

    Ok(())
}
