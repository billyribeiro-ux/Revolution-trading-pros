# Merge — Batch 4 (re-subscribe history + coupon admin recreate-and-sync)

| Field | Value |
|---|---|
| Date | 2026-04-29 |
| Feature branch | `batch4-coupon-admin-followups` |
| Feature commit | `4ecbfd47c39b3bdfb6eabecebb4d43786e28f4a1` |
| Merge commit on main | `9094cac8738999f04f50717bcddda328ed5ab32e` |
| Merge strategy | `--no-ff` |
| Pushed to origin | ✅ `04a2a5d06..9094cac87  main -> main` |

## What landed

- **Migration 063** — drops `(user_id, plan_id)` UNIQUE constraint added in 056; replaces with partial unique on `(user_id, plan_id) WHERE status IN ('active','trialing','past_due')`. Adds partial unique on `stripe_subscription_id WHERE NOT NULL` for webhook-retry idempotency.
- **`handle_checkout_completed`** — switched from `ON CONFLICT (user_id, plan_id) DO UPDATE` to `ON CONFLICT (stripe_subscription_id) DO UPDATE`. Re-subscription writes a new row; cancelled history preserved.
- **Admin coupon edit** — recreate-and-swap when discount math changes (`discount_type`, `discount_value_cents`, `duration`, `duration_in_months`). Orphan rollback on DB-side failure. `security_events` audit row written on every successful recreate.
- **`POST /api/admin/coupons/:id/sync-to-stripe`** — backfill mirror; returns 400 if already synced.
- **`min_purchase` enforcement** — `/api/payments/checkout` and `/api/checkout` reject under-threshold orders before attaching the discount; comparison in cents.
- **Frontend coupon form** — duration dropdown (`once` / `forever` / `repeating`) and conditional `duration_in_months` input on create + edit pages.
- **`PAYMENTS_ARCHITECTURE_STANDARD.md` §7** — re-subscription paragraph rewritten to spec the new-row contract.

## Gates at merge time

| Gate | Result |
|---|---|
| `cargo check` | ✅ 0/0 (9.30s) |
| `cargo clippy --no-deps` | ✅ 0/0 (14.95s) |
| `pnpm check` (frontend) | ✅ 0 errors / 0 warnings (5217 files) |

## Runtime verification

12 scenarios documented in [BATCH4_RESULT.md](BATCH4_RESULT.md), each marked **PENDING OPERATOR** with copy-paste-ready commands. Operator to walk through and convert to PASS / FAIL with raw evidence captured beneath the summary.

## Next

Stop. Wait for go on Batch 5a.
