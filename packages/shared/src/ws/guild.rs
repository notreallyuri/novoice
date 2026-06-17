use crate::data::{
    CategoryId, ChannelId, GuildId, UserId,
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
    MemberLeft {
        guild_id: GuildId,
        user_id: UserId,
    },
    CategoryCreated {
        guild_id: GuildId,
        category: Box<ChannelCategory>,
    },
    CategoryUpdated {
        guild_id: GuildId,
        category: Box<ChannelCategory>,
    },
    CategoryDeleted {
        guild_id: GuildId,
        category_id: CategoryId,
    },
    ChannelCreated {
        guild_id: GuildId,
        channel: Box<Channel>,
    },
    ChannelUpdated {
        guild_id: GuildId,
        channel: Box<Channel>,
    },
    ChannelDeleted {
        guild_id: GuildId,
        channel_id: ChannelId,
    },
    Deleted {
        guild_id: GuildId,
    },
}
