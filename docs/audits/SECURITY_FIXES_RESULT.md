# SECURITY_FIXES_RESULT.md

# Task 1 — Security Fixes

# Branch: payments-fix-2026-04

# Date: 2026-04-28

# Gates: cargo check ✅ 0 errors | pnpm check ✅ 0 errors 0 warnings

---

## 1.1 — Fix reactivate endpoint (FREE SUBSCRIPTION BYPASS — CRITICAL)

**Files changed:**

- `api/src/routes/subscriptions.rs` (lines ~1204–1259)
- `api/src/services/stripe.rs` (added `update_subscription`)
- `frontend/src/routes/my/subscriptions/+page.svelte` (line ~227)

**Before:**

```
POST /api/subscriptions/:id/reactivate
→ For fully cancelled subs: SET status='active' for 30 days, NO Stripe call
→ Any user with a cancelled subscription could get 30 days free
```

**After:**

- `cancel_at_period_end=true` path: calls `stripe.update_subscription(id, [("cancel_at_period_end","false")])` → updates DB only after Stripe confirms
- Fully cancelled path: returns `400 { error: "Subscription has fully ended. Please re-subscribe to restart.", resubscribe: true, plan_id }`
- Frontend: for `status === 'cancelled' && !cancelAtPeriodEnd`, `goto('/pricing?plan=...')` immediately, no API call; backstop in error handler for the API's 400 response

**Evidence:** No DB write path exists that sets `status='active'` without a confirmed Stripe response.

---

## 1.2 — Fix client-supplied price (MONETISATION BYPASS — HIGH)

**File:** `api/src/routes/payments.rs`

**Before:**

```rust
pub struct CheckoutItem {
    pub price: f64,  // ← accepted from client
    ...
}
// Used directly in Stripe checkout session and order_items insert
```

**After:**

- `price` field removed from `CheckoutItem`
- Each item must supply `plan_id` or `product_id`
- Price is looked up from `membership_plans.price` (for plans) or `products.price` (for products) with `AND is_active = true`
- Lookup failure → 400 "Plan X not found or inactive"
- Introduced `ResolvedItem` struct to carry DB-sourced name/price through the order items insert

---

## 1.3 — Fix open redirect (HIGH)

**Files changed:**

- `api/src/routes/payments.rs` — `CreateCheckoutRequest`
- `api/src/routes/checkout.rs` — `CheckoutRequest`

**Before:**

```rust
pub success_url: String,   // accepted verbatim from client
pub cancel_url: String,    // passed directly to Stripe
```

**After:**

```rust
pub success_path: Option<String>,   // must start with "/"
pub cancel_path: Option<String>,    // must start with "/"
```

- Validation: `if !success_path.starts_with('/') { return 400 }`
- URL construction: `format!("{}{}", state.config.app_url.trim_end_matches('/'), path)`
- Attacker cannot redirect Stripe's post-payment flow to an external domain

---

## 1.4 — Auth-gate coupon validate (ENUMERATION — HIGH)

**File:** `api/src/routes/coupons.rs`

**Before:**

```rust
async fn validate_coupon(State(state), Json(input)) { ... }
// Route: POST /api/coupons/validate — no auth required
```

**After:**

```rust
async fn validate_coupon(State(state), _user: User, Json(input)) { ... }
```

- Axum's `User` extractor returns 401 if no valid JWT present
- Unauthenticated coupon enumeration attacks blocked

---

## 1.5 — Webhook body as Bytes (HMAC INTEGRITY — HIGH)

**Files changed:**

- `api/src/routes/payments.rs` — webhook handler
- `api/src/services/stripe.rs` — `verify_webhook`

**Before:**

```rust
async fn webhook(..., body: String) { ... }
// stripe.rs:613: String::from_utf8_lossy(payload) — corrupts non-UTF8 bytes, breaking HMAC
```

**After:**

```rust
async fn webhook(..., body: Bytes) { ... }
match stripe_for_webhook.verify_webhook(&body, signature) { ... }
let body_str = std::str::from_utf8(&body)?;  // hard fail on non-UTF8
```

- `verify_webhook` now receives exact raw bytes — HMAC computed over unmodified payload
- `from_utf8_lossy` replaced with `from_utf8` — non-UTF8 payloads rejected with 400, not silently corrupted

---

## 1.6 — Webhook idempotency (DOUBLE-PROCESSING — HIGH)

**Files changed:**

- `api/migrations/055_webhook_idempotency_and_disputes.sql` (NEW)
- `api/src/routes/payments.rs` — webhook handler

**Migration creates:**

```sql
CREATE TABLE webhook_events (
    event_id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    error TEXT,
    payload JSONB NOT NULL
);
CREATE INDEX idx_webhook_events_unprocessed ON webhook_events(received_at)
    WHERE processed_at IS NULL;
```

**Handler logic:**

```
1. INSERT INTO webhook_events ... ON CONFLICT (event_id) DO NOTHING RETURNING 1
2. If returns NULL → duplicate → return 200 immediately
3. Process event handlers
4. UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1
```

- Stripe retries are safe; duplicate events are idempotently dropped
- `processed_at` allows monitoring for stuck/unfinished events

---

## 1.7 — Stop swallowing DB errors in webhook handlers (HIGH)

**File:** `api/src/routes/payments.rs`

**Before (two locations):**

```rust
.execute(&state.db.pool).await.ok();  // silently discards DB errors
```

**After — `handle_subscription_updated`:**

```rust
.execute(&state.db.pool).await.map_err(|e| {
    tracing::error!("DB write failed in subscription_updated — Stripe will retry");
    (INTERNAL_SERVER_ERROR, json!({"error": "Failed to update subscription"}))
})?;
```

**After — `handle_subscription_deleted`:**

```rust
.execute(&state.db.pool).await.map_err(|e| {
    tracing::error!("DB write failed in subscription_deleted — Stripe will retry");
    (INTERNAL_SERVER_ERROR, json!({"error": "Failed to cancel subscription"}))
})?;
```

- DB failures now propagate as 500, causing Stripe to retry the webhook
- Combined with idempotency (1.6), retries are safe and will eventually succeed

---

## 1.8 — Add charge.dispute.created handler (MISSING HANDLER — MEDIUM)

**Files changed:**

- `api/migrations/055_webhook_idempotency_and_disputes.sql` — `payment_disputes` table
- `api/src/routes/payments.rs` — dispatch + `handle_dispute_created`

**Migration creates:**

```sql
CREATE TABLE payment_disputes (
    id BIGSERIAL PRIMARY KEY,
    stripe_dispute_id TEXT UNIQUE NOT NULL,
    stripe_charge_id TEXT NOT NULL,
    reason TEXT,
    status TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    response_deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Handler:**

- Inserts into `payment_disputes` with `ON CONFLICT DO NOTHING` (idempotent)
- Inserts into `security_events` with `severity: "high"` in JSONB details
- `tracing::warn!` so it surfaces in log aggregation
- Admin email notification: wired in Task 4 (Postmark)

---

## 1.9 — Refund must revoke access (ACCESS CONTROL — HIGH)

**File:** `api/src/routes/payments.rs` — `handle_refund`

**Before:**

```rust
// Only updated orders table
// No access revocation on refund
.execute(...).await.ok();
```

**After — full refund path (`refund_amount >= total_amount`):**

1. Updates `orders` status → `'refunded'` (now propagates errors, not `.ok()`)
2. Cancels `user_memberships` rows linked to the order's `user_id`
3. Sets `user_course_enrollments.is_active = false` for the user
4. Sets `user_indicator_access.is_active = false` for the user
5. Logs `access_revoked_on_refund` event

Partial refund path: only updates order status → `'partial_refund'`, no access change.

---

## Gates

| Gate                        | Result                              |
| --------------------------- | ----------------------------------- |
| `cargo check`               | ✅ 0 errors, 0 warnings             |
| `pnpm check` (svelte-check) | ✅ 0 errors, 0 warnings, 5215 files |

---

## Files modified

| File                                                      | Change                                                                    |
| --------------------------------------------------------- | ------------------------------------------------------------------------- |
| `api/src/routes/subscriptions.rs`                         | 1.1 — reactivate endpoint                                                 |
| `api/src/services/stripe.rs`                              | 1.1 — `update_subscription` method; 1.5 — `from_utf8_lossy` → `from_utf8` |
| `api/src/routes/payments.rs`                              | 1.2, 1.3, 1.5, 1.6, 1.7, 1.8, 1.9                                         |
| `api/src/routes/checkout.rs`                              | 1.3 — open redirect                                                       |
| `api/src/routes/coupons.rs`                               | 1.4 — auth gate                                                           |
| `api/migrations/055_webhook_idempotency_and_disputes.sql` | 1.6, 1.8 — new migration                                                  |
| `frontend/src/routes/my/subscriptions/+page.svelte`       | 1.1 — redirect fully cancelled subs to pricing                            |

---

## Ready for Task 2 review

Awaiting explicit go-ahead before starting Task 2 (Real Stripe Checkout).
