use crate::core::error::AppError;
use chrono::{Datelike, TimeZone, Utc};
use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use uuid::Uuid;

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

pub async fn initialize_schema(session: &Session) -> Result<(), AppError> {
    tracing::info!("Ensuring ScyllaDB schema is initialized...");

    let schema_statements = [
        "CREATE KEYSPACE IF NOT EXISTS chat WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};",
        "USE chat;",
        "CREATE TABLE IF NOT EXISTS messages (
            channel_id        UUID,
            bucket            INT,
            id                UUID,
            created_at        TIMESTAMP,
            author_id         UUID,
            content           TEXT,
            edited_at         TIMESTAMP,
            deleted           BOOLEAN,
            thread_id         UUID,
            PRIMARY KEY ((channel_id, bucket), id)
        ) WITH CLUSTERING ORDER BY (id DESC);",
        "CREATE MATERIALIZED VIEW IF NOT EXISTS thread_messages AS
            SELECT * FROM messages
            WHERE thread_id IS NOT NULL 
              AND channel_id IS NOT NULL 
              AND bucket IS NOT NULL 
              AND id IS NOT NULL
            PRIMARY KEY ((thread_id), channel_id, bucket, id)
            WITH CLUSTERING ORDER BY (channel_id ASC, bucket ASC, id DESC);",
    ];

    for stmt in schema_statements {
        session.query_unpaged(stmt, &[]).await?;
    }

    tracing::info!("ScyllaDB schema is ready.");
    Ok(())
}

pub fn get_bucket_from_uuidv7(id: Uuid) -> i32 {
    let (secs, nanos) = id.get_timestamp().expect("Invalid UUIDv7").to_unix();
    let dt = Utc.timestamp_opt(secs as i64, nanos).unwrap();
    dt.year() * 100 + dt.month() as i32
}

pub fn current_bucket() -> i32 {
    let now = Utc::now();
    now.year() * 100 + now.month() as i32
}

pub fn previous_bucket(bucket: i32) -> i32 {
    let year = bucket / 100;
    let month = bucket % 100;
    if month == 1 {
        (year - 1) * 100 + 12
    } else {
        bucket - 1
    }
}

impl ScyllaStatements {
    pub async fn prepare(session: &Session) -> Result<Self, AppError> {
        Ok(Self {
            insert_message: session
                .prepare(
                    "INSERT INTO messages \
                         (channel_id, bucket, id, created_at, author_id, content, deleted, thread_id) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .await?,

            select_message_author: session
                .prepare(
                    "SELECT author_id \
                     FROM messages \
                     WHERE channel_id = ? AND bucket = ? AND id = ?",
                )
                .await?,

            select_message_for_edit: session
                .prepare(
                    "SELECT created_at, author_id, thread_id \
                     FROM messages \
                     WHERE channel_id = ? AND bucket = ? AND id = ?",
                )
                .await?,

            soft_delete_message: session
                .prepare(
                    "UPDATE messages \
                     SET deleted = true \
                     WHERE channel_id = ? AND bucket = ? AND id = ?",
                )
                .await?,

            update_message_content: session
                .prepare(
                    "UPDATE messages \
                     SET content = ?, edited_at = ? \
                     WHERE channel_id = ? AND bucket = ? AND id = ?",
                )
                .await?,

            get_messages: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? AND bucket = ? \
                     LIMIT ?",
                )
                .await?,

            get_messages_before: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM messages \
                     WHERE channel_id = ? AND bucket = ? AND id < ? \
                     LIMIT ?",
                )
                .await?,

            get_messages_by_thread: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM thread_messages \
                     WHERE thread_id = ? \
                     LIMIT ?",
                )
                .await?,

            get_messages_by_thread_before: session
                .prepare(
                    "SELECT id, channel_id, author_id, content, created_at, edited_at, thread_id, deleted \
                     FROM thread_messages \
                     WHERE thread_id = ? AND (channel_id, bucket, id) < (?, ?, ?) \
                     LIMIT ?",
                )
                .await?,
        })
    }
}
