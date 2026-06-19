use entity::channel;
use sea_orm::{EntityTrait, ModelTrait};
use shared::{
    data::{ChannelId, GuildId, UserId, audit_log::AuditActionType, permissions::Permissions},
    ws::{ServerMessage, guild::GuildServerEvents},
};

use crate::core::{
    audit::log_action, broadcast, error::AppError, guards::verify_permission, state::SharedState,
};

pub async fn delete(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    channel_id: ChannelId,
) -> Result<(), AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let target_channel = channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let target_name = target_channel.name.clone();

    if target_channel.guild_id != guild_id.0 {
        return Err(AppError::Forbidden(
            "Channel does not belong to this guild".into(),
        ));
    }

    target_channel.delete(&state.db).await?;

    let _ = log_action(
        &state.db,
        guild_id,
        user_id,
        AuditActionType::ChannelDelete,
        Some(channel_id.0),
        None,
        Some(serde_json::json!({
            "name": target_name
        })),
    )
    .await;

    let event = ServerMessage::Guild(GuildServerEvents::ChannelDeleted {
        guild_id,
        channel_id,
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(())
}
