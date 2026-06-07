use crate::core::{error::AppError, state::SharedState};
use entity::guild_member;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use shared::data::{GuildId, UserId};
use uuid::Uuid;

pub async fn get_cached_guild_members(
    state: &SharedState,
    guild_id: GuildId,
) -> Result<Vec<UserId>, AppError> {
    let cache_key = format!("guild_members:{}", guild_id.0);

    let mut conn = state.redis.cache.get().await?;

    let cached_members: Vec<String> = deadpool_redis::redis::cmd("SMEMBERS")
        .arg(&cache_key)
        .query_async(&mut conn)
        .await?;

    if !cached_members.is_empty() {
        let users: Vec<UserId> = cached_members
            .into_iter()
            .filter_map(|id_str| Uuid::parse_str(&id_str).ok())
            .map(UserId)
            .collect();

        return Ok(users);
    }

    let db_members = guild_member::Entity::find()
        .select_only()
        .column(guild_member::Column::UserId)
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .into_tuple::<Uuid>()
        .all(&state.db)
        .await?;

    if db_members.is_empty() {
        return Ok(Vec::new());
    }

    let members_str: Vec<String> = db_members.iter().map(|id| id.to_string()).collect();

    let mut pipeline = redis::pipe();
    pipeline
        .cmd("SADD")
        .arg(&cache_key)
        .arg(&members_str)
        .ignore();
    pipeline.cmd("EXPIRE").arg(&cache_key).arg(86400).ignore();

    let _: () = pipeline.query_async(&mut conn).await?;

    let users: Vec<UserId> = db_members.into_iter().map(UserId).collect();

    Ok(users)
}
