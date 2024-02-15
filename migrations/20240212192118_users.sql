CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT PRIMARY KEY,
    violations INTEGER NOT NULL 0,
    timeout BOOLEAN DEFAULT FALSE,
    ban BOOLEAN DEFAULT FALSE
);