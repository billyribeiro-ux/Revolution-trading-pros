-- A submitted campaign owns immutable content revisions and recipient snapshots.
-- Provider acceptance, delivery, tracking and uncertainty remain distinct facts.
ALTER TABLE crm_campaigns
    ADD COLUMN authoring_version BIGINT NOT NULL DEFAULT 1,
    ADD COLUMN pause_reason TEXT,
    ADD COLUMN audience_snapshot JSONB,
    ADD COLUMN delivery_stats JSONB;
ALTER TABLE crm_campaigns ADD CONSTRAINT crm_campaign_pause_reason_valid
    CHECK (pause_reason IS NULL OR pause_reason IN
        ('manual','provider_unavailable','uncertain_submission'));

CREATE TABLE crm_campaign_revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id BIGINT NOT NULL REFERENCES crm_campaigns(id) ON DELETE CASCADE,
    authoring_version BIGINT NOT NULL CHECK (authoring_version>0),
    definition JSONB NOT NULL CHECK (jsonb_typeof(definition)='object'),
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(campaign_id,authoring_version),
    UNIQUE(id,campaign_id)
);

CREATE TABLE crm_campaign_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id BIGINT NOT NULL REFERENCES crm_campaigns(id) ON DELETE CASCADE,
    revision_id UUID NOT NULL,
    contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
    email VARCHAR(255) NOT NULL CHECK (email=LOWER(email)),
    display_name TEXT NOT NULL,
    contact_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(contact_snapshot)='object'),
    sender_identity TEXT,
    provider_stream TEXT,
    status TEXT NOT NULL DEFAULT 'queued' CHECK
        (status IN ('queued','processing','accepted','failed','unknown','skipped')),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count>=0),
    attempts_remaining INTEGER NOT NULL DEFAULT 3 CHECK (attempts_remaining BETWEEN 0 AND 3),
    due_at TIMESTAMPTZ NOT NULL,
    lease_token UUID,
    lease_expires_at TIMESTAMPTZ,
    provider_message_id TEXT,
    last_error TEXT,
    unsubscribe_token UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,
    accepted_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,
    bounced_at TIMESTAMPTZ,
    complained_at TIMESTAMPTZ,
    unsubscribed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY(revision_id,campaign_id)
        REFERENCES crm_campaign_revisions(id,campaign_id) ON DELETE CASCADE,
    UNIQUE(campaign_id,email),
    CHECK(status<>'processing' OR (lease_token IS NOT NULL AND lease_expires_at IS NOT NULL)),
    CHECK(status<>'accepted' OR (provider_message_id IS NOT NULL AND accepted_at IS NOT NULL))
);
CREATE UNIQUE INDEX crm_campaign_delivery_provider_id
    ON crm_campaign_deliveries(provider_message_id) WHERE provider_message_id IS NOT NULL;
CREATE INDEX crm_campaign_delivery_due
    ON crm_campaign_deliveries(due_at,campaign_id) WHERE status='queued';
CREATE INDEX crm_campaign_delivery_lease
    ON crm_campaign_deliveries(lease_expires_at) WHERE status='processing';
CREATE INDEX crm_campaign_delivery_campaign_status
    ON crm_campaign_deliveries(campaign_id,status);

CREATE TABLE crm_campaign_events (
    fingerprint TEXT PRIMARY KEY,
    delivery_id UUID REFERENCES crm_campaign_deliveries(id) ON DELETE CASCADE,
    recipient_email VARCHAR(255) NOT NULL,
    provider_message_id TEXT,
    record_type TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    details JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(details)='object')
);
CREATE INDEX crm_campaign_events_delivery
    ON crm_campaign_events(delivery_id,occurred_at DESC);
CREATE INDEX crm_campaign_events_recipient
    ON crm_campaign_events(LOWER(recipient_email));
CREATE INDEX crm_campaign_newsletter_lookup
    ON newsletter_subscribers(LOWER(email),updated_at DESC NULLS LAST,id DESC);
CREATE INDEX crm_campaign_contact_email_lookup ON contacts(LOWER(email),id);
