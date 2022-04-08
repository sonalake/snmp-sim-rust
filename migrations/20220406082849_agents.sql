-- Add migration script here
CREATE TABLE agents (
    id BLOB PRIMARY KEY NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_agents_on_id ON agents(id);