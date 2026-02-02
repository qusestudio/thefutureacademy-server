-- Add migration script here
CREATE TABLE IF NOT EXISTS instructors (
    id SERIAL PRIMARY KEY,
    cognito_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT
);