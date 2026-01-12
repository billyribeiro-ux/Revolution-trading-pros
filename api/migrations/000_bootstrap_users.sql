-- Bootstrap migration: Create users table if it doesn't exist
-- This handles fresh databases without Laravel migrations

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    email_verified_at TIMESTAMP,
    avatar_url VARCHAR(500),
    mfa_enabled BOOLEAN DEFAULT false,
    remember_token VARCHAR(100),
    stripe_customer_id VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
