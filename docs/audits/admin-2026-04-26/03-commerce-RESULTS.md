# 03 — Commerce Audit: RESULTS

Companion to [`03-commerce.md`](./03-commerce.md) and
[`03-commerce-DEFERRED.md`](./03-commerce-DEFERRED.md).

This file lists every commerce finding that was implemented in the
2026-04-26 surgical-fix pass, the file(s) touched, and the gate results at
the end. Items deferred (schema migrations, system-wide refactors,
new-feature backends) live in the DEFERRED doc.

---

## Implemented in this pass

### P0-1 — Coupon edit page now reachable: backend has GET / PUT for `/admin/coupons/:id`

- `api/src/routes/admin.rs` registers
  `.route("/coupons/:id", get(get_coupon).put(update_coupon).delete(delete_coupon))`
  alongside new `get_coupon` and `update_coupon` handlers that read/write
  the migration-correct `coupons` columns.
- Validates `discount_type` ∈ `{percentage, fixed}` and uses
  `COALESCE(NULLIF(...), …)` so partial updates leave untouched columns
  alone.
- The `[...rest]/+server.ts` proxy already forwards `/api/admin/coupons/:id`
  to the backend via `createProxyShim('/api/admin/coupons')`; no proxy
  change needed.

### P0-3 — Coupon create payload now uses canonical backend field names

- `frontend/src/routes/admin/coupons/create/+page.svelte` builds a payload
  with `discount_type` / `discount_value` / `min_purchase` / `max_discount` /
  `starts_at` / `expires_at` / `applicable_plans` (matches
  `admin.rs::CreateCouponRequest`).
- `frontend/src/lib/api/admin.ts` adds `normalizeCouponPayload()` which
  translates legacy keys (`type`, `value`, `valid_from`, `valid_until`,
  `minimum_amount`, `max_discount_amount`) → canonical names and strips
  fields the backend doesn't accept (`stackable`, `priority`,
  `campaign_id`, etc.). All `couponsApi.create` and `couponsApi.update`
  calls flow through it, so older callers keep compiling.
- `Coupon` and `CouponCreateData` interfaces grew canonical fields with
  legacy aliases kept optional for read-side back-compat (P2-10, P2-11).
- Edit page (`coupons/edit/[id]/+page.svelte`) now reads `discount_type` /
  `discount_value` / `starts_at` / `expires_at` from the response (with
  legacy fallbacks) and writes canonical names on save.

### P0-4 — Product create no longer drops `sale_price` / `currency` / `features` / `slug`; type filter param matches backend

- `frontend/src/lib/api/admin.ts::normalizeProductPayload()` renames
  frontend `type` → backend `product_type` and tucks `sale_price`,
  `currency`, `features`, `slug` into `metadata` (the backend's
  `Option<serde_json::Value>` catch-all). All `productsApi.create` /
  `productsApi.update` calls go through this.
- `frontend/src/routes/admin/products/+page.svelte::loadProducts()` sends
  `?product_type=` (was `?type=`, which the backend silently dropped).

### P0-5 — Products page filter buttons now refetch on click

- `frontend/src/routes/admin/products/+page.svelte` reads `selectedType`
  *outside* the `untrack` block so the effect's dependency tracker sees
  it. The fetch itself stays in `untrack` to avoid re-firing when
  `products` is rewritten.

### P0-7 — Coupon edit code field no longer cursor-jumps on every keystroke

- `frontend/src/routes/admin/coupons/edit/[id]/+page.svelte` removed the
  `oninput={() => formData.code = formData.code.toUpperCase()}` handler.
  Display uses `style="text-transform: uppercase"` and the submit handler
  does `formData.code.trim().toUpperCase()` once. This matches the
  create-page pattern (P1-3).

### P0-8 — `RequestManager.deduplicateRequest` no longer collapses concurrent mutations

- `frontend/src/lib/api/admin.ts::makeRequest` only routes `GET`/`HEAD`
  through dedupe. POST/PUT/DELETE skip the dedupe map entirely (an
  alternative to including the body hash in the key — same correctness,
  simpler). Cache-key building (`getCacheKey`) already includes the body.

### P1-2 — Admin orders pagination total now respects filters

- `api/src/routes/orders.rs::admin_index` builds a parallel `count_sql`
  with the same `status` / `search` WHERE clauses as the list query, so
  the paginator math matches the visible result set. Errors propagate via
  `?` instead of `unwrap_or(0)` (per CLAUDE.md error-handling rule).

### P2-1 — Coupons list proxy forwards real backend status (no more silent EMPTY_DATA on 5xx)

- `frontend/src/routes/api/admin/coupons/+server.ts` GET handler now
  forwards the upstream status verbatim on `!response.ok`, so the list
  page can render its retry state instead of "No coupons yet" (which
  invited admins to create duplicates).
- Network-level failures return 502 (was masked as 200 + EMPTY_DATA).

### P2-2 — Products stats proxy forwards 401/403 instead of masking as 0 stats

- `frontend/src/routes/api/admin/products/stats/+server.ts` forwards
  `401` and `403` verbatim so the admin shell can redirect to login.
  5xx still falls back to mock data (intentional graceful-degradation).

### P2-4 — Order detail modal guards against missing `items` array

- `frontend/src/routes/admin/orders/+page.svelte` uses
  `{#each orderDetail.items ?? [] as item}` to avoid throwing when a
  legacy or partial order returns without `items`.

### P1-7 — Coupon `applicable_plans` now wired

- The create page's `selectedPlans: Set<number>` is now serialized into the
  POST payload (was collected by the UI but never sent). Backend has
  accepted this column since the migration; no schema change needed.

---

## Touched files

### Frontend (`frontend/`)

- `src/lib/api/admin.ts`
  – grew `Coupon` / `CouponCreateData` canonical fields; added
  `normalizeCouponPayload` and `normalizeProductPayload`; routed all
  `couponsApi.create/update` and `productsApi.create/update` through them.
- `src/routes/admin/products/+page.svelte`
  – fixed `?type=` → `?product_type=`; un-broke `untrack` reactivity.
- `src/routes/admin/coupons/+page.svelte`
  – list page reads `discount_type` / `discount_value` / `expires_at` /
  `min_purchase` with legacy fallback (P2-10).
- `src/routes/admin/coupons/create/+page.svelte`
  – sends canonical field names; wires `applicable_plans` (P1-7).
- `src/routes/admin/coupons/edit/[id]/+page.svelte`
  – reads canonical fields; writes canonical fields; removed cursor-jumpy
  `oninput` (P0-7); fixed `hasChanges` derivation (P2-11).
- `src/routes/admin/orders/+page.svelte`
  – defensive `{#each orderDetail.items ?? [] …}` (P2-4).
- `src/routes/api/admin/coupons/+server.ts`
  – forwards real backend status on error (P2-1).
- `src/routes/api/admin/products/stats/+server.ts`
  – forwards 401/403 (P2-2); graceful-degradation kept for 5xx.

### Backend (`api/`)

- `src/routes/admin.rs`
  – router registers `get(get_coupon).put(update_coupon)` for
  `/coupons/:id`; added `get_coupon` + `update_coupon` handlers using the
  migration-correct schema.
- `src/routes/orders.rs::admin_index`
  – paginator total now respects `status` / `search` filters (P1-2);
  errors propagate via `?`.

### Docs

- `docs/audits/admin-2026-04-26/03-commerce-DEFERRED.md` (new) — see for
  P0-2/P0-6/P0-9, P1-1, P1-10, P2-12, CC-8 follow-ups.
- `docs/audits/admin-2026-04-26/03-commerce-RESULTS.md` (this file).

---

## Gate results

### `cargo check` (api)

```
✅ cargo-check: Success
```

### `cargo clippy` (api)

```
✅ cargo-clippy: Success
```

### `pnpm check` (frontend)

`5248 files, 1 error, 8 warnings.` All errors and warnings are out of
commerce scope:

- 1 error: `src/routes/admin/crm/sequences/+page.svelte` "Cannot find name
  'api'." — sister cluster (CRM), unrelated.
- 8 warnings: `src/routes/admin/crm/*/new/+page.svelte` — unused CSS
  selector `.page-description code`, all in CRM, unrelated.

**Commerce-file errors / warnings: 0 / 0.**

(Filtered: `pnpm check 2>&1 | grep -E "ERROR|WARNING" | grep -E
"coupons|products|orders|cart"` returned no rows.)

---

## Items intentionally deferred

See [`03-commerce-DEFERRED.md`](./03-commerce-DEFERRED.md) for full context.

- **P0-2 / P2-12 / CC-1** — `coupons.rs` admin handlers contradict the DB
  schema; consolidating writers needs its own pass.
- **P0-6** — `/admin/cart/abandoned` page has no backend; building 6 new
  Rust handlers is real feature work.
- **P0-9** — `Membership` not in backend `ProductType` enum; adding it
  needs an exhaustiveness sweep.
- **P1-1 / CC-2** — Money is `f64` end-to-end; integer-cents pipeline is a
  system-wide refactor.
- **P1-10** — Hard-delete product CTA; soft-delete migration / archive UX
  decision needed.
- **CC-8** — 12 of 14 orphan client endpoints (the other 2,
  `GET`/`PUT /admin/coupons/:id`, are landed in this pass).

---

## Out of scope (P3 nits not addressed)

- P3-2 duplicate `id` attributes on table-row inputs — accessibility nit,
  not load-bearing for behavior.
- P3-3 `SvelteComponent_1` migration leftover — cosmetic.
- P3-4 stale `@ts-ignore` on `selectedType` — cosmetic.
- P3-5 dead types in coupon edit form — cosmetic.
- P3-6 `1500ms then redirect` UX nit — cosmetic.
- P3-8 / P3-9 / P3-10 / P3-11 — date / currency / source-of-truth
  inconsistencies, intentional follow-ups grouped with P1-1 (money types).
- P3-12 / P3-13 — cosmetic / out-of-scope orphan client methods.

These can land in a janitorial PR after the deferred items.
