-- Add migration script here
CREATE TABLE IF NOT EXISTS instructor_profiles (
    id TEXT PRIMARY KEY,
    instructor_id TEXT NOT NULL,

    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    title TEXT NOT NULL,

    bio TEXT,
    profile_image_url TEXT,

    qualifications TEXT,
    years_of_experience INT,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);