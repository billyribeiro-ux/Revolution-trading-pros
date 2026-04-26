# Comprehensive Full-Stack Audit Report

**Repo:** Revolution Trading Pros
**Date:** 2026-04-26
**Scope:** Frontend (SvelteKit 2.x + Svelte 5 + Tailwind v4) + Backend (Rust / Axum) + Contract
**Methodology:** 14 parallel sub-agents, evidence-based, file:line citations
**Sub-agent status at consolidation:** 13/14 complete (A11y agent still running — its findings are appended in §15 once available)

---

## Executive Summary

### Total Issues by Severity

| Severity | Count | Notes |
|---|---:|---|
| **Blocker** | **24** | Production auth broken (14 admin proxies wrong cookie), 2 forbidden Svelte 5 cascade time-bombs, 17 silent DB error swallows in CRM hot paths, frontend orphan API clients (boards/behavior — no backend) |
| **Major** | 88 | Outdated `axum 0.7`/`reqwest 0.11` (4 ignored CVEs), 351 hex/rgb in tokens, 213 off-scale breakpoints, dead code clusters (53 unimported lib files, 8–15k LOC), 17 production `.unwrap_or_default()` on Result, missing `handleError` in `hooks.server.ts`, no `+error.svelte` in 51 route groups, prod-shipped Vite dev plugins, CSP `'unsafe-inline'` defeats nonces |
| **Minor** | 230+ | 219 unformatted files, 17 ESLint errors, ~767 non-logical CSS properties, 1755 fixed-px font-sizes, 17 stub TODO breadcrumbs, 14 unused `errors.rs` helpers, 2 deprecated DB-types in lockfile |
| **Info** | 60+ | 32 skipped Vitest tests, 23 TODO/FIXME markers, 18 backend TODO markers, 6 dual-version dep installs, etc. |

### Frontend vs Backend vs Contract

| Domain | Issue Count | Highest Severity |
|---|---:|---|
| Frontend | ~210 | Blocker (Svelte 5 cascade time-bombs + auth proxy cookie name drift) |
| Backend | ~95 | Blocker (silent DB error swallows in CRM hot paths) |
| Contract | ~30 | Blocker (14 wrong-cookie proxies → 401 from backend; orphan API clients with no backend) |

### Top 5 Highest-Impact Problems

1. **14 admin/member SvelteKit proxies use wrong cookie name** (`auth_token`/`access_token`/`session` instead of `rtp_access_token`) — admin course mgmt, course player/downloads, favorites, bunny uploads ALL silently send no Bearer → backend returns 401 → users see empty data. Most painful production bug because it's invisible: no error is surfaced.
2. **2 unfixed `effect_update_depth_exceeded` time-bombs** at `routes/signup/+page.svelte:25-29` and `routes/account/sessions/+page.svelte:33-42` — same legacy `$store` autosubscribe inside `$effect` pattern that just blew up at `admin/+layout.svelte`. Will fire whenever signup completes or sessions page loads.
3. **17 silent DB error swallows in `routes/crm.rs`** plus 14 in `routes/courses_admin.rs` — `.fetch_all().await.unwrap_or_default()` returns empty data with HTTP 200 on Postgres failure. Production failures are invisible: wrong CRM data, empty course lists, empty product lists, no log entries.
4. **Frontend has complete API client surfaces with no backend at all**: `lib/api/boards.ts` (30+ endpoints under `/admin/boards`), `lib/api/behavior.ts`, `lib/api/bing-seo.ts`, `lib/api/bannedEmails.ts`, `lib/api/trading-room-sso.ts`. Pages are reachable and call these clients; every call 404s.
5. **Production builds ship dev tooling**: `svelteInspector` (visible toggle button) and `vite-plugin-devtools-json` (exposes `/.well-known/appspecific/com.chrome.devtools.json` to anyone with DevTools). CSP `'unsafe-inline'` in `script-src` defeats SvelteKit's nonce mechanism. `Dockerfile` uses `rust:latest` and `debian:trixie-slim` (testing channel).

### Overall Codebase Health

The codebase compiles cleanly on both stacks (`pnpm check` 0/0/0 across 8,799 files; `cargo check`, `cargo clippy -D warnings`, `cargo fmt --check` all green). Type and lint discipline at the surface level is in good shape. The user is also fully Svelte-5-runes compliant — zero `export let`, zero `$:`, zero `on:` directives, zero `<slot>`, zero `createEventDispatcher`. That said, the **production-correctness** layer beneath the typecheck has serious holes: silent error swallowing in DB hot paths, broken auth on multiple admin proxies, orphan frontend code calling non-existent backends, two forbidden reactivity loops still in code, an entire test infrastructure (115 integration tests) that can't run because the local Postgres `test` role isn't provisioned, and pervasive PE7 standard violations in CSS (213 off-scale breakpoints, 767 non-logical properties, 1,755 fixed-px font sizes). The codebase is shippable today only because the broken paths fail silently — the moment QA actually tests admin flows or signup, everything cascades.

---

## Findings by Domain

### 1. Type-Safety (Frontend)

#### Type-Safety — Finding 1
- **Severity:** info
- **Evidence:** `pnpm check` reports `COMPLETED 8799 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS`; `tsc --noEmit` exit code 0 with no output.
- **Root cause:** Codebase is fully type-clean.
- **Why it matters:** Establishes that all subsequent findings are real issues, not type-noise.

---

### 2. Lint (Frontend)

#### Lint — Finding 1: Parsing errors block ESLint on 2 files
- **File:** `src/lib/seo/Seo.svelte:129:45`, `src/routes/tools/options-calculator/+page.svelte:38:44`
- **Severity:** major
- **Evidence:** `Parsing error: Unexpected keyword or identifier`
- **Root cause:** Svelte 5 syntax not handled by current ESLint Svelte plugin version.
- **Why it matters:** ESLint cannot inspect these files at all — any lint regression goes undetected.

#### Lint — Finding 2: 7 `no-useless-assignment` errors
- **Files:** `src/lib/api/enterprise/csrf.ts:226`, `src/lib/components/blog/BlockEditor/BlockEditor.svelte:813,814`, `src/lib/components/blog/BlockEditor/SEOAnalyzer.svelte:342`, `src/lib/components/forms/FormEmbed.svelte:152`, `src/lib/options-calculator/engine/implied-volatility.ts:23`, `src/lib/utils/gravatar.ts:106`
- **Severity:** minor
- **Root cause:** Variables reassigned but unused after.

#### Lint — Finding 3: 5 `preserve-caught-error` errors
- **Files:** `src/lib/api/forms.ts:578`, `src/lib/components/forms/fields/CalculatedField.svelte:206,208`, `src/lib/stores/connections.svelte.ts:235`, `src/routes/blog/+page.svelte:119`
- **Severity:** minor
- **Root cause:** `try/catch` blocks swallow the original error context.

#### Lint — Finding 4: 219 unformatted files (Prettier)
- **Severity:** minor
- **Evidence:** Entire `src/lib/options-calculator/`, `src/lib/seo/`, multiple route files, all shadcn UI components.
- **Root cause:** No CI gate forcing prettier, options-calculator subtree imported without format pass.

#### Lint — Finding 5: ESLint config disables strict rules contradicting `tsconfig.json strict: true`
- **File:** `frontend/eslint.config.js:47-51,98-101,120-121`
- **Severity:** major
- **Evidence:** `'@typescript-eslint/no-explicit-any': 'off'`, `'no-non-null-assertion': 'off'`, `'ban-ts-comment': 'off'`, `'svelte/require-each-key': 'off'`, `'svelte/no-at-html-tags': 'off'`, `'no-console': 'off'`.
- **Root cause:** Loose ESLint defeats the value of `strict: true` in tsconfig.

---

### 3. Svelte A11y

#### A11y — Finding 1: ~26 `<img>` missing `alt` (WCAG 1.1.1 Level A)
- **Severity:** major
- **Files:** `lib/options-calculator/components/growth/WatermarkOverlay.svelte:86`, `ui/BlurHashImage.svelte:136`, `forms/pro/GeolocationAddress.svelte:355`, `traders/TraderHeader.svelte:30`, `resources/{ResourceCard,ResourceViewer,RecentlyAccessed}.svelte`, `admin/CourseDetailDrawer.svelte:599`, `blog/BlockEditor/{AssetManager.svelte:1171,1400, ImageUploader.svelte:686}`, `cms/blocks/trading/ChartBlock.svelte:522`, `cms/blocks/media/{GalleryBlock.svelte:428,479,510, ImageBlock.svelte:398,415,581}`, `media/{UploadProgress.svelte:215,221, ImageCropModal.svelte:529, MediaGrid.svelte:184, ResponsivePreview.svelte:148,221, OptimizedImage.svelte:164,192}`, `routes/admin/indicators/create/+page.svelte:1193`, `routes/admin/subscriptions/invoice-settings/+page.svelte:345`, `routes/checkout/+page.svelte:532,536,540`, `routes/blog/[slug]/+page.svelte:265,365`

#### A11y — Finding 2: Keyboard-inaccessible options chain
- **File:** `lib/options-calculator/components/OptionsChainViewer.svelte:216-218,240-242`
- **Severity:** blocker
- **Evidence:** Six `<td>` cells with `onclick` and `cursor-pointer` — no `tabindex`, no `role`, no `onkeydown`. Entire grid is mouse-only.

#### A11y — Finding 3: Unnamed dialog
- **File:** `lib/components/KeyboardShortcutsHelp.svelte:70-71`
- **Severity:** major
- **Evidence:** `role="dialog" aria-modal="true"` with no `aria-label` or `aria-labelledby`.

#### A11y — Finding 4: ~10 `<nav>` landmarks missing `aria-label`
- **Files:** `admin/CourseDetailDrawer.svelte:325`, `SegmentDetailDrawer.svelte:368`, `SubscriptionDetailDrawer.svelte:379`, `MemberDetailDrawer.svelte:349`, `CourseFormModal.svelte:297`, `SubscriptionFormModal.svelte:256`, `layout/AppSidebar.svelte:48`, `admin/Sidebar.svelte:97`, `dashboard/DashboardSidebar.svelte:244`
- **Severity:** minor

#### A11y — Finding 5: Form inputs without programmatic labels
- **Severity:** major
- **Files:** `lib/options-calculator/components/ScenarioEngine.svelte:95,99,103,108` (4 number inputs with `<span>` labels only), `TimeMachine.svelte:168` (range), `charts/VolSmile.svelte:186,191` (2 ranges), `cart/NonMemberCheckout.svelte:473` (coupon, placeholder only), `CommandPalette.svelte:358-368` (search), `PopupModal.svelte:1128-1129` (label rendered conditionally).

#### A11y — Finding 6: Icon-only buttons missing `aria-label`
- **Severity:** minor
- **Files:** `DashboardWidgetManager.svelte:161`, `admin/MediaUploadHub.svelte:519`, `admin/IndicatorBuilder.svelte:371`, `admin/BulkEditModal.svelte:148`, `admin/CourseBuilder.svelte:289,618,621,625` (use `title=` instead), `media/ImageCropModal.svelte:454,464,478,493`, `NotificationCenter.svelte:175`.

#### A11y — Finding 7: Tap targets <44px
- **Severity:** minor
- **Files:** `lib/options-calculator/components/ui/AnimatedSlider.svelte:91,120`, `OptionsChainViewer.svelte:216-242`, `DataSourceBadge.svelte:92`, `NotificationCenter.svelte:228`.

#### A11y — Finding 8: svelte-check verbose: 0/0
- **Severity:** info — clean.

#### A11y — Finding 9: NavBar / LoginForm / AdminToolbar / BunnyVideoPlayer = exemplary
- **Severity:** info — these implementations include proper roving tabindex, focus trap, live regions, full menu role pattern.

---

### 4. Runes & Svelte 5 Compliance

#### Runes — Finding 1: 2 `effect_update_depth_exceeded` time-bombs (BLOCKER)
- **File:** `frontend/src/routes/signup/+page.svelte:25-29`
- **Severity:** blocker
- **Evidence:**
  ```svelte
  $effect(() => {
      if ($authStore.user) {
          goto('/account');
      }
  });
  ```
- **Root cause:** `$store` autosubscribe inside `$effect` triggers `legacy_pre_subscribe` write during effect run → cascade. Same exact pattern user just fixed at `admin/+layout.svelte`.
- **Why it matters:** Successful signup will trigger this cascade, breaking the post-signup flow.

#### Runes — Finding 2: Auth-guard cascade in account/sessions
- **File:** `frontend/src/routes/account/sessions/+page.svelte:33-42`
- **Severity:** blocker
- **Evidence:** `$effect` reads `$isAuthenticated` AND `$authStore` (legacy autosubscribe) AND calls `loadSessions()` which writes `$state`.
- **Root cause:** Same pattern. Will loop on first render.

#### Runes — Finding 3: Missing-cleanup listener accumulation
- **File:** `frontend/src/lib/components/CommandPalette.svelte:324-329`
- **Severity:** major
- **Evidence:** `$effect` adds global `window.addEventListener('keydown', ...)` with no `return` cleanup; cleanup lives in `onDestroy` instead.
- **Root cause:** Effect re-fires on each reactive change and adds a new listener each time without removing the previous one — listener leak.

#### Runes — Finding 4: Double-fetch on mount in FormAnalyticsDashboard
- **File:** `frontend/src/lib/components/forms/FormAnalyticsDashboard.svelte:199-201`
- **Severity:** major
- **Evidence:** Both `onMount(() => fetchAnalytics())` AND `$effect(() => fetchAnalytics())` are present.
- **Root cause:** Two API calls on mount; `$effect` re-fires on any reactive change.

#### Runes — Finding 5: Pervasive shadow-state pattern
- **Files:** `forms/pro/PostTitleField.svelte:34`, `PostExcerptField.svelte:36`, `EnhancedCheckbox.svelte:48`, `ChainedSelectField.svelte:46`, `AddressField.svelte:38`, `FormStyler.svelte:69`, `TaxonomyField.svelte:69`, `RepeaterField.svelte:66`, `VideoEmbed.svelte:330-334` (4 instances), `CountdownTimer.svelte:218`, `routes/admin/trading-rooms/[slug]/+page.svelte:300-322` (6 instances)
- **Severity:** minor
- **Root cause:** `let foo = $state(...)` shadowed from props via `$effect` sync. Should be `$derived` or `$bindable`.

#### Runes — Finding 6: $effect that should be $derived
- **Files:** 13 instances across `routes/+page.svelte:38`, `BunnyVideoPlayer.svelte:126`, `BlurHashImage.svelte:114`, `PostTitleField`, `EnhancedCheckbox`, `ChainedSelectField`, `AddressField`, `FormStyler`, `TaxonomyField`, `RepeaterField`, `FormBuilder`, `FieldEditor`, `routes/admin/trading-rooms/[slug]/+page.svelte`
- **Severity:** minor

#### Runes — Finding 7: Zero legacy patterns
- **Severity:** info
- **Evidence:** 0 `export let`, 0 `$:`, 0 `on:click`, 0 `<slot>`, 0 `createEventDispatcher`, 0 `beforeUpdate`/`afterUpdate`, 0 `$$props`/`$$restProps`.

---

### 5. SvelteKit 2.x Routing

#### Routing — Finding 1: `hooks.server.ts` missing `handleError` export
- **File:** `frontend/src/hooks.server.ts`
- **Severity:** major
- **Root cause:** No server-side error tracking; load/action errors produce default response with no error ID.

#### Routing — Finding 2: 51 route groups lack `+error.svelte`
- **Severity:** major
- **Evidence:** Only `/`, `/admin`, `/blog` have scoped error boundaries. Missing for `/dashboard/`, `/checkout/`, `/account/`, `/auth/`, `/classes/`, `/store/`, `/tools/`, plus 44 others.
- **Why it matters:** Errors fall through to root, losing context-specific recovery UI.

#### Routing — Finding 3: Synthetic fallback user in auth handler
- **File:** `frontend/src/hooks.server.ts:254-264`
- **Severity:** major
- **Evidence:** On transient API failures, `locals.user` is set to `{ id: 0, email: 'session@preserved.local' }`.
- **Why it matters:** Any child load reading `locals.user` as truthy treats network errors as authenticated sessions.

#### Routing — Finding 4: No `depends()` registration anywhere
- **Severity:** minor
- **Evidence:** `invalidateAll()` in `routes/dashboard/[room_slug]/daily-videos/+page.svelte:103,110` re-runs every load function.
- **Root cause:** Use of broad invalidation instead of scoped `depends('app:daily-videos')`.

#### Routing — Finding 5: account/sessions auth-gated only client-side
- **File:** `frontend/src/routes/account/sessions/+page.svelte:33`
- **Severity:** major
- **Root cause:** No `+page.server.ts`; client-only `$effect` is the gate. (Doubly bad because of cascade Finding 2 in §4.)

---

### 6. CSS / PE7 Standard

#### CSS — Finding 1: 213 off-scale breakpoints
- **Severity:** major
- **Evidence:** Worst offenders: `480px` (~40), `360px` (~30), `1152px` (8 — all in `NavBar.svelte`), `992px`, `1200px`.
- **Why it matters:** Breaks the 9-tier scale; new device sizes will have inconsistent responses.

#### CSS — Finding 2: 351 hardcoded hex/rgb colors
- **Severity:** major
- **Evidence:** Entire `lib/options-calculator/styles/calculator-theme.css` (90+ hex tokens), `lib/styles/design-tokens.css` (51+), `marketing.css:31-32` (`#0a0a0a`, `#f5f5f5`), `lib/components/sections/Hero.svelte:822-875` (14 rgba/rgb).

#### CSS — Finding 3: 767 non-logical properties
- **Severity:** major
- **Root cause:** RTL or content-direction toggle would require hunting all 767 sites individually.

#### CSS — Finding 4: 1755 fixed `px` font sizes
- **Severity:** minor
- **Why it matters:** Brittle on user-zoom; should use `clamp()`.

#### CSS — Finding 5: 19 files missing `@layer` assignments
- **Severity:** major
- **Files:** `marketing.css`, `app.css:7,42,265-318` partial, all of `lib/styles/{dashboard,main,design-tokens,performance,print,admin-responsive}.css`, all of `lib/styles/tokens/`, all of `lib/styles/base/`.
- **Root cause:** Tailwind v4 requires `@layer` for predictable cascade.

#### CSS — Finding 6: 0 `!important` declarations
- **Severity:** info — clean.

---

### 7. Rust Correctness

#### Rust — Finding 1: 17 silent DB error swallows in `routes/crm.rs` (BLOCKER)
- **Severity:** blocker
- **Lines:** `crm.rs:539, 644, 730, 855, 956, 1052, 1086, 1185, 1269, 1370, 1463, 1727, 2326, 2343, 2419, 2637, 3158`
- **Pattern:** `sqlx::query_as!(...).fetch_all(&pool).await.unwrap_or_default()` — DB connection failure or schema mismatch returns empty list with HTTP 200.

#### Rust — Finding 2: 14 silent DB error swallows in `routes/courses_admin.rs`
- **Severity:** blocker
- **Lines:** `154, 162, 170, 179, 311, 364, 368, 438, 442, 1272, 1419, 1464, 1594, 1630`

#### Rust — Finding 3: Empty-string credentials in `config/mod.rs`
- **File:** `api/src/config/mod.rs:146-213`
- **Severity:** major
- **Evidence:** `std::env::var("R2_ACCESS_KEY_ID").unwrap_or_default()` produces empty string when unset; failure deferred to runtime.

#### Rust — Finding 4: Production `.unwrap()` in state_machine
- **File:** `api/src/domain/workflow/state_machine.rs:314`
- **Severity:** major
- **Root cause:** Logical invariant; will panic if `transitions` map diverges from `can_perform_action`.

#### Rust — Finding 5: 9 `.expect()` calls in production
- **Files:** `api/src/queue/worker.rs:78`, `api/src/services/cms_webhooks.rs:88`, `api/src/routes/newsletter.rs:32` (HMAC); `api/src/services/cms_scheduler.rs:74` (HTTP client); `api/src/routes/cms_seo.rs:73-113` (regex statics, ×9).
- **Severity:** minor
- **Most are acceptable Lazy-static patterns; HTTP client expect at startup is the riskiest.**

#### Rust — Finding 6: 1 unsafe block without `// SAFETY:` comment
- **File:** `api/src/bin/bootstrap_dev.rs:244` (`unsafe { libc::tcgetattr(...) }`)
- **Severity:** minor
- **Note:** Dev binary only.

#### Rust — Finding 7: cargo check / clippy / fmt all clean
- **Severity:** info — clean.

---

### 8. Rust Dependency & Security

#### Rust-Deps — Finding 1: reqwest 0.11 ⇒ 4 ignored CVEs
- **Severity:** major
- **Evidence:** `RUSTSEC-2026-0098/0099/0104` (rustls-webpki) and `RUSTSEC-2025-0134` (rustls-pemfile) — all close on `reqwest 0.12+`.

#### Rust-Deps — Finding 2: Massive duplicate-version bloat
- **Severity:** major
- **Evidence:** 40 crates with two copies — `hyper 0.14 + 1.x`, `rustls 0.21 + 0.23`, `tower 0.4 + 0.5`, `rand 0.8 + 0.9`, `http`, `h2`, `base64`, `getrandom`, etc.
- **Root cause:** `reqwest 0.11` pulls old ecosystem; `axum 0.7` pulls new.

#### Rust-Deps — Finding 3: Major upgrade lag
- **Severity:** major
- **Evidence:** axum 0.7 → 0.8, tower 0.4 → 0.5, reqwest 0.11 → 0.13, redis 0.27 → 1.2, jsonwebtoken 9 → 10, utoipa 4 → 5, thiserror 1 → 2.

#### Rust-Deps — Finding 4: 5 deferred aws-lc-rs CVEs
- **Severity:** info — risk noted in deny.toml.

#### Rust-Deps — Finding 5: cargo deny clean, profile.release tuned
- **Severity:** info — `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`.

---

### 9. Rust Test & Coverage

#### Rust-Test — Finding 1: 115 integration tests blocked by missing Postgres role
- **Severity:** major
- **Evidence:** `tests/explosive_swings_tests.rs` (101 tests) + `tests/integration_tests.rs` (14 tests) all panic at `fixtures.rs:98` — `role "test" does not exist`.
- **Root cause:** Local Postgres test role not provisioned. CI shows 101+ failures on every fresh checkout.

#### Rust-Test — Finding 2: 101 source files >100 LOC with zero test coverage
- **Severity:** major
- **Top untested:** `routes/crm.rs` (3,379 LOC), `routes/admin_courses.rs` (2,312), `routes/admin_videos.rs` (1,858), `routes/auth.rs` (1,103), `routes/oauth.rs` (1,100), `routes/payments.rs` (1,492), `routes/checkout.rs`, `routes/subscriptions.rs` (1,637).

#### Rust-Test — Finding 3: 4 pure-unit tests masquerading as integration
- **File:** `tests/explosive_swings/stats_test.rs`, `alerts_test.rs`
- **Severity:** info
- **Root cause:** Pure-arithmetic helpers in integration binary.

#### Rust-Test — Finding 4: Lib-unit + no-DB suites all pass
- **Severity:** info
- **Evidence:** lib-unit 58/58, utils_test 17/17, stripe_test 15/15, stripe_price_sync 6/6, stripe_subscription_migration 3/3.

---

### 10. API Contract

#### Contract — Finding 1 (BLOCKER): 14 proxies use wrong cookie name
- **Severity:** blocker
- **Evidence (auth_token, never set; cookie set is `rtp_access_token`):**
  - `frontend/src/routes/api/admin/courses/[id]/+server.ts:16,43,67`
  - `frontend/src/routes/api/admin/courses/+server.ts:16,65`
  - `frontend/src/routes/api/admin/courses/[id]/publish/+server.ts:16`
  - `frontend/src/routes/api/admin/courses/[id]/unpublish/+server.ts:16`
  - `frontend/src/routes/api/admin/subscriptions/plans/stats/+server.ts:17`
  - `frontend/src/routes/api/products/+server.ts:15`
  - `frontend/src/routes/api/admin/membership-plans/+server.ts:15`
  - `frontend/src/routes/api/admin/products/stats/+server.ts:30`
  - `frontend/src/routes/api/cms/newsletter/subscribe/+server.ts:40`
- **Evidence (`access_token`, never set):**
  - `frontend/src/routes/api/courses/slug/[slug]/player/+server.ts:27`
  - `frontend/src/routes/api/courses/slug/[slug]/downloads/+server.ts:23`
  - `frontend/src/routes/api/courses/[id]/downloads/+server.ts:23`
- **Evidence (`session`, never set):**
  - `frontend/src/routes/api/favorites/+server.ts:32`
  - `frontend/src/routes/api/favorites/[id]/+server.ts:29`
  - `frontend/src/routes/api/favorites/check/+server.ts:30`
  - `frontend/src/routes/api/admin/bunny/uploads/+server.ts:29`
  - `frontend/src/routes/api/weekly-video/[slug]/+server.ts:136`
  - `frontend/src/routes/api/export/watchlist/+server.ts:42`
- **Why it matters:** All 14 backend requests receive 401 with no observable error.

#### Contract — Finding 2 (BLOCKER): Frontend API clients with no backend
- **Severity:** blocker
- **Evidence:**
  - `frontend/src/lib/api/boards.ts` — 30+ endpoints under `/admin/boards/*` — no Rust route.
  - `frontend/src/lib/api/behavior.ts` — `/admin/behavior/*` — no Rust route. SvelteKit proxy at `/api/behavior/events` is a stub.
  - `frontend/src/lib/api/bing-seo.ts` — `/admin/404-errors/*` — no Rust route.
  - `frontend/src/lib/api/abandoned-carts.ts` — `/admin/crm/abandoned-carts/*` — no Rust sub-router registered.
  - `frontend/src/lib/api/trading-room-sso.ts` — no backend.
  - `frontend/src/lib/api/bannedEmails.ts` — `/admin/cms-v2/banned-emails` — no Rust route.

#### Contract — Finding 3: Subscriptions metrics is a stub
- **Severity:** major
- **Evidence:** `frontend/src/routes/api/subscriptions/metrics/+server.ts:10` returns hardcoded zeros; backend `/api/subscriptions/metrics` exists at `subscriptions.rs:1592`.

#### Contract — Finding 4: SSE proxy returns empty stream
- **Severity:** major
- **Evidence:** `frontend/src/routes/api/sse/+server.ts:9` immediately returns empty stream; backend `/api/realtime/events` (`realtime.rs:457`) is never connected.

#### Contract — Finding 5: Admin connections fully shadow-stubbed
- **Severity:** major
- **Evidence:** `frontend/src/routes/api/admin/connections/+server.ts` is in-memory mock; backend `connections.rs:1563` is real and never reached.

#### Contract — Finding 6: Refresh response field shape mismatch
- **Severity:** minor
- **Evidence:** Backend `RefreshTokenResponse` returns `{ token, refresh_token, expires_in }`; frontend prefers `access_token` first, falls back to `token`.

#### Contract — Finding 7: Non-canonical env vars in 14 server proxies
- **Severity:** major
- **Evidence:**
  - 7 use `env.VITE_API_URL` (non-canonical primary).
  - `frontend/src/routes/api/videos/+server.ts:18` uses `import.meta.env.VITE_API_URL` — **client-side env in server route → undefined at runtime**.
  - `frontend/src/routes/api/cms/newsletter/subscribe/+server.ts:3` imports `$lib/config` which uses `import.meta.env` — same issue.
  - `frontend/src/routes/api/videos/upload/+server.ts:20` uses `process.env.VITE_API_URL`.
  - 3 use `env.API_URL` (undocumented var name).

---

### 11. JS Dependency & Security

#### JS-Deps — Finding 1: Forbidden `@lucide/svelte` package present
- **File:** `frontend/package.json` (devDependency)
- **Severity:** major
- **Root cause:** CLAUDE.md bans Lucide; `@lucide/svelte` is the rebranded successor of `lucide-svelte`.

#### JS-Deps — Finding 2: 1 moderate + 1 low CVE
- **Severity:** minor
- **Evidence:**
  - `uuid <14.0.0` (moderate, GHSA-w5hq-g745-h8pq) via `vite-plugin-devtools-json` — dev-only.
  - `cookie <0.7.0` (low, GHSA-pxg6-pf52-xh8x) via `@sveltejs/kit@2.58.0` — blocked on Kit upgrade.

#### JS-Deps — Finding 3: vite + @sveltejs/vite-plugin-svelte one major behind
- **Severity:** minor
- **Evidence:** `vite 7.3.2 → 8.0.10`, `@sveltejs/vite-plugin-svelte 6.2.4 → 7.0.0`.

#### JS-Deps — Finding 4: Dead transitive DB types in lockfile
- **Severity:** info
- **Evidence:** `@types/mysql`, `@types/pg`, `@types/pg-pool`, `@types/tedious` — Node DB types in a Rust-backed app.

---

### 12. Dead Code

#### Dead-Code — Finding 1 (BLOCKER): Frontend API clients with no backend
- **Severity:** blocker (Cross-stack)
- **Same as Contract Finding 2.**

#### Dead-Code — Finding 2: 53 unimported `lib/` files (~8–15k LOC)
- **Severity:** major
- **Top:** all `admin/` UI components (`CourseBuilder`, `IndicatorBuilder`, `BulkUploadQueue`, ...), all `forms/` advanced components, `options-calculator` growth/education subset, `observability` adapters, `ai/content-assistant`, `versioning/version-manager`, `bannedEmails.ts`, `trading-room-sso.ts`.

#### Dead-Code — Finding 3: 14 unused public helpers in `utils/errors.rs` + `middleware/validation.rs`
- **Files:** `api/src/utils/errors.rs:66,80,106,129,139,197,236,245,254,266,279`; `api/src/middleware/validation.rs:78,86,127,132`
- **Severity:** minor

#### Dead-Code — Finding 4: Compiled-but-not-registered routers
- **Severity:** major
- **Evidence:**
  - `api/src/routes/organization.rs::profile_router` (line 1216) — defined, never registered.
  - `api/src/routes/admin_courses.rs::taxonomy_router` (line 2306) — defined, never registered.
  - `api/src/services/cms_scheduler.rs::start_scheduler` (line 44) — never called from `main.rs` → CMS scheduling silently no-op.

#### Dead-Code — Finding 5: WebSocket JWT accepted but never validated
- **File:** `api/src/routes/websocket.rs:344`
- **Severity:** blocker (security)
- **Evidence:** `// TODO: Validate JWT token` — token is accepted but not verified.

#### Dead-Code — Finding 6: 23 frontend + 18 backend TODO/FIXME markers
- **Severity:** info to minor
- **Notable:** 6 video-fetch TODOs in `+page.server.ts` files claim a "new approach" pending; `dashboard/account/view-subscription/[id]/+page.server.ts:60,73` returns stub data because backend not ready; `media/+page.svelte:33,127` references non-existent `CreateFolderModal`; `services/cms_content.rs:1428` regex matching unimplemented; `routes/email_templates.rs:384` no email service integration.

---

### 13. Config Integrity

#### Config — Finding 1 (BLOCKER): Production builds ship dev plugins
- **Severity:** blocker (security/UX)
- **Files:** `frontend/vite.config.ts:11-16`
- **Evidence:** `svelteInspector({ showToggleButton: 'always' })` and `vite-plugin-devtools-json` are unconditional. The latter exposes project metadata at `/.well-known/appspecific/com.chrome.devtools.json`.

#### Config — Finding 2: `script-src 'unsafe-inline'` defeats SvelteKit nonces
- **File:** `frontend/svelte.config.js:56`
- **Severity:** major
- **Root cause:** `mode: 'auto'` injects nonces but `'unsafe-inline'` overrides nonce trust → all inline scripts trusted.

#### Config — Finding 3: `wrangler.toml [vars]` for VITE_* keys is dead config
- **File:** `frontend/wrangler.toml:10-13`
- **Severity:** major
- **Why it matters:** Operators may believe changing these values updates production runtime; they don't (VITE_* is build-time only).

#### Config — Finding 4: `CORS_ORIGINS` undocumented + defaults to localhost in prod
- **Files:** `api/fly.toml`, `api/src/config/mod.rs:174`
- **Severity:** major
- **Why it matters:** If Fly secret missing during redeploy, prod API rejects all browser requests with no warning.

#### Config — Finding 5: Dockerfile uses non-deterministic + testing channels
- **File:** `api/Dockerfile:5,34`
- **Severity:** major
- **Evidence:** `FROM rust:latest`, `FROM debian:trixie-slim` (Debian testing — no stable security backports).

#### Config — Finding 6: tsconfig missing key strict flags
- **File:** `frontend/tsconfig.json`
- **Severity:** minor
- **Missing:** `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noPropertyAccessFromIndexSignature`.

#### Config — Finding 7: Playwright config uses `npm run dev`
- **File:** `frontend/playwright.config.ts:43`
- **Severity:** minor — repo is pnpm.

#### Config — Finding 8: docker-compose uses `condition: service_started` for redis
- **File:** `docker-compose.yml:82`
- **Severity:** minor — should be `service_healthy`.

#### Config — Finding 9: ESLint `ignores: ['tests/**', 'scripts/**']`
- **File:** `frontend/eslint.config.js:133-145`
- **Severity:** minor — tests skipped from lint entirely.

#### Config — Finding 10: VITE_DEVELOPER_EMAILS contains real email
- **File:** `frontend/.env.production:37-38`
- **Severity:** info — already public via build, but worth flagging.

---

### 14. Frontend Test Coverage

#### Test-Coverage — Finding 1: 1440 pass / 2 fail / 32 skipped (Vitest)
- **Severity:** minor
- **Failing tests:** `ChartBlock.test.ts:191` height assertion, `ColumnsBlock.test.ts:495` gap assertion — both jsdom CSS-custom-property limits.

#### Test-Coverage — Finding 2: Entire `lib/api/` layer untested (~25k LOC)
- **Severity:** major
- **Highest risk:** `client.svelte.ts:1604` (reactive API client + retry/refresh queue), `auth.ts:1478`, `cart.ts:1553`, `subscriptions.ts:1403`, `coupons.ts:1617`, `popups.ts:1743`, `forms.ts:1633`, `admin.ts:1688`.

#### Test-Coverage — Finding 3: BlockEditor ecosystem untested
- **Severity:** major
- **Files:** `AssetManager.svelte:3251`, `BlockEditor.svelte:2892`, `BlockSettingsPanel.svelte:2146`, `GlobalComponentLibrary.svelte:1953`, `SchedulingPanel.svelte:1598`.

#### Test-Coverage — Finding 4: Security utils untested
- **Severity:** major
- **Files:** `lib/utils/sanitization.ts:1091`, `lib/utils/safe-math-parser.ts:380` (replaces `eval()`).

#### Test-Coverage — Finding 5: 32 permanently-skipped tests
- **Severity:** minor
- **Files:** `ButtonBlock` (icon position L/R, link target, disabled, icon types, aria-hidden — 8 entire suites), `GalleryBlock` (interactive add-image flows — 4 tests), `CalloutBlock` (tip-type role — 2), `VideoBlock` (YouTube Shorts URL — 1), 17 more.

---

### 15. A11y Summary (consolidated from §3)

A11y agent completed. All svelte-check verbose runs return 0/0; the actual a11y debt comes from raw HTML patterns the compiler does not catch: missing `alt`, missing `aria-label`/`aria-labelledby` on dialogs and `<nav>` landmarks, mouse-only `<td>` cells in OptionsChainViewer, form inputs without `<label for>` association. Total a11y findings: ~50 distinct sites; ~26 missing `alt`, ~10 unnamed `<nav>`, ~10 form inputs without programmatic labels, 6 keyboard-inaccessible cells, plus 4 unnamed icon-buttons and 4 sub-44px tap targets. Several core flows (NavBar, LoginForm, AdminToolbar, BunnyVideoPlayer) are exemplary — the issues cluster in newer/peripheral surfaces (options-calculator, CMS block editor, media library, admin drawers).

---

## Root-Cause Clustering

### Cluster A — Cookie name drift in SvelteKit proxies (BLOCKER)
**Symptoms tied:** Contract Findings 1; affects 14+ admin/member proxies.
**Root cause:** Proxies were copy-pasted from older code that used `auth_token`/`access_token`/`session` cookies; current login proxy sets `rtp_access_token`. Single-symbol rename across all 14 files would fix every symptom.

### Cluster B — Svelte 5 legacy auto-subscribe in $effect (BLOCKER)
**Symptoms tied:** Runes Findings 1, 2; the `admin/+layout.svelte` cascade user just fixed.
**Root cause:** Auth-guard pages converted to runes but `$store` autosubscribe inside `$effect` is the booby trap. Same one-line `$effect → onMount` conversion fixes all three sites.

### Cluster C — Silent error swallowing in DB query handlers (BLOCKER)
**Symptoms tied:** Rust Findings 1, 2 (31 sites total).
**Root cause:** `.fetch_all().await.unwrap_or_default()` was used as a "safe default" but actually masks Postgres failures. Fix is mechanical: `.fetch_all().await?` + propagate.

### Cluster D — Frontend orphan API clients (BLOCKER)
**Symptoms tied:** Contract Finding 2, Dead-Code Finding 1.
**Root cause:** Pages and clients written speculatively before backend implementation. Either delete the frontend code (boards, behavior, bing-seo, banned-emails, abandoned-carts, trading-room-sso) or build the backend.

### Cluster E — Outdated reqwest 0.11 → cascade of duplicate-version bloat + 4 CVEs
**Symptoms tied:** Rust-Deps Findings 1, 2.
**Root cause:** Single dep version. Upgrade to `reqwest 0.12+` collapses 40 duplicate crates AND closes 4 CVEs.

### Cluster F — Dev plugins/CSP/Dockerfile = unsafe production builds
**Symptoms tied:** Config Findings 1, 2, 5.
**Root cause:** No "production hardening checklist" enforced before deploys.

### Cluster G — PE7 CSS standard not enforced
**Symptoms tied:** CSS Findings 1, 2, 3, 4, 5.
**Root cause:** No stylelint/regex CI gate. Single-pass codemod possible (logical properties + breakpoint snap-to-scale + clamp() conversion).

---

## Cross-Stack Risk Map

### Type Drift
- **Refresh token shape** (`user.rs:87` returns `token` flat; frontend prefers `access_token` then falls back to `token`).
- **Error response shape:** backend mixes `{ error }` and `{ message, errors? }`; frontend always normalizes to `{ error }`. No shared schema.
- **No code-generation** between Rust serde models and TypeScript types — every shape mismatch is undiscovered until runtime.

### Auth Flow Gaps
- **14 proxies using wrong cookie names** silently break admin/member features (Cluster A).
- **Synthetic fallback user** (`hooks.server.ts:254-264`) treats network errors as authenticated.
- **WebSocket JWT never validated** (`websocket.rs:344` TODO) — anyone can connect.
- **`account/sessions`** authenticated only client-side via legacy `$effect`.

### Error-Handling Inconsistency
- Backend: 31 silent DB error swallows in CRM + courses_admin; mixed `{ error }` vs `{ message }` shapes.
- Frontend: missing `handleError` in `hooks.server.ts`; 51 route groups missing `+error.svelte`.
- Result: production failures are invisible to logs and to users.

### Race Conditions / Reactivity
- 2 unfixed `$effect` cascade time-bombs (signup, account/sessions).
- `FormAnalyticsDashboard` double-fetch on mount.
- `CommandPalette` listener accumulation.
- Auth-store fan-out is microtask-deferred (good) but consumers must not read+write during effects (the cause of the just-fixed admin cascade).

---

## Files Referenced (key sites)

`frontend/src/routes/admin/+layout.svelte:52`, `frontend/src/routes/account/sessions/+page.svelte:33`, `frontend/src/routes/signup/+page.svelte:25`, `frontend/src/lib/components/CommandPalette.svelte:324`, `frontend/src/routes/api/admin/courses/[id]/+server.ts:16`, `frontend/src/routes/api/courses/slug/[slug]/player/+server.ts:27`, `frontend/src/routes/api/favorites/+server.ts:32`, `frontend/src/lib/api/boards.ts`, `frontend/src/lib/api/behavior.ts`, `frontend/vite.config.ts:11-16`, `frontend/svelte.config.js:56`, `api/Dockerfile:5,34`, `api/fly.toml`, `api/src/config/mod.rs:146-213`, `api/src/routes/crm.rs:539+`, `api/src/routes/courses_admin.rs:154+`, `api/src/routes/websocket.rs:344`, `api/src/services/cms_scheduler.rs:44`, `api/Cargo.toml:76` (reqwest 0.11), `api/src/domain/workflow/state_machine.rs:314`.

---

*A11y agent findings will be appended in §3 and §15 once complete; this report is otherwise final.*
