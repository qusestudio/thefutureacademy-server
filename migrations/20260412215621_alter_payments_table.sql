-- Add migration script here
ALTER TABLE payments
DROP COLUMN status,
DROP COLUMN updated_at,
ADD COLUMN subscription_id VARCHAR(255),
ADD COLUMN amount_received BIGINT,
ADD COLUMN transaction_id VARCHAR(255);


