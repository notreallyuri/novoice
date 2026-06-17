use crate::data::{CategoryId, ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocsChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    // TODO: Add docs-specific fields `readonly: bool`, `latest_revision_id`
}
