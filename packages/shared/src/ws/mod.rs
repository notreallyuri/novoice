use crate::data::user::UserPresence;
use serde::{Deserialize, Serialize};

pub mod guild;
pub mod message;
pub mod rtc;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Identify { token: String },
    Chat(message::ChatClientEvents),
    Rtc(rtc::RtcClientEvents),
    SetPresence { presence: UserPresence },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    User(user::UserServerEvents),
    Chat(message::ChatServerEvents),
    Guild(guild::GuildServerEvents),
    Rtc(rtc::RtcServerEvents),
    Error { message: String },
}
