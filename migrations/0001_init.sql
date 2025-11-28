-- Migration number: 0001 	 2025-11-19T18:23:56.763Z
 PRAGMA foreign_keys = ON;

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    discord_user_id TEXT NOT NULL UNIQUE,
    discord_username_cache TEXT NOT NULL
) STRICT;

CREATE TABLE draft_campaigns (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    lang TEXT NOT NULL DEFAULT 'en-US',
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;
CREATE INDEX idx_draft_campaigns_user_id ON draft_campaigns(user_id);

CREATE TABLE draft_skills (
    id INTEGER PRIMARY KEY,
    draft_campaign_id INTEGER NOT NULL REFERENCES draft_campaigns (id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE,
    emoji TEXT,
    description TEXT,
    flags INTEGER NOT NULL DEFAULT 0
) STRICT;
CREATE INDEX idx_draft_skills_draft_campaign_id ON draft_skills(draft_campaign_id);
CREATE UNIQUE INDEX idx_draft_skills_name ON draft_skills(draft_campaign_id, name);

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
) STRICT;
CREATE INDEX idx_gms_user_id ON gms(user_id);

CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns (id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE,
    emoji TEXT,
    description TEXT,
    flags INTEGER NOT NULL DEFAULT 0
) STRICT;
CREATE INDEX idx_skills_campaign_id ON skills(campaign_id);
CREATE UNIQUE INDEX idx_skills_name ON skills(campaign_id, name);