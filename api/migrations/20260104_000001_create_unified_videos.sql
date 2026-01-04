-- Unified Videos Table Migration
-- Apple ICT 11+ Principal Engineer Grade - January 2026
-- Supports: Daily Videos, Weekly Watchlist, Learning Center, Room Archives

-- Create enum types
DO $$ BEGIN
    CREATE TYPE video_content_type AS ENUM ('daily_video', 'weekly_watchlist', 'learning_center', 'room_archive');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE video_platform AS ENUM ('bunny', 'vimeo', 'youtube', 'wistia', 'jwplayer', 'direct');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Main unified_videos table
CREATE TABLE IF NOT EXISTS unified_videos (
    id BIGSERIAL PRIMARY KEY,
    
    -- Core Video Data
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    
    -- Video Source (Bunny.net primary)
    video_url VARCHAR(500) NOT NULL,
    video_platform video_platform DEFAULT 'bunny',
    video_id VARCHAR(100),
    bunny_video_guid VARCHAR(100),
    
    -- Thumbnail (Bunny Storage)
    thumbnail_url VARCHAR(500),
    thumbnail_path VARCHAR(500),
    
    -- Video Metadata
    duration INTEGER,
    quality VARCHAR(20) DEFAULT 'auto',
    
    -- Content Classification
    content_type video_content_type NOT NULL,
    difficulty_level VARCHAR(50),
    category VARCHAR(100),
    session_type VARCHAR(50),
    chapter_timestamps JSONB,
    
    -- Trader Association
    trader_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,
    
    -- Publishing
    video_date DATE NOT NULL,
    is_published BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    published_at TIMESTAMPTZ,
    scheduled_at TIMESTAMPTZ,
    
    -- Tags (for filtering)
    tags JSONB DEFAULT '[]'::jsonb,
    
    -- Analytics
    views_count INTEGER DEFAULT 0,
    likes_count INTEGER DEFAULT 0,
    completion_rate INTEGER DEFAULT 0,
    
    -- Bunny.net specific
    bunny_library_id BIGINT,
    bunny_encoding_status VARCHAR(50),
    bunny_thumbnail_url VARCHAR(500),
    
    -- Metadata & Audit
    metadata JSONB,
    created_by BIGINT,
    updated_by BIGINT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Video Room Assignments (Many-to-Many)
CREATE TABLE IF NOT EXISTS video_room_assignments (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    trading_room_id BIGINT NOT NULL REFERENCES trading_rooms(id) ON DELETE CASCADE,
    is_featured_in_room BOOLEAN DEFAULT false,
    is_pinned BOOLEAN DEFAULT false,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(video_id, trading_room_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_unified_videos_content_type ON unified_videos(content_type);
CREATE INDEX IF NOT EXISTS idx_unified_videos_video_date ON unified_videos(video_date DESC);
CREATE INDEX IF NOT EXISTS idx_unified_videos_is_published ON unified_videos(is_published);
CREATE INDEX IF NOT EXISTS idx_unified_videos_trader_id ON unified_videos(trader_id);
CREATE INDEX IF NOT EXISTS idx_unified_videos_bunny_guid ON unified_videos(bunny_video_guid);
CREATE INDEX IF NOT EXISTS idx_unified_videos_tags ON unified_videos USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_unified_videos_search ON unified_videos USING GIN(to_tsvector('english', title || ' ' || COALESCE(description, '')));

CREATE INDEX IF NOT EXISTS idx_video_room_assignments_video ON video_room_assignments(video_id);
CREATE INDEX IF NOT EXISTS idx_video_room_assignments_room ON video_room_assignments(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_video_room_assignments_featured ON video_room_assignments(trading_room_id, is_featured_in_room);

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_unified_videos_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_unified_videos_updated_at ON unified_videos;
CREATE TRIGGER trigger_unified_videos_updated_at
    BEFORE UPDATE ON unified_videos
    FOR EACH ROW
    EXECUTE FUNCTION update_unified_videos_updated_at();
