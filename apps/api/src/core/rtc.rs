use crate::core::error::AppError;
use axum::extract::ws::Message;
use dashmap::DashMap;
use shared::data::{ChannelId, UserId};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use webrtc::{
    api::{
        API, APIBuilder,
        interceptor_registry::register_default_interceptors,
        media_engine::{MIME_TYPE_OPUS, MediaEngine},
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        RTCPeerConnection, configuration::RTCConfiguration,
        sdp::session_description::RTCSessionDescription,
    },
    rtp_transceiver::rtp_codec::RTCRtpCodecCapability,
    track::{
        track_local::{TrackLocal, TrackLocalWriter, track_local_static_rtp::TrackLocalStaticRTP},
        track_remote::TrackRemote,
    },
};

pub struct PeerSession {
    pub connection: Arc<RTCPeerConnection>,
    pub outbound_audio_track: Arc<TrackLocalStaticRTP>,
}

pub struct VoiceRoom {
    pub channel_id: ChannelId,
    pub peers: DashMap<UserId, PeerSession>,
}

impl VoiceRoom {
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            peers: DashMap::new(),
        }
    }
}

pub struct WebRtcManager {
    pub api: API,
    pub rooms: DashMap<ChannelId, Arc<VoiceRoom>>,
}

impl WebRtcManager {
    pub fn new() -> Result<Self, AppError> {
        let mut m = MediaEngine::default();
        m.register_default_codecs()?;

        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut m)?;

        let api = APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        Ok(Self {
            api,
            rooms: DashMap::new(),
        })
    }

    pub async fn create_peer_connection(&self) -> Result<Arc<RTCPeerConnection>, AppError> {
        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            }],
            ..Default::default()
        };

        Ok(Arc::new(self.api.new_peer_connection(config).await?))
    }

    pub fn get_or_create_room(&self, channel_id: ChannelId) -> Arc<VoiceRoom> {
        self.rooms
            .entry(channel_id)
            .or_insert_with(|| Arc::new(VoiceRoom::new(channel_id)))
            .clone()
    }

    pub fn add_user_to_room(&self, channel_id: ChannelId, user_id: UserId, session: PeerSession) {
        let room = self.get_or_create_room(channel_id);
        room.peers.insert(user_id, session);
    }

    pub fn remove_user_from_room(&self, channel_id: &ChannelId, user_id: &UserId) {
        if let Some(room) = self.rooms.get(channel_id) {
            room.peers.remove(user_id);

            if room.peers.is_empty() {
                drop(room);
                self.rooms.remove(channel_id);
                tracing::info!("Destroyed empty voice room: {}", channel_id.0);
            }
        }
    }

    pub async fn accept_offer(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        offer_sdp: String,
        tx: UnboundedSender<Message>,
    ) -> Result<String, AppError> {
        tracing::info!("Processing SDP Offer for user {}", user_id.0);
        let pc = self.create_peer_connection().await?;

        pc.on_ice_candidate(Box::new(
            move |candidate: Option<webrtc::ice_transport::ice_candidate::RTCIceCandidate>| {
                let tx_clone = tx.clone();
                Box::pin(async move {
                    if let Some(c) = candidate {
                        if let Ok(json_string) = c.to_json() {
                            if let Ok(candidate_str) = serde_json::to_string(&json_string) {
                                let event = shared::ws::ServerMessage::Rtc(
                                    shared::ws::rtc::RtcServerEvents::IceCandidate {
                                        candidate: candidate_str,
                                    },
                                );

                                if let Ok(msg_json) = serde_json::to_string(&event) {
                                    let _ = tx_clone.send(Message::Text(msg_json.into()));
                                }
                            }
                        }
                    }
                })
            },
        ));

        let room = self.get_or_create_room(channel_id);
        let room_clone = Arc::clone(&room);
        let sender_id = user_id;

        pc.on_track(Box::new(
            move |track: Arc<TrackRemote>, _receiver, _transceiver| {
                let room_inner = Arc::clone(&room_clone);

                Box::pin(async move {
                    tracing::info!("Incoming track started for user {}", sender_id.0);
                    loop {
                        let (rtp_packet, _) = match track.read_rtp().await {
                            Ok(res) => res,
                            Err(webrtc::Error::ErrClosedPipe) => {
                                tracing::info!("Track closed for user {}", sender_id.0);
                                break;
                            }
                            Err(e) => {
                                tracing::error!("Error reading RTP packet: {}", e);
                                break;
                            }
                        };

                        for entry in room_inner.peers.iter() {
                            let peer_id = entry.key();
                            let peer_session = entry.value();

                            if *peer_id != sender_id {
                                let _ = peer_session
                                    .outbound_audio_track
                                    .write_rtp(&rtp_packet)
                                    .await;
                            }
                        }
                    }
                })
            },
        ));

        let session_desc = RTCSessionDescription::offer(offer_sdp)
            .map_err(|_| AppError::BadRequest("Invalid SPD Offer".into()))?;

        pc.set_remote_description(session_desc).await?;

        let outbound_audio_track = Arc::new(TrackLocalStaticRTP::new(
            RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                ..Default::default()
            },
            "audio".to_owned(),
            "novoice-sfu".to_owned(),
        ));

        pc.add_track(Arc::clone(&outbound_audio_track) as Arc<dyn TrackLocal + Send + Sync>)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to add track: {}", e)))?;

        let answer = pc
            .create_answer(None)
            .await
            .map_err(|_| AppError::Internal("Failed to generate WebRTC answer".into()))?;

        let local_desc = answer.clone();
        pc.set_local_description(local_desc)
            .await
            .map_err(|_| AppError::Internal("Failed to bind local media state".into()))?;

        let session = PeerSession {
            connection: Arc::clone(&pc),
            outbound_audio_track,
        };
        self.add_user_to_room(channel_id, user_id, session);

        Ok(answer.sdp)
    }

    pub async fn add_ice_candidate(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        candidate: String,
    ) -> Result<(), AppError> {
        if let Some(room) = self.rooms.get(&channel_id)
            && let Some(session) = room.peers.get(&user_id)
        {
            match serde_json::from_str::<webrtc::ice_transport::ice_candidate::RTCIceCandidateInit>(
                &candidate,
            ) {
                Ok(s) => {
                    let _ = session.connection.add_ice_candidate(s).await;
                }
                Err(e) => tracing::error!("Failed to parse ICE candidate JSON: {}", e),
            }
        }

        tracing::info!("Received ICE Candidate from user {}", user_id.0);
        Ok(())
    }

    pub async fn accept_answer(
        &self,
        user_id: UserId,
        _answer_sdp: String,
    ) -> Result<(), AppError> {
        tracing::info!("Received SDP Answer from user {}", user_id.0);
        Ok(())
    }
}
