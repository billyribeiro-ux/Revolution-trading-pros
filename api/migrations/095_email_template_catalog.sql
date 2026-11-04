-- Catalog metadata belongs to the same stored content used by authoring.
ALTER TABLE email_templates ADD COLUMN catalog_metadata JSONB NOT NULL DEFAULT '{}'::jsonb
    CHECK(jsonb_typeof(catalog_metadata)='object');

-- Count observed acceptances by immutable identity, not a reused/renamed slug.
-- Older receipts remain unassigned rather than guessing their template identity.
ALTER TABLE email_logs ADD COLUMN email_template_id BIGINT REFERENCES email_templates(id) ON DELETE SET NULL;
CREATE INDEX email_logs_template_accepted ON email_logs(email_template_id) WHERE status='sent';
