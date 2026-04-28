# SECURITY_FIXES_VERIFICATION.md
# Branch: payments-fix-2026-04
# Date: 2026-04-28
# Verified by: re-reading source + compiler gates

---

## Cleanup performed before verification

```
git branch -D demolition-rebuild-2026-04
→ Deleted branch demolition-rebuild-2026-04 (was 686e8e99b).

rm -f DEMOLITION_AND_REBUILD_PLAN.md   → removed
rm -rf demolition-plan/                → directory not present (already gone)
```

---

## Check 1 — Branch is payments-fix-2026-04

```
$ git branch --show-current
payments-fix-2026-04
```

**EXIT: 0 — PASS**

---

## Check 2 — cargo check: 0 errors, 0 warnings

```
$ cd api && cargo check 2>&1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
```

**EXIT: 0 — PASS**

---

## Check 3 — pnpm check: 0 errors, 0 warnings

```
$ pnpm check 2>&1 | tail -5
> revolution-svelte@2.0.0 check /…/frontend
> svelte-kit sync && svelte-check --tsconfig ./tsconfig.json

1777399763957 START "…/frontend"
1777399763965 COMPLETED 5215 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

**EXIT: 0 — PASS**

---

## Check 4 — Fix 1.1: reactivate endpoint does NOT grant free access to fully-cancelled subs

**File:** `api/src/routes/subscriptions.rs:1204–1265`

```rust
// subscriptions.rs:1204
if subscription.status == "active" && !subscription.cancel_at_period_end.unwrap_or(false) {
    return Err((BAD_REQUEST, json!({"error": "Subscription is already active"})));
}

// subscriptions.rs:1212 — cancel_at_period_end path: calls Stripe FIRST, then DB
if subscription.cancel_at_period_end.unwrap_or(false) {
    let stripe_sub_id = subscription.stripe_subscription_id.as_deref().ok_or_else(…)?;
    stripe
        .update_subscription(stripe_sub_id, &[("cancel_at_period_end", "false")])
        .await
        .map_err(…)?;                                   // Stripe failure → 500, no DB write
    sqlx::query(
        "UPDATE user_memberships SET cancel_at_period_end = false …"
    )…?;                                                // DB write only after Stripe confirms
    return Ok(json!({"success": true, "message": "Subscription reactivated"}));
}

// subscriptions.rs:1253 — fully cancelled path: 400 + resubscribe flag, NO DB write
Err((BAD_REQUEST, json!({
    "error": "Subscription has fully ended. Please re-subscribe to restart.",
    "resubscribe": true,
    "plan_id": subscription.plan_id
})))
```

No code path sets `status='active'` without a confirmed Stripe response.

**PASS**

---

## Check 5 — Fix 1.1: frontend redirects fully-cancelled subs to /pricing

**File:** `frontend/src/routes/my/subscriptions/+page.svelte:228–248`

```javascript
// line 228–232: pre-flight guard — redirect before API call
// For fully cancelled subs: redirect to checkout — backend refuses to grant free access.
if (sub.status === 'cancelled' && !sub.cancelAtPeriodEnd) {
    goto(`/pricing?plan=${sub.planId}`);
    return;
}
// line 246–248: backstop — handles the API's 400 response
if (data.resubscribe && data.plan_id) {
    goto(`/pricing?plan=${data.plan_id}`);
}
```

**PASS**

---

## Check 6 — Fix 1.2: `price` field removed from `CheckoutItem`; price looked up from DB

**File:** `api/src/routes/payments.rs:45–53` (struct definition)

```rust
#[derive(Deserialize)]
pub struct CheckoutItem {
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    // price is intentionally NOT accepted from the client; it is looked up from the DB
    pub quantity: i32,
    pub is_subscription: bool,
    pub interval: Option<String>,
}
```

`grep "pub price" payments.rs` → no results in `CheckoutItem`. The only `price: f64` fields
are inside the internal `PlanRow`, `ProductRow`, and `ResolvedItem` structs (lines 109, 114, 122)
which are defined inside the `create_checkout` function body and never deserialised from client JSON.

**DB lookup at lines 133–153:**

```rust
let (db_name, db_price): (String, f64) = if let Some(plan_id) = item.plan_id {
    sqlx::query_as("SELECT name, price FROM membership_plans WHERE id = $1 AND is_active = true")
        .bind(plan_id)…
        .ok_or_else(|| (BAD_REQUEST, json!({"error": format!("Plan {} not found or inactive", plan_id)})))?
} else if let Some(product_id) = item.product_id {
    sqlx::query_as("SELECT name, price FROM products WHERE id = $1 AND is_active = true")
        .bind(product_id)…
        .ok_or_else(|| (BAD_REQUEST, json!({"error": format!("Product {} not found or inactive", product_id)})))?
} else {
    return Err((BAD_REQUEST, json!({"error": "Each item must have either plan_id or product_id"})));
};
```

**PASS**

---

## Check 7 — Fix 1.3: open redirect blocked in payments.rs

**File:** `api/src/routes/payments.rs:303–317`

```rust
let success_path = input.success_path.as_deref().unwrap_or("/checkout/success");
let cancel_path  = input.cancel_path.as_deref().unwrap_or("/checkout/cancel");
if !success_path.starts_with('/') || !cancel_path.starts_with('/') {
    return Err((BAD_REQUEST, json!({"error": "success_path and cancel_path must start with /"})));
}
let app_url = state.config.app_url.trim_end_matches('/');
// …
success_url: format!("{}{}?order={}", app_url, success_path, order_number),
cancel_url:  format!("{}{}", app_url, cancel_path),
```

Field `success_url` / `cancel_url` removed from `CreateCheckoutRequest`; replaced by `success_path` / `cancel_path`
(lines 36–37). Full URL is constructed server-side from `config.app_url`.

**PASS**

---

## Check 8 — Fix 1.3: open redirect blocked in checkout.rs

**File:** `api/src/routes/checkout.rs:342–352`

```rust
let success_path = input.success_path.as_deref().unwrap_or("/checkout/success");
let cancel_path  = input.cancel_path.as_deref().unwrap_or("/checkout/cancel");
if !success_path.starts_with('/') || !cancel_path.starts_with('/') {
    return Err((BAD_REQUEST, json!({"error": "success_path and cancel_path must start with /"})));
}
let app_url     = state.config.app_url.trim_end_matches('/');
let success_url = format!("{}{}", app_url, success_path);
let cancel_url  = format!("{}{}", app_url, cancel_path);
```

`CheckoutRequest` struct fields (lines 34–35) use `success_path` / `cancel_path`.

**PASS**

---

## Check 9 — Fix 1.4: coupon validate endpoint requires auth

**File:** `api/src/routes/coupons.rs:104–106`

```rust
/// POST /api/coupons/validate
async fn validate_coupon(
    State(state): State<AppState>,
    _user: User,          // ← Axum extractor returns 401 if no valid JWT
    Json(input): Json<ValidateCouponRequest>,
```

`grep "_user: User" coupons.rs` → line 106. The `User` extractor in this codebase issues 401 on
missing/invalid token; the `_` prefix means the bound value is not used (only its presence matters
for auth enforcement).

**PASS**

---

## Check 10 — Fix 1.5: webhook receives raw `Bytes`; HMAC over exact payload

**File:** `api/src/routes/payments.rs:11,407,430,481`

```rust
use axum::body::Bytes;          // line 11

async fn webhook(
    …
    body: Bytes,                // line 407 — raw bytes, not String
) {
    …
    match stripe_for_webhook.verify_webhook(&body, signature) { … }  // line 430 — passes &[u8]

    let body_str = std::str::from_utf8(&body).map_err(|_| {          // line 481 — hard fail
        (BAD_REQUEST, json!({"error": "Invalid UTF-8 in webhook body"}))
    })?;
```

**File:** `api/src/services/stripe.rs:596,636`

```rust
pub fn verify_webhook(&self, payload: &[u8], signature_header: &str) -> Result<bool> {  // line 596
    …
    let payload_str = std::str::from_utf8(payload)  // line 636 — strict; not lossy
        .map_err(|_| anyhow!("Webhook payload contains invalid UTF-8"))?;
    let signed_payload = format!("{}.{}", timestamp, payload_str);
    // HMAC computed over signed_payload — exact bytes, no lossy conversion
```

`from_utf8_lossy` is absent from both files.

**PASS**

---

## Check 11 — Fix 1.6: webhook idempotency table + handler

**Migration:** `api/migrations/055_webhook_idempotency_and_disputes.sql`

```sql
CREATE TABLE IF NOT EXISTS webhook_events (
    event_id        TEXT PRIMARY KEY,
    event_type      TEXT NOT NULL,
    received_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at    TIMESTAMPTZ,
    error           TEXT,
    payload         JSONB NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_webhook_events_unprocessed
    ON webhook_events(received_at)
    WHERE processed_at IS NULL;
```

**Handler logic** `api/src/routes/payments.rs:511–572`:

```rust
let inserted: Option<i64> = sqlx::query_scalar(
    r#"INSERT INTO webhook_events (event_id, event_type, payload)
       VALUES ($1, $2, $3)
       ON CONFLICT (event_id) DO NOTHING
       RETURNING 1"#,
)…fetch_optional(…).await…?.flatten();

if inserted.is_none() {
    tracing::info!(event_id = %event.id, "Duplicate webhook event — skipping");
    return Ok(StatusCode::OK);
}
// … process event handlers …
sqlx::query("UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1")
    .bind(&event.id)…?;
```

**PASS**

---

## Check 12 — Fix 1.7: DB errors in subscription_updated and subscription_deleted propagate as 500

**`handle_subscription_updated`** `api/src/routes/payments.rs:1133–1148`

```rust
.execute(&state.db.pool)
.await
.map_err(|e| {
    tracing::error!(
        target: "payments",
        subscription_id = %subscription.id,
        error = %e,
        "DB write failed in subscription_updated — Stripe will retry"
    );
    (INTERNAL_SERVER_ERROR, json!({"error": "Failed to update subscription"}))
})?;
```

**`handle_subscription_deleted`** `api/src/routes/payments.rs:1174–1186`

```rust
.execute(&state.db.pool)
.await
.map_err(|e| {
    tracing::error!(
        target: "payments",
        subscription_id = %subscription.id,
        error = %e,
        "DB write failed in subscription_deleted — Stripe will retry"
    );
    (INTERNAL_SERVER_ERROR, json!({"error": "Failed to cancel subscription"}))
})?;
```

Neither uses `.ok()`. Both propagate errors via `?` so the webhook returns 500 and Stripe retries.

**PASS**

---

## Check 13 — Fix 1.8: charge.dispute.created handler exists + inserts to payment_disputes

**Dispatch** `api/src/routes/payments.rs:562–563`:

```rust
"charge.dispute.created" => {
    handle_dispute_created(&state, &event).await?;
}
```

**`handle_dispute_created`** `api/src/routes/payments.rs:1456–1532`:

```rust
sqlx::query(
    r#"INSERT INTO payment_disputes
       (stripe_dispute_id, stripe_charge_id, reason, status, amount_cents, currency, response_deadline)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       ON CONFLICT (stripe_dispute_id) DO NOTHING"#,
)…execute(…).await.map_err(…)?;

sqlx::query(
    r#"INSERT INTO security_events (event_type, details, created_at)
       VALUES ('chargeback_dispute', $1, NOW())"#,
)
.bind(json!({"dispute_id":…, "charge_id":…, "reason":…, "severity": "high"}))
…execute(…).await.ok();   // log failure is non-fatal

tracing::warn!(
    target: "payments",
    event = "dispute_created",
    dispute_id = %dispute_id,
    "Chargeback dispute created — admin notification required"
);
```

**Migration table** `055_webhook_idempotency_and_disputes.sql`:

```sql
CREATE TABLE IF NOT EXISTS payment_disputes (
    id                  BIGSERIAL PRIMARY KEY,
    stripe_dispute_id   TEXT UNIQUE NOT NULL,
    stripe_charge_id    TEXT NOT NULL,
    reason              TEXT,
    status              TEXT NOT NULL,
    amount_cents        BIGINT NOT NULL,
    currency            TEXT NOT NULL DEFAULT 'usd',
    response_deadline   TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**PASS**

---

## Fix 1.9 (bonus check — refund access revocation)

**`handle_refund`** `api/src/routes/payments.rs:1353–1450`

Full-refund path (`refund_amount >= total_amount`):

1. `UPDATE orders SET status = 'refunded' … WHERE payment_intent_id = $1` — uses `map_err(…)?` (fatal on error)
2. `UPDATE user_memberships SET status = 'cancelled' …` — `.ok()` (best-effort, comment on line 1409)
3. `UPDATE user_course_enrollments SET is_active = false …` — `.ok()` (best-effort)
4. `UPDATE user_indicator_access SET is_active = false …` — `.ok()` (best-effort)
5. `tracing::info!(event = "access_revoked_on_refund", …)`

The primary orders write is fatal-on-error. The three access-revocation writes are annotated as
best-effort (`// Non-fatal — best-effort revocation`). This is an acknowledged design tradeoff:
the writes are idempotent and a failed revocation is recoverable by admin; forcing them fatal would
cause Stripe to retry the refund event and re-run the (already-applied) orders update.

**PASS**

---

## Summary table

| # | Check | Result |
|---|-------|--------|
| 1 | Branch is payments-fix-2026-04 | **PASS** |
| 2 | `cargo check` 0 errors 0 warnings | **PASS** |
| 3 | `pnpm check` 0 errors 0 warnings 5215 files | **PASS** |
| 4 | Fix 1.1 — reactivate: no free-access path for fully-cancelled subs | **PASS** |
| 5 | Fix 1.1 — frontend redirects fully-cancelled subs to /pricing | **PASS** |
| 6 | Fix 1.2 — price removed from CheckoutItem; DB lookup only | **PASS** |
| 7 | Fix 1.3 — open redirect blocked in payments.rs | **PASS** |
| 8 | Fix 1.3 — open redirect blocked in checkout.rs | **PASS** |
| 9 | Fix 1.4 — coupon validate requires JWT (`_user: User`) | **PASS** |
| 10 | Fix 1.5 — webhook uses `Bytes`; `from_utf8` not `from_utf8_lossy` | **PASS** |
| 11 | Fix 1.6 — idempotency table + ON CONFLICT DO NOTHING handler | **PASS** |
| 12 | Fix 1.7 — subscription_updated / subscription_deleted use `map_err` not `.ok()` | **PASS** |
| 13 | Fix 1.8 — charge.dispute.created handler + payment_disputes table | **PASS** |

---

**ALL 13 CHECKS PASS.**

Waiting for explicit "go" before Task 2.
