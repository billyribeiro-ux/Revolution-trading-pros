# 03 — Commerce Audit: DEFERRED items

These commerce findings from `03-commerce.md` were intentionally NOT fixed in
the same pass that landed the P0 mechanical fixes. They are deferred because
their blast radius extends beyond the commerce surface (DB schema migration,
system-wide type refactor, or building an entirely missing backend feature)
and they should be planned as discrete pieces of work with explicit review.

The list is the canonical follow-up backlog for commerce. Re-read
`03-commerce.md` for full context on each.

---

## P0-3 — Two coupon Rust routers contradict the schema (CC-1)

**Status:** DEFERRED — schema/router consolidation, blast radius beyond commerce.

`api/src/routes/coupons.rs::create_coupon`/`update_coupon`/`list_coupons`
write to columns that do not exist in `001_initial_schema.sql`
(`type`, `value`, `max_uses`, `current_uses`, `expiry_date`,
`min_purchase_amount`). Any POST to `/api/coupons` returns 500.

`api/src/routes/admin.rs` writes to the migration-correct columns
(`discount_type`, `discount_value`, `min_purchase`, `max_discount`,
`usage_limit`, `usage_count`, `starts_at`, `expires_at`).

The right resolution is to either:

1. Delete the `coupons.rs` admin handlers (they are dead and broken), or
2. Migrate the schema to whatever the public surface needs.

Picking either touches a routing surface that is mounted at `/coupons` (not
`/admin/coupons`) and needs a careful audit of who else reads from
`/api/coupons`. Out of scope for the surgical-fix pass.

**Owner follow-up:** decide whether `/api/coupons` is a public storefront
surface (in which case it needs a real read-only impl) or dead code (in which
case stub it out).

---

## P0-6 — `/admin/cart/abandoned` page is an orphan (no backend)

**Status:** DEFERRED — requires building 6 new Rust handlers.

`frontend/src/lib/api/abandoned-carts.ts` is self-flagged as an orphan; the
page at `/admin/cart/abandoned` calls `GET /api/admin/abandoned-carts` plus
five sub-routes (recovery email send, single-cart get, etc.) that do not
exist in `api/src/routes/`.

Per the standing rule "CREATE, never DELETE" we do not delete the page or the
client. Building the backend is a real feature (cart-snapshot ingestion +
expiration policy + recovery-email orchestration) and lands in its own PR.

**Owner follow-up:** scope an `abandoned_carts` table + 6 handlers
(`list`, `get_one`, `bulk_delete`, `send_recovery`, `recover`, `stats`),
ideally re-using the existing `crm::bulk_delete_abandoned_carts` shape.

---

## P0-9 — `Membership` product type isn't in the backend enum

**Status:** DEFERRED — backend `ProductType` enum schema change.

`api/src/models/product.rs::ProductType` is `Course | Indicator | Bundle`.
The admin UI offers `Membership` as a fourth option in
`frontend/src/routes/admin/products/create/+page.svelte:51-56`. Choosing it
sends `type=membership`, which the backend stores as a free text string
(the column is `text`, not a postgres enum) and which silently breaks any
typed deserialization downstream.

The right resolution is one of:

1. Add `Membership` to the Rust enum (1 line), audit every usage of
   `ProductType` to make sure no exhaustive `match` arm is missed.
2. Remove `Membership` from the UI options.

Picking (1) is a backend schema-shaped change that needs its own pass with
the system-wide product audit (see `09-system.md`).

**Owner follow-up:** add `Membership` variant + grep `ProductType::` for
exhaustiveness.

---

## P1-1 / CC-2 — Money is `f64` end-to-end

**Status:** DEFERRED — system-wide refactor.

`AdminOrderResponse.total: f64`, `Product.price: f64`,
`CouponRow.discount_value: f64`. Every commerce surface ships floats over
the wire. The right pipeline is `rust_decimal::Decimal` server-side and
integer-cents on the wire.

Fixing this touches every model, every handler that reads/writes money,
every frontend formatter, and the `Intl.NumberFormat` callsites — at minimum
it is a multi-hundred-line PR and needs a migration script for in-flight
orders. Out of scope for the surgical fix pass.

**Owner follow-up:** standalone `feat(commerce): integer-cents money pipeline`
PR with its own audit.

---

## P1-10 — Hard-delete `products.rs::delete_product` (cascade or restrict)

**Status:** DEFERRED — DB cascade behavior + soft-delete migration.

`DELETE FROM products WHERE id = $1` in `api/src/routes/products.rs:347`
runs without checking foreign keys to `order_items`, `user_products`,
`cart_items`. Either:

- the FKs are `CASCADE` and the audit trail dies, or
- the FKs are `RESTRICT` and the delete throws.

The handler's own comment acknowledges archive/restore should be the path.
Wiring the frontend list page to call `archive_product` instead of
`delete_product` is a 1-line frontend change, but requires deciding the
"already deleted via the destructive route" recovery story first.

**Owner follow-up:** flip the frontend delete CTA to `archive`, confirm
backwards compat.

---

## P2-12 / CC-1 — `coupons.rs::list_coupons` and `admin.rs::list_coupons` both exist

**Status:** DEFERRED — see P0-3 above.

Same root cause. Pick a single writer/reader pair and stub the other.

---

## CC-8 — 14 orphan client endpoints

**Status:** PARTIALLY ADDRESSED.

Coupon `GET /admin/coupons/:id` and `PUT /admin/coupons/:id` are now
backed (P0-1, this pass).

The remaining 12 orphans (`/admin/coupons/check-code`,
`/admin/coupons/generate-code`, `/admin/coupons/import`,
`/admin/coupons/test`, `/admin/coupons/preview`,
`/admin/coupons/validate` (POST shape), `/admin/products/bulk-update`,
`/admin/products/:id/assign-user`, `/admin/products/:id/remove-user`,
`/admin/products/:id/users`, `/admin/products/type/:type`, plus all 6
`/admin/abandoned-carts/*` routes) are deferred — none are reachable from
the admin shell today, and building them is real feature work.

**Owner follow-up:** decide per-endpoint whether to build or document as
"client API surface that intentionally has no backend yet."
