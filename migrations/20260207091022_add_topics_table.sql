-- Add migration script here
CREATE TABLE IF NOT EXISTS topics (
    id VARCHAR(255) PRIMARY KEY,
    subject_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL
);