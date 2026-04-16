-- Add migration script here
ALTER TABLE checkouts
DROP COLUMN created_at,
DROP COLUMN updated_at,
DROP COLUMN month,
DROP COLUMN year,
DROP COLUMN amount,
ADD COLUMN plan_id VARCHAR(255),
ADD COLUMN gateway_reference VARCHAR(255);