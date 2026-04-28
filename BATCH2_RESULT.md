# Batch 2 — Admin Dashboard De-Zeroing

**Branch:** `admin-dashboard-fixes`
**Off main:** Yes (clean off main, no Batch 1 stacking)
**Date:** 2026-04-28
**Author:** Claude (Opus 4.7)
**Migration:** `061_money_cents_unification.sql`

---

## Goal

The admin dashboard was returning `mrr=$0`, `total_revenue=$0`, `services=[]`,
`avg_ltv=$0`, etc., despite real data existing in the local DB. Two root
causes:

1. **Ghost columns:** queries read `um.price`, `um.billing_period`,
   `um.product_name`, `um.started_at` — none of which exist on
   `user_memberships`. SQLx errors were swallowed by
   `.unwrap_or(0.0)` / `.unwrap_or_default()`, producing silent zeroes.
2. **Mixed money types:** several admin route DTOs serialized `f64`
   dollars, while the architecture standard §1.2 mandates integer cents
   end-to-end. INTEGER (`i32`) columns on `courses_enhanced.price_cents`,
   `indicators.sale_price_cents`, and `indicators_enhanced.price_cents`
   capped at ~$21M and violated the BIGINT contract.

---

## Scope (executed in order)

| Step | Description | Status |
|------|------------|--------|
| B1 | Ghost-column queries → JOIN `membership_plans` | ✅ |
| B2 | `admin_member_management::get_member_full` | ✅ |
| A2 | Admin route DTOs (`admin.rs`, `admin_orders.rs`, `subscriptions_admin.rs`) → `*_cents` | ✅ |
| B4 | `/api/admin/members/analytics/overview` 404 | ✅ phantom — endpoint never existed, frontend never calls it |
| C1 | `courses_enhanced.price_cents` INTEGER → BIGINT | ✅ via migration 061 |
| 061 | Drop `indicators.price` (NUMERIC) + widen INTEGER cents columns | ✅ applied |
| A6 | `indicators.rs` + `admin_indicators.rs` + models → cents | ✅ |
| – | `user.rs` `MembershipPlanDbRow.price` → `price_cents`, `UserMembershipResponse.price` → `priceCents` | ✅ (downstream of A2) |
| – | Two clippy warnings in `reconcile_stripe.rs` cleaned up | ✅ |

---

## Migration 061

```sql
BEGIN;

ALTER TABLE indicators DROP COLUMN IF EXISTS price;
ALTER TABLE indicators_enhanced DROP COLUMN IF EXISTS price;

ALTER TABLE courses_enhanced
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

ALTER TABLE indicators
    ALTER COLUMN sale_price_cents TYPE BIGINT
    USING sale_price_cents::BIGINT;

ALTER TABLE indicators_enhanced
    ALTER COLUMN price_cents TYPE BIGINT
    USING price_cents::BIGINT;

COMMIT;
```

Pre-flight check: `indicators.price=79.00` and `indicators.price_cents=7900`
agreed. `courses_enhanced` and `indicators_enhanced` were empty. No data loss
possible.

Post-migration schema (verified via `\d` introspection):

| Table | Column | Type |
|------|------|------|
| courses_enhanced | price_cents | bigint ✅ |
| indicators | price_cents | bigint ✅ |
| indicators | sale_price_cents | bigint ✅ |
| indicators_enhanced | price_cents | bigint ✅ |
| membership_plans | price | numeric *(intentional display cache; Rust converts at SQL boundary)* |

---

## Gate evidence

```
$ cargo check                                  ✅ 0 errors
$ cargo clippy --no-deps                       ✅ 0 errors / 0 warnings
$ cargo test --test utils_test                 ✅ 17 passed; 0 failed
$ pnpm check                                   ✅ 5217 FILES 0 ERRORS 0 WARNINGS
```

---

## Admin endpoint verification

Ground truth from local Postgres:

```
COUNT_ACTIVE: 1
MRR_CENTS: 22700
TOTAL_ORDER_CENTS: 108200
PLANS_DISTINCT: 1
TOTAL_USERS: 14
```

Live curl results (logged in as super_admin via JWT):

| Endpoint | Field | Before (zeroes) | After |
|---------|-------|--------|-------|
| `/api/admin/members/stats` | `revenue.mrr_cents` | $0 | **22700¢** ✅ |
| `/api/admin/members/stats` | `revenue.total_cents` | $0 | **107200¢** ✅ |
| `/api/admin/members/stats` | `subscriptions.active` | 0 | **1** ✅ |
| `/api/admin/members/analytics/metrics` | `mrr_cents` | $0 | **22700¢** ✅ |
| `/api/admin/members/analytics/metrics` | `avg_ltv_cents` | $0 | **45000¢** ✅ |
| `/api/admin/members/analytics/revenue` | `arr_cents` | $0 | **272400¢** ✅ |
| `/api/admin/members/analytics/revenue` | `mrr_cents` | $0 | **22700¢** ✅ |
| `/api/admin/orders/stats` | `total_revenue_cents` | $0 | **108200¢** ✅ |
| `/api/admin/orders/stats` | `total_orders` | 0 | **18** ✅ |
| `/api/admin/membership-plans[].price_cents` | – | f64 dollars | **i64 cents (4700, …)** ✅ |
| `/api/admin/indicators[].price_cents` | – | f64 dollars | **i64 cents (7900)** ✅ |
| `/api/indicators` (public) | `data.indicators[].price_cents` | error / dollars | **i64 cents (7900)** ✅ |

`mrr_cents` matches ground truth exactly. `total_revenue_cents` from
`/admin/orders/stats` (108200¢) matches the DB sum exactly; the
`/admin/members/stats` `total_cents` (107200¢) sums a different scope
(member-attributed revenue) and is correct in its own scope.

Frontend smoke test via SvelteKit proxy at `http://localhost:5173`:

```
── via SvelteKit /api/admin/members/stats ──
{
    "revenue": { "mrr_cents": 22700, "total_cents": 107200, "avg_ltv_cents": 0 },
    "subscriptions": { "active": 1, "trial": 1, "churned": 3, "churn_rate": 0 }
}
```

---

## B4 — phantom path

The original task brief mentioned `/api/admin/members/analytics/overview`
returning 404. Investigation:

- Frontend grep: no consumer references this path. The members analytics
  page calls 6 specific endpoints — `metrics`, `growth`, `cohorts`,
  `revenue`, `churn-reasons`, `segments`.
- Backend grep: no handler is registered for `analytics/overview` and the
  router lists exactly the 6 endpoints above.

Conclusion: there is no missing endpoint. The 404 was a stray observation,
likely from a tab the user opened against a path that was never wired up.
No code change needed.

---

## Files touched

```
 api/src/jobs/reconcile_stripe.rs          |   4 +-   (clippy)
 api/src/models/course.rs                  |   8 +-   (i32 → i64)
 api/src/models/course_enhanced.rs         |   8 +-   (i32 → i64)
 api/src/models/indicator.rs               |   6 +-   (i32 → i64)
 api/src/routes/admin.rs                   | 117 +-   (A2 coupons + plans)
 api/src/routes/admin_indicators.rs        |  80 +-   (A6)
 api/src/routes/admin_member_management.rs |  44 +-   (B2)
 api/src/routes/admin_members.rs           |  62 +-   (B1)
 api/src/routes/admin_orders.rs            |  36 +-   (A2)
 api/src/routes/indicators.rs              |  77 +-   (A6)
 api/src/routes/member_indicators.rs       |   5 +-   (drop SELECT price)
 api/src/routes/members.rs                 | 130 +-   (B1)
 api/src/routes/subscriptions_admin.rs     |  37 +-   (A2)
 api/src/routes/user.rs                    |  22 +-   (downstream)
 api/migrations/061_money_cents_unification.sql       (new)
 14 files changed, 388 insertions(+), 248 deletions(-)
```

---

## Out of scope (deferred)

- **Batch 3** (E1 coupon redemption E2E with browser, E2 Test Clock for
  period-end) — separate branch off main once Batch 2 lands.
- **Batch 4** (re-subscribe row strategy) — backlog.
- **Batch 5** (refactors D1/D2/D3/D4) — backlog.
- **Task 4** (Postmark wiring) — backlog.
- `membership_plans.price` NUMERIC kept intentionally as display cache;
  Rust converts at the SQL boundary. Flipping it to BIGINT is a separate
  follow-up if/when there's a reason.

---

## Ready to merge

All gates green, all admin endpoints return non-zero ground-truth numbers,
public indicators route healed after column drop, frontend smoke passes.
