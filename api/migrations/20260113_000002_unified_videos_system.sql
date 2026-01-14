-- ═══════════════════════════════════════════════════════════════════════════
-- UNIFIED VIDEO MANAGEMENT SYSTEM
-- ═══════════════════════════════════════════════════════════════════════════
-- Apple Principal Engineer ICT 7 Grade - January 2026
--
-- Comprehensive video system supporting:
-- - Daily Videos
-- - Weekly Watchlist Videos
-- - Learning Center Videos (with thumbnails & tags)
-- - Room Archive Videos
--
-- Platform Support: Bunny.net (primary), Vimeo, YouTube, Wistia, Direct URL
-- ═══════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════
-- MAIN VIDEO TABLE
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS unified_videos (
    id BIGSERIAL PRIMARY KEY,
    
    -- Basic Info
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) UNIQUE NOT NULL,
    description TEXT,
    
    -- Video Source
    video_url VARCHAR(1000) NOT NULL,
    video_platform VARCHAR(50) NOT NULL DEFAULT 'bunny',
    video_id VARCHAR(255),
    bunny_video_guid VARCHAR(255),
    bunny_library_id BIGINT,
    bunny_encoding_status VARCHAR(50),
    bunny_thumbnail_url VARCHAR(1000),
    
    -- Thumbnails
    thumbnail_url VARCHAR(1000),
    thumbnail_path VARCHAR(500),
    
    -- Video Metadata
    duration INTEGER, -- seconds
    quality VARCHAR(50),
    
    -- Content Classification
    content_type VARCHAR(50) NOT NULL, -- daily_video, weekly_watchlist, learning_center, room_archive
    difficulty_level VARCHAR(50), -- beginner, intermediate, advanced
    category VARCHAR(100),
    session_type VARCHAR(100),
    
    -- Enhanced Features
    chapter_timestamps JSONB,
    
    -- Relationships
    trader_id BIGINT,
    
    -- Scheduling
    video_date DATE NOT NULL,
    
    -- Publishing
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    published_at TIMESTAMP,
    scheduled_at TIMESTAMP,
    
    -- Tags (JSONB array of strings)
    tags JSONB DEFAULT '[]'::jsonb,
    
    -- Analytics
    views_count INTEGER DEFAULT 0,
    likes_count INTEGER DEFAULT 0,
    completion_rate INTEGER DEFAULT 0,
    
    -- Additional Metadata
    metadata JSONB,
    
    -- Audit
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    deleted_at TIMESTAMP
);

-- ═══════════════════════════════════════════════════════════════════════════
-- VIDEO-ROOM ASSIGNMENT (Junction Table)
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_room_assignments (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    trading_room_id BIGINT NOT NULL,
    is_featured_in_room BOOLEAN DEFAULT false,
    is_pinned BOOLEAN DEFAULT false,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(video_id, trading_room_id)
);

-- ═══════════════════════════════════════════════════════════════════════════
-- INDEXES FOR PERFORMANCE
-- ═══════════════════════════════════════════════════════════════════════════

-- Core lookups
CREATE INDEX IF NOT EXISTS idx_unified_videos_slug ON unified_videos(slug);
CREATE INDEX IF NOT EXISTS idx_unified_videos_content_type ON unified_videos(content_type);
CREATE INDEX IF NOT EXISTS idx_unified_videos_video_date ON unified_videos(video_date DESC);
CREATE INDEX IF NOT EXISTS idx_unified_videos_is_published ON unified_videos(is_published);
CREATE INDEX IF NOT EXISTS idx_unified_videos_is_featured ON unified_videos(is_featured);
CREATE INDEX IF NOT EXISTS idx_unified_videos_trader_id ON unified_videos(trader_id);
CREATE INDEX IF NOT EXISTS idx_unified_videos_created_at ON unified_videos(created_at DESC);

-- JSONB indexes for tags
CREATE INDEX IF NOT EXISTS idx_unified_videos_tags ON unified_videos USING GIN(tags);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_unified_videos_type_published ON unified_videos(content_type, is_published, video_date DESC);
CREATE INDEX IF NOT EXISTS idx_unified_videos_type_featured ON unified_videos(content_type, is_featured, video_date DESC);

-- Room assignment indexes
CREATE INDEX IF NOT EXISTS idx_video_room_assignments_video_id ON video_room_assignments(video_id);
CREATE INDEX IF NOT EXISTS idx_video_room_assignments_room_id ON video_room_assignments(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_video_room_assignments_featured ON video_room_assignments(trading_room_id, is_featured_in_room);

-- ═══════════════════════════════════════════════════════════════════════════
-- TRIGGERS FOR UPDATED_AT
-- ═══════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_unified_videos_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_unified_videos_updated_at
    BEFORE UPDATE ON unified_videos
    FOR EACH ROW
    EXECUTE FUNCTION update_unified_videos_updated_at();

-- ═══════════════════════════════════════════════════════════════════════════
-- MIGRATE EXISTING VIDEOS (if any)
-- ═══════════════════════════════════════════════════════════════════════════

INSERT INTO unified_videos (
    title,
    slug,
    description,
    video_url,
    video_platform,
    thumbnail_url,
    duration,
    content_type,
    video_date,
    is_published,
    views_count,
    created_at,
    updated_at
)
SELECT 
    title,
    slug,
    description,
    video_url,
    'direct' as video_platform,
    thumbnail as thumbnail_url,
    duration_seconds as duration,
    'learning_center' as content_type,
    COALESCE(created_at::date, CURRENT_DATE) as video_date,
    is_public as is_published,
    views_count,
    created_at,
    updated_at
FROM videos
WHERE NOT EXISTS (
    SELECT 1 FROM unified_videos WHERE unified_videos.slug = videos.slug
);

-- ═══════════════════════════════════════════════════════════════════════════
-- COMMENTS FOR DOCUMENTATION
-- ═══════════════════════════════════════════════════════════════════════════

COMMENT ON TABLE unified_videos IS 'Unified video system supporting daily videos, weekly watchlist, learning center, and room archives';
COMMENT ON COLUMN unified_videos.content_type IS 'Video type: daily_video, weekly_watchlist, learning_center, room_archive';
COMMENT ON COLUMN unified_videos.tags IS 'JSONB array of tag slugs for filtering and categorization';
COMMENT ON COLUMN unified_videos.video_platform IS 'Video hosting platform: bunny, vimeo, youtube, wistia, direct';
COMMENT ON COLUMN unified_videos.difficulty_level IS 'Learning difficulty: beginner, intermediate, advanced';
COMMENT ON TABLE video_room_assignments IS 'Junction table linking videos to trading rooms';
