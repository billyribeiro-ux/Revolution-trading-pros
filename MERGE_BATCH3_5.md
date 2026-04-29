# Merge — Batch 3.5: coupon-billing-fix → main

**Branch:** `coupon-billing-fix` → `main` (no-ff merge)
**Date:** 2026-04-28
**Predecessor:** Batch 3 (`task7-verification-gaps`) which uncovered the defect.

---

## What this batch ships

The coupon billing defect Batch 3 documented in TASK7_RESULT.md (scenario
N → FAIL) is fixed end-to-end. Customers redeeming server-applied coupons
are now charged the discounted amount they were promised, and our DB
records match Stripe to the cent.

Detailed forensic + verification record: see `BATCH3_5_RESULT.md`.

---

## Files changed

```
 api/migrations/062_coupons_stripe_mirror.sql       (new — schema)
 api/src/services/stripe.rs                          (Stripe Coupon API + DiscountSpec + session expand)
 api/src/routes/payments.rs                          (server-applied path + webhook reconciliation)
 api/src/routes/checkout.rs                          (server-applied path)
 api/src/routes/admin.rs                             (admin CRUD mirrors to Stripe)
 BATCH3_5_RESULT.md                                  (new — full evidence)
 MERGE_BATCH3_5.md                                   (new — this file)
```

---

## Verifications (all PASS)

| | Verdict | One-liner |
|---|---|---|
| **A** one-time product, 10% off | ✅ | $147 → Stripe charges 13230¢, was 14700¢ pre-fix |
| **B** sub `duration=once` | ✅ | Period 1 discounted, Period 2 full price (Test Clock) |
| **C** sub `duration=forever` | ✅ | Every period discounted (Test Clock) |
| **D** sub `duration=repeating` n=3 | ✅ | Periods 1–3 discounted, Period 4 full price (Test Clock) |
| **E** Stripe-side promo path | ✅ | No coupon_code → `allow_promotion_codes: true`, `discounts: []` |
| **F** admin delete coupon | ✅ | Stripe → 404, redeem → 400 "Invalid coupon code" |

---

## Gates

- `cargo check`: ✅ 0 errors
- `cargo clippy --no-deps`: ✅ 0 errors / 0 warnings
- `pnpm check`: ✅ 5217 files, 0 errors / 0 warnings

---

## Migration to apply on merge

Migration 062 must run on every environment before serving traffic:

```sql
api/migrations/062_coupons_stripe_mirror.sql
```

Local dev DB already has it applied. Pre-existing test data: only one
coupon in local dev had `stripe_coupon_id IS NULL` after migration; it
was created/deleted as part of verification. Production has no live
coupon rows yet (test mode only).

---

## Out of scope (deferred)

- Admin "edit coupon" should create a new Stripe coupon and flip the
  pointer (Stripe coupons are immutable). Currently `update_coupon` only
  updates DB metadata.
- Frontend admin form: needs `duration` + `duration_in_months` fields.

---

## Stop point

Per directive: stop here, wait for go before Batch 4.
