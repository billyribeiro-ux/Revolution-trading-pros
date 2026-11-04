-- Preserve existing hosted aliases until an operator explicitly saves content.
-- Newly authored local templates and content edits become authoritative.
ALTER TABLE email_templates ADD COLUMN local_content_authoritative BOOLEAN NOT NULL DEFAULT false;

-- Deletion/renaming must not silently reactivate a provider-hosted alias.
-- Recreating the alias explicitly clears its barrier in the same audited write.
CREATE TABLE email_template_revocations (
    alias TEXT PRIMARY KEY,
    template_id BIGINT,
    revoked_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    revoked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
