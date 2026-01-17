-- ═══════════════════════════════════════════════════════════════════════════
-- Advanced CMS Features Migration - Apple ICT 11+ Principal Engineer
-- Revolution Trading Pros - World's Most Advanced Headless CMS
--
-- Features Added:
-- 1. Content Versioning with full history
-- 2. Comprehensive Audit Logging
-- 3. Content Workflow System with approvals
-- 4. Content Webhooks for real-time notifications
-- 5. Publish Scheduling for future dates
-- 6. i18n/Localization support
-- 7. Draft Preview system
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- 1. CONTENT VERSIONING SYSTEM
-- Track every change with full rollback capability
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS content_versions (
    id BIGSERIAL PRIMARY KEY,

    -- Content reference (polymorphic)
    content_type VARCHAR(50) NOT NULL,  -- 'post', 'product', 'course', etc.
    content_id BIGINT NOT NULL,

    -- Version info
    version_number INTEGER NOT NULL DEFAULT 1,
    is_current BOOLEAN NOT NULL DEFAULT false,

    -- Snapshot of content at this version
    data JSONB NOT NULL,

    -- Change metadata
    change_summary VARCHAR(500),
    changed_fields TEXT[],  -- List of fields that changed

    -- Author info
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure unique version numbers per content
    UNIQUE(content_type, content_id, version_number)
);

-- Indexes for fast lookups
CREATE INDEX idx_content_versions_lookup ON content_versions(content_type, content_id);
CREATE INDEX idx_content_versions_current ON content_versions(content_type, content_id) WHERE is_current = true;
CREATE INDEX idx_content_versions_created_by ON content_versions(created_by);
CREATE INDEX idx_content_versions_created_at ON content_versions(created_at DESC);

-- ─────────────────────────────────────────────────────────────────────────────
-- 2. COMPREHENSIVE AUDIT LOGGING
-- Track all actions for compliance and debugging
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS audit_logs (
    id BIGSERIAL PRIMARY KEY,

    -- Action info
    action VARCHAR(50) NOT NULL,  -- 'create', 'update', 'delete', 'publish', 'login', etc.
    entity_type VARCHAR(50) NOT NULL,  -- 'post', 'user', 'media', etc.
    entity_id BIGINT,

    -- Actor info
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    user_email VARCHAR(255),  -- Denormalized for historical record
    user_role VARCHAR(50),

    -- Request context
    ip_address INET,
    user_agent TEXT,
    request_method VARCHAR(10),
    request_path VARCHAR(500),

    -- Change details
    old_values JSONB,  -- Previous state
    new_values JSONB,  -- New state
    metadata JSONB,    -- Additional context

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'success',  -- 'success', 'failed', 'pending'
    error_message TEXT,

    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for audit queries
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_ip ON audit_logs(ip_address);

-- Partial index for failed actions (security monitoring)
CREATE INDEX idx_audit_logs_failed ON audit_logs(created_at DESC) WHERE status = 'failed';

-- ─────────────────────────────────────────────────────────────────────────────
-- 3. CONTENT WORKFLOW SYSTEM
-- Multi-stage approval process for enterprise content
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS workflow_definitions (
    id BIGSERIAL PRIMARY KEY,

    -- Definition info
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,

    -- Workflow stages (ordered list)
    stages JSONB NOT NULL DEFAULT '[]'::jsonb,
    -- Example: [{"name": "draft", "label": "Draft"}, {"name": "review", "label": "Under Review"}, {"name": "approved", "label": "Approved"}]

    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT true,
    applies_to TEXT[] DEFAULT ARRAY['post'],  -- Content types this workflow applies to

    -- Metadata
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default workflow
INSERT INTO workflow_definitions (name, description, stages, applies_to) VALUES (
    'default_content_workflow',
    'Standard content review workflow',
    '[
        {"name": "draft", "label": "Draft", "order": 1},
        {"name": "pending_review", "label": "Pending Review", "order": 2},
        {"name": "in_review", "label": "In Review", "order": 3},
        {"name": "approved", "label": "Approved", "order": 4},
        {"name": "published", "label": "Published", "order": 5}
    ]'::jsonb,
    ARRAY['post', 'product', 'course']
) ON CONFLICT (name) DO NOTHING;

-- Track content through workflow
CREATE TABLE IF NOT EXISTS content_workflow_status (
    id BIGSERIAL PRIMARY KEY,

    -- Content reference
    content_type VARCHAR(50) NOT NULL,
    content_id BIGINT NOT NULL,

    -- Workflow reference
    workflow_id BIGINT REFERENCES workflow_definitions(id) ON DELETE SET NULL,

    -- Current status
    current_stage VARCHAR(50) NOT NULL DEFAULT 'draft',

    -- Assignment
    assigned_to BIGINT REFERENCES users(id) ON DELETE SET NULL,
    assigned_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    assigned_at TIMESTAMPTZ,

    -- Due date
    due_date TIMESTAMPTZ,
    priority VARCHAR(20) DEFAULT 'normal',  -- 'low', 'normal', 'high', 'urgent'

    -- Notes
    notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint
    UNIQUE(content_type, content_id)
);

-- Workflow transitions history
CREATE TABLE IF NOT EXISTS workflow_transitions (
    id BIGSERIAL PRIMARY KEY,

    -- Reference
    workflow_status_id BIGINT REFERENCES content_workflow_status(id) ON DELETE CASCADE,

    -- Transition info
    from_stage VARCHAR(50),
    to_stage VARCHAR(50) NOT NULL,

    -- Actor
    transitioned_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Reason/Notes
    comment TEXT,

    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_workflow_status_content ON content_workflow_status(content_type, content_id);
CREATE INDEX idx_workflow_status_stage ON content_workflow_status(current_stage);
CREATE INDEX idx_workflow_status_assigned ON content_workflow_status(assigned_to);
CREATE INDEX idx_workflow_transitions_status ON workflow_transitions(workflow_status_id);

-- ─────────────────────────────────────────────────────────────────────────────
-- 4. CONTENT WEBHOOKS
-- Real-time notifications for content changes
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS webhooks (
    id BIGSERIAL PRIMARY KEY,

    -- Webhook info
    name VARCHAR(100) NOT NULL,
    description TEXT,
    url VARCHAR(500) NOT NULL,

    -- Security
    secret VARCHAR(255),  -- For HMAC signature

    -- Events to trigger on
    events TEXT[] NOT NULL DEFAULT ARRAY['content.created', 'content.updated', 'content.published'],
    -- Examples: content.created, content.updated, content.deleted, content.published, user.created, media.uploaded

    -- Filters (optional)
    content_types TEXT[],  -- Filter by content type

    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT true,
    retry_count INTEGER NOT NULL DEFAULT 3,
    timeout_seconds INTEGER NOT NULL DEFAULT 30,

    -- Headers to send
    headers JSONB DEFAULT '{}'::jsonb,

    -- Owner
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Webhook delivery log
CREATE TABLE IF NOT EXISTS webhook_deliveries (
    id BIGSERIAL PRIMARY KEY,

    -- Reference
    webhook_id BIGINT REFERENCES webhooks(id) ON DELETE CASCADE,

    -- Event info
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,

    -- Delivery status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- 'pending', 'success', 'failed'
    attempt_count INTEGER NOT NULL DEFAULT 0,

    -- Response
    response_status INTEGER,
    response_body TEXT,
    response_time_ms INTEGER,

    -- Error info
    error_message TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    next_retry_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_webhooks_active ON webhooks(is_active) WHERE is_active = true;
CREATE INDEX idx_webhook_deliveries_webhook ON webhook_deliveries(webhook_id);
CREATE INDEX idx_webhook_deliveries_status ON webhook_deliveries(status);
CREATE INDEX idx_webhook_deliveries_retry ON webhook_deliveries(next_retry_at) WHERE status = 'pending';

-- ─────────────────────────────────────────────────────────────────────────────
-- 5. PUBLISH SCHEDULING
-- Schedule content for future publication
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS scheduled_content (
    id BIGSERIAL PRIMARY KEY,

    -- Content reference
    content_type VARCHAR(50) NOT NULL,
    content_id BIGINT NOT NULL,

    -- Schedule info
    scheduled_action VARCHAR(50) NOT NULL,  -- 'publish', 'unpublish', 'update'
    scheduled_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) DEFAULT 'UTC',

    -- Payload (for updates)
    payload JSONB,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'scheduled',  -- 'scheduled', 'executed', 'cancelled', 'failed'
    executed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Owner
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint (one schedule per content/action)
    UNIQUE(content_type, content_id, scheduled_action, status)
        -- Only applies to 'scheduled' status
);

-- Index for scheduler job
CREATE INDEX idx_scheduled_content_pending ON scheduled_content(scheduled_at)
    WHERE status = 'scheduled';
CREATE INDEX idx_scheduled_content_lookup ON scheduled_content(content_type, content_id);

-- ─────────────────────────────────────────────────────────────────────────────
-- 6. i18n/LOCALIZATION SUPPORT
-- Multi-language content management
-- ─────────────────────────────────────────────────────────────────────────────

-- Supported locales
CREATE TABLE IF NOT EXISTS locales (
    id BIGSERIAL PRIMARY KEY,

    -- Locale info
    code VARCHAR(10) NOT NULL UNIQUE,  -- 'en', 'en-US', 'es', 'fr', etc.
    name VARCHAR(100) NOT NULL,
    native_name VARCHAR(100),

    -- Settings
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    rtl BOOLEAN NOT NULL DEFAULT false,  -- Right-to-left

    -- Fallback
    fallback_locale VARCHAR(10) REFERENCES locales(code) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default locales
INSERT INTO locales (code, name, native_name, is_default) VALUES
    ('en', 'English', 'English', true),
    ('es', 'Spanish', 'Español', false),
    ('fr', 'French', 'Français', false),
    ('de', 'German', 'Deutsch', false),
    ('pt', 'Portuguese', 'Português', false),
    ('ja', 'Japanese', '日本語', false),
    ('zh', 'Chinese', '中文', false)
ON CONFLICT (code) DO NOTHING;

-- Content translations
CREATE TABLE IF NOT EXISTS content_translations (
    id BIGSERIAL PRIMARY KEY,

    -- Content reference
    content_type VARCHAR(50) NOT NULL,
    content_id BIGINT NOT NULL,

    -- Locale
    locale VARCHAR(10) NOT NULL REFERENCES locales(code) ON DELETE CASCADE,

    -- Translated fields (flexible JSON)
    translations JSONB NOT NULL,
    -- Example: {"title": "Translated Title", "excerpt": "Translated excerpt", "content_blocks": [...]}

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'draft',  -- 'draft', 'published', 'needs_review'

    -- Translator info
    translated_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    reviewed_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Quality
    machine_translated BOOLEAN NOT NULL DEFAULT false,
    quality_score INTEGER,  -- 0-100

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint
    UNIQUE(content_type, content_id, locale)
);

-- Indexes
CREATE INDEX idx_translations_content ON content_translations(content_type, content_id);
CREATE INDEX idx_translations_locale ON content_translations(locale);
CREATE INDEX idx_translations_status ON content_translations(status);

-- ─────────────────────────────────────────────────────────────────────────────
-- 7. DRAFT PREVIEW SYSTEM
-- Generate secure preview links for unpublished content
-- ─────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS preview_tokens (
    id BIGSERIAL PRIMARY KEY,

    -- Content reference
    content_type VARCHAR(50) NOT NULL,
    content_id BIGINT NOT NULL,

    -- Token (UUID for security)
    token UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,

    -- Settings
    expires_at TIMESTAMPTZ NOT NULL,
    max_views INTEGER,  -- NULL = unlimited
    view_count INTEGER NOT NULL DEFAULT 0,

    -- Access control
    password_hash VARCHAR(255),  -- Optional password protection
    allowed_emails TEXT[],  -- Optional email whitelist

    -- Owner
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_preview_tokens_content ON preview_tokens(content_type, content_id);
CREATE INDEX idx_preview_tokens_token ON preview_tokens(token);
CREATE INDEX idx_preview_tokens_expires ON preview_tokens(expires_at);

-- ─────────────────────────────────────────────────────────────────────────────
-- 8. ADDITIONAL ENHANCEMENTS
-- ─────────────────────────────────────────────────────────────────────────────

-- Add scheduling columns to posts table
ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS scheduled_publish_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS scheduled_unpublish_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS locale VARCHAR(10) DEFAULT 'en',
    ADD COLUMN IF NOT EXISTS is_primary_locale BOOLEAN DEFAULT true,
    ADD COLUMN IF NOT EXISTS parent_post_id BIGINT REFERENCES posts(id) ON DELETE SET NULL;

-- Add scheduling columns to products table (if exists)
DO $$
BEGIN
    ALTER TABLE products
        ADD COLUMN IF NOT EXISTS scheduled_publish_at TIMESTAMPTZ,
        ADD COLUMN IF NOT EXISTS scheduled_unpublish_at TIMESTAMPTZ,
        ADD COLUMN IF NOT EXISTS locale VARCHAR(10) DEFAULT 'en';
EXCEPTION
    WHEN undefined_table THEN NULL;
END $$;

-- Create index for scheduled publishing
CREATE INDEX IF NOT EXISTS idx_posts_scheduled_publish ON posts(scheduled_publish_at)
    WHERE scheduled_publish_at IS NOT NULL AND status = 'scheduled';

-- ─────────────────────────────────────────────────────────────────────────────
-- 9. HELPER FUNCTIONS
-- ─────────────────────────────────────────────────────────────────────────────

-- Function to create a content version
CREATE OR REPLACE FUNCTION create_content_version(
    p_content_type VARCHAR,
    p_content_id BIGINT,
    p_data JSONB,
    p_created_by BIGINT,
    p_change_summary VARCHAR DEFAULT NULL,
    p_changed_fields TEXT[] DEFAULT NULL
) RETURNS BIGINT AS $$
DECLARE
    v_version_number INTEGER;
    v_new_id BIGINT;
BEGIN
    -- Get next version number
    SELECT COALESCE(MAX(version_number), 0) + 1 INTO v_version_number
    FROM content_versions
    WHERE content_type = p_content_type AND content_id = p_content_id;

    -- Mark previous version as not current
    UPDATE content_versions
    SET is_current = false
    WHERE content_type = p_content_type AND content_id = p_content_id;

    -- Insert new version
    INSERT INTO content_versions (
        content_type, content_id, version_number, is_current,
        data, change_summary, changed_fields, created_by
    ) VALUES (
        p_content_type, p_content_id, v_version_number, true,
        p_data, p_change_summary, p_changed_fields, p_created_by
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$ LANGUAGE plpgsql;

-- Function to log an audit event
CREATE OR REPLACE FUNCTION log_audit_event(
    p_action VARCHAR,
    p_entity_type VARCHAR,
    p_entity_id BIGINT,
    p_user_id BIGINT,
    p_user_email VARCHAR,
    p_old_values JSONB DEFAULT NULL,
    p_new_values JSONB DEFAULT NULL,
    p_metadata JSONB DEFAULT NULL,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL
) RETURNS BIGINT AS $$
DECLARE
    v_new_id BIGINT;
BEGIN
    INSERT INTO audit_logs (
        action, entity_type, entity_id, user_id, user_email,
        old_values, new_values, metadata, ip_address, user_agent
    ) VALUES (
        p_action, p_entity_type, p_entity_id, p_user_id, p_user_email,
        p_old_values, p_new_values, p_metadata, p_ip_address, p_user_agent
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$ LANGUAGE plpgsql;

-- ─────────────────────────────────────────────────────────────────────────────
-- 10. COMMENTS FOR DOCUMENTATION
-- ─────────────────────────────────────────────────────────────────────────────
COMMENT ON TABLE content_versions IS 'Stores all versions of content for history tracking and rollback';
COMMENT ON TABLE audit_logs IS 'Comprehensive audit trail for all system actions';
COMMENT ON TABLE workflow_definitions IS 'Configurable workflow stages for content approval';
COMMENT ON TABLE content_workflow_status IS 'Current workflow status for each piece of content';
COMMENT ON TABLE webhooks IS 'Webhook configurations for external integrations';
COMMENT ON TABLE scheduled_content IS 'Content scheduled for future publication';
COMMENT ON TABLE locales IS 'Supported languages/locales';
COMMENT ON TABLE content_translations IS 'Translated content for i18n support';
COMMENT ON TABLE preview_tokens IS 'Secure preview links for unpublished content';
