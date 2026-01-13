-- Add Weekly Watchlist Membership Plan
-- ═══════════════════════════════════════════════════════════════════════════
-- Apple ICT 11+ Principal Engineer Grade - January 2026
-- 
-- Adds Weekly Watchlist as a standalone premium report membership
-- ═══════════════════════════════════════════════════════════════════════════

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, is_active, metadata, features, trial_days)
VALUES
    (
        'Weekly Watchlist',
        'weekly-watchlist',
        'Weekly stock watchlist with detailed analysis and trading opportunities from expert traders.',
        47.00,
        'monthly',
        true,
        '{"type": "premium-report", "icon": "calendar", "report_type": "weekly-watchlist"}',
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

-- Grant Weekly Watchlist to super admin
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
WHERE u.email = 'welberribeirodrums@gmail.com'
  AND p.slug = 'weekly-watchlist'
ON CONFLICT DO NOTHING;
