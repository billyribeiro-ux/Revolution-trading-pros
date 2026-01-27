-- CMS AI Assist History Table
-- Apple ICT 7+ Principal Engineer Grade - January 2026
--
-- Tracks all AI assistance requests for:
-- - Usage analytics and reporting
-- - Content versioning and audit trails
-- - Rate limiting optimization
-- - Cost tracking and monitoring

-- Create the AI assist history table
CREATE TABLE IF NOT EXISTS cms_ai_assist_history (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Request details
    action VARCHAR(50) NOT NULL,
    input_content TEXT NOT NULL,
    output_content TEXT NOT NULL,
    options JSONB,

    -- Content tracking (optional)
    content_id UUID,
    block_id VARCHAR(255),

    -- Model and performance metrics
    model VARCHAR(100) NOT NULL,
    input_tokens INTEGER NOT NULL DEFAULT 0,
    output_tokens INTEGER NOT NULL DEFAULT 0,
    processing_time_ms INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_cms_ai_assist_history_user_id
    ON cms_ai_assist_history(user_id);

CREATE INDEX IF NOT EXISTS idx_cms_ai_assist_history_action
    ON cms_ai_assist_history(action);

CREATE INDEX IF NOT EXISTS idx_cms_ai_assist_history_content_id
    ON cms_ai_assist_history(content_id)
    WHERE content_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_cms_ai_assist_history_created_at
    ON cms_ai_assist_history(created_at DESC);

-- Composite index for user history queries
CREATE INDEX IF NOT EXISTS idx_cms_ai_assist_history_user_created
    ON cms_ai_assist_history(user_id, created_at DESC);

-- Add comment for documentation
COMMENT ON TABLE cms_ai_assist_history IS 'Tracks AI assistance requests for content editing, including usage metrics and content tracking';

COMMENT ON COLUMN cms_ai_assist_history.action IS 'AI action type: improve, shorten, expand, fix_grammar, change_tone, summarize, generate_faq, generate_meta, generate_alt, suggest_related';
COMMENT ON COLUMN cms_ai_assist_history.options IS 'JSON options passed to the AI (e.g., tone, max_length)';
COMMENT ON COLUMN cms_ai_assist_history.content_id IS 'Optional reference to CMS content UUID for tracking';
COMMENT ON COLUMN cms_ai_assist_history.block_id IS 'Optional reference to specific content block';
COMMENT ON COLUMN cms_ai_assist_history.model IS 'AI model used (e.g., claude-sonnet-4-20250514)';
COMMENT ON COLUMN cms_ai_assist_history.input_tokens IS 'Number of tokens in the input prompt';
COMMENT ON COLUMN cms_ai_assist_history.output_tokens IS 'Number of tokens in the AI response';
COMMENT ON COLUMN cms_ai_assist_history.processing_time_ms IS 'Total processing time in milliseconds';
