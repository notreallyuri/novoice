use crate::data::{ChannelId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum RtcClientEvents {
    JoinVoice { channel_id: ChannelId },
    LeaveVoice,
    SdpOffer { sdp: String },
    SdpAnswer { sdp: String },
    IceCandidate { candidate: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum RtcServerEvents {
    SdpOffer { sdp: String },
    SdpAnswer { sdp: String },
    IceCandidate { candidate: String },
    UserJoinedVoice { user_id: UserId },
    UserLeftVoice { user_id: UserId },
}
