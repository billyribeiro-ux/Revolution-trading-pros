-- ═══════════════════════════════════════════════════════════════════════════════════
-- MEMBER MANAGEMENT TABLES - Revolution Trading Pros
-- Apple Principal Engineer ICT 11+ Grade - January 17, 2026
--
-- This migration adds tables for enterprise member management:
-- 1. user_status - Ban/suspend status tracking
-- 2. user_activity_log - Activity log for audit trail
-- 3. member_notes - Admin notes on members
-- 4. member_segments - Member segmentation
-- 5. member_tags - Tagging system
-- 6. member_filters - Saved filter presets
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. USER STATUS TABLE (Ban/Suspend tracking)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS user_status (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'banned', 'suspended', 'restricted')),

    -- Ban/Suspend details
    banned_until TIMESTAMPTZ,
    ban_reason TEXT,

    -- Restrictions (for partial access)
    restrictions JSONB DEFAULT '[]'::jsonb,

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_status_user_id ON user_status(user_id);
CREATE INDEX IF NOT EXISTS idx_user_status_status ON user_status(status);
CREATE INDEX IF NOT EXISTS idx_user_status_banned_until ON user_status(banned_until) WHERE banned_until IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. USER ACTIVITY LOG TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS user_activity_log (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Activity details
    action VARCHAR(100) NOT NULL,
    description TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,

    -- Request info
    ip_address VARCHAR(45),
    user_agent TEXT,

    -- Timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_activity_user_id ON user_activity_log(user_id);
CREATE INDEX IF NOT EXISTS idx_user_activity_action ON user_activity_log(action);
CREATE INDEX IF NOT EXISTS idx_user_activity_created_at ON user_activity_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_user_activity_user_created ON user_activity_log(user_id, created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. MEMBER NOTES TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_notes (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Note content
    content TEXT NOT NULL,

    -- Author
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_notes_user_id ON member_notes(user_id);
CREATE INDEX IF NOT EXISTS idx_member_notes_created_at ON member_notes(user_id, created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. MEMBER SEGMENTS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_segments (
    id BIGSERIAL PRIMARY KEY,

    -- Identity
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,

    -- Segment rules (JSON for flexible filtering)
    rules JSONB DEFAULT '{}'::jsonb,

    -- Computed member count (updated periodically)
    member_count INTEGER DEFAULT 0,

    -- Status
    is_active BOOLEAN DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_segments_slug ON member_segments(slug);
CREATE INDEX IF NOT EXISTS idx_member_segments_is_active ON member_segments(is_active);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. MEMBER TAGS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_tags (
    id BIGSERIAL PRIMARY KEY,

    -- Identity
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,

    -- Appearance
    color VARCHAR(20) DEFAULT '#6B7280',
    description TEXT,

    -- Computed member count
    member_count INTEGER DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_tags_slug ON member_tags(slug);
CREATE INDEX IF NOT EXISTS idx_member_tags_name ON member_tags(name);

-- User-Tag relationship
CREATE TABLE IF NOT EXISTS user_member_tags (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES member_tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_user_member_tags_user ON user_member_tags(user_id);
CREATE INDEX IF NOT EXISTS idx_user_member_tags_tag ON user_member_tags(tag_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. MEMBER FILTERS TABLE (Saved filter presets)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_filters (
    id BIGSERIAL PRIMARY KEY,

    -- Identity
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Filter configuration
    filters JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Settings
    is_default BOOLEAN DEFAULT false,
    is_public BOOLEAN DEFAULT true,

    -- Creator
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_filters_is_default ON member_filters(is_default);
CREATE INDEX IF NOT EXISTS idx_member_filters_created_by ON member_filters(created_by);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. MEMBER EMAILS TABLE (Email history)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_emails (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Email details
    subject VARCHAR(500) NOT NULL,
    body TEXT,
    template_id VARCHAR(100),
    campaign_type VARCHAR(50),

    -- Status
    status VARCHAR(50) DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'delivered', 'bounced', 'failed')),

    -- Timestamps
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_member_emails_user_id ON member_emails(user_id);
CREATE INDEX IF NOT EXISTS idx_member_emails_status ON member_emails(status);
CREATE INDEX IF NOT EXISTS idx_member_emails_created_at ON member_emails(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. TRIGGERS FOR UPDATED_AT
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Generic updated_at trigger function (if not exists)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to tables
DROP TRIGGER IF EXISTS trigger_user_status_updated_at ON user_status;
CREATE TRIGGER trigger_user_status_updated_at
    BEFORE UPDATE ON user_status FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_member_segments_updated_at ON member_segments;
CREATE TRIGGER trigger_member_segments_updated_at
    BEFORE UPDATE ON member_segments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_member_tags_updated_at ON member_tags;
CREATE TRIGGER trigger_member_tags_updated_at
    BEFORE UPDATE ON member_tags FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_member_filters_updated_at ON member_filters;
CREATE TRIGGER trigger_member_filters_updated_at
    BEFORE UPDATE ON member_filters FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 9. SEED DEFAULT TAGS
-- ═══════════════════════════════════════════════════════════════════════════════════
INSERT INTO member_tags (name, slug, color, description) VALUES
    ('VIP', 'vip', '#F59E0B', 'VIP customers with priority support'),
    ('Whale', 'whale', '#10B981', 'High-value customers'),
    ('At Risk', 'at-risk', '#EF4444', 'Members at risk of churning'),
    ('New', 'new', '#3B82F6', 'Recently joined members'),
    ('Active Trader', 'active-trader', '#8B5CF6', 'Frequently active members'),
    ('Mentor', 'mentor', '#EC4899', 'Community mentors'),
    ('Beta Tester', 'beta-tester', '#06B6D4', 'Beta program participants')
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 10. SEED DEFAULT SEGMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════
INSERT INTO member_segments (name, slug, description, rules, is_active) VALUES
    ('Active Subscribers', 'active-subscribers', 'Members with active subscriptions', '{"status": "active"}', true),
    ('Trial Users', 'trial-users', 'Members on trial plans', '{"status": "trial"}', true),
    ('Churned Members', 'churned-members', 'Members who cancelled or let subscriptions expire', '{"status": ["cancelled", "expired"]}', true),
    ('High Spenders', 'high-spenders', 'Members who spent over $500', '{"total_spent_gte": 500}', true),
    ('New This Month', 'new-this-month', 'Members who joined this month', '{"created_within_days": 30}', true),
    ('Inactive 30+ Days', 'inactive-30-days', 'Members inactive for 30+ days', '{"inactive_days_gte": 30}', true)
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 11. COMMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════
COMMENT ON TABLE user_status IS 'Tracks user ban/suspend status and restrictions';
COMMENT ON TABLE user_activity_log IS 'Audit trail of user actions for compliance and debugging';
COMMENT ON TABLE member_notes IS 'Admin notes on member accounts';
COMMENT ON TABLE member_segments IS 'Member segments for targeted marketing and analysis';
COMMENT ON TABLE member_tags IS 'Tags for categorizing members';
COMMENT ON TABLE user_member_tags IS 'Many-to-many relationship between users and tags';
COMMENT ON TABLE member_filters IS 'Saved filter presets for member searches';
COMMENT ON TABLE member_emails IS 'History of emails sent to members';

-- Done!
