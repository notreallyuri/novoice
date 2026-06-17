use crate::data::{ChannelId, user::UserPublic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DMChannel {
    pub id: ChannelId,
    pub recipients: Vec<UserPublic>,
    pub is_open: bool,
}
