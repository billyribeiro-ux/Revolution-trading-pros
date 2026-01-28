-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 031: Analytics Helper Functions
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Apple Principal Engineer ICT 7+ Grade - January 2026
--
-- Implements analytics helper functions for trading room performance:
-- - Equity curve calculation
-- - Ticker performance aggregation
-- - Monthly returns analysis
--
-- These functions are optimized for dashboard widgets and reporting.
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. EQUITY CURVE FUNCTION
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Returns cumulative P&L over time for charting equity growth

CREATE OR REPLACE FUNCTION get_equity_curve(
    p_room_slug TEXT,
    p_from_date DATE DEFAULT NULL,
    p_to_date DATE DEFAULT NULL
) RETURNS TABLE (
    trade_date DATE,
    cumulative_pnl NUMERIC,
    trade_id BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.exit_date::DATE as trade_date,
        SUM(t.pnl) OVER (ORDER BY t.exit_date) as cumulative_pnl,
        t.id as trade_id
    FROM room_trades t
    WHERE t.room_slug = p_room_slug
      AND t.status = 'closed'
      AND (p_from_date IS NULL OR t.exit_date >= p_from_date)
      AND (p_to_date IS NULL OR t.exit_date <= p_to_date)
    ORDER BY t.exit_date;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION get_equity_curve IS 'Returns equity curve data with cumulative P&L for charting';
GRANT EXECUTE ON FUNCTION get_equity_curve TO PUBLIC;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. TICKER PERFORMANCE FUNCTION
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Aggregates performance metrics by ticker symbol

CREATE OR REPLACE FUNCTION get_ticker_performance(
    p_room_slug TEXT
) RETURNS TABLE (
    ticker VARCHAR,
    total_trades BIGINT,
    wins BIGINT,
    losses BIGINT,
    win_rate NUMERIC,
    total_pnl NUMERIC,
    avg_pnl NUMERIC
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.ticker,
        COUNT(*)::BIGINT as total_trades,
        COUNT(*) FILTER (WHERE t.result = 'WIN')::BIGINT as wins,
        COUNT(*) FILTER (WHERE t.result = 'LOSS')::BIGINT as losses,
        ROUND(
            COUNT(*) FILTER (WHERE t.result = 'WIN')::NUMERIC /
            NULLIF(COUNT(*)::NUMERIC, 0) * 100, 2
        ) as win_rate,
        COALESCE(SUM(t.pnl), 0) as total_pnl,
        ROUND(COALESCE(AVG(t.pnl), 0), 2) as avg_pnl
    FROM room_trades t
    WHERE t.room_slug = p_room_slug AND t.status = 'closed'
    GROUP BY t.ticker
    ORDER BY total_pnl DESC;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION get_ticker_performance IS 'Returns aggregated performance metrics by ticker symbol';
GRANT EXECUTE ON FUNCTION get_ticker_performance TO PUBLIC;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. MONTHLY RETURNS FUNCTION
-- ═══════════════════════════════════════════════════════════════════════════════════
-- Returns monthly performance breakdown for historical analysis

CREATE OR REPLACE FUNCTION get_monthly_returns(
    p_room_slug TEXT,
    p_months INT DEFAULT 12
) RETURNS TABLE (
    month TEXT,
    pnl NUMERIC,
    trades BIGINT,
    win_rate NUMERIC
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        to_char(t.exit_date, 'YYYY-MM') as month,
        COALESCE(SUM(t.pnl), 0) as pnl,
        COUNT(*)::BIGINT as trades,
        ROUND(
            COUNT(*) FILTER (WHERE t.result = 'WIN')::NUMERIC /
            NULLIF(COUNT(*)::NUMERIC, 0) * 100, 2
        ) as win_rate
    FROM room_trades t
    WHERE t.room_slug = p_room_slug
      AND t.status = 'closed'
      AND t.exit_date >= (CURRENT_DATE - (p_months || ' months')::INTERVAL)
    GROUP BY to_char(t.exit_date, 'YYYY-MM')
    ORDER BY month DESC;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION get_monthly_returns IS 'Returns monthly P&L breakdown for historical performance analysis';
GRANT EXECUTE ON FUNCTION get_monthly_returns TO PUBLIC;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
