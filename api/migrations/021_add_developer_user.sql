-- ═══════════════════════════════════════════════════════════════════════════════
-- Migration 021: Ensure Developer Account Exists
-- ═══════════════════════════════════════════════════════════════════════════════
-- ICT 7 Principal Engineer Grade - Secure, Idempotent, Auditable
--
-- Purpose: Ensure platform owner account exists with developer privileges
-- Security: Password hash is Argon2id with OWASP-recommended parameters
-- Audit: email_verified_at set to migration time for traceability
-- ═══════════════════════════════════════════════════════════════════════════════

-- Upsert developer account (idempotent - safe to run multiple times)
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
    password_hash = EXCLUDED.password_hash,
    role = 'developer',
    email_verified_at = COALESCE(users.email_verified_at, NOW()),
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════════
-- Security Notes (ICT 7):
-- - Argon2id parameters: m=65536 (64MB), t=3 iterations, p=4 parallelism
-- - Email is in DEVELOPER_EMAILS and SUPERADMIN_EMAILS env vars by default
-- - ON CONFLICT ensures idempotency - migration can safely re-run
-- - COALESCE preserves original verification timestamp if already verified
-- ═══════════════════════════════════════════════════════════════════════════════
