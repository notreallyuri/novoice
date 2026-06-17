use entity::category;
use sea_orm::{EntityTrait, ModelTrait};
use shared::{
    data::{CategoryId, GuildId, UserId, permissions::Permissions},
    ws::{ServerMessage, guild::GuildServerEvents},
};

use crate::core::{broadcast, error::AppError, guards::verify_permission, state::SharedState};

pub async fn delete(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    category_id: CategoryId,
) -> Result<(), AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let target_category = category::Entity::find_by_id(category_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if target_category.guild_id != guild_id.0 {
        return Err(AppError::Forbidden(
            "Category does not belong to this guild".into(),
        ));
    }

    target_category.delete(&state.db).await?;

    let event = ServerMessage::Guild(GuildServerEvents::CategoryDeleted {
        guild_id,
        category_id,
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(())
}
