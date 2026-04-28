You are working on Revolution Trading Pros (SvelteKit + Rust/Axum + Postgres + Stripe).

Read these documents at the project root before writing any code:
- PAYMENTS_ARCHITECTURE_STANDARD.md (the canonical spec)
- SUBSCRIPTION_AUDIT.md
- SUBSCRIPTION_DISCOVERY.md
- ADMIN_SYSTEM_DISCOVERY.md
- AUTH_AUDIT.md
- CHANGELOG.md
- CLAUDE.md (if exists)

GOAL: Fix the existing payment/subscription system in place so that:
1. Everything works end-to-end through the existing admin dashboard
2. Real Stripe checkout sessions are created (not placeholder URLs)
3. The price-change feature with three options works (new only / next renewal / immediate proration)
4. Critical security bugs are closed
5. Postmark email is wired for transactional emails

DO NOT delete the existing membership_plans, courses, indicators, products, orders, user_memberships, coupons, or trading rooms tables. We are FIXING in place, not rebuilding.
DO NOT delete the existing admin pages for subscriptions, courses, indicators, memberships, orders, coupons, or rooms. We are FIXING them.
DO keep code that works. DO replace broken/placeholder logic. DO add new code where missing.

Branch: create and work on `payments-fix-2026-04`. Backup first: pg_dump to backups/.

=== TASK 1: SECURITY FIXES (DO FIRST, EACH IN ITS OWN COMMIT) ===

1.1 Fix the buggy reactivate endpoint (SUBSCRIPTION_DISCOVERY §C1)
  File: api/src/routes/subscriptions.rs:1217-1256
  Currently: when subscription is fully cancelled, sets status='active' for 30 days with no Stripe call
  Fix: For fully cancelled subscriptions, return 400 with message "Subscription has fully ended. Please re-subscribe to restart." Do NOT bypass Stripe. Keep the existing legitimate path (cancel_at_period_end=true → un-flag via Stripe API call) intact.
  Update the frontend at frontend/src/routes/my/subscriptions/+page.svelte:481 — for fully cancelled subs, the button should now redirect to the checkout page for that plan instead of calling reactivate.

1.2 Fix the client-supplied price bug (SUBSCRIPTION_DISCOVERY §H2)
  File: api/src/routes/payments.rs:31-51 (CreateCheckoutRequest)
  Currently: accepts `price: f64` from client
  Fix: Remove `price` field from the request struct. The endpoint must look up price from membership_plans (or products/courses/indicators) by ID. Reject the request if the looked-up plan has no stripe_price_id set.

1.3 Fix the open redirect (SUBSCRIPTION_DISCOVERY §H1)
  File: api/src/routes/payments.rs:34-36 and :249-250; api/src/routes/checkout.rs:29-35
  Currently: success_url and cancel_url accepted from client
  Fix: Remove success_url and cancel_url from request structs. Build them server-side from state.config.app_url. If the client wants to indicate destination, accept only a path string (must start with "/") and prepend app_url server-side.

1.4 Auth-gate the coupon validate endpoint (SUBSCRIPTION_DISCOVERY §H3)
  File: api/src/routes/coupons.rs:104-107 and router setup
  Fix: Add `user: User` extractor to validate_coupon. Update router to require auth.

1.5 Webhook body must be Bytes not String (SUBSCRIPTION_AUDIT §C2)
  File: api/src/routes/payments.rs:337-411
  Fix: Change handler signature to use axum::body::Bytes. Pass raw bytes to verify_webhook. Update api/src/services/stripe.rs:613 to use raw bytes (not String::from_utf8_lossy).

1.6 Add webhook idempotency (SUBSCRIPTION_AUDIT §H2)
  Create new migration (next available version, ~055):
    CREATE TABLE webhook_events (
      event_id TEXT PRIMARY KEY,
      event_type TEXT NOT NULL,
      received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      processed_at TIMESTAMPTZ,
      error TEXT,
      payload JSONB NOT NULL
    );
    CREATE INDEX idx_webhook_events_unprocessed ON webhook_events(received_at) WHERE processed_at IS NULL;
  In api/src/routes/payments.rs webhook handler: after signature verification, INSERT INTO webhook_events ... ON CONFLICT (event_id) DO NOTHING RETURNING id. If returns nothing → return 200 immediately. Otherwise process the event, then UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1.

1.7 Stop swallowing DB errors in subscription_updated handler (SUBSCRIPTION_AUDIT §M2)
  File: api/src/routes/payments.rs:985-1012
  Fix: Replace .ok() with proper Result propagation. If DB write fails, return error so Stripe retries.

1.8 Add charge.dispute.created handler (SUBSCRIPTION_AUDIT §M1)
  File: api/src/routes/payments.rs (webhook dispatch)
  Add handler that:
    - Inserts into payment_disputes table (create the table in the same migration as webhook_events)
    - Logs to security_events
    - Sends admin notification email (after Postmark wired in Task 4)
  payment_disputes columns: id, stripe_dispute_id UNIQUE, stripe_charge_id, reason, status, amount_cents, currency, response_deadline, created_at

1.9 Refund must revoke access (SUBSCRIPTION_AUDIT §H, scenario H)
  File: api/src/routes/payments.rs:1208-1257 handle_refund
  Fix: When refund is full (amount equals charge total), update user_memberships.status='cancelled' for the matching subscription. For courses/indicators in user_course_enrollments / user_indicator_access, set access to revoked.

After Task 1 complete, write SECURITY_FIXES_RESULT.md with each fix's diff, test result, file:line evidence. cargo check + pnpm check must pass.

=== TASK 2: REAL STRIPE CHECKOUT (REPLACE PLACEHOLDER) ===

2.1 Populate stripe_price_id and stripe_product_id for the 20 existing membership_plans
  In Stripe test mode dashboard, create one Product per row in membership_plans that doesn't already have one. For trading rooms: create one Stripe product per room with three prices (monthly, quarterly, yearly). For courses/indicators: one product, one price (one-time).
  Document the mapping in a script: scripts/seed_stripe_test_products.ts or similar. Output: a list of {plan_id, stripe_product_id, stripe_price_id} that the next step uses.
  Then UPDATE membership_plans SET stripe_product_id = $1, stripe_price_id = $2 WHERE id = $3 for each.
  For courses: the courses table has price_cents but no stripe_price_id column. Add a migration: ALTER TABLE courses ADD COLUMN stripe_product_id TEXT, ADD COLUMN stripe_price_id TEXT.
  Same for indicators: ALTER TABLE indicators ADD COLUMN stripe_product_id TEXT, ADD COLUMN stripe_price_id TEXT.
  Same for products if used.

2.2 Fix subscriptions.rs:444-453 placeholder
  File: api/src/routes/subscriptions.rs create_subscription
  Currently: returns "https://checkout.stripe.com/placeholder?price=..."
  Fix: Replace with real Stripe Checkout Session creation:
    - Look up plan from membership_plans
    - If stripe_price_id is empty → return 400 "Plan not configured for checkout"
    - Get or create stripe_customer_id for the user (lazy create on first checkout)
    - Call Stripe to create Checkout Session with mode="subscription", line_items=[{price: stripe_price_id, quantity: 1}], customer=customer_id, success_url and cancel_url built from app_url, allow_promotion_codes=true, client_reference_id=user_id, metadata={user_id, plan_id}
    - Return the Stripe-hosted URL
  Use the existing api/src/services/stripe.rs (after Task 1.5 fixes). If a real Stripe SDK call doesn't exist yet, add one.

2.3 Fix the missing /api/checkout/create-session endpoint
  The frontend (frontend/src/lib/api/cart.ts:1132) calls POST /api/checkout/create-session which doesn't exist. Either:
  Option A: Add the route in api/src/routes/checkout.rs that calls the same logic as 2.2 (preferred — reuse logic)
  Option B: Update the frontend to call the existing POST /api/payments/checkout
  Pick A. Both checkout entry points should land in the same canonical path inside the Rust API.
  After fix, the frontend checkout flow at /checkout/[slug] must end-to-end create a real Stripe session.

2.4 Wire stripe_customer_id to users table
  Add migration: ALTER TABLE users ADD COLUMN stripe_customer_id TEXT UNIQUE
  In the checkout handler, on first checkout, call Stripe to create a Customer with the user's email, store the returned cus_xxx ID on the user. On subsequent checkouts, reuse it.
  Update the User struct in api/src/models/user.rs to include stripe_customer_id: Option<String>.
  Update auth.rs SELECT queries to include the new column.

2.5 Fix subscription.created handler stub (SUBSCRIPTION_AUDIT §S2)
  File: api/src/routes/payments.rs:943-962 handle_subscription_created
  Currently: only logs
  Fix: Idempotent backstop. If a user_membership doesn't already exist for this subscription_id, create one from the Stripe subscription metadata (user_id from metadata.user_id; plan_id from metadata.plan_id). Insert with ON CONFLICT (stripe_subscription_id) DO NOTHING.

After Task 2: end-to-end test. Use Stripe CLI: stripe listen --forward-to localhost:8080/api/webhooks/stripe. Create a real Stripe checkout session via the frontend's "Subscribe" button on a trading room. Pay with test card 4242 4242 4242 4242. Verify: real session, real webhook, user_memberships row created, access works.

=== TASK 3: PRICE CHANGE FEATURE ===

Build the three-option price change feature per PAYMENTS_ARCHITECTURE_STANDARD.md §8.

3.1 Backend endpoint: POST /api/admin/membership-plans/:id/change-price
  Body:
    {
      "new_amount_cents": 12700,
      "currency": "usd",
      "interval": "month" | "quarter" | "year",
      "apply_to": "new_only" | "next_renewal" | "immediate_proration"
    }

  Logic:
    - Auth: AdminUser (require super_admin for "immediate_proration")
    - Look up the plan; verify it has stripe_product_id and current stripe_price_id
    - Call Stripe to create a new Price on the same product with new_amount_cents and the recurring interval matching the plan
    - Call Stripe to mark old Price as active=false
    - UPDATE membership_plans SET stripe_price_id = $new_price_id, price = $new_amount_dollars WHERE id = $plan_id
    - If apply_to == "new_only": done. Existing subscribers continue billing on the old (now-inactive) price.
    - If apply_to == "next_renewal":
        Fetch list of all Stripe subscriptions on the old price ID (paginated)
        For each, call Stripe subscription.update with items=[{id: <subscription_item_id>, price: <new_price_id>}], proration_behavior="none"
    - If apply_to == "immediate_proration":
        Same as next_renewal but proration_behavior="create_prorations"
    - Log the action to security_events with plan_id, old_price_id, new_price_id, apply_to, count_of_subscriptions_affected

3.2 Frontend admin UI
  Update or create frontend/src/routes/admin/memberships/[id]/+page.svelte (or wherever membership plan editing lives) to add a "Change Price" button.
  Modal shows:
    - Current price (read from Stripe by stripe_price_id, cached briefly)
    - New amount input (in dollars; convert to cents on submit)
    - Three radio buttons:
      ( ) Apply to new subscribers only — current subscribers keep their price
      ( ) Apply to all at next renewal — current subscribers move to new price on next billing cycle
      ( ) Apply to all immediately with proration — current subscribers charged/credited for partial period
    - Confirm button (with explicit warning text for option 3)
    - Preview line: "X active subscribers will be affected"
  On submit: call POST /api/admin/membership-plans/:id/change-price.

3.3 Same feature for courses and indicators
  For one-time products (courses, indicators), the price change flow is simpler — only "new buyers only" matters since one-time purchases don't renew. Build:
    - POST /api/admin/courses/:id/change-price
    - POST /api/admin/indicators/:id/change-price
  Same Stripe pattern (new Price, deactivate old). No subscription update logic needed.

After Task 3: manual test in Stripe test mode.
  - Create 2 test subscriptions on a trading room price
  - Run "new_only" with a higher price → existing subs unchanged, new checkouts use new price
  - Run "next_renewal" → existing subs scheduled to switch
  - Run "immediate_proration" → Stripe applies proration

=== TASK 4: WIRE POSTMARK FOR TRANSACTIONAL EMAIL ===

4.1 Setup
  Confirm POSTMARK_TOKEN exists in api/.env and POSTMARK_FROM_EMAIL is set.
  Add postmark = "0.x" to api/Cargo.toml (or use reqwest directly with Postmark HTTP API — pick one and pin it).

4.2 Build api/src/services/email.rs
  Struct EmailService with from_email, postmark client.
  async fn send_transactional(to: &str, template_alias: &str, model: serde_json::Value) -> Result<()>
  On send: insert into email_logs (status, to, template, sent_at, error). On failure: log error, write log row with status='failed', RETURN error (no .ok() swallowing).

4.3 Create email templates in Postmark dashboard with these aliases (build the templates as basic HTML, can be polished later):
  welcome
  email-verification
  password-reset
  subscription-confirmation
  subscription-canceled
  subscription-renewal-reminder
  trial-ending
  failed-payment
  receipt
  refund-confirmation
  dispute-created (admin notification)

4.4 Wire the triggers
  - api/src/routes/auth.rs register handler → send "welcome" + "email-verification"
  - api/src/routes/auth.rs forgot_password handler → send "password-reset"
  - api/src/routes/payments.rs handle_checkout_completed → if subscription, send "subscription-confirmation"; if one-time (course/indicator), send "receipt"
  - handle_subscription_deleted → send "subscription-canceled"
  - handle_payment_failed → send "failed-payment"
  - handle_refund → send "refund-confirmation"
  - handle_dispute_created → send "dispute-created" to admin email
  - For renewal reminders + trial ending: write a daily scheduled job api/src/jobs/email_reminders.rs

4.5 Test each trigger
  Use Postmark test server token; emails go to your real inbox. Verify each trigger sends.

=== TASK 5: CUSTOMER PORTAL ===

5.1 Endpoint already exists at api/src/routes/payments.rs:297 (create_portal). Verify it works:
  - Frontend has "Manage Billing" button somewhere (find or add in /my account)
  - Clicking it calls POST /api/payments/portal
  - Backend creates Stripe Customer Portal session bound to user's stripe_customer_id
  - Returns portal URL
  - Frontend redirects user there

5.2 Configure Customer Portal in Stripe test dashboard:
  - Allow customers to update payment method
  - Allow customers to cancel subscription (cancel_at_period_end)
  - Allow customers to view invoices and receipts
  - Allow customers to switch plans (optional; verify with operator before enabling)

=== TASK 6: RECONCILIATION JOB ===

6.1 Build api/src/jobs/reconcile_stripe.rs (per PAYMENTS_ARCHITECTURE_STANDARD.md §14)
  Daily job that:
    - Lists all Stripe subscriptions
    - For each, looks up matching user_memberships row
    - Compares: if Stripe says canceled but DB says active → update DB; if periods differ → update period_end
    - Lists DB user_memberships with status='active' that don't appear in Stripe → mark cancelled
    - Logs to a new reconciliation_log table (id, run_at, discrepancies_found, log JSONB)
  Wire to scheduler (tokio-cron-scheduler or similar; runs daily 03:00 UTC).
  Add admin endpoint GET /api/admin/reconciliation/log to view results.

=== TASK 7: END-TO-END VERIFICATION ===

Run these scenarios with Stripe test mode + 4242 4242 4242 4242:

A. Subscribe to a trading room (monthly) → real checkout, real webhook, user_membership row created with correct period_end, subscription-confirmation email arrives, access works
B. Cancel via Stripe Customer Portal → cancel_at_period_end=true, access continues to period end
C. Resume before period ends → cancellation un-flagged, no new payment
D. Let period end → access revoked via subscription_deleted webhook, cancellation email arrives
E. Re-subscribe → new checkout, new membership
F. Failed payment via stripe trigger invoice.payment_failed → past_due status, failed-payment email
G. Refund via Stripe dashboard → access revoked, refund email
H. Buy a course → user_course_enrollments row, receipt email, lifetime access
I. Refund the course → access revoked
J. Idempotency test: stripe events resend <event-id> → no duplicate access grant
K. Price change "new_only" → 2 existing subs unchanged, new checkout uses new price
L. Price change "next_renewal" → existing subs scheduled to switch
M. Coupon redemption: create promo code in Stripe dashboard, redeem at checkout → discount applied

Document each scenario PASS/FAIL with HTTP and DB evidence in PHASE_FIX_RESULT.md.

=== HARD RULES ===

- Branch: payments-fix-2026-04. Commit per task. Don't merge to main until all tasks pass verification.
- Do NOT delete tables or large code blocks. We are fixing in place.
- Do NOT use floating point for money. Convert prices to integer cents at all internal boundaries. The membership_plans.price column can stay as DECIMAL(10,2) for display, but all Stripe-bound math is cents.
- Do NOT bypass Stripe signature verification in any environment.
- Do NOT swallow errors with .ok() in webhook handlers — propagate so Stripe retries.
- Each task ends with cargo check + pnpm check passing 0/0.
- After each task, write a short result file. After all 7 tasks, write FIX_COMPLETE_REPORT.md with all evidence.
- Stop and report if anything is unclear rather than guessing.