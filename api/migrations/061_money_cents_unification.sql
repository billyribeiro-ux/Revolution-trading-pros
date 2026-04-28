-- Migration 061: Unify money columns on integer cents (architecture standard §1.2).
--
-- Two purposes, both safe and additive in semantics:
--
--   B3. Drop legacy NUMERIC `indicators.price` (and `indicators_enhanced.price`
--       if present). The canonical column is `price_cents` BIGINT, which all
--       Rust call-sites read and write after this batch.
--
--   C1. Widen INTEGER cents columns to BIGINT so they can represent any
--       Stripe-supported amount without overflow:
--         - courses_enhanced.price_cents
--         - indicators.sale_price_cents
--         - indicators_enhanced.price_cents
--
-- Pre-flight on local dev (2026-04-28): values in `indicators.price` and
-- `indicators.price_cents` agree (e.g. 79.00 / 7900); courses_enhanced and
-- indicators_enhanced are empty. No data loss possible.

BEGIN;

-- ── B3. Drop legacy NUMERIC `price` columns ──────────────────────────────────

ALTER TABLE indicators DROP COLUMN IF EXISTS price;

ALTER TABLE indicators_enhanced DROP COLUMN IF EXISTS price;

-- ── C1. Widen INTEGER cents → BIGINT ─────────────────────────────────────────

ALTER TABLE courses_enhanced
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

ALTER TABLE indicators
    ALTER COLUMN sale_price_cents TYPE BIGINT
    USING sale_price_cents::BIGINT;

ALTER TABLE indicators_enhanced
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

COMMIT;
