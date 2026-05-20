-- 069_widen_money_cents_to_bigint.sql
--
-- FIX (audit R17-D, 2026-05-20): close out the last CLAUDE.md "money is
-- i64 / BIGINT end-to-end" gaps that prior migrations missed or that
-- only manifested in Rust types.
--
-- BACKGROUND
--   CLAUDE.md hard rule: every `*_cents` value is i64 / BIGINT end-to-end,
--   never INTEGER. The cap on i32 / PG INTEGER is $21,474,836.47 — fine
--   per row, but the rule exists because the cap is hit on rollups (MRR,
--   LTV, revenue aggregates) and on Stripe's int64 wire format.
--
--   Audits R12-D, R13-B, R15-D each flagged a money-path violation that
--   was test-only or structural (split commit) and therefore did NOT
--   include the schema/type fix. This migration is the consolidated
--   schema fix; the matching Rust changes are in the same PR.
--
-- VIOLATIONS BEING CLOSED OUT
--   1. `routes/admin_courses/analytics_pricing.rs::change_course_price`
--      (R13-B): bound `input.amount_cents as i32` against
--      `courses.price_cents`. The DB column was widened to BIGINT in
--      migration 068, but the Rust `as i32` cast remained — a silent
--      truncation on writes above $21M.
--      Fix: Rust now binds the raw i64; column already BIGINT.
--
--   2. `models/indicator_enhanced.rs::IndicatorEnhanced.price_cents`
--      (R15-D): struct field was `Option<i32>` against
--      `indicators_enhanced.price_cents` BIGINT (widened in migration
--      061). Silent narrowing on read for values above i32::MAX.
--      Fix: Rust struct widened to Option<i64>; column already BIGINT.
--      Same applies to `CreateIndicatorRequest`, `UpdateIndicatorRequest`,
--      and `IndicatorResponse` wire DTOs — widened in the matching commit.
--
--   3. `models/course.rs::UserCourseEnrollment.price_paid_cents` and
--      `models/indicator.rs::UserIndicatorOwnership.price_paid_cents`
--      (R12-D): struct fields are `Option<i32>`, but the matching
--      columns do NOT exist in the current schema — they appear to be
--      legacy struct fields kept for compile compatibility on queries
--      that never reference them. Verified via:
--        grep -rEn "price_paid_cents" api/migrations/   # zero hits
--        \d user_course_enrollments / \d user_indicator_access in psql
--      No schema change here for those two — only the Rust types were
--      widened to keep the type contract aligned with the rule for the
--      day a `price_paid_cents` column is added.
--
--   4. `services/search.rs::SearchableCourse.price_cents` (cleanup):
--      Meilisearch payload struct was `i32`; the source column
--      `courses.price_cents` is BIGINT (since migration 068). Widened
--      in the matching commit.
--
-- WHAT THIS MIGRATION ACTUALLY DOES
--   - Defensive, idempotent re-widening of the two `*_cents` columns
--     most likely to drift in a future patch: `courses.price_cents` and
--     `indicators.price_cents`. Migration 068 already widened both, but
--     re-asserting here protects against a hand-edit / branch-merge that
--     could revert them. INTEGER -> BIGINT is non-lossy; BIGINT -> BIGINT
--     is a no-op cast.
--
--   - Does NOT add `user_course_enrollments.price_paid_cents` or
--     `user_indicator_access.price_paid_cents`. Adding a column to a
--     production table for type alignment alone is the wrong call;
--     when those columns are actually written, the migration that adds
--     them MUST declare BIGINT, per the rule in this file's header.
--
-- SAFETY
--   INTEGER -> BIGINT is a non-lossy widening. `USING price_cents::BIGINT`
--   matches the pattern from migrations 061 and 068. The ALTER on an
--   already-BIGINT column is a no-op cast inside the same transaction.
--
-- ROLLBACK
--   Reverting to INTEGER would break if any row exceeds i32::MAX cents
--   ($21,474,836.47). Pre-flight that with:
--     SELECT id, price_cents FROM courses
--       WHERE price_cents > 2147483647;
--     SELECT id, price_cents FROM indicators
--       WHERE price_cents > 2147483647;
--   If either returns a row, do NOT revert — the rule exists for that
--   exact reason. There is no .down for this migration; widening is
--   forward-only per CLAUDE.md ("forward-only migration").

BEGIN;

-- courses.price_cents : BIGINT -> BIGINT (re-assert, no-op when already widened)
ALTER TABLE courses
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

-- indicators.price_cents : BIGINT -> BIGINT (re-assert, no-op when already widened)
ALTER TABLE indicators
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

COMMIT;
