use crate::core::mappers::FromDomain;
use entity::{
    category::Model as CategoryModel,
    channel::{DbChannelKind, DbChannelMode, Model as ChannelModel},
};
use shared::data::{
    CategoryId, ChannelId, GuildId,
    channel::{
        Channel,
        canvas_channel::CanvasChannel,
        category::ChannelCategory,
        docs_channel::DocsChannel,
        text_channel::{ChannelMode, TextChannel},
        voice_channel::VoiceChannel,
    },
};

impl FromDomain<CategoryModel> for ChannelCategory {
    fn from_domain(model: CategoryModel) -> Self {
        ChannelCategory {
            id: CategoryId(model.id),
            guild_id: GuildId(model.guild_id),
            name: model.name,
            position: model.position,
        }
    }
}

impl FromDomain<ChannelModel> for Channel {
    fn from_domain(model: ChannelModel) -> Self {
        let channel_id = ChannelId(model.id);
        let guild_id = GuildId(model.guild_id);
        let category_id = model.category_id.map(CategoryId);

        match model.kind {
            DbChannelKind::Text => Channel::Text(TextChannel {
                id: channel_id,
                guild_id,
                category_id,
                name: model.name,
                position: model.position,
                mode: match model.mode {
                    Some(DbChannelMode::Board) => ChannelMode::Board,
                    Some(DbChannelMode::Threads) => ChannelMode::Threads,
                    _ => ChannelMode::Chat,
                },
            }),
            DbChannelKind::Voice => Channel::Voice(VoiceChannel {
                id: channel_id,
                guild_id,
                category_id,
                name: model.name,
                position: model.position,
                user_limit: model.user_limit,
                bitrate: model.bitrate.unwrap_or(64_000),
                participants: vec![],
            }),
            DbChannelKind::Docs => Channel::Docs(DocsChannel {
                id: channel_id,
                guild_id,
                category_id,
                name: model.name,
                position: model.position,
            }),
            DbChannelKind::Canvas => Channel::Canvas(CanvasChannel {
                id: channel_id,
                guild_id,
                category_id,
                name: model.name,
                position: model.position,
            }),
        }
    }
}
