use crate::core::{
    broadcast, error::AppError, state::SharedState, statements::get_bucket_from_uuidv7,
};
use scylla::value::CqlTimestamp;
use sea_orm::EntityTrait;
use shared::{
    data::{ChannelId, MessageId, UserId, channel::message::Message},
    ws::{ServerMessage, message::ChatServerEvents},
};
use uuid::Uuid;

pub async fn edit(
    state: &SharedState,
    channel_id: ChannelId,
    message_id: MessageId,
    content: String,
    author_id: UserId,
) -> Result<(), AppError> {
    entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let now = chrono::Utc::now();
    let bucket = get_bucket_from_uuidv7(message_id.0);

    let res = state
        .scylla
        .session
        .execute_unpaged(
            &state.scylla.statements.select_message_for_edit,
            (channel_id.0, bucket, message_id.0),
        )
        .await?
        .into_rows_result()?;

    let (created_at, stored_author, stored_thread) = res
        .rows::<(CqlTimestamp, Uuid, Option<Uuid>)>()?
        .next()
        .ok_or(AppError::NotFound)??;

    if stored_author != author_id.0 {
        return Err(AppError::Forbidden(
            "Cannot edit another user's message".into(),
        ));
    }

    state
        .scylla
        .session
        .execute_unpaged(
            &state.scylla.statements.update_message_content,
            (
                content.clone(),
                CqlTimestamp(now.timestamp_millis()),
                channel_id.0,
                bucket,
                message_id.0,
            ),
        )
        .await?;

    let message = Message {
        id: message_id,
        channel_id,
        author_id,
        content,
        created_at: chrono::DateTime::from_timestamp_millis(created_at.0).unwrap_or_default(),
        edited_at: Some(now),
        thread_id: stored_thread.map(MessageId),
        deleted: false,
    };

    let event = ServerMessage::Chat(ChatServerEvents::Edited {
        message: Box::new(message),
    });

    broadcast::to_channel(&state.redis.messages, &channel_id, &event).await?;

    Ok(())
}
