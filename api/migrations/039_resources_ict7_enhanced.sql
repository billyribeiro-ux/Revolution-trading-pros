-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 039: Resources System ICT 7 Enhanced
-- Apple Principal Engineer Grade - February 2026
--
-- Adds:
-- - Stock/ETF lists management
-- - Resource access logging (recently accessed)
-- - Enhanced analytics tables
-- ═══════════════════════════════════════════════════════════════════════════════════

-- 1. Stock Lists table for ETF/Stock/Watchlist management
CREATE TABLE IF NOT EXISTS stock_lists (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    list_type VARCHAR(50) NOT NULL DEFAULT 'watchlist', -- etf, stock, watchlist, sector
    trading_room_id BIGINT NOT NULL,
    symbols JSONB NOT NULL DEFAULT '[]'::jsonb, -- Array of {symbol, name, sector, notes, price_target, entry_price, stop_loss}
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    sort_order INTEGER DEFAULT 0,
    week_of DATE, -- For weekly watchlists
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_stock_lists_room ON stock_lists(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_stock_lists_type ON stock_lists(list_type);
CREATE INDEX IF NOT EXISTS idx_stock_lists_week ON stock_lists(week_of DESC) WHERE week_of IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_stock_lists_active ON stock_lists(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_stock_lists_featured ON stock_lists(is_featured) WHERE is_featured = true;

COMMENT ON TABLE stock_lists IS 'Stock, ETF, and watchlist management for trading rooms - ICT 7 Grade';
COMMENT ON COLUMN stock_lists.symbols IS 'JSON array with symbol details: {symbol, name, sector, notes, price_target, entry_price, stop_loss}';

-- 2. Resource Access Log table for recently accessed tracking
CREATE TABLE IF NOT EXISTS resource_access_log (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resource_id BIGINT NOT NULL REFERENCES room_resources(id) ON DELETE CASCADE,
    resource_type VARCHAR(50) NOT NULL,
    resource_title VARCHAR(500) NOT NULL,
    resource_thumbnail VARCHAR(500),
    accessed_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, resource_id)
);

CREATE INDEX IF NOT EXISTS idx_resource_access_user ON resource_access_log(user_id);
CREATE INDEX IF NOT EXISTS idx_resource_access_recent ON resource_access_log(user_id, accessed_at DESC);
CREATE INDEX IF NOT EXISTS idx_resource_access_resource ON resource_access_log(resource_id);

COMMENT ON TABLE resource_access_log IS 'Tracks user resource access for recently accessed feature - ICT 7 Grade';

-- 3. Ensure resource_favorites has proper constraints (may already exist from migration 036)
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'resource_favorites') THEN
        CREATE TABLE resource_favorites (
            id BIGSERIAL PRIMARY KEY,
            resource_id BIGINT NOT NULL REFERENCES room_resources(id) ON DELETE CASCADE,
            user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            created_at TIMESTAMP DEFAULT NOW(),
            UNIQUE(resource_id, user_id)
        );
        CREATE INDEX IF NOT EXISTS idx_resource_favorites_user ON resource_favorites(user_id);
        CREATE INDEX IF NOT EXISTS idx_resource_favorites_resource ON resource_favorites(resource_id);
    END IF;
END $$;

-- 4. Resource analytics materialized view for fast dashboard queries
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_resource_analytics AS
SELECT
    r.trading_room_id,
    r.resource_type,
    COALESCE(r.access_level, 'premium') as access_level,
    COUNT(*) as resource_count,
    COALESCE(SUM(r.views_count), 0) as total_views,
    COALESCE(SUM(r.downloads_count), 0) as total_downloads,
    COUNT(DISTINCT f.user_id) as unique_favorites,
    MAX(r.created_at) as latest_upload
FROM room_resources r
LEFT JOIN resource_favorites f ON r.id = f.resource_id
WHERE r.deleted_at IS NULL
GROUP BY r.trading_room_id, r.resource_type, COALESCE(r.access_level, 'premium');

CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_resource_analytics ON mv_resource_analytics(trading_room_id, resource_type, access_level);

-- 5. Function to refresh analytics materialized view
CREATE OR REPLACE FUNCTION refresh_resource_analytics()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY mv_resource_analytics;
END;
$$ LANGUAGE plpgsql;

-- 6. Insert default stock list types for reference
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM stock_lists LIMIT 1) THEN
        -- This is just a template, actual data will be managed by admins
        NULL;
    END IF;
END $$;

-- 7. Trigger to update stock_lists.updated_at
CREATE OR REPLACE FUNCTION update_stock_lists_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS tr_stock_lists_updated ON stock_lists;
CREATE TRIGGER tr_stock_lists_updated
    BEFORE UPDATE ON stock_lists
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_lists_timestamp();

-- 8. Cleanup old resource access logs (keep last 100 per user)
CREATE OR REPLACE FUNCTION cleanup_old_resource_access()
RETURNS void AS $$
BEGIN
    DELETE FROM resource_access_log
    WHERE id IN (
        SELECT id FROM (
            SELECT id, ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY accessed_at DESC) as rn
            FROM resource_access_log
        ) ranked
        WHERE rn > 100
    );
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION cleanup_old_resource_access IS 'Keeps only the last 100 accessed resources per user';
