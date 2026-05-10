# SUBSCRIPTION_AUDIT.md
## Revolution Trading Pros — Payments & Billing Security Audit
**Date:** 2026-04-28  
**Auditor:** Adversarial payments audit (cold, read-only)  
**Scope:** Stripe integration, checkout, subscriptions, webhooks, frontend billing  
**Status:** READ-ONLY — no code was modified

---

## 1. EXECUTIVE SUMMARY

**Ready for paying customers? NO — 3 critical blockers must be fixed first.**

### Top Critical Findings

| # | Finding | Severity |
|---|---------|----------|
| C1 | **Free subscription via `reactivate_subscription`** — any user can reactivate a previously-cancelled membership without going through Stripe at all. Direct DB write sets `status = 'active'` with no payment verification. | Critical |
| C2 | **Webhook body read as `String`, not raw bytes** — Stripe signature verification computes HMAC over the raw UTF-8 bytes. Axum's `String` extractor works for ASCII payloads but is technically wrong; Stripe specifies raw bytes. More critically: the `signed_payload` is computed via `String::from_utf8_lossy(payload)`, meaning a non-UTF-8 byte in the webhook body causes silent replacement (`U+FFFD`) which invalidates the HMAC — an attacker could craft a body where lossy decoding changes the signature input. | Critical |
| C3 | **Stripe secret key stored with base64-only "encryption"** in the `service_connections` DB table. `encrypt_credentials()` in `connections.rs` applies `STANDARD.encode()` (base64) with a comment: *"In production, use proper AES-256-GCM encryption"*. Base64 is not encryption. Anyone with DB read access has the Stripe secret key in plaintext. | Critical |
| H1 | **Client-controlled `success_url` and `cancel_url`** accepted verbatim from the request body in both `payments.rs` and `checkout.rs`. Allows open redirect via Stripe checkout. | High |
| H2 | **No webhook idempotency** — no `webhook_events` table, no event-ID deduplication. Stripe retries webhooks on non-2xx; double-processing a `checkout.session.completed` event grants access twice. | High |
| H3 | **Coupon validate endpoint (`POST /api/coupons/validate`) is unauthenticated** — anyone can enumerate valid coupon codes without logging in. The endpoint reveals whether a code exists, its discount type, and value. | High |
| H4 | **`reactivate_subscription` bypasses Stripe entirely** for fully cancelled subscriptions — sets `status = 'active'` with a synthetic 30-day period with no payment collected. | High (same root as C1) |
| H5 | **Stripe credentials "encrypted" with base64** — see C3. | High |
| M1 | `charge.dispute.created` webhook event is NOT handled — disputes silently produce no app-side action (no admin notification, no logging). Chargebacks will go unnoticed. | Medium |
| M2 | **`handle_subscription_updated` silently swallows DB errors** with `.ok()` — if the DB write fails, the subscription state in-app diverges from Stripe with no alert. | Medium |
| M3 | **Floating-point money arithmetic throughout** — `price: f64`, `subtotal += item.price * quantity as f64`. Floating-point is unsuitable for monetary calculations; cent rounding errors can accumulate. | Medium |
| M4 | No reconciliation mechanism — no cron job, no script to compare Stripe subscription state against `user_memberships`. Drift is invisible. | Medium |
| M5 | `calculate_tax` endpoint in checkout.rs is **unauthenticated** and returns hardcoded tax rates (not integrated with any tax service). Minor exposure but technically an info endpoint without auth. | Low |

---

## 2. FINDINGS BY SEVERITY

### CRITICAL

---

#### C1 — Free Subscription via `reactivate_subscription`
**File:** [api/src/routes/subscriptions.rs:1179-1258](api/src/routes/subscriptions.rs#L1179)  
**Severity:** Critical  

```rust
// api/src/routes/subscriptions.rs:1224-1256
// For fully cancelled subscriptions, need to re-subscribe
// This would typically involve creating a new Stripe subscription
// For now, just reactivate it directly
let now = chrono::Utc::now().naive_utc();
let period_end = now + chrono::Duration::days(30); // Default to 30 days

sqlx::query(
    r#"
    UPDATE user_memberships
    SET status = 'active',
        cancelled_at = NULL,
        cancel_at_period_end = false,
        current_period_start = $1,
        current_period_end = $2,
        updated_at = NOW()
    WHERE id = $3
    "#,
)
```

**Exploit path:** A user subscribes (pays once), cancels, then calls `POST /api/subscriptions/:id/reactivate`. The handler checks `WHERE id = $1 AND user_id = $2` (ownership enforced), but then — for any subscription that is NOT in `cancel_at_period_end` state — directly writes `status = 'active'` with a new 30-day period. No Stripe call is made. No payment is collected. The user gets 30 days of access for free and can repeat indefinitely.

**What passing requires:** For fully cancelled subscriptions, redirect to a new Stripe checkout session. Never set `status = 'active'` without Stripe confirming a successful charge.

---

#### C2 — Webhook Body Decoded as String (HMAC Integrity Risk)
**File:** [api/src/routes/payments.rs:337-411](api/src/routes/payments.rs#L337)  
**Severity:** Critical  

```rust
// payments.rs:337-341
async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,       // ← String, not Bytes
) -> ...
```

```rust
// payments.rs:363
match stripe_for_webhook.verify_webhook(body.as_bytes(), signature) {
```

```rust
// stripe.rs:613
let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
```

**Problem 1:** `body.as_bytes()` passes the UTF-8 bytes of the `String`, which Axum decoded from the raw bytes. For pure ASCII payloads this is equivalent, but the verify function then does `String::from_utf8_lossy(payload)` — which is a no-op on bytes already validated as UTF-8, making it safe for the normal case.

**Problem 2 (actual risk):** The `signed_payload` string is reconstructed as `format!("{}.{}", timestamp, String::from_utf8_lossy(payload))`. If an attacker can cause the request body to contain characters that differ after UTF-8 round-tripping (e.g., BOM characters, certain encodings), the HMAC input diverges from what Stripe signed. This is a PARTIAL finding — the normal Stripe webhook path is safe, but the code's approach is fragile and non-standard. **Stripe's libraries always use raw bytes.**

**More importantly:** Axum's `String` extractor will **reject** (400) any non-UTF-8 request body before the handler is even called. This means a Stripe webhook with a non-UTF-8 character in JSON metadata would result in a 400, causing Stripe to retry infinitely. In practice, Stripe always sends UTF-8 JSON, so this is low-probability — but it's still wrong.

**What passing requires:** Use `axum::body::Bytes` extractor. Pass the raw bytes directly to `verify_webhook` and use them directly for signature verification, avoiding any string conversion.

---

#### C3 — Stripe Secret Key "Encrypted" with Base64
**File:** [api/src/routes/connections.rs:112-125](api/src/routes/connections.rs#L112)  
**Severity:** Critical  

```rust
// connections.rs:112-118
/// Encrypt credentials for storage (uses base64 encoding as placeholder - in production use AES-256)
fn encrypt_credentials(credentials: &HashMap<String, String>) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};

    // In production, use proper AES-256-GCM encryption with a key from env
    // For now, we use base64 encoding as a placeholder
    STANDARD.encode(json.as_bytes())
```

```rust
// credential_resolver.rs:256-260
fn decrypt_credentials(encrypted: &str) -> HashMap<String, String> {
    match STANDARD.decode(encrypted) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}
```

**Exploit path:** An attacker who gains read access to the `service_connections` DB table (via SQLi, DB backup leak, misconfigured DB permissions, or insider threat) obtains the Stripe secret key by base64-decoding the `credentials_encrypted` field. There is NO encryption — base64 is encoding, not encryption. With the Stripe secret key, the attacker can:
- Create arbitrary charges
- Issue refunds
- Read all customer data
- Cancel all subscriptions

**What passing requires:** Use AES-256-GCM with a key stored in an env var (`CREDENTIALS_ENCRYPTION_KEY`). The `encrypt_credentials` function must perform real symmetric encryption.

---

### HIGH

---

#### H1 — Client-Controlled `success_url` and `cancel_url` (Open Redirect)
**File:** [api/src/routes/payments.rs:34-36, 249-250](api/src/routes/payments.rs#L34)  
**File:** [api/src/routes/checkout.rs:29-35, 341-346](api/src/routes/checkout.rs#L29)  
**Severity:** High  

```rust
// payments.rs:34-36 (CreateCheckoutRequest)
pub success_url: String,
pub cancel_url: String,
```

```rust
// payments.rs:249-250
success_url: format!("{}?order={}", input.success_url, order_number),
cancel_url: input.cancel_url,
```

```rust
// checkout.rs:341-346
let success_url = input
    .success_url
    .unwrap_or_else(|| format!("{}/checkout/success", state.config.app_url));
let cancel_url = input
    .cancel_url
    .unwrap_or_else(|| format!("{}/checkout/cancel", state.config.app_url));
```

**Exploit path:** An authenticated user sends `POST /api/payments/checkout` with `"success_url": "https://evil.com/capture"`. Stripe's checkout session is created with this attacker-controlled URL. When payment completes, Stripe redirects the user to `https://evil.com/capture?order=ORD-...`, an attacker-controlled phishing page. Note that the `checkout.rs` version provides a fallback to `state.config.app_url` when `success_url` is absent, but does not validate it when present.

**What passing requires:** Never accept redirect URLs from client input. Build `success_url` and `cancel_url` server-side from `state.config.app_url`. If the client must influence the post-checkout destination, accept only a path (e.g., `/checkout/success`) and prepend the trusted domain server-side.

---

#### H2 — No Webhook Idempotency
**File:** [api/src/routes/payments.rs:435-462](api/src/routes/payments.rs#L435)  
**Severity:** High  

```rust
// payments.rs:435-461
match event.event_type.as_str() {
    "checkout.session.completed" => {
        handle_checkout_completed(&state, &event).await?;
    }
    ...
}
```

There is no `webhook_events` table, no check for `event.id` before processing, and no DB constraint preventing duplicate processing. Stripe retries webhooks on non-2xx responses or network timeouts. The `handle_checkout_completed` handler uses `INSERT ... ON CONFLICT DO UPDATE` for `user_memberships` (deduplication at that layer), and `UPDATE orders SET status = 'completed'` (idempotent for orders). However, the **access granting loop** at payments.rs:724-899 uses `INSERT INTO user_products ... ON CONFLICT DO UPDATE` (safe), `INSERT INTO user_course_enrollments ... ON CONFLICT DO UPDATE` (safe), and `INSERT INTO user_indicator_access ... ON CONFLICT DO UPDATE` (safe). The coupon usage increment at payments.rs:684-691 is **NOT idempotent** — a second webhook fires an `UPDATE coupons SET usage_count = usage_count + 1` again, double-incrementing the counter.

**Exploit path for double-access:** If a 500 causes Stripe to retry `checkout.session.completed` and the first run partially succeeded, the retry may create a second `user_memberships` row for a different plan variant, or double-increment coupon usage. The `ON CONFLICT` clauses provide partial protection, but the code is not atomically idempotent end-to-end.

**What passing requires:** Create a `webhook_events` table with a `UNIQUE(event_id)` constraint. At the top of the webhook handler, `INSERT INTO webhook_events (event_id, event_type) VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING id`. If the insert returns nothing, the event was already processed — return 200 immediately.

---

#### H3 — Unauthenticated Coupon Enumeration
**File:** [api/src/routes/coupons.rs:104-107, 742-745](api/src/routes/coupons.rs#L104)  
**Severity:** High  

```rust
// coupons.rs:104-107 — no User or AdminUser extractor
async fn validate_coupon(
    State(state): State<AppState>,
    Json(input): Json<ValidateCouponRequest>,
) -> ...
```

```rust
// coupons.rs:742-745
pub fn router() -> Router<AppState> {
    ...
    .route("/validate", post(validate_coupon))  // No auth required
```

**Exploit path:** An unauthenticated attacker brute-forces `POST /api/coupons/validate` with a wordlist (PROMO2024, WELCOME10, etc.). Each response reveals whether the code is valid, expired, or usage-limited. No rate limiting was observed in the router setup. This leaks coupon inventory to adversarial users.

**What passing requires:** Add `user: User` extractor to `validate_coupon` (authentication) and/or add rate limiting. The discount amount calculation should only be returned to authenticated users completing a real checkout, not via an unauthenticated probe endpoint.

---

#### H4 — `reactivate_subscription` Bypasses Stripe (Full Cancellations)
**File:** [api/src/routes/subscriptions.rs:1179-1258](api/src/routes/subscriptions.rs#L1179)  
This is the same root cause as C1. See C1 above.

---

#### H5 — Stripe API Keys "Encrypted" with Base64
Same as C3. Listed separately because it is independently exploitable via DB access.

---

### MEDIUM

---

#### M1 — `charge.dispute.created` Not Handled
**File:** [api/src/routes/payments.rs:435-461](api/src/routes/payments.rs#L435)  
**Severity:** Medium  

```rust
// payments.rs:457-459
_ => {
    tracing::debug!("Unhandled webhook event: {}", event.event_type);
}
```

`charge.dispute.created` falls through to the catch-all. No admin notification, no logging to an audit table, no automated response. Stripe disputes have a response deadline (typically 7-21 days). Unresponded disputes result in automatic chargebacks (money lost + chargeback fee).

**What passing requires:** Add a handler for `charge.dispute.created` that at minimum logs to an audit table and sends an admin email alert with the dispute ID, charge ID, and response deadline.

---

#### M2 — Silent DB Error Swallowing in `handle_subscription_updated`
**File:** [api/src/routes/payments.rs:985-1012](api/src/routes/payments.rs#L985)  
**Severity:** Medium  

```rust
// payments.rs:1010-1012
.execute(&state.db.pool)
.await
.ok();   // ← silently swallows ALL DB errors
```

If the `UPDATE user_memberships SET status = $1 ...` fails (deadlock, connection error, constraint violation), the subscription state in the app diverges from Stripe with no log, no alert, and the webhook still returns 200 (telling Stripe "processed successfully"). The subscription may show active in Stripe but cancelled in-app, or vice versa.

**What passing requires:** Use `if let Err(e) = ... { tracing::error!(...); }` pattern (as done in `handle_checkout_completed` after the April 2026 fixes). Do not silently discard DB errors in payment state mutations.

---

#### M3 — Floating-Point Arithmetic on Money
**Files:** [api/src/routes/payments.rs:47, 104, 108](api/src/routes/payments.rs#L47), [api/src/routes/checkout.rs:100-101](api/src/routes/checkout.rs#L100)  
**Severity:** Medium  

```rust
// payments.rs:47
pub price: f64,

// payments.rs:104, 108
let mut subtotal = 0.0_f64;
let item_total = item.price * item.quantity as f64;
subtotal += item_total;
```

```rust
// payments.rs:123
amount: (item.price * 100.0).round() as i64,
```

IEEE 754 double-precision floating point cannot represent all decimal values exactly. Example: `$29.99 * 3 = 89.97000000000001` in floating point. The `.round()` call before converting to cents mitigates catastrophic errors, but accumulated float arithmetic across multiple items and discount calculations can produce incorrect cent amounts. The `membership_plans.price` column is `DECIMAL(10,2)` in the DB but is cast to `FLOAT8` throughout the Rust code for "SQLx compatibility."

**What passing requires:** Use `rust_decimal::Decimal` (already imported in `coupons.rs`) for all monetary calculations. Store prices internally as cents (`i64`) and convert only at display boundaries. Stop casting `DECIMAL` to `FLOAT8` at the query level.

---

#### M4 — No Stripe State Reconciliation
**Severity:** Medium  

There is no cron job, admin script, or reconcile endpoint that compares `user_memberships` in the DB against Stripe subscription state. Webhooks can be missed (network failure, server downtime, Stripe delivery failure). A user whose subscription was cancelled in Stripe may continue to have `status = 'active'` in the DB indefinitely.

**What passing requires:** A periodic job (daily) that: (1) lists all `user_memberships` with `status = 'active'` and a `stripe_subscription_id`, (2) retrieves each subscription from Stripe, (3) compares status, (4) flags discrepancies for admin review.

---

### LOW

---

#### L1 — `calculate_tax` Endpoint is Unauthenticated
**File:** [api/src/routes/checkout.rs:445-468](api/src/routes/checkout.rs#L445)  
**Severity:** Low  

```rust
async fn calculate_tax(
    State(_state): State<AppState>,
    Json(input): Json<CalculateTaxRequest>,
) -> ...
```

Hardcoded tax rates (CA=7.25%, NY=8%, TX=6.25%, GB=20%, DE=19%). No real tax service integration. Unauthenticated access is low-risk for this specific endpoint since it returns only static rates. However, the endpoint should be authenticated to prevent abuse as an info endpoint and because the rates are approximations, not production-ready.

---

#### L2 — `create_subscription` Creates Free Memberships for Plans Without Stripe Price IDs
**File:** [api/src/routes/subscriptions.rs:444-528](api/src/routes/subscriptions.rs#L444)  
**Severity:** Low (partially mitigated)  

```rust
// subscriptions.rs:445-452
if let Some(ref stripe_price_id) = plan.stripe_price_id {
    // TODO: Create Stripe checkout session
    // For now, return a placeholder
    return Ok(Json(json!({
        "checkout_url": format!("https://checkout.stripe.com/placeholder?price={}", stripe_price_id),
        ...
    })));
}

// Falls through to: create subscription directly (no payment)
```

Plans that lack a `stripe_price_id` result in a direct DB insert with `payment_provider = 'free'`, regardless of whether the plan is actually free. If a plan exists in DB without its `stripe_price_id` populated (migration gap, admin error), users can subscribe for free.

---

## 3. LAYER 1 CHECKLIST RESULTS

| # | Check | Status | Evidence |
|---|-------|--------|---------|
| 1 | Webhook signature verification | PASS | `stripe.rs:573-629` — HMAC-SHA256 with timestamp check |
| 2 | Sig verification not bypassable in prod | PASS | `payments.rs:386-387` — `is_dev = env_name.starts_with("dev")` closes in prod |
| 3 | Webhook reads raw body | **PARTIAL** | `payments.rs:340` uses `body: String`, not `Bytes`; `stripe.rs:613` uses `String::from_utf8_lossy` — technically incorrect but safe for Stripe's UTF-8 payloads in practice |
| 4 | Webhook idempotency | **FAIL** | No `webhook_events` table, no event-ID dedup; coupon counter double-increments on retry |
| 5 | Webhook timeout handling | PARTIAL | Email sent inline in `handle_payment_failed`; no async queue for heavy work |
| 6 | Webhook event filtering | PASS | Unknown events hit `_ =>` catch-all and return 200 |
| 7 | Server-side price validation | **FAIL** | `payments.rs:47` accepts `price: f64` from client; `checkout.rs:80-97` reads price FROM DB (PASS for checkout.rs). `payments.rs` create_checkout uses CLIENT price for ad-hoc Stripe sessions |
| 8 | User identity binding | PASS | `metadata["user_id"]` set from `user.id` (authenticated), not from request body |
| 9 | Coupon validation server-side | PASS | Both `payments.rs:145-175` and `checkout.rs:167-238` query DB for coupon. But validate endpoint is unauthenticated (H3) |
| 10 | Checkout session URL is single-use | N/A | Stripe enforces this |
| 11 | Success/cancel URLs validated | **FAIL** | `payments.rs:249-250` passes client-supplied URLs verbatim to Stripe |
| 12 | Promo code abuse prevention | PARTIAL | `max_uses` checked, but per-user enforcement absent; no check for same user re-using same coupon |
| 13 | Subscription.created provisions access | PARTIAL | `handle_subscription_created` at `payments.rs:943-962` only logs — does NOT provision access. Access is provisioned only in `checkout.session.completed`. If webhook ordering differs, access may never be granted |
| 14 | Subscription.deleted revokes access | PASS | `payments.rs:1036-1038` sets `status = 'cancelled'` on `stripe_subscription_id` |
| 15 | Trial expiration handling | NOT IMPLEMENTED | No trial-end webhook handler; `customer.subscription.trial_will_end` not subscribed |
| 16 | Failed payment handling | PARTIAL | Sets `past_due` + grace period — good. But grace period expiry never triggers access revocation (no cron; `status` stays `past_due`) |
| 17 | Refund handling | PARTIAL | `handle_refund` updates `orders.status` to `refunded`/`partial_refund`. Does NOT revoke membership/access |
| 18 | Dispute handling | **FAIL** | `charge.dispute.created` is not in the match — falls through to debug log only |
| 19 | Race condition: webhook before redirect | PARTIAL | `ON CONFLICT DO UPDATE` in membership insert helps; no frontend polling seen |
| 20 | Race: subscription change during request | PARTIAL | No transactions wrap subscription state changes |
| 21 | Cancel requires auth + ownership | PASS | `subscriptions.rs:540` — `WHERE id = $1 AND user_id = $2` |
| 22 | Customer portal bound to auth user | PASS | `payments.rs:303-317` — fetches `stripe_customer_id WHERE user_id = $1` |
| 23 | Invoice/receipt access | PASS | `payments.rs:1319-1326` — checks `order.user_id != user.id` |
| 24 | Payment method endpoints ownership | PASS | `user.rs:1204` — verifies `payment_method.customer == customer_id` |
| 25 | Admin-only billing endpoints | PASS | Refund endpoint (`payments.rs:499-506`) checks admin role. `subscriptions_admin.rs` uses `AdminUser` throughout |
| 26 | DB matches Stripe reconciliation | **FAIL** | No reconciliation mechanism exists |
| 27 | Stripe customer ID persisted | PARTIAL | Stored in `user_memberships.stripe_customer_id`, but not in `users` table. Multiple memberships could each have different customer IDs |
| 28 | Plan/price IDs referenced, not duplicated | PASS | `membership_plans.stripe_price_id` stores string reference |
| 29 | Currency handling | PASS | Single currency (`USD`) hardcoded throughout |
| 30 | Decimal/cents handling | **FAIL** | Monetary arithmetic uses `f64` throughout; `DECIMAL` cast to `FLOAT8` in queries |
| 31 | Stripe.js loaded from official URL | PASS | `add-payment-method/+page.svelte:29` — `https://js.stripe.com/v3/` |
| 32 | Publishable key only on frontend | PASS | `env.d.ts` declares `STRIPE_SECRET_KEY` as `$env/static/private` (server only). No usage in frontend routes confirmed |
| 33 | Card data never touches server | PASS | No `card_number`, `cvc`, `exp_month` in request structs. Stripe Elements / Checkout used |
| 34 | 3D Secure / SCA support | PASS | Stripe Checkout handles SCA automatically |
| 35 | Payment events logged | PARTIAL | `tracing::info!` for events; no dedicated `payment_events` audit table |
| 36 | Failed webhook attempts logged | PARTIAL | `payments.rs:369-404` logs signature failures via `tracing::error!`. No persistent log to DB |
| 37 | Reconciliation job | **FAIL** | Does not exist |
| 38 | Refund policy enforcement | NOT IMPLEMENTED | No refund deadline enforcement in code |
| 39 | Proration handling | PARTIAL | `subscriptions.rs:979-1007` — custom proration math using `f64`. Not integrated with Stripe's proration API |
| 40 | Tax handling | NOT IMPLEMENTED | `checkout.rs:241` — `let tax = 0.0_f64`. Hardcoded rates in `calculate_tax` are not applied to actual charges |

---

## 4. ADVERSARIAL SCENARIOS

### A. Free Subscription via Client-Side Price Manipulation
**Exploitable: PARTIAL**

In `payments.rs` `create_checkout` (POST /api/payments/checkout), the request accepts:
```rust
// payments.rs:46-51
pub price: f64,        // ← from client
pub quantity: i32,     // ← from client
pub is_subscription: bool,
pub interval: Option<String>,
```

And uses `price_id: None` (ad-hoc pricing):
```rust
// payments.rs:119-130
line_items.push(LineItem {
    price_id: None, // We create ad-hoc prices
    ...
    amount: (item.price * 100.0).round() as i64,
```

An attacker can submit `"price": 0.01` for a subscription plan and pay $0.01/month instead of the real price. Stripe will happily charge $0.01/month and send a webhook. The webhook reads `plan_id` from the order, so they'd get access to whatever plan was ordered. **This is exploitable for the `payments.rs` checkout endpoint.**

The `checkout.rs` endpoint fetches price FROM DB (safe) but also sends the URL-supplied `success_url` to Stripe.

**Exploit:** `POST /api/payments/checkout` with `{"items": [{"plan_id": 5, "name": "Pro Plan", "price": 0.01, "quantity": 1, "is_subscription": true}], "success_url": "...", "cancel_url": "..."}` → pay $0.01/month → receive full subscription access.

---

### B. Replay Attacks on Webhooks
**Exploitable: PARTIAL**

The 5-minute timestamp tolerance in `stripe.rs:601-609` prevents simple replays beyond 5 minutes. However, within the 5-minute window, the same event payload can be re-submitted. More critically: **Stripe itself retries webhooks**. Without idempotency tracking (H2), receiving the same `checkout.session.completed` twice (legitimate Stripe retry) causes `usage_count` to be incremented twice for coupons and potentially other effects.

For the signature-verified replay attack (outside 5 minutes): NOT exploitable due to timestamp check.
For Stripe-legitimate-retry double-processing: **EXPLOITABLE** for coupon counter.

---

### C. Coupon Farming
**Exploitable: YES (per-user not enforced)**

There is no check preventing the same user from using the same coupon code on multiple orders. The `max_uses` / `usage_count` fields are global counters. A user can:
1. Subscribe using coupon WELCOME50 (50% off)
2. Cancel subscription
3. Re-subscribe using WELCOME50 again (still valid until global limit reached)
4. Repeat

No `coupon_redemptions` table tracks which user redeemed which coupon. The validate endpoint (H3) is also unauthenticated, allowing code enumeration.

---

### D. BOLA on Subscription Endpoints
**Not Exploitable (user endpoints properly scoped)**

`cancel_subscription` at `subscriptions.rs:540`: `WHERE id = $1 AND user_id = $2` ✓  
`get_subscription_by_id` at `subscriptions.rs:1618`: `WHERE id = $1 AND user_id = $2` ✓  
`get_my_subscriptions` at `subscriptions.rs:249`: `WHERE um.user_id = $1` ✓  
`delete_payment_method` at `user.rs:1204`: verifies `payment_method.customer == customer_id` ✓  
`generate_invoice` at `payments.rs:1319-1326`: checks `order.user_id != user.id` ✓  

Admin endpoints all use `AdminUser` extractor. **BOLA does not appear exploitable on the primary user billing endpoints.**

---

### E. Webhook Spoofing
**Not Exploitable (in production)**

```rust
// payments.rs:386-387
let is_dev = env_name.starts_with("dev");
if !is_dev {
    // rejects
}
```

Without `STRIPE_WEBHOOK_SECRET` in production, webhooks are rejected with 500. With the secret, HMAC verification is applied. Signature verification uses `constant_time_compare` (`stripe.rs:1123-1133`). **NOT exploitable in production assuming `STRIPE_WEBHOOK_SECRET` is set.**

---

### F. Race Condition: Double-Provisioning
**Partially Exploitable**

The `user_memberships` INSERT uses `ON CONFLICT (user_id, plan_id) DO UPDATE` (`payments.rs:657-675`). For the same user + plan, two simultaneous `checkout.session.completed` webhooks would: first wins the INSERT, second hits ON CONFLICT and UPDATEs. The membership is not doubled. However, the coupon `usage_count` increment at `payments.rs:684-691` is NOT idempotent — two concurrent webhooks could both execute `UPDATE coupons SET usage_count = usage_count + 1`. Result: usage count incremented by 2 instead of 1 (minor integrity issue, not a billing exploit).

---

### G. Subscription Active Despite Missed `payment_failed` Webhook
**Partially Exploitable**

If `invoice.payment_failed` is missed (network issue, server downtime), the subscription stays `active` in the DB even though Stripe cancelled it. This is the "missed webhook = free access" scenario. Without reconciliation (M4), this is a permanent gap. A user whose subscription lapses at Stripe will continue to have app access until the next `customer.subscription.deleted` webhook fires OR a reconciliation job runs.

---

### H. Refund Without Access Revocation
**Exploitable: YES**

`handle_refund` at `payments.rs:1208-1257` updates `orders.status` to `refunded` but does NOT:
- Cancel the associated `user_membership`
- Revoke course enrollment
- Revoke indicator access
- Revoke product ownership

```rust
// payments.rs:1230-1243
sqlx::query(
    r#"UPDATE orders SET
       status = $1,
       refund_amount = COALESCE(refund_amount, 0) + $2,
       ...
       WHERE payment_intent_id = $3"#,
```

An admin refunding via Stripe dashboard triggers `charge.refunded` → `handle_refund`. The user gets their money back but retains full access. This is a policy question (some businesses allow access through end of period), but there is no code enforcing any policy here — the decision was simply not made.

---

### I. Customer Portal Scope
**NOT directly exploitable via the API**

`create_portal` at `payments.rs:297-333` fetches `stripe_customer_id WHERE user_id = $1` (authenticated user). The Stripe Customer Portal is configured in the Stripe dashboard — the scope of what users can do (cancel, change plan) is controlled by Stripe portal configuration, not application code. **Cannot assess portal configuration without Stripe dashboard access.**

---

### J. Cancellation Timing / "First Month Free" Promo Exploit
**Exploitable: YES (via C1)**

Since `reactivate_subscription` (C1) restores `status = 'active'` without payment:
1. Subscribe with a first-month promo → pay
2. Cancel at end of period
3. Call `POST /api/subscriptions/:id/reactivate`
4. Get 30 more days of free access
5. Repeat indefinitely

Even without C1, coupon farming (Scenario C) allows a user to repeatedly apply a first-month discount.

---

### K. Stripe API Key Exposure
**Not found in code/git — but see C3**

Grep results confirm no `sk_live_` or `sk_test_` literals in frontend source. The env.d.ts declaration of `STRIPE_SECRET_KEY` in `$env/static/private` is a type declaration only — it does not expose the key to the browser. The key appears only in the API (Rust) process.

**However:** The "encryption" (base64) in the `service_connections` table means the key is effectively plaintext in the database (C3). Anyone with DB access has the secret key.

---

### L. Test Mode vs Live Mode Confusion
**NOT IMPLEMENTED — no startup check**

There is no code that verifies `sk_test_` keys are only used when `ENVIRONMENT != 'production'` or vice versa. The `config/mod.rs` check panics if `ENVIRONMENT=development` but `APP_URL` looks like production — this prevents accidental dev config in prod, but does NOT prevent a test Stripe key being used in production (different failure mode). A misconfigured deployment could use `sk_test_` keys in production — payments would succeed from Stripe's test perspective but no real money would flow.

**What passing requires:** At startup, if `environment == "production"`, assert that `stripe_secret_key.starts_with("sk_live_")`.

---

## 5. SURPRISES

### S1 — Two Parallel Checkout Flows Exist
`POST /api/payments/checkout` (payments.rs) and `POST /api/checkout/` (checkout.rs) both create Stripe checkout sessions, both create orders, both accept coupon codes. They have different validation behaviors:
- `payments.rs` accepts client-supplied `price: f64` (exploitable — see A)
- `checkout.rs` fetches prices from DB (safe)
- Neither validates `success_url`/`cancel_url`

The existence of two checkout flows creates a larger attack surface and maintenance burden. It's unclear which is the "canonical" path that the frontend actually uses.

### S2 — `handle_subscription_created` Does Nothing
```rust
// payments.rs:943-962
async fn handle_subscription_created(...) {
    let subscription = event.as_subscription()...;
    tracing::info!(..., "Subscription created");
    Ok(())
}
```

The `customer.subscription.created` webhook handler is a stub. It logs and returns. Access provisioning happens only in `checkout.session.completed`. This means: if a subscription is created directly in Stripe (admin action, or Stripe subscription API bypass), the app never provisions access.

### S3 — `credentials_encrypted` Is Base64-Encoded JSON (C3 Extension)
The column name says "encrypted" but it is not. This misleads future developers and auditors. The `service_connections` table is used to store live Stripe keys entered through the admin UI — these are production credentials.

### S4 — Reactivate Endpoint Has Unreachable Guard
```rust
// subscriptions.rs:1212-1221
if subscription.cancel_at_period_end.unwrap_or(false) && subscription.status == "active" {
    // unset cancel_at_period_end ...
    return Ok(...);
}
```

This guard checks `status == "active"` to handle "undo cancellation". But the function already checks `if subscription.status == "active" { return Err(BAD_REQUEST) }` at line 1204. So this `cancel_at_period_end` branch can NEVER be reached — `status == "active"` would have already returned an error. The logic is dead code and the actual cancellation undo path (using Stripe's API to undo `cancel_at_period_end`) is missing.

### S5 — `create_subscription` Placeholder Returns Fake Stripe URL
```rust
// subscriptions.rs:448-452
return Ok(Json(json!({
    "checkout_url": format!("https://checkout.stripe.com/placeholder?price={}", stripe_price_id),
    ...
})));
```

This is labeled "TODO" and returns a non-functional URL. If the frontend sends the user to this URL, it will 404 on Stripe. Plans that have `stripe_price_id` set will always return this placeholder, which means the subscription creation endpoint is essentially broken for Stripe-backed plans.

---

## 6. WHAT'S NOT IN SCOPE (Layer 2)

The following were noted but not audited:

- **Tax compliance** — EU VAT, US sales tax nexus, Stripe Tax configuration
- **Multi-currency** — only USD supported; no localization review
- **PCI DSS compliance** — card data handling at infrastructure level
- **Fraud detection** — no Stripe Radar rules review
- **Advanced dunning policy** — Stripe smart retries not reviewed
- **SCA / 3DS2 for custom Elements flows** — only Stripe Checkout reviewed (handles automatically)
- **Subscription pause/resume** — business logic not fully audited
- **Webhook delivery configuration** — Stripe dashboard endpoint configuration not accessible

---

## 7. RECONCILIATION GAPS (Prioritized)

| Priority | Gap | Impact |
|----------|-----|--------|
| 1 | Missed `customer.subscription.deleted` → user retains access | Active user with cancelled Stripe sub |
| 2 | Missed `invoice.payment_failed` → user retains access despite non-payment | Revenue loss |
| 3 | `reactivate_subscription` sets `status=active` without Stripe call | Free access |
| 4 | `customer.subscription.created` stub → direct Stripe subs not provisioned | User paid, no access |
| 5 | `charge.refunded` → access not revoked | User gets money back + keeps access |
| 6 | Grace period expiry never enforced | `past_due` memberships stay accessible forever |
| 7 | No periodic reconciliation job | Drift accumulates silently |

---

## 8. SUMMARY TABLE (TOP 10 FIXES)

| Priority | Finding | File | Fix |
|----------|---------|------|-----|
| 1 | C1: Free subscription via reactivate | subscriptions.rs:1224 | Require Stripe checkout for reactivation |
| 2 | A: Client-supplied price in payments.rs checkout | payments.rs:47 | Fetch price from DB by plan_id |
| 3 | C3: Base64 "encryption" of Stripe keys | connections.rs:113 | AES-256-GCM with env-var key |
| 4 | H1: Client-controlled success_url/cancel_url | payments.rs:249 | Build URLs server-side from app_url |
| 5 | H2: No webhook idempotency | payments.rs:435 | Add webhook_events table + dedup |
| 6 | H3: Unauthenticated coupon validate | coupons.rs:104 | Add User extractor |
| 7 | M1: dispute.created unhandled | payments.rs:457 | Add handler + admin alert |
| 8 | L: Test/live key startup check | config/mod.rs | Assert sk_live_ in production |
| 9 | M3: Float money arithmetic | payments.rs:47 | Migrate to rust_decimal::Decimal |
| 10 | M4: No reconciliation | (missing) | Add daily reconcile cron |

---

*Audit completed 2026-04-28. All findings verified against source. No code was modified.*
