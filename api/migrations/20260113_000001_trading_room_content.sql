-- ═══════════════════════════════════════════════════════════════════════════════════
-- TRADING ROOM CONTENT MANAGEMENT SYSTEM
-- Apple Principal Engineer ICT 7+ Grade - January 13, 2026
-- ═══════════════════════════════════════════════════════════════════════════════════
-- 
-- This migration adds:
-- 1. room_trade_plans - Trade plan entries (ticker, bias, entry, targets, stop, options)
-- 2. room_alerts - Trading alerts (entry/exit/update with notes)
-- 3. room_weekly_videos - Featured weekly video per room (auto-archive)
-- 
-- Integrates with existing:
-- - membership_plans table (for room IDs)
-- - daily_videos table (for video updates)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. TRADE PLAN ENTRIES TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_trade_plans (
    id BIGSERIAL PRIMARY KEY,
    
    -- Room association (references membership_plans.id)
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    
    -- Week tracking
    week_of DATE NOT NULL DEFAULT CURRENT_DATE,
    
    -- Trade details
    ticker VARCHAR(10) NOT NULL,
    bias VARCHAR(20) NOT NULL CHECK (bias IN ('BULLISH', 'BEARISH', 'NEUTRAL')),
    entry VARCHAR(20),
    target1 VARCHAR(20),
    target2 VARCHAR(20),
    target3 VARCHAR(20),
    runner VARCHAR(20),
    stop VARCHAR(20),
    
    -- Options play
    options_strike VARCHAR(50),
    options_exp DATE,
    
    -- Notes (expandable dropdown)
    notes TEXT,
    
    -- Display order
    sort_order INTEGER DEFAULT 0,
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for performance
CREATE INDEX idx_room_trade_plans_room_slug ON room_trade_plans(room_slug);
CREATE INDEX idx_room_trade_plans_week_of ON room_trade_plans(week_of);
CREATE INDEX idx_room_trade_plans_room_week ON room_trade_plans(room_slug, week_of);
CREATE INDEX idx_room_trade_plans_active ON room_trade_plans(is_active) WHERE is_active = true;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ROOM ALERTS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_alerts (
    id BIGSERIAL PRIMARY KEY,
    
    -- Room association
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    
    -- Alert type
    alert_type VARCHAR(20) NOT NULL CHECK (alert_type IN ('ENTRY', 'EXIT', 'UPDATE')),
    
    -- Trade details
    ticker VARCHAR(10) NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    -- Detailed notes (expandable dropdown)
    notes TEXT,
    
    -- Status flags
    is_new BOOLEAN DEFAULT true,
    is_published BOOLEAN DEFAULT true,
    is_pinned BOOLEAN DEFAULT false,
    
    -- Timestamps
    published_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for performance
CREATE INDEX idx_room_alerts_room_slug ON room_alerts(room_slug);
CREATE INDEX idx_room_alerts_type ON room_alerts(alert_type);
CREATE INDEX idx_room_alerts_ticker ON room_alerts(ticker);
CREATE INDEX idx_room_alerts_published ON room_alerts(is_published) WHERE is_published = true;
CREATE INDEX idx_room_alerts_new ON room_alerts(is_new) WHERE is_new = true;
CREATE INDEX idx_room_alerts_published_at ON room_alerts(published_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ROOM WEEKLY VIDEOS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_weekly_videos (
    id BIGSERIAL PRIMARY KEY,
    
    -- Room association
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    
    -- Week identification
    week_of DATE NOT NULL,
    week_title VARCHAR(255) NOT NULL, -- "Week of January 13, 2026"
    
    -- Video details
    video_title VARCHAR(255) NOT NULL, -- "Weekly Breakdown: Top Swing Setups"
    video_url TEXT NOT NULL,
    video_platform VARCHAR(20) DEFAULT 'bunny' CHECK (video_platform IN ('bunny', 'vimeo', 'youtube', 'wistia')),
    thumbnail_url TEXT,
    duration VARCHAR(20), -- "24:35"
    
    -- Description
    description TEXT,
    
    -- Status
    is_current BOOLEAN DEFAULT true, -- Only one per room should be current
    is_published BOOLEAN DEFAULT true,
    
    -- Timestamps
    published_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    archived_at TIMESTAMPTZ
);

-- Indexes for performance
CREATE INDEX idx_room_weekly_videos_room_slug ON room_weekly_videos(room_slug);
CREATE INDEX idx_room_weekly_videos_week_of ON room_weekly_videos(week_of);
CREATE INDEX idx_room_weekly_videos_current ON room_weekly_videos(is_current) WHERE is_current = true;
CREATE INDEX idx_room_weekly_videos_room_current ON room_weekly_videos(room_slug, is_current) WHERE is_current = true;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. ROOM STATS CACHE TABLE (Auto-calculated)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_stats_cache (
    id BIGSERIAL PRIMARY KEY,
    
    -- Room association
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL UNIQUE,
    
    -- Performance stats
    win_rate FLOAT8 DEFAULT 0,
    weekly_profit VARCHAR(50) DEFAULT '$0',
    monthly_profit VARCHAR(50) DEFAULT '$0',
    active_trades INTEGER DEFAULT 0,
    closed_this_week INTEGER DEFAULT 0,
    total_trades INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    
    -- Timestamps
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Unique index on room_slug
CREATE UNIQUE INDEX IF NOT EXISTS idx_room_stats_cache_room_slug ON room_stats_cache(room_slug);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. TRIGGER: Auto-update updated_at timestamp
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers
DROP TRIGGER IF EXISTS update_room_trade_plans_updated_at ON room_trade_plans;
CREATE TRIGGER update_room_trade_plans_updated_at
    BEFORE UPDATE ON room_trade_plans
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_room_alerts_updated_at ON room_alerts;
CREATE TRIGGER update_room_alerts_updated_at
    BEFORE UPDATE ON room_alerts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_room_weekly_videos_updated_at ON room_weekly_videos;
CREATE TRIGGER update_room_weekly_videos_updated_at
    BEFORE UPDATE ON room_weekly_videos
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_room_stats_cache_updated_at ON room_stats_cache;
CREATE TRIGGER update_room_stats_cache_updated_at
    BEFORE UPDATE ON room_stats_cache
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. SEED: Initialize Explosive Swings room stats
-- ═══════════════════════════════════════════════════════════════════════════════════

INSERT INTO room_stats_cache (room_id, room_slug, win_rate, weekly_profit, active_trades, closed_this_week)
VALUES (5, 'explosive-swings', 82.00, '+$4,850', 4, 2)
ON CONFLICT (room_slug) DO UPDATE SET
    win_rate = EXCLUDED.win_rate,
    weekly_profit = EXCLUDED.weekly_profit,
    active_trades = EXCLUDED.active_trades,
    closed_this_week = EXCLUDED.closed_this_week,
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════
