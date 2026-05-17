-- 068_complete_money_cents_bigint.sql
--
-- FIX (audit 2026-05-17): complete the money-cents → BIGINT unification
-- that migration 061 began but left two LIVE columns behind.
--
-- BACKGROUND
--   CLAUDE.md hard rule: every `*_cents` value is i64 / BIGINT end-to-end,
--   never INTEGER. INTEGER cents caps at $21,474,836.47 — fine per row,
--   but the rule exists because the cap is hit on rollups / MRR / LTV
--   aggregates and on Stripe's int64 wire format.
--
--   Migration 061 widened: courses_enhanced.price_cents,
--   indicators.sale_price_cents, indicators_enhanced.price_cents.
--   It did NOT widen:
--     - courses.price_cents          (001_initial §126: INTEGER NOT NULL)
--     - indicators.price_cents       (015 §1074: INTEGER DEFAULT 0)
--   061 even references `indicators.price_cents` in its pre-flight note
--   yet only widened `sale_price_cents` — a gap in that prior fix, not a
--   deliberate exclusion.
--
--   NOT touched here: products.price_cents. The products read/write path
--   uses a NUMERIC `price` column ((price*100)::BIGINT on read,
--   $n::BIGINT/100.0 on write); its INTEGER `price_cents` appears
--   vestigial/derived. Widening or dropping it is a separate decision —
--   flagged in docs/audits, not bundled into a money-correctness fix.
--
-- SAFETY
--   INTEGER → BIGINT is a non-lossy widening. `USING price_cents::BIGINT`
--   is the exact pattern migration 061 used. Idempotent: re-running when
--   the column is already BIGINT is a no-op cast. Forward-only; does not
--   edit any shipped migration.
--
--   ⚠️ APPLY DECISION IS THE OPERATOR'S: this is a schema change on a
--   fintech production DB. Confirm the live column types first
--   (`\d courses` / `\d indicators` in psql) — if 061 or a later batch
--   already widened them, this is a harmless no-op; if not, this closes
--   a real i32-overflow exposure on course/indicator pricing.

BEGIN;

-- courses.price_cents : INTEGER NOT NULL DEFAULT 0  ->  BIGINT
ALTER TABLE courses
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

-- indicators.price_cents : INTEGER DEFAULT 0  ->  BIGINT
ALTER TABLE indicators
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

COMMIT;
