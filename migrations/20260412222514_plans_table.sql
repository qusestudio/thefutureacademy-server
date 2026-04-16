-- Add migration script here
CREATE TABLE IF NOT EXISTS plans (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price BIGINT NOT NULL,
    duration INT NOT NULL,
    is_active BOOLEAN NOT NULL
);