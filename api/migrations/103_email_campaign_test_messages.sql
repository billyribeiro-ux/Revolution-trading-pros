-- Explicit administrator samples have their own durable idempotency envelope.
-- They never create audience/outbox rows or contribute to campaign metrics.
CREATE TABLE email_campaign_test_requests (
    request_id UUID PRIMARY KEY,
    campaign_id BIGINT REFERENCES email_campaigns(id) ON DELETE SET NULL,
    original_campaign_id BIGINT NOT NULL CHECK(original_campaign_id > 0),
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    authoring_version BIGINT NOT NULL CHECK(authoring_version > 0),
    recipients TEXT[] NOT NULL CHECK(cardinality(recipients) BETWEEN 1 AND 5),
    variant TEXT NOT NULL CHECK(variant IN ('a','b')),
    source_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX email_campaign_test_history ON email_campaign_test_requests(original_campaign_id,created_at DESC,request_id);
CREATE UNIQUE INDEX email_campaign_test_receipt ON email_logs(template_alias,(model->>'request_id'),to_email)
 WHERE template_type='native_campaign_test' AND model->>'request_id' IS NOT NULL;
