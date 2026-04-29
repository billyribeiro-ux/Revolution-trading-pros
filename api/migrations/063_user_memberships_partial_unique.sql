-- Batch 4 — Re-subscribe history.
--
-- Migration 056 added a UNIQUE (user_id, plan_id) constraint on
-- user_memberships. That constraint forced `handle_checkout_completed`
-- to use ON CONFLICT DO UPDATE, which overwrote the prior cancelled
-- membership row when a user re-subscribed after a full cancellation —
-- destroying the historical record of the prior subscription period.
--
-- The intent of 056 was only to prevent *duplicate active* memberships
-- on the same plan. It accidentally also blocked legitimate
-- re-subscription history. This migration replaces the blunt unique
-- constraint with a partial unique index that only covers the live
-- statuses ('active', 'trialing', 'past_due'). Cancelled, expired, and
-- failed rows can coexist freely with a new active row, preserving
-- per-period history for auditing, reporting, and customer-support
-- lookups.
--
-- Behavior contract:
--   * A user can have at most ONE row per plan with a live status.
--   * A user re-subscribing after full cancellation gets a NEW row,
--     leaving the cancelled row(s) untouched.
--   * Index name `uq_user_memberships_active_plan` matches the spec.

ALTER TABLE user_memberships
    DROP CONSTRAINT IF EXISTS uq_user_memberships_user_plan;

CREATE UNIQUE INDEX IF NOT EXISTS uq_user_memberships_active_plan
    ON user_memberships(user_id, plan_id)
    WHERE status IN ('active', 'trialing', 'past_due');

-- Idempotency for Stripe webhook retries: each Stripe subscription
-- maps to at most one row. The webhook uses
-- `ON CONFLICT (stripe_subscription_id) DO UPDATE` to absorb retries
-- of the same `checkout.session.completed` event without creating
-- duplicate rows. NULLs are allowed (legacy non-Stripe rows) and
-- multiple NULLs do not collide under a partial unique index that
-- excludes them.
CREATE UNIQUE INDEX IF NOT EXISTS uq_user_memberships_stripe_subscription
    ON user_memberships(stripe_subscription_id)
    WHERE stripe_subscription_id IS NOT NULL;
