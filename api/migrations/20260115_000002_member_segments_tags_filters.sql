-- Member Segments, Tags, and Filters - CRM System
-- Apple ICT 7 Grade - January 2026
-- Revolution Trading Pros

-- ===============================================================================
-- MEMBER SEGMENTS - For targeted marketing and member grouping
-- ===============================================================================

CREATE TABLE IF NOT EXISTS member_segments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    rules JSONB DEFAULT '{}',
    member_count INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_segments_slug ON member_segments(slug);
CREATE INDEX IF NOT EXISTS idx_member_segments_active ON member_segments(is_active);

-- ===============================================================================
-- MEMBER TAGS - Tagging system for member categorization
-- ===============================================================================

CREATE TABLE IF NOT EXISTS member_tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    color VARCHAR(50),
    description TEXT,
    member_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_tags_slug ON member_tags(slug);
CREATE INDEX IF NOT EXISTS idx_member_tags_name ON member_tags(name);

-- User-Tag Junction Table
CREATE TABLE IF NOT EXISTS user_member_tags (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES member_tags(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_user_member_tags_user ON user_member_tags(user_id);
CREATE INDEX IF NOT EXISTS idx_user_member_tags_tag ON user_member_tags(tag_id);

-- ===============================================================================
-- MEMBER FILTERS - Saved filter presets for quick access
-- ===============================================================================

CREATE TABLE IF NOT EXISTS member_filters (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    filters JSONB NOT NULL DEFAULT '{}',
    is_default BOOLEAN DEFAULT false,
    is_public BOOLEAN DEFAULT true,
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_filters_default ON member_filters(is_default);
CREATE INDEX IF NOT EXISTS idx_member_filters_public ON member_filters(is_public);
CREATE INDEX IF NOT EXISTS idx_member_filters_created_by ON member_filters(created_by);

-- ===============================================================================
-- USER-SEGMENT JUNCTION TABLE (for manual segment assignment)
-- ===============================================================================

CREATE TABLE IF NOT EXISTS user_member_segments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    segment_id BIGINT NOT NULL REFERENCES member_segments(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, segment_id)
);

CREATE INDEX IF NOT EXISTS idx_user_member_segments_user ON user_member_segments(user_id);
CREATE INDEX IF NOT EXISTS idx_user_member_segments_segment ON user_member_segments(segment_id);

-- ===============================================================================
-- SEED DEFAULT DATA
-- ===============================================================================

-- Default Segments
INSERT INTO member_segments (name, slug, description, rules, is_active) VALUES
    ('All Members', 'all-members', 'All registered members', '{}', true),
    ('Active Subscribers', 'active-subscribers', 'Members with active subscriptions', '{"subscription_status": "active"}', true),
    ('Trial Members', 'trial-members', 'Members on trial subscriptions', '{"subscription_status": "trial"}', true),
    ('Premium Members', 'premium-members', 'Members with premium plans', '{"plan_type": "premium"}', true),
    ('New This Month', 'new-this-month', 'Members who joined this month', '{"date_range": "this_month"}', true),
    ('Churned', 'churned', 'Members who cancelled their subscription', '{"subscription_status": "cancelled"}', true)
ON CONFLICT (slug) DO NOTHING;

-- Default Tags
INSERT INTO member_tags (name, slug, color, description) VALUES
    ('VIP', 'vip', '#FFD700', 'VIP members with special access'),
    ('Early Adopter', 'early-adopter', '#4CAF50', 'Early adopters of the platform'),
    ('Beta Tester', 'beta-tester', '#2196F3', 'Beta feature testers'),
    ('Affiliate', 'affiliate', '#9C27B0', 'Affiliate program members'),
    ('Influencer', 'influencer', '#FF5722', 'Social media influencers'),
    ('Enterprise', 'enterprise', '#607D8B', 'Enterprise/business accounts'),
    ('High Value', 'high-value', '#E91E63', 'High lifetime value customers'),
    ('At Risk', 'at-risk', '#F44336', 'Members at risk of churning'),
    ('Engaged', 'engaged', '#00BCD4', 'Highly engaged members'),
    ('Inactive', 'inactive', '#9E9E9E', 'Inactive members')
ON CONFLICT (slug) DO NOTHING;

-- Default Filters
INSERT INTO member_filters (name, description, filters, is_default, is_public) VALUES
    ('All Members', 'Show all members without any filters', '{}', true, true),
    ('Active This Week', 'Members who logged in this week', '{"last_login": "this_week"}', false, true),
    ('Never Purchased', 'Members who have not made a purchase', '{"has_purchase": false}', false, true),
    ('High Spenders', 'Members with total purchases over $500', '{"min_total_spent": 500}', false, true),
    ('Expiring Soon', 'Members whose subscription expires within 7 days', '{"subscription_expires_within": 7}', false, true),
    ('Needs Attention', 'Members who may need support follow-up', '{"needs_attention": true}', false, true)
ON CONFLICT DO NOTHING;

-- Update segment member counts (will be recalculated on first API call)
UPDATE member_segments SET member_count = 0;
