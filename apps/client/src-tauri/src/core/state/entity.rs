use dashmap::DashMap;
use shared::data::{
    channel::prelude::DMChannel,
    guild::Guild,
    relationship::UserRelationship,
    user::{User, UserPublic},
    ChannelId, GuildId, UserId,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

#[derive(Default, Clone)]
pub struct EntityCache {
    pub current_user: Arc<RwLock<Option<User>>>,

    pub messages: Arc<DashMap<ChannelId, Vec<Message>>>,

    pub guilds: Arc<DashMap<GuildId, Guild>>,
    pub dm_channels: Arc<DashMap<ChannelId, DMChannel>>,
    pub relationships: Arc<DashMap<UserId, UserRelationship>>,
    pub users: Arc<DashMap<UserId, UserPublic>>,
}
