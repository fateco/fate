-- Migration number: 0003 	 2025-11-19T14:57:10.488Z
CREATE TABLE gm_assistants (
    campaign_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    PRIMARY KEY (campaign_id, user_id),
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
