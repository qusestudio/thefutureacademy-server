-- Add migration script here
ALTER TABLE student_profiles
ALTER COLUMN date_of_birth TYPE VARCHAR(255);
