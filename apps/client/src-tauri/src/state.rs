use dashmap::DashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::{
    data::{
        channel::{prelude::Message, Channel},
        guild::Guild,
        relationship::UserRelationship,
        user::{User, UserPublic},
        ChannelId, GuildId, UserId,
    },
    ws::ClientMessage,
};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::sync::{mpsc, Mutex};

#[derive(Default)]
pub struct Store {
    pub token: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewPanel {
    pub id: String,
    #[serde(rename = "type")]
    pub panel_type: String,
    pub target_id: String,
    pub title: String,
}

#[derive(Clone)]
pub struct Cache {
    pub current_panels: Arc<RwLock<Vec<ViewPanel>>>,
    pub current_guild_id: Arc<RwLock<Option<GuildId>>>,
    pub current_user: Arc<RwLock<Option<User>>>,

    pub guilds: Arc<DashMap<GuildId, Guild>>,
    pub channels: Arc<DashMap<ChannelId, Channel>>,
    pub relationships: Arc<DashMap<UserId, UserRelationship>>,

    pub messages: Arc<DashMap<ChannelId, Vec<Message>>>,
    pub users: Arc<DashMap<UserId, UserPublic>>,
}

pub struct AppState {
    pub ws: Arc<Mutex<Option<mpsc::UnboundedSender<ClientMessage>>>>,
    pub store: Arc<Mutex<Store>>,
    pub cache: Cache,
    pub http: Client,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            current_panels: Arc::new(RwLock::new(vec![
                ViewPanel {
                    id: "panel-1".to_string(),
                    panel_type: "channel".to_string(),
                    target_id: "c-123".to_string(),
                    title: "general".to_string(),
                },
                ViewPanel {
                    id: "panel-2".to_string(),
                    panel_type: "channel".to_string(),
                    target_id: "c-456".to_string(),
                    title: "dev-updates".to_string(),
                },
            ])),
            current_guild_id: Arc::new(RwLock::new(None)),
            current_user: Arc::new(RwLock::new(None)),
            guilds: Arc::new(DashMap::new()),
            channels: Arc::new(DashMap::new()),
            relationships: Arc::new(DashMap::new()),
            messages: Arc::new(DashMap::new()),
            users: Arc::new(DashMap::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            ws: Arc::new(Mutex::new(None)),
            store: Arc::new(Mutex::new(Store::default())),
            cache: Cache::default(),
            http,
        }
    }
}
