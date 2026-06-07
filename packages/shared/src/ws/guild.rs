use crate::data::{
    GuildId,
    channel::prelude::{Channel, ChannelCategory},
    guild::{Guild, GuildMember},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum GuildServerEvents {
    Created {
        guild: Box<Guild>,
    },
    Joined {
        guild: Box<Guild>,
    },
    MemberJoined {
        guild_id: GuildId,
        member: Box<GuildMember>,
    },
    CategoryCreated {
        guild_id: GuildId,
        category: Box<ChannelCategory>,
    },
    ChannelCreated {
        guild_id: GuildId,
        channel: Box<Channel>,
    },
    Deleted {
        guild_id: GuildId,
    },
}
