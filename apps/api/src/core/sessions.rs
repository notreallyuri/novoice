use deadpool_redis::{Pool, redis::AsyncCommands};
use uuid::Uuid;

use crate::core::error::AppError;

const SESSION_TTL_SECONDS: u64 = 604800;
const SESSION_PREFIX: &str = "session:";

fn build_key(key: &str) -> String {
    format!("{}{}", SESSION_PREFIX, key)
}

pub async fn create(pool: &Pool, user_id: &str) -> Result<String, AppError> {
    let key = Uuid::new_v4().to_string();
    let redis = build_key(&key);
    let mut conn = pool.get().await?;

    let _: () = conn.set_ex(&redis, user_id, SESSION_TTL_SECONDS).await?;

    Ok(key)
}

pub async fn verify_and_extend(pool: &Pool, key: &str) -> Result<String, AppError> {
    let redis = build_key(key);
    let mut conn = pool.get().await?;

    let res = deadpool_redis::redis::cmd("GETEX")
        .arg(&redis)
        .arg("EX")
        .arg(SESSION_TTL_SECONDS)
        .query_async::<Option<String>>(&mut conn)
        .await?;

    res.ok_or(AppError::Unauthorized)
}

pub async fn extend(pool: &Pool, key: &str) -> Result<(), AppError> {
    let redis = build_key(key);
    let mut conn = pool.get().await?;

    let _: () = deadpool_redis::redis::cmd("EXPIRE")
        .arg(&redis)
        .arg(SESSION_TTL_SECONDS)
        .query_async(&mut conn)
        .await?;

    Ok(())
}

pub async fn revoke(pool: &Pool, key: &str) -> Result<(), AppError> {
    let redis = build_key(key);
    let mut conn = pool.get().await?;

    let _: () = conn.del(&redis).await?;

    Ok(())
}
