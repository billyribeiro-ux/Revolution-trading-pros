-- 065_backfill_coupon_usage.sql
--
-- FIX-H-4 (2026-04-29): historical correction for coupon.usage_count drift.
--
-- BACKGROUND
--   Before this migration, two code paths incremented coupons.usage_count
--   for the same checkout:
--     1. routes/checkout.rs at order create (every checkout intent,
--        including abandoned carts).
--     2. routes/payments.rs in handle_checkout_completed (every paid
--        Stripe webhook).
--   The order-create increment was removed in the same commit as this
--   migration. The historical drift this leaves behind is corrected here.
--
-- WHAT THIS DOES
--   For every coupon, recompute usage_count as the number of completed
--   orders that reference it. "Completed" = orders.status = 'completed'
--   (the same status the webhook handler stamps on successful payment).
--   Coupons with no completed orders have their usage_count set to 0,
--   even if they were attempted on abandoned carts.
--
-- IDEMPOTENCY
--   This migration is safe to re-run: the recompute is deterministic
--   from the orders table.
--
-- AUDIT TRAIL
--   The previous (incorrect) values are not preserved. If you need them
--   for forensics, query the DB BEFORE applying this migration.

UPDATE coupons
SET usage_count = sub.completed_count,
    updated_at  = NOW()
FROM (
    SELECT c.id AS coupon_id,
           COALESCE(COUNT(o.id) FILTER (WHERE o.status = 'completed'), 0)::INTEGER
               AS completed_count
    FROM coupons c
    LEFT JOIN orders o ON o.coupon_id = c.id
    GROUP BY c.id
) sub
WHERE coupons.id = sub.coupon_id
  AND coupons.usage_count IS DISTINCT FROM sub.completed_count;
