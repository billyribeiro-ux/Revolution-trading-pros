-- Migration 043: Membership plan price-change history
-- ═══════════════════════════════════════════════════════════════════════════
-- Records every price change for a membership plan so admins can:
--   1. Audit who changed what, when, and which Stripe Price IDs were involved.
--   2. Roll back to a prior Stripe Price ID if a change was made in error.
--   3. Reason about which existing subscriptions were migrated to the new
--      price (apply_to: new_only | next_renewal | immediate_proration).
--
-- Each price change creates a NEW Stripe Price (Stripe prices are immutable);
-- we keep the old `stripe_price_id` here for rollback.
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS membership_plan_price_history (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES membership_plans(id) ON DELETE CASCADE,

    -- Old → new Stripe Price IDs
    old_stripe_price_id TEXT,
    new_stripe_price_id TEXT NOT NULL,

    -- Old → new pricing snapshot (cents). Storing both avoids ambiguity if
    -- membership_plans.price drifts later.
    old_amount_cents BIGINT,
    new_amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    billing_interval TEXT NOT NULL,  -- "month" | "year" | "one_time"

    -- How the change was rolled out:
    --   "new_only"            → existing subs grandfathered
    --   "next_renewal"        → existing subs migrate at next cycle, no proration
    --   "immediate_proration" → existing subs migrate now, prorated
    apply_to TEXT NOT NULL,

    -- Bulk-update accounting
    subscriptions_migrated INTEGER NOT NULL DEFAULT 0,
    subscriptions_failed INTEGER NOT NULL DEFAULT 0,
    failure_details JSONB,

    -- Who triggered the change (admin user)
    changed_by_user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT membership_plan_price_history_apply_to_check
        CHECK (apply_to IN ('new_only', 'next_renewal', 'immediate_proration')),
    CONSTRAINT membership_plan_price_history_billing_interval_check
        CHECK (billing_interval IN ('month', 'year', 'one_time'))
);

CREATE INDEX IF NOT EXISTS idx_mpph_plan_id
    ON membership_plan_price_history (plan_id);

CREATE INDEX IF NOT EXISTS idx_mpph_changed_at
    ON membership_plan_price_history (changed_at DESC);

CREATE INDEX IF NOT EXISTS idx_mpph_new_price
    ON membership_plan_price_history (new_stripe_price_id);

COMMENT ON TABLE membership_plan_price_history IS
    'Append-only history of every Stripe Price change on a membership plan.';
COMMENT ON COLUMN membership_plan_price_history.apply_to IS
    'Rollout strategy: new_only|next_renewal|immediate_proration';
