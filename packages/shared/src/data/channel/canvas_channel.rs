use crate::data::{CategoryId, ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CanvasChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    // TODO: Include later `canvas_state_id` or `background_color`
}
