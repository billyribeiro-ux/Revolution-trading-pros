-- Seed Data for Unified Videos
-- ═══════════════════════════════════════════════════════════════════════════
-- Apple ICT 11+ Principal Engineer Grade - January 2026
-- 
-- Populates unified_videos with test data for learning center and daily videos
-- ═══════════════════════════════════════════════════════════════════════════

-- First, ensure trading_rooms table exists and has data
INSERT INTO trading_rooms (id, name, slug, description, is_active) VALUES
    (1, 'Day Trading Room', 'day-trading-room', 'Premium day trading education and live trading sessions', true),
    (2, 'Swing Trading Room', 'swing-trading-room', 'Swing trading strategies and market analysis', true),
    (3, 'Small Accounts Room', 'small-accounts-room', 'Trading strategies for smaller account sizes', true),
    (4, 'Options Room', 'options-room', 'Options trading strategies and education', true),
    (5, 'High Octane Scanner', 'high-octane-scanner', 'Real-time market scanning and alerts', true)
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    slug = EXCLUDED.slug,
    is_active = true;

-- Insert room_traders (traders)
INSERT INTO room_traders (id, name, slug, bio, avatar_url) VALUES
    (1, 'John Carter', 'john-carter', 'Founder and lead trader at Simpler Trading', 'https://cdn.simplertrading.com/2024/01/john-carter.jpg'),
    (2, 'Kody Ashmore', 'kody-ashmore', 'Expert day trader and educator', 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg'),
    (3, 'Henry Gambell', 'henry-gambell', 'Technical analysis and options specialist', 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg'),
    (4, 'Chris Brecher', 'chris-brecher', 'Options and futures trading expert', 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg'),
    (5, 'Danielle Shay', 'danielle-shay', 'Options trading and technical analysis', 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    slug = EXCLUDED.slug;

-- Insert Learning Center Videos
INSERT INTO unified_videos (
    title, slug, description, video_url, video_platform, thumbnail_url,
    duration, content_type, difficulty_level, trader_id, video_date,
    is_published, is_featured, tags, views_count
) VALUES
-- Learning Center Videos
(
    'Q3 Market Outlook July 2025',
    'market-outlook-jul2025-john-carter',
    'Using the economic cycle, John Carter will share insights on what''s next in the stock market, commodities, Treasury yields, bonds, and more.',
    'https://iframe.mediadelivery.net/embed/123/abc123',
    'bunny',
    'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
    3600,
    'learning_center',
    'intermediate',
    1,
    '2025-07-15',
    true,
    true,
    '["trade-setups", "market-review"]',
    1250
),
(
    'Intro to Kody Ashmore''s Daily Sessions (and My Results!)',
    'kody-ashmore-daily-sessions-results',
    'Learn about Kody Ashmore''s daily trading sessions and see real results from following his methodology.',
    'https://iframe.mediadelivery.net/embed/123/def456',
    'bunny',
    'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
    2700,
    'learning_center',
    'beginner',
    2,
    '2025-06-20',
    true,
    false,
    '["methodology", "trade-setups"]',
    890
),
(
    'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
    'the-15-50-trade',
    'Chris Brecher explains how corporate buybacks affect the market in the last 10 minutes of every trading day.',
    'https://iframe.mediadelivery.net/embed/123/ghi789',
    'bunny',
    'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
    1800,
    'learning_center',
    'intermediate',
    4,
    '2025-05-10',
    true,
    false,
    '["member-webinar", "trade-management"]',
    756
),
(
    'Understanding Market Structure',
    'understanding-market-structure',
    'Learn how to identify key market structure levels and use them in your trading.',
    'https://iframe.mediadelivery.net/embed/123/jkl012',
    'bunny',
    'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
    2400,
    'learning_center',
    'beginner',
    3,
    '2025-04-25',
    true,
    true,
    '["methodology", "technical-analysis"]',
    1100
),
(
    'Options Trading Fundamentals',
    'options-trading-fundamentals',
    'Master the basics of options trading with this comprehensive guide.',
    'https://iframe.mediadelivery.net/embed/123/mno345',
    'bunny',
    'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
    3000,
    'learning_center',
    'beginner',
    1,
    '2025-03-15',
    true,
    false,
    '["options", "fundamentals"]',
    2100
),
(
    'Using Squeeze Pro Indicator',
    'squeeze-pro-indicator',
    'Learn how to use the Squeeze Pro indicator to identify high-probability trade setups.',
    'https://iframe.mediadelivery.net/embed/123/pqr678',
    'bunny',
    'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
    2100,
    'learning_center',
    'intermediate',
    1,
    '2025-02-28',
    true,
    true,
    '["indicators", "trade-setups"]',
    1890
),
(
    'Risk Management Essentials',
    'risk-management-essentials',
    'Protect your capital with proven risk management strategies.',
    'https://iframe.mediadelivery.net/embed/123/stu901',
    'bunny',
    'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
    1500,
    'learning_center',
    'beginner',
    4,
    '2025-01-20',
    true,
    false,
    '["risk-management", "trade-management"]',
    980
),
(
    'Futures Trading 101',
    'futures-trading-101',
    'Get started with futures trading - from basics to advanced strategies.',
    'https://iframe.mediadelivery.net/embed/123/vwx234',
    'bunny',
    'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
    2800,
    'learning_center',
    'beginner',
    2,
    '2024-12-15',
    true,
    false,
    '["futures", "fundamentals"]',
    1450
),
(
    'Psychology of Trading',
    'psychology-of-trading',
    'Master your emotions and develop the mindset of a successful trader.',
    'https://iframe.mediadelivery.net/embed/123/yza567',
    'bunny',
    'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
    2200,
    'learning_center',
    'intermediate',
    3,
    '2024-11-10',
    true,
    false,
    '["psychology", "fundamentals"]',
    1320
),

-- Daily Videos
(
    'How to use Bookmap to make more informed trades',
    'bookmap-informed-trades',
    'You asked for it, you got it. Here are Kody''s Bookmap tools and how he uses them to make better informed trades.',
    'https://iframe.mediadelivery.net/embed/123/daily001',
    'bunny',
    'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
    1200,
    'daily_video',
    'intermediate',
    2,
    '2026-01-02',
    true,
    true,
    '["technical-analysis", "indicators"]',
    450
),
(
    'Cautious entry into 2026',
    'cautious-entry-2026',
    'If Santa doesn''t show up, the first bit of 2026 may be a little precarious. With that in mind, let''s dive in to some of the most important charts for the new year.',
    'https://iframe.mediadelivery.net/embed/123/daily002',
    'bunny',
    'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
    900,
    'daily_video',
    'intermediate',
    3,
    '2025-12-31',
    true,
    false,
    '["market-review", "macro-structure"]',
    380
),
(
    'SPX Snoozefest',
    'spx-snoozefest',
    'We''ve had two days of some very narrow ranges in the indices. It''s almost as though the market has had an amazing year and just needs to rest a bit before making its next move!',
    'https://iframe.mediadelivery.net/embed/123/daily003',
    'bunny',
    'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
    720,
    'daily_video',
    'beginner',
    5,
    '2025-12-30',
    true,
    false,
    '["market-review"]',
    290
),
(
    'Year End Portfolio Review',
    'year-end-portfolio-review',
    'John Carter reviews his 2025 portfolio performance and shares lessons learned for the new year.',
    'https://iframe.mediadelivery.net/embed/123/daily004',
    'bunny',
    'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
    1500,
    'daily_video',
    'intermediate',
    1,
    '2025-12-29',
    true,
    true,
    '["trade-management", "psychology"]',
    520
),
(
    'Options Expiration Strategy',
    'options-expiration-strategy',
    'How to handle positions going into options expiration week.',
    'https://iframe.mediadelivery.net/embed/123/daily005',
    'bunny',
    'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
    1100,
    'daily_video',
    'advanced',
    5,
    '2025-12-28',
    true,
    false,
    '["options", "trade-strategies"]',
    410
);

-- Assign videos to trading rooms
INSERT INTO video_room_assignments (video_id, trading_room_id, is_featured_in_room, sort_order) 
SELECT v.id, 1, v.is_featured, ROW_NUMBER() OVER (ORDER BY v.video_date DESC)
FROM unified_videos v
WHERE v.content_type IN ('learning_center', 'daily_video')
ON CONFLICT DO NOTHING;

-- Also assign learning center videos to other rooms
INSERT INTO video_room_assignments (video_id, trading_room_id, is_featured_in_room, sort_order)
SELECT v.id, 2, false, ROW_NUMBER() OVER (ORDER BY v.video_date DESC)
FROM unified_videos v
WHERE v.content_type = 'learning_center'
ON CONFLICT DO NOTHING;

INSERT INTO video_room_assignments (video_id, trading_room_id, is_featured_in_room, sort_order)
SELECT v.id, 3, false, ROW_NUMBER() OVER (ORDER BY v.video_date DESC)
FROM unified_videos v
WHERE v.content_type = 'learning_center'
ON CONFLICT DO NOTHING;

-- Log success
DO $$
BEGIN
    RAISE NOTICE 'Unified videos seed data inserted successfully';
END $$;
