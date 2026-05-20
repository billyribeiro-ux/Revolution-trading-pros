# Batch 6 — postmark-integration

**Branch:** `postmark-integration` (off `main` at `5bf3841b7`)
**Status:** code complete + gates green; verification A-H captured in [BATCH6_RESULT.md](BATCH6_RESULT.md). Awaiting PR review.

## PR title (suggested)
```
feat(email): Postmark integration with graceful no-op fallback (Batch 6)
```

## PR body (suggested)

### Summary

Wires the 11 transactional email triggers the spec calls for (welcome, verification, password-reset, subscription confirmation/cancellation/renewal/trial-ending, failed-payment, receipt, refund-confirmation, dispute-created) through a redesigned `EmailService` that is **always present** and writes a row to `email_logs` for every send attempt — including when `POSTMARK_TOKEN` is unset.

The Postmark account is **not yet active**, so this lands in a graceful no-op state: the service writes `email_logs` rows with `status='skipped_no_token'` and skips the HTTP call. The moment the operator sets `POSTMARK_TOKEN` + `POSTMARK_FROM_EMAIL` + `ADMIN_NOTIFICATION_EMAIL` and restarts, the same call paths begin actually delivering. **Zero code change required to flip the switch.**

### Highlights

- **Migration 064** adds `to_email`, `template_alias`, `model JSONB`, `error`, `queued_at`, `sent_at` columns to `email_logs` (alongside the legacy newsletter columns; nothing renamed). Two indexes for the admin diagnostic queries. Existing rows backfilled.
- **`EmailService::send_transactional(pool, to, alias, model)`** is the new primary API. Always writes a queued row first, then either calls Postmark `email/withTemplate` or no-ops to `skipped_no_token`. Errors during HTTP propagate as `Err` (so callers can decide); errors during DB row insert abort the send (no half-state).
- **`Services.email` is no longer `Option<EmailService>`.** The 11 legacy callers that pattern-matched on `Some` were mechanically updated to direct refs. The token presence is now an internal detail of the service.
- **Daily 09:00 UTC scheduler** (`api/src/jobs/email_reminders.rs`, mirrors `reconcile_stripe::spawn_scheduler`) queues subscription renewal reminders (6-8 days out) and trial-ending fallback reminders (2-4 days out). Both queries dedup against the previous 24h of `email_logs` so a Stripe-driven webhook send doesn't double up with the scheduled send.
- **Admin diagnostics**: `GET /api/admin/email/status` shows `postmark_token_set` + 24h sent/failed/skipped counters. `GET /api/admin/email/logs?limit=N&status=X&template_alias=Y` returns newest rows with the `model` JSONB so admins can verify the right merge data was queued.
- **Bonus pre-existing-bug fix**: `forgot_password` was 500-ing on every request because the INSERT bound `gen_random_uuid()` to a `BIGSERIAL` `id` column. Same shape as the `current_uses` typo Batch 5a fixed. Now uses the sequence default. Surfaced during verification scenario E.

### Files changed

- [api/migrations/064_email_logs_batch6_columns.sql](api/migrations/064_email_logs_batch6_columns.sql) — new
- [api/src/services/email.rs](api/src/services/email.rs) — added `from_env`, `is_enabled`, `from_email`, `send_transactional`; refactored `send` to no-op when token unset; struct field `token: String` → `token: Option<String>`.
- [api/src/services/mod.rs](api/src/services/mod.rs) — `email: Option<EmailService>` → `email: EmailService`; constructor uses `EmailService::from_env(&config.app_url)`.
- [api/src/config/mod.rs](api/src/config/mod.rs) — added `admin_notification_email: Option<String>`; `postmark_token` now empty-string-means-None; reads `POSTMARK_FROM_EMAIL` (falls back to legacy `FROM_EMAIL`).
- [api/src/services/credential_resolver.rs](api/src/services/credential_resolver.rs) — test helper `dummy_config` updated for new field.
- [api/src/routes/auth.rs](api/src/routes/auth.rs) — register sends welcome + email-verification; resend-verify uses send_transactional; forgot-password uses send_transactional + bug fix on `password_resets` INSERT; legacy `Option<email>` callers cleaned up.
- [api/src/routes/payments.rs](api/src/routes/payments.rs) — `handle_checkout_completed` (sub vs one-time), `handle_subscription_deleted`, `handle_payment_failed`, `handle_refund`, `handle_dispute_created`, `handle_trial_will_end` all wired through `send_transactional`.
- [api/src/routes/admin.rs](api/src/routes/admin.rs) — new `GET /email/status` + `GET /email/logs` handlers, plus router entries.
- [api/src/routes/user.rs](api/src/routes/user.rs), [api/src/routes/forms.rs](api/src/routes/forms.rs), [api/src/routes/subscriptions.rs](api/src/routes/subscriptions.rs) — legacy `Option<email>` callers cleaned up.
- [api/src/jobs/email_reminders.rs](api/src/jobs/email_reminders.rs) — new daily scheduler.
- [api/src/jobs/mod.rs](api/src/jobs/mod.rs) — register the new module.
- [api/src/main.rs](api/src/main.rs) — spawn the scheduler.
- [api/.env](api/.env), [api/.env.example](api/.env.example) — placeholder `POSTMARK_TOKEN`, `POSTMARK_FROM_EMAIL`, `ADMIN_NOTIFICATION_EMAIL`; activation instructions inline.
- [BATCH6_RESULT.md](BATCH6_RESULT.md) — new, with verification evidence and operator instructions.

### Diff stat (anticipated, will sharpen at commit)

~10 src files touched, ~600 insertions / ~300 deletions, 1 new migration, 1 new module, 1 new doc.

### Gates

| Gate | Result |
|---|---|
| `cargo check` (api/, --locked) | ✅ 0/0 |
| `cargo clippy --no-deps` | ✅ 0/0 |
| `cargo test config::tests` | ✅ 8/8 (Batch 7 tests still passing) |
| `pnpm check` (frontend/) | ✅ 0 errors / 0 warnings (5217 files) |

### Verification (local stack, Postmark OFF)

8 scenarios A-H walked against `localhost:8080`. 7 fully PASS via CLI; F is PASS-by-construction (CLI portion confirmed; full webhook-driven email send requires browser checkout completion). Full transcripts in [BATCH6_RESULT.md](BATCH6_RESULT.md).

### Operator instructions to enable Postmark

1. Provision the Postmark server.
2. Create the 11 templates with these aliases: `welcome`, `email-verification`, `password-reset`, `subscription-confirmation`, `subscription-canceled`, `subscription-renewal-reminder`, `trial-ending`, `failed-payment`, `receipt`, `refund-confirmation`, `dispute-created`. The `TemplateModel` keys for each are documented in BATCH6_RESULT.md and visible live in `GET /api/admin/email/logs` after running flows in skip mode.
3. Set in `api/.env.local`:
   ```
   POSTMARK_TOKEN=<server token>
   POSTMARK_FROM_EMAIL=<verified Sender Signature>
   ADMIN_NOTIFICATION_EMAIL=<address for dispute-created>
   ```
4. `docker compose up -d api`. Confirm `GET /api/admin/email/status` returns `"postmark_token_set": true`.
5. Run a register against a real address. Expect `status='sent'` and a populated `provider_message_id` in the corresponding `email_logs` row.

No code change required.

### Test plan

- [ ] CI green.
- [ ] Smoke: a fresh register against a real address, after activation, returns 200 and `email_logs.status='sent'` with `provider_message_id` set.
- [ ] Smoke: `dispute-created` template rendered correctly when a chargeback fires (or when admin manually triggers via Stripe Dashboard).
- [ ] Daily scheduler: confirm 09:00 UTC log line `Email reminders job sleeping until 09:00 UTC` appears in `docker logs rtp-api`.
- [ ] BATCH6_RESULT.md scenario F (browser-driven): walk a Stripe-hosted checkout to confirm `subscription-confirmation` / `receipt` rows appear post-webhook.
