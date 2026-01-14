-- ═══════════════════════════════════════════════════════════════════════════
-- ROOM RESOURCES SYSTEM - Unified Content Management
-- ═══════════════════════════════════════════════════════════════════════════
-- Apple Principal Engineer ICT 7 Grade - January 2026
--
-- Comprehensive resource system supporting:
-- - Videos (Bunny.net, Vimeo, YouTube, Direct URL)
-- - PDFs (Trade plans, guides, cheat sheets)
-- - Documents (Word, Excel, CSV)
-- - Images (Charts, screenshots, diagrams)
--
-- Features:
-- - Room-specific isolation (each room has its own resources)
-- - Content type classification
-- - Featured/pinned resources per room
-- - SEO-friendly slugs
-- - Full-text search ready
-- - Analytics tracking
-- ═══════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════
-- MAIN ROOM RESOURCES TABLE
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_resources (
    id BIGSERIAL PRIMARY KEY,
    
    -- Basic Info
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) NOT NULL,
    description TEXT,
    
    -- Resource Type & Classification
    resource_type VARCHAR(50) NOT NULL DEFAULT 'video',
    -- Values: video, pdf, document, image, spreadsheet, archive, other
    
    content_type VARCHAR(50) NOT NULL DEFAULT 'daily_video',
    -- Values: tutorial, daily_video, weekly_watchlist, trade_plan, guide, 
    --         chart, screenshot, template, cheat_sheet, archive, other
    
    -- File/Video Source
    file_url VARCHAR(1000) NOT NULL,
    file_path VARCHAR(500),
    mime_type VARCHAR(100),
    file_size BIGINT,
    
    -- Video-specific fields (for resource_type = 'video')
    video_platform VARCHAR(50),
    -- Values: bunny, vimeo, youtube, wistia, direct
    video_id VARCHAR(255),
    bunny_video_guid VARCHAR(255),
    bunny_library_id BIGINT,
    duration INTEGER, -- seconds
    
    -- Thumbnails
    thumbnail_url VARCHAR(1000),
    thumbnail_path VARCHAR(500),
    
    -- Image-specific (for resource_type = 'image')
    width INTEGER,
    height INTEGER,
    
    -- Room Assignment (REQUIRED - every resource belongs to a room)
    trading_room_id BIGINT NOT NULL,
    
    -- Trader Assignment (optional)
    trader_id BIGINT,
    
    -- Scheduling & Publishing
    resource_date DATE NOT NULL DEFAULT CURRENT_DATE,
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_pinned BOOLEAN DEFAULT false,
    sort_order INTEGER DEFAULT 0,
    published_at TIMESTAMP,
    scheduled_at TIMESTAMP,
    
    -- Categorization
    category VARCHAR(100),
    tags JSONB DEFAULT '[]'::jsonb,
    difficulty_level VARCHAR(50),
    -- Values: beginner, intermediate, advanced
    
    -- Analytics
    views_count INTEGER DEFAULT 0,
    downloads_count INTEGER DEFAULT 0,
    likes_count INTEGER DEFAULT 0,
    
    -- Additional Metadata
    metadata JSONB,
    
    -- Audit
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT unique_slug_per_room UNIQUE(trading_room_id, slug)
);

-- ═══════════════════════════════════════════════════════════════════════════
-- INDEXES FOR PERFORMANCE
-- ═══════════════════════════════════════════════════════════════════════════

-- Primary lookups
CREATE INDEX IF NOT EXISTS idx_room_resources_slug ON room_resources(slug);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_id ON room_resources(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_room_resources_type ON room_resources(resource_type);
CREATE INDEX IF NOT EXISTS idx_room_resources_content_type ON room_resources(content_type);
CREATE INDEX IF NOT EXISTS idx_room_resources_date ON room_resources(resource_date DESC);
CREATE INDEX IF NOT EXISTS idx_room_resources_published ON room_resources(is_published);
CREATE INDEX IF NOT EXISTS idx_room_resources_featured ON room_resources(is_featured);
CREATE INDEX IF NOT EXISTS idx_room_resources_trader ON room_resources(trader_id);
CREATE INDEX IF NOT EXISTS idx_room_resources_created ON room_resources(created_at DESC);

-- JSONB indexes for tags
CREATE INDEX IF NOT EXISTS idx_room_resources_tags ON room_resources USING GIN(tags);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_room_resources_room_published 
    ON room_resources(trading_room_id, is_published, resource_date DESC);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_type_published 
    ON room_resources(trading_room_id, resource_type, is_published, resource_date DESC);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_content_published 
    ON room_resources(trading_room_id, content_type, is_published, resource_date DESC);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_featured 
    ON room_resources(trading_room_id, is_featured, is_published);

-- Soft delete index
CREATE INDEX IF NOT EXISTS idx_room_resources_not_deleted 
    ON room_resources(id) WHERE deleted_at IS NULL;

-- ═══════════════════════════════════════════════════════════════════════════
-- TRIGGERS FOR UPDATED_AT
-- ═══════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_room_resources_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_room_resources_updated_at ON room_resources;
CREATE TRIGGER trigger_room_resources_updated_at
    BEFORE UPDATE ON room_resources
    FOR EACH ROW
    EXECUTE FUNCTION update_room_resources_updated_at();

-- ═══════════════════════════════════════════════════════════════════════════
-- HELPER FUNCTION: Generate slug from title
-- ═══════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION generate_resource_slug(title TEXT, room_id BIGINT)
RETURNS TEXT AS $$
DECLARE
    base_slug TEXT;
    final_slug TEXT;
    counter INTEGER := 0;
BEGIN
    -- Generate base slug from title
    base_slug := lower(regexp_replace(title, '[^a-zA-Z0-9]+', '-', 'g'));
    base_slug := trim(both '-' from base_slug);
    
    -- Check for uniqueness within room
    final_slug := base_slug;
    WHILE EXISTS (SELECT 1 FROM room_resources WHERE slug = final_slug AND trading_room_id = room_id AND deleted_at IS NULL) LOOP
        counter := counter + 1;
        final_slug := base_slug || '-' || counter;
    END LOOP;
    
    RETURN final_slug;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════
-- MIGRATE EXISTING VIDEOS FROM unified_videos
-- ═══════════════════════════════════════════════════════════════════════════

INSERT INTO room_resources (
    title,
    slug,
    description,
    resource_type,
    content_type,
    file_url,
    mime_type,
    video_platform,
    video_id,
    bunny_video_guid,
    bunny_library_id,
    duration,
    thumbnail_url,
    trading_room_id,
    trader_id,
    resource_date,
    is_published,
    is_featured,
    tags,
    views_count,
    difficulty_level,
    created_at,
    updated_at
)
SELECT 
    uv.title,
    uv.slug,
    uv.description,
    'video' as resource_type,
    uv.content_type,
    uv.video_url as file_url,
    'video/mp4' as mime_type,
    uv.video_platform,
    uv.video_id,
    uv.bunny_video_guid,
    uv.bunny_library_id,
    uv.duration,
    COALESCE(uv.bunny_thumbnail_url, uv.thumbnail_url) as thumbnail_url,
    vra.trading_room_id,
    uv.trader_id,
    uv.video_date as resource_date,
    uv.is_published,
    uv.is_featured,
    uv.tags,
    uv.views_count,
    uv.difficulty_level,
    uv.created_at,
    uv.updated_at
FROM unified_videos uv
JOIN video_room_assignments vra ON vra.video_id = uv.id
WHERE uv.deleted_at IS NULL
  AND NOT EXISTS (
      SELECT 1 FROM room_resources rr 
      WHERE rr.slug = uv.slug 
        AND rr.trading_room_id = vra.trading_room_id
  );

-- ═══════════════════════════════════════════════════════════════════════════
-- COMMENTS FOR DOCUMENTATION
-- ═══════════════════════════════════════════════════════════════════════════

COMMENT ON TABLE room_resources IS 'Unified resource system for all trading room content (videos, PDFs, documents, images)';
COMMENT ON COLUMN room_resources.resource_type IS 'Type of resource: video, pdf, document, image, spreadsheet, archive, other';
COMMENT ON COLUMN room_resources.content_type IS 'Content classification: tutorial, daily_video, weekly_watchlist, trade_plan, guide, chart, screenshot, template, cheat_sheet, archive, other';
COMMENT ON COLUMN room_resources.trading_room_id IS 'Required room assignment - ensures room-specific isolation';
COMMENT ON COLUMN room_resources.is_featured IS 'Featured resources appear prominently (e.g., main tutorial video)';
COMMENT ON COLUMN room_resources.is_pinned IS 'Pinned resources stay at top of their category';
COMMENT ON COLUMN room_resources.tags IS 'JSONB array of tag slugs for filtering and categorization';
