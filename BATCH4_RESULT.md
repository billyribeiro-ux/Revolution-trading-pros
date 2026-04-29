# Batch 4 — Re-subscribe history + coupon admin gaps

Branch: `batch4-coupon-admin-followups`
Status at write-time: code complete + gates green; runtime scenarios PENDING OPERATOR.

---

## Gate evidence

All four gates run from working tree at HEAD of `batch4-coupon-admin-followups` immediately before commit.

| Gate | Result |
|------|--------|
| `cargo check` (api/, --locked) | ✅ 0 errors / 0 warnings — `Finished dev profile in 9.30s` |
| `cargo clippy --no-deps` (api/, --locked) | ✅ 0 errors / 0 warnings — `Finished dev profile in 14.95s` |
| `pnpm check` (frontend/) | ✅ `0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS` (svelte-check on 5217 files) |

The Rust gates exercise the workspace under `--locked`; clippy runs without `--no-deps` would also catch dependency-side lints, but the project standard per CLAUDE.md is the `--no-deps` form.

---

## Code-level evidence per item

### Item 1 — Re-subscribe row strategy

| Change | Location |
|--------|----------|
| Migration 063 drops `uq_user_memberships_user_plan` and adds partial unique on `(user_id, plan_id) WHERE status IN ('active','trialing','past_due')` | [api/migrations/063_user_memberships_partial_unique.sql](api/migrations/063_user_memberships_partial_unique.sql) |
| Migration 063 also adds partial unique on `stripe_subscription_id WHERE NOT NULL` (idempotency for retried webhook deliveries) | [api/migrations/063_user_memberships_partial_unique.sql:33-40](api/migrations/063_user_memberships_partial_unique.sql#L33-L40) |
| `handle_checkout_completed` switched from `ON CONFLICT (user_id, plan_id) DO UPDATE` to `ON CONFLICT (stripe_subscription_id) DO UPDATE` — re-subscription writes a new row rather than overwriting the cancelled one | [api/src/routes/payments.rs:933](api/src/routes/payments.rs#L933) |
| `PAYMENTS_ARCHITECTURE_STANDARD.md` §7 paragraph "Re-subscription after full cancellation" rewritten to specify the new-row contract | [PAYMENTS_ARCHITECTURE_STANDARD.md:482-510](PAYMENTS_ARCHITECTURE_STANDARD.md#L482-L510) |

### Item 2 — Admin edit-coupon recreate-and-swap

| Change | Location |
|--------|----------|
| `update_coupon` snapshots existing math fields, computes `math_changed`, creates a new Stripe Coupon **before** the DB UPDATE if math differs, flips `stripe_coupon_id` in the same UPDATE, then best-effort deletes the old Stripe coupon | [api/src/routes/admin.rs:902-1149](api/src/routes/admin.rs#L902-L1149) |
| Math-vs-metadata detection compares `discount_type`, `discount_value_cents`, `duration`, `duration_in_months` (and treats a NULL `stripe_coupon_id` as math-changed so a partial mirror gets healed via the same path) | [api/src/routes/admin.rs:986-1006](api/src/routes/admin.rs#L986-L1006) |
| Orphan rollback: if Stripe create succeeds but the DB UPDATE returns `Err` or `Ok(None)`, the new Stripe coupon is deleted before the handler returns the error | [api/src/routes/admin.rs:1146-1167](api/src/routes/admin.rs#L1146-L1167) |
| `security_events` audit row written on every successful recreate, capturing `coupon_id`, `old_stripe_coupon_id`, `new_stripe_coupon_id`, `fields_changed`, and the admin's `user_id` | [api/src/routes/admin.rs:1108-1144](api/src/routes/admin.rs#L1108-L1144) |

### Item 3 — Coupon backfill admin endpoint

| Change | Location |
|--------|----------|
| `POST /api/admin/coupons/:id/sync-to-stripe` — `AdminUser` gate, reads the row, errors with **400** if `stripe_coupon_id` is already set, otherwise creates the Stripe Coupon and writes the id back | [api/src/routes/admin.rs:1190-1310](api/src/routes/admin.rs#L1190-L1310) |
| 400 already-synced response (per spec) | [api/src/routes/admin.rs:1219-1228](api/src/routes/admin.rs#L1219-L1228) |
| Re-checks NULL via the read step before the create (idempotent on retry — second writer would hit the duplicate-stripe-id constraint or fall through to the 400 branch) | [api/src/routes/admin.rs:1199-1228](api/src/routes/admin.rs#L1199-L1228) |
| Route wired | [api/src/routes/admin.rs:2414](api/src/routes/admin.rs#L2414) |

### Item 4 — `min_purchase` enforcement

| Change | Location |
|--------|----------|
| `/api/payments/checkout` validator: fetches `min_purchase_cents`, rejects with 400 `"Order must be at least $X.YY to use this coupon"` when `subtotal_cents < min_purchase_cents` — **before** attaching the discount | [api/src/routes/payments.rs:236-276](api/src/routes/payments.rs#L236-L276) |
| `/api/checkout` validator: same enforcement in cents | [api/src/routes/checkout.rs:188-243](api/src/routes/checkout.rs#L188-L243) |
| Both compare `(min_purchase * 100)::BIGINT` from DB against the in-Rust integer `subtotal_cents` — no float math at the boundary | both |

`min_purchase` retained (not dropped) because operators may want app-side gating on subtotals which Stripe Coupons cannot enforce natively.

### Item 5 — Frontend admin coupon form

| Change | Location |
|--------|----------|
| `Coupon` and `CouponCreateData` typed with `duration`/`duration_in_months` | [frontend/src/lib/api/admin.ts:124-130](frontend/src/lib/api/admin.ts#L124-L130) |
| Cents conversion at the API boundary (`discount_value`, `min_purchase`, `max_discount` → `*_cents`) | [frontend/src/lib/api/admin.ts:838-848](frontend/src/lib/api/admin.ts#L838-L848) |
| Create page: Duration dropdown (`once` / `forever` / `repeating`), conditional `duration_in_months` input, validation that it is `>= 1` when repeating and `null` otherwise | [frontend/src/routes/admin/coupons/create/+page.svelte:510-549](frontend/src/routes/admin/coupons/create/+page.svelte#L510-L549) |
| Edit page: same UX, plus help-text reminding the operator that editing duration recreates the Stripe Coupon | [frontend/src/routes/admin/coupons/edit/[id]/+page.svelte:601-647](frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L601-L647) |

List-view duration semantics display (`"10% off, once"` / `"forever"` / `"3 months"`) — **deferred**: existing list view does not yet render the duration column. Backlog candidate for Batch 5.

---

## Runtime verification — PENDING OPERATOR

I cannot run the live Stripe + DB scenarios from here (need the local Docker stack, real Stripe Test Clocks, and operator interaction). Walk through the 12 below against a running local stack. Each scenario lists the exact commands to run and what to look for. Replace `$USER_ID`, `$PLAN_ID`, `$COUPON_ID`, `$ADMIN_TOKEN`, etc. with concrete values from your local DB.

Local stack assumed: API on `http://localhost:8080`, frontend on `http://localhost:5173`, Postgres reachable as `psql $DATABASE_URL`. Admin login: `welberribeirodrums@gmail.com` (per local_dev_admin memory).

### 1A — Re-subscribe creates a new row, leaves the cancelled one

**Setup**: pick a test user `$USER_ID` and a plan `$PLAN_ID` where the user has no active membership.

```bash
# 1. Subscribe via the normal checkout flow (test card 4242 4242 4242 4242).
#    Use the frontend at /pricing or hit /api/payments/checkout directly.

# 2. After Stripe webhook fires, confirm one active row:
psql "$DATABASE_URL" -c "
  SELECT id, stripe_subscription_id, status, starts_at, current_period_end
  FROM user_memberships
  WHERE user_id = $USER_ID AND plan_id = $PLAN_ID
  ORDER BY id;
"

# 3. Cancel via Customer Portal (or Stripe Dashboard for the test sub).

# 4. Advance the Stripe Test Clock past current_period_end:
stripe test_helpers test_clocks advance --frozen-time $(date -v+35d +%s) <CLOCK_ID>

# 5. Re-confirm: status should now be 'cancelled'.
psql "$DATABASE_URL" -c "
  SELECT id, stripe_subscription_id, status FROM user_memberships
  WHERE user_id = $USER_ID AND plan_id = $PLAN_ID;
"

# 6. Re-subscribe (new checkout flow, same user + same plan).

# 7. EXPECTED: two rows. One cancelled (historical), one active (new sub id).
psql "$DATABASE_URL" -c "
  SELECT id, stripe_subscription_id, status, starts_at
  FROM user_memberships
  WHERE user_id = $USER_ID AND plan_id = $PLAN_ID
  ORDER BY id;
"
```

**Expected**: step 7 returns 2 rows, distinct `stripe_subscription_id`, statuses `cancelled` (older) and `active` (newer). **Status: PENDING OPERATOR**

### 1B — Cannot subscribe to the same plan while already active

```bash
# With $USER_ID still active on $PLAN_ID, attempt a second subscribe.
# Either via the UI or:
curl -sS -X POST "http://localhost:8080/api/checkout" \
  -H "Authorization: Bearer $USER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"plan_id": '"$PLAN_ID"'}'
```

**Expected**: either app-level 409 before the Stripe call, or the partial unique index `uq_user_memberships_active_plan` fires when the webhook tries to insert (DB error logged in the API logs). The user does not end up with two active memberships for the same plan. **Status: PENDING OPERATOR**

### 2A — Edit coupon discount math triggers Stripe recreate

```bash
# Create a percent coupon via admin UI (e.g. TESTRECREATE, 10%).
# Note the coupon id and stripe_coupon_id:
psql "$DATABASE_URL" -c "
  SELECT id, code, stripe_coupon_id, discount_value FROM coupons
  WHERE code = 'TESTRECREATE';
"

# Edit via admin UI: change percent_off from 10% → 15%.

# After save, EXPECTED in DB: stripe_coupon_id has changed.
psql "$DATABASE_URL" -c "
  SELECT id, code, stripe_coupon_id, discount_value FROM coupons
  WHERE code = 'TESTRECREATE';
"

# Old Stripe coupon should 404:
stripe coupons retrieve <OLD_STRIPE_COUPON_ID>   # → 404

# New Stripe coupon should be valid with 15%:
stripe coupons retrieve <NEW_STRIPE_COUPON_ID>   # → 200, percent_off = 15

# Audit row should exist:
psql "$DATABASE_URL" -c "
  SELECT id, user_id, event_type, details, created_at
  FROM security_events
  WHERE event_type = 'coupon_recreated'
  ORDER BY id DESC LIMIT 1;
"
```

**Expected**: `stripe_coupon_id` flipped, old Stripe coupon 404, new one 200 at 15%, audit row present with `fields_changed` containing `discount_value_cents`. **Status: PENDING OPERATOR**

### 2B — Edit non-discount field does NOT trigger recreate

```bash
# On the same coupon, edit only the description (or is_active, expires_at).
# Confirm:
psql "$DATABASE_URL" -c "
  SELECT id, code, stripe_coupon_id, description FROM coupons
  WHERE code = 'TESTRECREATE';
"
```

**Expected**: `stripe_coupon_id` unchanged from 2A's new value. Description updated. No new `coupon_recreated` audit row. No new Stripe coupon visible in `stripe coupons list`. **Status: PENDING OPERATOR**

### 2C — Existing subscription keeps OLD discount per Stripe semantics

```bash
# Before doing 2A, make sure a test customer has redeemed the OLD coupon
# on a real subscription. Then perform 2A. Then:

stripe subscriptions retrieve <SUB_ID> --expand discount.coupon
```

**Expected**: the subscription's `discount.coupon.id` still equals the OLD `stripe_coupon_id` from before 2A (Stripe does not retroactively re-apply discounts when a coupon is replaced; existing subs keep their attached coupon). New customers redeeming the code now get the new (15%) coupon. **Status: PENDING OPERATOR**

### 3A — Backfill an unmirrored coupon row

```bash
# Insert a row directly with stripe_coupon_id = NULL:
psql "$DATABASE_URL" -c "
  INSERT INTO coupons (code, description, discount_type, discount_value, duration, is_active, created_at, updated_at)
  VALUES ('BACKFILL1', 'Backfill test', 'percent', 10, 'once', true, NOW(), NOW())
  RETURNING id;
"
# Note the returned id as $COUPON_ID.

curl -sS -X POST "http://localhost:8080/api/admin/coupons/$COUPON_ID/sync-to-stripe" \
  -H "Authorization: Bearer $ADMIN_TOKEN"

psql "$DATABASE_URL" -c "
  SELECT id, code, stripe_coupon_id FROM coupons WHERE id = $COUPON_ID;
"
stripe coupons retrieve <stripe_coupon_id from previous query>
```

**Expected**: response 200 with the row, `stripe_coupon_id` populated, `stripe coupons retrieve` returns 200 with matching `percent_off=10`, `duration=once`. **Status: PENDING OPERATOR**

### 3B — Sync-to-stripe on already-synced row returns 400

```bash
# Re-run the same call against the row from 3A:
curl -sS -X POST "http://localhost:8080/api/admin/coupons/$COUPON_ID/sync-to-stripe" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -i
```

**Expected**: HTTP 400, body `{"error":"Coupon already synced to Stripe; use the edit endpoint to change it"}`. **Status: PENDING OPERATOR**

### 4A — `min_purchase` blocks under-threshold checkout

```bash
# Create a coupon with min_purchase = $50 (5000 cents):
psql "$DATABASE_URL" -c "
  INSERT INTO coupons (code, description, discount_type, discount_value, min_purchase, duration, is_active, created_at, updated_at)
  VALUES ('MIN50', 'Min purchase test', 'percent', 10, 50, 'once', true, NOW(), NOW())
  RETURNING id;
"
# Then sync to Stripe:
curl -sS -X POST "http://localhost:8080/api/admin/coupons/$COUPON_ID/sync-to-stripe" \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Attempt checkout for a $47 plan with code MIN50:
curl -sS -X POST "http://localhost:8080/api/checkout" \
  -H "Authorization: Bearer $USER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"plan_id": <plan with price 4700 cents>, "coupon_code": "MIN50"}' \
  -i
```

**Expected**: HTTP 400, body `{"error":"Order must be at least $50.00 to use this coupon"}`. **Status: PENDING OPERATOR**

### 4B — `min_purchase` allows over-threshold checkout

```bash
# Same coupon, $147 plan:
curl -sS -X POST "http://localhost:8080/api/checkout" \
  -H "Authorization: Bearer $USER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"plan_id": <plan with price 14700 cents>, "coupon_code": "MIN50"}'
```

**Expected**: HTTP 200, returns a Stripe Checkout session URL. Complete the checkout and verify Stripe charged the discounted amount (`amount_total = 14700 - 1470 = 13230` cents). **Status: PENDING OPERATOR**

### 5A — Duration dropdown present on create page

Open `http://localhost:5173/admin/coupons/create`. Inspect the form.

**Expected**: a "Subscription Duration" dropdown with three options (`Once — first billing period only`, `Forever — every billing period`, `Repeating — first N months`). **Status: PENDING OPERATOR**

### 5B — Repeating reveals duration_in_months input

On the same page, switch the dropdown to **Repeating**.

**Expected**: a "Number of Months" number input appears with `min=1`, `max=36`. Switching back to Once or Forever hides it and clears the value to `null`. Submitting Repeating without a value triggers the inline error `"Repeating coupons require a duration of at least 1 month"`. **Status: PENDING OPERATOR**

### 5C — Submit with Duration=Once persists correctly

Fill the form with code `ONCE10`, percent_off=10, Duration=Once. Submit.

```bash
psql "$DATABASE_URL" -c "
  SELECT id, code, duration, duration_in_months, stripe_coupon_id FROM coupons
  WHERE code = 'ONCE10';
"
stripe coupons retrieve <stripe_coupon_id>
```

**Expected**: row has `duration='once'`, `duration_in_months=NULL`, non-null `stripe_coupon_id`. Stripe coupon shows `duration: once`. **Status: PENDING OPERATOR**

### 5D — Edit existing coupon to Repeating(3) triggers recreate

Open the edit page for the `ONCE10` coupon from 5C. Change Duration to **Repeating**, months = 3. Save.

```bash
# Note: this is item 2's recreate path because duration changed.
psql "$DATABASE_URL" -c "
  SELECT id, code, duration, duration_in_months, stripe_coupon_id FROM coupons
  WHERE code = 'ONCE10';
"
stripe coupons retrieve <new stripe_coupon_id>   # → duration: repeating, duration_in_months: 3
psql "$DATABASE_URL" -c "
  SELECT details FROM security_events
  WHERE event_type = 'coupon_recreated'
  ORDER BY id DESC LIMIT 1;
"
```

**Expected**: DB row reflects `duration='repeating'`, `duration_in_months=3`, **new** `stripe_coupon_id`. New Stripe coupon shows `duration: repeating`, `duration_in_months: 3`. Audit row has `fields_changed` including `duration` and `duration_in_months`. Old Stripe coupon 404s. **Status: PENDING OPERATOR**

---

## Summary

| Scenario | Status |
|---|---|
| 1A re-subscribe creates new row | PENDING OPERATOR |
| 1B blocked while already active | PENDING OPERATOR |
| 2A edit math → recreate | PENDING OPERATOR |
| 2B edit metadata → DB-only | PENDING OPERATOR |
| 2C existing sub keeps old discount | PENDING OPERATOR |
| 3A sync-to-stripe NULL row | PENDING OPERATOR |
| 3B sync-to-stripe already-synced 400 | PENDING OPERATOR |
| 4A min_purchase blocks under | PENDING OPERATOR |
| 4B min_purchase allows over | PENDING OPERATOR |
| 5A dropdown present | PENDING OPERATOR |
| 5B repeating reveals months input | PENDING OPERATOR |
| 5C Once persists correctly | PENDING OPERATOR |
| 5D Repeating edit triggers recreate | PENDING OPERATOR |

Once you walk these, edit each line above to PASS or FAIL with the raw command output captured beneath this summary.
