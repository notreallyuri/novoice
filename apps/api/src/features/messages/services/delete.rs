use crate::core::{broadcast, error::AppError, state::SharedState};
use sea_orm::EntityTrait;
use shared::{
    data::{ChannelId, MessageId, UserId},
    ws::{ServerMessage, message::ChatServerEvents},
};
use uuid::Uuid;

pub async fn delete(
    state: &SharedState,
    channel_id: ChannelId,
    message_id: MessageId,
    author_id: UserId,
) -> Result<(), AppError> {
    entity::channel::Entity::find_by_id(channel_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let res = state
        .scylla
        .session
        .execute_unpaged(
            &state.scylla.statements.select_message_author,
            (channel_id.0, message_id.0),
        )
        .await?
        .into_rows_result()?;

    let stored_author = res.rows::<(Uuid,)>()?.next().ok_or(AppError::NotFound)??.0;

    if stored_author != author_id.0 {
        return Err(AppError::Forbidden(
            "Você não tem permissão para deletar esta mensagem".into(),
        ));
    }

    state
        .scylla
        .session
        .execute_unpaged(
            &state.scylla.statements.soft_delete_message,
            (channel_id.0, message_id.0),
        )
        .await?;

    let payload = ServerMessage::Chat(ChatServerEvents::Deleted {
        channel_id,
        message_id,
    });

    broadcast::to_channel(&state.redis.messages, &channel_id, &payload).await?;

    Ok(())
}
