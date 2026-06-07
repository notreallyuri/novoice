CREATE KEYSPACE IF NOT EXISTS chat
WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

USE chat;

CREATE TABLE IF NOT EXISTS messages (
    channel_id        UUID,
    id                UUID,
    created_at        TIMESTAMP,
    author_id         UUID,
    content           TEXT,
    edited_at         TIMESTAMP,
    deleted           BOOLEAN,
    thread_id         UUID,

    PRIMARY KEY ((channel_id), id)
) WITH CLUSTERING ORDER BY (id DESC);
    
CREATE INDEX messages_thread_id_idx ON messages (thread_id);
