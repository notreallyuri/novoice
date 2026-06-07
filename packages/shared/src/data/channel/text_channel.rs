use crate::data::{CategoryId, ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChannelMode {
    #[default]
    Chat,
    Board,
    Threads,
}

impl ChannelMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelMode::Chat => "chat",
            ChannelMode::Board => "board",
            ChannelMode::Threads => "threads",
        }
    }

    pub fn from(s: &str) -> Self {
        match s {
            "board" => ChannelMode::Board,
            "threads" => ChannelMode::Threads,
            _ => ChannelMode::Chat,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    #[serde(default)]
    pub mode: ChannelMode,
}
