-- 070_user_memberships_one_live_per_user.sql
--
-- FIX (RUST_DEEP_AUDIT_2026-06-07, P1-2): enforce the invariant that
-- `create_subscription` already assumes but never enforced at the DB level.
--
-- BACKGROUND
--   routes/subscriptions/lifecycle.rs::create_subscription rejects a new
--   subscription when the user already has any row with status
--   IN ('active','trialing') — i.e. the product rule is "one LIVE
--   subscription per user, across all plans" ("Use upgrade/downgrade
--   endpoint instead.").
--
--   That guard is a non-transactional check-then-act with no backing unique
--   constraint, so two concurrent requests (or a double-click) can both pass
--   the SELECT and proceed — for Stripe plans both create checkout sessions,
--   for free/trial plans both INSERT — leaving the user with two live
--   memberships on different plans. Migration 063's index only covers
--   (user_id, plan_id), so it stops same-plan dupes but NOT this cross-plan
--   race.
--
-- APPROACH (de-dupe, then index — chosen explicitly for this rollout)
--   Step 1 demotes any pre-existing duplicate live rows so the unique index
--   can be built; Step 2 creates a partial unique index on (user_id) over the
--   same live-status set the handler checks, making the check-then-act
--   race-safe (the second concurrent writer now hits a unique violation).
--
--   The index predicate intentionally matches the handler's SELECT set
--   ('active','trialing'). NOTE: the free-trial INSERT path writes status
--   'trial' (singular), which is outside this set and outside the handler's
--   check — that pre-existing status-naming inconsistency is documented in
--   the audit and deliberately NOT changed here (it would need a broader
--   status-normalization migration + handler change).
--
-- SAFETY
--   Step 1 is conservative: it keeps the most recently created live row per
--   user (tie-broken by id) and demotes only the strictly-older live rows to
--   'cancelled', stamping cancelled_at/updated_at. It is a no-op once the data
--   already satisfies the invariant, so re-running is safe.

-- Step 1 — de-duplicate existing live memberships per user.
WITH ranked AS (
    SELECT
        id,
        row_number() OVER (
            PARTITION BY user_id
            ORDER BY created_at DESC, id DESC
        ) AS rn
    FROM user_memberships
    WHERE status IN ('active', 'trialing')
)
UPDATE user_memberships AS m
SET
    status = 'cancelled',
    cancelled_at = COALESCE(m.cancelled_at, NOW()),
    updated_at = NOW()
FROM ranked
WHERE m.id = ranked.id
  AND ranked.rn > 1;

-- Step 2 — enforce one live subscription per user going forward.
CREATE UNIQUE INDEX IF NOT EXISTS uq_user_memberships_one_live_per_user
    ON user_memberships (user_id)
    WHERE status IN ('active', 'trialing');
