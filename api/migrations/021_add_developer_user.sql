-- Migration: Add developer user account
-- User: welberribeirodrums@gmail.com with developer role

-- First, try to update if exists, then insert if not
INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
VALUES (
    'welberribeirodrums@gmail.com',
    '$argon2id$v=19$m=65536,t=3,p=4$grC3p39Jr39Ez1bJNDsZvg$lCcKEJ/tR77zJ+mc//c/yhUGfd57nP07a2sKK+H3uI4',
    'Billy Ribeiro',
    'developer',
    NOW(),
    NOW(),
    NOW()
)
ON CONFLICT (email) DO UPDATE SET
    password_hash = '$argon2id$v=19$m=65536,t=3,p=4$grC3p39Jr39Ez1bJNDsZvg$lCcKEJ/tR77zJ+mc//c/yhUGfd57nP07a2sKK+H3uI4',
    role = 'developer',
    email_verified_at = COALESCE(users.email_verified_at, NOW()),
    updated_at = NOW();
