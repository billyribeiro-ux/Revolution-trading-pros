-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 018: Complete Explosive Swings Schema
-- Apple Principal Engineer ICT 11 Grade - January 2026
--
-- Adds all missing columns to support full TOS (ThinkOrSwim) trading functionality:
-- - Enhanced room_alerts with TOS format support
-- - Enhanced room_trade_plans with all target levels
-- - New room_trades table for trade tracking
-- - Enhanced room_stats_cache with full metrics
-- - Seed trading_rooms with explosive-swings if not exists
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. SEED TRADING ROOMS (ensure explosive-swings exists)
-- ═══════════════════════════════════════════════════════════════════════════════════

INSERT INTO trading_rooms (id, name, slug, description, short_description, room_type, is_active, is_featured, is_public, sort_order, available_sections, features, icon, color)
VALUES
    (1, 'Day Trading Room', 'day-trading-room', 'Live day trading sessions with real-time trade alerts and daily market analysis.', 'Live day trading alerts & analysis', 'trading_room', true, true, true, 1, '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"]'::jsonb, '{"live_sessions": true, "alerts": true}'::jsonb, 'chart-line', '#3b82f6'),
    (2, 'Swing Trading Room', 'swing-trading-room', 'Live swing trading sessions with swing trade alerts and weekly watchlist.', 'Swing trade alerts & weekly watchlist', 'trading_room', true, false, true, 2, '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"]'::jsonb, '{"weekly_watchlist": true, "alerts": true}'::jsonb, 'trending-up', '#10b981'),
    (3, 'Small Account Mentorship', 'small-account-mentorship', 'Personalized mentorship for growing small trading accounts with proper risk management.', 'Grow small accounts with expert guidance', 'mentorship', true, false, true, 3, '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb, '{"mentorship": true}'::jsonb, 'wallet', '#f59e0b'),
    (4, 'Explosive Swings', 'explosive-swings', 'High momentum swing trade alerts with detailed analysis, TOS entry/exit strings, and risk/reward ratios. Weekly video breakdowns with exact entry, target, and stop levels.', 'High momentum swing trade alerts', 'alert_service', true, true, true, 4, '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "weekly_alerts", "trade_plan", "learning_center"]'::jsonb, '{"tos_alerts": true, "trade_plan": true, "weekly_video": true}'::jsonb, 'rocket', '#f69532'),
    (5, 'SPX Profit Pulse', 'spx-profit-pulse', 'Premium SPX options alerts for intraday opportunities with high win rate setups.', 'Premium SPX options alerts', 'alert_service', true, false, true, 5, '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb, '{"spx_alerts": true}'::jsonb, 'activity', '#8b5cf6'),
    (6, 'High Octane Scanner', 'high-octane-scanner', 'Real-time stock scanner with momentum alerts and breakout detection.', 'Real-time momentum scanner', 'alert_service', true, false, true, 6, '["introduction", "latest_updates", "premium_daily_videos", "learning_center"]'::jsonb, '{"scanner": true}'::jsonb, 'radar', '#06b6d4')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    short_description = EXCLUDED.short_description,
    available_sections = EXCLUDED.available_sections,
    features = EXCLUDED.features,
    icon = EXCLUDED.icon,
    color = EXCLUDED.color,
    updated_at = NOW();

-- Also insert by slug if id conflicts exist
INSERT INTO trading_rooms (name, slug, description, short_description, room_type, is_active, is_featured, is_public, sort_order, available_sections, features, icon, color)
VALUES
    ('Explosive Swings', 'explosive-swings', 'High momentum swing trade alerts with detailed analysis, TOS entry/exit strings, and risk/reward ratios.', 'High momentum swing trade alerts', 'alert_service', true, true, true, 4, '["introduction", "latest_updates", "premium_daily_videos", "watchlist", "weekly_alerts", "trade_plan", "learning_center"]'::jsonb, '{"tos_alerts": true, "trade_plan": true, "weekly_video": true}'::jsonb, 'rocket', '#f69532')
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ENHANCE ROOM_ALERTS TABLE (TOS Format Support)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add missing columns for full TOS support
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS title VARCHAR(500);
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS notes TEXT;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS trade_type VARCHAR(20); -- 'options' | 'shares'
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS action VARCHAR(10); -- 'BUY' | 'SELL'
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS quantity INTEGER;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS option_type VARCHAR(10); -- 'CALL' | 'PUT'
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS strike DECIMAL(10,2);
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS expiration DATE;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS contract_type VARCHAR(20); -- 'Weeklys' | 'Monthly' | 'LEAPS'
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS order_type VARCHAR(10); -- 'MKT' | 'LMT'
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS limit_price DECIMAL(10,2);
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS fill_price DECIMAL(10,2);
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS tos_string TEXT; -- Full TOS format string
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS entry_alert_id BIGINT REFERENCES room_alerts(id);
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS trade_plan_id BIGINT;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS is_new BOOLEAN DEFAULT true;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT true;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS is_pinned BOOLEAN DEFAULT false;
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE room_alerts ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_room_alerts_room_slug ON room_alerts(room_slug);
CREATE INDEX IF NOT EXISTS idx_room_alerts_type ON room_alerts(alert_type);
CREATE INDEX IF NOT EXISTS idx_room_alerts_ticker ON room_alerts(ticker);
CREATE INDEX IF NOT EXISTS idx_room_alerts_published ON room_alerts(is_published, published_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ENHANCE ROOM_TRADE_PLANS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add any missing columns
ALTER TABLE room_trade_plans ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE room_trade_plans ADD COLUMN IF NOT EXISTS runner VARCHAR(20);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_room_trade_plans_room ON room_trade_plans(room_slug, week_of DESC);
CREATE INDEX IF NOT EXISTS idx_room_trade_plans_ticker ON room_trade_plans(ticker);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. CREATE ROOM_TRADES TABLE (Trade Tracker)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_trades (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    ticker VARCHAR(10) NOT NULL,
    trade_type VARCHAR(20) NOT NULL DEFAULT 'shares', -- 'options' | 'shares'
    direction VARCHAR(10) NOT NULL DEFAULT 'long', -- 'long' | 'short'
    quantity INTEGER NOT NULL,
    -- Options specific
    option_type VARCHAR(10), -- 'CALL' | 'PUT'
    strike DECIMAL(10,2),
    expiration DATE,
    contract_type VARCHAR(20),
    -- Entry
    entry_alert_id BIGINT REFERENCES room_alerts(id),
    entry_price DECIMAL(10,2) NOT NULL,
    entry_date DATE NOT NULL,
    entry_tos_string TEXT,
    -- Exit (null if open)
    exit_alert_id BIGINT REFERENCES room_alerts(id),
    exit_price DECIMAL(10,2),
    exit_date DATE,
    exit_tos_string TEXT,
    -- Setup & Status
    setup VARCHAR(50), -- 'Breakout' | 'Momentum' | 'Reversal' | 'Earnings' | 'Pullback'
    status VARCHAR(20) NOT NULL DEFAULT 'open', -- 'open' | 'closed' | 'partial'
    result VARCHAR(10), -- 'WIN' | 'LOSS' | null if open
    -- P&L
    pnl DECIMAL(12,2),
    pnl_percent DECIMAL(6,2),
    holding_days INTEGER,
    -- Notes
    notes TEXT,
    -- Audit
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_room_trades_room ON room_trades(room_slug);
CREATE INDEX IF NOT EXISTS idx_room_trades_ticker ON room_trades(ticker);
CREATE INDEX IF NOT EXISTS idx_room_trades_status ON room_trades(status);
CREATE INDEX IF NOT EXISTS idx_room_trades_entry_date ON room_trades(entry_date DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. ENHANCE ROOM_STATS_CACHE TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add missing columns for comprehensive stats
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS win_rate DECIMAL(5,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS weekly_profit VARCHAR(50);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS monthly_profit VARCHAR(50);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS active_trades INTEGER DEFAULT 0;
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS closed_this_week INTEGER DEFAULT 0;
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS total_trades INTEGER DEFAULT 0;
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS wins INTEGER DEFAULT 0;
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS losses INTEGER DEFAULT 0;
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS avg_win DECIMAL(12,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS avg_loss DECIMAL(12,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS profit_factor DECIMAL(6,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS avg_holding_days DECIMAL(6,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS largest_win DECIMAL(12,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS largest_loss DECIMAL(12,2);
ALTER TABLE room_stats_cache ADD COLUMN IF NOT EXISTS current_streak INTEGER DEFAULT 0;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. CREATE ROOM_WEEKLY_VIDEOS TABLE (if not exists)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS room_weekly_videos (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    week_of DATE NOT NULL,
    week_title VARCHAR(255) NOT NULL,
    video_title VARCHAR(500) NOT NULL,
    video_url TEXT NOT NULL,
    video_platform VARCHAR(50) DEFAULT 'bunny',
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    thumbnail_url TEXT,
    duration VARCHAR(20),
    description TEXT,
    is_current BOOLEAN DEFAULT true,
    is_published BOOLEAN DEFAULT true,
    published_at TIMESTAMPTZ DEFAULT NOW(),
    archived_at TIMESTAMPTZ,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(room_slug, week_of)
);

CREATE INDEX IF NOT EXISTS idx_room_weekly_videos_room ON room_weekly_videos(room_slug, is_current);
CREATE INDEX IF NOT EXISTS idx_room_weekly_videos_week ON room_weekly_videos(week_of DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. SEED INITIAL DATA FOR EXPLOSIVE SWINGS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Get the room_id for explosive-swings
DO $$
DECLARE
    v_room_id BIGINT;
BEGIN
    SELECT id INTO v_room_id FROM trading_rooms WHERE slug = 'explosive-swings' LIMIT 1;

    IF v_room_id IS NULL THEN
        v_room_id := 4; -- fallback
    END IF;

    -- Insert sample trade plan entries for current week
    INSERT INTO room_trade_plans (room_id, room_slug, week_of, ticker, bias, entry, target1, target2, target3, runner, stop, options_strike, options_exp, notes, sort_order, is_active)
    VALUES
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'NVDA', 'BULLISH', '$142.50', '$148.00', '$152.00', '$158.00', '$165.00+', '$136.00', '$145 Call', (CURRENT_DATE + INTERVAL '7 days')::DATE, 'Breakout above consolidation. Wait for pullback to entry. Strong AI momentum.', 1, true),
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'TSLA', 'BULLISH', '$248.00', '$255.00', '$265.00', '$275.00', '$290.00+', '$235.00', '$250 Call', (CURRENT_DATE + INTERVAL '14 days')::DATE, 'Momentum building. Earnings catalyst ahead. Watch for pullback to entry.', 2, true),
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'AMZN', 'BULLISH', '$185.00', '$190.00', '$195.00', '$198.00', '$205.00+', '$178.00', '$185 Call', (CURRENT_DATE + INTERVAL '7 days')::DATE, 'Breaking resistance. Strong volume confirmation needed.', 3, true),
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'GOOGL', 'NEUTRAL', '$175.50', '$180.00', '$185.00', '$188.00', '$195.00+', '$168.00', '$177.50 Call', (CURRENT_DATE + INTERVAL '21 days')::DATE, 'Watching for breakout confirmation. Not triggered yet.', 4, true),
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'META', 'BULLISH', '$585.00', '$600.00', '$615.00', '$630.00', '$650.00+', '$565.00', '$590 Call', (CURRENT_DATE + INTERVAL '7 days')::DATE, 'Strong trend continuation. Buy dips to support level.', 5, true),
        (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'AMD', 'BEARISH', '$125.00', '$120.00', '$115.00', '$110.00', '$100.00', '$132.00', '$122 Put', (CURRENT_DATE + INTERVAL '14 days')::DATE, 'Breakdown in progress. Short on bounces to resistance.', 6, true)
    ON CONFLICT DO NOTHING;

    -- Insert sample alerts
    INSERT INTO room_alerts (room_id, room_slug, alert_type, ticker, title, message, notes, trade_type, action, quantity, order_type, limit_price, fill_price, tos_string, is_new, is_published, published_at)
    VALUES
        (v_room_id, 'explosive-swings', 'ENTRY', 'NVDA', 'Opening NVDA Swing Position', 'Entering NVDA at $142.50. First target $148, stop at $136. See trade plan for full details.', 'Entry based on breakout above $142 resistance with strong volume confirmation. RSI at 62 showing momentum. Watch for pullback to $140 support if entry missed. Position size: 150 shares. Risk/reward: 2.8:1 to T2.', 'shares', 'BUY', 150, 'LMT', 142.50, 142.50, 'BUY +150 NVDA @142.50 LMT', true, true, NOW()),
        (v_room_id, 'explosive-swings', 'UPDATE', 'TSLA', 'TSLA Approaching Entry Zone', 'TSLA pulling back to our entry zone. Be ready. Will alert when triggered.', 'Watching $248 entry level closely. Pullback is healthy after recent run. Volume declining on pullback (bullish). If entry triggers, will send immediate alert with exact entry price.', NULL, NULL, NULL, NULL, NULL, NULL, NULL, true, true, NOW() - INTERVAL '1 hour'),
        (v_room_id, 'explosive-swings', 'EXIT', 'MSFT', 'Closing MSFT for +8.2%', 'Taking profits on MSFT. Hit second target. +$2,450 on this trade.', 'Excellent trade execution. Entered at $425, scaled out 1/3 at T1 ($435), another 1/3 at T2 ($445). Final exit at $460. Held for 5 days. Key lesson: Patience paid off.', 'shares', 'SELL', 100, 'LMT', 460.00, 460.00, 'SELL -100 MSFT @460.00 LMT', false, true, NOW() - INTERVAL '1 day'),
        (v_room_id, 'explosive-swings', 'ENTRY', 'META', 'META Entry Triggered', 'META hit our entry at $585. Position active. Targets in trade plan.', 'Entry confirmed at $585 with volume spike. Stop placed at $565 (3.4% risk). Momentum strong with AI revenue narrative. Will trail stop after T1 hit.', 'options', 'BUY', 2, 'LMT', 12.50, 12.50, 'BUY +2 META 100 (Weeklys) 24 JAN 26 590 CALL @12.50 LMT', false, true, NOW() - INTERVAL '2 days'),
        (v_room_id, 'explosive-swings', 'UPDATE', 'AMD', 'AMD Short Setup Active', 'Bearish setup triggered on AMD. Short at $125 with stop at $132.', 'Bearish breakdown confirmed. Entered short at $125, currently at $123.50. Stop at $132 gives us 5.6% risk. First target $120, second target $115.', 'shares', 'SELL', 200, 'LMT', 125.00, 125.00, 'SELL -200 AMD @125.00 LMT', false, true, NOW() - INTERVAL '7 days')
    ON CONFLICT DO NOTHING;

    -- Insert sample stats
    INSERT INTO room_stats_cache (room_id, room_slug, win_rate, weekly_profit, monthly_profit, active_trades, closed_this_week, total_trades, wins, losses, avg_win, avg_loss, profit_factor, avg_holding_days, largest_win, largest_loss, current_streak)
    VALUES (v_room_id, 'explosive-swings', 82.00, '+$4,850', '+$18,750', 4, 2, 28, 23, 5, 1250.00, 450.00, 2.78, 4.5, 3200.00, 680.00, 3)
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
        calculated_at = NOW();

    -- Insert sample weekly video
    INSERT INTO room_weekly_videos (room_id, room_slug, week_of, week_title, video_title, video_url, video_platform, thumbnail_url, duration, description, is_current, is_published)
    VALUES (v_room_id, 'explosive-swings', CURRENT_DATE - EXTRACT(DOW FROM CURRENT_DATE)::INTEGER, 'Week of ' || TO_CHAR(CURRENT_DATE, 'Month DD, YYYY'), 'Weekly Breakdown: Top Swing Setups', 'https://iframe.mediadelivery.net/embed/123456/sample-guid', 'bunny', 'https://placehold.co/1280x720/143E59/FFFFFF/png?text=Weekly+Video+Breakdown', '24:35', 'Complete breakdown of this weeks top swing trade setups including NVDA, TSLA, and META.', true, true)
    ON CONFLICT (room_slug, week_of) DO UPDATE SET
        week_title = EXCLUDED.week_title,
        video_title = EXCLUDED.video_title,
        is_current = true,
        updated_at = NOW();

END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
