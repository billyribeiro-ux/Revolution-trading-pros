# Backlog

Forward-looking work captured as it surfaces during shipped batches.
Items here are intentionally not yet scoped to a specific branch — when
a batch picks them up, it pulls them out of this file into a
`MERGE_BATCH<n>.md` plan and removes the entry once shipped.

---

## Batch 4 candidates

### 1. Admin edit-coupon Stripe-coupon recreation flow
**Severity:** P1 (operators can silently desync coupons)

When admin edits discount math (percent_off, amount_off, duration),
backend creates a new Stripe coupon, flips `stripe_coupon_id` pointer on
the DB row, optionally deletes the old Stripe coupon. (Stripe Coupons
are immutable; existing subscriptions keep the discount they redeemed
per Stripe's `duration` semantics, which is the desired behavior.) The
current `update_coupon` only updates DB metadata, so an operator
changing 10% → 15% in our admin UI silently leaves the Stripe coupon at
10% and customers continue redeeming at 10%.

### 2. Coupon backfill admin tool
**Severity:** P2 (currently no rows affected; future-proof)

Add admin endpoint `POST /api/admin/coupons/:id/sync-to-stripe` that
mirrors a DB row whose `stripe_coupon_id IS NULL` into a fresh Stripe
Coupon and stores the id. Useful if a Stripe API outage during create
left a row half-mirrored, or if rows ever get imported from elsewhere.

### 3. Frontend admin coupon form fields for `duration` / `duration_in_months`
**Severity:** P1 (form is now incomplete vs. backend contract)

Add a `duration` dropdown (once / forever / repeating) and a
`duration_in_months` input (visible only when `duration=repeating`) to
the `/admin/coupons` create + edit pages. The backend now accepts and
requires these fields per Migration 062 + Batch 3.5; the frontend form
hasn't caught up yet.
