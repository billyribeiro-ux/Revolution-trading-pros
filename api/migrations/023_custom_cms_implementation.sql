-- ═══════════════════════════════════════════════════════════════════════════════════════
-- CUSTOM CMS IMPLEMENTATION - Apple ICT 7+ Principal Engineer Grade
-- Revolution Trading Pros - January 2026
--
-- This migration creates the comprehensive custom CMS.
-- Features:
-- - Unified content storage with all content types
-- - Block-based page builder support
-- - Digital Asset Management (DAM)
-- - Role-based permissions
-- - Navigation management
-- - SEO optimization
--
-- Design Principles:
-- 1. UUIDs for all primary keys (gen_random_uuid())
-- 2. Timestamps with timezone (TIMESTAMPTZ)
-- 3. Soft deletes for content (deleted_at)
-- 4. Audit columns on every table
-- 5. Optimistic locking (version column)
-- 6. Strict constraints with documented patterns
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 1. CONTENT TYPE ENUM
-- All supported content types in the CMS
-- ─────────────────────────────────────────────────────────────────────────────────────────
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_content_type') THEN
        CREATE TYPE cms_content_type AS ENUM (
            'page',
            'blog_post',
            'alert_service',
            'trading_room',
            'indicator',
            'course',
            'lesson',
            'testimonial',
            'faq',
            'author',
            'topic_cluster',
            'weekly_watchlist',
            'resource',
            'navigation_menu',
            'site_settings',
            'redirect'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_content_type IS 'All supported content types in the custom CMS';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 2. CMS USER ROLES ENUM
-- Role-based access control for CMS users
-- ─────────────────────────────────────────────────────────────────────────────────────────
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_user_role') THEN
        CREATE TYPE cms_user_role AS ENUM (
            'super_admin',
            'marketing_manager',
            'content_editor',
            'weekly_editor',
            'developer',
            'viewer'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_user_role IS 'CMS user roles with specific permissions';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 3. CONTENT STATUS ENUM
-- Workflow status for content items
-- ─────────────────────────────────────────────────────────────────────────────────────────
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_content_status') THEN
        CREATE TYPE cms_content_status AS ENUM (
            'draft',
            'in_review',
            'approved',
            'scheduled',
            'published',
            'archived'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_content_status IS 'Content workflow status';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 4. BLOCK TYPE ENUM
-- All supported block types for the page builder
-- ─────────────────────────────────────────────────────────────────────────────────────────
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_block_type') THEN
        CREATE TYPE cms_block_type AS ENUM (
            'hero-slider',
            'trading-rooms-grid',
            'alert-services-grid',
            'testimonials-masonry',
            'features-grid',
            'rich-text',
            'image',
            'video-embed',
            'spacer',
            'email-capture',
            'blog-feed',
            'indicators-showcase',
            'courses-grid',
            'faq-accordion',
            'pricing-table',
            'cta-banner',
            'stats-counter',
            'two-column-layout'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_block_type IS 'All supported block types for the page builder';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 5. CMS_USERS TABLE
-- CMS user profiles with roles and permissions
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Link to main users table
    user_id BIGINT UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    -- Profile information
    display_name VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(500),
    bio TEXT,

    -- Role and permissions
    role cms_user_role NOT NULL DEFAULT 'viewer',
    permissions JSONB DEFAULT '[]'::jsonb,

    -- Content type restrictions (for weekly_editor role)
    allowed_content_types cms_content_type[] DEFAULT NULL,

    -- Settings
    preferences JSONB DEFAULT '{}'::jsonb,
    notification_settings JSONB DEFAULT '{"email": true, "in_app": true}'::jsonb,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_login_at TIMESTAMPTZ,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_cms_users_user_id ON cms_users(user_id);
CREATE INDEX idx_cms_users_role ON cms_users(role);
CREATE INDEX idx_cms_users_active ON cms_users(is_active) WHERE is_active = true;

COMMENT ON TABLE cms_users IS 'CMS user profiles with roles and permissions';
COMMENT ON COLUMN cms_users.user_id IS 'Foreign key to main users table';
COMMENT ON COLUMN cms_users.role IS 'User role determining permissions';
COMMENT ON COLUMN cms_users.allowed_content_types IS 'Content types this user can edit (for restricted roles)';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 6. CMS_ASSET_FOLDERS TABLE
-- Hierarchical folder structure for asset organization
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_asset_folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Folder hierarchy
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    parent_id UUID REFERENCES cms_asset_folders(id) ON DELETE CASCADE,
    path TEXT NOT NULL DEFAULT '/',
    depth INTEGER NOT NULL DEFAULT 0,

    -- Metadata
    description TEXT,
    color VARCHAR(7),
    icon VARCHAR(50),

    -- Permissions
    is_public BOOLEAN NOT NULL DEFAULT false,
    allowed_roles cms_user_role[] DEFAULT NULL,

    -- Sorting
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Constraints
    CONSTRAINT valid_folder_slug CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT unique_folder_in_parent UNIQUE (parent_id, slug)
);

CREATE INDEX idx_cms_asset_folders_parent ON cms_asset_folders(parent_id);
CREATE INDEX idx_cms_asset_folders_path ON cms_asset_folders(path);
CREATE INDEX idx_cms_asset_folders_slug ON cms_asset_folders(slug);

COMMENT ON TABLE cms_asset_folders IS 'Hierarchical folder structure for organizing digital assets';
COMMENT ON COLUMN cms_asset_folders.path IS 'Full path from root (e.g., /images/blog/2026)';
COMMENT ON COLUMN cms_asset_folders.depth IS 'Nesting level (0 = root level)';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 7. CMS_ASSETS TABLE
-- Digital Asset Management - images, videos, documents
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Organization
    folder_id UUID REFERENCES cms_asset_folders(id) ON DELETE SET NULL,

    -- File information
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,
    file_extension VARCHAR(20) NOT NULL,

    -- Storage
    storage_provider VARCHAR(50) NOT NULL DEFAULT 'r2',
    storage_key VARCHAR(500) NOT NULL UNIQUE,
    cdn_url VARCHAR(500) NOT NULL,

    -- Image-specific metadata
    width INTEGER,
    height INTEGER,
    aspect_ratio DECIMAL(10, 6),
    blurhash VARCHAR(100),
    dominant_color VARCHAR(7),

    -- Video-specific metadata
    duration_seconds INTEGER,
    video_codec VARCHAR(50),
    audio_codec VARCHAR(50),
    bunny_video_id VARCHAR(100),
    bunny_library_id INTEGER,
    thumbnail_url VARCHAR(500),

    -- Variants (responsive images, transcodes)
    variants JSONB DEFAULT '[]'::jsonb,

    -- Metadata
    title VARCHAR(255),
    alt_text VARCHAR(500),
    caption TEXT,
    description TEXT,
    credits VARCHAR(500),

    -- SEO
    seo_title VARCHAR(255),
    seo_description VARCHAR(500),

    -- Taxonomy
    tags TEXT[] DEFAULT ARRAY[]::TEXT[],

    -- Usage tracking
    usage_count INTEGER NOT NULL DEFAULT 0,
    last_used_at TIMESTAMPTZ,

    -- Soft delete
    deleted_at TIMESTAMPTZ,

    -- Optimistic locking
    version INTEGER NOT NULL DEFAULT 1,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL
);

CREATE INDEX idx_cms_assets_folder ON cms_assets(folder_id);
CREATE INDEX idx_cms_assets_mime_type ON cms_assets(mime_type);
CREATE INDEX idx_cms_assets_storage_key ON cms_assets(storage_key);
CREATE INDEX idx_cms_assets_tags ON cms_assets USING GIN(tags);
CREATE INDEX idx_cms_assets_created_at ON cms_assets(created_at DESC);
CREATE INDEX idx_cms_assets_not_deleted ON cms_assets(id) WHERE deleted_at IS NULL;
CREATE INDEX idx_cms_assets_usage ON cms_assets(usage_count DESC, last_used_at DESC);

COMMENT ON TABLE cms_assets IS 'Digital Asset Management for all media files';
COMMENT ON COLUMN cms_assets.storage_key IS 'Unique key in cloud storage (R2/S3)';
COMMENT ON COLUMN cms_assets.variants IS 'Array of responsive image sizes or video transcodes';
COMMENT ON COLUMN cms_assets.blurhash IS 'BlurHash for progressive image loading';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 8. CMS_CONTENT TABLE
-- Unified content storage for all content types
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content identification
    content_type cms_content_type NOT NULL,
    slug VARCHAR(500) NOT NULL,

    -- Localization
    locale VARCHAR(10) NOT NULL DEFAULT 'en',
    is_primary_locale BOOLEAN NOT NULL DEFAULT true,
    parent_content_id UUID REFERENCES cms_content(id) ON DELETE SET NULL,

    -- Core fields
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    excerpt TEXT,
    content TEXT,

    -- Block-based content (page builder)
    content_blocks JSONB DEFAULT '[]'::jsonb,

    -- Media
    featured_image_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,
    og_image_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,
    gallery_ids UUID[] DEFAULT ARRAY[]::UUID[],

    -- SEO fields
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords TEXT[],
    canonical_url VARCHAR(500),
    robots_directives VARCHAR(100) DEFAULT 'index, follow',

    -- Structured data (JSON-LD)
    structured_data JSONB,

    -- Author information
    author_id UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    contributors UUID[] DEFAULT ARRAY[]::UUID[],

    -- Publishing
    status cms_content_status NOT NULL DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    scheduled_publish_at TIMESTAMPTZ,
    scheduled_unpublish_at TIMESTAMPTZ,

    -- Taxonomy
    primary_category_id UUID,
    categories UUID[] DEFAULT ARRAY[]::UUID[],

    -- Custom fields (content-type specific)
    custom_fields JSONB DEFAULT '{}'::jsonb,

    -- Template
    template VARCHAR(100),

    -- Soft delete
    deleted_at TIMESTAMPTZ,

    -- Optimistic locking
    version INTEGER NOT NULL DEFAULT 1,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Constraints
    CONSTRAINT valid_content_slug CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT unique_content_type_slug_locale UNIQUE (content_type, slug, locale)
);

CREATE INDEX idx_cms_content_type ON cms_content(content_type);
CREATE INDEX idx_cms_content_slug ON cms_content(slug);
CREATE INDEX idx_cms_content_locale ON cms_content(locale);
CREATE INDEX idx_cms_content_status ON cms_content(status);
CREATE INDEX idx_cms_content_author ON cms_content(author_id);
CREATE INDEX idx_cms_content_published ON cms_content(published_at DESC) WHERE status = 'published';
CREATE INDEX idx_cms_content_scheduled ON cms_content(scheduled_publish_at) WHERE scheduled_publish_at IS NOT NULL;
CREATE INDEX idx_cms_content_not_deleted ON cms_content(id) WHERE deleted_at IS NULL;
CREATE INDEX idx_cms_content_type_status ON cms_content(content_type, status) WHERE deleted_at IS NULL;
CREATE INDEX idx_cms_content_blocks ON cms_content USING GIN(content_blocks);
CREATE INDEX idx_cms_content_custom_fields ON cms_content USING GIN(custom_fields);

COMMENT ON TABLE cms_content IS 'Unified content storage for all CMS content types';
COMMENT ON COLUMN cms_content.content_blocks IS 'Array of content blocks for page builder';
COMMENT ON COLUMN cms_content.custom_fields IS 'Content-type specific fields stored as JSON';
COMMENT ON COLUMN cms_content.structured_data IS 'JSON-LD structured data for SEO';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 9. CMS_REVISIONS TABLE
-- Version history for content
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Version info
    revision_number INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT false,

    -- Snapshot
    data JSONB NOT NULL,

    -- Change metadata
    change_summary VARCHAR(500),
    changed_fields TEXT[],

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Unique revision per content
    CONSTRAINT unique_revision_number UNIQUE (content_id, revision_number)
);

CREATE INDEX idx_cms_revisions_content ON cms_revisions(content_id);
CREATE INDEX idx_cms_revisions_current ON cms_revisions(content_id) WHERE is_current = true;
CREATE INDEX idx_cms_revisions_created ON cms_revisions(created_at DESC);

COMMENT ON TABLE cms_revisions IS 'Version history for content with full rollback capability';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 10. CMS_TAGS TABLE
-- Taxonomy tags for content organization
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Tag identification
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,

    -- Hierarchy (optional parent for nested tags)
    parent_id UUID REFERENCES cms_tags(id) ON DELETE SET NULL,

    -- Metadata
    description TEXT,
    color VARCHAR(7),
    icon VARCHAR(50),

    -- SEO
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),

    -- Usage
    usage_count INTEGER NOT NULL DEFAULT 0,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Constraints
    CONSTRAINT valid_tag_slug CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$')
);

CREATE INDEX idx_cms_tags_slug ON cms_tags(slug);
CREATE INDEX idx_cms_tags_parent ON cms_tags(parent_id);
CREATE INDEX idx_cms_tags_usage ON cms_tags(usage_count DESC);

COMMENT ON TABLE cms_tags IS 'Taxonomy tags for content organization';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 11. CMS_CONTENT_TAGS TABLE
-- Junction table for content-tag relationships
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_content_tags (
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES cms_tags(id) ON DELETE CASCADE,

    -- Order within content
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    PRIMARY KEY (content_id, tag_id)
);

CREATE INDEX idx_cms_content_tags_tag ON cms_content_tags(tag_id);
CREATE INDEX idx_cms_content_tags_content ON cms_content_tags(content_id);

COMMENT ON TABLE cms_content_tags IS 'Junction table for content-tag relationships';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 12. CMS_CONTENT_RELATIONS TABLE
-- Many-to-many relationships between content items
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_content_relations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source content
    source_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Target content
    target_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Relation type
    relation_type VARCHAR(50) NOT NULL,

    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,

    -- Order
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Prevent self-referencing
    CONSTRAINT no_self_reference CHECK (source_id != target_id),
    CONSTRAINT unique_relation UNIQUE (source_id, target_id, relation_type)
);

CREATE INDEX idx_cms_relations_source ON cms_content_relations(source_id);
CREATE INDEX idx_cms_relations_target ON cms_content_relations(target_id);
CREATE INDEX idx_cms_relations_type ON cms_content_relations(relation_type);

COMMENT ON TABLE cms_content_relations IS 'Many-to-many relationships between content items';
COMMENT ON COLUMN cms_content_relations.relation_type IS 'Type of relation (e.g., "related_posts", "prerequisites", "see_also")';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 13. CMS_WORKFLOW_LOG TABLE
-- Audit trail for workflow transitions
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_workflow_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Transition details
    from_status cms_content_status,
    to_status cms_content_status NOT NULL,

    -- Actor
    transitioned_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Context
    comment TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,

    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cms_workflow_log_content ON cms_workflow_log(content_id);
CREATE INDEX idx_cms_workflow_log_actor ON cms_workflow_log(transitioned_by);
CREATE INDEX idx_cms_workflow_log_created ON cms_workflow_log(created_at DESC);

COMMENT ON TABLE cms_workflow_log IS 'Audit trail for all workflow status transitions';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 14. CMS_AUDIT_LOG TABLE (PARTITIONED)
-- General audit logging for all CMS actions
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_audit_log (
    id UUID DEFAULT gen_random_uuid(),

    -- Action details
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,

    -- Actor
    user_id UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    user_email VARCHAR(255),
    user_role cms_user_role,

    -- Request context
    ip_address INET,
    user_agent TEXT,
    request_method VARCHAR(10),
    request_path VARCHAR(500),

    -- Change data
    old_values JSONB,
    new_values JSONB,
    metadata JSONB,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'success',
    error_message TEXT,

    -- Timestamp (also partition key)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

-- Create monthly partitions for the next year
DO $$
DECLARE
    partition_date DATE := DATE_TRUNC('month', CURRENT_DATE);
    partition_name TEXT;
    start_date TEXT;
    end_date TEXT;
BEGIN
    FOR i IN 0..12 LOOP
        partition_name := 'cms_audit_log_' || TO_CHAR(partition_date, 'YYYY_MM');
        start_date := TO_CHAR(partition_date, 'YYYY-MM-DD');
        end_date := TO_CHAR(partition_date + INTERVAL '1 month', 'YYYY-MM-DD');

        EXECUTE format(
            'CREATE TABLE IF NOT EXISTS %I PARTITION OF cms_audit_log FOR VALUES FROM (%L) TO (%L)',
            partition_name,
            start_date,
            end_date
        );

        partition_date := partition_date + INTERVAL '1 month';
    END LOOP;
END$$;

CREATE INDEX idx_cms_audit_entity ON cms_audit_log(entity_type, entity_id);
CREATE INDEX idx_cms_audit_user ON cms_audit_log(user_id);
CREATE INDEX idx_cms_audit_action ON cms_audit_log(action);
CREATE INDEX idx_cms_audit_created ON cms_audit_log(created_at DESC);
CREATE INDEX idx_cms_audit_failed ON cms_audit_log(created_at DESC) WHERE status = 'failed';

COMMENT ON TABLE cms_audit_log IS 'Partitioned audit log for all CMS actions (compliance-ready)';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 15. CMS_COMMENTS TABLE
-- Editorial collaboration comments on content
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Thread hierarchy
    parent_id UUID REFERENCES cms_comments(id) ON DELETE CASCADE,
    thread_id UUID,

    -- Comment content
    body TEXT NOT NULL,

    -- Location in content (for inline comments)
    block_id VARCHAR(100),
    selection_start INTEGER,
    selection_end INTEGER,

    -- Status
    is_resolved BOOLEAN NOT NULL DEFAULT false,
    resolved_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    resolved_at TIMESTAMPTZ,

    -- Mentions
    mentioned_users UUID[] DEFAULT ARRAY[]::UUID[],

    -- Soft delete
    deleted_at TIMESTAMPTZ,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL
);

CREATE INDEX idx_cms_comments_content ON cms_comments(content_id);
CREATE INDEX idx_cms_comments_thread ON cms_comments(thread_id);
CREATE INDEX idx_cms_comments_parent ON cms_comments(parent_id);
CREATE INDEX idx_cms_comments_unresolved ON cms_comments(content_id) WHERE is_resolved = false AND deleted_at IS NULL;

COMMENT ON TABLE cms_comments IS 'Editorial collaboration comments on content items';
COMMENT ON COLUMN cms_comments.block_id IS 'Reference to specific block for inline comments';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 16. CMS_SCHEDULED_JOBS TABLE
-- Scheduled operations (publish, unpublish, etc.)
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_scheduled_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Job identification
    job_type VARCHAR(50) NOT NULL,

    -- Target
    content_id UUID REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Schedule
    scheduled_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) DEFAULT 'UTC',

    -- Job data
    payload JSONB DEFAULT '{}'::jsonb,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,

    -- Execution details
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    result JSONB,

    -- Lock for concurrent execution prevention
    locked_by VARCHAR(100),
    locked_at TIMESTAMPTZ,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL
);

CREATE INDEX idx_cms_jobs_pending ON cms_scheduled_jobs(scheduled_at) WHERE status = 'pending';
CREATE INDEX idx_cms_jobs_content ON cms_scheduled_jobs(content_id);
CREATE INDEX idx_cms_jobs_status ON cms_scheduled_jobs(status);
CREATE INDEX idx_cms_jobs_type ON cms_scheduled_jobs(job_type);

COMMENT ON TABLE cms_scheduled_jobs IS 'Queue for scheduled CMS operations';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 17. CMS_WEBHOOKS TABLE
-- Webhook configurations for external integrations
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Webhook info
    name VARCHAR(100) NOT NULL,
    description TEXT,
    url VARCHAR(500) NOT NULL,

    -- Security
    secret VARCHAR(255),

    -- Events
    events TEXT[] NOT NULL DEFAULT ARRAY['content.published'],
    content_types cms_content_type[],

    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT true,
    retry_count INTEGER NOT NULL DEFAULT 3,
    timeout_seconds INTEGER NOT NULL DEFAULT 30,

    -- Headers
    headers JSONB DEFAULT '{}'::jsonb,

    -- Stats
    last_triggered_at TIMESTAMPTZ,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL
);

CREATE INDEX idx_cms_webhooks_active ON cms_webhooks(is_active) WHERE is_active = true;
CREATE INDEX idx_cms_webhooks_events ON cms_webhooks USING GIN(events);

COMMENT ON TABLE cms_webhooks IS 'Webhook configurations for CMS event notifications';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 18. CMS_WEBHOOK_DELIVERIES TABLE
-- Webhook delivery log
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference
    webhook_id UUID NOT NULL REFERENCES cms_webhooks(id) ON DELETE CASCADE,

    -- Event
    event_type VARCHAR(100) NOT NULL,
    content_id UUID REFERENCES cms_content(id) ON DELETE SET NULL,
    payload JSONB NOT NULL,

    -- Delivery status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,

    -- Response
    response_status INTEGER,
    response_body TEXT,
    response_time_ms INTEGER,

    -- Error
    error_message TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    next_retry_at TIMESTAMPTZ
);

CREATE INDEX idx_cms_webhook_deliveries_webhook ON cms_webhook_deliveries(webhook_id);
CREATE INDEX idx_cms_webhook_deliveries_status ON cms_webhook_deliveries(status);
CREATE INDEX idx_cms_webhook_deliveries_retry ON cms_webhook_deliveries(next_retry_at) WHERE status = 'pending';
CREATE INDEX idx_cms_webhook_deliveries_created ON cms_webhook_deliveries(created_at DESC);

COMMENT ON TABLE cms_webhook_deliveries IS 'Log of webhook delivery attempts';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 19. CMS_SITE_SETTINGS TABLE
-- Global site settings (singleton pattern)
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_site_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Site identity
    site_name VARCHAR(255) NOT NULL DEFAULT 'Revolution Trading Pros',
    site_tagline VARCHAR(500),
    site_description TEXT,

    -- Logos and branding
    logo_light_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,
    logo_dark_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,
    favicon_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,
    og_default_image_id UUID REFERENCES cms_assets(id) ON DELETE SET NULL,

    -- Contact
    contact_email VARCHAR(255),
    support_email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,

    -- Social media
    social_links JSONB DEFAULT '{}'::jsonb,

    -- SEO defaults
    default_meta_title_suffix VARCHAR(100) DEFAULT ' | Revolution Trading Pros',
    default_robots VARCHAR(100) DEFAULT 'index, follow',
    google_analytics_id VARCHAR(50),
    google_tag_manager_id VARCHAR(50),

    -- Features
    maintenance_mode BOOLEAN NOT NULL DEFAULT false,
    maintenance_message TEXT,

    -- Custom scripts/styles
    head_scripts TEXT,
    body_start_scripts TEXT,
    body_end_scripts TEXT,
    custom_css TEXT,

    -- Settings blob for extensibility
    settings JSONB DEFAULT '{}'::jsonb,

    -- Audit fields
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Singleton constraint
    CONSTRAINT single_settings CHECK (id = '00000000-0000-0000-0000-000000000001'::UUID)
);

-- Insert default settings row
INSERT INTO cms_site_settings (id) VALUES ('00000000-0000-0000-0000-000000000001'::UUID)
ON CONFLICT (id) DO NOTHING;

COMMENT ON TABLE cms_site_settings IS 'Global site settings (singleton table)';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 20. CMS_NAVIGATION_MENUS TABLE
-- Navigation menu structures
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_navigation_menus (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Menu identification
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    location VARCHAR(50),

    -- Menu items (hierarchical JSON structure)
    items JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Constraints
    CONSTRAINT valid_menu_slug CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$')
);

CREATE INDEX idx_cms_navigation_slug ON cms_navigation_menus(slug);
CREATE INDEX idx_cms_navigation_location ON cms_navigation_menus(location);
CREATE INDEX idx_cms_navigation_active ON cms_navigation_menus(is_active) WHERE is_active = true;

COMMENT ON TABLE cms_navigation_menus IS 'Navigation menu structures with hierarchical items';
COMMENT ON COLUMN cms_navigation_menus.items IS 'JSON array of menu items with nested children';
COMMENT ON COLUMN cms_navigation_menus.location IS 'Menu location identifier (e.g., header, footer, sidebar)';

-- Insert default menus
INSERT INTO cms_navigation_menus (name, slug, location, items) VALUES
('Header Navigation', 'header', 'header', '[]'::jsonb),
('Footer Navigation', 'footer', 'footer', '[]'::jsonb),
('Mobile Navigation', 'mobile', 'mobile', '[]'::jsonb)
ON CONFLICT (slug) DO NOTHING;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 21. CMS_REDIRECTS TABLE
-- URL redirect management
-- ─────────────────────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cms_redirects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Redirect paths
    source_path VARCHAR(500) NOT NULL,
    target_path VARCHAR(500) NOT NULL,

    -- Type
    status_code INTEGER NOT NULL DEFAULT 301,
    is_regex BOOLEAN NOT NULL DEFAULT false,

    -- Conditions
    preserve_query_string BOOLEAN NOT NULL DEFAULT true,

    -- Stats
    hit_count INTEGER NOT NULL DEFAULT 0,
    last_hit_at TIMESTAMPTZ,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    -- Constraints
    CONSTRAINT valid_status_code CHECK (status_code IN (301, 302, 307, 308)),
    CONSTRAINT unique_source_path UNIQUE (source_path)
);

CREATE INDEX idx_cms_redirects_source ON cms_redirects(source_path);
CREATE INDEX idx_cms_redirects_active ON cms_redirects(is_active) WHERE is_active = true;

COMMENT ON TABLE cms_redirects IS 'URL redirect rules for SEO and content migration';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- HELPER FUNCTIONS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Function to create a content revision
CREATE OR REPLACE FUNCTION cms_create_revision(
    p_content_id UUID,
    p_data JSONB,
    p_created_by UUID,
    p_change_summary VARCHAR DEFAULT NULL,
    p_changed_fields TEXT[] DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_revision_number INTEGER;
    v_new_id UUID;
BEGIN
    -- Get next revision number
    SELECT COALESCE(MAX(revision_number), 0) + 1 INTO v_revision_number
    FROM cms_revisions
    WHERE content_id = p_content_id;

    -- Mark previous revision as not current
    UPDATE cms_revisions
    SET is_current = false
    WHERE content_id = p_content_id;

    -- Insert new revision
    INSERT INTO cms_revisions (
        content_id, revision_number, is_current,
        data, change_summary, changed_fields, created_by
    ) VALUES (
        p_content_id, v_revision_number, true,
        p_data, p_change_summary, p_changed_fields, p_created_by
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$ LANGUAGE plpgsql;

-- Function to log CMS audit event
CREATE OR REPLACE FUNCTION cms_log_audit(
    p_action VARCHAR,
    p_entity_type VARCHAR,
    p_entity_id UUID,
    p_user_id UUID,
    p_user_email VARCHAR DEFAULT NULL,
    p_user_role cms_user_role DEFAULT NULL,
    p_old_values JSONB DEFAULT NULL,
    p_new_values JSONB DEFAULT NULL,
    p_metadata JSONB DEFAULT NULL,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_new_id UUID;
BEGIN
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id, user_email, user_role,
        old_values, new_values, metadata, ip_address, user_agent
    ) VALUES (
        p_action, p_entity_type, p_entity_id, p_user_id, p_user_email, p_user_role,
        p_old_values, p_new_values, p_metadata, p_ip_address, p_user_agent
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$ LANGUAGE plpgsql;

-- Function to update tag usage counts
CREATE OR REPLACE FUNCTION cms_update_tag_usage() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_tags SET usage_count = GREATEST(usage_count - 1, 0) WHERE id = OLD.tag_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger for tag usage
DROP TRIGGER IF EXISTS cms_content_tags_usage_trigger ON cms_content_tags;
CREATE TRIGGER cms_content_tags_usage_trigger
    AFTER INSERT OR DELETE ON cms_content_tags
    FOR EACH ROW EXECUTE FUNCTION cms_update_tag_usage();

-- Function to update asset usage counts
CREATE OR REPLACE FUNCTION cms_update_asset_usage(
    p_asset_id UUID,
    p_increment INTEGER
) RETURNS VOID AS $$
BEGIN
    UPDATE cms_assets
    SET
        usage_count = GREATEST(usage_count + p_increment, 0),
        last_used_at = CASE WHEN p_increment > 0 THEN NOW() ELSE last_used_at END
    WHERE id = p_asset_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get folder path
CREATE OR REPLACE FUNCTION cms_get_folder_path(p_folder_id UUID) RETURNS TEXT AS $$
DECLARE
    v_path TEXT := '';
    v_current_id UUID := p_folder_id;
    v_slug VARCHAR;
    v_parent_id UUID;
BEGIN
    WHILE v_current_id IS NOT NULL LOOP
        SELECT slug, parent_id INTO v_slug, v_parent_id
        FROM cms_asset_folders
        WHERE id = v_current_id;

        IF v_slug IS NOT NULL THEN
            v_path := '/' || v_slug || v_path;
        END IF;

        v_current_id := v_parent_id;
    END LOOP;

    RETURN COALESCE(NULLIF(v_path, ''), '/');
END;
$$ LANGUAGE plpgsql;

-- Trigger to update folder path on insert/update
CREATE OR REPLACE FUNCTION cms_update_folder_path() RETURNS TRIGGER AS $$
BEGIN
    NEW.path := cms_get_folder_path(NEW.parent_id) || '/' || NEW.slug;
    NEW.depth := (SELECT COALESCE(depth, -1) + 1 FROM cms_asset_folders WHERE id = NEW.parent_id);
    IF NEW.depth IS NULL THEN NEW.depth := 0; END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_folder_path_trigger ON cms_asset_folders;
CREATE TRIGGER cms_folder_path_trigger
    BEFORE INSERT OR UPDATE ON cms_asset_folders
    FOR EACH ROW EXECUTE FUNCTION cms_update_folder_path();

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- UPDATED_AT TRIGGERS
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at trigger to all CMS tables
DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN SELECT unnest(ARRAY[
        'cms_users', 'cms_asset_folders', 'cms_assets', 'cms_content',
        'cms_tags', 'cms_comments', 'cms_scheduled_jobs', 'cms_webhooks',
        'cms_navigation_menus', 'cms_redirects'
    ]) LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS set_updated_at ON %I', t);
        EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %I FOR EACH ROW EXECUTE FUNCTION cms_set_updated_at()', t);
    END LOOP;
END$$;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- TABLE COMMENTS SUMMARY
-- ─────────────────────────────────────────────────────────────────────────────────────────

COMMENT ON FUNCTION cms_create_revision IS 'Create a new revision for content versioning';
COMMENT ON FUNCTION cms_log_audit IS 'Log an action to the partitioned audit table';
COMMENT ON FUNCTION cms_update_tag_usage IS 'Trigger function to maintain tag usage counts';
COMMENT ON FUNCTION cms_update_asset_usage IS 'Update asset usage statistics';
COMMENT ON FUNCTION cms_get_folder_path IS 'Calculate full path for a folder';
COMMENT ON FUNCTION cms_update_folder_path IS 'Trigger to maintain folder paths and depths';
COMMENT ON FUNCTION cms_set_updated_at IS 'Standard updated_at trigger function';
