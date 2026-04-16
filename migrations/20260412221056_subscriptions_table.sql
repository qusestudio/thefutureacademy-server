-- Add migration script here
CREATE TABLE IF NOT EXISTS subscriptions(
    id TEXT PRIMARY KEY,
    student_id VARCHAR(255) NOT NULL,
    plan_id VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,
    cancel_at_period_end BOOLEAN NOT NULL
);