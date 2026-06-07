use crate::data::MessageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueryParams {
    pub limit: Option<i64>,
    pub before: Option<MessageId>,
    pub thread_id: Option<MessageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendRequest {
    pub content: String,
    pub thread_id: Option<MessageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEditRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePinRequest {
    pub label: Option<String>,
}
