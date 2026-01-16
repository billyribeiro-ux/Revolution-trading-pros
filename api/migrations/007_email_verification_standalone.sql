-- Standalone Email Verification Tables
-- Migration 007: Creates email_verification_tokens table and superadmin
-- This migration is designed to run independently and will not fail if tables exist

-- Drop and recreate email_verification_tokens to ensure correct schema
DROP TABLE IF EXISTS email_verification_tokens CASCADE;

CREATE TABLE email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT NOT NULL,
    token VARCHAR(64) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_email_verification_user ON email_verification_tokens(user_id);
CREATE INDEX idx_email_verification_token ON email_verification_tokens(token);
CREATE INDEX idx_email_verification_expires ON email_verification_tokens(expires_at);

-- ICT 11+: NO HARDCODED CREDENTIALS IN MIGRATIONS
-- Bootstrap user creation is handled via DEVELOPER_BOOTSTRAP_* environment variables
-- See: api/src/db/mod.rs bootstrap_developer() function
-- Run: cargo run --bin bootstrap_dev to securely set up admin user

-- Ensure password_resets table has expires_at column
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'password_resets' AND column_name = 'expires_at'
    ) THEN
        ALTER TABLE password_resets ADD COLUMN expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '1 hour');
    END IF;
END $$;

