use crate::core::error::AppError;
use scylla::{client::session::Session, statement::prepared::PreparedStatement};

pub struct ScyllaStatements {
    pub insert_message: PreparedStatement,
    pub select_message_author: PreparedStatement,
    pub select_message_for_edit: PreparedStatement,
    pub soft_delete_message: PreparedStatement,
    pub update_message_content: PreparedStatement,

    pub get_messages: PreparedStatement,
    pub get_messages_before: PreparedStatement,
    pub get_messages_by_thread: PreparedStatement,
    pub get_messages_by_thread_before: PreparedStatement,
}


impl ScyllaStatements {
    pub async fn prepare(session: &Session) -> Result<Self, AppError> {
        Ok(Self {
            insert_message: session
                .prepare(
                    "INSERT INTO messages \
                         (channel_id, created_at, id, author_id, content, deleted, thread_id) \
                     VALUES (?, ?, ?, ?, ?, ?, ?)",
                )
                .await?,
 
            select_message_author: session
                .prepare(
                    "SELECT author_id \
                     FROM messages \
                     WHERE channel_id = ? AND id = ?",
                )
                .await?,
 
            select_message_for_edit: session
                .prepare(
                    "SELECT created_at, author_id, thread_id \
                     FROM messages \
                     WHERE channel_id = ? AND id = ?",
                )
                .await?,
 
            soft_delete_message: session
                .prepare(
                    "UPDATE messages \
                     SET deleted = true \
                     WHERE channel_id = ? AND id = ?",
                )
                .await?,
 
            update_message_content: session
                .prepare(
                    "UPDATE messages \
                     SET content = ?, edited_at = ? \
                     WHERE channel_id = ? AND id = ?",
                )
                .await?,
 
            get_messages: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? \
                     LIMIT ?",
                )
                .await?,
 
            get_messages_before: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? AND id < ? \
                     LIMIT ?",
                )
                .await?,
 
            get_messages_by_thread: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? AND thread_id = ? \
                     LIMIT ? ALLOW FILTERING",
                )
                .await?,
 
            get_messages_by_thread_before: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? AND thread_id = ? AND id < ? \
                     LIMIT ? ALLOW FILTERING",
                )
                .await?,
        })
    }
}
