# Money-path dig — findings (2026-05-17)

**How found:** after the lint easy-wins were exhausted, dug the surface a
lint rule *cannot* see: frontend↔Rust money-contract correctness and the
i64-cents rule. Method = end-to-end trace (form → API client/normalizer →
SvelteKit proxy → Rust struct → SQL), not grep counts.

**Headline:** there is a *family* of admin money forms where the frontend
sends a dollar value under a `price`/`discount_value` key, but the Rust
request struct requires an integer-cents field (`*_cents: i64`). Some
were fixed in prior batches; one was an unfixed P0; one is still broken.

---

## F1 — P0 FIXED: admin product create/edit silently broken

**Severity:** P0 (admin product create AND edit fully non-functional on a
product-catalog/payments app).

**Trace:**
- `src/routes/admin/products/create/+page.svelte:127` (and
  `[id]/edit/+page.svelte:199`) send `price: parseFloat(formData.price)`
  — a **dollar float** (`19.99`), key `price`.
- `productsApi.create/.update` (`src/lib/api/admin.ts:1431/1440`) route
  through `normalizeProductPayload` (`:1375`).
- That helper (added in `FIX-2026-04-26 P0-4`) renamed `type`→
  `product_type` and tucked extras into `metadata`, **but never mapped
  `price` → `price_cents`**.
- Rust `CreateProduct.price_cents: i64` (`api/src/models/product.rs`) is
  **required** with `#[validate(range(min = 0))]`;
  `UpdateProduct.price_cents: Option<i64>`.
- Missing required field ⇒ serde rejects ⇒ **every create/update fails**.

**Why P0-4 missed it:** P0-4 (2026-04-26) predates the "Batch 1 cents
refactor (2026-04-28)" that taught the *coupon* normalizer to convert
dollars→cents. Products never got that follow-up. (See F3 — coupons,
which DID get it, as the contrast that proves the diagnosis.)

**Fix (committed `1a6331c94`):** `normalizeProductPayload` now does
`out.price_cents = Math.round(dollars * 100)` and deletes the stray
`price`. `Math.round` is mandatory per the i64-cents rule:
`19.99 * 100 = 1998.9999…` in float — truncation corrupts by a cent.
Verified: `{type:'course',price:19.99}` →
`{product_type:'course',price_cents:1999}`. Covers both create (POST,
required) and update (PUT, Option) via the single normalizer chokepoint.
`pnpm check` 0/0/4541.

---

## F2 — P0 OPEN: subscription-plan create/edit drops the price

**Severity:** P0 for create (fails); P1 for edit (price silently ignored).
**Status:** NOT fixed — flagged for the operator (subscription billing is
the most sensitive surface; wanted eyes on the create-vs-edit-vs-`/price`
flow before patching).

**Trace:**
- `src/routes/admin/subscriptions/plans/+page.svelte:283-293` builds the
  update body by hand: `fetch('/api/admin/subscriptions/plans/${id}', {
  body: JSON.stringify({ ... price: editingPlan.price ... }) })`.
  `editingPlan.price` is a `number` dollars (`bind:value` at `:651`).
- `subscriptionPlansApi.create/.update` (`src/lib/api/admin.ts:1248/
  1257`) also send `JSON.stringify(data)` **raw — no normalizer**.
- Proxy `src/routes/api/admin/subscriptions/plans/+server.ts:63-82` is a
  pure passthrough (no remap).
- Rust `CreatePlanRequest.price_cents: i64` **required**,
  `UpdatePlanRequest.price_cents: Option<i64>`
  (`api/src/routes/subscriptions_admin.rs:119-136`).
- ⇒ Create: missing required `price_cents` → fails. Edit: `price` key is
  unknown to `UpdatePlanRequest` → **price change silently dropped**
  (Option, no error — the dangerous one).

**Nuance found while tracing (why I didn't blind-patch):** there is a
*separate* dedicated price-change endpoint used at
`plans/+page.svelte:234` (`POST /plans/${id}/price`). So plan price may
be intended to flow through that path, with the general create/update
deliberately price-less. This needs the operator/owner to confirm the
intended flow before a fix — patching the wrong path on a billing form
is worse than the documented bug.

**Recommended fix (once flow confirmed):** mirror the product fix — a
`normalizePlanPayload` (or reuse the pattern) that emits
`price_cents = Math.round(price * 100)` for both `subscriptionPlansApi`
and the hand-built bodies in `plans/+page.svelte`.

---

## F3 — VERIFIED CORRECT (no action): coupons

Investigated because `normalizeCouponPayload` (`admin.ts:806`) looked
like the same class. **It is correct** — the "Batch 1 cents refactor
(2026-04-28)" added `dollarsToCents('discount_value',
'discount_value_cents')` etc. (`:824-840`) with `Math.round`. Backend
`CreateCouponRequest` (`api/src/routes/admin.rs:528`) wants
`discount_value_cents: i64` / `min_purchase_cents` / `max_discount_cents`
— all produced. **Did not touch working code.** This file is the
*reference pattern* the product fix copied.

---

## F4 — i64-cents schema gap: migration 061 left two live columns INTEGER

**Severity:** P2 (latent i32 overflow at $21,474,836.47 on rollups, per
the CLAUDE.md money rule). **Status:** draft migration written; **apply
is the operator's call** (schema change on fintech prod).

Migration `061_money_cents_unification.sql` widened
`courses_enhanced.price_cents`, `indicators.sale_price_cents`,
`indicators_enhanced.price_cents` → BIGINT. It did **not** widen:
- `courses.price_cents` (`001_initial_schema.sql:126` `INTEGER NOT NULL`)
- `indicators.price_cents` (`015_consolidated_schema.sql:1074`
  `INTEGER DEFAULT 0`) — 061 widened this table's *sale*_price_cents but
  not the main price_cents; a gap in that prior fix, not deliberate.

`products.price_cents` (`015:111` INTEGER) deliberately **excluded** from
the migration: the products path stores money in a NUMERIC `price`
column (`(price*100)::BIGINT` on read), so its INTEGER `price_cents`
appears vestigial/derived — widening or dropping it is a separate
decision, not a correctness fix.

**Deliverable:** `api/migrations/068_complete_money_cents_bigint.sql` —
forward-only, idempotent, mirrors 061's exact safe `ALTER COLUMN … TYPE
BIGINT USING …::BIGINT` pattern. INTEGER→BIGINT is non-lossy. Apply only
after confirming live column types (`\d courses` / `\d indicators`) — if
already BIGINT it's a no-op; per CLAUDE.md, running migrations against
prod is the operator's explicit decision.

---

## F5 — minor (logged, not fixed)

- `src/lib/utils/readingAnalytics.ts:247` — `.catch(() => {})` silent
  swallow (CLAUDE.md landmine). Single non-test instance, low blast
  radius (analytics beacon). Pipe to a debug log or accept the swallow
  explicitly with a comment.

---

## Pattern / takeaway

The product P0 was not an isolated typo — it's one instance of a
recurring shape on this codebase: **a payload normalizer/form that
renames fields but doesn't complete the dollars→`*_cents` contract.**
Coupons got the fix (Batch 1), products didn't (P0-4 predated it),
subscription-plans never had a normalizer at all. Any *future* admin
money form should be checked against its Rust `*_cents: i64` struct
before it ships. A lint rule cannot catch this — only the trace can.
