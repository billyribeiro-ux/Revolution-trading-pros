# Task 6 Result — Stripe Reconciliation Job

Branch: `payments-fix-2026-04`
Date: 2026-04-28
Engineer: Billy Ribeiro

---

## Summary

All Task 6 objectives confirmed with raw evidence:

1. **`api/src/jobs/reconcile_stripe.rs`** — daily reconciliation job (03:00 UTC, tokio sleep loop)
2. **`api/src/routes/reconciliation.rs`** — admin endpoints: GET `/api/admin/reconciliation/log` + POST `/api/admin/reconciliation/run`
3. **Migration 058** — `reconciliation_log` table created and registered
4. **`list_subscriptions`** added to `StripeService` (paginated, all statuses)
5. **Desync test**: 2 rows intentionally corrupted → job found 3 discrepancies, fixed all 3
6. **`cargo check`** — 0 errors
7. **`pnpm check`** — 0 errors / 0 warnings

---

## Files Changed

| File | Change |
|------|--------|
| `api/src/jobs/mod.rs` | New module declaration |
| `api/src/jobs/reconcile_stripe.rs` | Reconciliation job: scheduler + `run_once()` + fix helpers |
| `api/src/routes/reconciliation.rs` | Admin: GET `/log`, POST `/run` |
| `api/src/routes/mod.rs` | `pub mod reconciliation` + `.nest("/admin/reconciliation", ...)` |
| `api/src/services/stripe.rs` | Added `list_subscriptions(status)` — paginated |
| `api/src/lib.rs` | Added `pub mod jobs` |
| `api/src/main.rs` | `reconcile_stripe::spawn_scheduler(state.clone())` after AppState build |
| `api/migrations/058_reconciliation_log.sql` | New migration |

---

## Architecture

### Scheduler (`spawn_scheduler`)

Uses a plain `tokio::spawn` + `tokio::time::sleep` loop — no external scheduler crate needed. Sleeps until 03:00 UTC each day. Avoids adding a dependency for what is essentially one timed loop.

### `run_once` Algorithm

1. Fetch all `active`, `trialing`, `past_due` subscriptions from Stripe (paginated 100/page).
2. Query all `user_memberships` with a `stripe_subscription_id` and `status NOT IN ('cancelled', 'expired')`.
3. For each DB row:
   - **Missing in Stripe** → set `status = 'cancelled'`
   - **Status drift** → update to match Stripe canonical status
   - **`cancel_at_period_end` drift** → update to match Stripe
   - **`current_period_end` drift >60 s** → update to match Stripe
4. Insert one `reconciliation_log` row with `discrepancies_found` count and per-entry JSONB array.

### Admin Endpoints

```
GET  /api/admin/reconciliation/log?limit=50&offset=0
POST /api/admin/reconciliation/run   (triggers run_once synchronously)
```

Both require `AdminUser` (any admin role).

---

## Migration 058

```sql
CREATE TABLE reconciliation_log (
    id                  BIGSERIAL PRIMARY KEY,
    run_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    discrepancies_found INTEGER NOT NULL DEFAULT 0,
    log                 JSONB NOT NULL DEFAULT '[]'
);
CREATE INDEX idx_reconciliation_log_run_at ON reconciliation_log (run_at DESC);
```

---

## Desync Test

### Setup — two intentional desyncs

| Row | sub_id | DB status | Stripe status | Kind |
|-----|--------|-----------|---------------|------|
| 3 | `sub_1TRI4t9HsGkDuN3buS4kkMVn` | `active` | `trialing` | status_drift + period_end_drift |
| 4 | `sub_FAKE_DELETED_12345` | `active` | _(not found)_ | missing_in_stripe |

### Trigger

```bash
POST /api/admin/reconciliation/run
Authorization: Bearer <admin-jwt>

→ { "success": true, "discrepancies_found": 3 }
```

### DB After Reconciliation

```sql
SELECT id, stripe_subscription_id, status FROM user_memberships WHERE id IN (3, 4);

 id |    stripe_subscription_id    |  status
----+------------------------------+-----------
  3 | sub_1TRI4t9HsGkDuN3buS4kkMVn | trial      ← was active
  4 | sub_FAKE_DELETED_12345       | cancelled  ← was active
```

### Reconciliation Log Entry

```sql
SELECT id, run_at, discrepancies_found, log FROM reconciliation_log;

 id |           run_at            | discrepancies_found | log
----+-----------------------------+---------------------+-----
  1 | 2026-04-28 20:18:27.260116  |                   3 | [
      {"kind":"status_drift",     "membership_id":3, "before":"active",        "after":"trial",                    "fixed":true},
      {"kind":"period_end_drift",  "membership_id":3, "before":"2026-05-28...", "after":"2026-05-01 20:17:55",     "fixed":true},
      {"kind":"missing_in_stripe", "membership_id":4, "action":"set status=cancelled",                            "fixed":true}
    ]
```

### GET /log Response (truncated)

```json
{
  "total": 1,
  "limit": 50,
  "offset": 0,
  "data": [{
    "id": 1,
    "run_at": "2026-04-28T20:18:27.260116",
    "discrepancies_found": 3,
    "log": [...]
  }]
}
```

---

## Final Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ 0 errors |
| `pnpm check` | ✅ 0 errors / 0 warnings |
| `reconciliation_log` table exists | ✅ version=58 in `_sqlx_migrations` |
| `list_subscriptions` fetches all Stripe subs | ✅ |
| Scheduler spawned at startup | ✅ logged "Stripe reconciliation scheduler started" |
| Manual run via POST `/run` | ✅ `discrepancies_found: 3` |
| status_drift fixed | ✅ row 3: active → trial |
| period_end_drift fixed | ✅ row 3: period_end corrected |
| missing_in_stripe fixed | ✅ row 4: active → cancelled |
| Log entry in `reconciliation_log` | ✅ 3 JSONB entries, all `fixed: true` |
