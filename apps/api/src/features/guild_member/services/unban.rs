use crate::core::{
    audit::log_action, error::AppError, guards::verify_permission, state::SharedState,
};
use entity::guild_ban;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::data::{GuildId, UserId, audit_log::AuditActionType, permissions::Permissions};

pub async fn unban(
    state: &SharedState,
    user_id: UserId,
    guild_id: GuildId,
    target_id: UserId,
) -> Result<(), AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::BAN_MEMBERS).await?;

    let ban_record = guild_ban::Entity::find()
        .filter(guild_ban::Column::GuildId.eq(guild_id.0))
        .filter(guild_ban::Column::UserId.eq(target_id.0))
        .one(&state.db)
        .await?;

    if let Some(b) = ban_record {
        b.delete(&state.db).await?;

        let _ = log_action(
            &state.db,
            guild_id,
            user_id,
            AuditActionType::MemberBanRemove,
            Some(target_id.0),
            None,
            None,
        )
        .await;
    }

    Ok(())
}
