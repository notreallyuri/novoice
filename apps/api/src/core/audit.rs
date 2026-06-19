use crate::core::error::AppError;
use entity::audit_log;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use shared::data::{GuildId, UserId, audit_log::AuditActionType};
use uuid::Uuid;

pub async fn log_action(
    db: &DatabaseConnection,
    guild_id: GuildId,
    actor_id: UserId,
    action_type: AuditActionType,
    target_id: Option<Uuid>,
    reason: Option<String>,
    changes: Option<serde_json::Value>,
) -> Result<(), AppError> {
    let now = chrono::Utc::now();

    let entry = audit_log::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        guild_id: Set(guild_id.0),
        actor_id: Set(actor_id.0),
        target_id: Set(target_id),
        action_type: Set(action_type as i32),
        reason: Set(reason),
        changes: Set(changes),
        created_at: Set(now.into()),
    };

    if let Err(e) = entry.insert(db).await {
        tracing::error!(
            "Failed to write audit log entry for guild {}: {}",
            guild_id.0,
            e
        );
    }

    Ok(())
}
