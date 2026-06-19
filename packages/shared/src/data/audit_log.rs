use crate::data::{GuildId, LogId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum AuditActionType {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    CategoryCreate = 13,
    CategoryUpdate = 14,
    CategoryDelete = 15,
    MemberKick = 20,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    MessageDelete = 72,
}

impl From<i32> for AuditActionType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::GuildUpdate,
            10 => Self::ChannelCreate,
            11 => Self::ChannelUpdate,
            12 => Self::ChannelDelete,
            13 => Self::CategoryCreate,
            14 => Self::CategoryUpdate,
            15 => Self::CategoryDelete,
            20 => Self::MemberKick,
            22 => Self::MemberBanAdd,
            23 => Self::MemberBanRemove,
            24 => Self::MemberUpdate,
            30 => Self::RoleCreate,
            31 => Self::RoleUpdate,
            32 => Self::RoleDelete,
            72 => Self::MessageDelete,
            _ => Self::GuildUpdate,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntry {
    pub id: LogId,
    pub guild_id: GuildId,
    pub actor_id: UserId,
    pub target_id: Option<Uuid>,
    pub action_type: AuditActionType,
    pub reason: Option<String>,
    pub changes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
