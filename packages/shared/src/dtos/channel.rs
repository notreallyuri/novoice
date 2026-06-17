use crate::data::{CategoryId, channel::prelude::ChannelMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateChannelKind {
    Text,
    Voice,
    Canvas,
    Docs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub kind: CreateChannelKind,
    pub category_id: Option<CategoryId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelRequest {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub category_id: Option<Option<CategoryId>>,
    pub mode: Option<ChannelMode>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<Option<i32>>,
}
