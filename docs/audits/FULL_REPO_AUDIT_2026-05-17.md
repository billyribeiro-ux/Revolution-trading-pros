# Full Repository Audit — 2026-05-17

End-to-end audit of the Revolution Trading Pros monorepo (SvelteKit 2 /
Svelte 5 frontend, Rust + Axum backend).

- **HEAD audited:** `c31c43d` (Merge PR #573)
- **Date:** 2026-05-17
- **Prior baseline:** [`REPO_AUDIT_2026-05-10.md`](REPO_AUDIT_2026-05-10.md) — 31 commits since; commit `4420d1d` claimed the P0 fixes.
- **Corpus:** frontend ≈664k LOC / 1,600 files · Rust API ≈105k LOC / 150 files · 60 SQL migrations · 745 route files.
- **Method:** every quality gate run end-to-end + six parallel deep-read
  audits (backend-core, payments/Stripe, frontend-architecture,
  Svelte-5-hygiene, security, tests/migrations/hygiene). Every finding
  marked **[V]** was opened and verified by hand in the main thread;
  unmarked findings are agent-reported with a cited file:line.

---

## 1. Quality gates — actual results

| Gate | Result | Detail |
|---|---|---|
| `pnpm check` (svelte-check) | ✅ | 4,541 files, **0 errors, 0 warnings** |
| `pnpm test:unit` (vitest) | ✅ | 36 files, **1,700 passed, 32 skipped, 0 failed** (78 s) |
| `cargo check --locked` | ✅ | clean compile (3m19s) |
| `cargo clippy --locked --all-targets -D warnings` | ✅ | **clean** — no warnings (2m00s) |
| `pnpm lint` (eslint) | ✅ | **0 errors**, 1,718 warnings (prior 5 errors fixed; warnings = pre-existing `any`/`!`/`@ts-ignore` debt) |
| `cargo test --test utils_test --test stripe_test` | ⏳ | compiling at report time — see §11 addendum |
| Playwright e2e | ⏸ | needs running Docker stack (not booted) |
| `cargo machete` / `cargo deny` | ⚠ | binaries not installed in this environment |
| `svelte-autofixer` MCP | ⚠ | **permission-denied** to the audit agents — Svelte-5 autofix coverage gap (CLAUDE.md mandates it; could not run) |

Type safety and unit tests are at a clean baseline. The environment
required two fixes before gates could run: frontend `node_modules` was
absent (installed); pnpm skipped the esbuild/sharp/workerd native build
scripts (rebuilt at workspace root).

---

## 2. Verdict on the prior (2026-05-10) P0 claims

| Prior P0 | Status | Proof |
|---|---|---|
| `subscriptions.rs:446` placeholder checkout URL | **FIXED** | `subscriptions.rs:457-473` now calls `create_subscription_checkout`; 502 on Stripe failure, persistence deferred to webhook |
| Idempotency-Key on every mutating Stripe POST | **FIXED** (keys not persisted — see P2-A) | `stripe.rs:396,546,639,705,754,826,842,867,900,1002,1013,1069,1135,1159,1198,1258,1307,1390` |
| Webhook multi-table writes wrapped in a transaction | **NOT FIXED** | zero `begin()`/`commit()` in `payments.rs`; `handle_checkout_completed` writes 6+ tables non-atomically |
| 5 eslint errors | **FIXED** | `svelte-check` 0/0; eslint src warnings driven to 0 in later commits (gate count in §11 addendum) |

The Stripe-checkout and idempotency P0s are genuinely closed. **The
webhook-atomicity P0 is still open** and is joined by several new
release-blockers below.

---

## 3. P0 — Release-blocking

### P0-1 [V] Customer checkout is broken end-to-end (route mismatch)
`frontend/src/routes/checkout/+page.svelte:19` imports `createCheckoutSession`
from `$lib/api/cart`; `cart.ts:1132` does `POST ${API_URL}/checkout/create-session`
with a `{cart,provider,returnUrl,cancelUrl}` body. The backend
`checkout::router()` (`api/src/routes/checkout.rs:654-659`, nested at
`routes/mod.rs:102`) only registers `/`, `/calculate-tax`, `/orders`,
`/orders/:order_number`. **`/checkout/create-session` does not exist
anywhere in `api/src/`**, and the body shape does not match
`CheckoutRequest` (`checkout.rs:26-36`). Every "Place Order" click →
`Failed to create checkout session`. The working, transactional
`create_checkout` handler (`checkout.rs:65`) is never called by the
customer page. **Fix:** point `cart.ts:1132` at `${API_URL}/checkout`
and send the `CheckoutRequest` shape (or swap the page to a client that
calls `POST /api/checkout`). Delete the second dead
`createCheckoutSession` in `lib/api/checkout.ts:149`.

### P0-2 [V] Committed superadmin credentials + production env file
`welberribeirodrums@gmail.com` / `Davedicenso01!` are hardcoded in
**four** committed e2e specs: `frontend/tests/e2e/admin/role-gate.spec.ts:33-34`,
`admin/subscription-checkout.spec.ts:32-33`, `admin-sweep.spec.ts:23,36-37`,
and `verify_tag_fix.spec.ts:26,47,50` (the last has **no** env fallback —
the literal password is the only value). The same email is set as
`VITE_SUPERADMIN_EMAILS` / `VITE_DEVELOPER_EMAILS` in the **git-tracked**
`frontend/.env.production:39-40`. Because the backend treats
`is_superadmin_email` as an elevation path (security M1 below), this is a
live superadmin email **and** password in git history. **Fix:** rotate
the password immediately; replace all fallbacks with a placeholder that
throws when the env var is unset; treat email-list elevation as
break-glass only (require DB role + `email_verified_at`).

### P0-3 [V] `/admin` role gate is client-side only (defense-in-depth gap)
No `frontend/src/routes/admin/+layout.server.ts` exists;
`admin/+layout.ts` sets `ssr = false`; the role check is a client
`onMount` redirect in `admin/+layout.svelte:87-103`.
`hooks.server.ts:32` lists `/admin` in `PROTECTED_ROUTES` but only
verifies the token resolves to *a user*, never `role`. An authenticated
non-admin is served the admin shell; only JS stands between them and
`/api/admin/*` calls. The Rust API still enforces RBAC on the data
(security audit confirms extractors are uniform), so this is a
defense-in-depth / data-exposure-window gap rather than full data
compromise — but the prior 2026-05-10 audit's "the /admin gate is real"
is **overstated** and should be corrected. **Fix:** add
`admin/+layout.server.ts` with a `load` that 303-redirects when
`!locals.user || !ADMIN_ROLES.has(locals.user.role)`.

### P0-4 Webhook handler multi-table writes are not atomic
`api/src/routes/payments.rs` contains **zero** `pool.begin()`/`tx.commit()`
across all 2,112 lines. `handle_checkout_completed` (`payments.rs:416-922`)
writes `orders` (475), `user_memberships` (565), `coupons.usage_count+1`
(602), `user_products` (676), `user_course_enrollments` (707),
`user_indicator_access` (737/789) as independent statements. A crash
mid-handler leaves the customer charged with no membership/access; the
webhook-dedup table (`payments.rs:237-244`) then **drops the Stripe
retry**, so the missing rows are never written. This is the still-open
prior P0. **Fix:** wrap the handler body in a transaction; only mark
`webhook_events.processed_at` after commit, and reprocess rows where
`processed_at IS NULL` instead of skipping by event-id presence.

### P0-5 Subscription-plan create/edit drops the price (money-correctness)
Independently re-confirmed from [`MONEY_PATH_DIG_2026-05-17.md`](MONEY_PATH_DIG_2026-05-17.md)
F2. `frontend/src/routes/admin/subscriptions/plans/+page.svelte:283-293`
and `subscriptionPlansApi.create/.update` (`lib/api/admin.ts:1248/1257`)
send `price` (dollars) raw; Rust `CreatePlanRequest.price_cents: i64` is
required (`subscriptions_admin.rs:119-136`). Create fails; **edit
silently drops the price change** (`Option`, no error — the dangerous
one). **Fix:** add a `normalizePlanPayload` emitting
`price_cents = Math.round(price*100)` (mirror the working coupon
normalizer); confirm intended flow vs the separate `/plans/:id/price`
endpoint first.

### P0-6 [V] Stored-XSS class via unescaped JSON-LD `{@html}`
`SEOHead.svelte:303-318` `stableStringify` sorts keys with
`JSON.stringify` on values but never escapes `<`, `>`, `/`, U+2028/9;
output is injected via `{@html}` at `:971`. `seo/SeoHead.svelte:217`
uses raw `JSON.stringify`. `store/scanners/+page.svelte:186` and
`tools/options-calculator/+page.svelte:38-39` repeat the pattern. Schema
fields draw from CMS/user content (`headline:title` :495, FAQ
`name:item.question` :581, `author` :459, breadcrumb/video names). A
field containing `</script><script>…` breaks out and executes. **Fix:**
escape `<`/`>`/`&`/U+2028/U+2029 in the serialized JSON before
interpolation (e.g. `.replace(/</g,'\\u003c')`), or use a vetted
JSON-LD helper. All ~70 other `{@html}` sinks are DOMPurify-sanitized
(verified) — this class is the exception.

### P0-7 Course/video paywall bypass
`api/src/routes/member_courses.rs:255-353` `get_course_player` loads the
course by slug for any authenticated user, returns all lessons including
`bunny_video_guid` + downloads, and only sets
`"is_enrolled": enrollment.is_some()` — **no `403` when not enrolled**
(verified: the handler has no `FORBIDDEN`/`return Err` on the
enrollment-None path; the lesson SELECT includes `bunny_video_guid`).
Any free account pulls paid-course video GUIDs and download links.
**Fix:** if the course is not free and `enrollment.is_none()` and not
admin → `403`, or strip `bunny_video_guid`/`downloads`/non-preview
lessons from the response.

---

## 4. P1 — High

### P1-1 [V] Real Google API key + OAuth client ID committed
`frontend/src/lib/config/trading-rooms.ts:57,116,167,277,336` hardcode
`calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw'` plus an
OAuth client ID, duplicated across `dashboard/*-room/+page.svelte` and
the `Implementation/`/`Do's/` dumps. Browser-exposed keys can't be
secret, but an unrestricted key invites quota/billing abuse. **Fix:**
rotate; add Google Cloud API + HTTP-referrer restriction; source from a
`VITE_` env var, not 8+ literals.

### P1-2 Stolen access JWT survives logout-all / password-reset / ban
`middleware/auth.rs` + `services/redis.rs:233-256` +
`auth.rs:1182-1208`: `invalidate_all_user_sessions()` only deletes Redis
`session:*` keys; access JWTs are blacklisted **only** on single-device
`/logout`. There is no per-user `token_version`/epoch, so a stolen
access token stays valid for its full TTL after the victim resets their
password or logs out everywhere; a stolen refresh token survives until
rotation. **Fix:** per-user `token_version` embedded in the JWT, bumped
on logout-all/reset/ban, checked in the auth extractor.

### P1-3 Per-IP rate limits bypassable via spoofed `X-Forwarded-For`
`api/src/routes/auth.rs:48-62` `client_ip()` trusts the client
`x-forwarded-for`/`x-real-ip` with no trusted-proxy allowlist or
`ConnectInfo` socket fallback. Rotating the header gives a fresh bucket
per request, defeating the login (10/15m), register (5/15m),
forgot (3/h), reset (5/h) limiters; only per-email login survives.
(Backend-core and security agents both flagged this.) **Fix:** derive IP
from `ConnectInfo<SocketAddr>`; honor `X-Forwarded-For` only from
configured trusted-proxy CIDRs.

### P1-4 Unauthenticated admin/infra endpoints
- `GET /api/admin/coupons/validate/:code` (`admin.rs:1287`, mounted
  `admin.rs:2317`) has **no auth extractor** — the lone gap among 37
  handlers in the file; returns full `CouponRow` incl. `stripe_coupon_id`
  to anonymous callers (coupon enumeration).
- `POST /init-db` (`health.rs:346-371`) auth is only a defaultable
  `ENVIRONMENT` string check; `setup_db`/`run_migrations`
  (`health.rs:159/294`, contain `DROP TABLE … CASCADE`) gated by
  `AdminUser` not `SuperAdminUser`.
- `/monitoring/metrics`, `/metrics/json`, `/health/detailed`
  (`monitoring/mod.rs:155-160`) unauthenticated recon surface.
**Fix:** add `AdminUser`/`SuperAdminUser` extractors respectively;
network-ACL the metrics endpoints.

### P1-5 Admin analytics serve fabricated data
`admin_members.rs:1119` `analytics_churn_reasons` is 100% hardcoded;
`analytics_revenue` (`:1073`) hardcodes `avg_order_value_cents: 12750`
and `revenue_by_month: []`. Operators see invented retention/revenue
numbers with no "synthetic" indicator. **Fix:** compute from real
tables or return `501`/`data_available:false`.

### P1-6 `/api/auth/set-session` sets httpOnly auth cookie from an unauthenticated body
`frontend/src/routes/api/auth/set-session/+server.ts:20-58` writes
`body.access_token` into `rtp_access_token` with no origin/CSRF/token
validation — session-fixation / login-CSRF. The OAuth callback comment
says it's "no longer needed". **Fix:** delete the POST handler, or
validate the token via `GET /api/auth/me` + add an origin check.

### P1-7 Money not stored as integer cents at rest
`orders`, `membership_plans.price`, `products.price`, `order_items` are
`DECIMAL/NUMERIC`; the i64-cents contract exists only in transit, with
`$n::BIGINT / 100.0` float division on read/write
(`payments.rs:480-482`, `checkout.rs:293/343`). `payments.rs:1321`
accumulates `refund_amount` as a running float sum (rounding-drift
hazard). `DECIMAL(10,2)` is exact today but this violates the CLAUDE.md
money rule. **Fix:** migrate money columns to BIGINT cents; replace the
refund float accumulation with integer cents.

### P1-8 `get_subscription` doesn't expand `items` → epoch membership periods
`payments.rs:519-522` calls `get_subscription` (`stripe.rs:794-808`,
plain `GET` with no `expand[]=items`); the 2026 Stripe API moved period
timestamps into `items.data[0]`, so `get_current_period()` falls back to
top-level zeros → `current_period_start/end = 1970-01-01`, corrupting
renewal-reminder/grace logic. **Fix:** add
`.query(&[("expand[]","items.data")])`.

### P1-9 [V] CI does not enforce lint/clippy
Root `.github/workflows/ci.yml` has **no eslint gate**; backend clippy is
`continue-on-error: true` (advisory). `frontend/.github/workflows/ci.yml`
has a lint job but is **dead** (GitHub only reads root `.github/workflows/`,
and it uses `npm` in a `pnpm` repo). `pnpm check`, `vitest`, and
chromium e2e *are* enforced. **Fix:** add an eslint job to the root
workflow; drop `continue-on-error` on clippy; delete the dead
`frontend/.github/workflows/ci.yml`.

### P1-10 Backend "tests" exercise duplicated logic, not shipped code
The only CI-safe backend tests (`utils_test.rs` 17, `stripe_test.rs` 15)
test **copied modules** (`stripe_test.rs:4` `mod stripe { …duplicated… }`),
not `api/src/`. A real regression in `services/stripe.rs` signature
verification would not be caught. Admin integration tests
(`integration_tests.rs:162-266`) use a fake `"test_admin_token"`
(`common/mod.rs:106`) and cannot pass against real auth middleware
(non-functional/aspirational). No tests for: webhook idempotency under
concurrency, tx-rollback on Stripe failure, JWT blacklist on logout,
coupon double-increment fix, CMS versioning. **Fix:** test the real
modules; build a real-JWT admin test harness.

---

## 5. P2 — Medium

- **P2-A** Stripe idempotency keys are random per-call
  (`uuid::new_v4()` inline), not persisted per logical operation — does
  not protect against webhook-handler retry or user double-submit
  double-charge. Derive deterministically from order/subscription id.
- **P2-B** `create_refund` (`payments.rs:298-389`) is reachable by the
  order owner with arbitrary `amount`/`reason`; not rate-limited.
- **P2-C** `handle_invoice_paid`/`handle_payment_failed`/`retry_payment`
  swallow DB errors with `.ok();` (`payments.rs:1127,1183,1905`) →
  `200 OK` to Stripe, no retry, membership left wrong.
- **P2-D** DB errors swallowed with `unwrap_or_default()`:
  `admin_members.rs:1143`, `room_resources.rs:1825`, `posts.rs:900`
  (CLAUDE.md prohibits).
- **P2-E** Limited SSRF: `cms_webhooks.rs:109-155` posts to an
  admin-stored URL with no internal/link-local block (cloud-metadata
  pivot by a malicious/compromised admin).
- **P2-F** Admin RBAC enforced at the proxy layer for only ~42 of 93
  `routes/api/admin/**/+server.ts` (rest check token presence only;
  rely solely on Rust RBAC). `admin/coupons/+server.ts:39-41` returns
  empty `200` instead of `401` when unauthenticated (masks auth state).
- **P2-G** Service worker (`service-worker.ts:264-279`) caches
  authenticated SSR HTML (`/dashboard/*`) into the shared cache —
  shared-device cross-user leak. Skip caching protected prefixes.
- **P2-H** `developer`/superadmin-email login path bypasses
  email-verification (`auth.rs:842-905`); compounds P0-2.
- **P2-I** `migration 068` (money→BIGINT) is committed but
  operator-gated; `courses.price_cents`/`indicators.price_cents` stay
  INTEGER (i32 overflow at $21.4M on aggregates) until applied. Verify
  `_sqlx_migrations` table state vs live DB before deploy (re-running
  the non-idempotent `016`/`007` `DROP TABLE … CASCADE` migrations would
  be data-loss — see `MIGRATION_REPAIR_2026-04-28.md` Phase 3).
- **P2-J** 16 component-name collisions persist (3 copies of
  `RepeaterField`; `forms/RepeaterField.svelte` ≈ `forms/fields/RepeaterField.svelte`
  byte-identical). Wrong-import risk.

---

## 6. P3 — Hygiene / dead code

- Dead Rust module `courses` (`routes/mod.rs:17`, never mounted — the
  *only* truly unmounted module; the prior "~25" claim does **not** hold
  at this HEAD). Orphan `cms_scheduler::start_scheduler` (CMS scheduling
  is silently a no-op). `EnvFilter` defaults to `debug` in prod if
  `RUST_LOG` unset (`main.rs:31-33`). Unbounded list queries
  (`admin.rs:1889` campaigns, `users.rs` no pagination).
- `consent.rs` now upserts to the shared `settings` table (prior
  "in-memory only" claim is **stale**) — residual: process-local
  `OnceLock` fallback on DB error → multi-replica inconsistency; file
  TODO for a dedicated table still open.
- 7 zero-reference dead Svelte components incl. 3,548 LOC in dead
  `admin/CourseBuilder.svelte` + `admin/IndicatorBuilder.svelte`.
- 57 components > 1,500 LOC; worst are route pages
  (`admin/courses/create/+page.svelte` **4,892**, `admin/videos`
  **3,746**, `admin/trading-rooms/[slug]` **3,278**).
- 90 suppressed (not fixed) a11y warnings; 7 real
  `a11y_label_has_associated_control` (form fields), 3 deprecated
  `<svelte:component>`.
- `/signup` vs `/register` — two full divergent implementations.
- ≈36 MB / ~190 tracked dead files: `frontend/Implementation/` (31 M),
  `frontend/Do's/` (3.7 M), `frontend/retired/` + `retired/` (584 K),
  orphaned `frontend/e2e/` (236 K, not in `playwright.config.ts`
  testDir), `docs/forensics/` (136 K). `docs/audits/` = 56 files / 1.7 M.
- Stale "Fly.io" comments in `watchlist.ts:17`,
  `admin/coupons/+server.ts:11`. Deprecated `X-XSS-Protection` header
  (`hooks.server.ts:298`).

---

## 7. Verified-clean (independently confirmed solid)

- **Svelte 5 legacy patterns fully eradicated:** 0 shadow-state, 0
  `export let`, 0 `$:`, 0 `on:click`, 0 `createEventDispatcher`, 0
  `svelte/store` in components. CLAUDE.md compliance is real.
- **Backend-URL proxy contract holds repo-wide:** 129/129 backend
  proxies + catch-all + hooks + axum client use exactly
  `env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080'`.
  Zero hardcoded production backend URLs.
- **JWT:** HS256 pinned (not `default()`), access/refresh segregated, 7
  regression tests. Config boot-asserts JWT_SECRET length/placeholder &
  Stripe key prefixes in production. Blacklist fail-closed.
- **SQL injection:** every dynamic `format!()` builder reviewed across
  ~20 route files — only whitelisted columns / `$N` placeholders /
  validated LIMIT-OFFSET; all user values `.bind()`-ed. None injectable.
- **Critical-path transactions:** registration, developer bootstrap, the
  working `create_checkout` handler are properly `begin()`/`commit()`-wrapped
  (the gap is specifically the *webhook* handlers — P0-4).
- **Webhook security:** signature is constant-time HMAC-SHA256 over the
  raw unmodified body, fail-closed in prod, ±300 s replay window;
  receipt idempotent via `webhook_events(event_id)` UNIQUE; coupon
  increment is exactly-once. (The weakness is atomicity, P0-4, not the
  crypto.)
- **CORS** explicit origin list, no `allow_origin(Any)` with
  credentials. **CSP** nonce-mode, no `unsafe-inline`/`unsafe-eval` in
  `script-src`, `frame-ancestors 'none'`. **Cookies** HttpOnly +
  SameSite=Lax + Secure-in-prod. **OAuth** no token-in-URL (cookie on
  redirect), Apple ID-token signature + nonce + PKCE validated.
- **File upload** size + MIME allowlist + magic-byte + filename
  traversal rejection + UUID keys, all admin-gated. **No command
  injection** anywhere. **No secret values logged** (Config Debug
  redacts). IDOR scoping correct except H2/P0-7.
- No committed `node_modules`/`target`. Gate commands in CLAUDE.md all
  exist and resolve. CHANGELOG security claims spot-checked accurate.

---

## 8. Corrections to the 2026-05-10 audit

1. "**Auth gate on /admin is real**" — **overstated.** Server-side it
   only blocks unauthenticated users; the *role* gate is bypassable
   client-side JS (P0-3).
2. "**~25 unmounted backend route files**" — **incorrect at this HEAD.**
   Exactly one (`courses`).
3. "**consent.rs persistence is in-memory**" — **stale.** Now upserts to
   the `settings` table (residual OnceLock fallback only).
4. "**/api/checkout route works; the subscriptions-plan upgrade flow
   does not**" — the *customer* `/checkout` page is in fact broken via a
   different defect (P0-1, route mismatch); the backend handler is fine.

---

## 9. Prioritized backlog

**P0 (release-blocking):** P0-1 checkout route mismatch · P0-2 rotate &
purge committed superadmin creds · P0-3 server-side `/admin` role gate ·
P0-4 webhook transaction wrap · P0-5 subscription-plan price normalizer ·
P0-6 JSON-LD escaping · P0-7 course paywall enrollment check.

**P1:** rotate Google key + env-source it · per-user token-version
(JWT revocation) · trusted-proxy IP for rate limits · auth-gate
`coupons/validate` + `/init-db` + metrics · real or 501 admin analytics ·
delete/validate `set-session` · money→BIGINT cents · `expand=items` on
`get_subscription` · CI eslint+clippy enforcement · backend tests on real
modules.

**P2:** persist idempotency keys · refund authz/rate-limit · stop
swallowing webhook DB errors · webhook-URL SSRF guard · standardize admin
proxy RBAC · SW auth-HTML caching · email-verification bypass ·
apply/confirm migration 068 + `_sqlx_migrations` state · dedupe component
collisions.

**P3:** delete dead module/components/cruft (~36 MB) · split mega route
pages · burn down a11y suppressions · consolidate `/signup`+`/register` ·
prod log level · docs/audits sprawl.

---

## 10. Honest scope statement

770k LOC cannot be read line-by-line by any single reader in one pass.
Coverage was achieved by: running every available gate (which
transitively compiles/type-checks every file the build sees), six
parallel agents that each read their subsystem's source and traced its
critical paths, and hand-verification in the main thread of every
finding marked **[V]** (P0-1, P0-2, P0-3, P0-6, P0-7, P1-1, P1-9, plus
the prior-P0 checkout/idempotency status). Not covered: Playwright e2e
(no Docker stack), live migration replay (no DB), `cargo machete`/`deny`
(absent), per-component visual/a11y review, and `svelte-autofixer` MCP
(permission-denied — a real Svelte-5 autofix coverage gap to close on a
host where that tool is permitted).

---

## 11. Gate addendum (filled when long gates complete)

- `pnpm lint`: _pending_
- `cargo clippy --locked --all-targets -D warnings`: _pending_
- `cargo test --test utils_test --test stripe_test`: _pending_
</content>
</invoke>
