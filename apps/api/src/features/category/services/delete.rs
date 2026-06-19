use crate::core::{
    audit::log_action, broadcast, error::AppError, guards::verify_permission, state::SharedState,
};
use entity::category;
use sea_orm::{EntityTrait, ModelTrait};
use shared::{
    data::{CategoryId, GuildId, UserId, audit_log::AuditActionType, permissions::Permissions},
    ws::{ServerMessage, guild::GuildServerEvents},
};

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

    let target_name = target_category.name.clone();

    if target_category.guild_id != guild_id.0 {
        return Err(AppError::Forbidden(
            "Category does not belong to this guild".into(),
        ));
    }

    target_category.delete(&state.db).await?;

    let _ = log_action(
        &state.db,
        guild_id,
        user_id,
        AuditActionType::CategoryDelete,
        Some(category_id.0),
        None,
        Some(serde_json::json!({
            "name": target_name,
        })),
    )
    .await;

    let event = ServerMessage::Guild(GuildServerEvents::CategoryDeleted {
        guild_id,
        category_id,
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(())
}
