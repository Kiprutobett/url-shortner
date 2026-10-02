-- Add migration script here
ALTER TABLE urls
ADD COLUMN user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE;