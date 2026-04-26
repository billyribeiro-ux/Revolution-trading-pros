# 03 — Commerce Audit (Products / Coupons / Orders / Cart)

Auditor: principal-engineer review, read-only
Date: 2026-04-26
Scope: every file under
`frontend/src/routes/admin/{products,coupons,orders,cart}` and
`frontend/src/routes/api/admin/{products,coupons,orders}`,
plus the Rust handlers and DB schema they hit, because in commerce
the bug lives at the seam.

The findings below name severity, the offending line, and what's
load-bearing about it. Nothing was modified.

## Files reviewed

Frontend pages:
- [`frontend/src/routes/admin/products/+page.svelte`](../../../frontend/src/routes/admin/products/+page.svelte)
- [`frontend/src/routes/admin/products/create/+page.svelte`](../../../frontend/src/routes/admin/products/create/+page.svelte)
- [`frontend/src/routes/admin/products/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/products/[id]/edit/+page.svelte)
- [`frontend/src/routes/admin/coupons/+page.svelte`](../../../frontend/src/routes/admin/coupons/+page.svelte)
- [`frontend/src/routes/admin/coupons/create/+page.svelte`](../../../frontend/src/routes/admin/coupons/create/+page.svelte)
- [`frontend/src/routes/admin/coupons/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte)
- [`frontend/src/routes/admin/orders/+page.svelte`](../../../frontend/src/routes/admin/orders/+page.svelte)
- [`frontend/src/routes/admin/cart/abandoned/+page.svelte`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte)

SvelteKit proxies:
- [`frontend/src/routes/api/admin/products/stats/+server.ts`](../../../frontend/src/routes/api/admin/products/stats/+server.ts)
- [`frontend/src/routes/api/admin/coupons/+server.ts`](../../../frontend/src/routes/api/admin/coupons/+server.ts)
- [`frontend/src/routes/api/admin/coupons/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/coupons/[...rest]/+server.ts)
- [`frontend/src/routes/api/admin/orders/+server.ts`](../../../frontend/src/routes/api/admin/orders/+server.ts)
- [`frontend/src/routes/api/admin/orders/[id]/+server.ts`](../../../frontend/src/routes/api/admin/orders/[id]/+server.ts)
- [`frontend/src/routes/api/admin/orders/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/orders/[...rest]/+server.ts)

Backend (read for impact analysis only):
- [`api/src/routes/products.rs`](../../../api/src/routes/products.rs)
- [`api/src/routes/coupons.rs`](../../../api/src/routes/coupons.rs)
- [`api/src/routes/admin.rs`](../../../api/src/routes/admin.rs) (coupon CRUD §`/admin/coupons`)
- [`api/src/routes/admin_orders.rs`](../../../api/src/routes/admin_orders.rs)
- [`api/src/routes/orders.rs`](../../../api/src/routes/orders.rs) (`admin_index`, stats, `AdminOrderResponse`)
- [`api/src/routes/mod.rs`](../../../api/src/routes/mod.rs) (mount points)
- [`api/src/models/product.rs`](../../../api/src/models/product.rs) (`CreateProduct`, `UpdateProduct`)
- [`api/migrations/001_initial_schema.sql`](../../../api/migrations/001_initial_schema.sql) (canonical `coupons` table)

Library:
- [`frontend/src/lib/utils/createProxyShim.ts`](../../../frontend/src/lib/utils/createProxyShim.ts)
- [`frontend/src/lib/api/admin.ts`](../../../frontend/src/lib/api/admin.ts)
- [`frontend/src/lib/api/abandoned-carts.ts`](../../../frontend/src/lib/api/abandoned-carts.ts)

---

## Critical bugs (P0)

These break the feature outright; admins will see something that "looks wired" and fails when clicked.

### P0-1 — Coupon edit page is unreachable: backend has no `PUT /admin/coupons/:id`

The admin router at [`api/src/routes/admin.rs:1670-1673`](../../../api/src/routes/admin.rs#L1670-L1673) registers only:

```rust
.route("/coupons", get(list_coupons).post(create_coupon))
.route("/coupons/:id", delete(delete_coupon))
.route("/coupons/validate/:code", get(validate_coupon))
```

There is no `.put(update_coupon)` and no `.get(get_coupon)`. The frontend edit page calls both:

- [`frontend/src/routes/admin/coupons/edit/[id]/+page.svelte:140`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L140) `couponsApi.get(couponId)` → `GET /api/admin/coupons/:id`
- [`frontend/src/routes/admin/coupons/edit/[id]/+page.svelte:223`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L223) `couponsApi.update(couponId, …)` → `PUT /api/admin/coupons/:id`

Both will 404/405. The edit screen cannot load the coupon and cannot save changes. (A separate `update_coupon` exists in [`api/src/routes/coupons.rs:460`](../../../api/src/routes/coupons.rs#L460), but it is mounted at `/coupons`, not `/admin/coupons` — see [`mod.rs:105`](../../../api/src/routes/mod.rs#L105) — and uses an *incompatible* schema; see P0-2.)

### P0-2 — Two coupon routers contradict the DB schema; the public one would crash on insert

Migration [`001_initial_schema.sql:310-327`](../../../api/migrations/001_initial_schema.sql#L310-L327) defines columns `discount_type, discount_value, min_purchase, max_discount, usage_limit, usage_count, starts_at, expires_at, applicable_products, applicable_plans`.

- [`admin.rs:534`](../../../api/src/routes/admin.rs#L534) `INSERT INTO coupons (code, description, discount_type, discount_value, min_purchase, max_discount, usage_limit, usage_count, is_active, starts_at, expires_at, applicable_products, applicable_plans, …)` — matches schema. ✓
- [`coupons.rs:407-414`](../../../api/src/routes/coupons.rs#L407-L414) `INSERT INTO coupons (code, type, value, max_uses, current_uses, expiry_date, applicable_products, min_purchase_amount, is_active, …)` — none of `type`, `value`, `max_uses`, `current_uses`, `expiry_date`, `min_purchase_amount` exist. Any POST to `/api/coupons` returns 500 with a column-not-found.

Either rip out `coupons.rs::create_coupon`/`update_coupon`/`delete_coupon`/`get_coupon`/`list_coupons` (they're dead and broken), or migrate the schema. Pick one writer.

### P0-3 — Coupon create payload field-renames the entire DTO; backend silently drops most fields

Frontend [`coupons/create/+page.svelte:262-277`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L262-L277) sends:

```js
const payload = {
    code, type, value, description,
    minimum_amount, max_discount_amount, usage_limit,
    valid_from, valid_until,
    applicable_products: …, is_active
};
```

Backend [`admin.rs:475-488`](../../../api/src/routes/admin.rs#L475-L488) `CreateCouponRequest` expects:

```rust
code, description, discount_type, discount_value,
min_purchase, max_discount, usage_limit, is_active,
starts_at, expires_at, applicable_products, applicable_plans
```

Every renamed key (`type→discount_type`, `value→discount_value`, `minimum_amount→min_purchase`, `max_discount_amount→max_discount`, `valid_from→starts_at`, `valid_until→expires_at`) is dropped because `serde::Deserialize` defaults to `None` for missing fields. Net effect: a coupon is created with `code` + `description`, an *empty* `discount_type` (silently inserted as `""`), `discount_value=0`, no expiration, no usage limit, always-active. Production-grade footgun.

### P0-4 — Product create/edit silently strips `sale_price`, `currency`, `features`, `slug` (and the type filter is broken)

[`api/src/models/product.rs:67-87`](../../../api/src/models/product.rs#L67-L87) `CreateProduct` has fields `name, product_type, description, long_description, price, is_active, metadata, thumbnail, meta_title, meta_description, indexable, canonical_url`. Frontend product create page [`products/create/+page.svelte:121-137`](../../../frontend/src/routes/admin/products/create/+page.svelte#L121-L137) sends `name, slug, type, sale_price, currency, features, …`. The backend:

- ignores `slug` (it server-generates from `name` at [`products.rs:220`](../../../api/src/routes/products.rs#L220), making the user-typed slug field non-functional)
- ignores `sale_price`, `currency`, `features`
- expects `product_type`, not `type` — the value `formData.type = 'course'` is dropped, so all created products fall through to whatever NULL/default `type` ends up as (likely a 500 since `type` is `NOT NULL` in `products` table)

[`products.rs:24`](../../../api/src/routes/products.rs#L24) `ProductListQuery.product_type` — frontend sends `?type=course` from [`products/+page.svelte:74-76`](../../../frontend/src/routes/admin/products/+page.svelte#L74-L76); backend expects `?product_type=course`. Filter has never worked.

Also, [`models/product.rs:15-19`](../../../api/src/models/product.rs#L15-L19) `ProductType` enum is `Course | Indicator | Bundle` — there is no `Membership`, but the UI offers it ([`products/create/+page.svelte:51-56`](../../../frontend/src/routes/admin/products/create/+page.svelte#L51-L56)). Choosing "Membership" sends a string `type=membership` that won't match the enum if it's ever validated.

### P0-5 — Products `+page.svelte` $effect uses `untrack`, so it never re-fires

```svelte
// frontend/src/routes/admin/products/+page.svelte:160-166
$effect(() => {
    // Track selectedType for reactivity
    // Use untrack to avoid infinite loops when updating products
    untrack(() => {
        loadProducts();
    });
});
```

The body is wrapped in `untrack`, so `selectedType` is never read by the effect's dependency tracking. The comment says "Track selectedType for reactivity" but no reference exists. The effect runs exactly once (on mount). Clicking a type tab at [`+page.svelte:215`](../../../frontend/src/routes/admin/products/+page.svelte#L215) updates `selectedType` but never reloads. Replace with explicit click handler `onclick={() => { selectedType = type.value; loadProducts(); }}` or simply read `selectedType` *outside* the `untrack` block.

The same anti-pattern is in [`products/[id]/edit/+page.svelte:262-269`](../../../frontend/src/routes/admin/products/[id]/edit/+page.svelte#L262-L269), where `untrack` makes `productId` re-loads not happen if the user navigates between two product IDs without unmounting.

### P0-6 — The whole `/admin/cart/abandoned` page is an orphan (no backend)

The page at [`admin/cart/abandoned/+page.svelte`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte) imports the `abandoned-carts` API client. That client is self-flagged as orphaned at [`abandoned-carts.ts:1-11`](../../../frontend/src/lib/api/abandoned-carts.ts#L1-L11):

```ts
/**
 * ⚠️ FIX-2026-04-26: ORPHAN — no matching backend route exists.
 * This client file was identified by the 2026-04-26 audit as having no Rust
 * handler under api/src/routes/. Calls from this file will 404.
 */
const API_BASE = '/api/admin/abandoned-carts';
```

Confirmed: nothing in `api/src/routes/` mounts `/abandoned-carts` or registers a handler for it (only `crm.rs:3352` has a single `bulk_delete_abandoned_carts` for `/admin/crm/abandoned-carts`, which is a different path). Every load on `/admin/cart/abandoned` hits 404 → toast "Failed to load abandoned carts data".

Per CLAUDE.md memory rule "CREATE, never DELETE — orphan = build the missing side." Either build the routes or move the page behind a feature flag; do not delete.

### P0-7 — Coupon code uppercased on every keystroke triggers cursor-jump and corrupts paste

[`coupons/edit/[id]/+page.svelte:460`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L460):

```svelte
oninput={() => (formData.code = formData.code.toUpperCase())}
```

Combined with `bind:value`, this is a write-while-reading update on every keystroke. On platforms where `oninput` fires before the binding flushes, the cursor jumps to end-of-input. More importantly, lowercase paste of e.g. `summer2024` sometimes loses the trailing chars on slow renders. Standard fix: rely on a CSS `text-transform: uppercase` for display, then `formData.code.trim().toUpperCase()` only at submit time. (The create page uses the cleaner submit-time approach already at [`coupons/create/+page.svelte:263`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L263).)

### P0-8 — `productsApi.create` for a "Membership" silently writes `type='membership'` but the schema doesn't accept it

The backend `ProductType` enum has only `Course/Indicator/Bundle`. The DB column `products.type` is plain `text` (it's `#[sqlx(type_name = "text")]` on the Rust enum, not a postgres enum), so the row gets inserted with `type='membership'`. The product list page then groups "by type" client-side and the type-icon lookup at [`products/+page.svelte:142-150`](../../../frontend/src/routes/admin/products/+page.svelte#L142-L150) maps `membership → IconCrown`, so it looks fine — but every other consumer of `Product.type` in Rust code that expects the enum will fail to deserialize. Examples to check: any handler that does `Json<Vec<Product>>` rather than `Json<Vec<ProductRow>>`.

### P0-9 — `requestManager.deduplicateRequest` keys by method+endpoint, ignoring body — concurrent mutations collapse

[`admin.ts:557`](../../../frontend/src/lib/api/admin.ts#L557):

```ts
const dedupeKey = `${fetchOptions.method || 'GET'}:${endpoint}`;
```

Two concurrent `POST /admin/coupons` calls with different bodies share the same key and the second caller receives the *first* request's response. For idempotent GETs this is fine; for `POST/PUT/DELETE` it's a correctness bug. The cache-key building at [`admin.ts:397-401`](../../../frontend/src/lib/api/admin.ts#L397-L401) does include the body and is correctly off for non-GET — but the dedupe path doesn't. Quick fix: include `options.body` in the dedupe key, or skip dedupe entirely for non-GET methods.

---

## High-severity issues (P1)

### P1-1 — Money is `f64` end-to-end; no integer-cents pipeline

`AdminOrderResponse.total: f64` ([`orders.rs:465`](../../../api/src/routes/orders.rs#L465)), `Product.price: f64`, `CouponRow.discount_value: f64`. The orders page formats with `Intl.NumberFormat({ style: 'currency' })` ([`admin/orders/+page.svelte:133-138`](../../../frontend/src/routes/admin/orders/+page.svelte#L133-L138)) which assumes a "major-unit" number, so `total=1.20` displays as `$1.20`. SQL stores `DECIMAL(10,2)` and casts to `FLOAT8` in transit ([`orders.rs:512`](../../../api/src/routes/orders.rs#L512), [`admin.rs:502-505`](../../../api/src/routes/admin.rs#L502-L505)). Two compounding problems:

1. Floats lose precision on tax/discount math. A 7.5% discount on $129.99 plus tax at the boundary of common sale prices won't reliably round-trip through `f64`. Move to `rust_decimal::Decimal` end-to-end (server) and `bigint cents` (wire).
2. Frontend `formatPrice(price.toFixed(2))` ([`products/+page.svelte:152-157`](../../../frontend/src/routes/admin/products/+page.svelte#L152-L157)) hardcodes `$` regardless of `currency`. The order detail does respect currency via `Intl.NumberFormat`, so the system is inconsistent.

### P1-2 — `admin_index` pagination total ignores filters

[`orders.rs:610`](../../../api/src/routes/orders.rs#L610):

```rust
let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
    .fetch_one(&state.db.pool).await.unwrap_or(0);
```

The list query honors `status` and `search` filters ([`orders.rs:524-540`](../../../api/src/routes/orders.rs#L524-L540)), but the count query doesn't. The pagination control at [`admin/orders/+page.svelte:412-441`](../../../frontend/src/routes/admin/orders/+page.svelte#L412-L441) shows e.g. "Showing 1 to 25 of 1,847 orders" while the filtered set is 12 rows — and the `total_pages` math will let admins click into pages that return `[]`.

Also note `unwrap_or(0)` swallows DB errors. Per CLAUDE.md "Don't swallow errors with `unwrap_or_default()` … propagate via `?`."

### P1-3 — Coupon code validation regex accepts case-insensitive but DB stores uppercase

[`coupons/create/+page.svelte:213`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L213): `/^[A-Z0-9_-]+$/i` — note the `/i` flag — accepts lowercase. Submit then forces `.toUpperCase()` at line 263. So a user typing `Summer2024` passes validation, gets uppercased on submit. The backend [`admin.rs:535`](../../../api/src/routes/admin.rs#L535) ALSO does `UPPER($1)`. This is fine in isolation but adjacent to P0-7 (cursor-jump in the edit page).

### P1-4 — `validateForm` allows expired coupons on edit

[`coupons/edit/[id]/+page.svelte:267-323`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L267-L323) checks `end <= start` but never checks `start < now()` or `end < now()`. An admin can save a coupon with `valid_until = 2020-01-01` and the UI accepts it. Similarly the create form at [`coupons/create/+page.svelte:225-231`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L225-L231) accepts past `starts_at`. The list page does color-code expired ([`coupons/+page.svelte:282`](../../../frontend/src/routes/admin/coupons/+page.svelte#L282)), but there's no save-time guard.

### P1-5 — Coupon `value` validation gap — negative values blocked, but `NaN` is not

[`coupons/create/+page.svelte:217-219`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L217-L219):

```js
if (!formData.discount_value || formData.discount_value <= 0) {
    errs.push('Discount value must be greater than 0');
}
```

`!NaN` is `true` so this catches `NaN`. But `parseFloat('1e9999') === Infinity` is *not* caught — `Infinity > 0` is true and `Infinity > 100` is true so the `>100` check fires for percentage but for `fixed` an admin can save a coupon with `value: Infinity`. Backend f64 gets `+Inf`, SQL `DECIMAL(10,2)` errors out at 500. Add `Number.isFinite(value)`.

Same issue on `min_purchase`, `max_discount`, `usage_limit` — none of which are checked at all.

### P1-6 — Coupon edit page silently discards `valid_from` if blank, but UPDATE turns "no value" into NULL

[`coupons/edit/[id]/+page.svelte:214-217`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L214-L217):

```ts
valid_from: formData.valid_from ? new Date(formData.valid_from).toISOString() : undefined,
valid_until: formData.valid_until ? new Date(formData.valid_until).toISOString() : undefined,
```

If the admin clears the start date in the input, `formData.valid_from === ''`, the field is sent as `undefined`, omitted from JSON, and *not* updated server-side. The user sees the old date persist. To clear a date the admin would have to know to send `null`. Either translate `''` → `null` or surface a "clear" button.

### P1-7 — `coupon.applicable_plans` is collected in UI but never sent

[`coupons/create/+page.svelte:90-96, 178-186`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L90-L96) builds `selectedPlans: Set<number>` and offers a UI for it, but the submit payload at [`262-277`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L262-L277) never references it. The `applicable_plans` column exists in the DB ([`001_initial_schema.sql:324`](../../../api/migrations/001_initial_schema.sql#L324)) and in the backend `CreateCouponRequest` ([`admin.rs:487`](../../../api/src/routes/admin.rs#L487)), but the frontend doesn't wire it.

### P1-8 — Order status filter is hard-coded; doesn't include all backend statuses

[`admin/orders/+page.svelte:319-326`](../../../frontend/src/routes/admin/orders/+page.svelte#L319-L326) lists `pending, completed, refunded, partial_refund, failed`. The orders schema in DB likely also has `processing, cancelled, fulfilled` (used by `admin_cancel`/`admin_fulfill`). Selecting `partial_refund` works because the `status_color` map handles it, but the filter dropdown is incomplete and there's no way to filter by `cancelled`. Inspect [`api/migrations`] for the actual order-status set and align.

### P1-9 — `bind_count` reused for $-positional substitution in admin_index

[`orders.rs:531-538`](../../../api/src/routes/orders.rs#L531-L538):

```rust
sql.push_str(&format!(
    " AND (o.order_number ILIKE ${0} OR u.email ILIKE ${0} OR u.name ILIKE ${0})",
    bind_count
));
```

Using positional argument 0 is correct (`{0}` substitutes `bind_count`'s value). However if both `status` and `search` are provided, the SQL becomes `... AND o.status = $1 AND (... ILIKE $2 OR ... ILIKE $2 OR ... ILIKE $2)` and the match-arm at line 561-568 binds in the right order. This works but is fragile — if anyone refactors the match arms order this drifts silently. Prefer `sqlx::QueryBuilder` which type-checks bind count.

### P1-10 — Hard-delete product, no cascade analysis, no transaction

[`products.rs:347`](../../../api/src/routes/products.rs#L347):

```rust
sqlx::query("DELETE FROM products WHERE id = $1").bind(id).execute(...)
```

This is a hard delete in a table referenced by `order_items`, `user_products`, `cart_items`. Either:
- the FK is `ON DELETE CASCADE` and historical orders lose their product reference (audit trail destroyed), or
- the FK is `ON DELETE RESTRICT` and the delete fails opaquely with "Failed to delete product" toast.

The handler comment at [`products.rs:411-412`](../../../api/src/routes/products.rs#L411-L412) acknowledges archive/restore should exist, and `archive_product`/`restore_product` are registered, but the destructive `DELETE` route is still wired and the frontend uses it ([`admin/products/+page.svelte:108`](../../../frontend/src/routes/admin/products/+page.svelte#L108)). The product list's confirmation modal text "This action cannot be undone" is at least honest.

### P1-11 — Order `loadOrderDetail` mutates global `selectedOrder` then races the fetch

[`admin/orders/+page.svelte:126-130`](../../../frontend/src/routes/admin/orders/+page.svelte#L126-L130):

```js
function openOrderDetail(order: Order) {
    selectedOrder = order;
    showDetailModal = true;
    loadOrderDetail(order.id);   // not awaited
}
```

If the admin clicks order A, then quickly clicks order B before A's fetch returns, A's late response writes to `orderDetail` and the modal — currently labeled "Order #B" — shows A's items. Cancel via `AbortController` keyed by the latest selected ID, or only render `orderDetail` if `orderDetail.id === selectedOrder.id`.

---

## Medium issues (P2)

### P2-1 — Admin coupons `+server.ts` returns `EMPTY_DATA` on backend 500

[`api/admin/coupons/+server.ts:53-55`](../../../frontend/src/routes/api/admin/coupons/+server.ts#L53-L55):

```ts
if (!response.ok) {
    return json(EMPTY_DATA);   // {"coupons": [], "total": 0}
}
```

The frontend list page treats this as a successful empty response and shows "No coupons yet" → "Create First Coupon" CTA. An admin will *create a duplicate* of an existing coupon because they can't see it. Forward a 5xx as a 5xx so the page can render its retry state ([`admin/coupons/+page.svelte:249-253`](../../../frontend/src/routes/admin/coupons/+page.svelte#L249-L253) already has one). The `[...rest]/+server.ts` shim correctly forwards status codes, so this asymmetry is even more confusing.

Also the response shape `{coupons:[], total:0}` doesn't match the page's expected `response.data || []` which checks `.data`, not `.coupons`. The list page will render nothing on 5xx because `coupons = response.data || []` evaluates to `[]` either way, but other consumers will break.

### P2-2 — `products/stats/+server.ts` swallows everything to mock data

[`api/admin/products/stats/+server.ts:15-58`](../../../frontend/src/routes/api/admin/products/stats/+server.ts#L15-L58) returns the same hardcoded `{total:0, active:0, …}` for every error case, including auth failures. An admin whose token expired sees "0 products" instead of being redirected to login. Bubble up at least 401/403.

### P2-3 — `productsApi.create`'s feature list and meta fields are sent but ignored

[`products/create/+page.svelte:130`](../../../frontend/src/routes/admin/products/create/+page.svelte#L130) sends `features: validFeatures` but the backend `CreateProduct` struct ([`models/product.rs:67-87`](../../../api/src/models/product.rs#L67-L87)) has no `features` field. There is no `features` column in the products DB schema either (nor `currency`, nor `sale_price`). Either:
- migrate `products` to add these columns, or
- store them in `metadata: Option<serde_json::Value>` (which the backend *does* accept) — then the frontend should serialize features into metadata.

The same applies to the entire SEO settings panel's interaction with sale_price.

### P2-4 — Order detail modal renders without checking `orderDetail.items`

[`admin/orders/+page.svelte:544-552`](../../../frontend/src/routes/admin/orders/+page.svelte#L544-L552):

```svelte
{#each orderDetail.items as item}
```

If the backend returns an order with no items array (legacy orders, malformed data), this throws "items is undefined". Wrap in `{#if orderDetail.items?.length}`.

### P2-5 — `validateForm` per-user-limit check is on create only

[`coupons/create/+page.svelte:233-237`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L233-L237) checks `usage_limit_per_user > usage_limit`, but the *edit* form ([`coupons/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte)) doesn't even have a `per_user_limit` field. Backend `CreateCouponRequest` doesn't have it either ([`admin.rs:475-488`](../../../api/src/routes/admin.rs#L475-L488)). The UI input at [`coupons/create/+page.svelte:491-501`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L491-L501) is decorative.

### P2-6 — `admin_orders` `total_count` query doesn't `JOIN` user/order_items so it's correct count, but pagination `total_pages` calc uses different filtered total_count vs unfiltered — see P1-2.

(Combined with P1-2 above for clarity.)

### P2-7 — Cart abandoned page redirects unauthenticated users via `goto('/admin')` instead of `/login`

[`admin/cart/abandoned/+page.svelte:250`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte#L250) "Back to Admin" goes to `/admin`. There's no 401-handling like the products page has at [`admin/products/+page.svelte:81-84`](../../../frontend/src/routes/admin/products/+page.svelte#L81-L84). If the API client throws 401 the toast just says "Failed to load" — admin gets stuck.

### P2-8 — `loadCarts` has no try/catch failure visualization

[`admin/cart/abandoned/+page.svelte:80-97`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte#L80-L97) — only console.error on failure. An empty result and a failed load look identical to the user.

### P2-9 — Recovery email modal arms `selectedCarts` for entire page even if you only wanted "one"

[`admin/cart/abandoned/+page.svelte:415-419`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte#L415-L419) on the "Send First Reminder" quick-action selects every pending cart on the *current page*. If pagination is on page 5, only that page's pendings get selected — admin thinks "Send to 47 carts" actually means "send to my entire pending pool". Misleading. Either confirm with cart count or operate on full backend filter set.

### P2-10 — `discount_value` in coupon list page reads `coupon.value` not `coupon.discount_value`

[`admin/coupons/+page.svelte:135-143`](../../../frontend/src/routes/admin/coupons/+page.svelte#L135-L143):

```js
function formatDiscountValue(coupon: Coupon): string {
    if (coupon.type === 'percentage') {
        return `${coupon.value}% off`;
    }
    …
}
```

But the backend [`admin.rs:455-471`](../../../api/src/routes/admin.rs#L455-L471) `CouponRow` returns `{discount_type, discount_value, …}` — there is no `type` or `value` on the returned JSON. The TypeScript `Coupon` interface at [`admin.ts:97-111`](../../../frontend/src/lib/api/admin.ts#L97-L111) declares `type` and `value`, but those fields don't exist in the response. `coupon.value` is `undefined`; the badge renders "undefined% off". This is the consequence of the type/discount_type schism in P0-2/P0-3.

### P2-11 — Coupon detail edit page reads/writes `coupon.minimum_amount` and `coupon.usage_limit` consistently with backend schema, but writes `valid_from / valid_until` mapped to backend `starts_at / expires_at`

[`coupons/edit/[id]/+page.svelte:153-154`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L153-L154) reads from `coupon.valid_from` (which doesn't exist on the response — it's `starts_at`) and writes to `valid_from` (which the backend doesn't accept — it expects `starts_at`). Round-trip is broken in both directions.

### P2-12 — `coupons.rs::list_coupons` and `admin.rs::list_coupons` both exist with different behaviors

[`coupons.rs:297`](../../../api/src/routes/coupons.rs#L297) uses different schema columns than [`admin.rs:492`](../../../api/src/routes/admin.rs#L492). Pick one. Same with `validate_coupon` (one in each file).

---

## Low / nits (P3)

### P3-1 — `IconFilter` imported but unused in [`admin/orders/+page.svelte:16`](../../../frontend/src/routes/admin/orders/+page.svelte#L16)

Used. (Disregard.)

### P3-2 — Duplicate `id="page-checkbox"` and `id="page-formdata-is-active"` etc.

[`coupons/create/+page.svelte:599, 634`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L599) and [`cart/abandoned/+page.svelte:520, 541`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte#L520) all use the same `id="page-checkbox"` for every row in a loop. HTML `id` must be unique. Screen readers and `<label for>` break. Drop the `id` (clicks already bound) or template it with the row id.

### P3-3 — `let SvelteComponent_1 = $derived(previewTypeIcon)` is a leftover from migration

[`products/create/+page.svelte:165`](../../../frontend/src/routes/admin/products/create/+page.svelte#L165) and the matching `<SvelteComponent_1 size={14} />` looks like it survived an automatic Svelte 5 migration. Replace with `<previewTypeIcon size={14} />` (Svelte 5 supports rune-typed components directly).

### P3-4 — `admin/products/+page.svelte:26`

```svelte
// @ts-ignore write-only state
let selectedType = $state('all');
```

The `@ts-ignore` is stale — the variable is read in `loadProducts`, `productCountByType`, etc.

### P3-5 — Coupons edit page dead types

[`coupons/edit/[id]/+page.svelte:46-53`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L46-L53) declares `'bogo' | 'free_shipping' | 'tiered' | 'bundle' | 'cashback' | 'points'` in the `type` union but the dropdown UI at [`469-474`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L469-L474) only offers 3 options. The discount-display function at [`111-119`](../../../frontend/src/routes/admin/coupons/edit/[id]/+page.svelte#L111-L119) handles only 3 types and returns "Custom Discount" for the rest.

### P3-6 — Hardcoded "1500ms then redirect" UX after coupon create

[`coupons/create/+page.svelte:285-287`](../../../frontend/src/routes/admin/coupons/create/+page.svelte#L285-L287) — fine but the toast pattern used elsewhere is more consistent.

### P3-7 — Stats math divides by zero check

[`admin_orders.rs:286-290`](../../../api/src/routes/admin_orders.rs#L286-L290) handles div-by-zero correctly. Good.

### P3-8 — Date formatting inconsistency

The orders page formats dates with `Intl.DateTimeFormat` ([`admin/orders/+page.svelte:141-150`](../../../frontend/src/routes/admin/orders/+page.svelte#L141-L150)) while the coupons list page hand-rolls it ([`admin/coupons/+page.svelte:145-152`](../../../frontend/src/routes/admin/coupons/+page.svelte#L145-L152)). Pick one.

### P3-9 — `formatCurrency(amount: number, currency = 'USD')`

[`admin/orders/+page.svelte:133`](../../../frontend/src/routes/admin/orders/+page.svelte#L133) defaults to `USD` even when the order has a non-USD currency. The signature *takes* currency, but several call sites omit it (line 493, 498). Pass `orderDetail.currency` explicitly everywhere.

### P3-10 — `admin/orders` `selectedOrder` and `orderDetail` are two separate sources of truth

[`admin/orders/+page.svelte:66-69`](../../../frontend/src/routes/admin/orders/+page.svelte#L66-L69). The header uses `selectedOrder.order_number`, the body uses `orderDetail.subtotal`. After the open-detail race in P1-11, these can show two different orders simultaneously.

### P3-11 — The `page.svelte` for `admin/products` `formatPrice` returns hardcoded USD `$`

See P1-1 — even though `Product.currency` exists as a field, it's never displayed.

### P3-12 — `admin/coupons/edit/[id]/+page.svelte:69` uses `parseInt(page.params.id ?? '0')` instead of `Number(...)`

Cosmetic; both work, but `Number` is consistent with the products edit page.

### P3-13 — `couponsApi` defines `import`, `test`, `preview`, `generateCode`, `validate`, `checkCode`

[`admin.ts:776-858`](../../../frontend/src/lib/api/admin.ts#L776-L858) declares 6 endpoints under `/admin/coupons/{validate,check-code,generate-code,import,test,preview}`. None of them exist in the backend admin router (cf. [`admin.rs:1670-1677`](../../../api/src/routes/admin.rs#L1670-L1677)). Six callable but-orphan client methods. Either build them or `// TODO:` them out so future devs don't wire calls.

---

## Cross-cutting concerns

### CC-1 — Two coupon writers, one schema

P0-2 is the load-bearing concern. The admin commerce surface touches the `coupons` table through *two* routers (`admin.rs` and `coupons.rs`) that wrote to disjoint column sets. The list page reads what `admin.rs` writes; the public/auxiliary surface (and the `couponsApi.update` path the edit page calls) writes what `coupons.rs` thinks the table looks like, which is out-of-date with `001_initial_schema.sql`. Resolution path:
1. Pick a single writer (`admin.rs`'s schema is the one the migration matches — keep it).
2. Delete or stub `coupons.rs`'s admin handlers.
3. Add the missing PUT handler at `/admin/coupons/:id` and a GET `/admin/coupons/:id` to back the edit page.
4. Sync the TypeScript `Coupon` interface ([`admin.ts:97-111`](../../../frontend/src/lib/api/admin.ts#L97-L111)) with the response shape `{discount_type, discount_value, starts_at, expires_at, min_purchase, max_discount, applicable_products, applicable_plans}` — and rename consumers (`coupon.type` → `coupon.discount_type`, `coupon.value` → `coupon.discount_value`).

### CC-2 — Money types

Whole commerce surface is `f64` end-to-end. Migrate to integer cents at the wire boundary and `Decimal` server-side. Bonus: surfaces the currency-dollar-mismatch (P3-11).

### CC-3 — Authentication on destructive ops

All admin routes that perform writes (delete product, delete coupon, refund, cancel, fulfill) gate via `AdminUser` extractor — confirmed at [`products.rs:209, 254, 336, 419, 432`](../../../api/src/routes/products.rs#L209), [`admin.rs:494, 528, 575`](../../../api/src/routes/admin.rs#L494), [`admin_orders.rs:88, 246`](../../../api/src/routes/admin_orders.rs#L88). No issues here. The frontend proxies all check `rtp_access_token` cookie before forwarding (see [`createProxyShim.ts:52`](../../../frontend/src/lib/utils/createProxyShim.ts#L52), [`api/admin/orders/[id]/+server.ts:29`](../../../frontend/src/routes/api/admin/orders/[id]/+server.ts#L29), etc.). Solid.

### CC-4 — No transaction wrappers

Coupon create ([`admin.rs:524`](../../../api/src/routes/admin.rs#L524)) and product create ([`products.rs:207`](../../../api/src/routes/products.rs#L207)) are single-statement INSERTs so transactions aren't strictly needed, but any future "create coupon + assign to products + write audit log" composite needs `Pool::begin()`. Worth a `// TODO` comment near each.

The only multi-statement path I saw is the admin order list (1× select + 1× count + 1× stats) — not a mutation, so no transactional concern. Refunds/cancel/fulfill (in `orders.rs::admin_refund` etc., not audited here per scope) absolutely need transactions; flag for §5 of this audit series.

### CC-5 — Idempotency on mutations

There's no idempotency-key mechanism on POST/PUT/DELETE in the admin surface. A network blip during coupon create could double-create. SvelteKit `+server.ts` shims forward the request once but don't dedupe on user-side retries. If the `RequestManager.deduplicateRequest` were body-aware (P0-9 fix), it would partially address this within a single tab/session.

### CC-6 — `+server.ts` proxies all use `$env/dynamic/private` ✓

I checked every commerce proxy. All use the canonical `env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'` pattern. Compliance: 100% within scope. No regressions here.

### CC-7 — Duplicate `id` attributes on table-row inputs

Cross-cutting — see P3-2. Affects abandoned-carts and coupons-create. Easy global fix.

### CC-8 — Orphan endpoints

| Endpoint client expects | Backend status |
|--|--|
| `GET /api/admin/coupons/:id` | not registered (admin router has no `.get`) |
| `PUT /api/admin/coupons/:id` | not registered |
| `POST /api/admin/coupons/validate` | client-side only; admin router has `/coupons/validate/:code` — different shape |
| `POST /api/admin/coupons/check-code` | not registered |
| `POST /api/admin/coupons/generate-code` | not registered |
| `POST /api/admin/coupons/import` | not registered |
| `POST /api/admin/coupons/test` | not registered |
| `POST /api/admin/coupons/preview` | not registered |
| `POST /api/admin/products/bulk-update` | not registered |
| `POST /api/admin/products/:id/assign-user` | not registered |
| `POST /api/admin/products/:id/remove-user` | not registered |
| `GET /api/admin/products/:id/users` | not registered |
| `GET /api/admin/products/type/:type` | not registered |
| `GET /api/admin/abandoned-carts` (and all 5 sub-routes) | not registered |

Per the user's standing rule "CREATE, never DELETE — orphan = build the missing side": these need backend handlers, not client-side deletion.

---

## Summary

Commerce admin is in worse shape than the structural pieces around it. The `+server.ts` proxies are correctly wired and use `$env/dynamic/private` end-to-end, but the data flowing through them lands on a backend whose DTOs and routing are out of sync with the frontend types. The headline issues:

- **Coupon edit is fundamentally broken** (P0-1): no `PUT /admin/coupons/:id` handler exists.
- **Coupon create silently drops most fields** (P0-3): every payload key is renamed away from what `admin.rs::CreateCouponRequest` expects.
- **Two coupon writers fight over one schema** (P0-2 / CC-1): `admin.rs` writes to migration-correct columns; `coupons.rs` writes to columns that don't exist.
- **Product create's slug, sale_price, currency, features are silently dropped** (P0-4); the type filter (`?type=` vs `?product_type=`) has never worked.
- **Products page filter buttons don't refresh the list** (P0-5): `untrack` swallows the dependency.
- **Abandoned-carts is a fully-implemented frontend page with zero backend** (P0-6): self-flagged orphan.
- **Money is `f64` end-to-end** (P1-1 / CC-2): no integer-cents discipline; currency badge hardcodes `$`.
- **Orders pagination total ignores filters** (P1-2): admin can paginate into empty pages.
- **`requestManager` dedupes mutations by URL only** (P0-9): concurrent POSTs collapse.
- **At least 14 orphaned client endpoints** (CC-8) that compile but 404 in production.

None of these are SvelteKit-routing bugs. The proxy plumbing is fine. The contract between the SvelteKit page → SvelteKit proxy → Rust router → DB is what's broken, in two places (frontend↔backend field renames; backend-router↔backend-router schema disagreement).

Recommended next moves, ranked:
1. Fix P0-1, P0-2, P0-3 together — they're one bug ("coupon CRUD never worked end-to-end").
2. Fix P0-4 — product create/edit either accepts or rejects sale_price/features; pick one.
3. Fix P0-5 — easy `untrack` removal in two files.
4. Build or feature-flag the abandoned-cart backend (P0-6).
5. Body-key the dedupe map (P0-9).
6. Migrate to integer-cents (CC-2) before any new commerce feature lands.
