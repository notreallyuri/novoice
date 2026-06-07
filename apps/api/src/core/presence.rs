use deadpool_redis::Pool;
use shared::data::user::{Status, UserPresence};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::error::AppError;

const PRESENCE_PREFIX: &str = "presence:";

pub async fn set_presence(
    pool: &Pool,
    user_id: &Uuid,
    presence: &UserPresence,
) -> Result<(), AppError> {
    let key = format!("{}{}", PRESENCE_PREFIX, user_id);
    let mut conn = pool.get().await?;

    let json_str = serde_json::to_string(presence)?;

    let _: () = deadpool_redis::redis::cmd("SET")
        .arg(&key)
        .arg(&json_str)
        .query_async(&mut conn)
        .await?;

    Ok(())
}

pub async fn get_presence(pool: &Pool, user_id: &Uuid) -> Result<UserPresence, AppError> {
    let key = format!("{}{}", PRESENCE_PREFIX, user_id);

    let mut conn = pool.get().await?;

    let result: Option<String> = deadpool_redis::redis::cmd("GET")
        .arg(&key)
        .query_async(&mut conn)
        .await?;

    let presence = result
        .and_then(|json_str| serde_json::from_str(&json_str).ok())
        .unwrap_or(UserPresence {
            status: Status::Offline,
            preset: None,
        });

    Ok(presence)
}

pub async fn get_bulk(
    pool: &Pool,
    user_ids: &[Uuid],
) -> Result<HashMap<Uuid, UserPresence>, AppError> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let keys: Vec<String> = user_ids
        .iter()
        .map(|id| format!("{}{}", PRESENCE_PREFIX, id))
        .collect();

    let mut conn = pool.get().await?;

    let results: Vec<Option<String>> = deadpool_redis::redis::cmd("MGET")
        .arg(&keys)
        .query_async(&mut conn)
        .await?;

    let mut presence_map = HashMap::new();

    for (id, res) in user_ids.iter().zip(results.into_iter()) {
        let presence = if let Some(json_str) = res {
            serde_json::from_str(&json_str).unwrap_or_else(|_| UserPresence {
                status: Status::Offline,
                preset: None,
            })
        } else {
            UserPresence {
                status: Status::Offline,
                preset: None,
            }
        };

        presence_map.insert(*id, presence);
    }

    Ok(presence_map)
}
