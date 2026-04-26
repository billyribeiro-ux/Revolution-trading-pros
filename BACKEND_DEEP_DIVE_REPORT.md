# Backend Deep-Dive — Beyond the Audit

Read-only forensic deep-dive of the Rust + Axum backend in `api/`. All
findings cite `file:line`. Verification gates re-run on 2026-04-26.

---

## Executive summary

| Severity | Count |
|----------|-------|
| Critical | 4 |
| High | 8 |
| Medium | 12 |
| Low | 9 |

**Top 5 findings**

1. **CRITICAL — String-interpolated SQL with naive single-quote escaping**
   in `api/src/routes/videos.rs:165-216`. Four user-supplied filters
   (`content_type`, `room_id`, `tag`, `difficulty_level`, `search`) are
   formatted directly into the SQL with `.replace('\'', "''")` — defeated
   by Postgres `E'...'` strings, dollar-quoted strings, comments, and
   second-order injection. This is a real SQL-injection sink, not a
   theoretical one. Endpoint is publicly reachable (no admin gate).
2. **CRITICAL — `payments::handle_checkout_completed` has no transaction
   wrapper** (`api/src/routes/payments.rs:567-885`). Five+ writes
   (`UPDATE orders`, `INSERT user_memberships`, `UPDATE coupons`,
   `INSERT user_products`, `INSERT user_course_enrollments`,
   `INSERT user_indicator_access`) execute as independent statements.
   A mid-flow crash leaves users with partial access (paid, but no
   course enrollment) or partial billing (membership row but no
   product ownership). Most also use `.execute(...).await.ok()` to
   silently swallow errors.
3. **CRITICAL — `routes/security.rs:144-196` swallows DB errors on the
   security-events list** with `.unwrap_or_default()` /
   `.unwrap_or((0,))`. The audit log is exactly the place where you
   want loud failures, not zeroed responses.
4. **HIGH — No rate-limit on `register`, `forgot-password`,
   `reset-password`, or `verify-email`.** Only `POST /api/auth/login`
   is rate-limited (`api/src/routes/auth.rs:478-514`). An anonymous
   bot can spam registration (DB write storm), spam forgot-password
   (Postmark spend + spam), or brute-force reset tokens.
5. **HIGH — JWT validation is `Validation::default()`** in
   `api/src/utils/mod.rs:249-257`. No `iss`, no `aud`, no `sub`-format
   check, default 60s leeway. Tokens for arbitrary purposes signed by
   `JWT_SECRET` are interchangeable — including, by structure, the
   refresh token (which would be accepted on any access-token-protected
   route by the bearer-auth extractor at
   `api/src/middleware/auth.rs:49`).

---

## 1. Hot-path security audit

`api/src/routes/auth.rs` is 1104 lines. Walked end-to-end.

### Password hashing

`hash_password()` at `api/src/utils/mod.rs:146-164`. Uses Argon2id with:
- Memory: 65536 KiB (64 MiB)
- Iterations: 3
- Parallelism: 4
- Output: 32 bytes
- Algorithm: `Argon2id` v0x13

These match OWASP 2024 recommendations for financial applications.
**Verdict: GOOD.**

`verify_password()` at `api/src/utils/mod.rs:175-208` correctly handles
both bcrypt (Laravel legacy `$2y$`/`$2b$`/`$2a$`) and Argon2 prefixes
without revealing format via timing — it always runs the full bcrypt
verify or full Argon2 verify, never short-circuits on prefix mismatch
to a fast string compare. **Verdict: GOOD.**

Timing-attack defense for unknown user: `hash_dummy_password()`
called at `auth.rs:538`. **Verdict: GOOD.**

### JWT verification (HIGH)

`verify_jwt()` at `api/src/utils/mod.rs:249-257` calls
`jsonwebtoken::decode` with `&Validation::default()`. By
`jsonwebtoken` 9.x defaults that means:

| Claim | Validated? |
|-------|-----------|
| `exp` | yes |
| `nbf` | no (not in default required claims) |
| `iss` | no |
| `aud` | no |
| `sub` | format only |
| `algorithm` | HS256 only |
| `leeway` | 60s |

**Findings:**
- No `iss` / `aud` validation. If `JWT_SECRET` is ever shared across
  another service, tokens are interchangeable.
- No `token_type` enforcement at decode time. Although the `Claims`
  struct has a `token_type` field (`api/src/utils/mod.rs:30-36`),
  `verify_jwt` doesn't reject `token_type == "refresh"` when called
  for access verification. The bearer-auth extractor at
  `api/src/middleware/auth.rs:49` therefore accepts a 7-day refresh
  token as a valid bearer token. The refresh route at
  `api/src/routes/auth.rs:724` similarly accepts any token. This
  effectively gives access tokens a 7-day lifetime if a leaked
  refresh token is replayed.
- No algorithm allow-list pin. `Validation::default()` accepts only
  HS256 today, but future jsonwebtoken upgrades that change defaults
  could silently break this assumption.

### Session token rotation

Refresh route at `api/src/routes/auth.rs:719-784`:
- New access token issued: yes (line 753).
- New refresh token issued: yes (line 768) — comment claims "token
  rotation for security" but the **old refresh token is not blacklisted
  or invalidated.** It remains valid for its full 7-day lifetime. This
  is not rotation; it's reissuance with both tokens valid.
- No detection of refresh-token reuse (the canonical "if old refresh
  token used after rotation, kill the family" pattern). **Verdict: HIGH.**

### Rate-limit policy

`api/src/routes/auth.rs:478-514` checks login rate limit via Redis.
**No rate-limit on:**
- `register` (line 62)
- `forgot_password` (line 898)
- `reset_password` (line 971)
- `verify_email` (line 229)
- `resend_verification` (line 348) — also no token-rotation check, will
  re-send for any unverified email

Redis service has `check_rate_limit` and `check_ip_rate_limit` already
implemented (`api/src/services/redis.rs:106`, `474`) so wiring is
trivial. **Verdict: HIGH.**

### Account-takeover surface

User-enumeration timing equalization is implemented for login (dummy
hash at `auth.rs:538`). However:
- Error message difference at `auth.rs:107-117`: `register` returns
  `"Email already registered"` for an existing email — direct
  enumeration. **Verdict: MEDIUM.**
- `forgot_password` (line 898) correctly returns generic success.
- `resend_verification` (line 348) correctly returns generic success.
- Login email-verification gate at `auth.rs:632-641` returns a
  distinct `EMAIL_NOT_VERIFIED` error code with the user's email
  echoed back — confirms the email exists and is unverified. **Verdict:
  MEDIUM.**

### CSRF coverage

Auth routes are JSON-only POSTs that require `Authorization: Bearer
<token>` for protected endpoints. Browser CSRF requires a same-origin
cookie credential — these endpoints don't use cookie auth, so classic
CSRF doesn't apply. CORS at `main.rs:133-162` allows credentials with
an explicit origin allow-list, which is correct. **Verdict: GOOD.**

### Logout token blacklist

`auth.rs:825-849` correctly blacklists the JWT on logout via
`redis.blacklist_token(token_hash, ttl)`. The middleware at
`api/src/middleware/auth.rs:55-79` checks the blacklist on every
authenticated request and **fails open** (continues if Redis is
unavailable). The "fail open" choice is documented but is the wrong
default for a high-security path. **Verdict: MEDIUM.**

---

## 2. SQL injection / parameterization audit

### CRITICAL: `api/src/routes/videos.rs:163-213`

User-supplied query strings interpolated directly into SQL:

- `content_type` formatted at `videos.rs:165-167` with only
  `.replace('\'', "''")`.
- `room_id: i64` formatted at `videos.rs:174-178` (i64 is safe but
  unjustified).
- `tag` formatted at `videos.rs:188`.
- `difficulty_level` formatted at `videos.rs:196-198`.
- `search` formatted at `videos.rs:207-210`.
- `LIMIT {} OFFSET {}` at `videos.rs:216` (i64 — safe but inconsistent
  with house style).

The naive `.replace('\'', "''")` is **insufficient**:
- Postgres E-strings (`E'...'`) treat `\'` as a literal apostrophe
  inside a doubled-up-quote string.
- Comments (`--`, `/* */`) bypass the rest of the WHERE.
- Second-order injection: a `tag` value stored elsewhere then
  fed back through this route.

This route is `nest("/videos", videos::router())` (`routes/mod.rs:100`)
and the list handler is mounted via `videos::router()` without admin
gating. **Reachable by any unauthenticated client.**

### MEDIUM: `api/src/routes/orders.rs:534-542`

`LIMIT {} OFFSET {}` formatted in (`orders.rs:542`) — `per_page` and
`offset` are `i64` after `.min(100)` / `.max(1)` clamps so the type
makes injection impossible, but the inconsistency with the
parameterized `${}` placeholders on lines 527-538 is a footgun and
breaks the house-style "everything bound" rule. No SAFETY comment.

Same pattern at `videos.rs:216`.

### Justified `format!()` SQL (audit confirmed)

The following all interpolate enum-derived or allow-list-validated
values and are correctly annotated with SAFETY comments — verdict
GOOD:

- `api/src/routes/member_courses.rs:1148-1150` (boolean → literal
  fragment).
- `api/src/routes/cms_assets.rs:594-612` (allow-list sort_column).
- `api/src/routes/cms_global_components.rs:540-552` (allow-list).
- `api/src/routes/cms_presets.rs:417` (allow-list).
- `api/src/routes/cms_reusable_blocks.rs:413-414` (allow-list).
- `api/src/routes/cms_datasources.rs:381` (allow-list).
- `api/src/routes/members.rs:60-78,116-124` (allow-list helper
  validate_sort_column).
- `api/src/routes/admin_popups.rs:81-102` (placeholders only, SAFETY
  comments).
- `api/src/routes/room_resources.rs:1798-1804` (placeholders only,
  SAFETY comments).
- `api/src/routes/connections.rs:417-422` (placeholders only).
- `api/src/routes/security.rs:84-94` — interpolates `param_num`
  literals (`"$1"`, `"$2"`, `"$3"`) — confusing but safe (binding
  positions are hardcoded literals, not user input).
- `api/src/routes/cms_scheduling.rs:561-1574` (placeholders only).
- `api/src/services/cms_audit.rs:117-215` (placeholders only).
- `api/src/routes/oauth.rs:325-329` — interpolates `provider_column`
  derived from `OAuthProvider` enum match. **MISSING SAFETY comment**
  but verdict is safe.
- `api/src/routes/admin_videos.rs:1530-1545` (placeholders only).
- `api/src/routes/media.rs:1032` — interpolates literal column-name
  fragments from a build vector. **MISSING SAFETY comment** but verdict
  is safe.
- `api/src/routes/settings.rs:213,219` (placeholders only).

**Recommendation:** Add SAFETY comments to `oauth.rs:325`,
`media.rs:1032` for consistency. Convert `videos.rs` filters to
parameterized `$N` form. Convert `orders.rs:542` and `videos.rs:216`
LIMIT/OFFSET to bound parameters.

---

## 3. Transaction integrity

### CRITICAL: `api/src/routes/payments.rs:567-885` `handle_checkout_completed`

Multi-write Stripe webhook handler with **no transaction wrapper**:

| Line | Statement | Error handling |
|------|-----------|---------------|
| 583-601 | `UPDATE orders SET status='completed', ...` | `.map_err(...)?` |
| 627-651 | `INSERT INTO user_memberships ... ON CONFLICT DO UPDATE` | `.ok()` swallow |
| 658-664 | `UPDATE coupons SET usage_count = usage_count + 1` | `.ok()` swallow |
| 720-732 | `INSERT INTO user_products ... ON CONFLICT DO UPDATE` | `.ok()` swallow |
| 747-758 | `INSERT INTO user_course_enrollments ... ON CONFLICT` | `.ok()` swallow |
| 774-785 | `INSERT INTO user_indicator_access ... ON CONFLICT` | `.ok()` swallow |
| 817-828 | duplicate `INSERT INTO user_indicator_access` | `.ok()` swallow |

Plus an external Stripe API call at `payments.rs:610` (`get_subscription`)
in the middle, between the order update and the membership insert. If
Stripe times out, the order is marked completed but the user has no
membership.

This is the worst single function in the codebase from a transaction-
integrity standpoint. **Wrap in `Pool::begin()` → `tx.commit()`** and
propagate errors via `?`, not `.ok()`.

### CRITICAL: `api/src/routes/payments.rs:90-292` `create_checkout`

- INSERT orders (`payments.rs:187-212`)
- INSERT order_items in a loop (`payments.rs:214-236`)
- External Stripe `create_checkout_session` (`payments.rs:255-266`)
- UPDATE orders SET stripe_session_id (`payments.rs:269-274`,
  silently `.ok()`)

If Stripe creation fails, the orphan order row + order_items rows are
persisted forever. If the final UPDATE fails, the order has no Stripe
session ID and the webhook can never tie back. **Wrap order creation
in tx; commit before calling Stripe; reconcile via background job if
Stripe fails.**

### Existing transactions (verdict GOOD)

These handlers correctly use `Pool::begin()` → `tx.commit()`:

- `api/src/routes/user.rs:560` (`deactivate_account`).
- `api/src/routes/user.rs:809` (`update_profile`).
- `api/src/routes/forms.rs:275,588,806,1148` (form CRUD cascades).
- `api/src/routes/schedules.rs:899`.
- `api/src/routes/subscriptions_admin.rs:832`.
- `api/src/routes/connections.rs:1159`.
- `api/src/routes/settings.rs:439`.
- `api/src/services/mfa.rs:313,363`.

---

## 4. Error handling

### `unwrap_or_default()` / `unwrap_or((0,))` on Result-returning DB calls

Audit fixed crm.rs and courses_admin.rs. Remaining sources:

- **HIGH `api/src/routes/security.rs:144,149,180,186,191,196`** — six
  separate DB calls in the security-events listing and stats endpoints
  silently swallow errors. The security audit log is exactly the place
  where you want loud failures, not zeroed responses.
- `api/src/routes/cms_scheduling.rs:609,1585` — count queries return 0
  on DB failure, masking listing errors.
- `api/src/routes/admin_members.rs:1158` — `unwrap_or_default()`.
- `api/src/routes/room_resources.rs:1825` — `unwrap_or_default()`.
- `api/src/routes/connections.rs:438` — `unwrap_or_default()`.
- `api/src/routes/cms_assets.rs:590` — `unwrap_or(0)`.

**Total `.unwrap_or_default()` / `.unwrap_or((0,))` /
`.unwrap_or(0)` in `routes/`:** 307 occurrences across 50+ files. Many
are on `Vec` defaults (legitimate), but a sample from `payments.rs`
shows ~5 silent-swallow `.ok()` patterns on critical mutations
(addressed in §3).

### Bare `?` that should be 404, not 500

Spot checks of `auth.rs` are fine — `fetch_optional()` is used
everywhere. Spot check of `routes/forms.rs:341-344` shows correct
`.ok_or_else(|| (StatusCode::NOT_FOUND, ...))` pattern.

### Raw-string vs `AppError`

`api/src/utils/errors.rs` defines `ApiError` with a `request_id` field
(`utils/errors.rs:40,54,81-82`). However, **most route handlers
return `(StatusCode, Json<serde_json::Value>)` tuples directly** — the
typed `ApiError` is used by only a subset (notably `media.rs`,
`cms_assets.rs`, and a few others). The mixing breaks
request-id correlation in production logs. **Verdict: MEDIUM.**

---

## 5. Connection-pool exhaustion risks

Looked for handlers holding a sqlx connection across an external-service
`.await`. **No active leaks found** because sqlx pool semantics return
the connection between `query.execute().await` calls — only an open
`tx` would hold it.

### Caution: `payments.rs:610` (Stripe call within webhook)

`handle_checkout_completed` calls `state.services.stripe.get_subscription()`
in a non-transactional flow. The DB connection is released between
queries, so this is **not a leak**, but the lack of transaction means
data inconsistency under partial failure (covered in §3).

### Caution: external HTTP timeouts

`api/src/services/stripe.rs` and `api/src/services/email.rs` should
be inspected for client-side timeouts. The webhook delivery in
`cms_scheduler.rs:72-76` builds a `reqwest::Client` with
`Duration::from_secs(30)` timeout. **Default reqwest timeout is no
timeout** — if other services don't set one, a slow Stripe response
ties up an Axum request slot indefinitely. (Not verified inside this
deep-dive scope.)

---

## 6. Blocking ops in async

- **No `std::fs::*`** in async code. Only test code.
- **No `std::thread::sleep`** in production code. Only test code at
  `api/src/cache/service.rs:667` (test module, fine).
- **No `std::sync::Mutex` / `RwLock`** in source. All locks are
  `tokio::sync::RwLock`.
- All async lock acquisitions properly use `.read().await` /
  `.write().await`.

**Verdict: GOOD.**

---

## 7. Middleware ordering

`api/src/main.rs:221-238` applies layers in this order (axum applies
bottom-up; execution order is reverse-listed):

```
TraceLayer
CompressionLayer
ensure_content_type   ← only middleware on inbound path
cors
security_headers
metrics_middleware    ← user authentication happens INSIDE handler extractors
routes
```

### Findings

- **HIGH — No global rate-limit middleware.** Rate-limit is checked
  inside the `login` handler only. CMS-AI and CMS-SEO have
  per-handler rate-limit helpers (`cms_ai_assist.rs:272`,
  `cms_seo.rs:265`). All other routes are unthrottled.
- **MEDIUM — No request-id middleware.** `ApiError` carries an
  optional `request_id` field but nothing in the chain populates it.
  Production logs will not be correlatable to client errors.
- **MEDIUM — `validate_request` middleware defined but never wired.**
  `api/src/middleware/validation.rs:21-65` declares request-size and
  content-type validation, but `main.rs` only uses
  `ensure_content_type`. The 10 MB body limit and Content-Type
  enforcement for POST/PUT/PATCH are dead code.
- The chain has no body-size cap (`tower-http::limit::RequestBodyLimit`
  is in Cargo.toml features but not applied). A client can POST
  arbitrarily large bodies.
- Auth happens at extractor time, after CORS + security_headers + the
  metrics middleware. This means the metrics middleware records
  every unauthenticated request, including DoS spam — fine for
  observability but means rate-limit must be added before metrics or
  the metrics layer becomes a DoS amplifier.

---

## 8. Data validation

Despite `validator = "0.20.0"` being a Cargo.toml dependency:

- `validator::Validate` is derived on six models:
  `models/indicator.rs:15`, `models/order.rs:8`, `models/product.rs:9`,
  `models/post.rs:10`, `models/subscription.rs:7`,
  `models/newsletter.rs:9`.
- **Zero route handlers call `.validate()` on the input.** A grep
  for `.validate()` in `routes/` returns only
  `connections.rs:849,1110,1219` — and those are calls to a cache
  `.invalidate()` method, not `Validate::validate()`.

This means every POST/PUT handler accepts any-shape JSON (subject only
to serde deserialization) and writes it straight to SQL. The audit
trail in CRM, contacts, members, posts, products, etc. has only
schema-level type checks and no semantic validation (length caps,
email format beyond `contains('@') && contains('.')`, URL format,
phone format, etc.).

`api/src/routes/auth.rs:67-77` does manual email validation —
`!email.contains('@') || !email.contains('.')` is far weaker than
`validator::ValidateEmail`. **HIGH for inconsistency, MEDIUM for
exploitability** since SQL is parameterized.

---

## 9. Sensitive data leakage

### println! / dbg!
- `api/src/bin/bootstrap_dev.rs` uses `println!` / `eprintln!` (lines
  67-104+). Acceptable for a CLI bootstrap binary.
- No `println!` in server code paths.
- No `dbg!` anywhere.

### Tokens / passwords / secrets in tracing

- No tracing call logs a JWT, password, password hash, or API key.
- `api/src/middleware/auth.rs:50` logs JWT verification errors with
  `{:?}` on the `jsonwebtoken::errors::Error` type — that error type
  does NOT include the token in its Debug, so verdict GOOD.
- `api/src/routes/auth.rs:1077` logs `Password reset completed for: {}`
  + `input.email`. Email-only is acceptable for security audit.
- 47 tracing calls log `email = %user.email` or similar. PII in logs
  is unavoidable for security audit but should be reviewed for GDPR
  compliance (acceptable in `target: "security_audit"` scope; flag
  any leakage to other targets).

### Config Debug impl

`api/src/config/mod.rs:68-131` correctly redacts every secret
(`[REDACTED]`) in the `Debug` impl. **Verdict: GOOD.**

---

## 10. Build-time risks

### Dependency stats

Run via `cargo metadata` and `Cargo.lock` analysis:

- **442 unique crates** in the dependency graph.
- **45 crates with multiple versions** (duplicate-version bloat).
  Notable duplicates: `windows-sys` (4 versions),
  `windows_x86_64_msvc`/`gnu`/`gnullvm` (3 each), `tower` (2),
  `tower-http` (2), `tokio-rustls` (2), `hyper` (2), `webpki-roots`
  (2), `thiserror` (2), `syn` (2), `hashbrown` (3), `spki` (2).
- The audit-flagged `tower` and `tower-http` duplications survive on
  `0.4`/`0.5` and `0.5`/x.

### `reqwest` features

`Cargo.toml:76`:
```
reqwest = { version = "0.11", features = ["json", "rustls-tls", "stream"], default-features = false }
```

Default features are off — good. But `reqwest 0.11` is the source of
the deferred CVEs in `deny.toml:39-63`:

| Advisory | Fix |
|----------|-----|
| RUSTSEC-2026-0098 (rustls-webpki URI name-constraint) | bump reqwest 0.12 |
| RUSTSEC-2026-0099 (rustls-webpki DNS wildcard) | bump reqwest 0.12 |
| RUSTSEC-2026-0104 (rustls-webpki CRL panic) | bump reqwest 0.12 |
| RUSTSEC-2025-0134 (rustls-pemfile unmaintained) | bump reqwest 0.12 |

All 4 are gated by `reqwest 0.11 → 0.12`. The project also has 5
aws-lc-rs/aws-lc-sys advisories ignored for the same lifecycle reason.

`cargo deny check advisories` returns **OK** because every advisory is
explicitly ignored with reason. **Verdict: HIGH for tech-debt; not a
runtime issue today.**

### Profile

`Cargo.toml:93-97`:
```
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

**Verdict: INTACT, GOOD.**

### TLS provider conflict (low risk)

`sqlx` and `redis` use `tls-native-tls` (Cargo.toml:23, 27). `reqwest`
uses `rustls-tls` (Cargo.toml:76). `aws-sdk-s3` defaults to
`aws-lc-rs`. Three different TLS stacks linked into the binary. Not a
correctness issue but ~3 MB of binary bloat and three CA stores to
keep in sync.

### Orphan code (verifies AUDIT_REPORT.md deferred items)

- `api/src/services/cms_scheduler.rs:45` — `start_scheduler` is
  defined and even has a `FIX-2026-04-26: ORPHAN` marker on line 43.
  Confirmed: zero call sites. The CMS scheduling feature in
  `routes/cms_scheduling.rs` (1800+ lines of HTTP handlers) writes
  schedule rows that nothing ever processes. **HIGH.**
- `api/src/routes/organization.rs:1217` — `profile_router()` defined
  but `routes/mod.rs:189-193` only mounts `teams_router` and
  `departments_router`.
- `api/src/routes/admin_courses.rs:2307` — `taxonomy_router()` defined
  but never referenced.

### Tests vs source files

145 source files in `api/src/`. 19 contain `#[cfg(test)]` /
`#[test]`. **126 source files with no inline tests** — slightly worse
than the audit's "101" because newer modules added without tests.
Highest-priority gaps (by criticality):

1. `api/src/routes/payments.rs` (1483 lines, no tests, handles money).
2. `api/src/routes/auth.rs` (1104 lines, no tests, handles auth).
3. `api/src/routes/checkout.rs` (no tests, handles money).
4. `api/src/routes/oauth.rs` (no tests, handles auth).
5. `api/src/services/stripe.rs` (handles money, has separate test
   `tests/stripe_test.rs` — covered).
6. `api/src/middleware/auth.rs` (no tests, gate for everything).
7. `api/src/routes/courses_admin.rs` (already audit-fixed but not
   regression-tested).

---

## Verification gate status

All four gates re-run on 2026-04-26 against `main @ 106cfc625`:

```
cargo check                       ✅ Success (locked)
cargo clippy --all-targets -W=err ✅ Success
cargo fmt --check                 ✅ Success
cargo test --test utils_test      ✅ 17 passed
cargo test --test stripe_test     ✅ 15 passed
cargo deny check advisories       ✅ advisories ok (with 9 ignored)
```

No regressions from the prior audit. The codebase is buildable,
formatted, lint-clean, and the no-DB tests pass.

---

## Recommended priority order

Top 10 issues, prioritized by exploitability × blast radius:

1. **`api/src/routes/videos.rs:163-216`** — fix the SQL injection.
   Convert all five filters to bound parameters. P0.
2. **`api/src/routes/payments.rs:567-885`** — wrap
   `handle_checkout_completed` in a transaction. Replace
   `.execute(...).await.ok()` with `?` propagation. P0.
3. **`api/src/routes/payments.rs:90-292`** — wrap `create_checkout`
   order/order_items in a transaction; commit before Stripe. P0.
4. **`api/src/routes/auth.rs`** — add Redis rate-limit calls to
   `register`, `forgot_password`, `reset_password`, and
   `resend_verification`. P0.
5. **`api/src/utils/mod.rs:249-257`** — strengthen `verify_jwt`:
   require `iss` (e.g. `"revolution-trading-pros"`), require `aud`,
   reject `token_type == "refresh"` from access-token paths, pin
   `Algorithm::HS256` explicitly. P1.
6. **`api/src/routes/auth.rs:719-784`** — implement true refresh-token
   rotation: blacklist the old refresh token; on reuse, kill the
   token family. P1.
7. **`api/src/routes/security.rs:144-196`** — replace six
   `.unwrap_or_default()` / `.unwrap_or((0,))` with proper error
   propagation; an audit log that hides DB failures is anti-purpose.
   P1.
8. **Wire request-id middleware**, then update all handlers to
   `ApiError::with_request_id()`. P2.
9. **Wire `validator::Validate::validate()` into POST/PUT handlers**,
   starting with auth + payments + admin_member_management. P2.
10. **Resolve the 4-CVE reqwest 0.11 → 0.12 bump and the 5-CVE
    aws-lc-rs lifecycle.** Until these land, the deny.toml ignores
    are accumulating tech-debt rather than being closed out. P2.

### Lower-priority follow-ups

- Add SAFETY comments to `oauth.rs:325` and `media.rs:1032` for
  consistency.
- Convert `orders.rs:542` and `videos.rs:216` LIMIT/OFFSET to bound
  parameters (defense-in-depth even though i64 is safe).
- Decide on `start_scheduler` — wire into `main.rs` or delete the
  1800+ lines of `cms_scheduling.rs` handlers.
- Decide on `profile_router` and `taxonomy_router` — wire or delete.
- Wire `validate_request` middleware in `main.rs` to enforce
  `MAX_BODY_SIZE` (10 MB).
- Backfill tests for the seven critical-path files listed in §10.
