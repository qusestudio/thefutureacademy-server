-- Add migration script here
ALTER TABLE subjects
ADD COLUMN image TEXT,
ADD COLUMN description TEXT[];