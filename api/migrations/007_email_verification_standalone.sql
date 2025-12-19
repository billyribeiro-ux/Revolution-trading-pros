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

-- Create superadmin user (password: Jesusforever01!)
-- BCrypt hash with cost 10
INSERT INTO users (email, password, name, role, email_verified_at, created_at, updated_at)
VALUES (
    'welberribeirodrums@gmail.com',
    '$2b$10$phx6SlBNC.zOAjORaWe7nuib4xKmD9YQrmC.sMpbt0.PLwIF2aLrq',
    'Welber Ribeiro',
    'super_admin',
    NOW(),
    NOW(),
    NOW()
)
ON CONFLICT (email) DO UPDATE SET
    role = 'super_admin',
    email_verified_at = COALESCE(users.email_verified_at, NOW()),
    password = EXCLUDED.password,
    updated_at = NOW();

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

