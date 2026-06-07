use crate::data::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum RtcClientEvents {
    SdpOffer {
        target_user_id: UserId,
        sdp: String,
    },
    SdpAnswer {
        target_user_id: UserId,
        sdp: String,
    },
    IceCandidate {
        target_user_id: UserId,
        candidate: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum RtcServerEvents {
    SdpOffer {
        from_user_id: UserId,
        sdp: String,
    },
    SdpAnswer {
        from_user_id: UserId,
        sdp: String,
    },
    IceCandidate {
        from_user_id: UserId,
        candidate: String,
    },
}
