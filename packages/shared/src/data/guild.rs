use super::{
    ChannelId, GuildId, UserId,
    channel::prelude::{Channel, ChannelCategory},
    user::UserPublic,
};
use crate::data::{RoleId, permissions::Permissions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildIdentity {
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub show_global_username: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub color: Option<i32>,
    pub hoist: bool,
    pub position: i32,
    pub permissions: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildMember {
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub roles: Vec<RoleId>,
    pub joined_at: String,
    pub data: UserPublic,
    pub identity: Option<GuildIdentity>,
}

impl GuildMember {
    pub fn has_permission(&self, all_roles: &[Role], required: Permissions) -> bool {
        let member_permissions: Vec<i64> = all_roles
            .iter()
            .filter(|r| self.roles.contains(&r.id))
            .map(|r| r.permissions)
            .collect();

        let total_perms = Permissions::compute_base_permissions(&member_permissions);
        total_perms.contains(required)
    }

    pub fn is_admin(&self, all_roles: &[Role]) -> bool {
        self.has_permission(all_roles, Permissions::ADMINISTRATOR)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildProfile {
    pub owner_id: UserId,
    pub banner_url: Option<String>,
    pub icon_url: Option<String>,
    pub name: String,
    pub default_channel_id: Option<ChannelId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildSummary {
    pub id: GuildId,
    #[serde(flatten)]
    pub profile: GuildProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: GuildId,
    #[serde(flatten)]
    pub profile: GuildProfile,
    pub roles: Vec<Role>,
    pub members: Vec<GuildMember>,
    pub categories: Vec<ChannelCategory>,
    pub channels: Vec<Channel>,
}
