use crate::data::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SdpPayload {
    pub from: UserId,
    pub sdp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IcePayload {
    pub from: UserId,
    pub candidate: String,
}
