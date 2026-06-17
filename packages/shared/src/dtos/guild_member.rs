use crate::data::{GuildId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateGuildMemberRequest {
    pub user_id: UserId,
    pub guild_id: GuildId,
    pub identity_display_name: Option<String>,
    pub identity_avatar: Option<String>,
    pub identity_banner: Option<String>,
    pub identity_bio: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BanMemberRequest {
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
