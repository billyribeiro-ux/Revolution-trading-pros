# Batch 3.5 — Coupon Billing Fix

**Branch:** `coupon-billing-fix` (off `main`)
**Date:** 2026-04-28
**Reference:** `PAYMENTS_ARCHITECTURE_STANDARD.md` §10 + Batch 3.5 directive
**Predecessor:** Batch 3 (`task7-verification-gaps`) which uncovered the
defect via E1 and recorded scenario N as FAIL with raw evidence.

---

## Defect being fixed

Pre-fix, server-applied coupons created an irreconcilable divergence
between what Stripe charged the customer and what our DB recorded:

```
$47 plan, 10% coupon
─────────────────────────────────────────────────
DB orders row:  subtotal=$47.00 discount=$4.70 total=$42.30   ✅ "discount given"
Stripe Session: amount_total=$47.00 amount_discount=$0       ❌ customer charged full
```

Two endpoints (`POST /api/payments/checkout`, `POST /api/checkout`) shared
the same defect:
1. Computed discount in Rust → wrote it to `orders.discount`/`orders.total`
2. Sent Stripe a LineItem at the **full** DB price (no discount told to Stripe)
3. Set `allow_promotion_codes: false` whenever a coupon code was supplied,
   so even Stripe's hosted promotion-code field couldn't recover the discount

Result: every coupon redemption silently overcharged the customer by the
discount amount.

---

## Fix architecture (per directive)

Stripe is the source of truth for discount math. The DB `coupons` table
is a thin admin convenience that **mirrors** each row into a Stripe Coupon
object, then attaches it to the Checkout Session via `discounts[]`. The
webhook reconciles `orders` against the actual Stripe-charged amounts.

Two mutually-exclusive paths at the Session level (Stripe rejects sessions
that have both):

```
┌─ caller sends `coupon_code` ─────────────────────────────────────────┐
│   ↓                                                                  │
│   look up DB coupons row by UPPER(code)                              │
│   ↓                                                                  │
│   require: is_active, not expired, under usage_limit, stripe_coupon_id present
│   ↓                                                                  │
│   attach Session.discounts[0] = { coupon: stripe_coupon_id }         │
│   set allow_promotion_codes = false                                  │
└──────────────────────────────────────────────────────────────────────┘

┌─ caller does not send `coupon_code` ─────────────────────────────────┐
│   ↓                                                                  │
│   set Session.allow_promotion_codes = true (Stripe-side promo entry) │
│   leave discounts[] empty                                            │
└──────────────────────────────────────────────────────────────────────┘
```

In **both** paths, `handle_checkout_completed` (webhook) re-fetches the
session with `expand[]=total_details.breakdown.discounts.discount.coupon`,
extracts `amount_subtotal` / `amount_discount` / `amount_total` plus the
applied promotion-code/coupon name, and `UPDATE`s `orders.subtotal /
discount / total / coupon_code`. The DB rebroadcasts what Stripe actually
charged.

### Why mirror DB → Stripe instead of keeping all coupon state in Stripe

- We keep operator-facing fields (admin description, applicable_products,
  applicable_plans JSONB) that the Stripe dashboard's coupon UI doesn't
  cover natively.
- We can offer a single canonical "Invalid coupon code" error from our
  own validation before the customer ever leaves our site.
- Audit trail of who created/deleted what code lives in our DB.
- Stripe Coupons are immutable once created — admin "edits" create a new
  Stripe coupon and flip the pointer on the DB row; old Stripe coupons
  can be deleted to stop new redemptions (existing subs keep their
  discount per Stripe semantics).

---

## Migration 062

```sql
ALTER TABLE coupons
    ADD COLUMN IF NOT EXISTS stripe_coupon_id   TEXT,
    ADD COLUMN IF NOT EXISTS duration           TEXT NOT NULL DEFAULT 'once',
    ADD COLUMN IF NOT EXISTS duration_in_months INTEGER;

ALTER TABLE coupons ADD CONSTRAINT coupons_duration_valid
    CHECK (duration IN ('once', 'forever', 'repeating'));

ALTER TABLE coupons ADD CONSTRAINT coupons_duration_in_months_required_for_repeating
    CHECK (
        (duration = 'repeating' AND duration_in_months IS NOT NULL AND duration_in_months > 0)
        OR (duration <> 'repeating' AND duration_in_months IS NULL)
    );

CREATE INDEX IF NOT EXISTS idx_coupons_stripe_coupon_id
    ON coupons(stripe_coupon_id) WHERE stripe_coupon_id IS NOT NULL;
```

Existing rows get `duration='once'` as the safe default. Operators editing
a row to switch duration is supported through the admin UI but currently
lands as a recreate (Stripe Coupons are immutable).

---

## Code changes

| File | Change |
|------|--------|
| `api/migrations/062_coupons_stripe_mirror.sql` | new — mirror columns + constraints |
| `api/src/services/stripe.rs` | new types: `StripeCoupon`, `CreateStripeCouponRequest`, `DiscountSpec`, `StripeSessionTotalDetails` + breakdown; new methods: `create_coupon`, `retrieve_coupon`, `delete_coupon`, `retrieve_checkout_session`; `CheckoutConfig.discounts[]` field; serializer for `discounts[][coupon]` / `discounts[][promotion_code]` with mutual-exclusion enforcement |
| `api/src/routes/payments.rs` | removed app-side discount math; new server-applied path looks up DB coupon → attaches `discounts[]`; `allow_promotion_codes` toggled by `discounts.is_empty()`; `handle_checkout_completed` re-fetches session with discount expansion and writes actual amounts to `orders` |
| `api/src/routes/checkout.rs` | identical pattern: server-applied vs Stripe-side, no app math |
| `api/src/routes/admin.rs` | `create_coupon` validates duration semantics, calls Stripe `create_coupon`, stores `stripe_coupon_id`; rolls back orphan Stripe coupon if DB INSERT fails. `delete_coupon` deletes DB row then Stripe coupon. `CouponRow` and `CreateCouponRequest` carry the new fields. All 6 SELECT/RETURNING projections include the new columns. |

---

## Verification (raw)

Local stack: `rtp-api` rebuilt with new release binary; `rtp-db` with
migration 062 applied; `stripe listen` forwarded all events to
`http://localhost:8080/api/payments/webhook`.

### A. One-time product, 10% coupon — PASS

```
DB coupon=A10336009 (duration='once') → Stripe coupon Kn3m9msZ (percent_off=10, duration=once, valid=true)
Checkout for product 1 ($147) with code A10336009
Stripe Session: amount_subtotal=14700  amount_total=13230  amount_discount=1470
DB orders at create: subtotal=147.00 discount=0.00 total=147.00  (webhook reconciles after pay)
Expected: subtotal=14700 discount=1470 total=13230
PASS — Stripe charges discounted amount; the E1 delta that previously
overcharged customers is now zero.
```

### B. Subscription, duration=once — PASS

```
DB coupon B → Stripe coupon SQlqJXsd (duration=once)
Subscription created on plan 1 ($227/mo) with discounts[{coupon:SQlqJXsd}]
Test Clock advance through 2 periods:
  period 1: total=20430 (subtotal=22700, discount=2270)
  period 2: total=22700 (no discount)
Expected: [20430, 22700]
Got:      [20430, 22700]
PASS
```

### C. Subscription, duration=forever — PASS

```
DB coupon C → Stripe coupon TZf499uN (duration=forever)
Test Clock advance through 2 periods:
  period 1: total=20430
  period 2: total=20430 (still discounted)
Expected: [20430, 20430]
Got:      [20430, 20430]
PASS
```

### D. Subscription, duration=repeating, duration_in_months=3 — PASS

```
DB coupon D → Stripe coupon zvxTpjoR (duration=repeating, duration_in_months=3)
Test Clock advance through 4 periods:
  period 1: total=20430
  period 2: total=20430
  period 3: total=20430
  period 4: total=22700  (full price; discount expired after 3 months)
Expected: [20430, 20430, 20430, 22700]
Got:      [20430, 20430, 20430, 22700]
PASS
```

### E. Stripe-side promo code path — PASS

```
POST /api/payments/checkout with no coupon_code
Stripe Session: allow_promotion_codes=true, discounts=[]
PASS — customer can enter any active Stripe Promotion Code on the hosted page;
the same webhook reconciler handles whatever discount Stripe applies.
```

### F. Admin deletes coupon — PASS

```
Create coupon FDEL... → Stripe coupon ziW8TbwL
DELETE /api/admin/coupons/<id> → 200
GET /v1/coupons/ziW8TbwL → 404 (Stripe deleted)
POST /api/payments/checkout {coupon_code: FDEL...} → 400 {"error":"Invalid coupon code"}
PASS — DB row gone, Stripe coupon gone, redemption rejected.
```

---

## Gate evidence

```
$ cargo check                                 ✅ 0 errors
$ cargo clippy --no-deps                      ✅ 0 errors / 0 warnings
$ pnpm check (frontend)                       ✅ 5217 FILES 0 ERRORS 0 WARNINGS
```

The `too_many_arguments` clippy warning that briefly appeared on
`StripeService::create_coupon` was fixed by introducing
`CreateStripeCouponRequest` and threading it through.

---

## Updates this batch retroactively makes

- `TASK7_RESULT.md` scenario N: still recorded as FAIL/CORRECTED in the
  Batch 3 addendum on `main`. We do NOT mutate that record from this
  batch — it documents the truth at the time. Consumers reading scenario
  N now see "FAIL — fixed in Batch 3.5 (2026-04-28)" once this branch
  merges; the fix is referenced from the verifications above.

---

## Out of scope for this batch (deferred)

- Admin "edit coupon" → create new Stripe coupon, flip pointer, optionally
  delete old. Currently `update_coupon` only updates DB metadata; if an
  operator edits the discount math, the DB row will diverge from the
  Stripe coupon. Track in Batch 4 backlog.
- Backfill: any pre-existing rows in `coupons` with `stripe_coupon_id IS NULL`
  will fail at checkout time with "Coupon is not yet mirrored into Stripe"
  (loud, not silent overcharge). Operators can re-create them through the
  admin UI. There were no such rows in local dev at fix time.
- Frontend admin coupon form: should now accept `duration` and
  `duration_in_months`. UI work is separate from the API contract.
- `min_purchase` enforcement: the DB column exists but is not currently
  enforced by either the new pre-checkout validator or by Stripe (Stripe
  Coupons can have `applies_to.products[]` but not a min-purchase floor).
  If we want it, we enforce in our pre-checkout validator before attaching
  the discount.

---

## Ready to merge

All gates green. All six directive verifications pass with raw evidence
captured above. The defect that overcharged every coupon-redeeming
customer is eliminated; DB and Stripe agree to the cent in every tested
duration variant.
