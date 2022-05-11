CREATE TABLE IF NOT EXISTS config(
    guild_id BIGSERIAL PRIMARY KEY,
    is_same_person BOOLEAN NOT NULL DEFAULT FALSE,
    already_setupped BOOLEAN NOT NULL DEFAULT FALSE,
    channel_id INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS logger(
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    ruined_chain_id BIGINT NOT NULL,
    ruiner_id BIGINT NOT NULL,
    ruined_jump_url TEXT NOT NULL,
    ruined_content TEXT NOT NULL,
    when_ruined TIMESTAMP NOT NULL,
    reason TEXT NOT NULL,
    previous_chain_id BIGINT NOT NULL
);
CREATE TABLE IF NOT EXISTS counting(
    guild_id BIGSERIAL PRIMARY KEY,
    current_alphabet TEXT,
    expected_next_alphabet TEXT NOT NULL,
    previous_alphabet TEXT,
    previous_person BIGINT
)