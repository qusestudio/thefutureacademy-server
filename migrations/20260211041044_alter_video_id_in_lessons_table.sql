-- Add migration script here
ALTER TABLE lessons
ALTER COLUMN video_id TYPE VARCHAR(255);
