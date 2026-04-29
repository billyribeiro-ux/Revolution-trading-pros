# Batch 5a — consolidate-checkout

**Branch:** `consolidate-checkout` (off `main` at `9094cac87`)
**Status:** code complete + gates green; awaiting PR review.

## PR title (suggested)
```
refactor: collapse duplicate checkout endpoints into /api/checkout (Batch 5a)
```

## PR body (suggested)

### Summary

The codebase had two checkout endpoints — `POST /api/checkout` (in [api/src/routes/checkout.rs](api/src/routes/checkout.rs)) and `POST /api/payments/checkout` (in [api/src/routes/payments.rs](api/src/routes/payments.rs)) — that had drifted out of sync. The frontend has been on `/api/checkout` exclusively since the payments-fix-2026-04 cleanup ([frontend/src/lib/api/config.ts](frontend/src/lib/api/config.ts)). The version in `payments.rs` was unreachable from the SPA but still publicly callable, and during Batch 4 verification it was the *only* one that completed end-to-end with a coupon attached because the canonical `/api/checkout` had a latent column-name typo.

This branch:

1. **Drops `POST /api/payments/checkout`.** The handler `create_checkout` and its `CreateCheckoutRequest` / `CheckoutItem` / `CheckoutResponse` request/response types were removed from `payments.rs`. The route registration was struck from `payments::router()`. Stripe integration, webhooks, customer portal, refund, and summary endpoints all stay on `/api/payments/*`.
2. **Fixes a latent bug in `/api/checkout`.** The order-creation transaction in `routes/checkout.rs` referenced a non-existent column `coupons.current_uses` (the actual column is `usage_count`). The error was masked by `.ok()` on the failing query, but Postgres had already poisoned the surrounding transaction — every subsequent statement died with `current transaction is aborted`. Surfaced during Batch 4 scenario 4B (over-threshold checkout with coupon → HTTP 500). Now uses `usage_count` and propagates the error explicitly.

### Why this is a net delete

`/api/payments/checkout` had ~340 lines of subtly-different logic from `/api/checkout` (different `orders` insert shape, no order-items transaction wrapper, different coupon-validator wording). Keeping it would mean every coupon, billing, or migration change has to land twice. The frontend has zero call sites on it, and any external integration on the API side should be migrated to `/api/checkout`.

### Diff stat

```
api/src/routes/checkout.rs |  23 ++-
api/src/routes/payments.rs | 388 ++-------------------------------------------
2 files changed, 31 insertions(+), 380 deletions(-)
```

### Gates

| Gate | Result |
|---|---|
| `cargo check` (api/, --locked) | ✅ 0/0 |
| `cargo clippy --no-deps` (api/, --locked) | ✅ 0/0 |
| `pnpm check` (frontend/) | ✅ 0 errors / 0 warnings (5217 files) |

### Verification

- **Item 4B regression unblocked.** With the `usage_count` typo fixed, `/api/checkout` will now complete an over-threshold coupon checkout without a poisoned-transaction 500. (Cannot run end-to-end here because it requires Stripe Checkout completion in a browser; PR reviewer may verify with `curl` against a non-blocking plan or through the SPA.)
- **No frontend changes needed** — `frontend/src/lib/api/config.ts` already points exclusively at `/api/checkout`.
- **No DB changes.**

### Risk

Low. The deleted handler had no frontend caller. External callers (if any) would receive HTTP 404 on `/api/payments/checkout` and need to migrate to `/api/checkout`, which has equivalent semantics with a slightly different request shape (`CartItem { plan_id, product_id, quantity? }` vs the old `CheckoutItem { plan_id, product_id, name, quantity, is_subscription, interval }` — most fields the old handler accepted were either client-side hints we ignored or DB-derived fields that should never have been client-supplied in the first place).

### Files

- [api/src/routes/payments.rs](api/src/routes/payments.rs) — handler + types removed; router updated.
- [api/src/routes/checkout.rs](api/src/routes/checkout.rs) — `current_uses` → `usage_count` typo fix; explicit error propagation instead of swallowed `.ok()`.

### Test plan

- [ ] CI: cargo check / clippy / pnpm check pass on this branch.
- [ ] Smoke: `POST /api/checkout` with a free plan + valid coupon completes end-to-end (returns Stripe URL + persists order with `coupon_id` set, `usage_count` incremented).
- [ ] Smoke: `POST /api/payments/checkout` returns 404.
