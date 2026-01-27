-- ═══════════════════════════════════════════════════════════════════════════════════════
-- MIGRATION 025: Blog Editor Enhancements
-- Apple ICT 7+ Principal Engineer Grade - Revolution Trading Pros
-- ═══════════════════════════════════════════════════════════════════════════════════════
--
-- Features:
-- 1. Reusable Blocks - Save blocks as templates, sync across content
-- 2. Enhanced Comments - Improved threading and @mentions
-- 3. User Preferences - Editor settings, shortcuts, focus mode
-- 4. AI Assist History - Track AI usage and results
-- 5. Offline Queue - Store pending changes for sync
--
-- @version 2.0.0 - January 2026
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- ENUMS
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- Reusable block categories
DO $$ BEGIN
    CREATE TYPE cms_reusable_block_category AS ENUM (
        'general',
        'trading',
        'layout',
        'callout',
        'marketing',
        'navigation',
        'media',
        'form'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- AI assist action types
DO $$ BEGIN
    CREATE TYPE cms_ai_action AS ENUM (
        'improve',
        'shorten',
        'expand',
        'fix_grammar',
        'change_tone',
        'summarize',
        'generate_faq',
        'generate_meta',
        'generate_alt',
        'suggest_related',
        'custom'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Offline sync status
DO $$ BEGIN
    CREATE TYPE cms_sync_status AS ENUM (
        'pending',
        'syncing',
        'synced',
        'conflict',
        'failed'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- REUSABLE BLOCKS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS cms_reusable_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    block_data JSONB NOT NULL,
    category cms_reusable_block_category NOT NULL DEFAULT 'general',
    tags TEXT[] DEFAULT '{}',
    thumbnail_url TEXT,
    preview_html TEXT,
    usage_count INTEGER NOT NULL DEFAULT 0,
    is_global BOOLEAN NOT NULL DEFAULT true,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    allowed_content_types cms_content_type[] DEFAULT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL
);

-- Track where reusable blocks are used
CREATE TABLE IF NOT EXISTS cms_reusable_block_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reusable_block_id UUID NOT NULL REFERENCES cms_reusable_blocks(id) ON DELETE CASCADE,
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,
    block_instance_id VARCHAR(255) NOT NULL,
    is_synced BOOLEAN NOT NULL DEFAULT true,
    detached_at TIMESTAMPTZ,
    detached_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(content_id, block_instance_id)
);

-- Indexes for reusable blocks
CREATE INDEX IF NOT EXISTS idx_reusable_blocks_category ON cms_reusable_blocks(category);
CREATE INDEX IF NOT EXISTS idx_reusable_blocks_tags ON cms_reusable_blocks USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_reusable_blocks_name_search ON cms_reusable_blocks USING GIN(to_tsvector('english', name || ' ' || COALESCE(description, '')));
CREATE INDEX IF NOT EXISTS idx_reusable_block_usage_block ON cms_reusable_block_usage(reusable_block_id);
CREATE INDEX IF NOT EXISTS idx_reusable_block_usage_content ON cms_reusable_block_usage(content_id);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- USER EDITOR PREFERENCES
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS cms_user_editor_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES cms_users(id) ON DELETE CASCADE UNIQUE,

    -- Autosave settings
    autosave_enabled BOOLEAN NOT NULL DEFAULT true,
    autosave_interval_seconds INTEGER NOT NULL DEFAULT 10,

    -- Focus mode settings
    focus_mode_enabled BOOLEAN NOT NULL DEFAULT false,
    focus_dim_sidebar BOOLEAN NOT NULL DEFAULT true,
    focus_dim_toolbar BOOLEAN NOT NULL DEFAULT true,
    focus_dim_blocks BOOLEAN NOT NULL DEFAULT true,

    -- Writing goals
    daily_word_goal INTEGER DEFAULT NULL,
    show_word_count BOOLEAN NOT NULL DEFAULT true,
    show_reading_time BOOLEAN NOT NULL DEFAULT true,
    show_character_count BOOLEAN NOT NULL DEFAULT false,

    -- Keyboard shortcuts customization (override defaults)
    custom_shortcuts JSONB DEFAULT '{}',

    -- UI preferences
    sidebar_collapsed BOOLEAN NOT NULL DEFAULT false,
    show_block_handles BOOLEAN NOT NULL DEFAULT true,
    show_formatting_toolbar BOOLEAN NOT NULL DEFAULT true,
    default_block_type VARCHAR(50) DEFAULT 'paragraph',

    -- AI preferences
    ai_enabled BOOLEAN NOT NULL DEFAULT true,
    ai_auto_suggest BOOLEAN NOT NULL DEFAULT false,

    -- Theme
    editor_theme VARCHAR(20) DEFAULT 'system',
    font_size INTEGER DEFAULT 16,
    line_height DECIMAL(3,2) DEFAULT 1.75,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- AI ASSIST HISTORY
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS cms_ai_assist_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES cms_users(id) ON DELETE CASCADE,
    content_id UUID REFERENCES cms_content(id) ON DELETE SET NULL,
    block_id VARCHAR(255),

    action cms_ai_action NOT NULL,
    input_text TEXT NOT NULL,
    output_text TEXT,

    options JSONB DEFAULT '{}',
    model_used VARCHAR(100) DEFAULT 'claude-sonnet-4-20250514',

    input_tokens INTEGER,
    output_tokens INTEGER,
    latency_ms INTEGER,

    was_applied BOOLEAN DEFAULT false,
    applied_at TIMESTAMPTZ,

    error_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for AI history
CREATE INDEX IF NOT EXISTS idx_ai_history_user ON cms_ai_assist_history(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_history_content ON cms_ai_assist_history(content_id);
CREATE INDEX IF NOT EXISTS idx_ai_history_action ON cms_ai_assist_history(action);
CREATE INDEX IF NOT EXISTS idx_ai_history_created ON cms_ai_assist_history(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- OFFLINE SYNC QUEUE
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS cms_offline_sync_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES cms_users(id) ON DELETE CASCADE,
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    client_id VARCHAR(255) NOT NULL,
    operation VARCHAR(50) NOT NULL,

    local_version INTEGER NOT NULL,
    server_version INTEGER,

    payload JSONB NOT NULL,

    status cms_sync_status NOT NULL DEFAULT 'pending',

    conflict_data JSONB,
    resolution VARCHAR(50),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,

    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TIMESTAMPTZ,
    error_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    synced_at TIMESTAMPTZ
);

-- Index for sync queue
CREATE INDEX IF NOT EXISTS idx_sync_queue_user ON cms_offline_sync_queue(user_id);
CREATE INDEX IF NOT EXISTS idx_sync_queue_content ON cms_offline_sync_queue(content_id);
CREATE INDEX IF NOT EXISTS idx_sync_queue_status ON cms_offline_sync_queue(status);
CREATE INDEX IF NOT EXISTS idx_sync_queue_pending ON cms_offline_sync_queue(status, created_at) WHERE status = 'pending';

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- REVISION ENHANCEMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- Add change type to revisions if not exists
DO $$ BEGIN
    ALTER TABLE cms_revisions ADD COLUMN IF NOT EXISTS change_type VARCHAR(50) DEFAULT 'manual';
    ALTER TABLE cms_revisions ADD COLUMN IF NOT EXISTS word_count INTEGER;
    ALTER TABLE cms_revisions ADD COLUMN IF NOT EXISTS diff_stats JSONB;
EXCEPTION
    WHEN duplicate_column THEN NULL;
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- COMMENT ENHANCEMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- Add notification tracking for comments
CREATE TABLE IF NOT EXISTS cms_comment_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id UUID NOT NULL REFERENCES cms_comments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES cms_users(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(comment_id, user_id, notification_type)
);

CREATE INDEX IF NOT EXISTS idx_comment_notifications_user ON cms_comment_notifications(user_id, is_read);

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- FUNCTIONS
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- Function to update reusable block usage count
CREATE OR REPLACE FUNCTION update_reusable_block_usage_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_reusable_blocks
        SET usage_count = usage_count + 1,
            updated_at = NOW()
        WHERE id = NEW.reusable_block_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_reusable_blocks
        SET usage_count = GREATEST(0, usage_count - 1),
            updated_at = NOW()
        WHERE id = OLD.reusable_block_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger for usage count
DROP TRIGGER IF EXISTS trg_reusable_block_usage_count ON cms_reusable_block_usage;
CREATE TRIGGER trg_reusable_block_usage_count
    AFTER INSERT OR DELETE ON cms_reusable_block_usage
    FOR EACH ROW
    EXECUTE FUNCTION update_reusable_block_usage_count();

-- Function to compute diff stats between two revisions
CREATE OR REPLACE FUNCTION cms_compute_revision_diff(
    p_content_id UUID,
    p_version_from INTEGER,
    p_version_to INTEGER
)
RETURNS JSONB AS $$
DECLARE
    v_from_data JSONB;
    v_to_data JSONB;
    v_from_title TEXT;
    v_to_title TEXT;
    v_from_content TEXT;
    v_to_content TEXT;
    v_added_blocks INTEGER := 0;
    v_removed_blocks INTEGER := 0;
    v_modified_blocks INTEGER := 0;
    v_result JSONB;
BEGIN
    -- Get revision data
    SELECT data INTO v_from_data
    FROM cms_revisions
    WHERE content_id = p_content_id AND revision_number = p_version_from;

    SELECT data INTO v_to_data
    FROM cms_revisions
    WHERE content_id = p_content_id AND revision_number = p_version_to;

    IF v_from_data IS NULL OR v_to_data IS NULL THEN
        RETURN NULL;
    END IF;

    -- Extract titles
    v_from_title := v_from_data->>'title';
    v_to_title := v_to_data->>'title';

    -- Extract content
    v_from_content := v_from_data->>'content';
    v_to_content := v_to_data->>'content';

    -- Count block changes (simplified - frontend does detailed diff)
    v_added_blocks := COALESCE(jsonb_array_length(v_to_data->'content_blocks'), 0) -
                      COALESCE(jsonb_array_length(v_from_data->'content_blocks'), 0);
    IF v_added_blocks < 0 THEN
        v_removed_blocks := ABS(v_added_blocks);
        v_added_blocks := 0;
    END IF;

    -- Build result
    v_result := jsonb_build_object(
        'version_from', p_version_from,
        'version_to', p_version_to,
        'title_changed', v_from_title IS DISTINCT FROM v_to_title,
        'content_changed', v_from_content IS DISTINCT FROM v_to_content,
        'blocks_added', v_added_blocks,
        'blocks_removed', v_removed_blocks,
        'from_data', v_from_data,
        'to_data', v_to_data
    );

    RETURN v_result;
END;
$$ LANGUAGE plpgsql;

-- Function to create comment notification
CREATE OR REPLACE FUNCTION cms_notify_comment_mention()
RETURNS TRIGGER AS $$
DECLARE
    v_mentioned_user UUID;
BEGIN
    -- Create notifications for mentioned users
    IF NEW.mentioned_users IS NOT NULL THEN
        FOREACH v_mentioned_user IN ARRAY NEW.mentioned_users
        LOOP
            INSERT INTO cms_comment_notifications (comment_id, user_id, notification_type)
            VALUES (NEW.id, v_mentioned_user, 'mention')
            ON CONFLICT (comment_id, user_id, notification_type) DO NOTHING;
        END LOOP;
    END IF;

    -- Notify thread participants of reply
    IF NEW.parent_id IS NOT NULL THEN
        INSERT INTO cms_comment_notifications (comment_id, user_id, notification_type)
        SELECT NEW.id, c.created_by, 'reply'
        FROM cms_comments c
        WHERE c.thread_id = NEW.thread_id
        AND c.created_by IS NOT NULL
        AND c.created_by != NEW.created_by
        ON CONFLICT (comment_id, user_id, notification_type) DO NOTHING;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for comment notifications
DROP TRIGGER IF EXISTS trg_comment_notifications ON cms_comments;
CREATE TRIGGER trg_comment_notifications
    AFTER INSERT ON cms_comments
    FOR EACH ROW
    EXECUTE FUNCTION cms_notify_comment_mention();

-- Function to process offline sync queue
CREATE OR REPLACE FUNCTION cms_process_sync_item(
    p_sync_id UUID,
    p_user_id UUID
)
RETURNS JSONB AS $$
DECLARE
    v_sync RECORD;
    v_content RECORD;
    v_result JSONB;
BEGIN
    -- Get sync item
    SELECT * INTO v_sync
    FROM cms_offline_sync_queue
    WHERE id = p_sync_id AND user_id = p_user_id AND status = 'pending'
    FOR UPDATE SKIP LOCKED;

    IF v_sync IS NULL THEN
        RETURN jsonb_build_object('success', false, 'error', 'Sync item not found or already processed');
    END IF;

    -- Get current content version
    SELECT id, version INTO v_content
    FROM cms_content
    WHERE id = v_sync.content_id;

    IF v_content IS NULL THEN
        UPDATE cms_offline_sync_queue
        SET status = 'failed', error_message = 'Content not found'
        WHERE id = p_sync_id;
        RETURN jsonb_build_object('success', false, 'error', 'Content not found');
    END IF;

    -- Check for conflict
    IF v_content.version > v_sync.local_version THEN
        UPDATE cms_offline_sync_queue
        SET status = 'conflict',
            server_version = v_content.version,
            conflict_data = (SELECT row_to_json(c) FROM cms_content c WHERE c.id = v_sync.content_id)::JSONB
        WHERE id = p_sync_id;
        RETURN jsonb_build_object('success', false, 'conflict', true, 'server_version', v_content.version);
    END IF;

    -- Apply the sync (update content)
    UPDATE cms_content
    SET title = COALESCE(v_sync.payload->>'title', title),
        content = COALESCE(v_sync.payload->>'content', content),
        content_blocks = COALESCE(v_sync.payload->'content_blocks', content_blocks),
        version = version + 1,
        updated_at = NOW(),
        updated_by = p_user_id
    WHERE id = v_sync.content_id;

    -- Mark sync as complete
    UPDATE cms_offline_sync_queue
    SET status = 'synced',
        synced_at = NOW(),
        server_version = v_content.version + 1
    WHERE id = p_sync_id;

    RETURN jsonb_build_object('success', true, 'new_version', v_content.version + 1);
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- UPDATED_AT TRIGGERS
-- ═══════════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION cms_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_reusable_blocks_updated ON cms_reusable_blocks;
CREATE TRIGGER trg_reusable_blocks_updated
    BEFORE UPDATE ON cms_reusable_blocks
    FOR EACH ROW
    EXECUTE FUNCTION cms_update_timestamp();

DROP TRIGGER IF EXISTS trg_user_editor_prefs_updated ON cms_user_editor_preferences;
CREATE TRIGGER trg_user_editor_prefs_updated
    BEFORE UPDATE ON cms_user_editor_preferences
    FOR EACH ROW
    EXECUTE FUNCTION cms_update_timestamp();

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- ROW LEVEL SECURITY
-- ═══════════════════════════════════════════════════════════════════════════════════════

ALTER TABLE cms_reusable_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_user_editor_preferences ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_ai_assist_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_offline_sync_queue ENABLE ROW LEVEL SECURITY;

-- Reusable blocks policies
CREATE POLICY cms_reusable_blocks_select ON cms_reusable_blocks
    FOR SELECT USING (is_global = true OR created_by = cms_current_user_id());

CREATE POLICY cms_reusable_blocks_insert ON cms_reusable_blocks
    FOR INSERT WITH CHECK (cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor'));

CREATE POLICY cms_reusable_blocks_update ON cms_reusable_blocks
    FOR UPDATE USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager') OR
        (created_by = cms_current_user_id() AND NOT is_locked)
    );

CREATE POLICY cms_reusable_blocks_delete ON cms_reusable_blocks
    FOR DELETE USING (
        cms_current_user_role() = 'super_admin' OR
        (created_by = cms_current_user_id() AND NOT is_locked AND usage_count = 0)
    );

-- User preferences - users can only access their own
CREATE POLICY cms_user_prefs_own ON cms_user_editor_preferences
    FOR ALL USING (user_id = cms_current_user_id());

-- AI history - users can only see their own
CREATE POLICY cms_ai_history_own ON cms_ai_assist_history
    FOR ALL USING (user_id = cms_current_user_id());

-- Sync queue - users can only access their own
CREATE POLICY cms_sync_queue_own ON cms_offline_sync_queue
    FOR ALL USING (user_id = cms_current_user_id());

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- SEED DATA
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- Insert some default reusable blocks
INSERT INTO cms_reusable_blocks (name, slug, description, block_data, category, tags, is_global)
VALUES
    ('Call to Action - Primary', 'cta-primary', 'Primary call-to-action block with button',
     '{"blockType": "cta-banner", "data": {"headline": "Ready to Start Trading?", "description": "Join thousands of successful traders", "buttonText": "Get Started", "buttonUrl": "/pricing", "variant": "primary"}}',
     'marketing', ARRAY['cta', 'conversion'], true),

    ('Trade Setup Template', 'trade-setup-template', 'Standard trade setup documentation block',
     '{"blockType": "trade-setup", "data": {"ticker": "", "direction": "long", "entryPrice": null, "stopLoss": null, "targets": [], "rationale": "", "timeframe": "swing"}}',
     'trading', ARRAY['trade', 'setup', 'template'], true),

    ('Two Column Layout', 'two-column-layout', 'Equal two-column layout for content',
     '{"blockType": "group", "data": {"layout": "columns", "columns": 2, "gap": "md"}, "children": []}',
     'layout', ARRAY['columns', 'layout', '2-col'], true),

    ('Info Callout', 'info-callout', 'Information callout box',
     '{"blockType": "callout", "data": {"type": "info", "title": "", "content": "", "icon": "info"}}',
     'callout', ARRAY['callout', 'info', 'notice'], true),

    ('Warning Callout', 'warning-callout', 'Warning callout box for important notices',
     '{"blockType": "callout", "data": {"type": "warning", "title": "Important", "content": "", "icon": "alert-triangle"}}',
     'callout', ARRAY['callout', 'warning', 'alert'], true),

    ('Performance Stats', 'performance-stats', 'Trading performance statistics display',
     '{"blockType": "performance-stats", "data": {"winRate": null, "avgWin": null, "avgLoss": null, "profitFactor": null, "period": "monthly"}}',
     'trading', ARRAY['stats', 'performance', 'metrics'], true)
ON CONFLICT (slug) DO NOTHING;

COMMIT;
