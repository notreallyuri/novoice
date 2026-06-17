CREATE KEYSPACE IF NOT EXISTS chat
WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

USE chat;

CREATE TABLE IF NOT EXISTS messages (
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
) WITH CLUSTERING ORDER BY (id DESC);

CREATE MATERIALIZED VIEW IF NOT EXISTS thread_messages AS
    SELECT * FROM messages
    WHERE thread_id IS NOT NULL 
      AND channel_id IS NOT NULL 
      AND bucket IS NOT NULL 
      AND id IS NOT NULL
    PRIMARY KEY ((thread_id), channel_id, bucket, id)
    WITH CLUSTERING ORDER BY (channel_id ASC, bucket ASC, id DESC);
