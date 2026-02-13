-- Add migration script here
CREATE TABLE IF NOT EXISTS enrollments (
    id VARCHAR(255) PRIMARY KEY,
    subject_id VARCHAR(255) NOT NULL,
    student_id VARCHAR(255) NOT NULL
);
