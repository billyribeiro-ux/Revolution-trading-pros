-- Migration 055: Webhook idempotency + payment disputes
-- Security fix: prevent duplicate webhook processing and track chargebacks

CREATE TABLE IF NOT EXISTS webhook_events (
    event_id        TEXT PRIMARY KEY,
    event_type      TEXT NOT NULL,
    received_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at    TIMESTAMPTZ,
    error           TEXT,
    payload         JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_webhook_events_unprocessed
    ON webhook_events(received_at)
    WHERE processed_at IS NULL;

CREATE TABLE IF NOT EXISTS payment_disputes (
    id                  BIGSERIAL PRIMARY KEY,
    stripe_dispute_id   TEXT UNIQUE NOT NULL,
    stripe_charge_id    TEXT NOT NULL,
    reason              TEXT,
    status              TEXT NOT NULL,
    amount_cents        BIGINT NOT NULL,
    currency            TEXT NOT NULL DEFAULT 'usd',
    response_deadline   TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_payment_disputes_charge
    ON payment_disputes(stripe_charge_id);
