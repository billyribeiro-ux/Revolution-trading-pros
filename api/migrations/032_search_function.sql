-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 032: Room Content Search Function (Simplified)
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Apple Principal Engineer ICT 7+ Grade - January 2026
--
-- Provides a simplified search function for room content.
-- Note: A more comprehensive search function exists in migration 030.
-- This provides an alternative lightweight interface.
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- SIMPLIFIED ROOM CONTENT SEARCH FUNCTION
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Returns search results with relevance ranking and highlighted excerpts

CREATE OR REPLACE FUNCTION search_room_content(
    p_room_slug TEXT,
    p_query TEXT,
    p_content_types TEXT[] DEFAULT ARRAY['alerts', 'trades', 'trade_plans'],
    p_limit INT DEFAULT 20,
    p_offset INT DEFAULT 0
) RETURNS TABLE (
    content_type TEXT,
    id BIGINT,
    ticker VARCHAR,
    title TEXT,
    rank REAL,
    headline TEXT,
    created_at TIMESTAMPTZ
) AS $$
DECLARE
    v_tsquery tsquery;
BEGIN
    v_tsquery := plainto_tsquery('english', p_query);

    RETURN QUERY
    -- Search alerts
    (SELECT
        'alert'::TEXT as content_type,
        a.id,
        a.ticker,
        a.title,
        ts_rank(
            to_tsvector('english', coalesce(a.ticker, '') || ' ' || coalesce(a.title, '') || ' ' || coalesce(a.message, '')),
            v_tsquery
        ) as rank,
        ts_headline('english', coalesce(a.title, '') || ' ' || coalesce(a.message, ''), v_tsquery) as headline,
        a.created_at
    FROM room_alerts a
    WHERE a.room_slug = p_room_slug
      AND 'alerts' = ANY(p_content_types)
      AND to_tsvector('english', coalesce(a.ticker, '') || ' ' || coalesce(a.title, '') || ' ' || coalesce(a.message, '')) @@ v_tsquery)

    UNION ALL

    -- Search trades
    (SELECT
        'trade'::TEXT as content_type,
        t.id,
        t.ticker,
        t.ticker as title,
        ts_rank(
            to_tsvector('english', coalesce(t.ticker, '') || ' ' || coalesce(t.notes, '')),
            v_tsquery
        ) as rank,
        ts_headline('english', coalesce(t.notes, ''), v_tsquery) as headline,
        t.created_at
    FROM room_trades t
    WHERE t.room_slug = p_room_slug
      AND 'trades' = ANY(p_content_types)
      AND to_tsvector('english', coalesce(t.ticker, '') || ' ' || coalesce(t.notes, '')) @@ v_tsquery)

    ORDER BY rank DESC
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION search_room_content(TEXT, TEXT, TEXT[], INT, INT) IS 'Simplified search function for room content with relevance ranking';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
