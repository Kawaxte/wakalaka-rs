CREATE TABLE IF NOT EXISTS guild_members (
    user_id BIGINT PRIMARY KEY,
    deaf BOOLEAN NOT NULL DEFAULT FALSE,
    mute BOOLEAN NOT NULL DEFAULT FALSE,
    timeout BOOLEAN NOT NULL DEFAULT FALSE,
    ban BOOLEAN NOT NULL DEFAULT FALSE,
    communication_disabled_until TIMESTAMP,
    guild_id BIGINT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(guild_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);