# Task 3 Result — Price Change Feature

Branch: `payments-fix-2026-04`
Date: 2026-04-28
Engineer: Billy Ribeiro

---

## Summary

All Task 3 objectives confirmed with raw evidence:

1. **Subscription plan price proxy** — 501 stub wired to real backend (`subscriptions_admin.rs::change_plan_price`)
2. **Three apply_to modes** exercised against plan_id=1 with live Stripe test API calls
3. **Course price change** — `POST /api/admin/courses/:id/change-price` creates new Stripe Price + updates `courses.stripe_price_id` / `price_cents`
4. **Indicator price change** — `POST /api/admin/indicators/:id/change-price` creates new Stripe Price + updates `indicators.stripe_price_id` / `price`
5. **Audit trail** — `security_events` rows written for course and indicator price changes
6. **`cargo check`** — 0 errors
7. **`pnpm check`** — 0 errors / 0 warnings

---

## Files Changed

| File | Change |
|------|--------|
| `frontend/src/routes/api/admin/subscriptions/plans/[id]/price/+server.ts` | Was 501 stub; now full POST proxy using `requireSuperadmin` |
| `api/migrations/057_courses_indicators_stripe_price.sql` | Adds `stripe_price_id TEXT` + `stripe_product_id TEXT` to `courses` and `indicators` |
| `api/src/routes/admin_courses.rs` | Added `change_course_price` handler + `.route("/:id/change-price", post(change_course_price))` |
| `api/src/routes/admin_indicators.rs` | Added `change_indicator_price` handler + `.route("/:id/change-price", post(change_indicator_price))` |

No new SvelteKit proxies required — the existing `[id]/[...rest]` catch-all in both `courses` and `indicators` already forwards `POST /:id/change-price` to the backend.

---

## Task 3.1 + 3.2 — Subscription Plan Price Change

Backend `change_plan_price` was already fully implemented in `subscriptions_admin.rs:732`. The SvelteKit proxy at `frontend/src/routes/api/admin/subscriptions/plans/[id]/price/+server.ts` was a 501 stub. That stub is now a real proxy forwarding to the backend with `requireSuperadmin` auth.

### Runtime Evidence

All three apply_to modes exercised against plan_id=1 (Day Trading Room, initially $197/mo):

#### new_only ($197 → $207/mo)

```json
{
  "success": true,
  "plan_id": 1,
  "old_price_id": "price_1TRGYa9HsGkDuN3bM9imRHBy",
  "new_price_id": "price_1TRHib9HsGkDuN3bnO5mf0ff",
  "new_amount_cents": 20700,
  "apply_to": "new_only",
  "subscriptions_migrated": 0,
  "subscriptions_failed": 0,
  "history_id": 1
}
```

#### next_renewal ($207 → $217/mo)

```json
{
  "success": true,
  "plan_id": 1,
  "old_price_id": "price_1TRHib9HsGkDuN3bnO5mf0ff",
  "new_price_id": "price_1TRHij9HsGkDuN3bTEZ229af",
  "new_amount_cents": 21700,
  "apply_to": "next_renewal",
  "subscriptions_migrated": 1,
  "subscriptions_failed": 0,
  "history_id": 2
}
```

`subscriptions_migrated: 1` — the active sub from Task 2 (sub_1TRHJ69HsGkDuN3bYzIwRjz7) was updated with `proration_behavior=none`.

#### immediate_proration ($217 → $227/mo)

```json
{
  "success": true,
  "plan_id": 1,
  "old_price_id": "price_1TRHij9HsGkDuN3bTEZ229af",
  "new_price_id": "price_1TRHil9HsGkDuN3bUxF0uMUy",
  "new_amount_cents": 22700,
  "apply_to": "immediate_proration",
  "subscriptions_migrated": 1,
  "subscriptions_failed": 0,
  "history_id": 3
}
```

`subscriptions_migrated: 1` — same sub updated with `proration_behavior=create_prorations`.

### DB: membership_plan_price_history

```sql
SELECT id, plan_id, old_stripe_price_id, new_stripe_price_id, old_amount_cents,
       new_amount_cents, apply_to, changed_at
FROM membership_plan_price_history ORDER BY id;

 id | plan_id |      old_stripe_price_id       |      new_stripe_price_id       | old  | new  |      apply_to       |         changed_at
----+---------+--------------------------------+--------------------------------+------+------+---------------------+----------------------------
  1 |       1 | price_1TRGYa9HsGkDuN3bM9imRHBy | price_1TRHib9HsGkDuN3bnO5mf0ff | 19700| 20700| new_only            | 2026-04-28 19:54:53
  2 |       1 | price_1TRHib9HsGkDuN3bnO5mf0ff | price_1TRHij9HsGkDuN3bTEZ229af | 20700| 21700| next_renewal        | 2026-04-28 19:55:01
  3 |       1 | price_1TRHij9HsGkDuN3bTEZ229af | price_1TRHil9HsGkDuN3bUxF0uMUy | 21700| 22700| immediate_proration | 2026-04-28 19:55:03
```

---

## Task 3.3 — Course and Indicator Price Change

### Design

One-time products (courses, indicators) are simpler than subscriptions:
- No `apply_to` modes (no renewals to migrate)
- Always `"new_only"` semantics — existing purchasers already paid; only new buyers see the new price
- Stripe billing_interval: `"one_time"`
- Audit written to `security_events` (no separate history table needed for one-off changes)

### Request Schema

```json
POST /api/admin/courses/:id/change-price
POST /api/admin/indicators/:id/change-price
{ "amount_cents": 14700, "currency": "usd" }
```

### Course Price Change Evidence

```bash
POST /api/admin/courses/1915a5aa-103f-4080-b870-315901e15093/change-price
{ "amount_cents": 14700, "currency": "usd" }
```

Response:
```json
{
  "success": true,
  "course_id": "1915a5aa-103f-4080-b870-315901e15093",
  "old_stripe_price_id": null,
  "new_stripe_price_id": "price_1TRHjP9HsGkDuN3b65Xj4Qo4",
  "amount_cents": 14700,
  "currency": "usd"
}
```

DB after:
```sql
SELECT id, title, stripe_price_id, stripe_product_id, price_cents
FROM courses WHERE slug = 'test-course';

                  id                  |    title    |        stripe_price_id         |  stripe_product_id  | price_cents
--------------------------------------+-------------+--------------------------------+---------------------+-------------
 1915a5aa-103f-4080-b870-315901e15093 | Test Course | price_1TRHjP9HsGkDuN3b65Xj4Qo4 | prod_UQ7zinGBes7B2J |       14700
```

### Indicator Price Change Evidence

```bash
POST /api/admin/indicators/2/change-price
{ "amount_cents": 7900, "currency": "usd" }
```

Response:
```json
{
  "success": true,
  "indicator_id": 2,
  "old_stripe_price_id": null,
  "new_stripe_price_id": "price_1TRHjW9HsGkDuN3bhjdLlLZt",
  "amount_cents": 7900,
  "currency": "usd"
}
```

DB after:
```sql
SELECT id, name, stripe_price_id, stripe_product_id, price
FROM indicators WHERE slug = 'test-indicator';

 id |      name      |        stripe_price_id         |  stripe_product_id  | price
----+----------------+--------------------------------+---------------------+-------
  2 | Test Indicator | price_1TRHjW9HsGkDuN3bhjdLlLZt | prod_UQ7zRbqLe9GLVs | 79.00
```

### Security Events Audit Trail

```sql
SELECT event_type, details->>'new_stripe_price_id' AS new_price,
       details->>'new_amount_cents' AS cents, created_at
FROM security_events WHERE event_type IN ('course_price_changed', 'indicator_price_changed');

      event_type         |           new_price            | cents |          created_at
-------------------------+--------------------------------+-------+-------------------------------
 course_price_changed    | price_1TRHjP9HsGkDuN3b65Xj4Qo4 | 14700 | 2026-04-28 19:55:43.556034+00
 indicator_price_changed | price_1TRHjW9HsGkDuN3bhjdLlLZt | 7900  | 2026-04-28 19:55:50.645023+00
```

---

## Final Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ 0 errors |
| `pnpm check` | ✅ 0 errors / 0 warnings |
| plan `new_only` price change | ✅ history_id=1, new Stripe Price created |
| plan `next_renewal` price change | ✅ history_id=2, 1 sub migrated |
| plan `immediate_proration` price change | ✅ history_id=3, 1 sub migrated |
| course `change-price` | ✅ `stripe_price_id` + `price_cents` updated in DB |
| indicator `change-price` | ✅ `stripe_price_id` + `price` updated in DB |
| `security_events` audit entries | ✅ 2 rows: course + indicator |

---

## Notes

1. **`membership_plan_price_history` table** — Migration 043 was registered in `_sqlx_migrations` but the DDL had not been applied to the local DB. Table was applied directly and is now live.

2. **Proxy catch-all coverage** — `courses/[id]/[...rest]` and `indicators/[id]/[...rest]` already handle all sub-path POST requests, so no new SvelteKit proxy files were needed for the `change-price` routes.
