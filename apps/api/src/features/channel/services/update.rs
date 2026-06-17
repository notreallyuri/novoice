use entity::channel::{self, DbChannelKind, DbChannelMode};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use shared::{
    data::{
        ChannelId, GuildId, UserId,
        channel::{Channel, prelude::ChannelMode},
        permissions::Permissions,
    },
    dtos::channel::UpdateChannelRequest,
    ws::{ServerMessage, guild::GuildServerEvents},
};

use crate::core::{
    broadcast, error::AppError, guards::verify_permission, mappers::IntoDomain, state::SharedState,
};

pub async fn update(
    state: SharedState,
    user_id: UserId,
    guild_id: GuildId,
    channel_id: ChannelId,
    req: UpdateChannelRequest,
) -> Result<Channel, AppError> {
    verify_permission(&state.db, user_id, guild_id, Permissions::MANAGE_CHANNELS).await?;

    let channel_model = channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if channel_model.guild_id != guild_id.0 {
        return Err(AppError::Forbidden(
            "Channel does not belong to this guild".into(),
        ));
    }

    let mut active_channel = channel_model.clone().into_active_model();
    let mut has_changes = false;

    if let Some(new_name) = req.name {
        active_channel.name = Set(new_name);
        has_changes = true;
    }

    if let Some(new_pos) = req.position {
        active_channel.position = Set(new_pos);
        has_changes = true;
    }

    if let Some(category_opt) = req.category_id {
        active_channel.category_id = Set(category_opt.map(|id| id.0));
        has_changes = true;
    }

    if let Some(mode) = req.mode
        && channel_model.kind == DbChannelKind::Text
    {
        let db_mode = match mode {
            ChannelMode::Chat => DbChannelMode::Chat,
            ChannelMode::Board => DbChannelMode::Board,
            ChannelMode::Threads => DbChannelMode::Threads,
        };
        active_channel.mode = Set(Some(db_mode));
        has_changes = true;
    }

    if let Some(bitrate) = req.bitrate
        && channel_model.kind == DbChannelKind::Voice
    {
        active_channel.bitrate = Set(Some(bitrate));
        has_changes = true;
    }

    if let Some(limit_opt) = req.user_limit
        && channel_model.kind == DbChannelKind::Voice
    {
        active_channel.user_limit = Set(limit_opt);
        has_changes = true;
    }

    let updated_model = if has_changes {
        active_channel.update(&state.db).await?
    } else {
        channel_model
    };

    let channel_dto: Channel = updated_model.into_domain();

    let event = ServerMessage::Guild(GuildServerEvents::ChannelUpdated {
        guild_id,
        channel: Box::new(channel_dto.clone()),
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &event).await?;

    Ok(channel_dto)
}
