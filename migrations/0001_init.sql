-- Migration number: 0001 	 2025-11-19T18:23:56.763Z
 PRAGMA foreign_keys = ON;

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    discord_user_id TEXT NOT NULL UNIQUE,
    discord_username_cache TEXT NOT NULL,
    last_draft_lang TEXT NOT NULL DEFAULT 'en-US'
) STRICT;

CREATE TABLE draft_skills (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    emoji TEXT,
    description TEXT,
    flags INTEGER NOT NULL DEFAULT 0
) STRICT;
CREATE INDEX idx_draft_skills_user_id ON draft_skills(user_id);

CREATE TABLE campaigns (
    id INTEGER PRIMARY KEY,
    snowflake TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    lang TEXT NOT NULL DEFAULT 'en-US',
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE TABLE gms (
    campaign_id INTEGER NOT NULL REFERENCES campaigns (id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    flags INTEGER NOT NULL DEFAULT 0,
    last_viewed_at INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (campaign_id, user_id)
) STRICT WITHOUT ROWID;
CREATE INDEX idx_gms_user_id ON gms(user_id);

CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    emoji TEXT,
    description TEXT,
    flags INTEGER NOT NULL DEFAULT 0
) STRICT;
CREATE INDEX idx_skills_campaign_id ON skills(campaign_id);
