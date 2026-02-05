-- Add migration script here
CREATE TABLE IF NOT EXISTS subjects (
    id VARCHAR(255) PRIMARY KEY,
    instructor_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    grade INTEGER NOT NULL,
    term INTEGER NOT NULL
);