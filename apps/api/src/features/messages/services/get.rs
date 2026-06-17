use crate::core::{
    error::AppError,
    response::PaginatedData,
    state::SharedState,
    statements::{current_bucket, get_bucket_from_uuidv7, previous_bucket},
};
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
    let mut messages = Vec::new();

    if let Some(t_id) = query.thread_id {
        let query_result = if let Some(b_id) = query.before {
            let b_bucket = get_bucket_from_uuidv7(b_id.0);
            state
                .scylla
                .session
                .execute_unpaged(
                    &state.scylla.statements.get_messages_by_thread_before,
                    (t_id.0, channel_id.0, b_bucket, b_id.0, limit),
                )
                .await?
        } else {
            state
                .scylla
                .session
                .execute_unpaged(
                    &state.scylla.statements.get_messages_by_thread,
                    (t_id.0, limit),
                )
                .await?
        };

        let _ = process_rows(&mut messages, query_result)?;
    } else {
        let mut limit_remaining = limit;
        let mut current_bkt = query
            .before
            .map_or_else(current_bucket, |b| get_bucket_from_uuidv7(b.0));
        let mut before_id_opt = query.before.map(|b| b.0);

        let mut buckets_searched = 0;

        while limit_remaining > 0 && buckets_searched < 3 {
            let query_result = if let Some(b_id) = before_id_opt {
                state
                    .scylla
                    .session
                    .execute_unpaged(
                        &state.scylla.statements.get_messages_before,
                        (channel_id.0, current_bkt, b_id, limit_remaining),
                    )
                    .await?
            } else {
                state
                    .scylla
                    .session
                    .execute_unpaged(
                        &state.scylla.statements.get_messages,
                        (channel_id.0, current_bkt, limit_remaining),
                    )
                    .await?
            };

            let rows_fetched = process_rows(&mut messages, query_result)?;

            limit_remaining -= rows_fetched;
            current_bkt = previous_bucket(current_bkt);
            before_id_opt = None;
            buckets_searched += 1;
        }
    }

    Ok(PaginatedData {
        total_items: messages.len() as u64,
        current_page: 1,
        total_pages: 1,
        data: messages,
    })
}

fn process_rows(
    messages: &mut Vec<Message>,
    query_result: scylla::response::query_result::QueryResult,
) -> Result<i32, AppError> {
    let rows = query_result.into_rows_result()?;
    let mut count = 0;

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

        count += 1;
    }

    Ok(count)
}
