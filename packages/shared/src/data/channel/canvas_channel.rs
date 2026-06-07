use crate::data::ChannelId;
use serde::{Deserialize, Serialize};

// TODO: Start implementing Canvas Channel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CanvasChannel {
    pub id: ChannelId,
}
