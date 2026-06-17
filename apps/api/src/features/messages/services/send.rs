use crate::core::{broadcast, error::AppError, state::SharedState, statements::current_bucket};
use scylla::value::CqlTimestamp;
use sea_orm::EntityTrait;
use shared::{
    data::{ChannelId, MessageId, UserId, channel::message::Message},
    dtos::message::MessageSendRequest,
    ws::{ServerMessage, message::ChatServerEvents},
};
use uuid::Uuid;

pub async fn send(
    state: &SharedState,
    channel_id: ChannelId,
    author_id: UserId,
    payload: MessageSendRequest,
) -> Result<(), AppError> {
    entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let message_id = Uuid::now_v7();
    let now = chrono::Utc::now();
    let created_at_ms = now.timestamp_millis();

    state
        .scylla
        .session
        .execute_unpaged(
            &state.scylla.statements.insert_message,
            (
                channel_id.0,
                current_bucket(),
                message_id,
                CqlTimestamp(created_at_ms),
                author_id.0,
                payload.content.clone(),
                false,
                payload.thread_id.as_ref().map(|t| t.0),
            ),
        )
        .await?;

    let message = Message {
        id: MessageId(message_id),
        channel_id,
        author_id,
        content: payload.content,
        created_at: now,
        edited_at: None,
        thread_id: payload.thread_id,
        deleted: false,
    };

    let event = ServerMessage::Chat(ChatServerEvents::Received {
        message: Box::new(message),
    });

    broadcast::to_channel(&state.redis.messages, &channel_id, &event).await?;

    Ok(())
}
