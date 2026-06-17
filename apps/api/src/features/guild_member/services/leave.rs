use crate::core::{
    broadcast, cache::guild_members::cache_remove_member, error::AppError, state::SharedState,
};
use entity::{guild, guild_member};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::{
    data::{GuildId, UserId},
    ws::{ServerMessage, guild::GuildServerEvents},
};

pub async fn leave(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
) -> Result<(), AppError> {
    let target_guild = guild::Entity::find_by_id(guild_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if target_guild.owner_id == user_id.0 {
        return Err(AppError::BadRequest(
            "Guild Owners cannot leave. Transfer ownership or delete the entire guild.".into(),
        ));
    }

    let member = guild_member::Entity::find()
        .filter(guild_member::Column::UserId.eq(user_id.0))
        .filter(guild_member::Column::GuildId.eq(guild_id.0))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    member.delete(&state.db).await?;

    cache_remove_member(state, guild_id, user_id).await;

    let event = ServerMessage::Guild(GuildServerEvents::MemberLeft { guild_id, user_id });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;
    broadcast::to_user(&state.redis.messages, &user_id, &event).await?;

    Ok(())
}
