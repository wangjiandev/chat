-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    fullname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email ON users (email);

CREATE TYPE chat_type AS ENUM ('single', 'group', 'private_channel', 'public_channel');

CREATE TABLE IF NOT EXISTS chats (
    id bigserial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type chat_type NOT NULL,
    members BIGINT[] NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
    id bigserial PRIMARY KEY,
    chat_id BIGINT NOT NULL REFERENCES chats(id),
    sender_id BIGINT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    images TEXT[],
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_messages_chat_id_created_at ON messages (chat_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_messages_sender_id ON messages (sender_id, created_at DESC);
