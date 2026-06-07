use crate::data::{guild::GuildSummary, relationship::UserRelationship, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeResponse {
    pub user: User,
    pub guilds: Vec<GuildSummary>,
    pub relationships: Vec<UserRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildIdentityRequest {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
    pub show_global_username: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGuildRequest {
    pub invite_code: Uuid,
    pub identity: Option<GuildIdentityRequest>,
}
