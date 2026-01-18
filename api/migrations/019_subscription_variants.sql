-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 019: Complete Subscription Variants System
-- Apple Principal Engineer ICT 11 Grade - January 2026
--
-- PURPOSE: Implement WooCommerce-style subscription IDs where each room has
-- multiple subscription variants (monthly, quarterly, annual), each with its
-- own unique ID and Stripe Price ID.
--
-- STRUCTURE:
-- - Each trading room has 3 subscription plans
-- - Each plan has: unique ID, unique slug, unique stripe_price_id
-- - Plans link to their parent trading_room via room_id
--
-- PRICING STRATEGY:
-- - Monthly: Base price
-- - Quarterly: ~10% discount (3 months for price of ~2.7)
-- - Annual: ~27% discount (12 months for price of ~8.75)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ADD ROOM LINKAGE TO MEMBERSHIP PLANS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add room_id to link membership_plans to trading_rooms
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS room_id BIGINT REFERENCES trading_rooms(id);

-- Add interval_count for Stripe subscription intervals
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS interval_count INTEGER DEFAULT 1;

-- Add stripe_product_id to track the parent Stripe product
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS stripe_product_id VARCHAR(255);

-- Add display_name for UI (e.g., "Explosive Swings - Quarterly")
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS display_name VARCHAR(255);

-- Add savings_percent for displaying discount amount
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS savings_percent INTEGER DEFAULT 0;

-- Add is_popular flag for highlighting recommended plan
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS is_popular BOOLEAN DEFAULT false;

-- Add sort_order for display ordering within a room
ALTER TABLE membership_plans 
ADD COLUMN IF NOT EXISTS sort_order INTEGER DEFAULT 0;

-- Create index for room lookups
CREATE INDEX IF NOT EXISTS idx_membership_plans_room ON membership_plans(room_id);
CREATE INDEX IF NOT EXISTS idx_membership_plans_billing ON membership_plans(billing_cycle);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. UPDATE EXISTING PLANS - Link to rooms and mark as monthly base
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Update Day Trading Room monthly plan
UPDATE membership_plans 
SET room_id = (SELECT id FROM trading_rooms WHERE slug = 'day-trading-room' LIMIT 1),
    display_name = 'Day Trading Room - Monthly',
    interval_count = 1,
    sort_order = 1
WHERE slug = 'day-trading-room';

-- Update Swing Trading Room monthly plan  
UPDATE membership_plans
SET room_id = (SELECT id FROM trading_rooms WHERE slug = 'swing-trading-room' LIMIT 1),
    display_name = 'Swing Trading Room - Monthly',
    interval_count = 1,
    sort_order = 1
WHERE slug = 'swing-trading-room';

-- Update Small Account Mentorship monthly plan
UPDATE membership_plans
SET room_id = (SELECT id FROM trading_rooms WHERE slug = 'small-account-mentorship' LIMIT 1),
    display_name = 'Small Account Mentorship - Monthly',
    interval_count = 1,
    sort_order = 1
WHERE slug = 'small-account-mentorship';

-- Update Explosive Swing monthly plan (link to explosive-swings room)
UPDATE membership_plans
SET room_id = (SELECT id FROM trading_rooms WHERE slug = 'explosive-swings' LIMIT 1),
    display_name = 'Explosive Swings - Monthly',
    interval_count = 1,
    sort_order = 1
WHERE slug = 'explosive-swing';

-- Update SPX Profit Pulse monthly plan
UPDATE membership_plans
SET room_id = (SELECT id FROM trading_rooms WHERE slug = 'spx-profit-pulse' LIMIT 1),
    display_name = 'SPX Profit Pulse - Monthly',
    interval_count = 1,
    sort_order = 1
WHERE slug = 'spx-profit-pulse';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. INSERT ALL SUBSCRIPTION VARIANTS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ───────────────────────────────────────────────────────────────────────────────────
-- DAY TRADING ROOM ($197/mo base)
-- Quarterly: $529 (saves $62 vs $591, ~10% off)
-- Annual: $1,727 (saves $637 vs $2,364, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Day Trading Room - Quarterly',
    'day-trading-room-quarterly',
    'Live day trading sessions with real-time trade alerts and daily market analysis. Billed quarterly.',
    529.00,
    'quarterly',
    3,
    true,
    '{"type": "trading-room", "icon": "chart-line", "room_label": "Day Trading Room", "billing_variant": "quarterly"}',
    '["Live Day Trading Sessions", "Discord Access", "Real-Time Trade Alerts", "Daily Market Analysis", "10% Savings"]',
    0,
    tr.id,
    'Day Trading Room - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'day-trading-room'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Day Trading Room - Annual',
    'day-trading-room-annual',
    'Live day trading sessions with real-time trade alerts and daily market analysis. Billed annually.',
    1727.00,
    'annual',
    12,
    true,
    '{"type": "trading-room", "icon": "chart-line", "room_label": "Day Trading Room", "billing_variant": "annual"}',
    '["Live Day Trading Sessions", "Discord Access", "Real-Time Trade Alerts", "Daily Market Analysis", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'Day Trading Room - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'day-trading-room'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ───────────────────────────────────────────────────────────────────────────────────
-- SWING TRADING ROOM ($147/mo base)
-- Quarterly: $397 (saves $44 vs $441, ~10% off)
-- Annual: $1,297 (saves $467 vs $1,764, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Swing Trading Room - Quarterly',
    'swing-trading-room-quarterly',
    'Live swing trading sessions with swing trade alerts and weekly watchlist. Billed quarterly.',
    397.00,
    'quarterly',
    3,
    true,
    '{"type": "trading-room", "icon": "trending-up", "room_label": "Swing Trading Room", "billing_variant": "quarterly"}',
    '["Live Swing Trading Sessions", "Discord Access", "Swing Trade Alerts", "Weekly Watchlist", "10% Savings"]',
    0,
    tr.id,
    'Swing Trading Room - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'swing-trading-room'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Swing Trading Room - Annual',
    'swing-trading-room-annual',
    'Live swing trading sessions with swing trade alerts and weekly watchlist. Billed annually.',
    1297.00,
    'annual',
    12,
    true,
    '{"type": "trading-room", "icon": "trending-up", "room_label": "Swing Trading Room", "billing_variant": "annual"}',
    '["Live Swing Trading Sessions", "Discord Access", "Swing Trade Alerts", "Weekly Watchlist", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'Swing Trading Room - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'swing-trading-room'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ───────────────────────────────────────────────────────────────────────────────────
-- SMALL ACCOUNT MENTORSHIP ($97/mo base)
-- Quarterly: $262 (saves $29 vs $291, ~10% off)
-- Annual: $857 (saves $307 vs $1,164, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Small Account Mentorship - Quarterly',
    'small-account-mentorship-quarterly',
    'Personalized mentorship for growing small trading accounts. Billed quarterly.',
    262.00,
    'quarterly',
    3,
    true,
    '{"type": "trading-room", "icon": "wallet", "room_label": "Small Account Mentorship", "billing_variant": "quarterly"}',
    '["Small Account Strategies", "Personalized Mentorship", "Risk Management", "Position Sizing", "10% Savings"]',
    0,
    tr.id,
    'Small Account Mentorship - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'small-account-mentorship'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Small Account Mentorship - Annual',
    'small-account-mentorship-annual',
    'Personalized mentorship for growing small trading accounts. Billed annually.',
    857.00,
    'annual',
    12,
    true,
    '{"type": "trading-room", "icon": "wallet", "room_label": "Small Account Mentorship", "billing_variant": "annual"}',
    '["Small Account Strategies", "Personalized Mentorship", "Risk Management", "Position Sizing", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'Small Account Mentorship - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'small-account-mentorship'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ───────────────────────────────────────────────────────────────────────────────────
-- EXPLOSIVE SWINGS ($147/mo base)
-- Quarterly: $397 (saves $44 vs $441, ~10% off)
-- Annual: $1,297 (saves $467 vs $1,764, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Explosive Swings - Quarterly',
    'explosive-swing-quarterly',
    'High momentum swing trade alerts with detailed analysis and risk/reward ratios. Billed quarterly.',
    397.00,
    'quarterly',
    3,
    true,
    '{"type": "alert-service", "icon": "rocket", "room_label": "Explosive Swings", "billing_variant": "quarterly"}',
    '["Explosive Swing Trade Alerts", "High Momentum Plays", "Detailed Analysis", "Risk/Reward Ratios", "TOS Strings", "10% Savings"]',
    0,
    tr.id,
    'Explosive Swings - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'explosive-swings'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'Explosive Swings - Annual',
    'explosive-swing-annual',
    'High momentum swing trade alerts with detailed analysis and risk/reward ratios. Billed annually.',
    1297.00,
    'annual',
    12,
    true,
    '{"type": "alert-service", "icon": "rocket", "room_label": "Explosive Swings", "billing_variant": "annual"}',
    '["Explosive Swing Trade Alerts", "High Momentum Plays", "Detailed Analysis", "Risk/Reward Ratios", "TOS Strings", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'Explosive Swings - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'explosive-swings'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ───────────────────────────────────────────────────────────────────────────────────
-- SPX PROFIT PULSE ($197/mo base)
-- Quarterly: $529 (saves $62 vs $591, ~10% off)
-- Annual: $1,727 (saves $637 vs $2,364, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'SPX Profit Pulse - Quarterly',
    'spx-profit-pulse-quarterly',
    'Premium SPX options alerts for intraday opportunities. Billed quarterly.',
    529.00,
    'quarterly',
    3,
    true,
    '{"type": "alert-service", "icon": "activity", "room_label": "SPX Profit Pulse", "billing_variant": "quarterly"}',
    '["SPX Options Alerts", "Intraday Opportunities", "Premium Analysis", "High Win Rate Setups", "10% Savings"]',
    0,
    tr.id,
    'SPX Profit Pulse - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'spx-profit-pulse'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'SPX Profit Pulse - Annual',
    'spx-profit-pulse-annual',
    'Premium SPX options alerts for intraday opportunities. Billed annually.',
    1727.00,
    'annual',
    12,
    true,
    '{"type": "alert-service", "icon": "activity", "room_label": "SPX Profit Pulse", "billing_variant": "annual"}',
    '["SPX Options Alerts", "Intraday Opportunities", "Premium Analysis", "High Win Rate Setups", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'SPX Profit Pulse - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'spx-profit-pulse'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ───────────────────────────────────────────────────────────────────────────────────
-- HIGH OCTANE SCANNER ($97/mo base) - Adding if not exists
-- Quarterly: $262 (saves $29 vs $291, ~10% off)
-- Annual: $857 (saves $307 vs $1,164, ~27% off)
-- ───────────────────────────────────────────────────────────────────────────────────

-- Monthly
INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'High Octane Scanner - Monthly',
    'high-octane-scanner-monthly',
    'Real-time stock scanner with momentum alerts and breakout detection.',
    97.00,
    'monthly',
    1,
    true,
    '{"type": "scanner", "icon": "radar", "room_label": "High Octane Scanner", "billing_variant": "monthly"}',
    '["Real-Time Scanner", "Momentum Alerts", "Breakout Detection", "Pre-Market Scans", "After-Hours Scans"]',
    7,
    tr.id,
    'High Octane Scanner - Monthly',
    0,
    false,
    1
FROM trading_rooms tr WHERE tr.slug = 'high-octane-scanner'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- Quarterly
INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'High Octane Scanner - Quarterly',
    'high-octane-scanner-quarterly',
    'Real-time stock scanner with momentum alerts and breakout detection. Billed quarterly.',
    262.00,
    'quarterly',
    3,
    true,
    '{"type": "scanner", "icon": "radar", "room_label": "High Octane Scanner", "billing_variant": "quarterly"}',
    '["Real-Time Scanner", "Momentum Alerts", "Breakout Detection", "Pre-Market Scans", "After-Hours Scans", "10% Savings"]',
    0,
    tr.id,
    'High Octane Scanner - Quarterly',
    10,
    true,
    2
FROM trading_rooms tr WHERE tr.slug = 'high-octane-scanner'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    is_popular = EXCLUDED.is_popular,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- Annual
INSERT INTO membership_plans (name, slug, description, price, billing_cycle, interval_count, is_active, metadata, features, trial_days, room_id, display_name, savings_percent, is_popular, sort_order)
SELECT 
    'High Octane Scanner - Annual',
    'high-octane-scanner-annual',
    'Real-time stock scanner with momentum alerts and breakout detection. Billed annually.',
    857.00,
    'annual',
    12,
    true,
    '{"type": "scanner", "icon": "radar", "room_label": "High Octane Scanner", "billing_variant": "annual"}',
    '["Real-Time Scanner", "Momentum Alerts", "Breakout Detection", "Pre-Market Scans", "After-Hours Scans", "27% Savings", "Priority Support"]',
    0,
    tr.id,
    'High Octane Scanner - Annual',
    27,
    false,
    3
FROM trading_rooms tr WHERE tr.slug = 'high-octane-scanner'
ON CONFLICT (slug) DO UPDATE SET
    price = EXCLUDED.price,
    room_id = EXCLUDED.room_id,
    display_name = EXCLUDED.display_name,
    savings_percent = EXCLUDED.savings_percent,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. CREATE VIEW FOR EASY PLAN QUERYING
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE VIEW subscription_plans_view AS
SELECT 
    mp.id AS plan_id,
    mp.name AS plan_name,
    mp.slug AS plan_slug,
    mp.display_name,
    mp.description,
    mp.price,
    mp.billing_cycle,
    mp.interval_count,
    mp.savings_percent,
    mp.is_popular,
    mp.is_active,
    mp.features,
    mp.trial_days,
    mp.stripe_price_id,
    mp.stripe_product_id,
    mp.sort_order,
    tr.id AS room_id,
    tr.name AS room_name,
    tr.slug AS room_slug,
    tr.room_type,
    tr.icon AS room_icon,
    tr.color AS room_color,
    mp.created_at,
    mp.updated_at
FROM membership_plans mp
LEFT JOIN trading_rooms tr ON tr.id = mp.room_id
WHERE mp.is_active = true
ORDER BY tr.sort_order, mp.sort_order;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. CREATE FUNCTION TO GET PLANS BY ROOM
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION get_room_subscription_plans(p_room_slug TEXT)
RETURNS TABLE (
    plan_id BIGINT,
    plan_slug VARCHAR(255),
    display_name VARCHAR(255),
    price DECIMAL(10,2),
    billing_cycle VARCHAR(20),
    interval_count INTEGER,
    savings_percent INTEGER,
    is_popular BOOLEAN,
    features JSONB,
    stripe_price_id VARCHAR(255)
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        mp.id,
        mp.slug,
        mp.display_name,
        mp.price,
        mp.billing_cycle,
        mp.interval_count,
        mp.savings_percent,
        mp.is_popular,
        mp.features,
        mp.stripe_price_id
    FROM membership_plans mp
    JOIN trading_rooms tr ON tr.id = mp.room_id
    WHERE tr.slug = p_room_slug
      AND mp.is_active = true
    ORDER BY mp.sort_order;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. GRANT SUPER ADMIN ACCESS TO NEW PLANS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Grant all new plans to super admins
INSERT INTO user_memberships (user_id, plan_id, starts_at, expires_at, status, current_period_start, current_period_end, payment_provider)
SELECT
    u.id,
    mp.id,
    NOW(),
    NOW() + INTERVAL '100 years',
    'active',
    NOW(),
    NOW() + INTERVAL '100 years',
    'admin'
FROM users u
CROSS JOIN membership_plans mp
WHERE u.role IN ('super_admin', 'super-admin', 'developer')
  AND mp.id NOT IN (
      SELECT plan_id FROM user_memberships WHERE user_id = u.id
  )
ON CONFLICT DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- SUMMARY: SUBSCRIPTION PLAN IDS
-- ═══════════════════════════════════════════════════════════════════════════════════
--
-- After running this migration, plans will have sequential IDs. Example structure:
--
-- Day Trading Room:
--   ID 1: day-trading-room (monthly) - $197/mo
--   ID 8: day-trading-room-quarterly - $529/3mo
--   ID 9: day-trading-room-annual - $1,727/yr
--
-- Swing Trading Room:
--   ID 2: swing-trading-room (monthly) - $147/mo
--   ID 10: swing-trading-room-quarterly - $397/3mo
--   ID 11: swing-trading-room-annual - $1,297/yr
--
-- Small Account Mentorship:
--   ID 3: small-account-mentorship (monthly) - $97/mo
--   ID 12: small-account-mentorship-quarterly - $262/3mo
--   ID 13: small-account-mentorship-annual - $857/yr
--
-- Explosive Swings:
--   ID 5: explosive-swing (monthly) - $147/mo
--   ID 14: explosive-swing-quarterly - $397/3mo
--   ID 15: explosive-swing-annual - $1,297/yr
--
-- SPX Profit Pulse:
--   ID 6: spx-profit-pulse (monthly) - $197/mo
--   ID 16: spx-profit-pulse-quarterly - $529/3mo
--   ID 17: spx-profit-pulse-annual - $1,727/yr
--
-- High Octane Scanner:
--   ID 18: high-octane-scanner-monthly - $97/mo
--   ID 19: high-octane-scanner-quarterly - $262/3mo
--   ID 20: high-octane-scanner-annual - $857/yr
--
-- Each plan needs a corresponding Stripe Price ID set via admin panel or API.
-- ═══════════════════════════════════════════════════════════════════════════════════
