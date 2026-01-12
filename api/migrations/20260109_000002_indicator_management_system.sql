-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATOR MANAGEMENT SYSTEM - Database Schema
-- Apple Principal Engineer ICT 7 Grade - January 2026
-- Revolution Trading Pros
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATORS TABLE (Core indicator data)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicators (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    tagline VARCHAR(500),
    description TEXT,
    
    -- Pricing
    price_cents INTEGER NOT NULL DEFAULT 0,
    is_free BOOLEAN DEFAULT FALSE,
    sale_price_cents INTEGER,
    sale_ends_at TIMESTAMPTZ,
    
    -- Display
    logo_url VARCHAR(1024),
    card_image_url VARCHAR(1024),
    banner_image_url VARCHAR(1024),
    featured_video_url VARCHAR(1024),
    bunny_video_guid VARCHAR(255),
    
    -- Content
    short_description TEXT,
    long_description TEXT,
    features JSONB DEFAULT '[]'::jsonb,
    requirements JSONB DEFAULT '[]'::jsonb,
    compatibility JSONB DEFAULT '[]'::jsonb,
    
    -- Platform Support
    supported_platforms JSONB DEFAULT '["thinkorswim", "tradingview", "metatrader", "ninjatrader"]'::jsonb,
    
    -- Metadata
    version VARCHAR(50) DEFAULT '1.0',
    release_date DATE,
    last_update DATE,
    
    -- SEO
    meta_title VARCHAR(255),
    meta_description TEXT,
    og_image_url VARCHAR(1024),
    
    -- Status
    status VARCHAR(50) DEFAULT 'draft',
    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    
    -- Analytics
    view_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    purchase_count INTEGER DEFAULT 0,
    
    -- Associations
    creator_id INTEGER REFERENCES users(id),
    product_id INTEGER,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_indicators_slug ON indicators(slug);
CREATE INDEX idx_indicators_status ON indicators(status);
CREATE INDEX idx_indicators_is_published ON indicators(is_published);
CREATE INDEX idx_indicators_creator ON indicators(creator_id);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATOR FILES (Multi-platform downloadable files)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicator_files (
    id SERIAL PRIMARY KEY,
    indicator_id UUID NOT NULL REFERENCES indicators(id) ON DELETE CASCADE,
    
    -- File Info
    file_name VARCHAR(255) NOT NULL,
    original_name VARCHAR(255),
    file_path VARCHAR(1024) NOT NULL,
    file_size_bytes BIGINT,
    file_type VARCHAR(100),
    mime_type VARCHAR(100),
    checksum_sha256 VARCHAR(64),
    
    -- Platform
    platform VARCHAR(50) NOT NULL,
    platform_version VARCHAR(50),
    
    -- Storage
    storage_provider VARCHAR(50) DEFAULT 'cloudflare_r2',
    storage_bucket VARCHAR(255),
    storage_key VARCHAR(1024),
    cdn_url VARCHAR(1024),
    
    -- Secure Download
    download_token VARCHAR(255),
    token_expires_at TIMESTAMPTZ,
    
    -- Version
    version VARCHAR(50) DEFAULT '1.0',
    is_current_version BOOLEAN DEFAULT TRUE,
    changelog TEXT,
    
    -- Display
    display_name VARCHAR(255),
    display_order INTEGER DEFAULT 0,
    icon_url VARCHAR(1024),
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    
    -- Analytics
    download_count INTEGER DEFAULT 0,
    
    -- Timestamps
    uploaded_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_indicator_files_indicator ON indicator_files(indicator_id);
CREATE INDEX idx_indicator_files_platform ON indicator_files(platform);
CREATE INDEX idx_indicator_files_active ON indicator_files(is_active);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATOR VIDEOS (Tutorial/demo videos via Bunny Stream)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicator_videos (
    id SERIAL PRIMARY KEY,
    indicator_id UUID NOT NULL REFERENCES indicators(id) ON DELETE CASCADE,
    
    -- Video Info
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Bunny Stream
    bunny_video_guid VARCHAR(255) NOT NULL,
    bunny_library_id VARCHAR(50),
    embed_url VARCHAR(1024),
    play_url VARCHAR(1024),
    thumbnail_url VARCHAR(1024),
    
    -- Duration
    duration_seconds INTEGER,
    
    -- Display
    display_order INTEGER DEFAULT 0,
    is_featured BOOLEAN DEFAULT FALSE,
    is_preview BOOLEAN DEFAULT FALSE,
    
    -- Status
    encoding_status VARCHAR(50) DEFAULT 'pending',
    is_published BOOLEAN DEFAULT TRUE,
    
    -- Analytics
    view_count INTEGER DEFAULT 0,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_indicator_videos_indicator ON indicator_videos(indicator_id);
CREATE INDEX idx_indicator_videos_featured ON indicator_videos(is_featured);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- USER INDICATOR OWNERSHIP (Who owns which indicators)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_indicator_ownership (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    indicator_id UUID NOT NULL REFERENCES indicators(id) ON DELETE CASCADE,
    
    -- Purchase Info
    order_id INTEGER,
    order_item_id INTEGER,
    price_paid_cents INTEGER,
    currency VARCHAR(3) DEFAULT 'USD',
    
    -- Access
    access_granted_at TIMESTAMPTZ DEFAULT NOW(),
    access_expires_at TIMESTAMPTZ,
    is_lifetime_access BOOLEAN DEFAULT TRUE,
    
    -- Source
    source VARCHAR(50) DEFAULT 'purchase',
    granted_by INTEGER REFERENCES users(id),
    notes TEXT,
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, indicator_id)
);

CREATE INDEX idx_user_indicator_ownership_user ON user_indicator_ownership(user_id);
CREATE INDEX idx_user_indicator_ownership_indicator ON user_indicator_ownership(indicator_id);
CREATE INDEX idx_user_indicator_ownership_active ON user_indicator_ownership(is_active);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATOR DOWNLOADS (Download tracking with secure tokens)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicator_downloads (
    id SERIAL PRIMARY KEY,
    
    -- Associations
    user_id INTEGER REFERENCES users(id),
    indicator_id UUID NOT NULL REFERENCES indicators(id),
    file_id INTEGER NOT NULL REFERENCES indicator_files(id),
    ownership_id INTEGER REFERENCES user_indicator_ownership(id),
    
    -- Download Token (WordPress-compatible hash-based)
    download_token VARCHAR(255) NOT NULL UNIQUE,
    token_hash VARCHAR(64),
    token_expires_at TIMESTAMPTZ NOT NULL,
    
    -- Request Info
    ip_address INET,
    user_agent TEXT,
    referer VARCHAR(1024),
    
    -- Status
    status VARCHAR(50) DEFAULT 'pending',
    downloaded_at TIMESTAMPTZ,
    download_count INTEGER DEFAULT 0,
    max_downloads INTEGER DEFAULT 5,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_indicator_downloads_token ON indicator_downloads(download_token);
CREATE INDEX idx_indicator_downloads_user ON indicator_downloads(user_id);
CREATE INDEX idx_indicator_downloads_indicator ON indicator_downloads(indicator_id);
CREATE INDEX idx_indicator_downloads_file ON indicator_downloads(file_id);
CREATE INDEX idx_indicator_downloads_status ON indicator_downloads(status);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- DOWNLOAD ANALYTICS (Aggregate analytics for reporting)
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicator_download_analytics (
    id SERIAL PRIMARY KEY,
    
    -- Dimensions
    indicator_id UUID NOT NULL REFERENCES indicators(id),
    file_id INTEGER REFERENCES indicator_files(id),
    platform VARCHAR(50),
    date DATE NOT NULL,
    
    -- Metrics
    download_count INTEGER DEFAULT 0,
    unique_users INTEGER DEFAULT 0,
    bandwidth_bytes BIGINT DEFAULT 0,
    
    -- Breakdown
    by_country JSONB DEFAULT '{}'::jsonb,
    by_device JSONB DEFAULT '{}'::jsonb,
    by_browser JSONB DEFAULT '{}'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(indicator_id, file_id, platform, date)
);

CREATE INDEX idx_indicator_analytics_indicator ON indicator_download_analytics(indicator_id);
CREATE INDEX idx_indicator_analytics_date ON indicator_download_analytics(date);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- INDICATOR CATEGORIES
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS indicator_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    icon_url VARCHAR(1024),
    parent_id INTEGER REFERENCES indicator_categories(id),
    display_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_category_mapping (
    indicator_id UUID NOT NULL REFERENCES indicators(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES indicator_categories(id) ON DELETE CASCADE,
    PRIMARY KEY (indicator_id, category_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- PLATFORM DEFINITIONS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS trading_platforms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    icon_url VARCHAR(1024),
    file_extension VARCHAR(20),
    install_instructions TEXT,
    documentation_url VARCHAR(1024),
    is_active BOOLEAN DEFAULT TRUE,
    display_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default platforms
INSERT INTO trading_platforms (name, slug, display_name, file_extension, display_order) VALUES
    ('ThinkorSwim', 'thinkorswim', 'ThinkorSwim (TD Ameritrade)', '.ts', 1),
    ('TradingView', 'tradingview', 'TradingView', '.pine', 2),
    ('MetaTrader 4', 'mt4', 'MetaTrader 4', '.mq4', 3),
    ('MetaTrader 5', 'mt5', 'MetaTrader 5', '.mq5', 4),
    ('NinjaTrader', 'ninjatrader', 'NinjaTrader 8', '.cs', 5),
    ('TradeStation', 'tradestation', 'TradeStation', '.els', 6),
    ('Sierra Chart', 'sierrachart', 'Sierra Chart', '.cpp', 7),
    ('cTrader', 'ctrader', 'cTrader', '.algo', 8)
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- TRIGGERS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_indicator_timestamps()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_indicators_updated
    BEFORE UPDATE ON indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_indicator_timestamps();

CREATE TRIGGER trigger_indicator_files_updated
    BEFORE UPDATE ON indicator_files
    FOR EACH ROW
    EXECUTE FUNCTION update_indicator_timestamps();

CREATE TRIGGER trigger_indicator_videos_updated
    BEFORE UPDATE ON indicator_videos
    FOR EACH ROW
    EXECUTE FUNCTION update_indicator_timestamps();

CREATE TRIGGER trigger_user_indicator_ownership_updated
    BEFORE UPDATE ON user_indicator_ownership
    FOR EACH ROW
    EXECUTE FUNCTION update_indicator_timestamps();

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- FUNCTIONS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION check_indicator_ownership(
    p_user_id INTEGER,
    p_indicator_id UUID
) RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM user_indicator_ownership
        WHERE user_id = p_user_id
        AND indicator_id = p_indicator_id
        AND is_active = TRUE
        AND (access_expires_at IS NULL OR access_expires_at > NOW())
    );
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION generate_download_token(
    p_user_id INTEGER,
    p_indicator_id UUID,
    p_file_id INTEGER
) RETURNS TEXT AS $$
DECLARE
    v_token TEXT;
    v_hash TEXT;
BEGIN
    v_token := encode(gen_random_bytes(32), 'hex');
    v_hash := encode(sha256(v_token::bytea), 'hex');
    
    INSERT INTO indicator_downloads (
        user_id, indicator_id, file_id, download_token, token_hash, token_expires_at
    ) VALUES (
        p_user_id, p_indicator_id, p_file_id, v_token, v_hash, NOW() + INTERVAL '24 hours'
    );
    
    RETURN v_token;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- VIEWS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE VIEW v_indicators_with_stats AS
SELECT 
    i.*,
    COUNT(DISTINCT uio.user_id) as owner_count,
    COUNT(DISTINCT f.id) as file_count,
    COUNT(DISTINCT v.id) as video_count,
    COALESCE(SUM(f.download_count), 0) as total_downloads
FROM indicators i
LEFT JOIN user_indicator_ownership uio ON i.id = uio.indicator_id AND uio.is_active = TRUE
LEFT JOIN indicator_files f ON i.id = f.indicator_id AND f.is_active = TRUE
LEFT JOIN indicator_videos v ON i.id = v.indicator_id AND v.is_published = TRUE
GROUP BY i.id;

CREATE OR REPLACE VIEW v_user_indicators AS
SELECT 
    uio.user_id,
    i.*,
    uio.access_granted_at,
    uio.access_expires_at,
    uio.is_lifetime_access,
    uio.source
FROM user_indicator_ownership uio
JOIN indicators i ON uio.indicator_id = i.id
WHERE uio.is_active = TRUE
AND (uio.access_expires_at IS NULL OR uio.access_expires_at > NOW());

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- DONE
-- ═══════════════════════════════════════════════════════════════════════════════════════
