# Task 2 Result — Real Stripe Checkout E2E

Branch: `payments-fix-2026-04`
Date: 2026-04-28
Engineer: Billy Ribeiro

---

## Summary

All 5 Task 2 objectives confirmed with raw evidence:

1. **20 membership_plans seeded** with real Stripe test product/price IDs
2. **checkout.rs** uses `stripe_price_id` from DB for subscription line items
3. **Real Stripe Checkout session** completed with test card (tok_visa / 4242...)
4. **Webhook fired → `user_memberships` row created** (`status=active`)
5. **Idempotency confirmed** — duplicate delivery skipped, still 1 row

---

## Fixes Made in This Session

### Fix A: `StripeSubscription` serde default for missing period fields

**File**: `api/src/services/stripe.rs:42-45`

Stripe's newer API version (2026-03-25.dahlia) no longer returns
`current_period_start` / `current_period_end` at the subscription top level
(they moved to `items.data[0]`). The struct required these as bare `i64`,
causing `as_subscription()` to return `None` and every `customer.subscription.*`
webhook to return 400.

Fix: added `#[serde(default)]` on both fields. They default to `0i64`
(Unix epoch) when absent; callers already handle `Option<DateTime>` via
`.map()` so epoch is safe.

**Before**: `customer.subscription.created` → 400 (every time), `processed_at = NULL`
**After**: `customer.subscription.created` → 200, `processed_at` set

### Fix B: Missing unique constraint on `user_memberships(user_id, plan_id)`

The `INSERT INTO user_memberships ... ON CONFLICT (user_id, plan_id) DO UPDATE`
in `handle_checkout_completed` requires a unique constraint that did not exist.
Every membership creation attempt returned:
```
there is no unique or exclusion constraint matching the ON CONFLICT specification
```

Fix: `ALTER TABLE user_memberships ADD CONSTRAINT uq_user_memberships_user_plan UNIQUE (user_id, plan_id);`

This DDL was applied to the local dev DB. A migration file needs to be created
for production (out of scope for this task).

---

## Raw Evidence

### E1 — 20 plans seeded with stripe_price_id

```sql
SELECT id, name, stripe_price_id FROM membership_plans WHERE stripe_price_id IS NOT NULL;
-- 20 rows returned (plan_id 1-20, all with prod_UQ6m* / price_1TRGYa-n* IDs)
```

Source: `scripts/stripe_test_product_mapping.json`

### E2 — checkout.rs uses stripe_price_id

Checkout session `cs_test_b1OKzo1cCvIf4a2B7Ui5UwZisQRx8vxshP0UyHU2on9eCWlxC1YFTAgseE`
created for plan_id=1 (Day Trading Room, $197/month). Stripe confirmed
`price_id: price_1TRGYa9HsGkDuN3bM9imRHBy` in the session line items.

### E3 — Real Stripe Checkout completed (test card)

```
POST /v1/payment_pages/cs_test_b1OKzo1cCvIf4a2B7Ui5UwZisQRx8vxshP0UyHU2on9eCWlxC1YFTAgseE/confirm
payment_method_data[card][token]=tok_visa
→ {"status": "complete"}
```

Webhook event received:
```
evt_1TRH5k9HsGkDuN3b5m8E8MMl | checkout.session.completed | processed_at: 2026-04-28 19:14:44.95
```

API log:
```
INFO payments: Order completed via Stripe event="order_completed" order_id=6 subscription_id=Some("sub_1TRH5f9HsGkDuN3bjsGiQMfh")
```

### E4 — user_memberships row created

```sql
SELECT id, user_id, plan_id, stripe_subscription_id, stripe_customer_id, status, starts_at
FROM user_memberships WHERE user_id = 2 AND plan_id = 1;

 id | user_id | plan_id |    stripe_subscription_id    | stripe_customer_id | status |         starts_at
----+---------+---------+------------------------------+--------------------+--------+----------------------------
  1 |       2 |       1 | sub_1TRH5f9HsGkDuN3bjsGiQMfh | cus_UQ7KWtmR8zPBkT | active | 2026-04-28 19:14:45.154048
```

### E5 — Idempotency: duplicate delivery skipped

```bash
stripe events resend evt_1TRH5k9HsGkDuN3b5m8E8MMl
```

API log on second delivery:
```
INFO payments: Duplicate webhook event — skipping event_id=evt_1TRH5k9HsGkDuN3b5m8E8MMl
```

DB count after resend:
```sql
SELECT COUNT(*) FROM user_memberships WHERE user_id = 2 AND plan_id = 1;
-- 1
```

### E6 — Frontend typecheck: 0 errors / 0 warnings

```
pnpm check
→ 5215 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

### E7 — cargo check: 0 errors

```
cargo check --manifest-path api/Cargo.toml --locked
→ ✅ Success
```

---

## Bugs Found (not in scope to fix now)

1. **Postmark email signature** — `noreply@example.com` not configured as a
   Sender Signature in Postmark. Non-blocking (email send is fire-and-forget,
   order completes correctly).

---

## Follow-up 1: Real period timestamps (resolved)

**Root cause**: Stripe 2026 API moved `current_period_start`/`current_period_end`
from the subscription top level to `items.data[0]`. The original `#[serde(default)]`
workaround stored Unix epoch (1970) in `user_memberships`.

**Fix in `api/src/services/stripe.rs`**:
- Added `current_period_start`/`current_period_end` fields to `StripeSubscriptionItem`
- Added `StripeSubscriptionItemList` wrapper struct
- Added `items: Option<StripeSubscriptionItemList>` to `StripeSubscription`
- Added `get_current_period() -> (i64, i64)` method: reads from `items.data[0]` first,
  falls back to top-level fields (handles older fixtures/mocks)

**Fix in `api/src/routes/payments.rs`**:
- `handle_checkout_completed`: replaced `sub.current_period_start` / `sub.current_period_end`
  with `sub.get_current_period()`
- `handle_subscription_updated`: same replacement

**Evidence — period timestamps are real (not 1970)**:

```sql
SELECT id, current_period_start, current_period_end
FROM user_memberships ORDER BY id DESC LIMIT 1;

 id | current_period_start | current_period_end
----+----------------------+---------------------
  2 | 2026-04-28 19:28:32  | 2026-05-28 19:28:32
```

Checkout session `cs_test_b10bkV1O9bu10mKwWXo3l3CWuE0o2PvjoV2IGsoRrLkrIlmGtVPFjxf5oF`,
subscription `sub_1TRHJ69HsGkDuN3bYzIwRjz7`.

---

## Follow-up 2: Unique constraint migration (resolved)

**Migration file**: `api/migrations/056_user_memberships_unique.sql`

```sql
ALTER TABLE user_memberships
    DROP CONSTRAINT IF EXISTS uq_user_memberships_user_plan;

ALTER TABLE user_memberships
    ADD CONSTRAINT uq_user_memberships_user_plan UNIQUE (user_id, plan_id);
```

`DROP ... IF EXISTS` makes it idempotent (handles dev DBs where the constraint was
already added manually).

**Registered in `_sqlx_migrations`**:

```sql
SELECT version FROM _sqlx_migrations WHERE version = 56;
-- version: 56
```

---

## Final Gates (Follow-ups)

| Gate | Result |
|------|--------|
| `cargo check` | ✅ 0 errors |
| `pnpm check` | ✅ 0 errors / 0 warnings |
| `current_period_start` in DB | ✅ 2026-04-28 (not 1970) |
| `current_period_end` in DB | ✅ 2026-05-28 (not 1970) |
| Migration 056 in `_sqlx_migrations` | ✅ version=56, success=true |

---

## Files Changed (this session)

| File | Change |
|------|--------|
| `api/src/services/stripe.rs` | `StripeSubscriptionItem` + `StripeSubscriptionItemList` structs; `items` field + `get_current_period()` on `StripeSubscription` |
| `api/src/routes/payments.rs` | Both period-timestamp callers use `get_current_period()` |
| `api/migrations/056_user_memberships_unique.sql` | New migration (idempotent) |
| `TASK2_RESULT.md` | This file |
