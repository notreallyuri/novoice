use crate::data::{CategoryId, ChannelId, GuildId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceParticipant {
    pub user_id: UserId,
    pub muted: bool,
    pub deafened: bool,
    pub speaking: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    pub user_limit: Option<i32>,
    #[serde(default = "default_bitrate")]
    pub bitrate: i32,
    #[serde(default)]
    pub participants: Vec<VoiceParticipant>,
}

fn default_bitrate() -> i32 {
    64_000
}
