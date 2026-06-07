use crate::data::{
    ChannelId, MessageId,
    channel::message::{Message, PinnedMessage},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum ChatClientEvents {
    Send {
        channel_id: ChannelId,
        content: String,
        thread_id: Option<MessageId>,
    },
    Edit {
        channel_id: ChannelId,
        message_id: MessageId,
        content: String,
    },
    Delete {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    Pin {
        channel_id: ChannelId,
        message_id: MessageId,
        label: Option<String>,
    },
    Unpin {
        channel_id: ChannelId,
        message_id: MessageId,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum ChatServerEvents {
    Received {
        message: Box<Message>,
    },
    Edited {
        message: Box<Message>,
    },
    Deleted {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    Pinned {
        channel_id: ChannelId,
        pin: Box<PinnedMessage>,
    },
    Unpinned {
        channel_id: ChannelId,
        message_id: MessageId,
    },
}
