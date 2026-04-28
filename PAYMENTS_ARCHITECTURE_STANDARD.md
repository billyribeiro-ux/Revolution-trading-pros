# Payments Architecture Standard

**Version:** 1.0
**Status:** Canonical — applies to all current and future projects
**Owner:** Billy Ribeiro
**Last updated:** 2026-04-28

This document defines the single correct way to build payment, checkout, and subscription systems. If a project deviates from this standard, that deviation must be justified in writing in the project's `CLAUDE.md` and approved before code is written.

Examples use SvelteKit + Rust/Axum + Postgres + Stripe (the current default stack). Principles transfer to any stack.

---

## 1. First Principles (Non-Negotiable)

Every design decision below derives from these. Violating any is a bug.

1. **Stripe is the source of truth for products, prices, coupons, and billing state.** The application database stores references (Stripe IDs) and access state, never duplicated price data, never custom coupon logic.
2. **Money is integers (cents).** All monetary amounts are integer cents in the smallest currency unit. Floating-point math on money is forbidden.
3. **Webhooks are the only path that grants or revokes paid access.** Frontend success redirects are UX, not authority. Access is granted exclusively when Stripe confirms via webhook that money was received.
4. **Webhooks are idempotent.** Every event is processed at most once, regardless of how many times Stripe delivers it.
5. **Webhooks fail closed.** Signature verification is mandatory in all environments. No verification = reject. No bypass for "dev mode" in production.
6. **Server-side authorization on every billing endpoint.** Prices, URLs, customer IDs, and user IDs are never accepted from client input. The authenticated user identity drives everything.
7. **The frontend never touches money or secrets.** Card data goes directly to Stripe via Stripe Elements or Stripe Checkout. The frontend triggers checkout with product/price IDs only. The publishable key is the only Stripe credential it sees.
8. **One Stripe price ID, one canonical source.** Each price exists in exactly one place: Stripe. The app references it by string ID. The amount is never copied into the application database.

---

## 2. Architecture Overview

### Roles

```
┌─────────────────┐         ┌────────────────────┐         ┌──────────────┐
│                 │         │                    │         │              │
│  Stripe         │         │   Your App         │         │  Your User   │
│  (catalog +     │◀───────▶│   (access state +  │◀───────▶│              │
│   billing)      │ webhook │    Stripe IDs)     │         │              │
│                 │         │                    │         │              │
└─────────────────┘         └────────────────────┘         └──────────────┘
   Source of truth            Source of truth             Browser only
   for: products,             for: who has              talks to either
   prices, coupons,           access to what            via authenticated
   customers, subs,           and until when.           requests.
   payments, refunds,
   disputes, tax.
```

### What lives where

**In Stripe (managed via Stripe dashboard, NOT custom admin code):**
- Products
- Prices (one-time + recurring with monthly/quarterly/yearly intervals)
- Coupons and promotion codes
- Customers and their payment methods
- Subscription state (active, past_due, canceled, trialing, paused)
- Invoices, payment history, receipts
- Tax configuration (Stripe Tax)
- Refund records and dispute records

**In the application database:**
- Users (your authentication system)
- One `stripe_customer_id` per user (created lazily on first checkout)
- Access state table — for each user, what they currently have access to and until when
- Stripe ID references that link your products to Stripe products/prices
- Webhook event log for idempotency and audit
- Application-specific metadata: which course is which, slugs, descriptions, featured flags, sort order, marketing copy

**In the frontend:**
- Display logic only
- Stripe publishable key (safe to expose)
- Stripe Elements / Stripe Checkout JS for card collection
- No prices stored in code, no plan logic, no authorization decisions

---

## 3. Product Catalog Model

### Stripe-side organization

Every sellable item is a **Stripe Product** with one or more **Stripe Prices**.

| What you sell | Stripe Product type | Stripe Prices on it |
|---|---|---|
| Course | One-time product | Single price (one-time) |
| Indicator | One-time product | Single price (one-time) |
| Trading Room (e.g., "Futures Room") | Recurring product | 3 prices: monthly, quarterly, yearly |
| Trading Alerts (e.g., "Options Alerts") | Recurring product | 3 prices: monthly, quarterly, yearly |

Each trading room is its own Stripe product. Each alert subscription is its own Stripe product. A user might subscribe to multiple rooms and multiple alert services simultaneously.

### Application database schema

#### `products` — references Stripe products
```sql
CREATE TABLE products (
    id BIGSERIAL PRIMARY KEY,
    stripe_product_id TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK (type IN ('course', 'indicator', 'room', 'alert')),
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_featured BOOLEAN NOT NULL DEFAULT false,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

What's intentionally NOT here:
- No `price` column
- No `currency` column
- No `billing_interval` column

All of those live in Stripe.

#### `product_prices` — references Stripe prices
```sql
CREATE TABLE product_prices (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    stripe_price_id TEXT NOT NULL UNIQUE,
    interval TEXT,
    interval_count INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_prices_product ON product_prices(product_id) WHERE is_active = true;
```

This table answers "which Stripe prices apply to this product, and how should they appear on the pricing page?" It does NOT store the price amount.

When the frontend renders a pricing page, the app:
1. Loads `products` and `product_prices` from the database
2. For each `stripe_price_id`, fetches the live amount from Stripe (cached briefly, ~5 minutes)
3. Displays

If a price is changed in Stripe, the frontend reflects the new amount on the next cache refresh. No code change required.

### Operator workflow: adding a new product

1. **In Stripe dashboard:** Create a Product. Add Prices (one for one-time products, three for recurring with monthly/quarterly/yearly intervals).
2. **In your admin UI:** Create a row in `products` referencing the Stripe product ID, with the slug, type, description, and marketing content. Create rows in `product_prices` referencing each Stripe price ID with its interval label.
3. Done. The product appears on the site at its next cache refresh.

The admin UI never displays a dollar amount input field. Pricing is a Stripe-only operation.

---

## 4. Access State Model (Entitlements)

This lives entirely in the application database. It answers: "what does this user currently have access to and until when?"

### `user_entitlements` table

```sql
CREATE TABLE user_entitlements (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id BIGINT NOT NULL REFERENCES products(id),

    -- The Stripe artifacts that granted this entitlement
    source TEXT NOT NULL CHECK (source IN ('one_time', 'subscription', 'comp', 'manual_grant')),
    stripe_subscription_id TEXT,
    stripe_payment_intent_id TEXT,
    stripe_invoice_id TEXT,

    -- Access window
    granted_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_entitlements_user_active ON user_entitlements(user_id)
    WHERE revoked_at IS NULL AND (expires_at IS NULL OR expires_at > NOW());

CREATE UNIQUE INDEX idx_entitlements_subscription
    ON user_entitlements(stripe_subscription_id)
    WHERE stripe_subscription_id IS NOT NULL;
```

### Entitlement semantics by product type

| Product type | `expires_at` | `stripe_subscription_id` | Notes |
|---|---|---|---|
| Course | NULL (lifetime) | NULL | One-time purchase. Never revoked except by manual admin action. |
| Indicator | NULL (lifetime) | NULL | Same as course. |
| Trading room subscription | Set to `current_period_end` | Set | Renewed by webhook each billing cycle. Revoked when subscription canceled and period ends. |
| Trading alert subscription | Same as room | Set | Same as room. |
| Comp (admin-granted free access) | Set or NULL per admin choice | NULL | Operator decides duration. Logged in audit. |

### The single access check

All access checks throughout the application use one function:

```rust
async fn user_has_access_to_product(
    db: &PgPool,
    user_id: i64,
    product_slug: &str,
) -> Result<bool> {
    let row = sqlx::query!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM user_entitlements ue
            JOIN products p ON p.id = ue.product_id
            WHERE ue.user_id = $1
              AND p.slug = $2
              AND ue.revoked_at IS NULL
              AND (ue.expires_at IS NULL OR ue.expires_at > NOW())
        ) AS "has_access!"
        "#,
        user_id, product_slug,
    ).fetch_one(db).await?;
    Ok(row.has_access)
}
```

Every gated route, every component, every service uses this. There is no other access logic.

---

## 5. Checkout Flow (One Canonical Pattern)

One pattern for all four product types. No exceptions.

### Sequence

```
1. User clicks "Buy" or "Subscribe" on the frontend
2. Frontend calls POST /api/checkout
   Body: { stripe_price_id: "price_ABC123" }
3. Backend:
   a. Authenticates the user
   b. Validates the supplied stripe_price_id exists and is active in product_prices
   c. Creates or retrieves the user's stripe_customer_id
   d. Creates a Stripe Checkout Session with:
      - mode: "payment" or "subscription" (derived from the price's recurring field)
      - line_items: [{ price: <stripe_price_id>, quantity: 1 }]
      - customer: <stripe_customer_id>
      - success_url: built server-side from APP_URL
      - cancel_url: built server-side from APP_URL
      - allow_promotion_codes: true
      - client_reference_id: <user_id>
      - metadata: { user_id, product_id }
   e. Returns the session URL to the frontend
4. Frontend redirects user to the Stripe-hosted checkout page
5. User enters card details on Stripe's page
6. Stripe processes payment and redirects user to success_url
7. Stripe sends checkout.session.completed webhook
8. Webhook handler:
   a. Verifies signature
   b. Checks idempotency
   c. Reads metadata to find user_id and product_id
   d. Creates user_entitlement row (with subscription_id if recurring, expires_at)
9. User is redirected to success_url, which polls for entitlement and shows confirmation
```

### Backend endpoint contract

**POST /api/checkout** — one endpoint for ALL product types

Request body (only this — nothing else accepted):
```json
{ "stripe_price_id": "price_ABC123" }
```

Response:
```json
{
  "checkout_url": "https://checkout.stripe.com/c/pay/cs_...",
  "session_id": "cs_test_..."
}
```

Forbidden in this endpoint:
- Accepting any monetary amount
- Accepting `success_url` or `cancel_url` from the client
- Accepting `customer_id` from the client
- Accepting `user_id` from the client (use authenticated session only)
- Accepting any product details except the price ID

The endpoint validates that the supplied `stripe_price_id` exists in `product_prices` and is `is_active = true`. Any other ID is rejected with 400.

### Frontend success handling

After Stripe redirects to `success_url`:

1. The success page extracts `session_id` from the URL parameters
2. It polls `GET /api/checkout/session/:id` for up to ~10 seconds
3. The backend returns:
   - `status: "pending"` if the webhook hasn't arrived yet
   - `status: "complete"` with the granted entitlement details once it has
4. The page shows a confirmation message and a link to the purchased product

Polling exists because Stripe redirects can sometimes arrive before the webhook does. The user's access is provisioned by the webhook, not by the redirect.

### Why one endpoint, not separate endpoints per product type

Because the only thing that varies between courses, indicators, rooms, and alerts is the price ID. Stripe handles all the differences (one-time vs. subscription, billing interval, etc.) based on how the price is configured. The application logic is identical:

- Validate price ID
- Create checkout session
- Wait for webhook
- Grant entitlement

If the same logic applies, the same endpoint applies. Splitting it creates surface area for bugs and divergence.

---

## 6. Webhook Handler Architecture

### The endpoint

**POST /api/webhooks/stripe**

```rust
async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,  // axum::body::Bytes — NOT String
) -> Result<StatusCode, StatusCode>
```

HMAC verification operates on raw bytes.

### Step-by-step contract

Every webhook request must:

1. **Read raw body** as `Bytes`.
2. **Extract `Stripe-Signature` header**. Reject with 400 if absent.
3. **Verify signature** via Stripe's signature scheme using `STRIPE_WEBHOOK_SECRET`. Reject with 400 if verification fails. **No bypass in any environment.**
4. **Check timestamp** within tolerance (5 minutes). Reject if outside.
5. **Parse the event JSON**.
6. **Idempotency check**: insert the event ID into `webhook_events` table. If the row already existed, return 200 immediately.
7. **Dispatch** to a handler function based on event type.
8. **Mark the event as processed** in `webhook_events`.
9. Return 200.

If any step fails, return non-2xx so Stripe retries.

### `webhook_events` table

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

The `PRIMARY KEY` on `event_id` enforces idempotency at the database level.

### Required event handlers

| Stripe Event | What it triggers |
|---|---|
| `checkout.session.completed` | Create `user_entitlement` from `client_reference_id` and price metadata. |
| `customer.subscription.created` | Idempotent backstop in case checkout webhook missed it. Same logic. |
| `customer.subscription.updated` | If `cancel_at_period_end` flipped, update entitlement metadata. If price changed, log it. |
| `customer.subscription.deleted` | Set `revoked_at = NOW()` on the entitlement (period has fully ended). |
| `invoice.paid` | Extend entitlement's `expires_at` to new `current_period_end`. |
| `invoice.payment_failed` | Update entitlement metadata to "past_due"; do NOT revoke yet (Stripe handles dunning). |
| `charge.refunded` | If full refund: revoke entitlement immediately. If partial: log only. |
| `charge.dispute.created` | Insert into `payment_disputes` table; send admin notification. Do NOT auto-revoke. |
| `customer.subscription.trial_will_end` | Send "trial ending" notification email if trials are used. |

Unrecognized events return 200 (acknowledged but not acted on).

### Handler structure

```rust
async fn handle_event(state: &AppState, event: &StripeEvent) -> Result<()> {
    match event.event_type.as_str() {
        "checkout.session.completed" => handle_checkout_completed(state, event).await,
        "customer.subscription.created" => handle_subscription_created(state, event).await,
        "customer.subscription.updated" => handle_subscription_updated(state, event).await,
        "customer.subscription.deleted" => handle_subscription_deleted(state, event).await,
        "invoice.paid" => handle_invoice_paid(state, event).await,
        "invoice.payment_failed" => handle_payment_failed(state, event).await,
        "charge.refunded" => handle_refund(state, event).await,
        "charge.dispute.created" => handle_dispute_created(state, event).await,
        "customer.subscription.trial_will_end" => handle_trial_ending(state, event).await,
        other => {
            tracing::info!(event_type = other, "Unhandled webhook event");
            Ok(())
        }
    }
}
```

DB errors inside a handler must be logged at `error!` level and propagated. Never use `.ok()` to swallow errors silently. A failed DB write should result in a non-2xx response so Stripe retries.

---

## 7. Subscription Lifecycle

### State diagram

```
                 user clicks "Buy"
                        │
                        ▼
                ┌──────────────────┐
                │  checkout pending │
                └──────────────────┘
                        │
            payment succeeds (webhook)
                        │
                        ▼
                ┌──────────────────┐
                │      ACTIVE       │◀────┐
                └──────────────────┘     │
                  │            │         │
       user clicks         payment       │
       "cancel"            fails         │
                  │            │         │
                  ▼            ▼         │
       ┌─────────────────┐  ┌─────────┐  │
       │ canceling at    │  │ past_due│  │
       │ period end      │  └─────────┘  │
       └─────────────────┘    │   │      │
              │   │            │ Stripe  │
       user clicks  period     │ retries │
       "resume"     ends       │ succeeds│
              │   │            ▼         │
              │   ▼      same as canceled│
              │  ┌──────────────────┐    │
              │  │     CANCELED     │    │
              │  └──────────────────┘    │
              │                          │
              └──────────────────────────┘
                  (resume = un-cancel
                   before period ends)
```

### Cancellation behavior

When a user clicks "Cancel":

1. Frontend calls `POST /api/subscriptions/:id/cancel`
2. Backend:
   - Verifies the subscription belongs to the authenticated user
   - Calls Stripe: `subscription.update(stripe_sub_id, { cancel_at_period_end: true })`
   - Returns success
3. Stripe sends `customer.subscription.updated` webhook
4. Webhook handler updates entitlement metadata to reflect "canceling at period end"
5. User keeps access until `expires_at`
6. When period ends, Stripe sends `customer.subscription.deleted` webhook
7. Webhook handler sets `revoked_at = NOW()` on the entitlement

### Resume behavior (the legitimate version of "reactivation")

A user who has canceled but is still within their paid period can resume:

1. Frontend calls `POST /api/subscriptions/:id/resume`
2. Backend:
   - Verifies ownership
   - Verifies the subscription is in `cancel_at_period_end = true` state via Stripe API call
   - If yes: calls `subscription.update(stripe_sub_id, { cancel_at_period_end: false })`
   - If no (already fully canceled): returns 400 with "Subscription cannot be resumed; please subscribe again"
3. Stripe sends `customer.subscription.updated` webhook
4. Webhook handler clears the "canceling at period end" metadata

**No payment is collected on resume.** The original subscription continues uninterrupted.

### Re-subscription after full cancellation

A user whose subscription has fully ended re-subscribes by going through the full checkout flow. New Stripe Checkout session, new payment, new entitlement. There is no "reactivate" code path that bypasses Stripe.

### Plan changes (upgrade / downgrade / interval change)

When a user changes plan:

1. Frontend calls `POST /api/subscriptions/:id/change-plan`
2. Backend:
   - Verifies ownership
   - Validates the target `stripe_price_id` exists in `product_prices`
   - Calls Stripe: `subscription.update` with the new price and `proration_behavior: "create_prorations"`
3. Stripe handles proration — credits unused time on old plan, charges for new plan
4. Webhook fires; handler updates the entitlement

---

## 8. Price Change Feature (Operator-Controlled)

Operator changes a subscription price in the admin UI. The admin UI offers three options:

### Option 1: Apply to new subscribers only

Effect: New checkouts use the new price. Existing subscribers keep their current price forever (grandfathered).

Implementation:
1. Operator clicks "Change Price" on a product in admin UI
2. Operator enters the new amount (sent to Stripe, NOT stored in app DB)
3. Operator selects "Apply to new subscribers only"
4. Backend calls Stripe API:
   - Create a new Stripe Price with the new amount on the same product
   - Mark the old Stripe Price as inactive (`active: false`) — existing subscriptions continue billing on it, but it can't be used for new checkouts
5. Backend updates `product_prices`:
   - Insert new row referencing the new `stripe_price_id`
   - Mark old row as `is_active = false`
6. New checkouts use the new price. Existing subscribers keep paying the old amount.

### Option 2: Apply to all at next renewal

Effect: New checkouts use the new price. Existing subscribers move to the new price on their next billing cycle, with no proration.

Implementation:
1. Same as Option 1 steps 1-3, but operator selects "Apply to all at next renewal"
2. Backend creates new Stripe Price as in Option 1
3. Backend updates `product_prices` as in Option 1
4. Backend iterates through all active subscriptions on the old price:
   - Calls Stripe `subscription.update` with `items: [{ price: <new_price_id> }]` and `proration_behavior: "none"`
   - Stripe schedules the price change to take effect at the next renewal
5. Webhooks fire as each subscription is updated; the application logs each change

### Option 3: Apply to all immediately with proration

Effect: New checkouts use the new price. Existing subscribers immediately move to the new price; Stripe charges or credits them for the partial period.

Implementation: Same as Option 2 but `proration_behavior: "create_prorations"`. Stripe immediately charges or credits each subscriber based on the unused portion of their current period.

### Admin UI

A button on the product page: "Change subscription price"

Modal opens:
```
Current monthly price: $97.00
Current quarterly price: $260.00
Current yearly price: $970.00

Which price do you want to change?  [ ▼ Monthly ]
New amount: [ $127.00 ]

Apply to:
( ) Only new subscribers (current subscribers keep $97/month forever)
( ) All subscribers, starting at next renewal
( ) All subscribers immediately (Stripe will prorate)

[ Confirm price change ]
```

The operator never sees a database table. They see Stripe-side concepts presented as application UI.

### What this is NOT

- Admin UI does NOT store the new price in the application database
- Admin UI does NOT compute proration — Stripe does
- Admin UI does NOT loop through subscriptions in code beyond calling Stripe API on each — Stripe handles the actual billing

---

## 9. Refund and Revocation Policy

### Per product type

| Product type | Refund allowed? | Access on refund |
|---|---|---|
| Course | **No.** Full content access on purchase. | N/A |
| Indicator | **No.** Full content access on purchase. | N/A |
| Trading room subscription | Cancellation, not refund. User keeps access until period end, no money returned. | Access continues to period end; revoked when Stripe sends `customer.subscription.deleted`. |
| Trading alert subscription | Same as room. | Same as room. |

### How "no refunds" is enforced

The frontend does not expose a "request refund" button for courses or indicators.

If an operator manually issues a refund via the Stripe dashboard for a course or indicator (exceptional case):
1. Stripe sends `charge.refunded` webhook
2. The webhook handler:
   - Inserts a record into a `refunds` audit table
   - **Revokes the entitlement** (sets `revoked_at = NOW()`, `revoked_reason = 'refund'`)
3. The user loses access immediately

This is intentional: if an admin overrides policy and issues a refund, access is also revoked. Money returned and access removed go together.

### Subscription cancellation flow

Already covered in Section 7. User-initiated cancellation = `cancel_at_period_end`. No money returned. Access continues to period end.

If an admin issues a "refund this month" gesture for a subscription:
- Stripe sends `charge.refunded` for that invoice
- The webhook handler revokes the entitlement immediately

If the operator wants to be more generous (let user keep access despite refund), they should not issue a refund — they should comp the customer and not refund at all.

### Disputes (chargebacks)

When a user files a chargeback:
1. Stripe sends `charge.dispute.created` webhook
2. The webhook handler:
   - Inserts into `payment_disputes` audit table with dispute ID, charge ID, reason, response deadline
   - Sends admin notification email
   - Does NOT auto-revoke access yet (gives operator time to respond)
3. If dispute is lost, Stripe sends `charge.refunded` (chargeback is essentially a forced refund) and the standard refund handler revokes access

---

## 10. Coupons via Stripe Promotion Codes

### Why no custom coupon table

Stripe has a complete coupon and promotion code system. The application does not duplicate it.

Stripe handles:
- Percentage and fixed amount discounts
- Once / forever / repeating-for-N-months durations
- Maximum redemptions globally
- Per-customer redemption limits
- Expiration dates
- Restrictions to specific products or prices
- Currency-specific amounts

### Operator workflow

1. Operator goes to Stripe dashboard → Products → Coupons
2. Creates a coupon (e.g., "WELCOME50" — 50% off first month)
3. Optionally creates a Promotion Code (customer-facing redemption code) bound to the coupon
4. Done

No code change. No DB row. Stripe handles it.

### Customer redemption

In the checkout session creation, set `allow_promotion_codes: true`. Stripe Checkout shows a "promotion code" field on its hosted page. Customer enters code, Stripe validates and applies discount, charges the discounted amount.

If you want to validate/preview a code before checkout (e.g., "code accepted, here's your new total"), call Stripe's API to retrieve the promotion code:

```rust
// Authenticated endpoint, requires logged-in user
async fn preview_promotion_code(code: &str) -> Result<...> {
    // Calls Stripe API to look up the promotion code
    // Returns the discount it represents
    // Stripe handles eligibility (already redeemed, expired, restricted, etc.)
}
```

### What the application database does NOT have

- No `coupons` table
- No `validate_coupon` endpoint with custom logic
- No `coupon_redemptions` tracking
- No discount calculation in application code

All of those exist in Stripe.

### Handling abuse

Coupon farming (re-redeeming the same code repeatedly) is prevented by setting "max redemptions per customer" on the Stripe coupon. Stripe enforces it server-side. No application code needed.

---

## 11. Customer Portal (Stripe-Hosted)

Users manage their billing through Stripe's Customer Portal, not a custom-built UI.

### What the portal handles

- View invoices and receipts
- Update payment method
- See subscription details
- Cancel subscription (sets `cancel_at_period_end`)
- Switch plans (if configured)
- Update billing address

### How the application integrates

1. Frontend has a "Manage Billing" button
2. Button calls `POST /api/billing/portal`
3. Backend:
   - Authenticates user
   - Looks up `stripe_customer_id` for the user
   - Calls Stripe to create a Customer Portal session bound to that customer
   - Returns the portal URL
4. Frontend redirects user to the portal URL
5. User does whatever they want in Stripe's UI
6. Any changes (plan switch, cancellation, payment method update) trigger webhooks
7. The application's webhook handlers update entitlement state accordingly

### Configuration

Portal capabilities are configured in Stripe dashboard under Settings → Customer Portal. The application does not control them programmatically.

---

## 12. Admin UI Specifications

### What admins can do

| Capability | API endpoint | What happens |
|---|---|---|
| Add a new product | `POST /api/admin/products` | Inserts row in `products`. Operator must have already created the Stripe product. |
| Add a new price reference | `POST /api/admin/products/:id/prices` | Inserts row in `product_prices`. Operator must have already created the Stripe price. |
| Change a price | `POST /api/admin/products/:id/change-price` | Calls Stripe to create new price + deactivate old; updates `product_prices`. Triggers Section 8 flow. |
| Comp a user (free access) | `POST /api/admin/users/:id/comp` | Inserts `user_entitlement` with `source = 'comp'`, optional `expires_at`. No Stripe interaction. Audit logged. |
| Revoke comp access | `POST /api/admin/users/:id/revoke-comp` | Sets `revoked_at` on the comped entitlement. |
| View subscription details | `GET /api/admin/subscriptions/:id` | Read-only display of Stripe state + DB state. |
| Cancel a user's subscription (admin override) | `POST /api/admin/subscriptions/:id/cancel` | Calls Stripe `cancel_at_period_end: true`. Audit logged. |
| Issue refund | `POST /api/admin/orders/:id/refund` | Calls Stripe `refunds.create`. Webhook handler will revoke entitlement. |
| View payment events | `GET /api/admin/payment-events` | Reads `webhook_events` table. |
| View dispute queue | `GET /api/admin/disputes` | Reads `payment_disputes` table. |

### What admins CANNOT do

- Set prices manually (must go through Stripe)
- Bypass payment to grant subscription access (must use comp endpoint, which is audited)
- Edit `webhook_events` rows (immutable audit log)
- Delete `user_entitlements` (use revoke instead — preserves history)

### Admin authorization

All admin endpoints require `AdminUser` extractor. Granting `super_admin` or `developer` roles requires `SuperAdminUser`. Every admin action that affects a user's billing state writes to a `security_events` row.

---

## 13. Test Mode Setup and Live Mode Promotion

### Test mode (current and default)

- `STRIPE_SECRET_KEY=sk_test_...`
- `STRIPE_PUBLISHABLE_KEY=pk_test_...`
- `STRIPE_WEBHOOK_SECRET=whsec_...` (from Stripe CLI for local dev, or test endpoint in dashboard)
- All products, prices, coupons created in Stripe test mode
- Card numbers like `4242 4242 4242 4242` (success), `4000 0000 0000 0002` (decline), etc.

### Local webhook delivery

For local development, use Stripe CLI:
```
stripe listen --forward-to localhost:8080/api/webhooks/stripe
```

This gives a webhook secret for local. Set it in `.env`. Stripe CLI relays test events to your local webhook endpoint.

### Live mode promotion checklist

Before flipping to live keys:

- [ ] All products and prices recreated in Stripe live mode (test ↔ live data is separate)
- [ ] Coupons recreated in live mode
- [ ] `STRIPE_SECRET_KEY` set to `sk_live_...` in production environment
- [ ] `STRIPE_PUBLISHABLE_KEY` set to `pk_live_...` in production frontend env
- [ ] `STRIPE_WEBHOOK_SECRET` set to live webhook endpoint secret
- [ ] Live webhook endpoint configured in Stripe dashboard, pointing at production URL
- [ ] Customer Portal configured for live mode
- [ ] Stripe Tax configured (if applicable)
- [ ] Startup check: assert that in production environment, `STRIPE_SECRET_KEY.starts_with("sk_live_")`
- [ ] Test a full purchase with a real card before announcing
- [ ] Reconciliation job scheduled (Section 14)

---

## 14. Reconciliation Job

### Purpose

Webhooks can be missed (network failures, Stripe outages, app downtime). A daily reconciliation job catches drift.

### What it does

Runs daily:

1. Fetches all active subscriptions from Stripe (paginated)
2. For each Stripe subscription:
   - Looks up the corresponding `user_entitlement` by `stripe_subscription_id`
   - Compares state:
     - If Stripe says canceled but entitlement is active → revoke entitlement, log
     - If Stripe says active but entitlement is revoked → reinstate (rare), log
     - If `current_period_end` differs → update `expires_at`
3. Lists all active entitlements with `stripe_subscription_id`
4. For any that don't appear in the Stripe fetch → revoke (subscription was deleted long ago, webhook missed)
5. Logs all discrepancies to a `reconciliation_log` table for admin review

### Frequency

Daily is sufficient. Faster (hourly) if higher accuracy is needed.

---

## 15. Webhook Event Coverage Matrix

The full list of events the system subscribes to in the Stripe dashboard:

| Event | Required | Handler logic |
|---|---|---|
| `checkout.session.completed` | Yes | Provision entitlement |
| `customer.subscription.created` | Yes | Backstop provisioning |
| `customer.subscription.updated` | Yes | Update entitlement metadata |
| `customer.subscription.deleted` | Yes | Revoke entitlement |
| `customer.subscription.trial_will_end` | If trials used | Notify user |
| `invoice.paid` | Yes | Extend entitlement period |
| `invoice.payment_failed` | Yes | Mark past_due metadata |
| `invoice.payment_action_required` | Yes | Notify user (3DS reauth) |
| `charge.refunded` | Yes | Revoke entitlement |
| `charge.dispute.created` | Yes | Audit + admin notification |
| `charge.dispute.closed` | Yes | Update audit |
| `customer.created` | No | Optional analytics |
| `customer.updated` | No | Optional sync |

---

## 16. What This Standard Forbids (and Why)

These are non-negotiable. If code violates any of these, it's a bug.

| Forbidden | Why |
|---|---|
| Storing prices in the application database | Two sources of truth = inevitable drift |
| Floating-point math on money | Rounding errors, IEEE 754 precision issues |
| Accepting prices from the client | Client can submit `$0.01`; trivial bypass |
| Accepting `success_url` / `cancel_url` from the client | Open redirect attack vector |
| Bypassing webhook signature verification | Webhook spoofing → free access |
| Custom coupon table or coupon validation logic | Stripe already does this; duplication = bugs |
| Custom invoice generation | Stripe handles invoices; duplication = drift |
| Storing card data anywhere on application servers | PCI scope, regulatory burden |
| Granting access without a paid Stripe transaction (except via audited admin "comp") | Free access via `reactivate` path bug |
| Multiple checkout flows for different product types | Surface area for bugs and divergence |
| Deserializing webhook bodies as `String` instead of `Bytes` | Encoding edge cases breaking HMAC |
| Swallowing DB errors in webhook handlers (`.ok()`, `let _ =`) | Drift between Stripe and DB with no alert |
| Looping through subscriptions in app code to apply price changes | Stripe API handles this; doing it in app code is fragile |
| Building a custom Customer Portal | Stripe portal handles every edge case |

---

## 17. Quality Gates Before "Done"

Any payment system implementation must pass these gates before being considered ready for live mode:

1. **All Section 1 first principles satisfied** with file:line evidence.
2. **End-to-end test in Stripe test mode**: real Stripe Checkout session, real test card (`4242 4242 4242 4242`), webhook arrives, entitlement provisioned, access works.
3. **Idempotency proof**: same webhook delivered twice produces a single entitlement and a single access change. Verified with Stripe CLI `stripe trigger` or webhook resend.
4. **Cancel + resume cycle works** through Stripe Customer Portal end-to-end without admin intervention.
5. **Refund (admin-initiated) properly revokes access** and logs to audit table.
6. **Reconciliation job runs successfully** against test data and reports zero discrepancies.
7. **All admin endpoints require proper role** (verified with manual curl as regular user → 403).
8. **`pnpm check` 0/0, `cargo check` clean.**
9. **Webhook signature verification works** with the Stripe CLI test secret and rejects mismatched signatures.
10. **Live mode startup check**: assert `sk_live_` prefix when `ENVIRONMENT=production`.

---

## 18. Glossary

| Term | Definition |
|---|---|
| **Stripe Customer** | A Stripe-side record representing a paying user. Has an ID like `cus_ABC123`. One per user, persistent for life. |
| **Stripe Product** | A Stripe-side record representing something for sale. Has prices attached. ID like `prod_ABC123`. |
| **Stripe Price** | A Stripe-side record representing the cost of a product, including currency, amount, and (for subscriptions) recurring interval. ID like `price_ABC123`. |
| **Stripe Subscription** | A Stripe-side record representing an active recurring billing relationship. ID like `sub_ABC123`. |
| **Stripe Checkout Session** | A short-lived Stripe-hosted page for collecting payment. ID like `cs_test_...`. |
| **Stripe Promotion Code** | A customer-facing code that maps to a Stripe Coupon. The coupon defines the discount; the code is what users type. |
| **Webhook** | An HTTP POST from Stripe to the application, signaling a state change. |
| **Entitlement** | An application-side record of a user's right to access a product. Granted by webhook, revoked by webhook or admin. |
| **Idempotency** | Property where processing the same operation multiple times produces the same result as processing it once. |
| **Proration** | Calculation of a partial-period charge or credit when a subscription changes mid-cycle. Stripe handles this. |
| **Dunning** | The process of retrying failed payments and notifying customers. Stripe handles this. |

---

*End of Payments Architecture Standard v1.0.*
