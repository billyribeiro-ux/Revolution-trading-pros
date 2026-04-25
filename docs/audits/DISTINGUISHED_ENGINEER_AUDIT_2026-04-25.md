# Distinguished Engineer Audit — Anti-Patterns, Tests, Dead Code, Perf, Security, A11y/SEO/Observability

**Auditor:** Claude (Opus 4.7) acting as a panel of distinguished-engineer reviewers
**Date:** 2026-04-25 · **Commit:** `52e64a3b2` (with one mid-audit fix to `setup.ts:80` documented in §0.2)
**Scope:** six parallel read-only sweeps + live verification of all four test gates.

This report is the third in the series. Read first:
1. [`PRODUCT_AND_AUTH_AUDIT_2026-04-25.md`](PRODUCT_AND_AUTH_AUDIT_2026-04-25.md) — auth, RBAC/ABAC, CRUD coverage, products, remote-functions plan
2. [`ADMIN_AND_CMS_AUDIT_2026-04-25.md`](ADMIN_AND_CMS_AUDIT_2026-04-25.md) — backend admin, CMS v2, frontend admin/dashboard, integration
3. **this doc** — anti-patterns, test gaps, dead code, perf, security hygiene, a11y/SEO/observability/i18n

Every finding cites `file:line`. The doc is long because the codebase is
large; skim §0 for the executive summary and use the per-area sections to
plan PRs.

---

## 0. Live evidence captured at audit time

### 0.1 Test-gate results

| Gate | Command | Result | Time |
|------|---------|--------|------|
| **Frontend typecheck** | `pnpm --filter revolution-svelte run check` | **8799 files / 0 errors / 0 warnings** | ~30s |
| **Frontend unit (vitest)** | `pnpm --filter revolution-svelte test:unit` | **1442 passed / 32 skipped / 0 errors / 0 unhandled** | 6.0s (post-cache) |
| **Frontend e2e (playwright chromium)** | `pnpm exec playwright test tests/e2e --project=chromium` | **85 passed / 8 skipped / 0 failed** | 29.5s |
| **Backend cargo check** | `cargo check` | clean compile | 1m 8s |
| **Backend cargo test --no-run** | `cargo test --no-run` | clean compile (5 test binaries built) | 1m 53s |
| **Backend cargo unit tests (no-DB)** | `cargo test --test utils_test --test stripe_test` | **17 passed / 0 failed** | 0.5s |
| **Backend integration / e2e** | `cargo test --test integration_tests` etc. | **NOT RUN** (Fly DB unreachable) | — |

The 8 e2e skips: 3 are `test.fixme` for the CMS toolbar regression (well
known, see prior audit); 5 are conditional skips because admin / accessibility
tests redirect to `/login` when there's no admin session in the test runner.

### 0.2 One regression introduced by the previous migration commit

While running gate 1 I found that `frontend/src/test/setup.ts:80` — the
canvas-context polyfill I added in commit `05acf3231` — fails type-checking
under TypeScript 6's stricter inference. Single-line fix; applied in this
audit pass (`setup.ts:80-106`). The runtime behavior was always correct
(vitest had been green); the gate-1 typecheck regression was new and now
clean again.

### 0.3 The audit findings, ranked

This is the canonical list. Every item is hyperlinked to its full section
below.

| # | Finding | Where | Severity |
|---|---------|-------|----------|
| 1 | **CMS editor toolbar broken** — `<button onclick={…addBlock(type)}>` doesn't fire after bits-ui v2 upgrade. Three e2e tests are `test.fixme` because of this. | [`frontend/src/routes/cms/editor/+page.svelte:113`](frontend/src/routes/cms/editor/+page.svelte#L113) | **Critical** |
| 2 | **Stripe Checkout-Session creation is a `// TODO` stub** — funnel can't take money via self-serve | `api/src/routes/subscriptions.rs:446` | **Critical** |
| 3 | **15+ `let _ = sqlx::query(…)`** in `cms_scheduling.rs` and `admin_courses.rs:1829` — schedule release logic silently fails, no audit trail | [§3.A2](#3-anti-patterns-rust-backend) | **High — data integrity** |
| 4 | **WebSocket has no JWT validation** — anyone with the URL subscribes to live alerts | `api/src/routes/websocket.rs:344` | **High — paid content leak** |
| 5 | **`/admin` frontend has no role gate** — confirmed in this sweep again | `frontend/src/routes/admin/+layout.ts:1-4` | **High — defense-in-depth** |
| 6 | **Favorites proxy reads wrong cookie** | `frontend/src/routes/api/favorites/+server.ts:32` | **High — feature broken in prod** |
| 7 | **4 unsanitized `{@html …}` components** — PostContentField, AccordionTabField, MobileResponsiveTable, VideoTranscript | [§7.B1](#7-security-hygiene) | **High — XSS** |
| 8 | **30+ static `gsap` imports** ship the full library on every page that touches the options-calculator routes | [§5](#5-performance) | **High — bundle bloat** |
| 9 | **N+1 INSERT in orders + course-section reorder + lesson reorder** | `orders.rs:917-931`, `courses_admin.rs:715-725, 898-910` | **High — perf** |
| 10 | **Test coverage ~23% measured by hand** — auth, OAuth, checkout, admin routes have **zero** integration tests | [§4](#4-test-coverage) | **High — regression risk** |
| 11 | **194 `as any` + 29 `@ts-ignore/expect-error`** | [§3.B1, B2](#3-anti-patterns-typescript--svelte-frontend) | **Medium** |
| 12 | **OAuth tokens travel in URL query string on callback** | `api/src/routes/oauth.rs:840-854` | Medium |
| 13 | **No transactions on multi-step admin mutations** (already documented in prior audit) | `admin.rs:958-1037` | Medium |
| 14 | **Mixed admin auth pattern** — `User+require_admin()` vs `AdminUser` extractor (already documented) | `admin.rs` vs `admin_*.rs` | Medium |
| 15 | **14 hardcoded fly.dev URLs** (already documented) | spread across frontend | Medium |
| 16 | **3 retry implementations** (already documented) | catch-all + Axum + admin.ts | Medium |
| 17 | **MFA dormant** — service complete, login never branches on `mfa_enabled` | `api/src/services/mfa.rs` | Medium |
| 18 | **No rate limit on register / forgot-password / reset-password endpoints** — only login is rate-limited | `api/src/routes/auth.rs` | Medium |
| 19 | **186 `console.log`** in frontend production code (some in `hooks.server.ts:157, 222, 233, 281, 300` leaking auth flow details) | [§3.B4](#3-anti-patterns-typescript--svelte-frontend) | Medium |
| 20 | **92 `<img>` without `alt`** out of 103 — 11% compliance | [§8.A](#8-accessibility-seo-observability-i18n) | Medium |
| 21 | **3,983 of 4,131 icons (~96%) lack `aria-hidden`** | [§8.A](#8-accessibility-seo-observability-i18n) | Medium |
| 22 | **No skip-link to main content** | `hooks.server.ts` lacks the markup | Medium |
| 23 | **i18n score 1/10** — no Paraglide, all hardcoded English, all dates/numbers `'en-US'`, prices `$`-only | [§8.D](#8-accessibility-seo-observability-i18n) | Low (depends on roadmap) |
| 24 | **2 unused frontend deps** — `vivus`, `lottie-web` | [§6.A](#6-dead-code-and-unused-dependencies) | Low |
| 25 | **1 doc drift** — `SETUP_GUIDE.md:75` still says `npm install` | `SETUP_GUIDE.md:75` | Low |
| 26 | **`#[allow(dead_code)]` × 13 + 3 commented-out modules** in routes/mod.rs | [§6.B](#6-dead-code-and-unused-dependencies) | Low |
| 27 | **No Lighthouse CI / no perf budget** | repo lacks both configs | Low |
| 28 | **`SITEMAP_CACHE` static `RwLock<HashMap>` with no TTL** — sitemap can serve stale forever | `api/src/routes/sitemap.rs:29-30` | Low |
| 29 | **`Regex::new(...)` per request** — should be `LazyLock` / `once_cell` | `api/src/routes/cms_seo.rs:351` | Low |
| 30 | **CSP allows `'unsafe-inline'` for scripts** — could tighten to nonces | `frontend/svelte.config.js:54-61`; backend `main.rs:189-190` | Low |

The first 10 are the worst by impact-per-hour-of-fix. Items 11-22 are the
"slow hum" — none of them block a release alone, together they're why the
codebase feels slippery.

---

## 1. Methodology

Six independent agents ran read-only sweeps in parallel:
- **Anti-patterns / code smells** (Rust + TS + repo hygiene)
- **Test coverage** (vitest + playwright + cargo)
- **Dead code / unused deps / doc drift**
- **Performance** (bundle composition, N+1, caching, observability)
- **Security hygiene** (input handling, secrets, headers, CORS, rate limit, audit log)
- **A11y / SEO / Observability / i18n**

All findings cite `file:line` from the actual code at commit `52e64a3b2`.
The full audit transcripts are recoverable from the conversation log; this
doc is the consolidated, deduped, severity-ranked output.

---

## 2. What's surprisingly good

Worth naming so we don't accidentally regress them:

- **Frontend typecheck is 0/0/0** — not a single error or warning across
  8799 files. The migration in `05acf3231` and the cleanup in
  this audit closed the last leak (canvas-context polyfill).
- **1442 vitest tests, 0 unhandled errors.** The CMS block components are
  particularly well-tested (23 block components × 50-100 tests each).
- **All `cargo` SQL is parameterized.** I sampled 5 unrelated route files
  and found zero string-interpolated SQL.
- **Argon2id password hashing** with bcrypt fallback for legacy users —
  rehashed on next login (`auth.rs:557-563`).
- **Indicator download flow** — short-lived tokens, IP-bound, max-downloads;
  the most thoughtful authz mechanism in the repo.
- **CMS data model** — 13 content types, 63 block types, revisions with
  rollback, scheduled releases, presets, datasources, audit log shaped
  for SOC 2 / GDPR.
- **CMS audit logging** — `services/cms_audit.rs` writes structured
  before/after JSON with IP, UA, actor email + role.
- **Strict security headers** — HSTS with preload, X-Frame-Options DENY,
  X-Content-Type-Options nosniff, Permissions-Policy with explicit denylist
  of camera/mic/geolocation/etc.
- **Argon2id for password reset tokens** — also for the 10 backup MFA codes.
- **Modal focus trap** is WCAG-2.1-AA-grade — Tab/Shift-Tab wrapping,
  return-focus-to-trigger, Escape, backdrop click, prefers-reduced-motion
  honored, 44×44 minimum touch targets.
- **SEOHead component** generates JSON-LD for Article/Product/Course/FAQ/
  HowTo/VideoObject + GEO + Speakable schema for voice search.
- **Connection pool sized at 50** (up from default 10), with `min=5` and
  30 s acquire timeout.
- **Redis L2 cache** with in-memory L1 fallback (10k entries) — explosive
  swings room content uses it.
- **Existing `.remote.ts` pattern** — explosive-swings dashboard is the
  template every other migration should copy.

---

## 3. Anti-patterns

### 3.A — Rust backend

#### A1. `unwrap_or_default()` in config loading (8 instances)

Config values fall back to empty strings on missing env vars instead of
failing at startup. R2 endpoints and credentials at
`api/src/config/mod.rs:146-148`; Meilisearch at
`config/mod.rs:201, 205, 213`; environment detection at
`api/src/utils/errors.rs:184`.

**Why it matters:** misconfigurations silently succeed; failures surface in
the wrong layer at runtime. Should `expect()` with a clear message and let
the process die at boot.

#### A2. `let _ = sqlx::query(…)` swallowing errors (15+ instances)

Critical mutations discard results:

- `api/src/routes/cms_scheduling.rs:495, 738, 805, 916, 1348, 1640, 1652, 1660, 1688, 1702, 1829, 1856, 1865`
- `api/src/routes/admin_courses.rs:1829`

**Why it matters:** scheduled-release transitions fail without a trace.
Audit trails become unreliable. **This is the single highest-impact backend
fix** — one helper that returns the result properly + a sweep through these
file:lines.

#### A3. `.unwrap()` in route handlers (7 instances)

- `api/src/routes/organization.rs:624, 708, 1032, 1121`
- `api/src/routes/connections.rs:1059, 1142`
- `api/src/domain/workflow/state_machine.rs:314`

**Why it matters:** any of these panics crashes the request worker. Rust
won't restart the worker per-request; under tokio, this aborts the task and
logs a stack trace. Should propagate via `?` or `.ok_or(StatusCode::…)?`.

#### A4. Hardcoded magic numbers (20+ pagination limits)

`.min(100)` literal across `products.rs:58, 520`, `orders.rs:141, 495`,
`cms_scheduling.rs:536, 956`, plus 10 more files.

#### A5. `format!()` for SQL where-clause assembly

Parameterized for values, but column names interpolated:

- `api/src/routes/room_resources.rs:1787-1791`
- `api/src/routes/media.rs:185, 1022`
- `api/src/routes/admin_popups.rs:96`

**Why it matters:** not exploitable today (column names are static), but
the next dev who reaches for "just one more dynamic filter" can introduce
an injection. Should use a query builder (sea-query, sqlx::QueryBuilder).

#### A6. RwLock held across `.await` (3 patterns)

- `api/src/cache/service.rs:420-426` — `write().await` held while
  `redis.delete().await` runs
- `api/src/cache/service.rs:460-469` — same shape
- `api/src/routes/sitemap.rs:29-30` — `static SITEMAP_CACHE: RwLock<HashMap>`
  with **no TTL**

**Why it matters:** under load, lock contention serializes requests behind
network I/O. The sitemap cache never expires, so editorial changes stay
hidden until restart.

#### A7. `#[allow(dead_code)]` annotations (13 instances)

`api/src/middleware/auth.rs`, `middleware/content_type.rs`, `models/job.rs`
(2), `models/membership.rs` (3), `queue/mod.rs` (2), `routes/search.rs`,
`routes/admin.rs`, `services/email.rs`, `services/search.rs`.

If they're truly unused, delete. If they're serialization-only, document.

#### A8. Pervasive `.clone()` (504 instances)

Hard to argue in isolation; shape is "passing owned `String`/`Vec<…>`
through layers when a borrow would do." Worth a single PR with a cargo-
clippy `clippy::clone_on_ref_ptr`/`clone_on_copy` pass and a rule sweep.

#### A9. Production TODOs in routes (20+ instances, the biggest by impact)

| Location | Note |
|----------|------|
| `routes/subscriptions.rs:446` | Stripe Checkout-Session creation |
| `routes/websocket.rs:344` | JWT validation on the WS handshake |
| `routes/health.rs:320` | "Re-enable when migration file is created" |
| `routes/newsletter.rs:251, 318` | "Actually send confirmation email" |
| `routes/media.rs:1174` | Malware scanning integration |
| `routes/admin_videos.rs:323-324, 366-367, 644-645, 662-663` | Trader/room metadata, weekly/monthly aggregates |
| `routes/mod.rs:40, 109` | `// TODO: Fix SQLx tuple decoding issues` (admin-indicators-enhanced disabled) |
| `routes/connections.rs:near GA4 placeholder` | GA4 measurement-ID placeholder marker |

#### A10. `Regex::new(...)` per request

`api/src/routes/cms_seo.rs:351` compiles a regex on every SEO-validate
call. Should be a `LazyLock<Regex>`.

### 3.B — TypeScript / Svelte frontend

#### B1. `as any` (194 instances)

Concentrated in test setup, browser-API polyfills, web-vitals casts,
payment integrations. Sample:
`/frontend/src/test/setup.ts:34, 41, 44, 70` (mocks; tolerable);
`/frontend/src/lib/utils/webVitals.ts:119-120, 145, 183, 212, 215-216`
(less tolerable — these are public API casts);
`/frontend/src/lib/stores/subscriptions.svelte.ts:377` (payment method).

#### B2. `@ts-ignore` / `@ts-expect-error` (29 instances)

A specific pattern stands out: **5 instances in
`routes/admin/blog/+page.svelte:76, 78, 80, 87, 89` are all suppressing
"write-only state" errors**, plus more at
`admin/indicators/+page.svelte:30, 94`,
`admin/products/+page.svelte:26`,
`InvalidatePositionModal.svelte:18`. That's a Svelte 5 reactivity bug
pattern (binding/derive misuse), not "external API has no types."

#### B3. Direct `fetch()` from client components (60+ instances)

Bypasses the proxy/auth/CSRF/rate-limit layer the rest of the system
relies on.

Sample call sites:
- `lib/options-calculator/components/growth/LeadCaptureModal.svelte:76` — `fetch('/api/newsletter/subscribe')`
- `lib/components/ClassDownloads.svelte:99` — `fetch(url, { credentials: 'include' })`
- `lib/components/PopupModal.svelte:601` — `fetch(currentPopup.formAction)`
- `lib/components/video/VideoTranscript.svelte:116`
- `lib/components/video/BunnyVideoPlayer.svelte:285`

…plus 55 more in forms, video, dashboard widgets.

#### B4. `console.log` in production code (186 instances)

Including auth flow details:
- `hooks.server.ts:157, 222, 233, 281, 300` — token issues and refresh
  failures with full token IDs visible in browser DevTools.

#### B5. `onMount` for data fetching (50+ instances)

Should be `load` / `query` / `prerender`. Sample:
- `RateLimitIndicator.svelte:143`
- `RelatedVideos.svelte:72`
- `FormCollaborators.svelte:207`
- `WatchHistory.svelte:72`
- `RoomNotification.svelte:230`

…plus 45 more.

### 3.C — Repo hygiene

- **Inconsistent naming** — `userId` vs `user_id` vs `uid`; `day-trading-room`
  vs `trading-room` route slugs.
- **Three commented-out routes** in `api/src/routes/mod.rs:16, 21, 40` —
  one of which (`indicators_admin`) is the disabled file-upload pipeline.
- **35+ frontend TODO/FIXME**, sample:
  - `routes/indicators/[id]/+page.svelte:62` — "TODO: Replace with actual API call"
  - `routes/admin/members/segments/+page.svelte:389, 516, 522` — 3 TODOs in admin UI
  - `routes/dashboard/+page.svelte:61, 67, 73` — three trading-room links wired to `href: '#'`
  - `routes/dashboard/[room_slug]/video/[slug]/+page.svelte:32, 37, 42` — three more `href: '#'`
  - `routes/dashboard/account/view-subscription/[id]/+page.server.ts:60, 73` — "TODO: Call actual API when backend is ready"

---

## 4. Test coverage

### 4.1 Inventory

| Layer | Files | Test count | Pass status |
|-------|-------|-----------|-------------|
| Vitest unit/component | 34 | 1442 (32 skipped) | ✅ all green |
| Playwright frontend e2e | 10 specs | ~91 | ✅ 85 / 8 skipped / 0 failed |
| Playwright blog-editor e2e | 6 specs | ~279 | ✅ |
| Cargo (Rust) | 24 files | 99 functions | partial — 17 utils+stripe pass; integration tests need DB |

### 4.2 What's NOT tested — the gap matrix

| Surface | Lines | Tests | Status |
|---------|-------|-------|--------|
| `routes/auth.rs` (login/register/refresh/logout/reset-password/verify-email) | 1103 | 0 | 🟥 **CRITICAL** |
| `routes/oauth.rs` (Google + Apple flows) | 1100 | 0 | 🟥 |
| `routes/checkout.rs` | 536 | 0 | 🟥 |
| `routes/admin.rs` | 2892 | 0 | 🟥 |
| `routes/admin_courses.rs` | 1211 | 0 | 🟥 |
| `routes/admin_indicators.rs` | 1302 | 0 | 🟥 |
| `routes/admin_members.rs` | 2680 | 0 | 🟥 |
| `routes/admin_videos.rs` | 1300 | 0 | 🟥 |
| `routes/admin_member_management.rs` | 1700 | 0 | 🟥 |
| `middleware/admin.rs` (AdminUser extractor) | — | 0 | 🟥 |
| `middleware/auth.rs` (JWT verify + blacklist) | — | 0 | 🟥 |
| `services/mfa.rs` (TOTP + backup codes) | — | 0 | 🟥 |
| `services/cms_audit.rs` | — | 0 | 🟥 |
| `services/cms_workflow.rs` | — | 0 | 🟥 |
| `services/cms_scheduler.rs` | — | 0 | 🟥 |
| `routes/cms_delivery.rs` | — | 0 | 🟥 |
| `routes/cms_revisions.rs` | — | 0 | 🟥 |
| `LoginForm.svelte` / Register / PasswordReset | — | 0 | 🟥 |
| `hooks.server.ts` (auth gate) | — | 0 | 🟥 |
| 12 auth proxy `+server.ts` files | — | 0 | 🟥 |
| 27 admin pages + 15 dashboard pages (component-level) | — | 0 | 🟥 |
| `data.remote.ts` / `commands.remote.ts` (explosive-swings) | — | 0 | 🟥 |

### 4.3 Critical user flows — coverage table

| Flow | E2E test? |
|------|-----------|
| Sign up | ❌ |
| Login (email/password) | ⚠ smoke loads `/login`, doesn't submit |
| OAuth (Google) | ❌ |
| OAuth (Apple) | ❌ |
| Logout | ❌ |
| Password reset | ❌ |
| Subscribe / checkout / payment | ❌ |
| Buy an indicator | ❌ |
| Enroll in course | ❌ |
| Watch course video | ❌ |
| Receive trade alert | ✅ (explosive-swings/admin.spec.ts — fixtures-mocked) |
| Add to favorites / watchlist | ❌ |
| Admin grants membership | ⚠ partial (fixtures only) |
| Admin publishes CMS content | ❌ |

### 4.4 Estimated overall coverage

By LOC (rough): **~23%**. Heavily biased toward CMS block-component
rendering (~1400 of the 1442 vitest tests) and the explosive-swings
dashboard (most of the e2e tests).

### 4.5 Test quality issues

- The explosive-swings e2e fleet **mocks every API call** — validates UI
  shape, not contract.
- Smoke tests assert page-load only, no form submissions or error paths.
- Block-renderer component tests sometimes mock the renderer itself,
  reducing them to "the mock returned what we told it to."
- The 3 quarantined block-editor tests are correctly tagged `test.fixme` —
  they're not weak, they're blocked on the toolbar fix.

### 4.6 Top 10 worst gaps to close (by blast radius)

1. `routes/auth.rs` HTTP login/register/refresh — the entire user lifecycle
2. `routes/oauth.rs` Google + Apple — auth flows that handle 3rd-party tokens
3. `routes/admin.rs` — 2892 LOC of unaudited admin CRUD
4. `routes/checkout.rs` — revenue path
5. `routes/admin_courses.rs` enrollment — core product
6. `routes/admin_members.rs` membership grant/revoke — billing-adjacent
7. `services/cms_workflow.rs` — publishing state machine
8. `services/cms_audit.rs` — compliance trail
9. `LoginForm.svelte` + auth proxies — frontend auth UI end-to-end
10. The 27 admin pages — none have a single component test

---

## 5. Performance

### 5.1 Bundle composition (heavy deps that ship to browser)

| Dep | Approx KB (min+gzip) | Static import? | Notes |
|-----|----------------------|----------------|-------|
| `gsap` | ~370 | **30+ static imports** across `options-calculator/*` | Worst offender |
| `three` | ~500 (with @threlte) | guarded by dynamic Scene3D wrapper | OK |
| `lightweight-charts` | ~200 | dashboard pages | acceptable on dashboard |
| `d3` + 7 sub-modules | ~150 | 25 import sites | spread over admin analytics |
| `html2canvas` | ~120 | dynamic via options-calculator export | OK |
| `lottie-web` | ~80 | **unused** (dep declared, no imports) | Drop |
| `vivus` | ~20 | **unused** | Drop |
| `typed.js` | ~35 | static in auth/TypedHeadline | OK |
| `embla-carousel` + `-svelte` | small | ✅ used | OK |
| `canvas-confetti` | ~8 | dynamic in register | OK |
| `blurhash` | ~2 | image placeholders | OK |
| `@anthropic-ai/sdk` | server-only | ✓ confirmed isolated to `api/cms/ai/generate/+server.ts` | OK |

### 5.2 N+1 patterns (top 10)

1. **`api/src/routes/orders.rs:917-931`** — INSERT per item in loop. Should
   batch with multi-row VALUES.
2. **`api/src/routes/courses_admin.rs:715-725`** — UPDATE per item in
   reorder-sections loop.
3. **`api/src/routes/courses_admin.rs:898-910`** — same shape for lessons.
4. **`api/src/routes/orders.rs:910-915`** — separate fetch for user-id then
   insert products.
5. **`api/src/routes/courses_admin.rs:1550-1600`** — `format!()`-built
   SQL for batch schema changes.
6. **`api/src/routes/orders.rs:500-530`** — dynamic filter SQL via
   `format!()` (parameterized values, but the pattern is risky).

The rest of the candidates I scanned are not strictly N+1 but adjacent
(separate metadata fetches that could be a single JOIN).

### 5.3 Caching

- ✅ Connection pool: max 50 / min 5 / 30 s acquire timeout.
- ✅ Redis L2 + in-memory L1 (10k cap) for explosive-swings room content.
- ✅ HTTP cache headers on read-only public routes (`/live-trading-rooms`,
  feeds, sitemaps; max-age=300/3600 with stale-while-revalidate).
- ⚠ No static-asset cache headers in a `_headers` file (Cloudflare Pages).
  Static assets (JS/CSS/images) inherit defaults.
- ⚠ Sitemap cache (`SITEMAP_CACHE`) has no TTL — see [§3.A6](#a6-rwlock-held-across-await-3-patterns).

### 5.4 RAF / animation cleanup

- ✅ `lib/utils/performance.ts:261-265` — RAF loop stores `rafId` for
  cleanup.
- ⚠ `Scene3D.svelte`, `Surface3D.svelte` — partial cleanup
  (`Surface3D.svelte:229` is `cleanupPromise.then(cleanup => cleanup?.())`,
  which swallows the cleanup error).
- ⚠ GSAP timeline cleanup not visible in most option-calculator components.
  This is the same root cause as the
  `waitForLoadState('networkidle')` regression we fixed in `tests/e2e/smoke/homepage.spec.ts`.

### 5.5 Observability gaps for perf

- ❌ No Lighthouse CI config (`.lighthouserc.*`) anywhere in the repo.
- ❌ No declared performance budget.
- ⚠ `web-vitals` is installed but its reports plumb only to GTM (`hooks.client.ts`),
  not to a dashboard you can alert on.

### 5.6 Recommended budgets (next sprint)

- Homepage / login initial JS: **≤ 200 KB gzip** (currently estimated 250-300 KB)
- Dashboard initial JS: **≤ 350 KB gzip**
- Options Calculator: **≤ 180 KB gzip** if we dynamic-import GSAP
- LCP ≤ 2.5 s, CLS ≤ 0.1, INP ≤ 200 ms

---

## 6. Dead code and unused dependencies

### 6.A Unused frontend deps

Confirmed by import-grep across `frontend/src/`:

- `vivus` — declared, **zero imports**.
- `lottie-web` — declared, **zero imports** (types still referenced; safe to drop).

### 6.B Rust dead-code hygiene

- 13 `#[allow(dead_code)]` annotations (see [§3.A7](#a7-allowdead_code-annotations-13-instances)).
- 3 commented-out `pub mod` declarations in `api/src/routes/mod.rs:16, 21, 40`.

All declared modules ARE wired into `api_router()` in `lib.rs` — no orphan
routers.

### 6.C Frontend orphans

- `frontend/retired/` is a documented archive with `RETIREMENT_MANIFEST.md`
  listing 110+ retired components — intentional, not dead code.
- All examined `$lib/` exports are imported somewhere.
- 309 active routes, 58 `+page.server.ts` files, all reachable.

### 6.D Doc drift

- **`SETUP_GUIDE.md:75`** — `npm install -g wrangler` should be
  `pnpm add -g wrangler`.
- All other major docs (`DEPLOYMENT_GUIDE.md`, `DEPENDENCY_AUDIT_REPORT.md`,
  `CLAUDE.md`, etc.) are accurate at this commit.

---

## 7. Security hygiene

### 7.A Re-confirmed open issues from prior audits

All still open at `52e64a3b2`:
- WebSocket no JWT validation (`websocket.rs:344`)
- OAuth tokens via URL query string (`oauth.rs:840-854`)
- Redis blacklist fails open (`middleware/auth.rs:60+`)
- MFA dormant (no branch on `mfa_enabled` in `login`)
- `/admin` frontend has no role gate (`admin/+layout.ts:1-4`)
- Favorites proxy reads wrong cookie (`api/favorites/+server.ts:32`)

### 7.B New findings

#### B1. Unsanitized `{@html …}` in 4 components — XSS risk

| Component | What renders | Sanitized? |
|-----------|-------------|------------|
| `PopupDisplay.svelte` | popup content | ✅ via `sanitizePopupContent` |
| `VideoEmbed.svelte` | overlay markup | ✅ via `sanitizeVideoOverlay` |
| `VideoTranscript.svelte` | search-highlighted transcript | ⚠ via custom `highlightSearch` formatter — verify it escapes attacker input |
| `PostContentField.svelte` | post body | 🟥 `{@html content || …}` raw |
| `AccordionTabField.svelte` | section content | 🟥 `{@html section.content}` raw |
| `MobileResponsiveTable.svelte` | cell value | 🟥 `{@html getValue(...)}` raw |
| `SEOHead.svelte` | JSON-LD `<script>` | ✅ generated server-side |
| `Breadcrumbs.svelte` | self-built `<scr+ipt>` | ✅ self-generated |

Three of these (PostContentField, AccordionTabField, MobileResponsiveTable)
render content that could be user-controlled with no sanitization —
classic stored-XSS shape.

#### B2. Rate limiting incomplete

`services/rate_limit.rs` exists and is sophisticated (token bucket, L0
Redis → L1 in-memory → L2 DB). It's only applied to **login**.

Missing on:
- `register` — spam / enumeration
- `forgot-password` — brute force email enumeration
- `reset-password` — token brute force
- `mfa/verify` — TOTP brute force (when MFA gets wired)
- All admin write endpoints — no per-admin rate limiting

#### B3. CSP allows `unsafe-inline`

`frontend/svelte.config.js:54-61` and backend `main.rs:189-190`. Acceptable
for an SPA at this maturity, but the next hardening pass should move to
nonces.

#### B4. CORS allows credentials globally

`api/src/main.rs:113-162` — `allow_credentials(true)` against the env
allowlist. Correct config; just want to be sure the allowlist stays
minimal.

#### B5. No CSRF tokens on form-style mutations

The proxy pattern + `SameSite=Lax` cookies covers most cases, but the 60+
direct-fetch components in [§3.B3](#b3-direct-fetch-from-client-components-60-instances)
expose us to CSRF on any handler that takes form-encoded bodies.

### 7.C Secrets / logging hygiene

- ✅ No hardcoded `sk_live_` / `pk_live_` / Bearer tokens in tracked files.
- ✅ All env reads via `std::env::var()`.
- ✅ Auth audit logs use the right shape:
  `event = "verification_email_sent", token_expires = "24_hours"` — never
  the actual token.
- ⚠ `hooks.server.ts:157, 222, 233, 281, 300` — `console.log` of token-
  refresh decisions; in dev this is fine, in prod browsers' DevTools tabs it
  leaks signals.

### 7.D Top 15 security concerns (severity-ranked)

| # | Issue | Where | Severity |
|---|-------|-------|----------|
| 1 | 4 unsanitized `{@html}` components | §7.B1 | High |
| 2 | OAuth tokens via URL | `oauth.rs:840-854` | High |
| 3 | WebSocket no JWT | `websocket.rs:344` | High |
| 4 | `/admin` no frontend role gate | `admin/+layout.ts` | Medium |
| 5 | MFA dormant | `services/mfa.rs` (not called) | Medium |
| 6 | Redis blacklist fails open | `middleware/auth.rs` | Medium |
| 7 | No rate limit on password-reset | `routes/auth.rs` | Medium |
| 8 | No rate limit on register | `routes/auth.rs` | Medium |
| 9 | CSP `unsafe-inline` for scripts | both configs | Low |
| 10 | Dual auth pattern in admin | `admin.rs` vs `admin_*.rs` | Low |
| 11 | Password-rule UX | `auth.rs:80` | Low |
| 12 | CORS credentials globally allowed | `main.rs:161` | Low |
| 13 | Bootstrap CLI uses raw process::Command | `bin/bootstrap_dev.rs:167` | Low (deploy-only) |
| 14 | console.log of auth flow in hooks | `hooks.server.ts` | Low |
| 15 | No Content-Encoding on large CSV exports | `routes/export.rs` | Very Low (perf, not sec) |

### 7.E A 1-day hardening sprint

1. **WebSocket JWT validation** (1 h) — `websocket.rs:344`. Reuse the
   existing `verify_jwt`. Reject invalid tokens with 401 at handshake.
2. **Rate limit register + reset-password** (30 min) — call the existing
   `rate_limit_service` with a 5-attempt-per-24h-per-email policy.
3. **Sanitize the 3 `{@html}` components** (30 min) — wrap content with
   `sanitizeHtml` from `lib/utils/sanitization.ts`.
4. **Frontend `/admin` role gate** (15 min) — new `admin/+layout.server.ts`
   that redirects to `/login` when `event.locals.user?.is_admin !== true`.
5. **Document password requirements** in the register UI (10 min).

**Total ≈ 2.5 hours. Closes 4 of the top 7 issues.**

---

## 8. Accessibility, SEO, observability, i18n

### 8.A Accessibility — score 6.5/10

- **`<img>` alt text:** 92 of 103 missing alt → 11% compliance. Worst
  offenders: `WatermarkOverlay.svelte`, `BlurHashImage.svelte`,
  `BunnyVideoPlayer.svelte`, `TraderHeader.svelte`, `ResourceCard.svelte`,
  many admin form image inputs.
- **Icons:** ~4131 icon/SVG elements; only 148 have `aria-hidden="true"`
  (3.6%). Most are decorative; should be marked.
- **Modal:** WCAG-grade focus trap ✅.
- **Skip link:** ❌ none in the markup.
- **Live regions:** ❌ no `aria-live` for toasts or form errors.
- **Reduced motion:** Modal honors `prefers-reduced-motion`; GSAP
  scroll-triggers not audited but likely don't.
- **Keyboard nav:** custom dropdowns in options-calculator
  (StrategyPresets, TickerSearch) need arrow-key support.

### 8.B SEO — score 7.5/10

- **`SEOHead.svelte`** is excellent — generates Article / Product /
  Course / FAQ / HowTo / VideoObject + GEO + Speakable schema for voice.
- **`robots.txt`** at `frontend/src/routes/robots.txt/+server.ts` allows
  blog/courses/indicators/alerts/live-trading-rooms; disallows admin /
  api / checkout; includes AI-crawler rules.
- **Sitemap:** `super-sitemap` v1.0.12 generates `/sitemap.xml` +
  `/video-sitemap.xml` + `/news-sitemap.xml`.
- **Canonical URLs:** auto-generated from path — risk of mis-canonicalizing
  intentional duplicates.
- **Per-page metadata:** mostly present, but
  `routes/indicators/[id]/+page.svelte` has hard-coded title and missing
  dynamic description.

### 8.C Observability — score 6/10

- ✅ **Sentry** initialized at `lib/monitoring/sentry.ts` (DSN from
  `VITE_SENTRY_DSN`, prod-only, 1% session replay normal / 100% on error,
  filters NetworkError + AbortError, `setUser` / `clearUser` hooks).
- ✅ **Error boundaries** — `lib/components/patterns/ErrorBoundary.svelte`
  uses `<svelte:boundary>`; nested `BlockErrorBoundary.svelte` for the CMS.
- ✅ **Health endpoint** — `/health` simple + `/health/detailed` checks DB
  + Redis + storage with latency.
- ✅ **Web Vitals** — LCP, CLS, INP tracked in `hooks.client.ts`, reported to GTM.
- ✅ **Server-Timing** header — basic per-request total duration.
- ⚠ **Frontend logger** is no-op in prod (good for security, bad for
  diagnostics — 1218 `console.*` calls go nowhere in prod).
- ⚠ **No distributed tracing** — no `X-Request-ID` propagation.
- ⚠ **No cache hit/miss metrics** exported (the data exists in
  `cache/service.rs` but isn't surfaced).
- ⚠ **No Lighthouse CI**.

### 8.D Internationalization — score 1/10

- ❌ Paraglide / Inlang **not installed**. No `@inlang/*` in package.json.
- ❌ ~95% of UI strings hardcoded in English.
- ⚠ `Intl.DateTimeFormat` and `Intl.NumberFormat` are used (good!) but
  every call hardcodes `'en-US'`.
- ❌ Currency hardcoded to USD with `$` symbol literal.
- ✅ The infra to *add* i18n (locale-aware formatters, SEOHead supports
  `ogAlternateLocales`) is present — it's just not wired.

### 8.E Quick wins per dimension

**A11y (top 5):**
1. Add `alt` to 92 images (~3 h)
2. Add `aria-hidden="true"` to ~3900 decorative icons in a sweep (~2 h)
3. Add skip-to-main-content link in `+layout.svelte` (30 min)
4. Arrow-key nav on StrategyPresets + TickerSearch (1 h)
5. `aria-live` on toast container + form-error containers (1 h)

**SEO (top 5):**
1. Populate dynamic descriptions for indicators / courses / blog posts (2 h)
2. Audit title pattern across 10 public routes (1 h)
3. Validate Article / Course schema with Google Rich Results test (2 h)
4. Add `hreflang` placeholder when i18n lands (1 h)
5. Verify `super-sitemap` regenerates after content changes (30 min)

**Observability (top 5):**
1. Propagate `X-Request-ID` from frontend → backend (1 h)
2. Sentry breadcrumbs for nav / form submit / click (1 h)
3. Surface cache hit/miss metrics on Prometheus endpoint (1 h)
4. SQLx slow-query logging (>100 ms) (1 h)
5. Alert on web-vitals regression beyond budget (2 h)

**i18n (top 5):**
1. Install Paraglide (2 h)
2. Extract 20 highest-traffic strings to messages (2 h)
3. Make formatters locale-aware (1 h)
4. Add locale selector in header (1 h)
5. Pseudo-localization test pass (30 min)

---

## 9. The remediation backlog

Combining this audit with the prior two, in priority order:

### Tier 0 — blocking
1. **Restore Fly Postgres** (the DB outage from §7 of the prior audit). Nothing matters until login works.
2. **Fix the CMS toolbar** (`cms/editor/+page.svelte:113`) — the headline feature can't author content.
3. **Wire Stripe Checkout-Session creation** (`subscriptions.rs:446`) — revenue blocker.

### Tier 1 — under a day each, big payoff
4. Frontend `/admin` role gate (`admin/+layout.server.ts`)
5. Favorites proxy cookie fix (`api/favorites/+server.ts:32`)
6. WebSocket JWT validation (`websocket.rs:344`)
7. Sanitize the 3 unsanitized `{@html}` components
8. Rate limit register + forgot-password + reset-password
9. The `setup.ts` polyfill regression I introduced and fixed mid-audit (already done; landing in this commit)
10. Centralize the 14 hardcoded fly.dev URLs in `lib/config/api.ts`
11. Unify env-var precedence between catch-all proxy and Axum client
12. Remove unused `vivus` and `lottie-web` deps
13. Fix `SETUP_GUIDE.md:75` doc drift

### Tier 2 — a week
14. Wrap multi-step admin mutations in transactions (`grant_membership`, `bulk_assign_tags`, role updates)
15. Migrate `admin.rs` to the `AdminUser` extractor uniformly
16. Activate MFA at login (the service is complete)
17. Fix the `let _ = sqlx::query(…)` swallowing in `cms_scheduling.rs` + `admin_courses.rs:1829`
18. Replace 7 handler-level `.unwrap()` with `?` propagation
19. Migrate the 5 dashboard `+page.server.ts` files to `.remote.ts`
20. Fix the two correctness issues in the existing `.remote.ts` files (hardcoded refresh keys; unawaited `.refresh()`)
21. Move `SITEMAP_CACHE` off `RwLock<HashMap>`-no-TTL to a proper TTL'd cache
22. Move `cms_seo.rs:351` regex to `LazyLock`
23. Add CMS audit coverage for admin user-management mutations
24. Add `aria-hidden` to ~3900 decorative icons (sweep)
25. Add `alt` to 92 images
26. Fix the 30+ static GSAP imports — dynamic-import per page

### Tier 3 — a month
27. Test coverage for `routes/auth.rs`, `routes/oauth.rs`, `routes/checkout.rs`, `services/mfa.rs`, `services/cms_workflow.rs`
28. E2E specs for sign-up, login, OAuth, logout, password reset, subscribe, indicator purchase, course enrollment
29. Lighthouse CI + perf budget
30. Frontend admin/dashboard component migration to destructured `$bindable()` (the legacy `let props = $props()` shadow-state)
31. CMS editor UX: autosave, undo/redo, drag-reorder, AI panel, preset picker
32. Distributed tracing (X-Request-ID propagation)
33. Tighten CSP from `unsafe-inline` to nonces
34. i18n with Paraglide

### Tier 4 — when ready
35. SuperAdmin impersonation real JWT (currently placeholder string at `admin.rs:1508-1510`)
36. N+1 fixes (orders.rs:917, courses_admin.rs:715, 898)
37. Trading-rooms admin CRUD UI (model exists; no editor)
38. User profile self-service (PUT /users/:id, change-password, avatar)
39. SuperAdmin email allowlist consistency between back and front

---

## Appendix — what was verified live during this audit

- ✅ `pnpm --filter revolution-svelte run check` — clean, after fixing one
  TS error I introduced in commit `05acf3231`.
- ✅ `pnpm --filter revolution-svelte test:unit` — 1442/1442.
- ✅ `pnpm exec playwright test tests/e2e --project=chromium` — 85 pass /
  8 skipped / 0 failed.
- ✅ `cargo check` — clean compile of `revolution-api`.
- ✅ `cargo test --no-run` — all 5 test binaries compile.
- ✅ `cargo test --test utils_test --test stripe_test` — 17/17 (no DB
  needed: utils + stripe webhook signatures).
- ⏸ `cargo test --test integration_tests` — not attempted; needs the Fly
  Postgres DB.
- ⏸ `pnpm audit` — not run (would surface dependency vulns; recommended
  next-step).

The audit transcripts (six independent sweep agents) are recoverable from
the conversation log if any single claim needs re-verification.
