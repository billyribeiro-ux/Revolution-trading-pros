# Security Gaps — Revolution Trading Pros

**Date:** 2026-04-29
**Audit pass:** `main` @ commit `281aaa1dc`
**Fix pass:** branch `security-hardening-2026-04-29`
**Method:** Direct file read of every cited line. Every claim grounded in code I personally read this session, not interpolated from secondary reports.
**Verification:** `cargo check` clean. `cargo test --lib config` 13/13 passing (including 5 new tests for FIX-H-1). `pnpm check` 0 errors / 0 warnings on frontend.

---

## Status overview

| ID | Severity | Status | Component |
|---|---|---|---|
| C-1 | CRITICAL | ✅ FIXED | OAuth callback URL leaks tokens |
| C-2 | CRITICAL | ✅ FIXED | Frontend hooks decodes JWT without verify |
| C-3 | CRITICAL | ⏳ USER ACTION | R2 credentials rotation |
| H-1 | HIGH | ✅ FIXED | No JWT_SECRET production assertion |
| H-2 | HIGH | ✅ FIXED | Password reset doesn't invalidate sessions |
| H-3 | HIGH | ✅ FIXED | JWT blacklist fail-open |
| H-4 | HIGH | ✅ FIXED | Coupon `usage_count` double-incremented |
| H-5 | HIGH | ✅ FIXED | Impersonate stub footgun |
| M-1 | MEDIUM | ✅ FIXED | CSP `'unsafe-inline'` + missing on frontend |
| M-2 | MEDIUM | ✅ FIXED | `RateLimitService` dead code |
| M-3 | MEDIUM | ✅ FIXED | `oauth_tokens` schema-only |
| M-4 | MEDIUM | ✅ FIXED | Bcrypt fallback never migrates to argon2 |
| L-1 | LOW | ✅ FIXED | Email PII in tracing logs |
| L-2 | LOW | ✅ FIXED | Verification tokens not deleted at register |

13 of 14 fixed in code. C-3 requires you to log into the Cloudflare console — see the C-3 entry below.

---

## CRITICAL

### C-1 — OAuth callback returns full JWT + refresh token in URL query string ✅ FIXED

**Audit cross-ref:** AUTH_AUDIT H-3

**Was:** The Google and Apple callback handlers built a redirect URL of the form
`/auth/callback?provider=...&token=<JWT>&refresh_token=<JWT>&session_id=...&expires_in=...`.
Tokens leaked into Cloudflare access logs, browser history, and Referer headers.

**Now:** The callbacks return an `axum::response::Response` (no longer `Redirect`). On success they call a new helper `oauth_callback_response_with_cookies()` which:
- Sets `rtp_access_token`, `rtp_refresh_token`, and `rtp_session_id` as `HttpOnly; SameSite=Lax; Path=/` cookies on the redirect response. `Secure` is added in production (`state.config.is_production()`).
- Redirects to `/auth/callback?provider=<google|apple>` with NO credential params.

The frontend `/auth/callback/+page.svelte` was rewritten to no longer read `?token=...` from the URL. It calls `GET /api/auth/me` (cookie-authenticated via `credentials: 'include'`) to confirm the session and fetch the user record.

**Files changed:**
- `api/src/routes/oauth.rs`: imports updated (`HeaderValue`, `IntoResponse`, `Response`); `oauth_callback_response_with_cookies()` added (lines ~563+); `google_callback` and `apple_callback` signatures changed from `Result<Redirect, ...>` to `Result<Response, ...>`; success paths use the new helper; error paths use `Redirect::to(...).into_response()`.
- `frontend/src/routes/auth/callback/+page.svelte`: script section rewritten. No more URL token parsing, no `set-session` POST, no `replaceState`. Cookies are already set by the backend redirect.

---

### C-2 — Frontend `hooks.server.ts` decoded JWT payload without verifying signature ✅ FIXED

**Audit cross-ref:** AUTH_AUDIT M-1

**Was:** `frontend/src/hooks.server.ts:267-291` ran `JSON.parse(Buffer.from(parts[1], 'base64').toString())` and trusted the resulting `payload.sub`, `payload.email`, `payload.role`. Triggered on transient API failures. An attacker who could induce an API failure and plant a forged JWT in the cookie could land on admin pages.

**Now:** That entire decode-and-trust block is deleted. On any auth path failure (transient or permanent), `event.locals.user = null` and the user must re-authenticate. The `console.log` that echoed `payload.email` is gone (PII-out-of-logs as a side benefit).

**Files changed:**
- `frontend/src/hooks.server.ts`: lines 263-306 replaced with a one-line `event.locals.user = null` and a single warning log. Old block is documented in a `FIX-C-2` comment for git archeologists.

---

### C-3 — R2 storage credentials in plaintext `api/.env` ⏳ REQUIRES USER ACTION

**Audit cross-ref:** AUTH_AUDIT C-2

**Status:** This is the only finding I cannot fix from code. Requires logging into Cloudflare and revoking/rotating an API token.

**What you do:**
1. Open Cloudflare dashboard → R2 → Manage R2 API Tokens.
2. Find and **revoke** the token whose access key id is `c59b71db89704deb941119ac5324a12a` (this is the value currently in `api/.env`).
3. Generate a new token with the same scope (read + write on your R2 bucket).
4. Update `api/.env` locally with the new `R2_ACCESS_KEY_ID` and `R2_SECRET_ACCESS_KEY`.
5. When you migrate to your future hosting platform, put the new values in that platform's secret store. **Never** put them back in a `.env` file that lives on disk in the repo directory.
6. Smoke-test an upload (e.g. through the admin media page) to confirm the new key works.

`api/.env` is correctly in `.gitignore` (verified — the file is untracked). The breach surface is "the file existed on disk in this repo dir with real values," not "the file is in git history." Rotation closes that surface in ~10 minutes of ops work.

---

## HIGH

### H-1 — `JWT_SECRET` had no production startup assertion ✅ FIXED

**Was:** `api/src/config/mod.rs:208` read `JWT_SECRET` with no validation. The production-secret validator at lines 314-369 covered Stripe (sk_live_, pk_live_, whsec_) but not JWT. A prod deploy with `JWT_SECRET` accidentally set to the `.env.example` placeholder string `replace-me-with-a-long-random-secret-at-least-32-characters-long` (which is ≥ 32 chars, so a naive length check wouldn't catch it) would boot, sign JWTs with a known string, and let any attacker forge admin JWTs.

**Now:** `validate_production_secrets()` panics at boot if, when `ENVIRONMENT=production`:
- `JWT_SECRET.len() < 32`, OR
- `JWT_SECRET` (lowercased) contains any of: `replace-me`, `placeholder`, `changeme`, `your-secret-here`.

Both checks come before the existing Stripe checks so they fail first and loudest. The error messages tell the operator exactly what to do (`openssl rand -hex 32`).

**Files changed:**
- `api/src/config/mod.rs`: new check block at the top of `validate_production_secrets()`. `live_config()` test fixture updated to use a real-length test secret. 5 new `#[should_panic]` tests added: short secret, empty secret, literal `.env.example` placeholder, generic `placeholder` substring, `changeme` marker.

**Test evidence:** all 13 config tests pass (`cargo test --lib config`).

---

### H-2 — Password reset did NOT invalidate existing sessions ✅ FIXED

**Was:** `reset_password` (auth.rs:1285-1403) updated `password_hash` and deleted the reset row, then returned success. No call to `invalidate_all_user_sessions`, no JWT blacklist, no `security_events` row. A thief holding the user's stolen access/refresh token kept full access for the rest of the token's TTL even after the user reset their password.

**Now:** After the password update + token delete, the handler:
1. Looks up the user's id by email (it had only the email at this point).
2. Calls `redis.invalidate_all_user_sessions(user_id)` — every Redis session for this user is killed.
3. Calls `redis.invalidate_user_cache(user_id)` — forces the next request to re-fetch from DB.
4. Inserts a `security_events` row of type `password_reset`, category `authentication`, severity `high`.
5. Logs a `password_reset_completed` audit event.

All Redis steps are best-effort: a Redis fault is logged but does not roll back the password change (rolling back would be a worse outcome — the password is already changed).

**Files changed:**
- `api/src/routes/auth.rs`: `reset_password` handler tail rewritten.

---

### H-3 — JWT blacklist check failed OPEN in auth middleware ✅ FIXED

**Was:** `middleware/auth.rs:71-80`. When the Redis blacklist lookup returned `Err`, the request proceeded with a warning log. A Redis fault during a logout/ban window let the just-revoked token continue to work.

**Now:** Fail-closed. If `redis.is_token_blacklisted` returns `Err`, the middleware returns `401 "Authentication temporarily unavailable"`. The frontend's existing 401-then-refresh-then-re-auth flow handles the user-facing UX.

The only branch that still proceeds without a blacklist check is when `state.services.redis` is `None` — i.e. Redis is not configured at all. That's a development-time scenario; production-time is caught by `validate_production_secrets` (would need a future check that Redis is configured in prod, called out in the inline comment).

**Files changed:**
- `api/src/middleware/auth.rs`: error branch now returns 401 instead of falling through.

---

### H-4 — Coupon `usage_count` was double-incremented per real order ✅ FIXED

**Audit cross-ref:** SUBSCRIPTION_AUDIT H2

**Was:** `usage_count` was bumped twice per paid order: once at order-create time inside `routes/checkout.rs::create_checkout` (lines 311-330), and again in the webhook handler `routes/payments.rs::handle_checkout_completed` (lines 604-611). Coupon caps were hit at half the intended rate. Worse: the order-create path fired even on abandoned carts that never paid, so the count was off by an unbounded amount.

**Now:** Single source of truth is the webhook. Reasoning: the webhook is idempotent at the `webhook_events(event_id) UNIQUE` level, and only fires for actually-paid orders.

The order-create increment block has been removed (replaced by a `FIX-H-4` comment). The webhook increment is unchanged. A new migration **`065_backfill_coupon_usage.sql`** rebuilds `coupons.usage_count` from `COUNT(orders.id) WHERE orders.status = 'completed'`, correcting historical drift.

**Files changed:**
- `api/src/routes/checkout.rs`: the `if let Some(coupon_id) = applied_coupon_id` block at lines 311-330 replaced with a FIX-H-4 comment + `let _ = applied_coupon_id;` (still used downstream as `orders.coupon_id`).
- `api/migrations/065_backfill_coupon_usage.sql`: new file. Idempotent UPDATE that recomputes counts from the orders table.

---

### H-5 — Admin `impersonate_user` endpoint was a footgun stub ✅ FIXED

**Was:** `api/src/routes/admin.rs:2140-2199` defined an `impersonate_user` handler guarded by `SuperAdminUser`. It returned a fake string `impersonate_{id}_{timestamp}` (not a valid JWT, so harmless on its own). The inline comment said "In a real implementation, you would generate a JWT token for the target user" — instructing the next person (or AI) to literally call `create_jwt(target_user.id, ...)`, which would mint a back-door JWT for any user with no audit trail, no time bound, and no allowlist.

**Now:** The handler body is replaced with a long FIX-H-5 comment block explaining (a) the previous behavior, (b) why it was removed, and (c) what a correct future implementation would require. The route registration `.route("/users/:id/impersonate", post(impersonate_user))` is removed from the admin router.

The frontend client wrappers `usersApi.impersonate` (admin.ts:1040-1044) and the enterprise adapter `usersApi.impersonate` (admin-adapter.ts:173-174) are removed. No UI page actually called either, so no end-user-visible feature regression.

**Files changed:**
- `api/src/routes/admin.rs`: handler body and route registration removed.
- `frontend/src/lib/api/admin.ts`: client wrapper removed.
- `frontend/src/lib/api/enterprise/admin-adapter.ts`: enterprise wrapper removed.

---

## MEDIUM

### M-1 — CSP had `'unsafe-inline'` and missing frontend coverage ✅ FIXED

**Was:**
- `api/src/main.rs:206-207`: API CSP allowed `'unsafe-inline'` in `script-src` and `style-src`. Localhost entries shipped in production builds.
- `frontend/svelte.config.js:50-95`: frontend CSP was decent but missed Stripe domains and had a duplicated `http://localhost:8080` entry.

**Now:**

API side (main.rs):
- `script-src 'self'` only (no `'unsafe-inline'`).
- `style-src 'self'` only.
- `default-src 'none'` (every directive explicit).
- `frame-ancestors 'none'`, `base-uri 'self'`, `form-action 'self'` added.
- Localhost entries removed from `connect-src`.

Frontend side (svelte.config.js):
- Stripe added: `https://js.stripe.com` to `script-src`, `https://api.stripe.com` to `connect-src`, `https://js.stripe.com` and `https://hooks.stripe.com` to `frame-src` (required for Stripe Elements / 3DS).
- Duplicated `http://localhost:8080` removed from the always-present `connect-src` list (still in the `NODE_ENV === 'development'` branch).
- `style-src 'unsafe-inline'` retained — Svelte component-scoped styles compile to `style=""` attributes that need it; SvelteKit's `mode: 'auto'` adds nonces to `<style>` blocks but cannot rewrite inline attributes. Removing it would break the UI; explicitly noted in the file comment.

**Files changed:**
- `api/src/main.rs`: CSP `HeaderValue::from_static` rewritten with multi-line strict policy.
- `frontend/svelte.config.js`: `kit.csp.directives` updated.

---

### M-2 — `RateLimitService` was dead code ✅ FIXED

**Was:** `api/src/services/rate_limit.rs` defined a 452-line multi-tier service that was never instantiated anywhere. Login rate-limiting goes through `state.services.redis.check_login_rate_limit` directly. The dead service still wrote `login_rate_limits` table entries in code that was never called.

**Now:**
- `api/src/services/rate_limit.rs` deleted.
- `api/src/services/mod.rs`: `pub mod rate_limit` declaration removed (replaced by a FIX-M-2 comment explaining why).
- `api/migrations/067_drop_unused_login_rate_limits.sql` new — drops the empty `login_rate_limits` table.

If multi-tier rate limiting becomes a real requirement later, design it from scratch — git history has the deleted code as a reference.

---

### M-3 — Unused `oauth_tokens` table with misleading "encrypted" column names ✅ FIXED

**Was:** `oauth_tokens` table created in `033_oauth_providers.sql:41-52` with columns `access_token_encrypted` and `refresh_token_encrypted`. Never written to by any handler in `routes/oauth.rs`. The column names implied an encryption helper that doesn't exist anywhere in `api/src/`.

**Now:** `api/migrations/066_drop_unused_oauth_tokens.sql` new — drops the table. If persistent provider-token storage is ever needed, the new feature must build the encryption helper from scratch and ship the schema in a fresh migration alongside the writer code.

---

### M-4 — Bcrypt fallback never migrated users to argon2id ✅ FIXED

**Was:** `verify_password` (utils/mod.rs:180-213) accepted both bcrypt (Laravel legacy) and argon2id. There was no migration path: a user with a bcrypt hash kept it forever.

**Now:** `routes/auth.rs::login` re-hashes silently after a successful password verify, when the stored hash starts with `$2` (i.e. bcrypt). The new argon2id hash overwrites the column via `UPDATE users SET password_hash = ...`. Best-effort: a Redis or DB fault during the rehash logs an error and keeps the existing bcrypt hash for next time.

A `bcrypt_rehashed_to_argon2id` audit event is logged on each successful migration, so you can grep your logs for migration progress (or just `SELECT COUNT(*) FROM users WHERE password_hash LIKE '$2%'` to check remaining count).

**Files changed:**
- `api/src/routes/auth.rs`: rehash block added between "clear failed login attempts" and the `is_active` ban check.

---

## LOW

### L-1 — Tracing emitted user emails into security_audit logs ✅ FIXED

**Was:** 28 tracing call sites across `routes/auth.rs`, `middleware/admin.rs`, `routes/oauth.rs`, and `routes/admin.rs` included `email = %user.email` (or `%input.email`, `%admin_email`, etc.). Emails landed in whatever log sink is configured.

**Now:** All `email = %x.email,` lines removed via batch perl pass. `user_id = ...` is retained on every event, so correlation with the user is one DB lookup away (same as before, just without the PII shortcut). The `security_events` table still records the user_id; emails are admin-accessible there under the `AdminUser` extractor's auth.

**Files changed:**
- `api/src/routes/auth.rs`, `api/src/middleware/admin.rs`, `api/src/routes/oauth.rs`: bulk strip.
- `api/src/routes/admin.rs`: the impersonate handler (which had two email lines) was removed entirely as part of H-5.

---

### L-2 — Verification tokens not defensively cleared at register ✅ FIXED

**Was:** `register` handler INSERTed a new verification token without first deleting any prior tokens for the same `user_id`. The email-already-exists check at line 196 made this unreachable today, but a future code path that creates a user via a different route then issues a verification email could accumulate stale rows.

**Now:** A `DELETE FROM email_verification_tokens WHERE user_id = $1` runs inside the same transaction before the new INSERT. Matches the pattern already used in `resend_verification`. Idempotent — DELETE of zero rows is a no-op.

**Files changed:**
- `api/src/routes/auth.rs`: register handler.

---

## Verification

### Build
```
$ cargo check
   Checking revolution-api v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.43s
```

### Tests (backend)
```
$ cargo test --test utils_test --test stripe_test --lib config
running 13 tests
test config::tests::validate_production_secrets_is_noop_outside_production ... ok
test config::tests::validate_production_secrets_passes_with_all_live_keys ... ok
test config::tests::validate_production_secrets_allows_empty_publishable ... ok
test config::tests::validate_production_secrets_panics_on_placeholder_webhook_secret ... ok
test config::tests::validate_production_secrets_panics_on_test_secret ... ok
test config::tests::validate_production_secrets_panics_on_test_publishable ... ok
test config::tests::validate_production_secrets_panics_on_empty_webhook_secret ... ok
test config::tests::validate_production_secrets_panics_on_short_jwt_secret ... ok       [NEW — H-1]
test config::tests::validate_production_secrets_panics_on_changeme ... ok                [NEW — H-1]
test config::tests::validate_production_secrets_panics_on_replace_me_placeholder ... ok  [NEW — H-1]
test config::tests::validate_production_secrets_panics_on_empty_jwt_secret ... ok        [NEW — H-1]
test config::tests::validate_production_secrets_panics_on_bad_webhook_prefix ... ok
test config::tests::validate_production_secrets_panics_on_placeholder_substring ... ok   [NEW — H-1]

test result: ok. 13 passed; 0 failed
```

### Typecheck (frontend)
```
$ pnpm check
COMPLETED 5217 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

### What was NOT verified by automated tests
- End-to-end OAuth flow with cookies (requires a real Google/Apple client). Recommended manual smoke test before merging: log in via Google, confirm cookies are set, confirm `/auth/callback?provider=google` URL has no token params, confirm `GET /api/auth/me` returns the user.
- Password reset session-invalidation A/B (log in on two devices, reset on one, confirm the other 401s on next request). Requires Redis running.
- Bcrypt rehash migration (only triggers for users with `$2*` hashes; verify by checking `SELECT COUNT(*) FROM users WHERE password_hash LIKE '$2%'` before and after a couple of test logins).
- Webhook idempotency under retry for the new coupon-counter behavior.

These are scenario tests that need a running stack, not unit tests.

---

## What I did NOT touch (out of scope this pass)

- **Stripe webhook signature verification.** Confirmed the call site exists at `payments.rs:137`, did not read `services/stripe.rs::verify_webhook` body. AUTH_AUDIT marks signature verification as implemented; trust but verify before launch.
- **CMS authorization endpoints.** Outside auth scope.
- **Bunny / Postmark credentials handling.** Outside auth scope.
- **Reconciliation job correctness.** Outside auth scope.
- **Frontend admin guard at every admin sub-route.** I read `+layout.svelte` and confirmed it has a role check inside `onMount`. I did NOT verify every individual admin page has its own server-side guard or relies solely on the layout.
- **Dispute / refund webhook handlers.** Read the dispatch code at `payments.rs:269-271`; did not read `handle_dispute_created` or `handle_refund` bodies in detail.

A second pass covering these is recommended before launch.

---

*End of report.*
