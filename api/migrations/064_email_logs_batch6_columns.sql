-- Batch 6 — Postmark email integration.
--
-- Adds the spec-shaped columns to `email_logs` (introduced in 001) so
-- the new `EmailService::send_transactional` API can write a row
-- before every send attempt. Existing columns (`email`, `template_type`,
-- `metadata`, `error_message`) are LEFT IN PLACE — older newsletter
-- code still references them via the `EmailLog` model in
-- `api/src/models/newsletter.rs`. The new columns sit alongside.
--
-- New columns (all nullable so existing inserts keep working):
--   * to_email TEXT             — destination address (mirrors `email`)
--   * template_alias TEXT       — Postmark template alias (mirrors `template_type`)
--   * model JSONB               — Postmark model dict (mirrors `metadata`)
--   * error TEXT                — error string (mirrors `error_message`)
--   * queued_at TIMESTAMPTZ     — when the row was created (always set)
--   * sent_at TIMESTAMPTZ       — when Postmark accepted the send
--
-- New status values introduced by this migration:
--   * 'queued'              — row written, send not yet attempted
--   * 'skipped_no_token'    — POSTMARK_TOKEN unset; no HTTP call made
--   * 'sent'                — Postmark 200, MessageID stored
--   * 'failed'              — Postmark non-200 or transport error
--
-- Backfill: existing rows get queued_at/sent_at = created_at and
-- to_email/template_alias/model/error mirrored from the legacy columns
-- so admin diagnostic queries are uniform across old + new rows.

ALTER TABLE email_logs
    ADD COLUMN IF NOT EXISTS to_email VARCHAR(255),
    ADD COLUMN IF NOT EXISTS template_alias VARCHAR(100),
    ADD COLUMN IF NOT EXISTS model JSONB,
    ADD COLUMN IF NOT EXISTS error TEXT,
    ADD COLUMN IF NOT EXISTS queued_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS sent_at TIMESTAMPTZ;

UPDATE email_logs
SET
    to_email       = COALESCE(to_email, email),
    template_alias = COALESCE(template_alias, template_type),
    model          = COALESCE(model, metadata),
    error          = COALESCE(error, error_message),
    queued_at      = COALESCE(queued_at, created_at),
    sent_at        = CASE
                       WHEN sent_at IS NOT NULL THEN sent_at
                       WHEN status = 'sent' THEN created_at
                       ELSE NULL
                     END
WHERE
    to_email IS NULL
    OR template_alias IS NULL
    OR queued_at IS NULL;

-- Index for the admin /email/status diagnostic ("last 24h sent/failed/skipped").
CREATE INDEX IF NOT EXISTS idx_email_logs_status_queued_at
    ON email_logs(status, queued_at DESC);

-- Index for the admin /email/logs list view.
CREATE INDEX IF NOT EXISTS idx_email_logs_queued_at_desc
    ON email_logs(queued_at DESC);
