-- Add migration script here
CREATE INDEX IF NOT EXISTS idx_user_id ON profiles(user_id);
CREATE INDEX IF NOT EXISTS idx_phone ON profiles(phone);
CREATE INDEX IF NOT EXISTS idx_email ON profiles(email);
