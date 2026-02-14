-- Add migration script here
CREATE TABLE IF NOT EXISTS student_profiles (
    id VARCHAR(255) PRIMARY KEY,
    student_id VARCHAR(255) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    date_of_birth VARCHAR(20) NOT NULL,
    school_name VARCHAR(255) NOT NULL,
    grade INTEGER NOT NULL
);