use crate::data::ChannelId;
use serde::{Deserialize, Serialize};

// TODO: Start implementing Docs channel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocsChannel {
    pub id: ChannelId,
}
