-- Add expires_at column to api_tokens for token TTL support
ALTER TABLE api_tokens ADD COLUMN expires_at INTEGER;
