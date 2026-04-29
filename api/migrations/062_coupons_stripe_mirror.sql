-- Migration 062: mirror DB coupons to Stripe Coupons.
--
-- Per Batch 3.5 directive: every DB coupon row gets a corresponding
-- immutable Stripe Coupon. The coupon admin UI creates the Stripe coupon
-- when the DB row is created, stores its id here, and at checkout time the
-- backend attaches it to the Stripe Checkout Session via
-- `discounts[0][coupon]=<stripe_coupon_id>`. Stripe is the source of truth
-- for the actual discount math.
--
-- Adds:
--   - stripe_coupon_id   TEXT      — populated by admin create_coupon flow
--   - duration           TEXT      — once | forever | repeating
--   - duration_in_months INT       — required when duration='repeating'
--
-- Existing rows get duration='once' as the safe default (single-period
-- discount). Operators can edit existing coupons after the migration if
-- they want forever/repeating semantics; editing creates a new Stripe
-- coupon (Stripe coupons are immutable) and updates stripe_coupon_id.

BEGIN;

ALTER TABLE coupons
    ADD COLUMN IF NOT EXISTS stripe_coupon_id   TEXT,
    ADD COLUMN IF NOT EXISTS duration           TEXT NOT NULL DEFAULT 'once',
    ADD COLUMN IF NOT EXISTS duration_in_months INTEGER;

ALTER TABLE coupons
    ADD CONSTRAINT coupons_duration_valid
    CHECK (duration IN ('once', 'forever', 'repeating'));

ALTER TABLE coupons
    ADD CONSTRAINT coupons_duration_in_months_required_for_repeating
    CHECK (
        (duration = 'repeating' AND duration_in_months IS NOT NULL AND duration_in_months > 0)
        OR (duration <> 'repeating' AND duration_in_months IS NULL)
    );

CREATE INDEX IF NOT EXISTS idx_coupons_stripe_coupon_id
    ON coupons(stripe_coupon_id)
    WHERE stripe_coupon_id IS NOT NULL;

COMMIT;
