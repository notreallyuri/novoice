use crate::core::{broadcast, error::AppError, state::SharedState};
use sea_orm::EntityTrait;
use shared::{
    data::{ChannelId, GuildId, MessageId, UserId, channel::message::PinnedMessage},
    ws::{ServerMessage, message::ChatServerEvents},
};

pub async fn pin(
    state: &SharedState,
    channel_id: ChannelId,
    message_id: MessageId,
    label: Option<String>,
    author_id: UserId,
) -> Result<(), AppError> {
    let channel = entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let guild_id = GuildId(channel.guild_id);

    let pin = PinnedMessage {
        message_id,
        pinned_by: author_id,
        pinned_at: chrono::Utc::now(),
        label,
    };

    let payload = ServerMessage::Chat(ChatServerEvents::Pinned {
        channel_id,
        pin: Box::new(pin),
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &payload).await?;

    Ok(())
}

pub async fn unpin(
    state: &SharedState,
    channel_id: ChannelId,
    message_id: MessageId,
) -> Result<(), AppError> {
    let channel = entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let guild_id = GuildId(channel.guild_id);

    let payload = ServerMessage::Chat(ChatServerEvents::Unpinned {
        channel_id,
        message_id,
    });

    broadcast::to_guild(&state.redis.messages, &guild_id, &payload).await?;

    Ok(())
}
