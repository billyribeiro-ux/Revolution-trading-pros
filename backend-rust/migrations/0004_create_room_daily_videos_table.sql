-- Create room_daily_videos table
-- ICT 11+ Principal Engineer: Daily video content for trading rooms

CREATE TABLE IF NOT EXISTS room_daily_videos (
    id BIGSERIAL PRIMARY KEY,
    trading_room_id BIGINT REFERENCES trading_rooms(id) ON DELETE CASCADE,
    trader_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500),
    description TEXT,
    video_url TEXT NOT NULL,
    thumbnail_url TEXT,
    duration INTEGER,
    is_published BOOLEAN NOT NULL DEFAULT true,
    is_featured BOOLEAN NOT NULL DEFAULT false,
    views_count INTEGER NOT NULL DEFAULT 0,
    published_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_room_videos_trading_room_id ON room_daily_videos(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_room_videos_trader_id ON room_daily_videos(trader_id);
CREATE INDEX IF NOT EXISTS idx_room_videos_is_published ON room_daily_videos(is_published);
CREATE INDEX IF NOT EXISTS idx_room_videos_published_at ON room_daily_videos(published_at DESC);
