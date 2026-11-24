-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(120) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    remarks TEXT,
    token TEXT,
    salt VARCHAR(32),
    remember_me boolean DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    created_by VARCHAR(36),
    updated_at TIMESTAMPTZ,
    updated_by VARCHAR(36)
    CONSTRAINT user_status_known CHECK(status IN ('active','inactive','suspended'))
);

CREATE INDEX idx_users_username ON users (username);
