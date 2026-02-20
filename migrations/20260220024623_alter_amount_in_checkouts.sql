-- Add migration script here
ALTER TABLE checkouts
ALTER COLUMN amount TYPE INT8;