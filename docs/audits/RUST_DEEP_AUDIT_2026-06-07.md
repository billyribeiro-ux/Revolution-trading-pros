# Rust Backend Deep Audit — 2026-06-07

**Scope:** `api/src/` (~110k LOC, Axum 0.8 / SQLx 0.9 / Tokio). `retiredmay26/` excluded.
**Method:** Toolchain pass (`cargo fmt`, `cargo check`, `cargo clippy` — all clean / 0 warnings,
incl. a pedantic+nursery sweep), then five domain deep-dives (money path, auth & secrets, SQL
safety, async/concurrency, error-handling & panics). Every P0/P1 below was re-read and verified
by hand at the cited `file:line`.
**Release profile note:** `Cargo.toml` sets `panic = "abort"`, so a reachable panic in a request
handler aborts the worker process — reachable handler panics are treated as high severity.

---

## Executive summary

The codebase is, on the whole, **well-engineered and security-conscious** — the auth core
(Argon2id, pinned-alg JWT with token-epoch revocation, JWKS-verified Apple/Google OAuth, CSPRNG
tokens, constant-time compares), the Stripe **webhook** core (constant-time signature check with
timestamp tolerance, event-id idempotency, single-transaction atomicity, integer-cents money),
and concurrency hygiene (zero `std::sync` guards held across `.await`; bounded channels) are all
genuinely solid. See **Positives** at the end — they are extensive and should be preserved.

The findings cluster in three places: **(a)** the member-indicator download/license paths
(home-grown token signing that bypasses its own checks), **(b)** the *outbound* subscription
mutation paths (DB updated without telling Stripe), and **(c)** a systemic habit of swallowing
database errors with `.unwrap_or_default()`, in violation of this repo's own CLAUDE.md rule.

| Severity | Count | Headline items |
|----------|-------|----------------|
| **P0** | 3 | Download IDOR (live); forgeable download tokens (empty secret); `ORDER BY` injection (latent/dead-code) |
| **P1** | 7 | `cancel_subscription` never calls Stripe; subscribe race; forgeable license keys; plaintext stored credentials; 2× WebSocket leaks; ~107 swallowed DB errors |
| **P2** | 9 | Redis `KEYS` stall + broken L1 glob; TOTP replay; pagination overflow; UTF-8 panic; proration drift; refund `amount` default; uncapped service LIMIT; N+1; WS dup off-by-one |
| **P3** | 7 | dev-mode webhook bypass; OAuth-state TOCTOU; fail-open IP limit; no graceful shutdown; `INFINITY`→`null`; misc swallows |

---

## P0 — Fix first

### P0-1 · IDOR: download token not bound to the requested file *(exploitable today)*
`api/src/routes/member_indicators/downloads.rs:166–199`

`download_file` loads the token row — which **carries the authorized `file_id`** (`download.1`) —
but then serves the file by the **URL path** `file_id`, never comparing the two and never
re-verifying the token hash:
```rust
// SELECT id, file_id, download_count, max_downloads ... WHERE download_token = $1
let download: Option<(i32, i32, Option<i32>, Option<i32>)> = ...;   // download.1 = authorized file
...
let file = sqlx::query_as("SELECT * FROM indicator_files WHERE id = $1")
    .bind(file_id)   // <-- Path((slug, file_id)), NOT download.1
```
Any member holding **one** valid, unexpired token (for a file they legitimately own) can change
`:file_id` in the URL and download **any file of any indicator**, bypassing the ownership check in
`generate_download_url`.
**Fix:** reject when `file_id != download.1`; recompute and constant-time-verify the HMAC over
`(user_id, file_id, expiry, secret)` before streaming.

### P0-2 · Forgeable download tokens — empty signing secret over public inputs
`api/src/routes/member_indicators/downloads.rs:96–104`
```rust
let secret = std::env::var("MEMBER_INDICATOR_SECRET").unwrap_or_default(); // "" if unset
let token  = sha256(format!("{user_id}{file_id}{expiry_timestamp}{secret}"));
```
`MEMBER_INDICATOR_SECRET` is referenced **only** in this file — `Config` never reads or
boot-validates it (unlike `JWT_SECRET`). If unset, the token is `SHA256` of entirely
attacker-known values → anyone can mint valid tokens offline. Even when set, it is a bare
`SHA256(concat)`, not a keyed HMAC.
**Fix:** read the secret in `Config::from_env()` with no default and panic in
`validate_production_secrets()` if empty or `< 32` chars (mirror `JWT_SECRET`, `config/mod.rs:496`);
switch to HMAC-SHA256.

> *Note:* the empty-secret default at `downloads.rs:96` / `license.rs:26` was changed from
> `unwrap_or_else(|_| "".to_string())` to `unwrap_or_default()` in the prior idiomatic-cleanup
> commit — that was a faithful refactor of **identical** behavior (both yield `""`). The security
> defect (defaulting at all) predates and survives it; it is fixed by the change above.

### P0-3 · SQL injection via unvalidated `ORDER BY {sort_by}` *(latent — currently dead code)*
`api/src/domain/content/repository.rs:229`
```rust
ORDER BY {sort_by} {order_direction}   // sort_by = query.sort_by (HTTP), interpolated raw
```
`order_direction` is normalized to ASC/DESC (safe); `sort_by` is interpolated verbatim from a
`Deserialize` query field with no allowlist → arbitrary `ORDER BY` subquery injection.
**Mitigant:** `PgContentRepository` is **not wired to any route** (grep: it appears only in its own
file); the live CMS list path uses `cms_content::list_content`. This is a P0-grade defect that
becomes live the instant the repository is connected.
**Fix:** allowlist `sort_by` (`match` → fixed identifiers), exactly as the ~12 sibling list
handlers already do.

---

## P1 — High

### P1-1 · `cancel_subscription` updates the DB but never calls Stripe → billing continues
`api/src/routes/subscriptions/lifecycle.rs:230–248`
Both branches only `UPDATE user_memberships`; there is no `stripe.cancel_subscription(...)`. Stripe
remains the billing source of truth, so the card is charged next cycle. Worse, the nightly
reconcile (`jobs/reconcile_stripe.rs:161`) sees the sub still active in Stripe and **flips the row
back to `active`** — the user's cancellation silently undoes itself. (Contrast `reactivate_subscription`
at `:307` and `change_plan`, which correctly route through Stripe.)
**Fix:** call Stripe first; write DB status only on confirmation (or let the
`customer.subscription.{updated,deleted}` webhook do the DB write). Fail closed if
`stripe_subscription_id` is absent.

### P1-2 · `create_subscription` check-then-act race (double subscribe)
`api/src/routes/subscriptions/lifecycle.rs:51–71` (and free-path INSERT `:146–163`)
A non-transactional `SELECT ... WHERE status IN ('active','trialing')` then act, with **no unique
index** on live memberships per user. Two concurrent requests / a double-click both read `None` and
both create subscriptions.
**Fix:** add a partial unique index `user_memberships(user_id) WHERE status IN ('active','trialing')`
and rely on `ON CONFLICT` instead of check-then-act.

### P1-3 · Forgeable / weak license keys — empty signing secret
`api/src/routes/member_indicators/license.rs:25–41`
Same unvalidated `MEMBER_LICENSE_SECRET.unwrap_or_default()` problem; key = `SHA256(user-indicator-ts)[..16]`
(64 bits) from a non-secret formula. Impact is bounded (validation also checks a stored
ownership row), but the shareable license artifact should be a CSPRNG value or keyed HMAC.
**Fix:** generate via `rand` (as the other token helpers in `utils/mod.rs` do) and boot-validate the secret.

### P1-4 · Third-party credentials stored as base64 (≈ plaintext at rest)
`api/src/routes/connections/crypto.rs:8–24`, `api/src/services/credential_resolver.rs:256–261`
`encrypt_credentials` base64-encodes JSON (the comment admits "placeholder … use AES-256-GCM").
`service_connections.credentials_encrypted` holds **live** Stripe secret keys; any DB read / backup /
replica leak yields them directly.
**Fix:** AES-256-GCM (or `chacha20poly1305`) with a boot-validated key/KMS; base64 only the
ciphertext+nonce envelope.

### P1-5 · WebSocket room-forwarder tasks orphaned on disconnect (task + count leak)
`api/src/routes/websocket.rs:443–451` (cleanup `:578–590`)
Each subscribed room spawns a detached forwarder whose `JoinHandle` is dropped. On disconnect only
`send`/`receive`/`heartbeat` handles are stopped; the forwarders are never aborted and only exit on
the *next* room broadcast finding the mpsc closed. On quiet rooms the task + its `broadcast::Receiver`
linger indefinitely, and the connection count drifts.
**Fix:** track forwarder handles in a per-connection `JoinSet` and `.abort()` them in cleanup.

### P1-6 · `room_senders` map grows unbounded (client-controlled keys)
`api/src/routes/websocket.rs:257–275`
`unsubscribe_room` deliberately keeps the `broadcast::Sender` ("for potential reconnections") when
the count hits zero, so the map is append-only over process lifetime. Room slugs are
client-supplied, so subscribing to N distinct slugs retains N 1000-slot channels → unbounded memory.
**Fix:** drop the sender at zero (re-create on next subscribe via `entry().or_insert_with`), and/or
validate slugs against a known-room allowlist.

### P1-7 · Systemic: ~107 SQLx `Result`s swallowed with `.unwrap_or_default()` across ~47 files
Directly violates CLAUDE.md ("Don't swallow errors with `unwrap_or_default()` on `Result<T,E>` —
propagate via `?`"). A DB error (pool exhaustion, deadlock, dropped conn) becomes an empty
`Vec`/default returned as **`200 OK`**, masking outages and serving incomplete data as authoritative.
Member-facing / data-integrity examples:
- `routes/member_courses/player.rs:162,169,176` — lessons / downloads / progress: a DB error renders a **paid course empty** to the member.
- `routes/videos.rs:442,469,483,528`; `routes/coupons.rs:292` (user coupons → "none"); `routes/members.rs:306,790`; `admin_member_management/crud.rs` (5); `indicators_admin.rs` (8); `room_resources/admin_crud.rs` (8); `member_courses/quizzes.rs` (4); and more.
**Fix:** mechanical sweep to `.map_err(|e| { error!(...); ApiError::... })?`, matching the webhook
handlers' propagation style (which is exemplary). Prioritize member/payment-facing handlers first.

---

## P2 — Medium

- **P2-1 · Redis invalidation stalls + silently no-ops L1.** `services/redis.rs:589` uses blocking
  `KEYS` (O(N), stalls the single-threaded server for *all* clients) despite a comment claiming
  `SCAN`; and the L1 `PatternMatcher` (`cache/service.rs:633`) only honors the **first** `*`, while
  every pattern in `cache/keys.rs` has 2+ wildcards → L1 invalidation matches **nothing**, serving
  stale alerts/trades until TTL. **Fix:** cursor-based `SCAN`; real glob matching.
- **P2-2 · TOTP code replay within the ±1-period (90s) window.** `services/mfa.rs:257–285` never
  records the consumed counter (backup codes *are* single-use). **Fix:** persist last-accepted
  `time/period` per user, reject `<=`.
- **P2-3 · Pagination offset overflow.** `(page-1)*per_page` with unbounded `page: i64` on ~25 list
  endpoints (`posts.rs:201`, `members.rs:90`, `videos.rs:174`, `indicators_admin.rs:35`, …). Release
  (`panic=abort`) wraps to a garbage offset. **Fix:** clamp `page` or `saturating_mul`; add a shared
  pagination helper.
- **P2-4 · `mask_sensitive_value` UTF-8 panic.** `routes/settings.rs:146` slices `&value[len-4..]`
  behind a **byte**-length guard; a multibyte char near the end panics the handler. **Fix:** slice on
  a char boundary.
- **P2-5 · Proration whole-day truncation.** `routes/subscriptions/change_plan.rs:96–104` uses
  `num_days()` + integer division for the *preview* credit; diverges from Stripe's actual proration
  shown to the user. **Fix:** compute in seconds, or surface Stripe's upcoming-invoice preview.
- **P2-6 · Refund webhook defaults `charge.amount` to 0.** `routes/payments/webhook_refund.rs:30–32`
  — a malformed/absent `amount` makes any nonzero refund look "full" (`refund >= 0`) and revokes all
  access. **Fix:** treat missing `amount` as 5xx (let Stripe retry); key off `charge.refunded == true`.
- **P2-7 · Uncapped service-layer LIMIT.** `services/room_search.rs:225` binds `LIMIT $3` with no cap;
  safe only because all four current callers `.clamp(1,100)`. **Fix:** clamp inside the service as a backstop.
- **P2-8 · N+1 in quiz fetch.** `routes/member_courses/quizzes.rs:175` runs one answers query per
  question. **Fix:** `WHERE question_id = ANY($1)` then group in Rust.
- **P2-9 · WebSocket initial-rooms not deduped.** `routes/websocket.rs:433–457` — `?rooms=a,a`
  double-increments the count (cleanup iterates a `HashSet` once) → permanent off-by-one, inflating
  `ws_stats`. **Fix:** dedupe `initial_rooms` before subscribing.

---

## P3 — Low / defensive

- **P3-1 · dev-mode webhook signature bypass.** `routes/payments/webhook.rs:69–86` skips verification
  when the secret is unset **and** `environment` starts with `"dev"`. Gated on parsed config and
  rejects in prod (good), but a prod deploy misconfigured as `ENVIRONMENT=development` would accept
  unsigned webhooks. Consider an explicit `ALLOW_UNSIGNED_WEBHOOKS` flag, fail-closed otherwise.
- **P3-2 · OAuth-state consumption is not atomic (TOCTOU).** `routes/oauth/google.rs:155–189`,
  `apple.rs:152–186` — `SELECT ... used_at IS NULL` then separate `UPDATE`; the `UPDATE` error is
  `.ok()`-swallowed. PKCE/nonce bound the impact. **Fix:** single `UPDATE ... WHERE used_at IS NULL
  RETURNING`, zero rows = invalid.
- **P3-3 · Per-IP login rate limit is fail-open.** `routes/auth/login.rs:51` (documented; per-email is
  fail-closed). Confirm intended.
- **P3-4 · No graceful shutdown.** `queue/worker.rs` `loop {}`, detached schedulers, and `axum::serve`
  without `.with_graceful_shutdown()` → SIGTERM kills mid-job (rows stuck `processing`; self-heals via
  `FOR UPDATE SKIP LOCKED` + attempt cap). **Fix:** thread a `CancellationToken`/`watch` and wire
  graceful shutdown.
- **P3-5 · `profit_factor = f64::INFINITY`** (`services/room_analytics.rs:257`) serializes to JSON
  `null`, silently dropping the value. **Fix:** finite sentinel / explicit flag.
- **P3-6 · `reconciliation.rs:57` `unwrap_or((0,))`** on a COUNT — admin read, reports 0 on DB error.
- **P3-7 · Unlogged fire-and-forget writes** (`let _ = sqlx::query(...)`) for counters/analytics
  (`videos.rs:369`, `room_resources/public.rs:243`, `analytics.rs:240`, …) — intentional best-effort,
  but `if let Err(e) = … { warn!(…) }` would make failures observable.

---

## Positives (verified — preserve these)

- **Stripe webhook core:** constant-time signature verify with 5-min timestamp tolerance over exact
  bytes (`services/stripe.rs:956`), `webhook_events`-PK idempotency that correctly *reprocesses* a
  partially-processed event, and full single-transaction atomicity with `?`-propagation; rejects
  before any DB write; prod-with-no-secret returns 500. Idempotency keys on refunds.
- **Money types:** `i64` cents end-to-end; DB `NUMERIC` with exact `::BIGINT / 100.0` (numeric, not
  float) conversion; `saturating_*` arithmetic; no `f64` in stored amounts or charge logic.
- **Auth:** Argon2id (OWASP params) + bcrypt-legacy verify with silent rehash + dummy-hash timing
  defense; HS256-pinned JWT with `exp` and access/refresh type segregation; Redis token-epoch
  revocation that **fails closed**; refresh rotation with reuse-detection chain invalidation.
- **OAuth:** Apple id_token JWKS-verified (RS256, iss/aud/exp/nonce); Google single-use state +
  PKCE + `email_verified`. Roles hardcoded server-side (`'user'`) — no client-supplied role/is_admin.
  `AdminUser`/`SuperAdminUser` extractors enforce DB role; email allowlist only honors verified accounts.
- **CSPRNG** for all sessions/tokens/state/nonce/PKCE/TOTP/backup codes; **constant-time** compares
  via `subtle`. Spoof-resistant client-IP via trusted-proxy CIDR allowlist.
- **SQL:** every *live* dynamic builder (`room_analytics`, `room_search`, `cms_audit`, `oauth/user`'s
  `AssertSqlSafe`, `schedules/admin`, ~12 `ORDER BY` sites) is correctly parameterized — interpolates
  only `$N` placeholders or hardcoded/allowlisted identifiers, never user values.
- **Concurrency:** **zero** `std::sync` guards held across `.await`; all shared state on
  `tokio::sync`; bounded mpsc/broadcast channels; L1 cache bounded with eviction; spawned tasks log
  their own errors; schedulers isolate per-run errors; Redis startup uses bounded exponential backoff;
  rate-limit mutations are atomic pipelines.
- **Panic safety:** slice/`parts[0]` access is length-guarded; proration divisor `.max(1)`; webhook
  handlers are the model for error propagation. No remotely-triggerable memory-unsafety found.

---

## Suggested remediation order

1. **P0-1** bind `download_file` to the token's `file_id` + verify HMAC (live IDOR, no tooling needed).
2. **P0-2 / P1-3** boot-validate `MEMBER_INDICATOR_SECRET` / `MEMBER_LICENSE_SECRET`; keyed HMAC / CSPRNG.
3. **P1-1 / P1-2** route subscription cancel through Stripe; add the live-membership unique index.
4. **P1-4** encrypt stored service credentials (AES-256-GCM).
5. **P1-5 / P1-6** WebSocket: abort forwarders + drop senders at zero (memory/FD leak under churn).
6. **P1-7** sweep `.unwrap_or_default()` on DB `Result`s → propagate (member/payment handlers first).
7. **P2** Redis `SCAN` + real L1 glob; TOTP replay counter; clamp pagination; UTF-8-safe masking.

> The toolchain gates (`fmt`, `check`, `clippy --all-targets` = 0 warnings) are green as of this
> audit. None of the findings above are caught by Clippy — they are logic/security issues requiring
> the fixes listed. This document is an audit only; no code changes were made for these findings.
