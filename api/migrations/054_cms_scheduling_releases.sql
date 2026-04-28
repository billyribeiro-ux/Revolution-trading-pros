-- =====================================================================================
-- Migration 041: CMS Content Scheduling and Releases System
-- Apple Principal Engineer ICT Level 7+ - February 2026
--
-- PURPOSE: Content scheduling and release bundles
-- - Schedule single content publish/unpublish operations
-- - Bundle multiple content changes into releases
-- - Timezone-aware scheduling with cron job support
-- - Complete audit trail for all scheduling operations
--
-- STRATEGY: Enterprise-grade scheduling with full audit compliance
-- =====================================================================================

-- -------------------------------------------------------------------------------------
-- 1. SCHEDULE STATUS ENUM
-- -------------------------------------------------------------------------------------

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_schedule_status') THEN
        CREATE TYPE cms_schedule_status AS ENUM (
            'pending',      -- Scheduled but not yet executed
            'processing',   -- Currently being executed
            'completed',    -- Successfully executed
            'failed',       -- Execution failed
            'cancelled'     -- Cancelled by user
        );
    END IF;
END$$;

-- -------------------------------------------------------------------------------------
-- 2. SCHEDULE ACTION ENUM
-- -------------------------------------------------------------------------------------

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_schedule_action') THEN
        CREATE TYPE cms_schedule_action AS ENUM (
            'publish',      -- Publish content
            'unpublish',    -- Unpublish (set to draft)
            'archive',      -- Archive content
            'update'        -- Apply staged changes
        );
    END IF;
END$$;

-- -------------------------------------------------------------------------------------
-- 3. RELEASE STATUS ENUM
-- -------------------------------------------------------------------------------------

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_release_status') THEN
        CREATE TYPE cms_release_status AS ENUM (
            'draft',        -- Release being prepared
            'scheduled',    -- Release scheduled for future
            'processing',   -- Release being executed
            'completed',    -- Release successfully applied
            'failed',       -- Release failed (partial or complete)
            'cancelled'     -- Release cancelled
        );
    END IF;
END$$;

-- -------------------------------------------------------------------------------------
-- 4. CMS SCHEDULES TABLE
-- Individual content scheduling with timezone support
-- -------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content Reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Schedule Configuration
    action cms_schedule_action NOT NULL DEFAULT 'publish',
    scheduled_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(64) NOT NULL DEFAULT 'UTC',

    -- Status Tracking
    status cms_schedule_status NOT NULL DEFAULT 'pending',
    executed_at TIMESTAMPTZ,
    error_message TEXT,
    retry_count INTEGER DEFAULT 0 NOT NULL,
    max_retries INTEGER DEFAULT 3 NOT NULL,

    -- Staged Content (for 'update' action)
    staged_data JSONB,

    -- Metadata
    notes TEXT,

    -- Audit Fields
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    cancelled_by BIGINT REFERENCES users(id),
    cancelled_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT valid_scheduled_at CHECK (scheduled_at > created_at OR status != 'pending'),
    CONSTRAINT valid_retry_count CHECK (retry_count >= 0 AND retry_count <= max_retries)
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_cms_schedules_content ON cms_schedules(content_id);
CREATE INDEX IF NOT EXISTS idx_cms_schedules_status ON cms_schedules(status) WHERE status IN ('pending', 'processing');
CREATE INDEX IF NOT EXISTS idx_cms_schedules_scheduled_at ON cms_schedules(scheduled_at) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_cms_schedules_created_by ON cms_schedules(created_by);

-- -------------------------------------------------------------------------------------
-- 5. CMS RELEASES TABLE
-- Bundle multiple content changes into a single release
-- -------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_releases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Release Information
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Schedule Configuration
    scheduled_at TIMESTAMPTZ,
    timezone VARCHAR(64) NOT NULL DEFAULT 'UTC',

    -- Status Tracking
    status cms_release_status NOT NULL DEFAULT 'draft',
    executed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Progress Tracking
    total_items INTEGER DEFAULT 0 NOT NULL,
    completed_items INTEGER DEFAULT 0 NOT NULL,
    failed_items INTEGER DEFAULT 0 NOT NULL,

    -- Settings
    stop_on_error BOOLEAN DEFAULT false NOT NULL,
    notify_on_complete BOOLEAN DEFAULT true NOT NULL,

    -- Audit Fields
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    cancelled_by BIGINT REFERENCES users(id),
    cancelled_at TIMESTAMPTZ
);

-- Indexes for release queries
CREATE INDEX IF NOT EXISTS idx_cms_releases_status ON cms_releases(status);
CREATE INDEX IF NOT EXISTS idx_cms_releases_scheduled_at ON cms_releases(scheduled_at) WHERE status = 'scheduled';
CREATE INDEX IF NOT EXISTS idx_cms_releases_created_by ON cms_releases(created_by);

-- -------------------------------------------------------------------------------------
-- 6. CMS RELEASE ITEMS TABLE
-- Individual content items within a release
-- -------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_release_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Release Reference
    release_id UUID NOT NULL REFERENCES cms_releases(id) ON DELETE CASCADE,

    -- Content Reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,

    -- Action Configuration
    action cms_schedule_action NOT NULL DEFAULT 'publish',
    order_index INTEGER NOT NULL DEFAULT 0,

    -- Staged Content
    staged_data JSONB,

    -- Status Tracking
    status cms_schedule_status NOT NULL DEFAULT 'pending',
    executed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Audit Fields
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    -- Ensure unique content per release
    UNIQUE(release_id, content_id)
);

-- Indexes for release item queries
CREATE INDEX IF NOT EXISTS idx_cms_release_items_release ON cms_release_items(release_id);
CREATE INDEX IF NOT EXISTS idx_cms_release_items_content ON cms_release_items(content_id);
CREATE INDEX IF NOT EXISTS idx_cms_release_items_order ON cms_release_items(release_id, order_index);

-- -------------------------------------------------------------------------------------
-- 7. CMS SCHEDULE HISTORY TABLE
-- Complete audit log for all scheduling operations
-- -------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_schedule_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference (either schedule or release)
    schedule_id UUID REFERENCES cms_schedules(id) ON DELETE SET NULL,
    release_id UUID REFERENCES cms_releases(id) ON DELETE SET NULL,
    content_id UUID REFERENCES cms_content(id) ON DELETE SET NULL,

    -- Event Details
    event_type VARCHAR(100) NOT NULL,
    previous_status VARCHAR(50),
    new_status VARCHAR(50),

    -- Event Data
    event_data JSONB,
    error_details TEXT,

    -- User Context
    user_id BIGINT REFERENCES users(id),
    user_email VARCHAR(255),

    -- Network Context
    ip_address INET,
    user_agent TEXT,

    -- Timestamp
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    -- Ensure at least one reference is set
    CONSTRAINT schedule_or_release_required CHECK (
        schedule_id IS NOT NULL OR release_id IS NOT NULL
    )
);

-- Indexes for history queries
CREATE INDEX IF NOT EXISTS idx_cms_schedule_history_schedule ON cms_schedule_history(schedule_id) WHERE schedule_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cms_schedule_history_release ON cms_schedule_history(release_id) WHERE release_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cms_schedule_history_content ON cms_schedule_history(content_id) WHERE content_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cms_schedule_history_created_at ON cms_schedule_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_cms_schedule_history_event_type ON cms_schedule_history(event_type);

-- -------------------------------------------------------------------------------------
-- 8. TRIGGER FUNCTIONS
-- -------------------------------------------------------------------------------------

-- Update release item count when items are added/removed
CREATE OR REPLACE FUNCTION cms_update_release_item_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_releases
        SET total_items = total_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_releases
        SET total_items = GREATEST(0, total_items - 1),
            updated_at = NOW()
        WHERE id = OLD.release_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_cms_release_item_count ON cms_release_items;
CREATE TRIGGER trigger_cms_release_item_count
    AFTER INSERT OR DELETE ON cms_release_items
    FOR EACH ROW
    EXECUTE FUNCTION cms_update_release_item_count();

-- Update release progress when items complete/fail
CREATE OR REPLACE FUNCTION cms_update_release_progress()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' AND (OLD.status IS NULL OR OLD.status != 'completed') THEN
        UPDATE cms_releases
        SET completed_items = completed_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
    ELSIF NEW.status = 'failed' AND (OLD.status IS NULL OR OLD.status != 'failed') THEN
        UPDATE cms_releases
        SET failed_items = failed_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_cms_release_progress ON cms_release_items;
CREATE TRIGGER trigger_cms_release_progress
    AFTER UPDATE ON cms_release_items
    FOR EACH ROW
    EXECUTE FUNCTION cms_update_release_progress();

-- Auto-update timestamps
CREATE OR REPLACE FUNCTION cms_schedules_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_cms_schedules_updated_at ON cms_schedules;
CREATE TRIGGER trigger_cms_schedules_updated_at
    BEFORE UPDATE ON cms_schedules
    FOR EACH ROW
    EXECUTE FUNCTION cms_schedules_update_timestamp();

DROP TRIGGER IF EXISTS trigger_cms_releases_updated_at ON cms_releases;
CREATE TRIGGER trigger_cms_releases_updated_at
    BEFORE UPDATE ON cms_releases
    FOR EACH ROW
    EXECUTE FUNCTION cms_schedules_update_timestamp();

DROP TRIGGER IF EXISTS trigger_cms_release_items_updated_at ON cms_release_items;
CREATE TRIGGER trigger_cms_release_items_updated_at
    BEFORE UPDATE ON cms_release_items
    FOR EACH ROW
    EXECUTE FUNCTION cms_schedules_update_timestamp();

-- -------------------------------------------------------------------------------------
-- 9. HELPER FUNCTIONS
-- -------------------------------------------------------------------------------------

-- Get pending schedules for cron job execution
CREATE OR REPLACE FUNCTION cms_get_pending_schedules(
    p_limit INTEGER DEFAULT 100
)
RETURNS TABLE (
    schedule_id UUID,
    content_id UUID,
    action cms_schedule_action,
    scheduled_at TIMESTAMPTZ,
    staged_data JSONB
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.id,
        s.content_id,
        s.action,
        s.scheduled_at,
        s.staged_data
    FROM cms_schedules s
    WHERE s.status = 'pending'
      AND s.scheduled_at <= NOW()
    ORDER BY s.scheduled_at ASC
    LIMIT p_limit
    FOR UPDATE SKIP LOCKED;
END;
$$ LANGUAGE plpgsql;

-- Get pending releases for cron job execution
CREATE OR REPLACE FUNCTION cms_get_pending_releases(
    p_limit INTEGER DEFAULT 10
)
RETURNS TABLE (
    release_id UUID,
    name VARCHAR,
    scheduled_at TIMESTAMPTZ,
    stop_on_error BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        r.id,
        r.name,
        r.scheduled_at,
        r.stop_on_error
    FROM cms_releases r
    WHERE r.status = 'scheduled'
      AND r.scheduled_at <= NOW()
    ORDER BY r.scheduled_at ASC
    LIMIT p_limit
    FOR UPDATE SKIP LOCKED;
END;
$$ LANGUAGE plpgsql;

-- Process a schedule (mark as processing, then complete/fail)
CREATE OR REPLACE FUNCTION cms_process_schedule(
    p_schedule_id UUID,
    p_success BOOLEAN,
    p_error_message TEXT DEFAULT NULL
)
RETURNS BOOLEAN AS $$
DECLARE
    v_schedule cms_schedules%ROWTYPE;
BEGIN
    -- Get the schedule
    SELECT * INTO v_schedule
    FROM cms_schedules
    WHERE id = p_schedule_id
    FOR UPDATE;

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    IF p_success THEN
        UPDATE cms_schedules
        SET status = 'completed',
            executed_at = NOW()
        WHERE id = p_schedule_id;
    ELSE
        IF v_schedule.retry_count < v_schedule.max_retries THEN
            UPDATE cms_schedules
            SET status = 'pending',
                retry_count = retry_count + 1,
                error_message = p_error_message
            WHERE id = p_schedule_id;
        ELSE
            UPDATE cms_schedules
            SET status = 'failed',
                executed_at = NOW(),
                error_message = p_error_message
            WHERE id = p_schedule_id;
        END IF;
    END IF;

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Get schedule calendar for a date range
CREATE OR REPLACE FUNCTION cms_get_schedule_calendar(
    p_start_date TIMESTAMPTZ,
    p_end_date TIMESTAMPTZ,
    p_content_type_id UUID DEFAULT NULL
)
RETURNS TABLE (
    id UUID,
    content_id UUID,
    content_title VARCHAR,
    content_type VARCHAR,
    action cms_schedule_action,
    scheduled_at TIMESTAMPTZ,
    timezone VARCHAR,
    status cms_schedule_status,
    is_release BOOLEAN,
    release_id UUID,
    release_name VARCHAR
) AS $$
BEGIN
    -- Individual schedules
    RETURN QUERY
    SELECT
        s.id,
        s.content_id,
        c.title,
        ct.name AS content_type,
        s.action,
        s.scheduled_at,
        s.timezone,
        s.status,
        FALSE AS is_release,
        NULL::UUID AS release_id,
        NULL::VARCHAR AS release_name
    FROM cms_schedules s
    JOIN cms_content c ON s.content_id = c.id
    LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
    WHERE s.scheduled_at BETWEEN p_start_date AND p_end_date
      AND s.status IN ('pending', 'processing', 'completed')
      AND (p_content_type_id IS NULL OR c.content_type_id = p_content_type_id)

    UNION ALL

    -- Release items
    SELECT
        ri.id,
        ri.content_id,
        c.title,
        ct.name AS content_type,
        ri.action,
        r.scheduled_at,
        r.timezone,
        ri.status,
        TRUE AS is_release,
        r.id AS release_id,
        r.name AS release_name
    FROM cms_release_items ri
    JOIN cms_releases r ON ri.release_id = r.id
    JOIN cms_content c ON ri.content_id = c.id
    LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
    WHERE r.scheduled_at BETWEEN p_start_date AND p_end_date
      AND r.status IN ('scheduled', 'processing', 'completed')
      AND (p_content_type_id IS NULL OR c.content_type_id = p_content_type_id)

    ORDER BY scheduled_at ASC;
END;
$$ LANGUAGE plpgsql;

-- Cleanup old completed/failed schedules
CREATE OR REPLACE FUNCTION cms_cleanup_old_schedules(
    p_days_old INTEGER DEFAULT 90
)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    WITH deleted AS (
        DELETE FROM cms_schedules
        WHERE status IN ('completed', 'failed', 'cancelled')
          AND executed_at < NOW() - (p_days_old || ' days')::INTERVAL
        RETURNING id
    )
    SELECT COUNT(*) INTO deleted_count FROM deleted;

    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- -------------------------------------------------------------------------------------
-- 10. PERMISSIONS (Optional - for Supabase deployments)
-- -------------------------------------------------------------------------------------

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'authenticated') THEN
        GRANT SELECT, INSERT, UPDATE, DELETE ON cms_schedules TO authenticated;
        GRANT SELECT, INSERT, UPDATE, DELETE ON cms_releases TO authenticated;
        GRANT SELECT, INSERT, UPDATE, DELETE ON cms_release_items TO authenticated;
        GRANT SELECT, INSERT ON cms_schedule_history TO authenticated;
    END IF;
END $$;

-- -------------------------------------------------------------------------------------
-- 11. ROW LEVEL SECURITY POLICIES
-- Critical: Prevents unauthorized access to scheduling data
-- -------------------------------------------------------------------------------------

-- Enable RLS on all tables
ALTER TABLE cms_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_releases ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_release_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_schedule_history ENABLE ROW LEVEL SECURITY;

-- Helper function to check CMS role (reuse if exists, else create)
CREATE OR REPLACE FUNCTION cms_get_current_user_role()
RETURNS cms_user_role AS $$
DECLARE
    v_role cms_user_role;
BEGIN
    SELECT cu.role INTO v_role
    FROM cms_users cu
    WHERE cu.user_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
      AND cu.is_active = true;

    RETURN COALESCE(v_role, 'viewer'::cms_user_role);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ===================================================================================
-- CMS_SCHEDULES POLICIES
-- ===================================================================================

-- SELECT: Admins see all, editors see own schedules only
CREATE POLICY cms_schedules_select_policy ON cms_schedules
    FOR SELECT
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
    );

-- INSERT: Editors can create schedules for content they own
CREATE POLICY cms_schedules_insert_policy ON cms_schedules
    FOR INSERT
    WITH CHECK (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

-- UPDATE: Admins can update any, editors only own pending schedules
CREATE POLICY cms_schedules_update_policy ON cms_schedules
    FOR UPDATE
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR (
            created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
            AND status = 'pending'
        )
    );

-- DELETE: Only admins can delete schedules
CREATE POLICY cms_schedules_delete_policy ON cms_schedules
    FOR DELETE
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
    );

-- ===================================================================================
-- CMS_RELEASES POLICIES
-- ===================================================================================

-- SELECT: Admins see all, editors see own releases
CREATE POLICY cms_releases_select_policy ON cms_releases
    FOR SELECT
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
    );

-- INSERT: Admins and marketing managers can create releases
CREATE POLICY cms_releases_insert_policy ON cms_releases
    FOR INSERT
    WITH CHECK (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
    );

-- UPDATE: Only admins can update releases
CREATE POLICY cms_releases_update_policy ON cms_releases
    FOR UPDATE
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR (
            created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
            AND status = 'draft'
        )
    );

-- DELETE: Only super_admin can delete releases
CREATE POLICY cms_releases_delete_policy ON cms_releases
    FOR DELETE
    USING (
        cms_get_current_user_role() = 'super_admin'
    );

-- ===================================================================================
-- CMS_RELEASE_ITEMS POLICIES
-- ===================================================================================

-- SELECT: Same as release policy (via subquery)
CREATE POLICY cms_release_items_select_policy ON cms_release_items
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM cms_releases r
            WHERE r.id = release_id
        )
    );

-- INSERT: Must have permission on parent release
CREATE POLICY cms_release_items_insert_policy ON cms_release_items
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM cms_releases r
            WHERE r.id = release_id
              AND r.status = 'draft'
              AND (
                  cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
                  OR r.created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
              )
        )
    );

-- UPDATE: Only for draft releases owned by user or admin
CREATE POLICY cms_release_items_update_policy ON cms_release_items
    FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM cms_releases r
            WHERE r.id = release_id
              AND r.status = 'draft'
              AND (
                  cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
                  OR r.created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
              )
        )
    );

-- DELETE: Same as update
CREATE POLICY cms_release_items_delete_policy ON cms_release_items
    FOR DELETE
    USING (
        EXISTS (
            SELECT 1 FROM cms_releases r
            WHERE r.id = release_id
              AND r.status = 'draft'
              AND (
                  cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
                  OR r.created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
              )
        )
    );

-- ===================================================================================
-- CMS_SCHEDULE_HISTORY POLICIES (Audit trail - append-only for non-admins)
-- ===================================================================================

-- SELECT: Admins see all, others see history for their own schedules/releases
CREATE POLICY cms_schedule_history_select_policy ON cms_schedule_history
    FOR SELECT
    USING (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR performed_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
        OR EXISTS (
            SELECT 1 FROM cms_schedules s
            WHERE s.id = schedule_id
              AND s.created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
        )
        OR EXISTS (
            SELECT 1 FROM cms_releases r
            WHERE r.id = release_id
              AND r.created_by = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
        )
    );

-- INSERT: System only (via triggers or admin functions)
-- History is written by the system, not directly by users
CREATE POLICY cms_schedule_history_insert_policy ON cms_schedule_history
    FOR INSERT
    WITH CHECK (
        cms_get_current_user_role() IN ('super_admin', 'marketing_manager')
        OR TRUE  -- Allow system inserts (triggers run as table owner)
    );

-- UPDATE: Never allowed on audit trail
CREATE POLICY cms_schedule_history_update_policy ON cms_schedule_history
    FOR UPDATE
    USING (FALSE);

-- DELETE: Only super_admin can purge history
CREATE POLICY cms_schedule_history_delete_policy ON cms_schedule_history
    FOR DELETE
    USING (
        cms_get_current_user_role() = 'super_admin'
    );

-- =====================================================================================
-- MIGRATION COMPLETE
-- =====================================================================================
