-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 030: Room Content Full-Text Search Indexes
-- Apple Principal Engineer ICT 11+ Grade - January 2026
--
-- Creates GIN indexes for PostgreSQL Full-Text Search on room content tables:
-- - room_alerts (ticker, title, message, notes)
-- - room_trades (ticker, notes)
-- - room_trade_plans (ticker, notes)
--
-- These indexes enable O(log n) search performance with ts_rank relevance scoring.
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ROOM ALERTS FULL-TEXT SEARCH INDEX
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Drop existing index if recreating
DROP INDEX IF EXISTS idx_room_alerts_fts;

-- Create GIN index for full-text search on alerts
-- Includes: ticker, title, message, notes
-- Uses 'english' language configuration for stemming and stop words
CREATE INDEX idx_room_alerts_fts ON room_alerts USING GIN (
    to_tsvector('english',
        COALESCE(ticker, '') || ' ' ||
        COALESCE(title, '') || ' ' ||
        COALESCE(message, '') || ' ' ||
        COALESCE(notes, '')
    )
);

-- Create index for date-filtered searches (composite with published_at)
DROP INDEX IF EXISTS idx_room_alerts_fts_date;
CREATE INDEX idx_room_alerts_fts_date ON room_alerts (room_slug, published_at DESC)
WHERE deleted_at IS NULL AND is_published = true;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ROOM TRADES FULL-TEXT SEARCH INDEX
-- ═══════════════════════════════════════════════════════════════════════════════════

DROP INDEX IF EXISTS idx_room_trades_fts;

-- Create GIN index for full-text search on trades
-- Includes: ticker, notes
CREATE INDEX idx_room_trades_fts ON room_trades USING GIN (
    to_tsvector('english',
        COALESCE(ticker, '') || ' ' ||
        COALESCE(notes, '')
    )
);

-- Create index for date-filtered searches
DROP INDEX IF EXISTS idx_room_trades_fts_date;
CREATE INDEX idx_room_trades_fts_date ON room_trades (room_slug, entry_date DESC)
WHERE deleted_at IS NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ROOM TRADE PLANS FULL-TEXT SEARCH INDEX
-- ═══════════════════════════════════════════════════════════════════════════════════

DROP INDEX IF EXISTS idx_room_trade_plans_fts;

-- Create GIN index for full-text search on trade plans
-- Includes: ticker, notes
CREATE INDEX idx_room_trade_plans_fts ON room_trade_plans USING GIN (
    to_tsvector('english',
        COALESCE(ticker, '') || ' ' ||
        COALESCE(notes, '')
    )
);

-- Create index for date-filtered searches
DROP INDEX IF EXISTS idx_room_trade_plans_fts_date;
CREATE INDEX idx_room_trade_plans_fts_date ON room_trade_plans (room_slug, week_of DESC)
WHERE deleted_at IS NULL AND is_active = true;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. TICKER LOOKUP INDEX (for suggestions/autocomplete)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Index for ticker autocomplete (LIKE 'PREFIX%' queries)
DROP INDEX IF EXISTS idx_room_alerts_ticker_upper;
CREATE INDEX idx_room_alerts_ticker_upper ON room_alerts (UPPER(ticker), room_slug)
WHERE deleted_at IS NULL;

DROP INDEX IF EXISTS idx_room_trades_ticker_upper;
CREATE INDEX idx_room_trades_ticker_upper ON room_trades (UPPER(ticker), room_slug)
WHERE deleted_at IS NULL;

DROP INDEX IF EXISTS idx_room_trade_plans_ticker_upper;
CREATE INDEX idx_room_trade_plans_ticker_upper ON room_trade_plans (UPPER(ticker), room_slug)
WHERE deleted_at IS NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. VERIFY INDEXES CREATED
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Log index creation
DO $$
BEGIN
    RAISE NOTICE 'Room Search FTS indexes created successfully:';
    RAISE NOTICE '  - idx_room_alerts_fts (GIN)';
    RAISE NOTICE '  - idx_room_trades_fts (GIN)';
    RAISE NOTICE '  - idx_room_trade_plans_fts (GIN)';
    RAISE NOTICE '  - idx_room_*_fts_date (B-tree composite)';
    RAISE NOTICE '  - idx_room_*_ticker_upper (B-tree for autocomplete)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
