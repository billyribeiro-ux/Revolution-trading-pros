-- ═══════════════════════════════════════════════════════════════════════════════════════
-- Migration 027: CMS-V2 Enterprise Features
-- Apple Principal Engineer ICT 7 Grade - January 27, 2026
--
-- PURPOSE: Add missing enterprise features to CMS-V2 to enable legacy CMS retirement
-- - Audit logging for compliance
-- - Workflow management for editorial process
-- - Preview tokens for stakeholder review
--
-- STRATEGY: Reuse proven legacy CMS patterns, adapt for CMS-V2 UUID-based architecture
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 1. CMS AUDIT LOGS
-- Compliance-grade audit trail for all CMS operations
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS cms_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Action tracking
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,
    
    -- User context
    user_id BIGINT REFERENCES users(id),
    user_email VARCHAR(255),
    
    -- Change tracking
    old_values JSONB,
    new_values JSONB,
    metadata JSONB,
    
    -- Network context
    ip_address INET,
    user_agent TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Indexes for audit log queries
CREATE INDEX IF NOT EXISTS idx_cms_audit_logs_entity ON cms_audit_logs(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_cms_audit_logs_user ON cms_audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_cms_audit_logs_action ON cms_audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_cms_audit_logs_created_at ON cms_audit_logs(created_at DESC);

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 2. CMS WORKFLOW STATUS
-- Multi-stage editorial workflow with assignments
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TYPE cms_workflow_stage AS ENUM (
    'draft',
    'in_review',
    'approved',
    'published',
    'archived'
);

CREATE TYPE cms_workflow_priority AS ENUM (
    'low',
    'normal',
    'high',
    'urgent'
);

CREATE TABLE IF NOT EXISTS cms_workflow_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Content reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,
    
    -- Workflow state
    current_stage cms_workflow_stage NOT NULL DEFAULT 'draft',
    previous_stage cms_workflow_stage,
    
    -- Assignment
    assigned_to BIGINT REFERENCES users(id),
    assigned_by BIGINT REFERENCES users(id),
    assigned_at TIMESTAMPTZ,
    due_date TIMESTAMPTZ,
    priority cms_workflow_priority DEFAULT 'normal',
    
    -- Notes and comments
    notes TEXT,
    transition_comment TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Ensure one workflow per content
    UNIQUE(content_id)
);

-- Indexes for workflow queries
CREATE INDEX IF NOT EXISTS idx_cms_workflow_status_content ON cms_workflow_status(content_id);
CREATE INDEX IF NOT EXISTS idx_cms_workflow_status_assigned ON cms_workflow_status(assigned_to) WHERE assigned_to IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cms_workflow_status_stage ON cms_workflow_status(current_stage);
CREATE INDEX IF NOT EXISTS idx_cms_workflow_status_due_date ON cms_workflow_status(due_date) WHERE due_date IS NOT NULL;

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 3. CMS WORKFLOW HISTORY
-- Track all workflow transitions for audit trail
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS cms_workflow_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Workflow reference
    workflow_id UUID NOT NULL REFERENCES cms_workflow_status(id) ON DELETE CASCADE,
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,
    
    -- Transition details
    from_stage cms_workflow_stage,
    to_stage cms_workflow_stage NOT NULL,
    comment TEXT,
    
    -- User context
    transitioned_by BIGINT REFERENCES users(id),
    
    -- Timestamp
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cms_workflow_history_workflow ON cms_workflow_history(workflow_id);
CREATE INDEX IF NOT EXISTS idx_cms_workflow_history_content ON cms_workflow_history(content_id);
CREATE INDEX IF NOT EXISTS idx_cms_workflow_history_created_at ON cms_workflow_history(created_at DESC);

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 4. CMS PREVIEW TOKENS
-- Shareable preview links for unpublished content
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS cms_preview_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Content reference
    content_id UUID NOT NULL REFERENCES cms_content(id) ON DELETE CASCADE,
    
    -- Token details
    token UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    
    -- Access control
    max_views INTEGER,
    view_count INTEGER DEFAULT 0 NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    
    -- Creator
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Last access tracking
    last_accessed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_cms_preview_tokens_content ON cms_preview_tokens(content_id);
CREATE INDEX IF NOT EXISTS idx_cms_preview_tokens_token ON cms_preview_tokens(token);
CREATE INDEX IF NOT EXISTS idx_cms_preview_tokens_expires ON cms_preview_tokens(expires_at);

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 5. CMS WEBHOOKS
-- Event-driven integrations for CMS operations
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS cms_webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Webhook details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    url TEXT NOT NULL,
    secret VARCHAR(255),
    
    -- Event configuration
    events TEXT[] NOT NULL DEFAULT '{}',
    content_types TEXT[],
    
    -- Status
    is_active BOOLEAN DEFAULT true NOT NULL,
    
    -- Retry configuration
    retry_count INTEGER DEFAULT 3 NOT NULL,
    timeout_seconds INTEGER DEFAULT 30 NOT NULL,
    
    -- Custom headers
    headers JSONB,
    
    -- Metadata
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cms_webhooks_active ON cms_webhooks(is_active) WHERE is_active = true;

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 6. CMS WEBHOOK DELIVERIES
-- Track webhook delivery attempts and responses
-- ───────────────────────────────────────────────────────────────────────────────────────

CREATE TYPE cms_webhook_delivery_status AS ENUM (
    'pending',
    'retrying',
    'delivered',
    'failed'
);

CREATE TABLE IF NOT EXISTS cms_webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Webhook reference
    webhook_id UUID NOT NULL REFERENCES cms_webhooks(id) ON DELETE CASCADE,
    
    -- Event details
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    
    -- Delivery status
    status cms_webhook_delivery_status DEFAULT 'pending' NOT NULL,
    attempts INTEGER DEFAULT 0 NOT NULL,
    
    -- Response tracking
    response_status INTEGER,
    response_body TEXT,
    error_message TEXT,
    
    -- Retry scheduling
    next_retry_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cms_webhook_deliveries_webhook ON cms_webhook_deliveries(webhook_id);
CREATE INDEX IF NOT EXISTS idx_cms_webhook_deliveries_status ON cms_webhook_deliveries(status) WHERE status IN ('pending', 'retrying');
CREATE INDEX IF NOT EXISTS idx_cms_webhook_deliveries_next_retry ON cms_webhook_deliveries(next_retry_at) WHERE next_retry_at IS NOT NULL;

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 7. HELPER FUNCTIONS
-- ───────────────────────────────────────────────────────────────────────────────────────

-- Function to automatically create workflow status for new content
CREATE OR REPLACE FUNCTION cms_create_workflow_status()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO cms_workflow_status (content_id, current_stage)
    VALUES (NEW.id, 'draft'::cms_workflow_stage)
    ON CONFLICT (content_id) DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-create workflow status
DROP TRIGGER IF EXISTS trigger_cms_create_workflow_status ON cms_content;
CREATE TRIGGER trigger_cms_create_workflow_status
    AFTER INSERT ON cms_content
    FOR EACH ROW
    EXECUTE FUNCTION cms_create_workflow_status();

-- Function to log workflow transitions
CREATE OR REPLACE FUNCTION cms_log_workflow_transition()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.current_stage IS DISTINCT FROM NEW.current_stage THEN
        INSERT INTO cms_workflow_history (
            workflow_id,
            content_id,
            from_stage,
            to_stage,
            comment,
            transitioned_by
        ) VALUES (
            NEW.id,
            NEW.content_id,
            OLD.current_stage,
            NEW.current_stage,
            NEW.transition_comment,
            (SELECT updated_by FROM cms_content WHERE id = NEW.content_id)
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-log workflow transitions
DROP TRIGGER IF EXISTS trigger_cms_log_workflow_transition ON cms_workflow_status;
CREATE TRIGGER trigger_cms_log_workflow_transition
    AFTER UPDATE ON cms_workflow_status
    FOR EACH ROW
    EXECUTE FUNCTION cms_log_workflow_transition();

-- Function to clean up expired preview tokens
CREATE OR REPLACE FUNCTION cms_cleanup_expired_preview_tokens()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM cms_preview_tokens
    WHERE expires_at < NOW()
       OR (max_views IS NOT NULL AND view_count >= max_views);
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- ───────────────────────────────────────────────────────────────────────────────────────
-- 8. GRANT PERMISSIONS
-- ───────────────────────────────────────────────────────────────────────────────────────

-- Grant access to authenticated users (adjust as needed for your auth system)
GRANT SELECT, INSERT, UPDATE ON cms_audit_logs TO authenticated;
GRANT SELECT, INSERT, UPDATE ON cms_workflow_status TO authenticated;
GRANT SELECT, INSERT ON cms_workflow_history TO authenticated;
GRANT SELECT, INSERT, UPDATE, DELETE ON cms_preview_tokens TO authenticated;
GRANT SELECT, INSERT, UPDATE, DELETE ON cms_webhooks TO authenticated;
GRANT SELECT, INSERT, UPDATE ON cms_webhook_deliveries TO authenticated;

-- ═══════════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════════
