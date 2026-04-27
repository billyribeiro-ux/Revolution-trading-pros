-- ═══════════════════════════════════════════════════════════════════════════
-- 045: Blog post metadata columns + seed predefined categories
-- ═══════════════════════════════════════════════════════════════════════════
-- The frontend admin form has been collecting these fields for months but
-- the Rust DTO didn't accept them and the table didn't have columns. Result:
-- silent data loss for featured-image alt text, captions, meta keywords, and
-- comment-allow toggles. See BLOG_SYSTEM_AUDIT.md §2 for the audit record.
--
-- This migration is additive only — no existing data is touched.
-- ═══════════════════════════════════════════════════════════════════════════

-- ─── posts: featured-image metadata, meta_keywords, allow_comments ────────
ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS featured_media_id           BIGINT,
    ADD COLUMN IF NOT EXISTS featured_image_alt          TEXT,
    ADD COLUMN IF NOT EXISTS featured_image_caption      TEXT,
    ADD COLUMN IF NOT EXISTS featured_image_title        VARCHAR(255),
    ADD COLUMN IF NOT EXISTS featured_image_description  TEXT,
    ADD COLUMN IF NOT EXISTS meta_keywords               TEXT[],
    ADD COLUMN IF NOT EXISTS allow_comments              BOOLEAN NOT NULL DEFAULT TRUE;

COMMENT ON COLUMN posts.featured_media_id          IS 'Optional FK to a media table once one is wired; today this is a free integer for forward-compat.';
COMMENT ON COLUMN posts.featured_image_alt         IS 'Accessibility / SEO alt text for the featured image.';
COMMENT ON COLUMN posts.featured_image_caption     IS 'Caption shown below the featured image.';
COMMENT ON COLUMN posts.featured_image_title       IS 'Display title overlay or hover-text for the featured image.';
COMMENT ON COLUMN posts.featured_image_description IS 'Long-form description; useful for image SEO.';
COMMENT ON COLUMN posts.meta_keywords              IS 'Legacy SEO meta keywords (kept for completeness).';
COMMENT ON COLUMN posts.allow_comments             IS 'Whether to show comments on this post.';

-- ─── Seed the 18 predefined blog categories the admin UI hard-codes ───────
-- These match `predefinedCategories` in frontend/src/lib/data/predefined-categories.ts.
-- ON CONFLICT keeps the migration idempotent.
INSERT INTO categories (name, slug, description) VALUES
    ('Market Analysis',         'market-analysis',         'Market analysis posts'),
    ('Trading Strategies',      'trading-strategies',      'Trading strategy posts'),
    ('Risk Management',         'risk-management',         'Risk management posts'),
    ('Options Trading',         'options-trading',         'Options trading posts'),
    ('Technical Analysis',      'technical-analysis',      'Technical analysis posts'),
    ('Fundamental Analysis',    'fundamental-analysis',    'Fundamental analysis posts'),
    ('Psychology',              'psychology',              'Trading psychology posts'),
    ('Education',               'education',               'Educational posts'),
    ('News & Updates',          'news',                    'News and updates'),
    ('Earnings',                'earnings',                'Earnings coverage'),
    ('Stocks',                  'stocks',                  'Stock market posts'),
    ('Futures',                 'futures',                 'Futures market posts'),
    ('Forex',                   'forex',                   'Forex market posts'),
    ('Crypto',                  'crypto',                  'Crypto market posts'),
    ('Small Accounts',          'small-accounts',          'Small-account trading'),
    ('Day Trading',             'day-trading',             'Day-trading posts'),
    ('Swing Trading',           'swing-trading',           'Swing-trading posts'),
    ('Beginners Guide',         'beginners',               'Beginner guides')
ON CONFLICT (slug) DO NOTHING;
