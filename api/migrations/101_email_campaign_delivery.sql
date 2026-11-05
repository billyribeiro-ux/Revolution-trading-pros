-- New submissions freeze exact subjects and cohort assignments. Historical
-- deliveries remain untouched; no subject or template attribution is inferred.
ALTER TABLE crm_campaigns
 ADD COLUMN native_email_campaign_id BIGINT UNIQUE REFERENCES email_campaigns(id) ON DELETE CASCADE,
 ADD COLUMN email_template_id BIGINT REFERENCES email_templates(id) ON DELETE SET NULL,
 ADD COLUMN ab_test_config JSONB,
 ADD CONSTRAINT crm_campaign_ab_object CHECK (ab_test_config IS NULL OR COALESCE(
  jsonb_typeof(ab_test_config)='object' AND jsonb_typeof(ab_test_config->'enabled')='boolean',false));
ALTER TABLE crm_campaign_deliveries
 ADD COLUMN email_template_id BIGINT REFERENCES email_templates(id) ON DELETE SET NULL,
 ADD COLUMN variant TEXT,
 ADD COLUMN subject_template TEXT,
 ADD COLUMN rendered_subject TEXT,
 ADD CONSTRAINT crm_campaign_variant_valid CHECK(variant IS NULL OR (variant IN ('a','b') AND subject_template IS NOT NULL));
CREATE INDEX crm_campaign_template_acceptance ON crm_campaign_deliveries(email_template_id) WHERE status='accepted' AND email_template_id IS NOT NULL;

ALTER TABLE crm_campaigns ADD CONSTRAINT crm_campaign_native_pair UNIQUE(native_email_campaign_id,id);
ALTER TABLE email_campaigns ADD CONSTRAINT email_campaign_engine_identity
 FOREIGN KEY(id,delivery_campaign_id) REFERENCES crm_campaigns(native_email_campaign_id,id)
 DEFERRABLE INITIALLY DEFERRED;

-- Opt-out remains possible after an administrator deletes a campaign. Only
-- existing, exact token/email mappings are retained; none are inferred.
CREATE TABLE campaign_unsubscribe_links (
 token UUID PRIMARY KEY,
 email VARCHAR(255) NOT NULL CHECK(email=LOWER(email)),
 delivery_id UUID UNIQUE REFERENCES crm_campaign_deliveries(id) ON DELETE SET NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
INSERT INTO campaign_unsubscribe_links(token,email,delivery_id,created_at)
 SELECT unsubscribe_token,email,id,created_at FROM crm_campaign_deliveries;
CREATE FUNCTION preserve_campaign_unsubscribe_link() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
 INSERT INTO campaign_unsubscribe_links(token,email,delivery_id)
 VALUES(NEW.unsubscribe_token,NEW.email,NEW.id);
 RETURN NEW;
END $$;
CREATE TRIGGER preserve_campaign_unsubscribe_link AFTER INSERT ON crm_campaign_deliveries
 FOR EACH ROW EXECUTE FUNCTION preserve_campaign_unsubscribe_link();
CREATE INDEX email_logs_accepted_template ON email_logs(email_template_id)
 WHERE email_template_id IS NOT NULL AND status IN ('sent','delivered','opened','clicked','bounced');
