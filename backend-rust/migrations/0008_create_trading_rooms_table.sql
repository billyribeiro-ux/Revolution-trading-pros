-- Trading Rooms Table Migration

CREATE TABLE IF NOT EXISTS trading_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    external_url VARCHAR(500),
    sso_secret VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_trading_rooms_slug ON trading_rooms(slug);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_active ON trading_rooms(is_active);

-- Room Access (which memberships grant access to which rooms)
CREATE TABLE IF NOT EXISTS trading_room_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trading_room_id UUID NOT NULL REFERENCES trading_rooms(id) ON DELETE CASCADE,
    membership_plan_id UUID NOT NULL REFERENCES membership_plans(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_room_access_unique ON trading_room_access(trading_room_id, membership_plan_id);

-- Room Traders
CREATE TABLE IF NOT EXISTS room_traders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trading_room_id UUID NOT NULL REFERENCES trading_rooms(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Room Daily Videos
CREATE TABLE IF NOT EXISTS room_daily_videos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trading_room_id UUID NOT NULL REFERENCES trading_rooms(id) ON DELETE CASCADE,
    trader_id UUID REFERENCES room_traders(id) ON DELETE SET NULL,
    title VARCHAR(500) NOT NULL,
    video_url VARCHAR(500) NOT NULL,
    thumbnail_url VARCHAR(500),
    duration INTEGER,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_room_videos_room ON room_daily_videos(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_room_videos_date ON room_daily_videos(published_at DESC);
