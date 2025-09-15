-- Add migration script here
ALTER TABLE comments ADD COLUMN annotations JSONB;
