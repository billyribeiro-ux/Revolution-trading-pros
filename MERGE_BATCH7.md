# Batch 7 — pre-launch-checks

**Branch:** `pre-launch-checks` (off `main` at `9094cac87`)
**Status:** code complete + gates green; awaiting PR review.

## PR title (suggested)
```
feat(launch): production secret prefix assertion + reconcile schedule docs (Batch 7)
```

## PR body (suggested)

### Summary

Pre-launch hardening so a production deploy with test-mode credentials cannot silently boot. Three changes:

1. **`Config::validate_production_secrets()`** — new method called from `main.rs` immediately after `Config::from_env()`. When `ENVIRONMENT=production`, it panics at boot if any of:
   - `STRIPE_SECRET` does not start with `sk_live_`,
   - `STRIPE_WEBHOOK_SECRET` is empty / placeholder / not prefixed with `whsec_`,
   - `STRIPE_PUBLISHABLE_KEY` is set but does not start with `pk_live_`.
2. **Reconciliation scheduler comment fix.** The comment on `spawn_scheduler` claimed "runs once immediately at startup, then sleeps until 03:00 UTC." Reading the code shows the loop sleeps *first* and runs *after* — the first reconciliation pass after a deploy happens at the next 03:00 UTC, not at boot. Comment rewritten to reflect actual behaviour, with a pointer to `POST /api/admin/reconciliation/run` for on-demand runs.
3. **TASK7 re-run guidance.** [TASK7_RESULT.md](TASK7_RESULT.md) already records the 28 scenarios A-AB with PASS/INCONCLUSIVE/CONCERN status from the prior audit (most recently 2026-04-28 in the Batch 3 addendum). This branch does NOT re-execute those scenarios — most require a browser, Stripe Test Clocks, or operator interaction (same constraint as Batch 4). Instead, [BATCH7_RESULT.md](BATCH7_RESULT.md) lists which subset is unaffected vs which needs a fresh walk-through given the changes in Batches 4, 5a, 5b, 5c.

### Why "panic" instead of "return Result"

A production binary booting with `sk_test_...` is a security incident, not a startup hiccup. Returning an error would let an init-system retry layer (systemd, k8s liveness) keep restarting the binary in degraded mode — possibly accepting webhooks, possibly issuing real customer Checkout sessions against a test-mode account. The right behaviour is "kernel panic, page someone." `panic!` produces a non-zero exit + stderr message that's hard to miss in logs.

### Files

- [api/src/config/mod.rs](api/src/config/mod.rs) — `validate_production_secrets` method (~70 lines).
- [api/src/main.rs](api/src/main.rs) — single call site after `Config::from_env()`.
- [api/src/jobs/reconcile_stripe.rs](api/src/jobs/reconcile_stripe.rs) — comment fix on `spawn_scheduler`.
- [BATCH7_RESULT.md](BATCH7_RESULT.md) — TASK7 status + walk-through guidance for the operator.

### Diff stat

```
api/src/config/mod.rs           |  68 ++++++++
api/src/jobs/reconcile_stripe.rs|  10 +-
api/src/main.rs                 |   6 +
BATCH7_RESULT.md                | <new>
4 files changed, ~85 insertions, ~3 deletions
```

### Gates

| Gate | Result |
|---|---|
| `cargo check` (api/, --locked) | ✅ 0/0 |
| `cargo clippy --no-deps` (api/, --locked) | ✅ 0/0 |
| `pnpm check` (frontend/) | ✅ 0 errors / 0 warnings (5217 files) |

### Manual verification (before deploy)

1. **Local boot still works** with `ENVIRONMENT=development` and `STRIPE_SECRET=sk_test_*`:
   ```
   docker compose up -d api
   docker logs rtp-api 2>&1 | head -20
   # Should NOT panic; should NOT log "production_secrets_validated".
   ```
2. **Production boot fails fast** if any prefix is wrong:
   ```
   ENVIRONMENT=production STRIPE_SECRET=sk_test_xxx \
     STRIPE_WEBHOOK_SECRET=whsec_xxx STRIPE_PUBLISHABLE_KEY=pk_live_xxx \
     ./revolution-api
   # Expected: panic with "FATAL: ENVIRONMENT=production but STRIPE_SECRET does not start with 'sk_live_'..."
   ```
3. **Production boot succeeds** with all live keys:
   ```
   ENVIRONMENT=production STRIPE_SECRET=sk_live_... \
     STRIPE_WEBHOOK_SECRET=whsec_... STRIPE_PUBLISHABLE_KEY=pk_live_... \
     ./revolution-api
   # Expected: log "production_secrets_validated" + normal startup.
   ```

### Test plan

- [ ] CI green.
- [ ] Manual verification 1-3 above on staging.
- [ ] Reconciliation: at the next 03:00 UTC, observe one row added to `reconciliation_log`. (Or trigger immediately via `POST /api/admin/reconciliation/run` with admin auth.)
- [ ] TASK7 28 scenarios re-walk per BATCH7_RESULT.md — operator-driven.

### Unit tests

Eight `#[cfg(test)]` tests in `api/src/config/mod.rs` exercise every branch of `validate_production_secrets`:

| Test | Asserts |
|---|---|
| `..._passes_with_all_live_keys` | happy path: no panic when all three keys are live-mode |
| `..._is_noop_outside_production` | dev/staging stacks still boot with `sk_test_*` and placeholder webhook |
| `..._panics_on_test_secret` | `STRIPE_SECRET=sk_test_...` → panic with prefix message |
| `..._panics_on_placeholder_webhook_secret` | `STRIPE_WEBHOOK_SECRET=whsec_placeholder` → panic |
| `..._panics_on_empty_webhook_secret` | empty webhook secret → panic |
| `..._panics_on_bad_webhook_prefix` | webhook secret without `whsec_` prefix → panic |
| `..._panics_on_test_publishable` | `STRIPE_PUBLISHABLE_KEY=pk_test_...` → panic |
| `..._allows_empty_publishable` | empty publishable key is fine (frontend may pull from a separate env var) |

A test-only `live_config()` helper in `mod tests` constructs a fully-populated `Config` without reading process env. `#[should_panic(expected = "...")]` attributes lock in the exact error wording so future edits to the messages don't silently regress the operator-facing signal.

```
cargo test validate_production_secrets --lib
running 8 tests
... 8 passed; 0 failed
```
