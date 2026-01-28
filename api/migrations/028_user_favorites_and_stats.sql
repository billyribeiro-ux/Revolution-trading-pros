-- User Favorites & Real-time Stats Calculation
-- Apple Principal Engineer ICT 7+ Grade - January 2026
-- ═══════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════
-- USER FAVORITES TABLE
-- Persisted favorites for alerts, videos, trade plans per user
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_favorites (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_slug VARCHAR(100) NOT NULL,
    item_type VARCHAR(50) NOT NULL, -- 'alert', 'video', 'trade_plan', 'resource'
    item_id BIGINT NOT NULL,
    title VARCHAR(500),
    excerpt TEXT,
    href VARCHAR(500),
    thumbnail_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint: one favorite per user per item
    UNIQUE(user_id, item_type, item_id)
);

-- Indexes for fast lookups
CREATE INDEX IF NOT EXISTS idx_user_favorites_user_room ON user_favorites(user_id, room_slug);
CREATE INDEX IF NOT EXISTS idx_user_favorites_item ON user_favorites(item_type, item_id);

-- ═══════════════════════════════════════════════════════════════════════════════
-- ROOM STATS CACHE TABLE (if not exists)
-- Real-time calculated stats from trades
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_stats_cache (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT,
    room_slug VARCHAR(100) NOT NULL UNIQUE,
    win_rate DECIMAL(5,2),
    weekly_profit VARCHAR(50),
    monthly_profit VARCHAR(50),
    active_trades INTEGER DEFAULT 0,
    closed_this_week INTEGER DEFAULT 0,
    total_trades INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    avg_win DECIMAL(12,2),
    avg_loss DECIMAL(12,2),
    profit_factor DECIMAL(6,2),
    avg_holding_days DECIMAL(6,2),
    largest_win DECIMAL(12,2),
    largest_loss DECIMAL(12,2),
    current_streak INTEGER DEFAULT 0,
    -- 30-day chart data (JSON array of daily P&L)
    daily_pnl_30d JSONB DEFAULT '[]',
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════
-- STATS CALCULATION FUNCTION
-- Recalculates room stats from room_trades table
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION calculate_room_stats(p_room_slug VARCHAR)
RETURNS void AS $$
DECLARE
    v_wins INTEGER;
    v_losses INTEGER;
    v_total INTEGER;
    v_win_rate DECIMAL(5,2);
    v_active INTEGER;
    v_closed_week INTEGER;
    v_total_pnl DECIMAL(12,2);
    v_weekly_pnl DECIMAL(12,2);
    v_monthly_pnl DECIMAL(12,2);
    v_avg_win DECIMAL(12,2);
    v_avg_loss DECIMAL(12,2);
    v_profit_factor DECIMAL(6,2);
    v_avg_days DECIMAL(6,2);
    v_largest_win DECIMAL(12,2);
    v_largest_loss DECIMAL(12,2);
    v_streak INTEGER;
    v_daily_pnl JSONB;
BEGIN
    -- Count wins and losses
    SELECT 
        COALESCE(SUM(CASE WHEN result = 'WIN' THEN 1 ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN result = 'LOSS' THEN 1 ELSE 0 END), 0),
        COUNT(*)
    INTO v_wins, v_losses, v_total
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL;
    
    -- Calculate win rate
    IF v_total > 0 THEN
        v_win_rate := (v_wins::DECIMAL / v_total::DECIMAL) * 100;
    ELSE
        v_win_rate := 0;
    END IF;
    
    -- Count active trades
    SELECT COUNT(*) INTO v_active
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'open' AND deleted_at IS NULL;
    
    -- Count closed this week
    SELECT COUNT(*) INTO v_closed_week
    FROM room_trades 
    WHERE room_slug = p_room_slug 
    AND status = 'closed' 
    AND deleted_at IS NULL
    AND exit_date >= CURRENT_DATE - INTERVAL '7 days';
    
    -- Calculate P&L metrics
    SELECT 
        COALESCE(SUM(pnl), 0),
        COALESCE(SUM(CASE WHEN exit_date >= CURRENT_DATE - INTERVAL '7 days' THEN pnl ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN exit_date >= CURRENT_DATE - INTERVAL '30 days' THEN pnl ELSE 0 END), 0),
        COALESCE(AVG(CASE WHEN result = 'WIN' THEN pnl END), 0),
        COALESCE(AVG(CASE WHEN result = 'LOSS' THEN ABS(pnl) END), 0),
        COALESCE(AVG(holding_days), 0),
        COALESCE(MAX(CASE WHEN pnl > 0 THEN pnl END), 0),
        COALESCE(MAX(CASE WHEN pnl < 0 THEN ABS(pnl) END), 0)
    INTO v_total_pnl, v_weekly_pnl, v_monthly_pnl, v_avg_win, v_avg_loss, v_avg_days, v_largest_win, v_largest_loss
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL;
    
    -- Calculate profit factor
    IF v_avg_loss > 0 THEN
        v_profit_factor := (v_wins * v_avg_win) / NULLIF((v_losses * v_avg_loss), 0);
    ELSE
        v_profit_factor := v_avg_win;
    END IF;
    
    -- Calculate current streak (consecutive wins or losses)
    -- Fixed: avoid nested window functions by using subquery
    WITH recent_trades AS (
        SELECT result, ROW_NUMBER() OVER (ORDER BY exit_date DESC, id DESC) as rn
        FROM room_trades 
        WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL
        ORDER BY exit_date DESC, id DESC
        LIMIT 20
    ),
    with_lag AS (
        SELECT result, rn, LAG(result) OVER (ORDER BY rn) as prev_result
        FROM recent_trades
    ),
    streak_calc AS (
        SELECT result, rn,
            SUM(CASE WHEN result != prev_result OR prev_result IS NULL THEN 1 ELSE 0 END) OVER (ORDER BY rn) as grp
        FROM with_lag
    )
    SELECT COUNT(*) INTO v_streak
    FROM streak_calc
    WHERE grp = 1;
    
    -- Calculate 30-day daily P&L for chart
    -- Fixed: compute cumulative in CTE first, then aggregate
    WITH daily_dates AS (
        SELECT generate_series(
            CURRENT_DATE - INTERVAL '29 days',
            CURRENT_DATE,
            INTERVAL '1 day'
        )::DATE as date
    ),
    daily_trades AS (
        SELECT exit_date, SUM(pnl) as daily_pnl
        FROM room_trades
        WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL
        AND exit_date >= CURRENT_DATE - INTERVAL '30 days'
        GROUP BY exit_date
    ),
    daily_with_pnl AS (
        SELECT 
            d.date,
            COALESCE(t.daily_pnl, 0) as pnl,
            SUM(COALESCE(t.daily_pnl, 0)) OVER (ORDER BY d.date) as cumulative
        FROM daily_dates d
        LEFT JOIN daily_trades t ON d.date = t.exit_date
    )
    SELECT COALESCE(jsonb_agg(
        jsonb_build_object(
            'date', date::TEXT,
            'pnl', pnl,
            'cumulative', cumulative
        ) ORDER BY date
    ), '[]'::jsonb)
    INTO v_daily_pnl
    FROM daily_with_pnl;
    
    -- Upsert stats cache
    INSERT INTO room_stats_cache (
        room_slug, win_rate, weekly_profit, monthly_profit, active_trades,
        closed_this_week, total_trades, wins, losses, avg_win, avg_loss,
        profit_factor, avg_holding_days, largest_win, largest_loss, 
        current_streak, daily_pnl_30d, calculated_at
    ) VALUES (
        p_room_slug, v_win_rate, 
        CASE WHEN v_weekly_pnl >= 0 THEN '+$' || v_weekly_pnl::TEXT ELSE '-$' || ABS(v_weekly_pnl)::TEXT END,
        CASE WHEN v_monthly_pnl >= 0 THEN '+$' || v_monthly_pnl::TEXT ELSE '-$' || ABS(v_monthly_pnl)::TEXT END,
        v_active, v_closed_week, v_wins + v_losses, v_wins, v_losses,
        v_avg_win, v_avg_loss, COALESCE(v_profit_factor, 0), v_avg_days,
        v_largest_win, v_largest_loss, v_streak, v_daily_pnl, NOW()
    )
    ON CONFLICT (room_slug) DO UPDATE SET
        win_rate = EXCLUDED.win_rate,
        weekly_profit = EXCLUDED.weekly_profit,
        monthly_profit = EXCLUDED.monthly_profit,
        active_trades = EXCLUDED.active_trades,
        closed_this_week = EXCLUDED.closed_this_week,
        total_trades = EXCLUDED.total_trades,
        wins = EXCLUDED.wins,
        losses = EXCLUDED.losses,
        avg_win = EXCLUDED.avg_win,
        avg_loss = EXCLUDED.avg_loss,
        profit_factor = EXCLUDED.profit_factor,
        avg_holding_days = EXCLUDED.avg_holding_days,
        largest_win = EXCLUDED.largest_win,
        largest_loss = EXCLUDED.largest_loss,
        current_streak = EXCLUDED.current_streak,
        daily_pnl_30d = EXCLUDED.daily_pnl_30d,
        calculated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════
-- TRIGGER: Auto-recalculate stats when trades change
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION trigger_recalculate_room_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- Recalculate stats for the affected room
    PERFORM calculate_room_stats(COALESCE(NEW.room_slug, OLD.room_slug));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_recalculate_stats ON room_trades;
CREATE TRIGGER trg_recalculate_stats
    AFTER INSERT OR UPDATE OR DELETE ON room_trades
    FOR EACH ROW
    EXECUTE FUNCTION trigger_recalculate_room_stats();

-- ═══════════════════════════════════════════════════════════════════════════════
-- SEED INITIAL STATS FOR EXISTING ROOMS
-- ═══════════════════════════════════════════════════════════════════════════════

SELECT calculate_room_stats('explosive-swings');
SELECT calculate_room_stats('small-account-mentorship');
SELECT calculate_room_stats('spx-profit-pulse');

-- ═══════════════════════════════════════════════════════════════════════════════
-- GRANT PERMISSIONS (Optional - only if 'authenticated' role exists)
-- ═══════════════════════════════════════════════════════════════════════════════

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'authenticated') THEN
        GRANT SELECT, INSERT, UPDATE, DELETE ON user_favorites TO authenticated;
        GRANT SELECT ON room_stats_cache TO authenticated;
        GRANT USAGE, SELECT ON SEQUENCE user_favorites_id_seq TO authenticated;
    END IF;
END $$;
