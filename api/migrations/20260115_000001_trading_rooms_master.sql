-- ═══════════════════════════════════════════════════════════════════════════════════
-- TRADING ROOMS MASTER TABLE
-- Apple Principal Engineer ICT 7 Grade - January 15, 2026
-- ═══════════════════════════════════════════════════════════════════════════════════
--
-- Creates the master trading_rooms table with all 6 rooms in correct order:
-- 1. Day Trading Room
-- 2. Swing Trading Room
-- 3. Small Account Mentorship
-- 4. Explosive Swings
-- 5. SPX Profit Pulse
-- 6. High Octane Scanner
--
-- Also adds 'section' field to room_resources for section-based organization:
-- - introduction (main videos)
-- - latest_updates
-- - premium_daily_videos
-- - watchlist
-- - weekly_alerts (Explosive Swings specific)
-- - learning_center
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. TRADING ROOMS MASTER TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS trading_rooms (
    id BIGSERIAL PRIMARY KEY,
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    type VARCHAR(50) NOT NULL DEFAULT 'trading_room',
    -- Values: 'trading_room', 'alert_service', 'mentorship'
    
    -- Descriptions
    description TEXT,
    short_description TEXT,
    
    -- Visual Branding
    icon VARCHAR(100),
    color VARCHAR(20),
    logo_url VARCHAR(500),
    image_url VARCHAR(500),
    banner_url VARCHAR(500),
    
    -- Display Order (CRITICAL for admin UI ordering)
    sort_order INTEGER NOT NULL DEFAULT 0,
    
    -- Status Flags
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    is_public BOOLEAN DEFAULT true,
    
    -- Features & Metadata
    features JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Section Configuration (which sections this room has)
    available_sections JSONB DEFAULT '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb,
    
    -- Pricing (optional, can link to membership_plans)
    price DECIMAL(10, 2),
    billing_cycle VARCHAR(20),
    membership_plan_id BIGINT,
    
    -- Audit Trail
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_trading_rooms_slug ON trading_rooms(slug);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_type ON trading_rooms(type);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_sort_order ON trading_rooms(sort_order);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_active ON trading_rooms(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_trading_rooms_featured ON trading_rooms(is_featured) WHERE is_featured = true;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. SEED ALL 6 TRADING ROOMS IN CORRECT ORDER
-- ═══════════════════════════════════════════════════════════════════════════════════

INSERT INTO trading_rooms (name, slug, type, sort_order, is_active, is_featured, icon, color, short_description, available_sections) VALUES
(
    'Day Trading Room',
    'day-trading-room',
    'trading_room',
    1,
    true,
    true,
    'chart-line',
    '#3b82f6',
    'Live day trading sessions with real-time trade alerts and daily market analysis.',
    '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"]'::jsonb
),
(
    'Swing Trading Room',
    'swing-trading-room',
    'trading_room',
    2,
    true,
    false,
    'trending-up',
    '#10b981',
    'Live swing trading sessions with swing trade alerts and weekly watchlist.',
    '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"]'::jsonb
),
(
    'Small Account Mentorship',
    'small-account-mentorship',
    'mentorship',
    3,
    true,
    false,
    'wallet',
    '#f59e0b',
    'Personalized mentorship for growing small trading accounts with proper risk management.',
    '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb
),
(
    'Explosive Swings',
    'explosive-swings',
    'alert_service',
    4,
    true,
    false,
    'rocket',
    '#ef4444',
    'High momentum swing trade alerts with detailed analysis and risk/reward ratios.',
    '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "weekly_alerts", "learning_center"]'::jsonb
),
(
    'SPX Profit Pulse',
    'spx-profit-pulse',
    'alert_service',
    5,
    true,
    false,
    'activity',
    '#8b5cf6',
    'Premium SPX options alerts for intraday opportunities with high win rate setups.',
    '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb
),
(
    'High Octane Scanner',
    'high-octane-scanner',
    'alert_service',
    6,
    true,
    false,
    'radar',
    '#06b6d4',
    'Real-time stock scanner with momentum alerts and breakout detection.',
    '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    type = EXCLUDED.type,
    sort_order = EXCLUDED.sort_order,
    icon = EXCLUDED.icon,
    color = EXCLUDED.color,
    short_description = EXCLUDED.short_description,
    available_sections = EXCLUDED.available_sections,
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ADD SECTION FIELD TO ROOM_RESOURCES
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add section column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'room_resources' AND column_name = 'section'
    ) THEN
        ALTER TABLE room_resources ADD COLUMN section VARCHAR(100) DEFAULT 'latest_updates';
    END IF;
END $$;

-- Create index for section queries
CREATE INDEX IF NOT EXISTS idx_room_resources_section ON room_resources(section);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_section ON room_resources(trading_room_id, section);
CREATE INDEX IF NOT EXISTS idx_room_resources_room_section_published 
    ON room_resources(trading_room_id, section, is_published, resource_date DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. BUNNY.NET UPLOAD TRACKING TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS bunny_uploads (
    id BIGSERIAL PRIMARY KEY,
    
    -- Upload Identification
    upload_id VARCHAR(255) UNIQUE,
    video_guid VARCHAR(255) UNIQUE,
    library_id BIGINT NOT NULL,
    collection_id VARCHAR(255),
    
    -- File Information
    original_filename VARCHAR(500),
    file_size BIGINT,
    mime_type VARCHAR(100),
    
    -- Processing Status
    status VARCHAR(50) DEFAULT 'uploading',
    -- Values: uploading, processing, encoding, ready, failed
    encode_progress INTEGER DEFAULT 0,
    
    -- Generated URLs (populated after processing completes)
    video_url VARCHAR(1000),
    thumbnail_url VARCHAR(1000),
    preview_url VARCHAR(1000),
    
    -- Video Metadata (populated after processing)
    duration INTEGER,
    width INTEGER,
    height INTEGER,
    framerate DECIMAL(5,2),
    
    -- Link to Resource (nullable until assigned)
    resource_id BIGINT REFERENCES room_resources(id) ON DELETE SET NULL,
    trading_room_id BIGINT,
    section VARCHAR(100),
    
    -- Error Tracking
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    
    -- Audit Trail
    uploaded_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for bunny_uploads
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_status ON bunny_uploads(status);
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_video_guid ON bunny_uploads(video_guid);
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_upload_id ON bunny_uploads(upload_id);
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_resource ON bunny_uploads(resource_id);
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_room ON bunny_uploads(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_created ON bunny_uploads(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. ROOM SECTIONS CONFIGURATION TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_sections (
    id BIGSERIAL PRIMARY KEY,
    
    -- Section Definition
    section_key VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(100),
    
    -- Display Settings
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    
    -- Allowed Content
    allowed_resource_types JSONB DEFAULT '["video"]'::jsonb,
    max_items INTEGER, -- NULL = unlimited
    
    -- Room Restrictions (NULL = available to all rooms)
    restricted_to_rooms JSONB, -- e.g., '["explosive-swings"]'
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed section definitions
INSERT INTO room_sections (section_key, name, description, icon, sort_order, allowed_resource_types, max_items, restricted_to_rooms) VALUES
('introduction', 'Introduction', 'Main welcome and overview videos for the room', 'video', 1, '["video"]'::jsonb, 3, NULL),
('latest_updates', 'Latest Updates', 'Recent announcements, news, and updates', 'bell', 2, '["video", "pdf", "document"]'::jsonb, NULL, NULL),
('premium_daily_videos', 'Premium Daily Videos', 'Daily trading analysis and market commentary', 'calendar', 3, '["video"]'::jsonb, NULL, NULL),
('watchlist', 'Watchlist', 'Weekly stock watchlist videos and documents', 'list-check', 4, '["video", "pdf", "spreadsheet"]'::jsonb, NULL, NULL),
('weekly_alerts', 'Weekly Alerts', 'Weekly alert summaries and trade recaps', 'alert-triangle', 5, '["video"]'::jsonb, NULL, '["explosive-swings"]'::jsonb),
('learning_center', 'Learning Center', 'Educational tutorials, guides, and courses', 'school', 6, '["video", "pdf", "document"]'::jsonb, NULL, NULL)
ON CONFLICT (section_key) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    sort_order = EXCLUDED.sort_order,
    allowed_resource_types = EXCLUDED.allowed_resource_types,
    max_items = EXCLUDED.max_items,
    restricted_to_rooms = EXCLUDED.restricted_to_rooms,
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. TRIGGERS FOR UPDATED_AT
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Trading Rooms
CREATE OR REPLACE FUNCTION update_trading_rooms_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_trading_rooms_updated_at ON trading_rooms;
CREATE TRIGGER trigger_trading_rooms_updated_at
    BEFORE UPDATE ON trading_rooms
    FOR EACH ROW
    EXECUTE FUNCTION update_trading_rooms_updated_at();

-- Bunny Uploads
CREATE OR REPLACE FUNCTION update_bunny_uploads_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_bunny_uploads_updated_at ON bunny_uploads;
CREATE TRIGGER trigger_bunny_uploads_updated_at
    BEFORE UPDATE ON bunny_uploads
    FOR EACH ROW
    EXECUTE FUNCTION update_bunny_uploads_updated_at();

-- Room Sections
CREATE OR REPLACE FUNCTION update_room_sections_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_room_sections_updated_at ON room_sections;
CREATE TRIGGER trigger_room_sections_updated_at
    BEFORE UPDATE ON room_sections
    FOR EACH ROW
    EXECUTE FUNCTION update_room_sections_updated_at();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. HELPER VIEWS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- View: Trading rooms with resource counts per section
CREATE OR REPLACE VIEW v_trading_rooms_with_stats AS
SELECT 
    tr.*,
    COALESCE(stats.total_resources, 0) as total_resources,
    COALESCE(stats.introduction_count, 0) as introduction_count,
    COALESCE(stats.latest_updates_count, 0) as latest_updates_count,
    COALESCE(stats.premium_daily_count, 0) as premium_daily_count,
    COALESCE(stats.watchlist_count, 0) as watchlist_count,
    COALESCE(stats.weekly_alerts_count, 0) as weekly_alerts_count,
    COALESCE(stats.learning_center_count, 0) as learning_center_count
FROM trading_rooms tr
LEFT JOIN (
    SELECT 
        trading_room_id,
        COUNT(*) as total_resources,
        COUNT(*) FILTER (WHERE section = 'introduction') as introduction_count,
        COUNT(*) FILTER (WHERE section = 'latest_updates') as latest_updates_count,
        COUNT(*) FILTER (WHERE section = 'premium_daily_videos') as premium_daily_count,
        COUNT(*) FILTER (WHERE section = 'watchlist') as watchlist_count,
        COUNT(*) FILTER (WHERE section = 'weekly_alerts') as weekly_alerts_count,
        COUNT(*) FILTER (WHERE section = 'learning_center') as learning_center_count
    FROM room_resources
    WHERE deleted_at IS NULL AND is_published = true
    GROUP BY trading_room_id
) stats ON tr.id = stats.trading_room_id
WHERE tr.is_active = true
ORDER BY tr.sort_order;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════
