-- Migration number: 0002 	 2025-11-19T08:41:32.794Z
CREATE TABLE skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    emoji TEXT,
    description TEXT
);