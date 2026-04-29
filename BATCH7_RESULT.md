# Batch 7 — pre-launch-checks verification

Branch: `pre-launch-checks`
Status at write-time: code complete + gates green; runtime TASK7 re-run PENDING OPERATOR.

## Gate evidence

| Gate | Result |
|------|--------|
| `cargo check` (api/, --locked) | ✅ 0/0 |
| `cargo clippy --no-deps` (api/, --locked) | ✅ 0/0 |
| `pnpm check` (frontend/) | ✅ 0 errors / 0 warnings (5217 files) |

## Code-level evidence

| Item | Location |
|---|---|
| `validate_production_secrets()` checks `STRIPE_SECRET` for `sk_live_` prefix | [api/src/config/mod.rs](api/src/config/mod.rs) — `validate_production_secrets` |
| Same fn checks `STRIPE_WEBHOOK_SECRET` for `whsec_` + non-placeholder | [api/src/config/mod.rs](api/src/config/mod.rs) |
| Same fn checks `STRIPE_PUBLISHABLE_KEY` for `pk_live_` if non-empty | [api/src/config/mod.rs](api/src/config/mod.rs) |
| Wired into boot path right after `Config::from_env()` | [api/src/main.rs](api/src/main.rs) — after line 39 |
| Reconciliation scheduler comment corrected (was claiming first-run-at-boot; actual is first-run-at-next-03:00-UTC) | [api/src/jobs/reconcile_stripe.rs:22-32](api/src/jobs/reconcile_stripe.rs#L22-L32) |
| Reconciliation job spawned in main | [api/src/main.rs:120-121](api/src/main.rs#L120-L121) (was already at line 114 before this PR) |

## Reconciliation runs daily — verification

```bash
# 1. Confirm scheduler is alive after boot:
docker logs rtp-api 2>&1 | grep -E "scheduler_sleep|reconcile" | head -5
# Expected: a "Reconciliation job sleeping until 03:00 UTC" line with seconds count.

# 2. Force a run on demand (admin auth required):
ADMIN_TOKEN=$(curl -sS -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"welberribeirodrums@gmail.com","password":"<pass>"}' | \
  python3 -c "import sys,json; print(json.load(sys.stdin)['access_token'])")
curl -sS -X POST http://localhost:8080/api/admin/reconciliation/run \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# 3. Verify a reconciliation_log row was written:
docker exec rtp-db psql -U rtp -d revolution_trading_pros \
  -c "SELECT id, run_at, discrepancies_found FROM reconciliation_log ORDER BY id DESC LIMIT 3;"
```

## TASK7 re-run — 28 scenarios A-AB

The original TASK7_RESULT.md scored these scenarios on 2026-04-26, with addendum updates on 2026-04-28 (Batches 3/3.5). Most are PENDING OPERATOR for the same reason as Batch 4's browser scenarios — they require a real Stripe Checkout, Customer Portal interaction, or Test Clock.

Below is the status with respect to **changes since the prior audit** (Batches 4, 5a, 5b, 5c, and this branch). Scenarios that *might* be affected by recent changes are flagged "RE-RUN RECOMMENDED."

| ID | Topic | Prior status | Recent change touches | Recommendation |
|---|---|---|---|---|
| A | Subscribe monthly | PASS | Batch 4: re-subscribe partial unique; Batch 5a fix | RE-RUN |
| B | Cancel via Customer Portal | PASS | Batch 4 | RE-RUN |
| C | Resume before period ends | PASS | — | spot-check |
| D | Period ends → subscription deleted | PASS | Batch 4 partial unique on stripe_subscription_id | RE-RUN |
| E | Re-subscribe after full cancellation | PASS (architectural upsert) | **Batch 4: this changed semantics — now NEW row, not upsert** | **MUST RE-RUN** (was 1A in BATCH4_RESULT.md, still PENDING) |
| F | Failed payment → past_due | PASS (after schema fix 060) | — | spot-check |
| G | Refund a subscription charge | PASS | — | spot-check |
| H | Buy a course | PASS | Batch 5a fixed `current_uses` typo (only affects coupon path) | spot-check |
| I | Refund a course | INCONCLUSIVE | — | (still inconclusive; out of scope here) |
| J | Idempotency | PASS | Batch 5c: IDEMPOTENT-BY comments document the contract | sanity check (doc-only change) |
| K, L, M | Price changes | PASS | — | spot-check |
| N | Coupon redemption | PASS (API-level) | Batch 4 (sync-to-stripe, min_purchase); Batch 5a (current_uses fix) | **MUST RE-RUN** (overlaps BATCH4_RESULT 4A/4B) |
| O | 7-day trial with card upfront | PASS | — | spot-check |
| P | Trial converts to paid | PASS | — | spot-check |
| Q | 14-day trial without card | PASS | — | spot-check |
| R | Card-free trial cancels | PASS | — | spot-check |
| S | trial_will_end webhook | PASS | Batch 5c: IDEMPOTENT-BY notes Postmark MessageID dedup is currently absent | spot-check + note follow-up |
| T | Open redirect blocked | PASS | — | unaffected |
| U | Client-side price rejected | PASS | — | unaffected |
| V | Coupon validate requires auth | PASS | Batch 5b: 403 body changed from JSON to plain text | RE-RUN if any client parses 403 body |
| W | Webhook bad signature rejected | PASS | — | unaffected |
| X | Reactivate-bypass closed for cancelled | PASS | Batch 4: re-subscribe now creates new row (not the same as bypass) | RE-RUN to confirm distinction |
| Y | Money is integer cents | CONCERN | Batch 5c: CRM amount migrated to cents | Re-score (CRM was the largest residual concern) |
| Z | No webhook handler swallows DB errors | PASS | Batch 5a fixed `.ok()` swallow in /api/checkout (different file but same anti-pattern) | spot-check |
| AA | Migrations all registered | PASS | Batch 4 added migration 063; need to verify it's in `_sqlx_migrations` after deploy | RE-RUN (one-line check) |
| AB | Reconciliation job catches drift | PASS | This branch fixed the schedule comment + added prod-secret check (no behaviour change to reconcile) | sanity check |

## TASK7 RE-RUN summary

| Status |
|---|
| **Must re-run** (semantics changed since last score): E, N, AA — each has a clear behavioural change |
| **Re-run recommended** (likely unaffected but cheap to confirm): A, B, D, V, X, Y |
| **Spot-check / sanity check** (no functional change in path): C, F, G, H, J, K, L, M, O, P, Q, R, S, Z, AB |
| **Unaffected**: T, U, W |
| **Out of scope**: I |

Walk these in priority order. Update each row in TASK7_RESULT.md with PASS / FAIL / INCONCLUSIVE plus raw evidence after the run. Particularly:

- **E** is what BATCH4_RESULT.md scenario 1A walks; the same operator session covers both.
- **N** overlaps with BATCH4_RESULT.md scenarios 4A/4B (already PASS via CLI for the validator) plus 2A-2C for the recreate-and-swap path.
- **AA** is a single psql query: `SELECT version FROM _sqlx_migrations WHERE version = 63;` should return one row after the new image deploys.

## Combined scenario status (TASK7 + BATCH4)

```
TASK7:    28 scenarios — 27 PASS, 1 INCONCLUSIVE  (as of 2026-04-28)
BATCH4:   13 scenarios —  4 PASS (CLI), 9 PENDING OPERATOR (browser/Stripe Test Clock)
BATCH7:    0 net new scenarios — re-grading TASK7 above per recent changes
```

The launch-readiness assertion can be considered "code-complete" once:

1. The 3 MUST RE-RUN TASK7 scenarios (E, N, AA) PASS against a fresh stack with this branch + Batches 4, 5a, 5b, 5c merged.
2. The BATCH4 PENDING OPERATOR scenarios (1A, 1B, 2A-2C, 5A-5D) PASS.
3. The manual verification 1-3 from MERGE_BATCH7.md PR body succeeds on staging (especially #2 — production boot must panic on test-mode keys).
