-- Preserve legacy campaign identities/content/counters. Authoring does not send.
ALTER TABLE email_campaigns
 ADD COLUMN authoring_config JSONB NOT NULL DEFAULT '{}'::jsonb,
 ADD COLUMN created_by BIGINT REFERENCES users(id),
 ADD COLUMN authoring_version BIGINT NOT NULL DEFAULT 1,
 ADD COLUMN delivery_campaign_id BIGINT UNIQUE REFERENCES crm_campaigns(id),
 ADD CONSTRAINT email_campaign_authoring_object CHECK(jsonb_typeof(authoring_config)='object'),
 ADD CONSTRAINT email_campaign_authoring_version CHECK(authoring_version>0);
CREATE INDEX email_campaign_created_order ON email_campaigns(created_at DESC,id DESC);
