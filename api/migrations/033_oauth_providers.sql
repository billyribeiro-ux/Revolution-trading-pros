-- ============================================================================
-- OAuth Providers Migration - January 2026
-- Apple Principal Engineer ICT Level 7 Grade
-- ============================================================================
-- Adds OAuth provider fields to users table for Google and Apple Sign-In
-- Follows Apple's security best practices for token storage and user linking
-- ============================================================================

-- Add OAuth provider columns to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS google_id VARCHAR(255) UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS apple_id VARCHAR(255) UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_provider VARCHAR(50);
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_linked_at TIMESTAMP;
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_metadata JSONB DEFAULT '{}';

-- Create index for OAuth provider lookups (performance critical)
CREATE INDEX IF NOT EXISTS idx_users_google_id ON users(google_id) WHERE google_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_users_apple_id ON users(apple_id) WHERE apple_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_users_oauth_provider ON users(oauth_provider) WHERE oauth_provider IS NOT NULL;

-- OAuth state tokens table for CSRF protection
-- ICT 7: State tokens expire after 10 minutes to prevent replay attacks
CREATE TABLE IF NOT EXISTS oauth_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    state VARCHAR(255) UNIQUE NOT NULL,
    provider VARCHAR(50) NOT NULL,
    redirect_uri TEXT,
    code_verifier VARCHAR(128), -- PKCE for enhanced security
    nonce VARCHAR(255), -- For Apple Sign-In ID token validation
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT (NOW() + INTERVAL '10 minutes'),
    used_at TIMESTAMP
);

-- Index for state lookups
CREATE INDEX IF NOT EXISTS idx_oauth_states_state ON oauth_states(state);
CREATE INDEX IF NOT EXISTS idx_oauth_states_expires ON oauth_states(expires_at);

-- OAuth refresh tokens table (for provider token refresh if needed)
-- ICT 7: Encrypted storage for external provider refresh tokens
CREATE TABLE IF NOT EXISTS oauth_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    access_token_encrypted TEXT, -- Encrypted with server key
    refresh_token_encrypted TEXT, -- Encrypted with server key
    token_expires_at TIMESTAMP,
    scope TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, provider)
);

-- Index for user token lookups
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_user_provider ON oauth_tokens(user_id, provider);

-- OAuth account linking audit log
-- ICT 11+: Full audit trail for security compliance
CREATE TABLE IF NOT EXISTS oauth_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    provider VARCHAR(50) NOT NULL,
    action VARCHAR(50) NOT NULL, -- 'link', 'unlink', 'login', 'register'
    provider_user_id VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT NOW()
);

-- Index for audit log queries
CREATE INDEX IF NOT EXISTS idx_oauth_audit_log_user ON oauth_audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_audit_log_created ON oauth_audit_log(created_at);

-- Cleanup function for expired OAuth states
-- ICT 7: Automatic cleanup prevents state table bloat
CREATE OR REPLACE FUNCTION cleanup_expired_oauth_states()
RETURNS void AS $$
BEGIN
    DELETE FROM oauth_states WHERE expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- Comment documentation
COMMENT ON COLUMN users.google_id IS 'Google OAuth2 subject identifier (sub claim)';
COMMENT ON COLUMN users.apple_id IS 'Apple Sign-In subject identifier (sub claim)';
COMMENT ON COLUMN users.oauth_provider IS 'Primary OAuth provider used for registration: google, apple, or null for email';
COMMENT ON COLUMN users.oauth_linked_at IS 'Timestamp when OAuth was first linked to this account';
COMMENT ON COLUMN users.oauth_metadata IS 'Additional OAuth provider data (profile picture URL, locale, etc.)';
COMMENT ON TABLE oauth_states IS 'CSRF protection states for OAuth flow - auto-expire after 10 minutes';
COMMENT ON TABLE oauth_tokens IS 'Encrypted OAuth provider tokens for API access (optional)';
COMMENT ON TABLE oauth_audit_log IS 'Security audit trail for OAuth operations';
