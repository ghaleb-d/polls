CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    user_creation_time TIMESTAMP NOT NULL,
    voted_polls UUID[] NOT NULL DEFAULT '{}'
);

