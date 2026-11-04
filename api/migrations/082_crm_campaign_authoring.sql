-- Durable authoring fields used by the established CRM campaign contract.
ALTER TABLE crm_campaigns
    ADD COLUMN campaign_type TEXT NOT NULL DEFAULT 'regular',
    ADD COLUMN email_pre_header TEXT,
    ADD COLUMN design_template TEXT NOT NULL DEFAULT 'raw',
    ADD COLUMN template_config JSONB,
    ADD COLUMN settings JSONB,
    ADD COLUMN subscriber_settings JSONB,
    ADD COLUMN utm_settings JSONB,
    ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    ADD COLUMN failed_count BIGINT,
    ADD COLUMN last_error TEXT;
-- Unknown legacy delivery failures remain unknown; do not invent historical data.
ALTER TABLE crm_campaigns ADD CONSTRAINT crm_campaign_type_valid
    CHECK (campaign_type IN ('regular','plain_text','raw_html'));
