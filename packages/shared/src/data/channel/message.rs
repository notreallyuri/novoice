use crate::data::{ChannelId, MessageId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub thread_id: Option<MessageId>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinnedMessage {
    pub message_id: MessageId,
    pub pinned_by: UserId,
    pub pinned_at: DateTime<Utc>,
    pub label: Option<String>,
}
