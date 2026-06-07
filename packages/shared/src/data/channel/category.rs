use crate::data::{CategoryId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelCategory {
    pub id: CategoryId,
    pub guild_id: GuildId,
    pub name: String,
    pub position: i32,
}
