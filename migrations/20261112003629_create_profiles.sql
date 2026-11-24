-- Add migration script here
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY,
    name VARCHAR(120) NOT NULL,
    email VARCHAR(120) NOT NULL,
    email_verified_at timestamptz,
    phone VARCHAR(20),
    phone_verified_at timestamptz,
    avatar VARCHAR(120),
    created_at timestamptz default current_timestamp,
    created_by VARCHAR(36),
    updated_at timestamptz,
    updated_by VARCHAR(36)
);
