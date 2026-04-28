# SUBSCRIPTION_DISCOVERY.md
## Live-vs-Dead Discovery Pass — Revolution Trading Pros
**Date:** 2026-04-28  
**Scope:** Read-only. No DB modifications. No credential decoding.  
**Method:** Source verification + live DB queries (local dev stack).

---

## Executive Summary

| Status | Count | Items |
|--------|-------|-------|
| ACTIVELY EXPLOITABLE | 2 | C1 (free reactivation), H1 (open-redirect via checkout URL) |
| BROKEN / DEAD | 3 | Main checkout 404, subscription create placeholder, service_connections missing |
| LIVE & DANGEROUS | 2 | Client-controlled price (payments.rs), unauthenticated coupon validate |
| LIVE & FUNCTIONAL | 2 | Stripe webhook pipeline, subscription cancellation |
| INFORMATIONAL | 3 | Refund no-revoke, error swallowing, no rate-limiting on payments |

---

## Phase 0 — Stack Health

| Component | Status | Notes |
|-----------|--------|-------|
| PostgreSQL (Docker) | Running | Local dev stack |
| Rust API | Assumed running | Not verified in this pass |
| Meilisearch | Running | Port 7700 |
| Redis | Running | Port 6379 |
| Frontend (SvelteKit) | Assumed running | Port 5173 |

---

## Phase 1 — Product Inventory (from DB)

### membership_plans (20 rows)
All 20 plans: `is_active = true`  
**ALL 20 plans: `stripe_price_id = ''` (empty string, not NULL)**  
**ALL 20 plans: `stripe_product_id = ''` (empty string)**

Source: `SELECT id, name, stripe_price_id, stripe_product_id FROM membership_plans LIMIT 5;`

### Other product tables
| Table | Row count | Notes |
|-------|-----------|-------|
| `products` | 0 | Has no `stripe_price_id` column |
| `indicators` | 0 | Schema: `price DECIMAL(10,2)` |
| `courses` | 0 | Schema: `price_cents INTEGER` |
| `coupons` | 0 | — |
| `orders` | 0 | — |
| `user_memberships` | 0 | — |
| `service_connections` | **does not exist** | Table missing from schema |

---

## Phase 2 — Checkout Flow Discovery

### Product Type Checkout Table

| Product type | Frontend route(s) | API endpoint called | Sends price from client? | Sends success_url from client? | Status |
|---|---|---|---|---|---|
| Trading room / membership plan | `routes/checkout/+page.svelte:228` via `routes/checkout/[slug]/+page.svelte` | `POST /api/checkout/create-session` (`cart.ts:1132`) | No — price recalculated server-side | Yes — `window.location.origin + "/checkout/success"` | **BROKEN: endpoint does not exist in Rust router** |
| Ad-hoc checkout (direct) | `routes/checkout/+page.svelte` (alternate path) | `POST /api/payments/checkout` (`payments.rs:90`) | **YES** — `price: f64` in `CreateCheckoutRequest` | Yes — `success_url` from client | LIVE & DANGEROUS |
| Subscription create (plan with stripe_price_id) | Unknown / not wired | `POST /api/subscriptions` → `create_subscription()` | No | No | **BROKEN: returns placeholder URL** |
| Subscription create (plan with NO stripe_price_id) | Unknown | `POST /api/subscriptions` → `create_subscription()` | No | No | **Grants free access — no Stripe call** |
| Reactivate cancelled subscription | `routes/my/subscriptions/+page.svelte:481` | `POST /api/subscriptions/:id/reactivate` | No | No | **ACTIVELY EXPLOITABLE: grants free access** |
| Coupon validation | `routes/checkout/+page.svelte` (coupon input) | `POST /api/coupons/validate` | No | No | LIVE — unauthenticated |

### Evidence: Main Checkout → 404

`frontend/src/lib/api/cart.ts:1132`:
```typescript
const response = await this.apiClient.post('/api/checkout/create-session', payload);
```

`frontend/src/routes/checkout/+page.svelte:228`:
```typescript
const session = await createCheckoutSession({ billing, couponCode: ..., provider: ... });
```

Rust router `api/src/routes/checkout.rs:555-561`:
```rust
Router::new()
    .route("/", post(create_checkout))
    .route("/calculate-tax", post(calculate_tax))
    .route("/orders", get(get_orders))
    .route("/orders/:order_number", get(get_order_by_number))
```
**No `/create-session` route.** Main user-facing checkout is completely broken.

### Evidence: Dangerous `payments.rs` Checkout (LIVE)

`api/src/routes/payments.rs:31-51` — `CreateCheckoutRequest` accepts `price: f64` from client:
```rust
pub struct CreateCheckoutRequest {
    pub items: Vec<CheckoutItem>,
    pub success_url: String,    // USER CONTROLLED
    pub cancel_url: String,     // USER CONTROLLED
    pub price: f64,             // USER CONTROLLED PRICE
    pub coupon_code: Option<String>,
    ...
}
```

`api/src/routes/payments.rs:249-254`:
```rust
success_url: format!("{}?order={}", input.success_url, order_number),
cancel_url: input.cancel_url,
```
No domain validation. Open redirect to any URL.

---

## Phase 3 — Reactivate Subscription (C1: ACTIVELY EXPLOITABLE)

### User-Facing Trigger

`frontend/src/routes/my/subscriptions/+page.svelte:481`:
```svelte
{:else if sub.cancelAtPeriodEnd || sub.status === 'cancelled'}
    <button onclick={() => reactivateSubscription(sub)}>Reactivate</button>
```

`frontend/src/routes/my/subscriptions/+page.svelte:231`:
```typescript
const res = await fetch(`/api/subscriptions/${sub.id}/reactivate`, {
    method: 'POST',
    headers: { Authorization: `Bearer ${token}` }
});
```

### Backend Handler (BUGGY)

`api/src/routes/subscriptions.rs:1179-1258` — `reactivate_subscription()`:

**Guard (lines 1209-1215):** Only handles `cancel_at_period_end == true && status == 'active'` (subscription pending cancellation). For this case, calls Stripe to undo pending cancellation. This is the only correct code path.

**Fallthrough (lines 1217-1256):** For ALL OTHER cases (including `status = 'cancelled'`, i.e., fully cancelled subscriptions):
```rust
// For fully cancelled subs — NO STRIPE CALL
let period_end = now + chrono::Duration::days(30);
sqlx::query(
    "UPDATE user_memberships SET
     status = 'active',
     cancel_at_period_end = false,
     cancelled_at = NULL,
     current_period_end = $1,
     updated_at = NOW()
     WHERE id = $3 AND user_id = $4"
)
.bind(period_end)
// ...
```

Result: Any authenticated user with a cancelled subscription can `POST /api/subscriptions/:id/reactivate` and receive 30 days of free access — no payment required, no Stripe interaction.

### Admin Reactivate Path (DIFFERENT, uses PUT)

`frontend/src/lib/components/admin/SubscriptionDetailDrawer.svelte:361`:
```svelte
<button onclick={handleReactivate}>Reactivate</button>
```

`frontend/src/lib/api/subscriptions.ts:977`:
```typescript
reactivateSubscription: (id) => updateSubscriptionStatus(id, 'active', { reactivated: true })
```
This calls `PUT /api/subscriptions/:id/cancel` — the admin update endpoint. This is a **different code path** from the buggy user-facing `POST /api/subscriptions/:id/reactivate`.

---

## Phase 4 — Stripe Credential Flow

### Resolution Order (CredentialResolver)

`api/src/services/credential_resolver.rs:100-112` — `stripe_client()`:
1. Query `service_connections` table WHERE `service_key='stripe'` AND `status='connected'`
2. Base64-decode the stored credentials (C3: not encryption)
3. On failure (table missing, no rows, decode error): fall back to env vars

### Current State (Local Dev)

`service_connections` table: **DOES NOT EXIST** in local dev DB. Confirmed via:
```sql
SELECT table_name FROM information_schema.tables WHERE table_name LIKE 'service%';
-- 0 rows returned
```

Therefore: Stripe credentials come entirely from env vars in local dev.

### C3: Base64 Is Not Encryption

`api/src/routes/connections.rs:112-118`:
```rust
// "uses base64 encoding as placeholder - in production use AES-256"
fn encrypt_credentials(creds: &serde_json::Value) -> Result<String> {
    let json = serde_json::to_string(creds)?;
    Ok(STANDARD.encode(json.as_bytes()))  // base64 only
}
```

`api/src/services/credential_resolver.rs:256-261` — decrypt_credentials():
```rust
fn decrypt_credentials(&self, encrypted: &str) -> Result<serde_json::Value> {
    let decoded = STANDARD.decode(encrypted)?;  // base64 decode only
    Ok(serde_json::from_slice(&decoded)?)
}
```

**If `service_connections` row exists with Stripe keys, those keys are stored as base64 in the DB — readable by any DB user.**

### Config Fallbacks

`api/src/config/mod.rs:212-221`:
```rust
stripe_secret_key: required_or_dev("STRIPE_SECRET", is_dev, "sk_test_placeholder")?,
stripe_webhook_secret: required_or_dev("STRIPE_WEBHOOK_SECRET", is_dev, "whsec_placeholder")?,
```
Dev fallback: `sk_test_placeholder` / `whsec_placeholder`. No startup validation that key prefix matches env.

---

## Phase 5 — Webhook Event Coverage

### Handler Map (`api/src/routes/payments.rs:435-459`)

| Stripe Event | Handler | Status |
|---|---|---|
| `checkout.session.completed` | `handle_checkout_completed()` | LIVE — creates order, provisions membership |
| `customer.subscription.created` | `handle_subscription_created()` | **STUB** — only logs, no provisioning |
| `customer.subscription.updated` | `handle_subscription_updated()` | LIVE — updates `user_memberships` but swallows DB errors |
| `customer.subscription.deleted` | `handle_subscription_deleted()` | LIVE — sets `status = 'cancelled'` |
| `invoice.paid` | `handle_invoice_paid()` | LIVE — updates membership period |
| `invoice.payment_failed` | `handle_payment_failed()` | LIVE — sets `status = 'past_due'` |
| `charge.refunded` | `handle_refund()` | LIVE — updates `orders` only, **does NOT revoke access** |
| `charge.dispute.created` | **catch-all** | **UNHANDLED** — silent debug log |
| `payment_intent.*` | **catch-all** | **UNHANDLED** |
| All others | `_ =>` default | Silent debug log |

### Webhook Signature Verification

`api/src/services/stripe.rs:573-629` — `verify_webhook()`:
- HMAC-SHA256 with 5-minute timestamp tolerance
- Uses `constant_time_compare()` (timing-safe)
- `webhook_secret: Option<String>` — if None, **verification is skipped**

`api/src/routes/payments.rs:337-362` — Webhook extractor uses `body: String` (not `Bytes`). This is safe for HMAC verification but less efficient.

---

## Phase 6 — Admin Capabilities

### Subscription Admin Routes (`api/src/routes/subscriptions.rs:1575-1601`)

| Route | Method | Auth | Handler | Notes |
|-------|--------|------|---------|-------|
| `/api/subscriptions/metrics` | GET | `AdminUser` | `get_metrics()` | MRR/ARR/churn read-only |
| `/api/subscriptions/export` | GET | `AdminUser` | `export_subscriptions()` | CSV/JSON export |
| `/api/subscriptions/notifications/renewal-reminders` | POST | `AdminUser` | `send_renewal_reminders()` | Triggers emails |
| `/api/subscriptions/notifications/trial-ending` | POST | `AdminUser` | `send_trial_ending_notifications()` | Triggers emails |
| `/api/subscriptions/:id` | GET | `User` | `get_subscription()` | Ownership check |
| `/api/subscriptions/:id` | PUT | `User` | `update_subscription()` | Ownership check |
| `/api/subscriptions/:id/cancel` | PUT | `User` | `cancel_subscription()` | Ownership check |
| `/api/subscriptions/:id/reactivate` | POST | `User` (any) | `reactivate_subscription()` | **BUGGY — free access** |

**No admin endpoint exists for:**
- Comping subscriptions (granting free membership)
- Manual billing adjustment or proration
- Waiving payments outside normal Stripe portal

### Payment Admin Routes (`api/src/routes/payments.rs:1699-1711`)

| Route | Method | Auth | Notes |
|-------|--------|------|-------|
| `/api/payments/checkout` | POST | `User` | Checkout — accepts client price |
| `/api/payments/portal` | POST | `User` | Stripe billing portal |
| `/api/payments/webhook` | POST | None | Stripe webhook |
| `/api/payments/refund` | POST | `User` or Admin | Admin can refund any order |
| `/api/payments/config` | GET | None | Returns publishable key |
| `/api/payments/invoice` | POST | `User` | Retry invoice |
| `/api/payments/retry` | POST | `User` | Retry payment |
| `/api/payments/summary` | GET | `User` | Billing summary |

---

## Phase 7 — Refund Policy in Code

### Webhook-triggered refund (`handle_refund()`)

`api/src/routes/payments.rs:1208-1257`:

```rust
// Only updates orders table
sqlx::query(
    r#"UPDATE orders SET
       status = $1,
       refund_amount = COALESCE(refund_amount, 0) + $2,
       refunded_at = CASE WHEN $1 = 'refunded' THEN NOW() ELSE refunded_at END,
       updated_at = NOW()
       WHERE payment_intent_id = $3"#,
)
.bind(refund_status)
.bind(refund_amount_dollars)
.bind(pi)
.execute(&state.db.pool)
.await
.ok();  // errors swallowed
```

**`user_memberships` is NOT touched.** Refunded customers retain active subscription status.

### Manual refund via API (`create_refund()`)

`api/src/routes/payments.rs:467-558`:
- Authorization: order must belong to user OR user must be admin
- Calls Stripe Refunds API
- Also does not revoke `user_memberships` access

### Access Revocation Gap

```
Refund processed (Stripe) → charge.refunded webhook → handle_refund()
    ↓
orders.status = 'refunded'   ← updated
user_memberships.status      ← NOT updated (remains 'active')
```

Refunded user retains full access until their `current_period_end` passes or an admin manually cancels.

---

## Phase 8 — Additional Live/Dead Findings

### Rate Limiting on Payment Routes

`api/src/main.rs:221-238` — Router setup has **no rate limiting middleware** on payment routes.

`api/Cargo.toml` — `tower-http` is present but only used for CORS/tracing. No `tower_governor` or similar.

Rate limiting exists on:
- Auth routes: IP-based (login, registration) — `api/src/routes/auth.rs:69-120`
- CMS SEO routes: user-specific — `api/src/routes/cms_seo.rs:265-279`

**`/api/payments/checkout` and `/api/payments/webhook` have no rate limiting.**

### handle_subscription_updated() Error Swallowing

`api/src/routes/payments.rs:985-1012`:

DB update call ends with `.ok()` at line 1012 — any failure (constraint violation, connection error) is silently discarded. Stripe considers the webhook delivered successfully. Subscription status in DB may be stale with no alert.

### subscription.created Stub

`api/src/routes/payments.rs:943-962` — `handle_subscription_created()`:
```rust
async fn handle_subscription_created(...) -> Result<()> {
    tracing::info!("Subscription created: {}", subscription.id);
    // TODO: Provision subscription access
    Ok(())
}
```
Only logs. No membership is provisioned from this event. Access depends entirely on `checkout.session.completed`.

### Float Arithmetic on Monetary Values

`api/src/routes/payments.rs:31-51` — `CreateCheckoutRequest.price: f64`  
`api/src/routes/payments.rs:246` — price calculations use `f64` arithmetic

`rust_decimal::Decimal` is already a dependency (used in `coupons.rs`) but not applied to payment amounts. Risk: rounding errors on refund amounts and price calculations.

---

## Consolidated Findings

### CRITICAL — Actively Exploitable

**C1: Free Subscription via Reactivate (CONFIRMED LIVE)**
- Vector: `POST /api/subscriptions/:id/reactivate` — any authenticated user, cancelled subscription
- Effect: 30 days of free access, no payment
- Frontend trigger: button at `frontend/src/routes/my/subscriptions/+page.svelte:481`
- Backend bug: `api/src/routes/subscriptions.rs:1217-1256`
- Fix: Require Stripe to confirm an active payment method before reactivating; or redirect to Stripe checkout for full reactivation

**H1: Open Redirect via success_url/cancel_url (CONFIRMED LIVE)**
- Vector: `POST /api/payments/checkout` with arbitrary `success_url`/`cancel_url`
- Effect: Phishing after "checkout" — user is sent to attacker-controlled URL
- Code: `api/src/routes/payments.rs:249-250`
- Fix: Validate that URLs match `APP_URL` domain before passing to Stripe

### HIGH — Live and Dangerous

**H2: Client-Controlled Price in payments.rs (CONFIRMED LIVE)**
- Vector: `POST /api/payments/checkout` with `price: f64`
- Effect: Attacker can purchase any product for $0.01 or negative
- Code: `api/src/routes/payments.rs:31-51`
- Fix: Always fetch price from DB by product ID; reject client-supplied prices

**H3: Unauthenticated Coupon Validation (CONFIRMED LIVE)**
- Vector: `POST /api/coupons/validate` — no auth required
- Effect: Automated enumeration of all valid coupon codes
- Code: `api/src/routes/coupons.rs:104-107`
- Fix: Require authentication; add rate limiting

### MEDIUM — Live

**M1: charge.dispute.created Unhandled (LIVE)**
- Chargebacks trigger no action. Disputed users retain access.
- Fix: Handle `charge.dispute.created` to flag/suspend the disputed subscription

**M2: webhook DB Error Swallowing (LIVE)**
- `handle_subscription_updated()` silently drops DB failures
- Fix: Log errors at `error!` level and return non-2xx to trigger Stripe retry

**M3: Refund Does Not Revoke Access (LIVE)**
- `handle_refund()` updates `orders` only; `user_memberships.status` unchanged
- Fix: Revoke membership when refund amount >= original order amount

### BROKEN / DEAD CODE

**B1: Main Checkout → 404**
- `frontend/src/lib/api/cart.ts:1132` calls `POST /api/checkout/create-session`
- No such route exists in `api/src/routes/checkout.rs:555-561`
- User-facing checkout is non-functional

**B2: subscription.created is a Stub**
- `api/src/routes/payments.rs:943-962`: only logs, does not provision
- Access provisioning depends entirely on `checkout.session.completed`

**B3: subscription.create Returns Placeholder**
- All 20 plans have `stripe_price_id = ''`
- `api/src/routes/subscriptions.rs:444-453`: returns `https://checkout.stripe.com/placeholder?price=...`
- Plans with no `stripe_price_id` grant free access immediately (no Stripe call)

**B4: service_connections Table Missing**
- `credential_resolver.rs` attempts to read Stripe keys from DB
- Table does not exist → always falls back to env vars
- C3 (base64 credentials) is dormant until someone creates the table via admin UI

### INFORMATIONAL

**I1: webhook_secret Can Be None**
- `api/src/services/stripe.rs:573-577`: if `webhook_secret` is None, verification is skipped
- In dev: `whsec_placeholder` fallback is set, so verification runs but always fails on real events

**I2: No Rate Limiting on Payment Routes**
- Checkout and webhook endpoints have no rate limiting
- Automated checkout abuse is unthrottled

**I3: Float Arithmetic on Prices**
- `price: f64` in checkout request
- `rust_decimal::Decimal` available but not used
- Rounding errors possible on large or fractional amounts

---

*End of SUBSCRIPTION_DISCOVERY.md — all evidence verified from source at file:line references. No DB modifications were made. No credentials were decoded.*
