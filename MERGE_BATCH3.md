# Batch 3 — Task 7 Verification Gap Closure

**Branch:** `task7-verification-gaps`
**Off main:** Yes (clean off main, no prior batches stacked)
**Date:** 2026-04-28
**Author:** Claude (Opus 4.7)
**Scope:** Verification only — no code changes.

---

## Goal

Close the two verification gaps left open in Task 7's report:

- **E1.** Coupon redemption against real Stripe end-to-end. (Original
  scenario N was marked "PASS (API-level)"; that verdict only verified the
  no-coupon promo path, never the server-applied-coupon path.)
- **E2.** Period-end renewal driven by Stripe Test Clocks. (Original
  scenario D used immediate `DELETE` instead of advancing time, so the
  renewal pathway via `invoice.paid` + `customer.subscription.updated`
  was never observed.)

---

## Outcomes

| Test | Verdict | Notes |
|------|---------|-------|
| **E1** Coupon redemption E2E | **FAIL** | Real defect found: Stripe charges full price while DB records the discount. ~10% revenue mismatch on every coupon redemption. Fix scoped to Batch 3.5. |
| **E2** Period-end via Test Clock | **PASS** | Stripe rolled the period from `2026-05-28T23:45:53Z` to `2026-06-28T23:45:53Z` (+30 days); our `handle_subscription_updated` matched Stripe to the second. |

Updated tally vs. original Task 7 report:
- N: PASS (API-level) → **FAIL** (corrected based on E1 evidence).
- E2 (new row): **PASS**.

Total: **24 PASS, 1 FAIL (N), 1 INCONCLUSIVE (I), 1 CONCERN (Y — addressed
on `main` by Batch 1+2 cents refactor).**

---

## E1 — coupon redemption E2E

**Setup.** Created a 10%-off `percent` coupon via `POST /api/admin/coupons`
authenticated as super_admin. Picked the $47 / month Weekly Watchlist
plan. Called `POST /api/payments/checkout` with `coupon_code` set, then
fetched the Stripe Checkout Session via Stripe API to inspect what would
actually be charged.

**Raw output:**

```
LOGIN_OK len=168
COUPON_CREATED id=2 code=E1TEST349106 discount_value_cents=1000
PLAN id=7 name=Weekly Watchlist price_cents=4700
CHECKOUT session_id=cs_test_a1NrZcM48A0PlvHyyPqdAv2UQYCEwLPc0iSN0a58plBuWZZ3iTi7rIlp8o order_id=19
STRIPE session amount_subtotal=4700 amount_total=4700
  line_item: amount_total=4700 amount_subtotal=4700 qty=1 name=Weekly Watchlist
STRIPE total_details={"amount_discount":0,"amount_shipping":0,"amount_tax":0}
DB orders row: 47.00|4.70|42.30|E1TEST349106
---
EXPECTED: subtotal=4700 discount=470 total=4230
DB:       subtotal=4700 discount=470 total=4230
STRIPE:   amount_total=4700 amount_subtotal=4700
DB_MATCH:        PASS
STRIPE_MATCH_DB: FAIL — Stripe charges full price; DB discount is decorative
CLEANUP coupon deleted
```

**Defect.** Source-of-truth divergence between Stripe and our DB:

- DB orders row: subtotal=$47.00, discount=$4.70, total=$42.30 ✅
- Stripe Checkout Session: `amount_total=$47.00`, `amount_discount=$0` ❌

The customer's card is charged full price ($47.00) by Stripe, while our
own records show them as having received the 10% discount. On every
coupon redemption, customer-paid vs DB-recorded diverges by the discount
amount.

**Root cause** in `payments.rs`:

- Lines 175–185 build the Stripe `LineItem` with `amount = db_price_cents`
  (full DB price), regardless of any coupon.
- Lines 198–248 compute the discount in cents and apply it only to the
  `orders.total` field (lines 261–287).
- Line 337 sets `allow_promotion_codes: input.coupon_code.is_none()`, so
  when our admin coupon is used, Stripe is also told NOT to prompt for a
  promo code. The customer has no path to apply the discount on Stripe's
  side either.

**Disposition.** Fix is out-of-scope for the verification batch and will
land in **Batch 3.5** (`coupon-billing-fix` branch) per architecture
standard §10. The fix will use Stripe's Coupon API and attach `discounts[]`
to the Checkout Session so Stripe is the source of truth for discount
math, and our DB stays consistent by reading back from
`session.total_details` and `invoice` events.

---

## E2 — Stripe Test Clock period-end renewal

**Setup.** Started `stripe listen --forward-to http://localhost:8080/api/payments/webhook`
in the background (signing secret already matched `STRIPE_WEBHOOK_SECRET`).
Built a Test Clock at `T0` (now), pinned a Customer to it with
`pm_card_visa` as default, created a real Subscription on plan 1's
`stripe_price_id` with `metadata.user_id=16, plan_id=1`. Attached the new
`stripe_subscription_id` to the existing `user_memberships` row for
(user_id=16, plan_id=1) so the webhook handler's `WHERE
stripe_subscription_id = $1` matches.

**Raw output:**

```
CLOCK clock_1TRLKA9HsGkDuN3bt431vHug frozen_at=2026-04-28T23:45:53.000Z
CUSTOMER cus_UQBh4G4SJPMYE3 PM pm_1TRLKA9HsGkDuN3bqlciRkeb
SUB sub_1TRLKB9HsGkDuN3bN3al0eiR status=active first_period_end=2026-05-28T23:45:53.000Z
DB seeded: 5|active|2026-05-28 23:45:53
ADVANCE clock to 2026-05-29T00:45:53.000Z
CLOCK ready iter=7
DB after advance: 5|active|2026-06-28 23:45:53
---
initial DB period_end: 2026-05-28 23:45:53
new DB period_end:     2026-06-28 23:45:53
Stripe period_end:     2026-06-28T23:45:53.000Z
PERIOD_ROLLED_FORWARD: PASS
DB_MATCHES_STRIPE:     PASS
CLEANUP done
```

**Webhook events delivered during the advance** (from `stripe listen`):

```
invoice.created                  → 200
customer.subscription.updated    → 200   ← rolls period in DB
invoice_payment.paid             → 200
invoice.updated                  → 200
invoice.finalized                → 200
invoice.paid                     → 200
invoice.payment_succeeded        → 200
```

**Conclusion.** The renewal pathway works correctly. When Stripe issues
the next-period invoice and emits `customer.subscription.updated` with
the new `current_period_end`, `handle_subscription_updated`
(`payments.rs:1116`) writes the new period to the DB and the values match
Stripe to the second.

---

## Files touched

```
TASK7_RESULT.md         (modified — N corrected to FAIL; addendum appended)
MERGE_BATCH3.md         (new — this file)
```

No code changes. No migrations.

---

## Gate evidence

```
$ cargo check --manifest-path api/Cargo.toml --offline    ✅ 0 errors
$ cargo clippy --manifest-path api/Cargo.toml --no-deps   ✅ 0 errors / 0 warnings
$ pnpm check (frontend)                                   ✅ 0 errors / 0 warnings
```

(No code changed in this branch; gates run as a sanity check that nothing
was disturbed.)

---

## Out of scope (deferred)

- **Batch 3.5** (`coupon-billing-fix` off main) — implement the Stripe
  Coupon API integration so server-applied coupons charge the discounted
  amount. Will re-run E1 to PASS when complete. Subscription duration
  variants to verify: `once` (first month only), `forever` (every month),
  `repeating` with `duration_in_months=N`.
- **Batch 4+** — re-subscribe row strategy, refactors D1–D4, Postmark
  wiring (Task 4), scenario I (course refund cycle).

---

## Ready to merge

All gates green. Verification report is internally consistent with
addendum + summary table both showing N as FAIL with cross-reference. No
code changed; merge is a docs-only commit.
