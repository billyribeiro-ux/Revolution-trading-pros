# Admin Price Changes — Stripe-syncing without the dashboard

The product contract: **whenever an admin changes a price on the dashboard,
no Stripe-dashboard login is required.** This document describes the three
grandfathering options the admin gets, the server-side flow per option, the
webhook events Stripe sends back, and the rollback procedure.

---

## The three `apply_to` options

When the admin opens *Subscription Plans → Change Price*, the modal asks who
the new price applies to. The choice is sent to the backend as the JSON
field `apply_to`.

| `apply_to`              | Plain English                                                                  |
|-------------------------|--------------------------------------------------------------------------------|
| `new_only`              | New members only. Existing subscribers stay on the old price (grandfathered). |
| `next_renewal`          | Everyone, on next renewal. Existing subs migrate at their next billing cycle, with NO proration. Renewal date is preserved. |
| `immediate_proration`   | Everyone, immediately. Stripe migrates existing subs now and issues a prorated charge or credit on the next invoice. |

The modal includes a confirmation step. `immediate_proration` is rendered
as a destructive action because it triggers real-money movement.

---

## Server-side flow

The endpoint is `POST /api/admin/subscriptions/plans/:id/price` (Rust:
`api/src/routes/subscriptions_admin.rs::change_plan_price`). All three
paths share steps 1 – 5; steps 6 – 8 only run for `next_renewal` and
`immediate_proration`.

### Common steps (all three modes)

1. **Validate** the request body: `amount_cents > 0`, `billing_interval ∈
   {month, year, one_time}`, `apply_to ∈ {new_only, next_renewal,
   immediate_proration}`.
2. **Resolve a Stripe client** via `CredentialResolver::stripe_client(pool,
   env)`. This reads the secret key from `service_connections` (set in
   `/admin/settings`) with env-var fallback. No redeploy needed after
   rotation.
3. **Ensure a Stripe Product exists** for this plan. If
   `membership_plans.stripe_product_id` is null, create one via
   `POST /v1/products`. Persist the new id later in step 5.
4. **Create a new Stripe Price** via `POST /v1/prices` (Stripe Prices are
   immutable — every change is a new Price object).
5. **DB write (transaction)**:
   - `UPDATE membership_plans SET stripe_price_id, stripe_product_id,
     price WHERE id = :id`.
   - `INSERT INTO membership_plan_price_history (plan_id, old_*, new_*,
     apply_to, changed_by_user_id, ...)`.
   - `tx.commit()`.

   We commit BEFORE the bulk subscription updates. Stripe calls happen
   over external HTTP and we don't hold a Postgres tx open across them.

### Mode-specific steps

#### `new_only`

6. Done. The new Price is live for new checkouts; existing subscriptions
   keep referencing the old Price.

#### `next_renewal`

6. `SELECT stripe_subscription_id FROM user_memberships WHERE plan_id = :id
   AND status IN ('active', 'trial', 'past_due') AND payment_provider =
   'stripe'`.
7. For each subscription:
   - `POST /v1/subscriptions/:sub_id` with `items[0][id]=<existing_item>`,
     `items[0][price]=<new_price_id>`, `proration_behavior=none`,
     `billing_cycle_anchor=unchanged`.
   - The renewal date is preserved; no proration is issued.
8. `UPDATE membership_plan_price_history SET subscriptions_migrated,
   subscriptions_failed, failure_details` for the row inserted in step 5.

#### `immediate_proration`

6. Same query as `next_renewal`.
7. For each subscription:
   - `POST /v1/subscriptions/:sub_id` with `items[0][id]=<existing_item>`,
     `items[0][price]=<new_price_id>`,
     `proration_behavior=create_prorations`. NO billing_cycle_anchor
     override — Stripe issues a prorated credit/charge on the next invoice.
8. Same history-row update.

Per-subscription failures are logged and stored as JSON in
`membership_plan_price_history.failure_details`; they do NOT abort the
batch. This means a single bad subscription can't block the rest.

---

## Webhook events Stripe sends back

The webhook handler is `POST /api/payments/webhook` (Rust:
`api/src/routes/payments.rs::webhook`). It uses
`CredentialResolver::stripe_client` so the webhook signing secret is
read from `service_connections` first, env fallback otherwise.

Events you should expect after a price change:

| Event                                                | When                                                            |
|------------------------------------------------------|-----------------------------------------------------------------|
| `price.created`                                      | Always (we POST `/v1/prices` in step 4).                        |
| `customer.subscription.updated`                      | Per migrated subscription (next_renewal / immediate_proration). |
| `invoice.created` + `invoice.upcoming`               | At each renewal that uses the new price.                        |
| `invoice.created` (immediate)                        | Right after `immediate_proration`, with proration line items.   |
| `invoice.payment_succeeded` / `_failed`              | When the prorated invoice is paid (or fails).                   |
| `invoice.finalized`                                  | When the invoice closes for collection.                         |

The webhook handler currently consumes a subset of these for our internal
order/membership state. If you add more handlers, register the event types
in the Stripe dashboard webhook config (or via API) — there is no app-side
allow-list.

---

## Rollback procedure

Every price change writes a row to `membership_plan_price_history` with
the **old** Stripe Price ID. To roll back:

1. Open the plan in `/admin/subscriptions/plans` and note the row in the
   *Recent Price Changes* table you want to revert from.
2. SQL approach (until a UI affordance is built):

   ```sql
   -- Find the row.
   SELECT id, plan_id, old_stripe_price_id, new_stripe_price_id, apply_to
   FROM membership_plan_price_history
   WHERE plan_id = $1
   ORDER BY changed_at DESC
   LIMIT 5;

   -- Re-point the plan at the old Price ID. Keep the new Price active in
   -- Stripe (don't deactivate it — existing subs may still reference it).
   UPDATE membership_plans
   SET stripe_price_id = (
       SELECT old_stripe_price_id
       FROM membership_plan_price_history
       WHERE id = $history_id
   ),
       updated_at = NOW()
   WHERE id = $plan_id;
   ```

3. If the original change was `next_renewal` or `immediate_proration` and
   you want existing subscribers reverted, re-run the price-change
   endpoint with `apply_to = next_renewal` and the old amount/interval —
   that creates a NEW Stripe Price at the old amount and migrates each
   subscriber back. (Stripe doesn't let you "un-create" a Price, but
   creating an equivalent one is fine; the `_history` table records the
   round-trip cleanly.)

4. **Do not delete** rows from `membership_plan_price_history`. The table
   is append-only and is the source of truth for who-changed-what-when.

---

## Where the code lives

- Endpoint: `api/src/routes/subscriptions_admin.rs::change_plan_price`
- Stripe client: `api/src/services/stripe.rs`
  - `create_product`, `create_price`, `migrate_subscription_to_price`,
    `list_subscription_items`, `deactivate_price`.
- Credential resolver: `api/src/services/credential_resolver.rs`
- Migration: `api/migrations/043_membership_plan_price_history.sql`
- Migration: `api/migrations/044_service_connections.sql`
- Frontend modal:
  `frontend/src/routes/admin/subscriptions/plans/+page.svelte`
  (look for `showPriceModal`, `submitPriceChange`).
- Audit row read endpoint:
  `GET /api/admin/subscriptions/plans/:id/price-history`.

---

## Testing without a real Stripe account

`api/tests/stripe_price_sync_test.rs` and
`api/tests/stripe_subscription_migration_test.rs` skip when
`STRIPE_TEST_SECRET_KEY` is unset (returns early with a stderr line, no
panic). The PE7 gate (`scripts/pe7_gate.sh`) runs them with the env unset
so they are no-ops in CI.

To exercise them locally:

```bash
export STRIPE_TEST_SECRET_KEY="sk_test_…"
cd api && cargo test --test stripe_price_sync_test -- --nocapture
cd api && cargo test --test stripe_subscription_migration_test -- --nocapture
```

The price-change endpoint itself is exercised end-to-end manually from the
admin UI — there's no Postgres-backed integration test yet because the
test harness lacks a Stripe-mock layer (no `wiremock`/`mockito` in
`Cargo.toml`). That's tracked as a follow-up; until then, the test_mode
keys + local stripe-cli `listen` are the integration surface.
