use crate::core::{error::AppError, response::PaginatedData, state::SharedState};
use scylla::value::CqlTimestamp;
use shared::{
    data::{ChannelId, MessageId, UserId, channel::message::Message},
    dtos::message::MessageQueryParams,
};
use uuid::Uuid;

pub async fn get_messages(
    state: &SharedState,
    channel_id: ChannelId,
    query: MessageQueryParams,
) -> Result<PaginatedData<Message>, AppError> {
    let limit = query.limit.unwrap_or(50).min(100) as i32;

    let before_id = query.before.map(|b| b.0);
    let thread_id = query.thread_id.map(|t| t.0);

    let query_result = match (before_id, thread_id) {
        (Some(b_id), Some(t_id)) => {
            state
                .scylla
                .session
                .execute_unpaged(
                    &state.scylla.statements.get_messages_by_thread_before,
                    (channel_id.0, t_id, b_id, limit),
                )
                .await?
        }
        (Some(b_id), None) => {
            state
                .scylla
                .session
                .execute_unpaged(
                    &state.scylla.statements.get_messages_before,
                    (channel_id.0, b_id, limit),
                )
                .await?
        }
        (None, Some(t_id)) => {
            state
                .scylla
                .session
                .execute_unpaged(
                    &state.scylla.statements.get_messages_by_thread,
                    (channel_id.0, t_id, limit),
                )
                .await?
        }
        (None, None) => {
            state
                .scylla
                .session
                .execute_unpaged(&state.scylla.statements.get_messages, (channel_id.0, limit))
                .await?
        }
    };

    let rows = query_result.into_rows_result()?;

    let mut messages = Vec::new();

    for row in rows.rows::<(
        Uuid,
        Uuid,
        Uuid,
        String,
        CqlTimestamp,
        Option<CqlTimestamp>,
        Option<Uuid>,
        bool,
    )>()? {
        let (id, c_id, a_id, content, created_at, edited_at, t_id, deleted) = row?;

        let created_at = chrono::DateTime::from_timestamp_millis(created_at.0).unwrap_or_default();
        let edited_at = edited_at.and_then(|ts| chrono::DateTime::from_timestamp_millis(ts.0));

        let final_content = if deleted { "".to_string() } else { content };

        messages.push(Message {
            id: MessageId(id),
            channel_id: ChannelId(c_id),
            author_id: UserId(a_id),
            content: final_content,
            created_at,
            edited_at,
            thread_id: t_id.map(MessageId),
            deleted,
        });
    }

    Ok(PaginatedData {
        total_items: messages.len() as u64,
        current_page: 1,
        total_pages: 1,
        data: messages,
    })
}
