-- Create trading_rooms table
-- ICT 11+ Principal Engineer: Trading room management

CREATE TABLE IF NOT EXISTS trading_rooms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index for lookups
CREATE INDEX IF NOT EXISTS idx_trading_rooms_slug ON trading_rooms(slug);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_is_active ON trading_rooms(is_active);
