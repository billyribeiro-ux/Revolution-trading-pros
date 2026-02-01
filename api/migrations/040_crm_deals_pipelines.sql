-- ═══════════════════════════════════════════════════════════════════════════════════
-- CRM Deals & Pipelines - ICT Level 7 Complete Implementation
-- Apple Principal Engineer Grade - January 2026
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. PIPELINES
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_pipelines (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_default BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    color VARCHAR(20) DEFAULT '#6366f1',
    icon VARCHAR(50),
    position INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Ensure only one default pipeline
CREATE UNIQUE INDEX IF NOT EXISTS idx_crm_pipelines_default ON crm_pipelines(is_default) WHERE is_default = true;
CREATE INDEX IF NOT EXISTS idx_crm_pipelines_active ON crm_pipelines(is_active, position);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. PIPELINE STAGES
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_pipeline_stages (
    id BIGSERIAL PRIMARY KEY,
    pipeline_id BIGINT NOT NULL REFERENCES crm_pipelines(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    position INTEGER DEFAULT 0,
    probability INTEGER DEFAULT 50 CHECK (probability >= 0 AND probability <= 100),
    is_closed_won BOOLEAN DEFAULT false,
    is_closed_lost BOOLEAN DEFAULT false,
    auto_advance_after_days INTEGER,
    required_activities JSONB DEFAULT '[]'::jsonb,
    color VARCHAR(20) DEFAULT '#6366f1',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_crm_stages_pipeline ON crm_pipeline_stages(pipeline_id, position);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. DEALS
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_deals (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
    company_id BIGINT REFERENCES crm_companies(id) ON DELETE SET NULL,
    pipeline_id BIGINT NOT NULL REFERENCES crm_pipelines(id) ON DELETE CASCADE,
    stage_id BIGINT NOT NULL REFERENCES crm_pipeline_stages(id) ON DELETE RESTRICT,
    owner_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    amount DECIMAL(15, 2) DEFAULT 0.00,
    currency VARCHAR(3) DEFAULT 'USD',
    probability INTEGER DEFAULT 50 CHECK (probability >= 0 AND probability <= 100),
    status VARCHAR(20) DEFAULT 'open' CHECK (status IN ('open', 'won', 'lost')),
    expected_close_date DATE,
    close_date DATE,
    lost_reason TEXT,
    won_details TEXT,
    priority VARCHAR(20) DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    source_channel VARCHAR(100),
    source_campaign VARCHAR(255),
    tags JSONB DEFAULT '[]'::jsonb,
    custom_fields JSONB DEFAULT '{}'::jsonb,
    stage_entered_at TIMESTAMPTZ DEFAULT NOW(),
    closed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_crm_deals_pipeline ON crm_deals(pipeline_id, stage_id);
CREATE INDEX IF NOT EXISTS idx_crm_deals_status ON crm_deals(status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_crm_deals_contact ON crm_deals(contact_id);
CREATE INDEX IF NOT EXISTS idx_crm_deals_owner ON crm_deals(owner_id);
CREATE INDEX IF NOT EXISTS idx_crm_deals_expected_close ON crm_deals(expected_close_date) WHERE status = 'open';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. DEAL STAGE HISTORY (for tracking stage changes)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_deal_stage_history (
    id BIGSERIAL PRIMARY KEY,
    deal_id BIGINT NOT NULL REFERENCES crm_deals(id) ON DELETE CASCADE,
    from_stage_id BIGINT REFERENCES crm_pipeline_stages(id) ON DELETE SET NULL,
    to_stage_id BIGINT NOT NULL REFERENCES crm_pipeline_stages(id) ON DELETE CASCADE,
    changed_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    reason TEXT,
    duration_in_stage INTEGER, -- seconds in previous stage
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_crm_deal_history_deal ON crm_deal_stage_history(deal_id, created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. DEAL ACTIVITIES
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_deal_activities (
    id BIGSERIAL PRIMARY KEY,
    deal_id BIGINT NOT NULL REFERENCES crm_deals(id) ON DELETE CASCADE,
    activity_type VARCHAR(50) NOT NULL, -- call, meeting, email, task, note
    title VARCHAR(255),
    description TEXT,
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    assigned_to BIGINT REFERENCES users(id) ON DELETE SET NULL,
    priority VARCHAR(20) DEFAULT 'medium',
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_crm_deal_activities_deal ON crm_deal_activities(deal_id, created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. SEED DEFAULT PIPELINE
-- ═══════════════════════════════════════════════════════════════════════════════════
INSERT INTO crm_pipelines (name, description, is_default, is_active, color, position)
VALUES ('Sales Pipeline', 'Default sales pipeline for tracking deals', true, true, '#6366f1', 0)
ON CONFLICT DO NOTHING;

-- Get the default pipeline ID and insert stages
DO $$
DECLARE
    v_pipeline_id BIGINT;
BEGIN
    SELECT id INTO v_pipeline_id FROM crm_pipelines WHERE is_default = true LIMIT 1;

    IF v_pipeline_id IS NOT NULL THEN
        -- Insert default stages if they don't exist
        INSERT INTO crm_pipeline_stages (pipeline_id, name, position, probability, color, is_closed_won, is_closed_lost)
        SELECT v_pipeline_id, name, position, probability, color, is_closed_won, is_closed_lost
        FROM (VALUES
            ('Lead', 0, 10, '#94a3b8', false, false),
            ('Qualified', 1, 25, '#22d3ee', false, false),
            ('Proposal', 2, 50, '#a78bfa', false, false),
            ('Negotiation', 3, 75, '#fbbf24', false, false),
            ('Closed Won', 4, 100, '#22c55e', true, false),
            ('Closed Lost', 5, 0, '#ef4444', false, true)
        ) AS stages(name, position, probability, color, is_closed_won, is_closed_lost)
        WHERE NOT EXISTS (SELECT 1 FROM crm_pipeline_stages WHERE pipeline_id = v_pipeline_id);
    END IF;
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. ADD MISSING COLUMNS TO EXISTING CRM TABLES (for compatibility)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add missing columns to crm_tags
ALTER TABLE crm_tags ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_tags ADD COLUMN IF NOT EXISTS slug VARCHAR(255);
ALTER TABLE crm_tags ADD COLUMN IF NOT EXISTS contacts_count INTEGER DEFAULT 0;
ALTER TABLE crm_tags ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- Update title from name if title is null
UPDATE crm_tags SET title = name WHERE title IS NULL;
UPDATE crm_tags SET slug = LOWER(REPLACE(name, ' ', '-')) WHERE slug IS NULL;

-- Add missing columns to crm_lists
ALTER TABLE crm_lists ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_lists ADD COLUMN IF NOT EXISTS slug VARCHAR(255);
ALTER TABLE crm_lists ADD COLUMN IF NOT EXISTS is_public BOOLEAN DEFAULT false;
ALTER TABLE crm_lists ADD COLUMN IF NOT EXISTS contacts_count INTEGER DEFAULT 0;

-- Update title from name if title is null
UPDATE crm_lists SET title = name WHERE title IS NULL;
UPDATE crm_lists SET slug = LOWER(REPLACE(name, ' ', '-')) WHERE slug IS NULL;

-- Add missing columns to crm_segments
ALTER TABLE crm_segments ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_segments ADD COLUMN IF NOT EXISTS slug VARCHAR(255);
ALTER TABLE crm_segments ADD COLUMN IF NOT EXISTS match_type VARCHAR(20) DEFAULT 'all';
ALTER TABLE crm_segments ADD COLUMN IF NOT EXISTS contacts_count INTEGER DEFAULT 0;
ALTER TABLE crm_segments ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ;

-- Update title from name if title is null
UPDATE crm_segments SET title = name WHERE title IS NULL;
UPDATE crm_segments SET slug = LOWER(REPLACE(name, ' ', '-')) WHERE slug IS NULL;

-- Add missing columns to crm_campaigns
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS recipients_count INTEGER DEFAULT 0;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS emails_sent INTEGER DEFAULT 0;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS opens INTEGER DEFAULT 0;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS clicks INTEGER DEFAULT 0;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS open_rate DECIMAL(5, 2) DEFAULT 0.00;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS click_rate DECIMAL(5, 2) DEFAULT 0.00;
ALTER TABLE crm_campaigns ADD COLUMN IF NOT EXISTS template_id BIGINT REFERENCES crm_templates(id) ON DELETE SET NULL;

-- Update title from name if title is null
UPDATE crm_campaigns SET title = name WHERE title IS NULL;

-- Add missing columns to crm_sequences
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'draft';
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS trigger_type VARCHAR(50);
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS email_count INTEGER DEFAULT 0;
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS total_subscribers INTEGER DEFAULT 0;
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS emails_sent INTEGER DEFAULT 0;
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS open_rate DECIMAL(5, 2) DEFAULT 0.00;
ALTER TABLE crm_sequences ADD COLUMN IF NOT EXISTS click_rate DECIMAL(5, 2) DEFAULT 0.00;

-- Update title from name if title is null
UPDATE crm_sequences SET title = name WHERE title IS NULL;

-- Add missing columns to crm_automations
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'draft';
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS trigger_name VARCHAR(255);
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS actions_count INTEGER DEFAULT 0;
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS subscribers_count INTEGER DEFAULT 0;
ALTER TABLE crm_automations ADD COLUMN IF NOT EXISTS conversion_rate DECIMAL(5, 2) DEFAULT 0.00;

-- Update title from name if title is null
UPDATE crm_automations SET title = name WHERE title IS NULL;

-- Add missing columns to crm_templates
ALTER TABLE crm_templates ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_templates ADD COLUMN IF NOT EXISTS body TEXT;
ALTER TABLE crm_templates ADD COLUMN IF NOT EXISTS thumbnail VARCHAR(500);
ALTER TABLE crm_templates ADD COLUMN IF NOT EXISTS is_global BOOLEAN DEFAULT false;

-- Update title from name and body from content if null
UPDATE crm_templates SET title = name WHERE title IS NULL;
UPDATE crm_templates SET body = content WHERE body IS NULL;

-- Add missing columns to crm_smart_links
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS short VARCHAR(50);
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS target_url VARCHAR(500);
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true;
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS unique_clicks INTEGER DEFAULT 0;
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS actions JSONB DEFAULT '[]'::jsonb;
ALTER TABLE crm_smart_links ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- Update title from name and short from short_code if null
UPDATE crm_smart_links SET title = name WHERE title IS NULL;
UPDATE crm_smart_links SET short = short_code WHERE short IS NULL;
UPDATE crm_smart_links SET target_url = url WHERE target_url IS NULL;

-- Add missing columns to crm_webhooks
ALTER TABLE crm_webhooks ADD COLUMN IF NOT EXISTS trigger_count INTEGER DEFAULT 0;
ALTER TABLE crm_webhooks ADD COLUMN IF NOT EXISTS failure_count INTEGER DEFAULT 0;
ALTER TABLE crm_webhooks ADD COLUMN IF NOT EXISTS last_triggered_at TIMESTAMPTZ;

-- Add missing columns to crm_companies
ALTER TABLE crm_companies ADD COLUMN IF NOT EXISTS logo_url VARCHAR(500);
ALTER TABLE crm_companies ADD COLUMN IF NOT EXISTS contacts_count INTEGER DEFAULT 0;
ALTER TABLE crm_companies ADD COLUMN IF NOT EXISTS deals_count INTEGER DEFAULT 0;
ALTER TABLE crm_companies ADD COLUMN IF NOT EXISTS total_deal_value DECIMAL(15, 2) DEFAULT 0.00;

-- Add missing columns to crm_recurring_campaigns
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS title VARCHAR(255);
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'active';
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS scheduling_settings JSONB;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS total_campaigns_sent INTEGER DEFAULT 0;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS total_emails_sent INTEGER DEFAULT 0;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS total_revenue DECIMAL(15, 2) DEFAULT 0.00;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS last_sent_at TIMESTAMPTZ;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS next_scheduled_at TIMESTAMPTZ;
ALTER TABLE crm_recurring_campaigns ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. ABANDONED CARTS TABLE (for cart recovery feature)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_abandoned_carts (
    id BIGSERIAL PRIMARY KEY,
    contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
    email VARCHAR(255) NOT NULL,
    cart_total DECIMAL(15, 2) DEFAULT 0.00,
    cart_items JSONB DEFAULT '[]'::jsonb,
    status VARCHAR(20) DEFAULT 'abandoned' CHECK (status IN ('abandoned', 'recovered', 'expired')),
    recovery_email_sent BOOLEAN DEFAULT false,
    recovered BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_crm_abandoned_carts_status ON crm_abandoned_carts(status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_crm_abandoned_carts_contact ON crm_abandoned_carts(contact_id);
