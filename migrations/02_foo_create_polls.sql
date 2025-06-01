CREATE TABLE IF NOT EXISTS polls (
    id UUID PRIMARY KEY,
    question TEXT NOT NULL,
    choices TEXT[] NOT NULL,
    vote_counts INTEGER[] NOT NULL,
    creation_time TIMESTAMP NOT NULL,
    deadline TIMESTAMP,
    created_by UUID REFERENCES users(id)
);
