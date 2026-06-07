use entity::relationship::{self, DbRelationshipStatus};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use shared::data::UserId;
use uuid::Uuid;

use crate::core::{error::AppError, state::SharedState};

pub async fn get_cached_friends(
    state: &SharedState,
    user_id: UserId,
) -> Result<Vec<UserId>, AppError> {
    let cache_key = format!("user_friends:{}", user_id.0);
    let mut conn = state.redis.cache.get().await?;

    let cached_friends: Vec<String> = deadpool_redis::redis::cmd("SMEMBERS")
        .arg(&cache_key)
        .query_async(&mut conn)
        .await?;

    if !cached_friends.is_empty() {
        let friends: Vec<UserId> = cached_friends
            .into_iter()
            .filter_map(|id_str| Uuid::parse_str(&id_str).ok())
            .map(UserId)
            .collect();

        return Ok(friends);
    }

    let db_friends = relationship::Entity::find()
        .filter(relationship::Column::Status.eq(DbRelationshipStatus::Friend))
        .filter(
            Condition::any()
                .add(relationship::Column::UserId.eq(user_id.0))
                .add(relationship::Column::TargetId.eq(user_id.0)),
        )
        .all(&state.db)
        .await?;

    if db_friends.is_empty() {
        return Ok(Vec::new());
    }

    let friend_ids: Vec<Uuid> = db_friends
        .iter()
        .map(|rel| {
            if rel.user_id == user_id.0 {
                rel.target_id
            } else {
                rel.user_id
            }
        })
        .collect();

    let friend_ids_str: Vec<String> = friend_ids.iter().map(|id| id.to_string()).collect();

    let mut pipeline = redis::pipe();
    pipeline
        .cmd("SADD")
        .arg(&cache_key)
        .arg(&friend_ids_str)
        .ignore();
    pipeline.cmd("EXPIRE").arg(&cache_key).arg(86400).ignore();
    let _: () = pipeline.query_async(&mut conn).await?;

    let users: Vec<UserId> = friend_ids.into_iter().map(UserId).collect();

    Ok(users)
}

pub async fn cache_add_friend(state: &SharedState, user_a: UserId, user_b: UserId) {
    let mut conn = match state.redis.cache.get().await {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut pipeline = redis::pipe();
    pipeline
        .cmd("SADD")
        .arg(format!("user_friends:{}", user_a.0))
        .arg(user_b.0.to_string())
        .ignore();
    pipeline
        .cmd("SADD")
        .arg(format!("user_friends:{}", user_b.0))
        .arg(user_a.0.to_string())
        .ignore();

    let _: Result<(), _> = pipeline.query_async(&mut conn).await;
}

pub async fn cache_remove_friend(state: &SharedState, user_a: UserId, user_b: UserId) {
    let mut conn = match state.redis.cache.get().await {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut pipeline = redis::pipe();
    pipeline
        .cmd("SREM")
        .arg(format!("user_friends:{}", user_a.0))
        .arg(user_b.0.to_string())
        .ignore();
    pipeline
        .cmd("SREM")
        .arg(format!("user_friends:{}", user_b.0))
        .arg(user_a.0.to_string())
        .ignore();

    let _: Result<(), _> = pipeline.query_async(&mut conn).await;
}
