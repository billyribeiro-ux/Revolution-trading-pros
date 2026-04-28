# FULL_SYSTEM_AUDIT.md
## Revolution Trading Pros — Master System Audit
**Date:** 2026-04-28
**Sources merged:** SUBSCRIPTION_AUDIT.md · SUBSCRIPTION_DISCOVERY.md · ADMIN_SYSTEM_DISCOVERY.md
**Status:** Read-only. No code or DB modifications. No credentials decoded or used.

---

## 1. Executive Summary

### Payment readiness: NO — 3 critical blockers before first paying customer.

| Severity | Count | Items |
|---|---|---|
| CRITICAL | 3 | C1 free reactivation, C2 webhook bytes, C3 base64 "encryption" |
| HIGH | 5 | H1 open redirect, H2 no webhook idempotency, H3 unauthenticated coupon enum, H4 client-controlled price, H5 Stripe key plaintext in DB |
| MEDIUM | 4 | M1 dispute unhandled, M2 DB error swallowed, M3 float money, M4 no reconciliation |
| LOW | 2 | L1 unauthenticated tax endpoint, L2 free access on missing stripe_price_id |
| BROKEN / DEAD | 4 | B1 main checkout 404, B2 subscription.created stub, B3 placeholder checkout URL, B4 service_connections table missing |

### Admin / system readiness

| Dimension | State |
|---|---|
| Stack | All 4 services healthy (api, db, meili, redis) |
| Backend route modules | ~50 mounted |
| Frontend admin pages | 190+ across 35+ subdirectories |
| Database tables | 163 total; ~10 have real data; ~150 are empty scaffolding |
| Email subsystem | Skeleton only — no Postmark/SendGrid wired |
| CMS v2 | 20 tables + 30+ endpoints, 0 frontend callers |
| Indicators admin | Backend commented out (SQLx bug) |

### Top 5 things that work end-to-end

1. **Blog CRUD + public render** — fixed 2026-04-27 (CSP, tag-shape, `published_at`, proxy, R2 upload)
2. **Admin auth gate** — `admin/+layout.svelte` + `require_admin()` + `AdminUser` extractor
3. **Admin user management** — list/create/edit/ban/role-change post-2026-04-27 hardening
4. **Media upload to R2** — `POST /api/admin/media/upload` byte-identical roundtrip confirmed
5. **Trading room content** — 6 rooms, 12 schedules, 5 alerts, 6 trade plans, 1 weekly video

### Top 5 things broken or missing

1. **Main checkout is a 404** — `cart.ts:1132` calls `POST /api/checkout/create-session`; no such route exists
2. **All 20 membership plans have empty `stripe_price_id`** — subscription checkout returns a placeholder URL
3. **Email subsystem sends nothing** — `email_templates.rs:384` is a TODO stub; no Postmark calls anywhere
4. **Reactivate-subscription grants free access** — `POST /api/subscriptions/:id/reactivate` (C1, actively exploitable)
5. **CMS v2 is fully orphaned** — 3,000+ lines of Rust with zero frontend callers

---

## 2. Payments & Billing Security Findings

### 2.1 CRITICAL

---

#### C1 — Free Subscription via `reactivate_subscription`
**File:** [api/src/routes/subscriptions.rs:1179](api/src/routes/subscriptions.rs#L1179)
**Confirmed live:** Yes — user-facing button at [frontend/src/routes/my/subscriptions/+page.svelte:481](frontend/src/routes/my/subscriptions/+page.svelte#L481)

```rust
// subscriptions.rs:1224-1256 — NO STRIPE CALL for fully cancelled subscriptions
let period_end = now + chrono::Duration::days(30);
sqlx::query(
    "UPDATE user_memberships SET status = 'active', cancelled_at = NULL,
     current_period_end = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3"
)
```

**Exploit:** Any authenticated user with a cancelled subscription POSTs to `/api/subscriptions/:id/reactivate` and receives 30 days of free access — no payment, no Stripe interaction. Repeatable indefinitely.

The handler has a guard for `cancel_at_period_end == true && status == 'active'` (pending cancellation undo — correct, calls Stripe). But the guard is dead code: the function already returns `BAD_REQUEST` for `status == 'active'` at line 1204. The real cancelled-subscription branch falls straight through to the free DB write.

**Fix:** Redirect fully cancelled subscriptions to a new Stripe checkout session. Never set `status = 'active'` without a confirmed Stripe payment.

---

#### C2 — Webhook Body Decoded as String (HMAC Integrity Risk)
**File:** [api/src/routes/payments.rs:337](api/src/routes/payments.rs#L337)

```rust
async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,   // ← should be Bytes
) -> ...
```

```rust
// stripe.rs:613
let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
```

Axum's `String` extractor rejects non-UTF-8 bodies with a 400 before the handler runs — a Stripe webhook with any non-UTF-8 byte in JSON metadata causes infinite Stripe retries. The `String::from_utf8_lossy` in `signed_payload` is a no-op on already-validated UTF-8, making the normal case safe, but the approach is non-standard and fragile.

**Fix:** Use `axum::body::Bytes`. Pass raw bytes directly to `verify_webhook`.

---

#### C3 — Stripe Secret Key "Encrypted" with Base64
**File:** [api/src/routes/connections.rs:112](api/src/routes/connections.rs#L112)

```rust
// "uses base64 encoding as placeholder - in production use AES-256"
fn encrypt_credentials(creds: &serde_json::Value) -> Result<String> {
    Ok(STANDARD.encode(json.as_bytes()))  // base64 only — not encryption
}
```

```rust
// credential_resolver.rs:256
fn decrypt_credentials(&self, encrypted: &str) -> Result<serde_json::Value> {
    let decoded = STANDARD.decode(encrypted)?;  // base64 decode only
    Ok(serde_json::from_slice(&decoded)?)
}
```

Anyone with DB read access can base64-decode `service_connections.credentials_encrypted` and obtain the live Stripe secret key. With it: create arbitrary charges, issue refunds, read all customer data, cancel all subscriptions.

**Current status:** `service_connections` table does not exist in the local dev DB (confirmed via `information_schema` query) — so C3 is dormant until an admin creates a connection via the UI. Once the table exists, any DB read = Stripe key.

**Fix:** AES-256-GCM with key stored in `CREDENTIALS_ENCRYPTION_KEY` env var.

---

### 2.2 HIGH

---

#### H1 — Client-Controlled `success_url` / `cancel_url` (Open Redirect)
**Files:** [api/src/routes/payments.rs:249](api/src/routes/payments.rs#L249), [api/src/routes/checkout.rs:341](api/src/routes/checkout.rs#L341)
**Confirmed live:** Yes

```rust
// payments.rs:249-250
success_url: format!("{}?order={}", input.success_url, order_number),
cancel_url: input.cancel_url,
```

Attacker sends `"success_url": "https://evil.com/phish"`. Stripe redirects the user there after payment. No domain validation.

**Fix:** Build URLs server-side from `state.config.app_url`. If a client-controlled destination is needed, accept only a path and prepend the trusted domain.

---

#### H2 — No Webhook Idempotency
**File:** [api/src/routes/payments.rs:435](api/src/routes/payments.rs#L435)

No `webhook_events` table. No event-ID deduplication. Stripe retries on non-2xx. Most `ON CONFLICT DO UPDATE` clauses provide partial protection for membership rows, but the coupon `usage_count` increment is **not idempotent** — two legitimate Stripe retries of `checkout.session.completed` double-increment the counter.

**Fix:** `webhook_events(event_id UNIQUE, event_type, processed_at)` table. At handler entry: `INSERT ... ON CONFLICT DO NOTHING RETURNING id`. If no row returned, return 200 immediately.

---

#### H3 — Unauthenticated Coupon Enumeration
**File:** [api/src/routes/coupons.rs:104](api/src/routes/coupons.rs#L104)
**Confirmed live:** Yes

`POST /api/coupons/validate` has no `User` extractor. Anyone can brute-force coupon codes (PROMO2024, WELCOME10, etc.) — response reveals whether a code is valid, expired, or usage-limited. No rate limiting observed.

**Fix:** Add `user: User` extractor and/or rate limiting.

---

#### H4 — Client-Controlled Price in `payments.rs` Checkout
**File:** [api/src/routes/payments.rs:31](api/src/routes/payments.rs#L31)
**Confirmed live:** Yes

```rust
pub struct CreateCheckoutRequest {
    pub items: Vec<CheckoutItem>,
    pub price: f64,          // USER CONTROLLED
    pub success_url: String, // USER CONTROLLED
    pub cancel_url: String,  // USER CONTROLLED
    ...
}
```

`POST /api/payments/checkout` with `"price": 0.01` → Stripe charges $0.01/month → webhook fires with plan_id from the order → user gets full subscription access.

Note: `checkout.rs` fetches price from DB (safe). `payments.rs` does not.

**Fix:** Fetch price from DB by plan/product ID. Reject any client-supplied price.

---

#### H5 — Stripe API Keys Plaintext in DB (Same Root as C3)
Same finding as C3 from a different angle. Listed separately because it is independently reachable via any DB read access vector (SQLi, backup leak, insider).

---

### 2.3 MEDIUM

---

#### M1 — `charge.dispute.created` Not Handled
**File:** [api/src/routes/payments.rs:457](api/src/routes/payments.rs#L457)

Falls through to `_ => { tracing::debug!("Unhandled webhook event: {}", event.event_type) }`. No admin notification, no audit log. Stripe disputes have a 7–21 day response deadline; unresponded disputes = automatic chargeback.

**Fix:** Add handler: log to audit table + send admin email alert with dispute ID, charge ID, and deadline.

---

#### M2 — Silent DB Error Swallowing in `handle_subscription_updated`
**File:** [api/src/routes/payments.rs:1010](api/src/routes/payments.rs#L1010)

```rust
.execute(&state.db.pool)
.await
.ok();   // ← ALL DB errors silently discarded
```

Subscription state in the app diverges from Stripe with no log, no alert. Webhook returns 200 (tells Stripe "processed") even when the DB write failed.

**Fix:** `if let Err(e) = ... { tracing::error!(...) }` — propagate non-2xx on DB failure to trigger Stripe retry.

---

#### M3 — Floating-Point Arithmetic on Money
**Files:** [api/src/routes/payments.rs:47](api/src/routes/payments.rs#L47), [api/src/routes/checkout.rs:100](api/src/routes/checkout.rs#L100)

```rust
pub price: f64,
let mut subtotal = 0.0_f64;
subtotal += item.price * item.quantity as f64;
// ...
amount: (item.price * 100.0).round() as i64,
```

`$29.99 * 3 = 89.97000000000001` in IEEE 754. The `.round()` mitigates the worst case but accumulated errors across discounts and quantities can still produce incorrect cent amounts. `membership_plans.price` is `DECIMAL(10,2)` in the DB but cast to `FLOAT8` in all queries.

`rust_decimal::Decimal` is already a dependency (used in `coupons.rs`) but not applied to payment amounts.

**Fix:** Use `Decimal` for all monetary math. Store prices as cents (`i64`). Stop casting `DECIMAL` to `FLOAT8`.

---

#### M4 — No Stripe State Reconciliation

No cron job, no admin script, no reconcile endpoint. Missed webhooks (network failure, server downtime) leave subscriptions `active` in the DB after Stripe has cancelled them. Drift is invisible.

**Fix:** Daily job: list all `user_memberships` where `status = 'active'` and `stripe_subscription_id IS NOT NULL`, retrieve each from Stripe, flag discrepancies for admin review.

---

### 2.4 LOW

---

#### L1 — `calculate_tax` Endpoint is Unauthenticated
**File:** [api/src/routes/checkout.rs:445](api/src/routes/checkout.rs#L445)

Hardcoded rates (CA=7.25%, NY=8%, TX=6.25%, GB=20%, DE=19%). Not integrated with any tax service. Low risk but should require auth.

---

#### L2 — Plans Without `stripe_price_id` Grant Free Access
**File:** [api/src/routes/subscriptions.rs:444](api/src/routes/subscriptions.rs#L444)

```rust
if let Some(ref stripe_price_id) = plan.stripe_price_id {
    // returns placeholder URL
}
// falls through → INSERT user_memberships with payment_provider = 'free'
```

All 20 plans currently have `stripe_price_id = ''` (empty string, not NULL), so this branch is hit for every subscription attempt.

---

### 2.5 Additional Live/Dead Findings (from Discovery Pass)

#### B1 — Main Checkout → 404
**Vector:** [frontend/src/lib/api/cart.ts:1132](frontend/src/lib/api/cart.ts#L1132) → `POST /api/checkout/create-session`

```rust
// checkout.rs router — no /create-session route:
Router::new()
    .route("/", post(create_checkout))
    .route("/calculate-tax", post(calculate_tax))
    .route("/orders", get(get_orders))
    .route("/orders/:order_number", get(get_order_by_number))
```

**The primary user-facing checkout is completely broken.** Every checkout attempt results in a 404.

---

#### B2 — `customer.subscription.created` Is a Stub
**File:** [api/src/routes/payments.rs:943](api/src/routes/payments.rs#L943)

```rust
async fn handle_subscription_created(...) -> Result<()> {
    tracing::info!("Subscription created: {}", subscription.id);
    // TODO: Provision subscription access
    Ok(())
}
```

Access provisioning depends entirely on `checkout.session.completed`. Subscriptions created directly in Stripe (admin action, API) never provision access in the app.

---

#### B3 — `create_subscription` Returns Placeholder Stripe URL
**File:** [api/src/routes/subscriptions.rs:446](api/src/routes/subscriptions.rs#L446)

```rust
return Ok(Json(json!({
    "checkout_url": format!("https://checkout.stripe.com/placeholder?price={}", stripe_price_id),
})));
```

If sent to this URL, Stripe returns a 404. Plans with no `stripe_price_id` skip Stripe entirely and grant free access.

---

#### B4 — `service_connections` Table Does Not Exist
Confirmed via `SELECT table_name FROM information_schema.tables WHERE table_name LIKE 'service%'` → 0 rows. All Stripe credentials fall back to env vars. C3 is dormant but will activate the moment an admin saves a connection.

---

### 2.6 Layer 1 Checklist (40 items)

| # | Check | Status |
|---|-------|--------|
| 1 | Webhook signature verification | PASS |
| 2 | Sig verification not bypassable in prod | PASS |
| 3 | Webhook reads raw body | **PARTIAL** — `String` not `Bytes` |
| 4 | Webhook idempotency | **FAIL** — no `webhook_events` table |
| 5 | Webhook timeout handling | PARTIAL — email inline, no async queue |
| 6 | Webhook event filtering | PASS — unknown events → 200 |
| 7 | Server-side price validation | **FAIL** — `payments.rs` accepts client price |
| 8 | User identity binding | PASS — `user.id` from JWT, not request body |
| 9 | Coupon validation server-side | PASS (checkout.rs) / FAIL (unauthenticated endpoint) |
| 10 | Checkout session URL is single-use | N/A — Stripe enforces |
| 11 | Success/cancel URLs validated | **FAIL** — verbatim from client |
| 12 | Promo code abuse prevention | PARTIAL — no per-user coupon tracking |
| 13 | subscription.created provisions access | **FAIL** — stub only |
| 14 | subscription.deleted revokes access | PASS |
| 15 | Trial expiration handling | NOT IMPLEMENTED |
| 16 | Failed payment handling | PARTIAL — `past_due` set; expiry never enforced |
| 17 | Refund handling | PARTIAL — orders updated; membership NOT revoked |
| 18 | Dispute handling | **FAIL** — unhandled |
| 19 | Race: webhook before redirect | PARTIAL — `ON CONFLICT DO UPDATE` helps |
| 20 | Race: sub change during request | PARTIAL — no transactions |
| 21 | Cancel requires auth + ownership | PASS |
| 22 | Customer portal bound to auth user | PASS |
| 23 | Invoice/receipt access | PASS |
| 24 | Payment method endpoints ownership | PASS |
| 25 | Admin-only billing endpoints | PASS |
| 26 | DB matches Stripe reconciliation | **FAIL** — no mechanism |
| 27 | Stripe customer ID persisted | PARTIAL — in `user_memberships`, not `users` |
| 28 | Plan/price IDs referenced not duplicated | PASS |
| 29 | Currency handling | PASS — USD only |
| 30 | Decimal/cents handling | **FAIL** — `f64` throughout |
| 31 | Stripe.js from official URL | PASS |
| 32 | Publishable key only on frontend | PASS |
| 33 | Card data never touches server | PASS |
| 34 | 3DS / SCA support | PASS — Stripe Checkout handles |
| 35 | Payment events logged | PARTIAL — `tracing::info!`, no audit table |
| 36 | Failed webhook attempts logged | PARTIAL — `tracing::error!`, no DB log |
| 37 | Reconciliation job | **FAIL** — does not exist |
| 38 | Refund policy enforcement | NOT IMPLEMENTED |
| 39 | Proration handling | PARTIAL — custom `f64` math, not Stripe API |
| 40 | Tax handling | NOT IMPLEMENTED — hardcoded rates not applied |

---

### 2.7 Adversarial Scenarios

| Scenario | Exploitable? | Notes |
|---|---|---|
| A. Client-price manipulation | **YES** | `POST /api/payments/checkout` with `price: 0.01` |
| B. Webhook replay (>5 min) | NO | Timestamp check prevents |
| B. Legitimate Stripe retry double-process | **YES (partial)** | Coupon counter double-increments |
| C. Coupon farming | **YES** | No per-user coupon tracking; unauthenticated validate |
| D. BOLA on subscription endpoints | NO | All user endpoints check `user_id` |
| E. Webhook spoofing | NO (in prod) | HMAC + constant-time compare |
| F. Double-provisioning race | Partial | Coupon counter not atomic |
| G. Missed webhook = free access | YES (persistent) | No reconciliation job |
| H. Refund without revocation | **YES** | `handle_refund` touches `orders` only |
| I. Customer portal scope | Cannot assess | Requires Stripe dashboard |
| J. First-month promo + C1 loop | **YES** | C1 enables indefinite free reactivation |
| K. Stripe key in DB | YES (if table exists) | C3 — base64 is not encryption |
| L. Test key in production | Not implemented | No startup assertion for `sk_live_` |

---

### 2.8 Surprises

**S1 — Two parallel checkout flows.** `POST /api/payments/checkout` (accepts client price — dangerous) and `POST /api/checkout/` (fetches from DB — safe) both exist. Frontend calls neither correctly — it calls `/api/checkout/create-session` which doesn't exist (B1).

**S2 — `handle_subscription_created` does nothing.** The `customer.subscription.created` webhook is a logging stub with a TODO comment. This is undocumented and unexpected.

**S3 — `credentials_encrypted` column name is a lie.** The column is base64-encoded JSON. Misleads future developers and auditors.

**S4 — Reactivate guard is dead code.** The `cancel_at_period_end` guard in `reactivate_subscription` can never be reached because `status == 'active'` would have already returned `BAD_REQUEST` at the function entry check.

**S5 — `create_subscription` returns a fake Stripe URL.** Plans with `stripe_price_id` get back `https://checkout.stripe.com/placeholder?price=...` — a 404 on Stripe's side. Plans without it get free access immediately.

---

### 2.9 Reconciliation Gaps (Prioritized)

| Priority | Gap | Impact |
|---|---|---|
| 1 | Missed `customer.subscription.deleted` | User retains access after Stripe cancels |
| 2 | Missed `invoice.payment_failed` | User retains access despite non-payment |
| 3 | `reactivate_subscription` sets `status=active` without Stripe | Free access on demand |
| 4 | `customer.subscription.created` stub | User pays, never gets access |
| 5 | `charge.refunded` → access not revoked | User gets refund + keeps access |
| 6 | Grace period expiry never enforced | `past_due` memberships stay accessible forever |
| 7 | No periodic reconciliation job | Drift accumulates silently |

---

## 3. Live vs Dead — Checkout Flow Map

| Product type | Frontend route | API endpoint | Price from client? | Status |
|---|---|---|---|---|
| Membership plan (main) | `routes/checkout/+page.svelte:228` | `POST /api/checkout/create-session` | No | **BROKEN — 404** |
| Ad-hoc checkout | `routes/checkout/+page.svelte` (alt path) | `POST /api/payments/checkout` | **YES** | LIVE & DANGEROUS |
| Subscription (plan with stripe_price_id) | Unknown | `POST /api/subscriptions` | No | **BROKEN — placeholder URL** |
| Subscription (plan with no stripe_price_id) | Unknown | `POST /api/subscriptions` | No | BROKEN — grants free access |
| Reactivate cancelled subscription | `routes/my/subscriptions/+page.svelte:481` | `POST /api/subscriptions/:id/reactivate` | No | **ACTIVELY EXPLOITABLE** |
| Coupon validation | `routes/checkout/+page.svelte` | `POST /api/coupons/validate` | No | LIVE — unauthenticated |

### Webhook event coverage

| Stripe Event | Handler | Status |
|---|---|---|
| `checkout.session.completed` | `handle_checkout_completed()` | LIVE — creates order, provisions membership |
| `customer.subscription.created` | `handle_subscription_created()` | **STUB** — logs only, no provisioning |
| `customer.subscription.updated` | `handle_subscription_updated()` | LIVE — swallows DB errors (M2) |
| `customer.subscription.deleted` | `handle_subscription_deleted()` | LIVE — sets `status = 'cancelled'` |
| `invoice.paid` | `handle_invoice_paid()` | LIVE — updates membership period |
| `invoice.payment_failed` | `handle_payment_failed()` | LIVE — sets `status = 'past_due'` |
| `charge.refunded` | `handle_refund()` | LIVE — orders only, **no access revocation** |
| `charge.dispute.created` | catch-all `_ =>` | **UNHANDLED** |
| `payment_intent.*` | catch-all `_ =>` | UNHANDLED |

---

## 4. Admin / CMS Surface

### 4.1 Backend Route Inventory

All routes mounted under `/api/*` from `api/src/routes/mod.rs:83-242`:

| Mount path | Module | Notes |
|---|---|---|
| `/admin` | admin.rs | Root admin CRUD |
| `/admin/posts` | posts.rs | Blog — WORKING |
| `/admin/products` | products.rs | 0 rows |
| `/admin/schedules` | schedules.rs | — |
| `/admin/courses` | admin_courses.rs | 0 rows; multiple TODOs |
| `/admin/courses-enhanced` | courses_admin.rs | — |
| `/admin/indicators` | admin_indicators.rs | Backend partial; enhanced router commented out |
| `/admin/videos` | admin_videos.rs | TODOs for stats, room assignments |
| `/admin/page-layouts` | admin_page_layouts.rs | — |
| `/admin/media` | media.rs | Upload confirmed working |
| `/admin/popups` | admin_popups.rs | — |
| `/admin/forms` | forms.rs | 0 rows |
| `/admin/email`, `/admin/email/templates` | email_templates.rs | **STUB — no actual send** |
| `/admin/subscriptions`, `/admin/subscriptions/plans` | subscriptions_admin.rs | — |
| `/admin/trading-rooms`, `/admin/rooms` | trading_rooms.rs | 6 rooms active |
| `/admin/room-content`, `/admin/room-resources` | room_content.rs, room_resources.rs | — |
| `/admin/cms-v2`, `/admin/cms-v2/enterprise` | cms_v2.rs, cms_v2_enterprise.rs | **0 frontend callers** |
| `/cms/ai`, `/cms/seo`, `/cms/reusable-blocks`, `/cms/assets`, etc. | cms_*.rs | **0 frontend callers** |
| `/admin/crm` | crm.rs | 0 rows; 34 frontend pages; no real usage |
| `/admin/connections` | connections.rs | `service_connections` table missing |
| `/admin/consent` | consent.rs | In-memory only; no DB persist |
| `/admin/orders` | admin_orders.rs | 0 rows |
| `/admin/members`, `/admin/member-management` | admin_members.rs, admin_member_management.rs | — |
| `/admin/organization/teams`, `/departments` | organization.rs | `locations`, `onboarding-plans`, `training-modules` NOT mounted |
| `/admin/bunny` | bunny_upload.rs | Secondary CDN |

**Commented-out (dead) modules:**
- `mod.rs:16` — `// pub mod settings;` (replaced by admin.rs)
- `mod.rs:21` — `// pub mod indicators;` (replaced by member_indicators)
- `mod.rs:40` — `// pub mod indicators_admin; // TODO: Fix SQLx tuple decoding issues`
- `mod.rs:111` — `// .nest("/admin/indicators-enhanced", indicators_admin::router())`

### 4.2 Frontend Admin Page Status

| Frontend route | API endpoint(s) | Status |
|---|---|---|
| `/admin` (dashboard) | `/api/admin/members/stats`, `/api/admin/posts/stats`, etc. | PARTIAL — KPI tiles show NULL when no data |
| `/admin/blog` | `/api/admin/posts`, `/api/admin/tags`, `/api/admin/categories` | **WORKING** (2026-04-27 fixes confirmed) |
| `/admin/users` | `/api/admin/users`, `/api/admin/organization/departments` | PARTIAL (code-only) |
| `/admin/users/create` | + `locations`, `onboarding-plans`, `training-modules` | **BROKEN** — those endpoints not mounted |
| `/admin/members` | `/api/admin/members/*` | PARTIAL (code-only) |
| `/admin/orders` | `/api/admin/orders/*` | PARTIAL — 0 orders |
| `/admin/products` | `/api/admin/products/*` | PARTIAL — 0 products |
| `/admin/courses` | `/api/admin/courses/*` | PARTIAL — 0 courses; TODOs in handler |
| `/admin/indicators` | `/api/admin/indicators/*` | PARTIAL — enhanced router commented out |
| `/admin/trading-rooms` | `/api/admin/trading-rooms/*`, `/api/admin/room-content/*` | PARTIAL — 6 rooms in DB |
| `/admin/media` | `/api/admin/media` | PARTIAL — upload confirmed; malware scan TODO |
| `/admin/videos` | `/api/admin/videos/*` | PARTIAL — stats/trader endpoints return hardcoded 0s |
| `/admin/subscriptions` | `/api/admin/subscriptions/*` | PARTIAL — admin reactivate uses different path than buggy user POST |
| `/admin/memberships` | `/api/admin/membership-plans` | PARTIAL — 20 plans, all empty `stripe_price_id` |
| `/admin/email` | `/api/admin/email/templates/*` | **BROKEN** — no actual email send |
| `/admin/analytics/*` (13 sub-pages) | `/api/analytics/*`, `/api/admin/analytics/*` | PARTIAL — bounce rate TODO; goals endpoint may be missing |
| `/admin/seo/*` (15 sub-pages) | `/api/admin/seo/*`, `/cms/seo/*` | PARTIAL (code-only) |
| `/admin/cms/*` | (cms-v2 endpoints) | **DEAD CODE** — no page calls cms-v2 |
| `/admin/crm/*` (34 pages) | `/api/admin/crm/*` | DEAD CODE / SCAFFOLDING — 0 rows, no usage |
| `/admin/boards/*` (7 pages) | — | **DEAD CODE** — no `boards` Rust module |
| `/admin/behavior` | — | **DEAD CODE** — no backend route |
| `/admin/cart` | — | **DEAD CODE** — no backend route |
| `/admin/connections` | `/api/admin/connections` | PARTIAL — `service_connections` table missing |
| `/admin/categories` | `/api/admin/categories` | WORKING — 18 categories |

### 4.3 Auth Enforcement

**Frontend:** `admin/+layout.svelte:87-118` — `onMount()` guard (changed from `$effect` per FIX-2026-04-26):
1. If not authenticated → `goto('/login?redirect=/admin')`
2. If not admin → `goto('/?error=admin_required', { replaceState: true })`

**Backend:** Every admin route uses `require_admin(&user)?` or `AdminUser` extractor. `require_super_admin` used only for `POST /api/admin/users/:id/impersonate` and privileged role changes.

### 4.4 User Management Capabilities

| Capability | Status |
|---|---|
| List / search / filter users | WORKING |
| Create user | WORKING (password validation H-4 fix applied) |
| Edit user (role, profile) | WORKING (H-7: role escalation blocked for non-super_admin) |
| Ban/unban | WORKING (H-6: Redis session invalidated on ban) |
| Delete user | WORKING (code-only) |
| Impersonate | PARTIAL — returns non-JWT placeholder token (`impersonate_{id}_{timestamp}`) |
| Comp subscription | **MISSING** — no admin endpoint |
| Force password reset | **MISSING** — no admin endpoint |

---

## 5. Database State

### 5.1 Table Inventory (163 tables total)

#### Auth (8 tables)
| Table | Rows | Notes |
|---|---|---|
| users | 1 | Canonical |
| email_verification_tokens | 1 | Canonical |
| password_resets | 0 | Canonical |
| user_mfa_secrets | 0 | Canonical |
| oauth_tokens | 0 | Canonical |
| user_status | 0 | Legacy — `users.is_active` already covers this |
| mfa_attempts | 0 | Canonical |
| oauth_audit_log | 0 | Canonical |

#### Products & Memberships (8 tables)
| Table | Rows | Notes |
|---|---|---|
| membership_plans | 20 | All `stripe_price_id = ''` |
| membership_features | 0 | Sparsely used |
| products | 0 | Legacy stub |
| user_products | 0 | Legacy — should be `user_entitlements` |
| user_memberships | 0 | Canonical |
| user_coupons | 0 | Canonical |
| coupons | 0 | Canonical |
| **(missing)** product_prices | — | Needed for Stripe-owned price model |
| **(missing)** user_entitlements | — | Single canonical access table |

#### Courses & Learning (10 tables, all 0 rows)
courses, course_modules, course_lessons, course_sections, course_resources, course_live_sessions, course_downloads, lessons (legacy duplicate), user_course_enrollments, user_lesson_progress.

`courses.price_cents` is a local price column — violates "Stripe owns prices" pattern.

#### Indicators (13 tables)
| Table | Rows | Notes |
|---|---|---|
| indicators | 0 | — |
| indicator_platforms | 3 | MT4, MT5, TradingView |
| indicator_files, _videos, _documentation | 0 | — |
| user_indicator_access | 0 | Three near-duplicate tables — cleanup target |
| user_indicators | 0 | ^ |
| user_indicator_ownership | 0 | ^ |
| indicators_enhanced | 0 | Legacy — duplicate of indicators |

#### Trading Rooms (10 tables) — Active Data
| Table | Rows |
|---|---|
| trading_rooms | 6 |
| trading_room_schedules | 12 |
| room_alerts | 5 |
| room_trade_plans | 6 |
| room_weekly_videos | 1 |
| room_stats_cache | 3 |
| room_trades, room_traders, room_sections, room_resources | 0 |

#### Orders & Payments (5 tables, all 0 rows)
orders, order_items, invoices, webhooks (stub), webhook_deliveries.

**Missing canonical tables:**
- `webhook_events` — idempotency table (NEEDED for H2 fix)
- `payment_disputes` — chargeback handling
- `refunds` — refund records
- `reconciliation_log` — Stripe vs DB drift
- `service_connections` — referenced in code, does not exist

#### Content & CMS v1
| Table | Rows |
|---|---|
| posts | 1 |
| categories | 18 |
| tags | 2 |
| media | 22 |
| post_categories, post_tags, redirects, page_layouts | 0 |

#### CMS v2 (20 tables)
All 0 rows except `cms_reusable_blocks=6`, `cms_navigation_menus=3`, `cms_site_settings=1`.
No admin UI page calls these tables. Includes 13 partitions of `cms_audit_log` (Apr 2026 → Apr 2027) — significant infrastructure for unused functionality.

#### Email & Newsletter (4 tables, all 0 rows)
email_templates, email_logs, email_campaigns, newsletter_subscribers.

#### CRM (11 tables, all 0 rows)
crm_campaigns, crm_lists, crm_segments, crm_sequences, crm_automations, crm_templates, crm_companies, crm_tags, crm_smart_links, crm_webhooks, crm_recurring_campaigns.

#### Security & Audit (5 tables)
security_events (1 row), audit_logs (0), member_audit_logs (0), oauth_audit_log (0), mfa_attempts (0).

### 5.2 Legacy / Missing Classification

**Legacy (consolidate or delete):**
- `user_status` — duplicates `users.is_active`
- `lessons` — duplicates `course_lessons`
- `indicators_enhanced` — duplicates `indicators`
- `user_indicator_access`, `user_indicators`, `user_indicator_ownership` — three near-identical tables
- `webhooks`, `webhook_deliveries` (stub columns) — replace with canonical `webhook_events`

**Missing (needed for production-grade):**
- `product_prices` — separate price records keyed to Stripe
- `user_entitlements` — single source of truth for access
- `webhook_events` — idempotency / replay protection
- `payment_disputes`, `refunds`, `reconciliation_log`
- `service_connections` — referenced by code, not in schema
- `consent_settings` — consent.rs is in-memory only
- `course_views` — admin_courses.rs TODO

---

## 6. Email Subsystem

**Status: SKELETON ONLY.**

| Surface | State |
|---|---|
| email_templates table | Exists, 0 rows |
| email_logs / email_campaigns | Exist, 0 rows |
| newsletter_subscribers | Exists, 0 rows |
| email_templates.rs admin endpoints | Exist: list/create/update/delete/preview/test-send/send |
| Postmark / SendGrid integration | **NONE** |
| TODO marker | `email_templates.rs:384` — "For now, return success as placeholder" |
| Config fields `postmark_token`, `from_email`, `app_url` | Defined in config; consumed nowhere |

**Automated email triggers — all unimplemented:**
Welcome, email verification, password reset, subscription confirmation, renewal reminder, trial-ending, cancellation confirmation, failed payment, receipt, refund confirmation.

---

## 7. Product Management Gaps

| Product type | Admin UI | Local price? | Stripe price ID? | Gap |
|---|---|---|---|---|
| Course | `admin/courses/` | YES (`price_cents`) | NO column | Needs canonical products + product_prices migration |
| Indicator | `admin/indicators/` | YES (`price`) | NO column | Enhanced backend commented out |
| Trading room | `admin/trading-rooms/` | Via `membership_plans` | Empty for all 20 | Checkout placeholder |
| Membership plan | `admin/memberships/` | YES | Empty for all 20 | Stripe IDs must be populated |

**Architectural pattern violation across all four:** Local price columns + missing/empty `stripe_price_id` violates "Stripe is source of truth for prices." Fix sequence:
1. Create real `products` table keyed to a single product ID
2. Create `product_prices` with `stripe_price_id`, currency, recurring/one-time
3. Drop or reduce local price columns to display cache
4. Populate Stripe IDs for all 20 membership plans

---

## 8. Analytics & Media

### Analytics (Admin Dashboard)

Dashboard at `frontend/src/routes/admin/+page.svelte` (2,000+ lines) calls 5 endpoints. Reality check:

| Metric | Source | Real? |
|---|---|---|
| Total posts | `SELECT COUNT(*) FROM posts` | REAL (1) |
| Total products | `SELECT COUNT(*) FROM products` | REAL (0) |
| Total members / subscriptions / MRR | `user_memberships` | REAL (all 0 — no subscribers) |
| Sessions / pageviews / unique users | `analytics_events` aggregate | REAL — fix applied 2026-04-27; data may now flow |
| Bounce rate | analytics.rs:464,567 | **STUB — returns 0** |
| SEO metrics | `/api/admin/analytics/dashboard` | UNVERIFIED — depends on Search Console config |

### Media

- Upload confirmed working (CHANGELOG 2026-04-27 byte-identical roundtrip)
- 22 media rows in local dev DB
- Malware scan: `media.rs:1184` TODO stub
- R2 credentials in `api/.env` need rotation before production

---

## 9. Dead and Broken Code Summary

### Backend TODOs / Stubs

| File:Line | Item | Status |
|---|---|---|
| subscriptions.rs:446 | Stripe checkout session | TODO — placeholder URL |
| email_templates.rs:384 | Email send | TODO — placeholder |
| media.rs:1184 | Malware scan | TODO — placeholder |
| admin_courses.rs:2226 | course_views table | TODO |
| admin_courses.rs:2318 | order_items.course_id swap | TODO |
| admin_videos.rs:323-663 | Video stats, room assignments, traders | Multiple TODOs — hardcoded 0s |
| analytics.rs:464,567 | Bounce rate | TODO — stub |
| payments.rs:943-962 | subscription.created provisioner | STUB — logs only |
| websocket.rs:345,362 | Impersonation token validation | TODO — unvalidated |
| consent.rs:11-13,213 | Consent persistence | TODO — in-memory only |

### Orphan Backend (Registered, 0 Frontend Callers)

- `/api/cms/v2/*` — 30+ endpoints, ~3,000 lines of Rust
- `/api/admin/cms-v2/enterprise/*` — enterprise audit/workflow/preview
- `/api/cms/scheduling/*`, `/cms/datasources/*`, `/cms/global-components/*`, `/cms/ai/*`, `/cms/revisions/*`, `/cms/assets/*`

### Dead Frontend Routes (No Backend)

`/admin/boards/*` (7 pages), `/admin/behavior`, `/admin/cart`, `/admin/performance`, `/admin/resources`

### Missing Backend Routes (Frontend Calls Them)

`/api/checkout/create-session`, `/api/admin/organization/locations`, `/api/admin/organization/onboarding-plans`, `/api/admin/organization/training-modules`, `/api/admin/analytics/goals` (verify)

---

## 10. Architecture Alignment Matrix

| Subsystem | Current state | Required | Gap | Effort |
|---|---|---|---|---|
| Product catalog | 20 plans with empty `stripe_price_id`; local prices on courses/indicators | `products` + `product_prices`; Stripe-owned price; legacy columns → display cache | Full rebuild + migration | High |
| Checkout | Two parallel paths; frontend calls non-existent `/create-session` | Single canonical flow; prices from DB/Stripe | Significant | High |
| Webhook handling | 7 events; subscription.created stub; dispute unhandled; DB errors swallowed; no idempotency | All events handled idempotently via `webhook_events`; dispute handler; no `.ok()` on payment state | Medium | Medium |
| Refund revocation | `handle_refund` touches orders only | Revoke entitlement when refund ≥ original total | Low–Medium | Low |
| Reactivate subscription | Free access bug — ACTIVELY EXPLOITABLE | Require Stripe payment or redirect to checkout | URGENT | Low |
| Email | 0/10 triggers implemented | Full Postmark integration + 10 triggers | Medium | Medium |
| Indicators admin | Frontend exists; enhanced backend commented out | Fix SQLx or delete orphan frontend | Medium | Medium |
| CMS v2 | 20 tables + 30+ endpoints; 0 frontend wiring | **Decision:** complete or delete | Strategic | High |
| User management | Solid post-2026-04-27; missing comp-sub / force-reset | Add admin comp-subscription and forced password reset flows | Low | Low |
| Media | Upload works; malware scan TODO; creds need rotation | Wire malware scan; rotate creds | Low–Medium | Low |
| Analytics | Tracking fixed; bounce rate stub | Implement bounce rate; verify Search Console | Low | Low |

---

## 11. Open Questions

1. **CMS v2 intent.** Complete the frontend or delete 3,000+ lines of orphaned Rust + 20 tables?
2. **CRM intent.** 34 frontend pages and 11 backend tables with zero data — real feature in flight or aspirational scaffolding?
3. **Indicators admin.** What is the SQLx tuple-decoding issue? Enhanced router meant to replace or supplement `admin_indicators.rs`?
4. **Dead admin pages.** Boards/Behavior/Cart/Performance/Resources — spec mockups or deleted backends?
5. **Organization sub-routes.** `locations`, `training-modules`, `onboarding-plans` called by frontend but not mounted — are they planned?
6. **Email timeline.** When to wire Postmark vs delete the email-templates UI?
7. **service_connections.** Table referenced in code but missing from schema. Is the admin/connections UI meant to create it on first save, or is the subsystem stale?
8. **Standards docs.** `MIGRATION_ISSUES.md` and `PAYMENTS_ARCHITECTURE_STANDARD.md` were referenced in prompts but not found under `docs/` or project root.

---

## 12. Recommended Sequencing

### Tier 1 — Block release (security / revenue) — Hours to days

1. **Patch reactivate-subscription** (C1) — remove button or require Stripe payment. **Hours.**
2. **Lock down client-supplied price** in `payments.rs` (H4) — fetch from DB. **~1 day.**
3. **Validate `success_url` / `cancel_url`** against `APP_URL` domain (H1). **Hours.**
4. **Auth-gate `/api/coupons/validate`** (H3). **Hours.**
5. **Rotate R2 credentials** in production. **Operational.**

### Tier 2 — Real subscription billing — 1–2 weeks

6. Canonical product schema (`products` + `product_prices`); deprecate local price columns.
7. Real Stripe checkout sessions in `subscriptions.rs`; populate all 20 `stripe_price_id` values.
8. `webhook_events` idempotency table (H2 fix).
9. Refund revokes entitlement.
10. `subscription.created` provisioner.
11. Dispute handler + admin alert.
12. `sk_live_` startup assertion (L — test/live mode guard).

### Tier 3 — Email — ~1 week

13. Postmark integration + 4 highest-value triggers (welcome, verify, password reset, receipt).
14. Remaining 6 triggers (renewal, trial-ending, cancel, failed-payment, refund, comp).

### Tier 4 — Cleanup (after Tier 1–3 ship)

15. CMS v2 decision: complete or delete.
16. CRM decision: complete or delete.
17. Indicators admin SQLx fix or removal.
18. Delete unbacked frontend admin pages (boards, behavior, cart, performance, resources) or add backends.
19. Migrate course / indicator catalog to canonical product schema.
20. Consolidate legacy tables (user_status, lessons, indicators_enhanced, three user_indicator_* tables).
21. Admin "comp subscription" + admin-triggered password reset.
22. Broaden audit-log coverage.

### Tier 5 — Nice-to-have

- Malware scan integration
- Bounce rate implementation
- Analytics goals endpoint
- `consent_settings` persistence table
- `course_views` table

---

*Merged from SUBSCRIPTION_AUDIT.md, SUBSCRIPTION_DISCOVERY.md, ADMIN_SYSTEM_DISCOVERY.md — all read-only. No code, DB, or credentials modified. 2026-04-28.*
