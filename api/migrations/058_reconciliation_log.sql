-- Migration 058: Reconciliation log table
-- Records each run of the Stripe reconciliation job so admins can audit
-- what drift was found and what corrections were applied.

CREATE TABLE IF NOT EXISTS reconciliation_log (
    id          BIGSERIAL PRIMARY KEY,
    run_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    -- How many user_memberships rows were out of sync with Stripe
    discrepancies_found INTEGER NOT NULL DEFAULT 0,
    -- Per-discrepancy details: array of objects with sub_id, kind, before, after
    log         JSONB NOT NULL DEFAULT '[]'::JSONB
);

CREATE INDEX IF NOT EXISTS idx_reconciliation_log_run_at
    ON reconciliation_log (run_at DESC);

COMMENT ON TABLE reconciliation_log IS
    'Append-only record of every Stripe reconciliation job run.';
COMMENT ON COLUMN reconciliation_log.log IS
    'Array of discrepancy objects: {sub_id, kind, before, after, fixed}';
