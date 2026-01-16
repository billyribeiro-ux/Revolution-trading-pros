-- Revolution Trading Pros - Seed Membership Plans
-- December 2025
-- Seeds the 6 core services:
-- Trading Rooms: Day Trading Room, Swing Trading Room, Small Account Mentorship
-- Alert Services: Alerts Only, Explosive Swing, SPX Profit Pulse

-- ═══════════════════════════════════════════════════════════════════════════
-- INSERT MEMBERSHIP PLANS
-- ═══════════════════════════════════════════════════════════════════════════

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, is_active, metadata, features, trial_days)
VALUES
    -- Trading Rooms
    (
        'Day Trading Room',
        'day-trading-room',
        'Live day trading sessions with real-time trade alerts and daily market analysis.',
        197.00,
        'monthly',
        true,
        '{"type": "trading-room", "icon": "chart-line", "room_label": "Day Trading Room"}',
        '["Live Day Trading Sessions", "Discord Access", "Real-Time Trade Alerts", "Daily Market Analysis"]',
        7
    ),
    (
        'Swing Trading Room',
        'swing-trading-room',
        'Live swing trading sessions with swing trade alerts and weekly watchlist.',
        147.00,
        'monthly',
        true,
        '{"type": "trading-room", "icon": "trending-up", "room_label": "Swing Trading Room"}',
        '["Live Swing Trading Sessions", "Discord Access", "Swing Trade Alerts", "Weekly Watchlist"]',
        7
    ),
    (
        'Small Account Mentorship',
        'small-account-mentorship',
        'Personalized mentorship for growing small trading accounts with proper risk management.',
        97.00,
        'monthly',
        true,
        '{"type": "trading-room", "icon": "wallet", "room_label": "Small Account Mentorship"}',
        '["Small Account Strategies", "Personalized Mentorship", "Risk Management", "Position Sizing"]',
        7
    ),
    -- Alert Services
    (
        'Alerts Only',
        'alerts-only',
        'Real-time trade alerts with entry points, exit points, and stop loss levels.',
        97.00,
        'monthly',
        true,
        '{"type": "alert-service", "icon": "bell"}',
        '["Real-Time Trade Alerts", "Entry & Exit Points", "Stop Loss Levels", "Mobile Notifications"]',
        0
    ),
    (
        'Explosive Swing',
        'explosive-swing',
        'High momentum swing trade alerts with detailed analysis and risk/reward ratios.',
        147.00,
        'monthly',
        true,
        '{"type": "alert-service", "icon": "rocket"}',
        '["Explosive Swing Trade Alerts", "High Momentum Plays", "Detailed Analysis", "Risk/Reward Ratios"]',
        0
    ),
    (
        'SPX Profit Pulse',
        'spx-profit-pulse',
        'Premium SPX options alerts for intraday opportunities with high win rate setups.',
        197.00,
        'monthly',
        true,
        '{"type": "alert-service", "icon": "activity"}',
        '["SPX Options Alerts", "Intraday Opportunities", "Premium Analysis", "High Win Rate Setups"]',
        0
    ),
    -- Premium Reports
    (
        'Weekly Watchlist',
        'weekly-watchlist',
        'Weekly stock watchlist with detailed analysis and trading opportunities from expert traders.',
        47.00,
        'monthly',
        true,
        '{"type": "premium-report", "icon": "calendar"}',
        '["Weekly Stock Watchlist", "Expert Analysis", "Trading Opportunities", "Downloadable Spreadsheets", "Video Rundowns"]',
        0
    )
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    price = EXCLUDED.price,
    billing_cycle = EXCLUDED.billing_cycle,
    is_active = EXCLUDED.is_active,
    metadata = EXCLUDED.metadata,
    features = EXCLUDED.features,
    trial_days = EXCLUDED.trial_days,
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════
-- ICT 11+: NO HARDCODED EMAILS IN MIGRATIONS
-- Super admin memberships are granted via the bootstrap_developer() function
-- which reads from DEVELOPER_BOOTSTRAP_EMAIL environment variable
-- ═══════════════════════════════════════════════════════════════════════════

-- Grant all memberships to users with super_admin or super-admin role
INSERT INTO user_memberships (user_id, plan_id, starts_at, expires_at, status, current_period_start, current_period_end)
SELECT
    u.id,
    p.id,
    NOW(),
    NOW() + INTERVAL '100 years',
    'active',
    NOW(),
    NOW() + INTERVAL '100 years'
FROM users u
CROSS JOIN membership_plans p
WHERE u.role IN ('super_admin', 'super-admin', 'developer')
ON CONFLICT DO NOTHING;
