# Task 5 Result — Customer Portal

Branch: `payments-fix-2026-04`
Date: 2026-04-28
Engineer: Billy Ribeiro

---

## Summary

All Task 5 objectives confirmed with raw evidence:

1. **`POST /api/payments/portal`** verified — returns a real Stripe billing portal URL
2. **Stripe Customer Portal configured** — `bpc_1TRHrh9HsGkDuN3bwBmgS386` (default, active)
3. **Manage Billing button** added to `/my/subscriptions`
4. **SvelteKit proxy** created at `frontend/src/routes/api/payments/portal/+server.ts`
5. **E2E cancel** — `cancel_at_period_end = true` after `customer.subscription.updated` webhook
6. **E2E resume** — `cancel_at_period_end = false` after second webhook
7. **`cargo check`** — 0 errors
8. **`pnpm check`** — 0 errors / 0 warnings

---

## Files Changed

| File | Change |
|------|--------|
| `frontend/src/routes/api/payments/portal/+server.ts` | New proxy: POST → `API_URL/api/payments/portal` with cookie token |
| `frontend/src/routes/my/subscriptions/+page.svelte` | Added `openBillingPortal()` function + Manage Billing button + styles |

---

## Task 5.1 — Portal Endpoint Verification

Backend `create_portal` at `api/src/routes/payments.rs:364` was already fully implemented. It:
1. Reads `stripe_customer_id` from `user_memberships` for the authenticated user
2. Calls `stripe.create_portal_session(customer_id, return_url)`
3. Returns `{ url }` pointing to `billing.stripe.com`

No backend changes needed.

### HTTP Evidence

```bash
POST /api/payments/portal
Authorization: Bearer <user-jwt>
{ "return_url": "http://localhost:5173/my/subscriptions" }

→ 200 OK
{
  "url": "https://billing.stripe.com/p/session/test_YWNjdF8x..."
}
```

---

## Task 5.2 — Stripe Customer Portal Configuration

Created via Stripe API (`POST /v1/billing_portal/configurations`):

**Config ID:** `bpc_1TRHrh9HsGkDuN3bwBmgS386`

| Feature | Setting |
|---------|---------|
| `payment_method_update` | enabled |
| `invoice_history` | enabled |
| `subscription_cancel` | enabled, mode=`at_period_end` |
| `subscription_cancel.cancellation_reason` | enabled (too_expensive, missing_features, switched_service, unused, other) |
| `subscription_update` | enabled, proration=`create_prorations` |
| `customer_update` | enabled (email, name) |

---

## E2E: Cancel → Webhook → DB → Resume

### Setup
Active subscription before test:
```sql
SELECT id, status, cancel_at_period_end, stripe_subscription_id
FROM user_memberships WHERE user_id = 2;

 id | status | cancel_at_period_end |    stripe_subscription_id
----+--------+----------------------+------------------------------
  2 | active | f                    | sub_1TRHJ69HsGkDuN3bYzIwRjz7
```

### Cancel (set cancel_at_period_end=true)

```bash
POST https://api.stripe.com/v1/subscriptions/sub_1TRHJ69HsGkDuN3bYzIwRjz7
cancel_at_period_end=true

→ cancel_at_period_end: True | status: active
```

Webhook received:
```
evt_1TRHsF9HsGkDuN3bLkGyOqgS | customer.subscription.updated
API: "subscription_updated" sub_1TRHJ69HsGkDuN3bYzIwRjz7 status=active
```

DB after cancel webhook:
```sql
SELECT id, status, cancel_at_period_end, updated_at
FROM user_memberships WHERE stripe_subscription_id = 'sub_1TRHJ69HsGkDuN3bYzIwRjz7';

 id | status | cancel_at_period_end |         updated_at
----+--------+----------------------+----------------------------
  2 | active | t                    | 2026-04-28 20:04:51.91894
```

`cancel_at_period_end = true` ✅

### Resume (set cancel_at_period_end=false)

```bash
POST https://api.stripe.com/v1/subscriptions/sub_1TRHJ69HsGkDuN3bYzIwRjz7
cancel_at_period_end=false

→ cancel_at_period_end: False | status: active
```

DB after resume webhook:
```sql
SELECT id, status, cancel_at_period_end, updated_at
FROM user_memberships WHERE stripe_subscription_id = 'sub_1TRHJ69HsGkDuN3bYzIwRjz7';

 id | status | cancel_at_period_end |         updated_at
----+--------+----------------------+----------------------------
  2 | active | f                    | 2026-04-28 20:05:16.613962
```

`cancel_at_period_end = false` ✅

---

## Manage Billing Button

Added to `frontend/src/routes/my/subscriptions/+page.svelte` in the footer-actions section (visible whenever the user has at least one subscription):

```svelte
<button class="btn-manage-billing" onclick={openBillingPortal} disabled={openingPortal}>
  Manage Billing
</button>
```

`openBillingPortal()` calls `POST /api/payments/portal` with `return_url = window.location.href`, then redirects to the returned URL. Shows a spinner while the redirect is in progress.

---

## Final Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ 0 errors |
| `pnpm check` | ✅ 0 errors / 0 warnings |
| `POST /api/payments/portal` returns URL | ✅ |
| Stripe portal config created | ✅ `bpc_1TRHrh9HsGkDuN3bwBmgS386` |
| cancel → `cancel_at_period_end = true` | ✅ `2026-04-28 20:04:51` |
| resume → `cancel_at_period_end = false` | ✅ `2026-04-28 20:05:16` |
