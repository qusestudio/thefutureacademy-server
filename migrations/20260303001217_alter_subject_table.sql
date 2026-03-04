-- Add migration script here
ALTER TABLE subjects
DROP COLUMN IF EXISTS instructor_id;