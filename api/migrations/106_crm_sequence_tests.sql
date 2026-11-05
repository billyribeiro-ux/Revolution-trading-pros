-- Explicit administrator samples retain their immutable source revision and
-- receipt. They never create enrollments or production delivery/outbox rows.
CREATE TABLE crm_sequence_test_requests (
    request_id UUID PRIMARY KEY,
    sequence_id BIGINT NOT NULL REFERENCES crm_sequences(id),
    revision_id UUID NOT NULL,
    email_id UUID NOT NULL,
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    authoring_version BIGINT NOT NULL CHECK(authoring_version > 0),
    recipient TEXT NOT NULL CHECK(recipient = LOWER(BTRIM(recipient)) AND LENGTH(recipient) BETWEEN 1 AND 255),
    source_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY(revision_id,sequence_id) REFERENCES crm_sequence_revisions(id,sequence_id)
);
CREATE INDEX crm_sequence_test_history ON crm_sequence_test_requests(sequence_id,created_at DESC,request_id);
CREATE UNIQUE INDEX crm_sequence_test_receipt ON email_logs(template_alias,(model->>'request_id'),to_email)
 WHERE template_type='sequence_test' AND model->>'request_id' IS NOT NULL;
