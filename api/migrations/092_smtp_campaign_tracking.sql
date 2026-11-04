-- SMTP acceptance has no provider callback. Track only actual capability requests;
-- an image/link request does not assert inbox delivery or a human interaction.
ALTER TABLE crm_campaign_deliveries
    ADD COLUMN provider TEXT,
    ADD COLUMN tracking_token UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE;
UPDATE crm_campaign_deliveries SET provider='postmark' WHERE provider_stream IS NOT NULL;
ALTER TABLE crm_campaign_deliveries ADD CONSTRAINT crm_campaign_provider_valid
    CHECK(provider IS NULL OR provider IN ('postmark','smtp'));
CREATE TABLE crm_campaign_tracking_links (
    id UUID PRIMARY KEY,
    delivery_id UUID NOT NULL REFERENCES crm_campaign_deliveries(id) ON DELETE CASCADE,
    target TEXT NOT NULL CHECK(octet_length(target) BETWEEN 1 AND 4096),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX crm_campaign_tracking_links_delivery ON crm_campaign_tracking_links(delivery_id);
