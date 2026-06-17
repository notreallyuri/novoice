use crate::core::{
    broadcast, cache::guild_members::cache_remove_member, error::AppError,
    guards::verify_permission, state::SharedState,
};
use entity::{guild, guild_member};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::{
    data::{GuildId, UserId, permissions::Permissions},
    ws::{ServerMessage, guild::GuildServerEvents},
};

pub async fn kick(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    target_id: UserId,
) -> Result<(), AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::KICK_MEMBERS).await?;

    let target_guild = guild::Entity::find_by_id(guild_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if target_guild.owner_id == target_id.0 {
        return Err(AppError::Forbidden("Cannot kick the serrver owner.".into()));
    }

    let member = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(target_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    member.delete(&state.db).await?;
    cache_remove_member(state, guild_id, user_id).await;

    let event = ServerMessage::Guild(GuildServerEvents::MemberLeft {
        guild_id,
        user_id: target_id,
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;
    broadcast::to_user(&state.redis.messages, &user_id, &event).await?;

    Ok(())
}
