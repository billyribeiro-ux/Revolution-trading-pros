# Audit Fix Plan — Phased

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.

**Source:** `./AUDIT_REPORT.md`
**Goal:** Restore production correctness, then enforce standards.
**Dependency rule:** Phase N must complete and pass its verification before Phase N+1 starts unless marked `parallel-safe`.
**Commit/push rule:** Per user-stated session rule, no commits/pushes occur without explicit user approval after each phase.

---

## Phase 1 — Blockers (effort: M, ~6–10 hours)

### 1.1 Fix the 14 wrong-cookie auth proxies (Cluster A)
**Effort:** small (mechanical rename across 14 files)
**Dependency:** none (standalone)
**Files:** all 18 sites cited in AUDIT_REPORT §10 Finding 1.
**Change:** every occurrence of `cookies.get('auth_token')` / `cookies.get('access_token')` / `cookies.get('session')` → `cookies.get('rtp_access_token')`. Comment out (don't delete) the original line per session rule.
**Verification:**
1. `pnpm --filter revolution-svelte check` → 0/0/0.
2. Smoke admin login → `/admin/courses` should load real data, not show empty list.
3. Smoke `/dashboard/.../courses/<slug>/player` → should return 200 with token forwarded.
4. Smoke favorites add/remove on logged-in account.

### 1.2 Fix 2 remaining `effect_update_depth_exceeded` time-bombs (Cluster B)
**Effort:** trivial (5-line change × 2 files)
**Dependency:** none (parallel-safe with 1.1)
**Files:** `frontend/src/routes/signup/+page.svelte:25-29`, `frontend/src/routes/account/sessions/+page.svelte:33-42`.
**Change:** convert each `$effect` reading `$authStore`/`$isAuthenticated` to `onMount`. Same exact pattern just landed in `admin/+layout.svelte`.
**Verification:**
1. Sign up a new test user → no `effect_update_depth_exceeded` in console.
2. Visit `/account/sessions` while authenticated → no cascade, sessions load.
3. `pnpm check` → 0/0/0.

### 1.3 Stop production from shipping dev plugins
**Effort:** trivial
**Dependency:** none (parallel-safe)
**File:** `frontend/vite.config.ts:11-16`.
**Change:** Wrap `svelteInspector(...)` and `devtoolsJson()` in `mode === 'development'` guard inside the `defineConfig(({ mode }) => ...)` callback.
**Verification:**
1. `pnpm --filter revolution-svelte build`.
2. Inspect built HTML in `.svelte-kit/cloudflare/` — no `svelteInspector`/`devtools-json` script tags.
3. Visit `/.well-known/appspecific/com.chrome.devtools.json` on a deployed preview → 404.

### 1.4 Fix CSP `'unsafe-inline'` overriding nonce mechanism
**Effort:** trivial
**Dependency:** parallel-safe with 1.3
**File:** `frontend/svelte.config.js:56`.
**Change:** Remove `'unsafe-inline'` from `script-src`. Let SvelteKit's `mode: 'auto'` inject nonces.
**Verification:**
1. `pnpm --filter revolution-svelte build && pnpm --filter revolution-svelte preview`.
2. View source — `<script>` tags carry `nonce="..."`.
3. CSP report-only test in browser console: no nonce-violations.

### 1.5 Fix 31 silent DB error swallows (Cluster C)
**Effort:** medium (mechanical, 31 sites in 2 files)
**Dependency:** parallel-safe with 1.1–1.4
**Files:** `api/src/routes/crm.rs:539,644,730,855,956,1052,1086,1185,1269,1370,1463,1727,2326,2343,2419,2637,3158`; `api/src/routes/courses_admin.rs:154,162,170,179,311,364,368,438,442,1272,1419,1464,1594,1630`.
**Change:** every `.fetch_all(...).await.unwrap_or_default()` → `.fetch_all(...).await.map_err(AppError::from)?`. Existing `AppError::DbError` already handled by Axum's `IntoResponse`.
**Verification:**
1. `cargo check --manifest-path api/Cargo.toml` → green.
2. `cargo clippy --manifest-path api/Cargo.toml --all-targets -- -D warnings` → green.
3. Cause a deliberate DB failure (kill local Postgres) and hit `/api/admin/crm/contacts` → expect HTTP 500 with `{ error, request_id }`, not HTTP 200 with `[]`.

### 1.6 Validate JWT in WebSocket handler
**Effort:** small
**Dependency:** parallel-safe
**File:** `api/src/routes/websocket.rs:344` (TODO marker).
**Change:** Replace TODO with actual `jsonwebtoken::decode` call using existing `JwtConfig` from `AppState`. Reject connection with `4001` close code on invalid/expired token.
**Verification:**
1. Connect with no token → 4001 close.
2. Connect with expired token → 4001 close.
3. Connect with valid token → 101 Switching Protocols.

### 1.7 Decide and act on orphan API clients (Cluster D)
**Effort:** small (decision) + medium (codemod) — but this is a user-decision phase
**Dependency:** none
**Files:** `frontend/src/lib/api/{boards,behavior,bing-seo,bannedEmails,trading-room-sso,abandoned-carts}.ts` and any `+page.svelte` that imports them.
**Change:** **Decision required from user**: (a) delete the frontend code, OR (b) build the matching Rust backend routes. Either way, the current state — pages that reachably 404 — must end.
**Verification:** depends on decision.

---

## Phase 2 — Frontend type & a11y errors (effort: M, ~4–6 hours)

### 2.1 Fix the 2 ESLint parsing errors
**Effort:** trivial
**Files:** `frontend/src/lib/seo/Seo.svelte:129:45`, `frontend/src/routes/tools/options-calculator/+page.svelte:38:44`.
**Change:** Investigate Svelte 5 syntax that breaks the current ESLint plugin. May be `$props()` destructure with type assertion or new snippet syntax. Likely fix: upgrade `eslint-plugin-svelte` to latest, or restructure the offending line.
**Verification:** `pnpm --filter revolution-svelte exec eslint src/lib/seo/Seo.svelte src/routes/tools/options-calculator/+page.svelte` → 0 errors.

### 2.2 Add missing `alt` to ~26 `<img>` elements
**Effort:** medium (mechanical but file-by-file)
**Files:** see AUDIT_REPORT §3 Finding 1.
**Change:** Each gets either `alt={meaningful-description}` (content images) or `alt=""` + `aria-hidden="true"` (decorative).
**Verification:** `grep -rn '<img\b' frontend/src --include="*.svelte" | grep -v 'alt='` → 0 results.

### 2.3 Make OptionsChainViewer keyboard-accessible
**Effort:** medium
**File:** `frontend/src/lib/options-calculator/components/OptionsChainViewer.svelte:216-218,240-242`.
**Change:** Wrap each clickable `<td>` in a `<button>` (with `display: contents` styling) OR add `role="button" tabindex={0} onkeydown={handleKey}` to the `<td>`. Mirror the pattern in `lib/components/admin/CourseBuilder.svelte:539`.
**Verification:** Tab-navigate the chain → focus rings visible on each contract; `Enter`/`Space` selects.

### 2.4 Add `aria-label` to `KeyboardShortcutsHelp` dialog
**File:** `frontend/src/lib/components/KeyboardShortcutsHelp.svelte:70-71`.
**Change:** Add `aria-labelledby` referencing the existing `<h2>` id, or add `aria-label="Keyboard shortcuts"`.

### 2.5 Add `aria-label` to ~10 `<nav>` landmarks
**Files:** see AUDIT_REPORT §3 Finding 4.
**Change:** Each `<nav>` gets a distinct `aria-label` (e.g. `aria-label="Drawer tabs"`, `aria-label="Sidebar navigation"`).

### 2.6 Add `<label for>` to unlabeled form inputs
**Files:** AUDIT_REPORT §3 Finding 5.
**Change:** Add proper `<label for={id}>` or `aria-label` per input.

### 2.7 Convert `title=` to `aria-label` on icon buttons
**Files:** `admin/CourseBuilder.svelte:289,618,621,625`, `media/ImageCropModal.svelte:454,464,478,493`.
**Change:** keep `title=` for tooltip but add `aria-label` for AT.

---

## Phase 3 — Rust correctness (effort: M, ~3–5 hours)

### 3.1 Fail-fast on missing required env vars
**Effort:** small
**File:** `api/src/config/mod.rs:146-213`.
**Change:** Replace `.unwrap_or_default()` for `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `MEILI_*`, `STRIPE_*`, `JWT_SECRET` with explicit `.context("R2_ACCESS_KEY_ID env var is required")?` and surface the panic at startup. Optional vars stay as `.unwrap_or_default()`.
**Verification:** Boot API with `unset R2_ACCESS_KEY_ID` → API exits with clear message at startup, not at runtime.

### 3.2 Eliminate production `.unwrap()` in state_machine
**File:** `api/src/domain/workflow/state_machine.rs:314`.
**Change:** Replace `.unwrap()` with `.ok_or(WorkflowError::InvariantViolation { ... })`.

### 3.3 Add `// SAFETY:` comment to `bootstrap_dev.rs:244`
**File:** `api/src/bin/bootstrap_dev.rs:244`.
**Change:** Add 3-line safety justification comment above the `unsafe` block (single-threaded, dev-only, valid stdin fd).

### 3.4 Audit and convert .expect() in HMAC paths
**Files:** `api/src/queue/worker.rs:78`, `api/src/services/cms_webhooks.rs:88`, `api/src/routes/newsletter.rs:32`.
**Change:** HMAC `.expect("HMAC can take key of any size")` is technically infallible (HMAC-SHA256 accepts any key). Acceptable as-is, but document the invariant in the expect message: `.expect("HMAC-SHA256 accepts any key length per RFC 2104")`.

### 3.5 Move `axum-test` and `hyper` to `[dev-dependencies]`
**Effort:** trivial
**File:** `api/Cargo.toml:86-91`.
**Change:** Move `axum-test` (it's test-only — verify with grep that no `src/` file imports it). Move `hyper` if no `src/` file imports `hyper::` directly.
**Verification:** `cargo build --manifest-path api/Cargo.toml --release` succeeds.

---

## Phase 4 — API contract mismatches (effort: M, ~3–4 hours)

### 4.1 Wire up `/api/subscriptions/metrics` proxy
**File:** `frontend/src/routes/api/subscriptions/metrics/+server.ts`.
**Change:** Replace hardcoded zeros with `fetch(${API_BASE}/api/subscriptions/metrics)` using canonical env-var pattern + Bearer forwarding.

### 4.2 Replace `/api/sse` stub with real `/api/realtime/events` proxy
**File:** `frontend/src/routes/api/sse/+server.ts`.
**Change:** Either delete the stub and have the client connect directly to `/api/realtime/events` via the catch-all proxy, or rewrite this as a streaming proxy that pipes the SSE response.

### 4.3 Replace in-memory `/api/admin/connections` with real proxy
**Files:** `frontend/src/routes/api/admin/connections/+server.ts`, `frontend/src/routes/api/admin/connections/[key]/connect/+server.ts`, etc.
**Change:** Delete the standalone implementation. Forward to backend `/api/admin/connections` via Bearer-authenticated proxy.

### 4.4 Standardize env var pattern in 14 server proxies (Cluster F sub-piece)
**Files:** see AUDIT_REPORT §10 Finding 7.
**Change:** Every server proxy uses canonical pattern from CLAUDE.md:
```ts
import { env } from '$env/dynamic/private';
const API_URL = env.API_BASE_URL || env.BACKEND_URL || '<your-api-host>';
```
Rip out `import.meta.env.VITE_*`, `process.env.VITE_*`, `env.API_URL`, and `env.VITE_API_URL` first-position uses.
**Verification:** grep for `VITE_API_URL` in `frontend/src/routes/api/` — 0 results in `+server.ts` files.

### 4.5 Align refresh response field shape
**File:** `api/src/models/user.rs:87` OR `frontend/src/routes/api/auth/refresh/+server.ts:81`.
**Change:** Two options — pick the cheap one:
- (a) Add `access_token` field to backend `RefreshTokenResponse` for parity with `AuthResponse`. Frontend keeps preferring `access_token` and the fallback becomes dead code.
- (b) Frontend reads `data.token` first instead of `data.access_token`.
Recommend (a) for symmetry.

---

## Phase 5 — Lint & format (effort: S, ~1 hour)

### 5.1 Fix the 17 ESLint errors (excluding parsing — done in Phase 2.1)
**Files:** see AUDIT_REPORT §2.
**Change:** address `no-useless-assignment`, `preserve-caught-error`, `no-extraneous-class`, `no-unused-vars`, `no-useless-constructor`, `no-unsafe-function-type` per their rule docs.
**Verification:** `pnpm --filter revolution-svelte exec eslint . --max-warnings=0` → exit 0.

### 5.2 Format 219 unformatted files
**Effort:** trivial (one command, large diff)
**Change:** `pnpm --filter revolution-svelte exec prettier --write .` — review the diff before approving since this is a large mechanical change.
**Verification:** `pnpm --filter revolution-svelte exec prettier --check .` → 0 files unformatted.

### 5.3 Re-evaluate ESLint loose rules
**File:** `frontend/eslint.config.js`.
**Change:** Re-enable `'@typescript-eslint/no-explicit-any'` as `'warn'`, `'no-non-null-assertion'` as `'warn'`, `'ban-ts-comment'` as `'warn'`, `'svelte/require-each-key'` as `'warn'`, `'svelte/no-at-html-tags'` as `'warn'`, `'no-console'` as `'warn'` (allow `console.error`, `console.warn`, `console.info`).
**Verification:** Run lint and capture new violation list; fix or suppress per case.

---

## Phase 6 — Architecture & pattern fixes (effort: L, ~10–15 hours; parallelizable across sub-tasks)

### 6.1 Svelte 5 — fix shadow-state pattern
**Effort:** medium
**Files:** `forms/pro/{PostTitleField,PostExcerptField,EnhancedCheckbox,ChainedSelectField,AddressField,FormStyler,TaxonomyField,RepeaterField}.svelte`, `VideoEmbed.svelte:330-334`, `CountdownTimer.svelte:218`, `routes/admin/trading-rooms/[slug]/+page.svelte:300-322`.
**Change:** Convert `let foo = $state(...)` + `$effect(() => { foo = props.bar; })` pattern to `let foo = $derived(props.bar)` (read-only) or `let { bar = $bindable() } = $props()` (two-way).

### 6.2 Svelte 5 — fix CommandPalette listener leak
**File:** `frontend/src/lib/components/CommandPalette.svelte:324-329`.
**Change:** Move keydown listener attach to `onMount` with cleanup return.

### 6.3 Svelte 5 — fix FormAnalyticsDashboard double-fetch
**File:** `frontend/src/lib/components/forms/FormAnalyticsDashboard.svelte:199-201`.
**Change:** Remove the `onMount(() => fetchAnalytics())`. Keep the `$effect` and wrap the call in `untrack(() => fetchAnalytics())` so only `dateRange` reactivity fires it.

### 6.4 SvelteKit — add `handleError` to `hooks.server.ts`
**File:** `frontend/src/hooks.server.ts`.
**Change:** Add export of `HandleServerError` that mirrors the client's pattern (errorId + GA4 reporting + structured log + optional Sentry).

### 6.5 SvelteKit — add `+error.svelte` for top-traffic route groups
**Files:** create `frontend/src/routes/dashboard/+error.svelte`, `frontend/src/routes/checkout/+error.svelte`, `frontend/src/routes/account/+error.svelte`, `frontend/src/routes/auth/+error.svelte`, `frontend/src/routes/store/+error.svelte`.
**Change:** Each renders `page.error.message` with route-appropriate recovery actions.

### 6.6 SvelteKit — replace synthetic fallback user with real fail
**File:** `frontend/src/hooks.server.ts:254-264`.
**Change:** On transient API failure, set `locals.user = null` rather than `{ id: 0, email: 'session@preserved.local' }`. Force the user to re-authenticate; this is correct for security.

### 6.7 PE7 CSS — collapse off-scale breakpoints to 9-tier
**Effort:** large but bulk-mechanical
**Change:** codemod / find-and-replace `480` → `640` (sm), `360` → `xs (320)` or `sm (640)` per visual review, `1152` → `lg (1024)` or `xl (1280)`, `992` → `lg (1024)` or `xl (1280)` per case, `1200` → `xl (1280)`.
**Verification:** `grep -rE "@media[^{]*(min-width|max-width):" frontend/src` — no values outside the 9-tier scale.

### 6.8 PE7 CSS — convert non-logical to logical properties
**Change:** codemod `margin-(left|right): X` → `margin-inline-(start|end): X`; `padding-(left|right)` → `padding-inline-(start|end)`; `text-align: (left|right)` → `text-align: (start|end)`. Spot-check RTL behavior on a sample page.

### 6.9 PE7 CSS — convert hex/rgb tokens to OKLCH
**Effort:** large
**Files:** `lib/options-calculator/styles/calculator-theme.css`, `lib/styles/design-tokens.css`, `lib/styles/tokens/colors.css`, `lib/styles/tokens/shadows.css`, `Hero.svelte` HSL/RGB tokens.
**Change:** convert each color to OKLCH (color picker tool: Tailwind's `oklch()` recommendations or `@csstools/postcss-oklab-function`).

### 6.10 PE7 CSS — wrap orphan stylesheets in `@layer`
**Files:** see AUDIT_REPORT §6 Finding 5 (19 files).
**Change:** Wrap each file's content in `@layer base { ... }`, `@layer components { ... }`, or `@layer utilities { ... }` per content type.

### 6.11 PE7 CSS — convert px font-sizes to clamp()
**Effort:** medium-large (1755 sites; codemod with manual review)
**Change:** for body/heading text, replace `font-size: Npx` with `font-size: clamp(min, fluid, max)`. Icon-size px (`simpler-icons.css`) and 1px borders are exempt.

### 6.12 Rust — register orphan routers + start scheduler
**Files:** `api/src/routes/mod.rs`, `api/src/main.rs`.
**Change:** Either register `profile_router`, `taxonomy_router`, and call `cms_scheduler::start_scheduler(...)` from `main.rs`, OR delete those modules. Document decision.

### 6.13 Rust — upgrade reqwest 0.11 → 0.12 (Cluster E)
**Effort:** large (touches Stripe HTTP client, all reqwest builders)
**Files:** `api/Cargo.toml:76`, plus every `reqwest::Client::new()` / `.build()` call in `api/src/services/`.
**Change:** Update version. Fix builder API breaking changes (mostly the `redirect`/`tcp_keepalive` setter renames).
**Verification:**
1. `cargo build --release --manifest-path api/Cargo.toml` succeeds.
2. `cargo deny --manifest-path api/Cargo.toml check` — 4 RUSTSEC advisories now resolved.
3. Inspect `Cargo.lock` — `hyper 0.14` and `tower 0.4` should be gone (only `hyper 1.x`, `tower 0.5+`).

### 6.14 Replace `@lucide/svelte` with Iconify equivalent
**File:** `frontend/package.json`.
**Change:** `pnpm remove @lucide/svelte`. Install `@iconify-json/ph` (Phosphor) or `@iconify-json/carbon`. Replace usages — codemod via grep + manual icon-name map.

---

## Phase 7 — Dead code, dependencies, config integrity (effort: M, ~3–5 hours)

### 7.1 Delete orphan `lib/` files (53 unimported)
**Effort:** medium (verify with `grep -rn` per file before delete)
**Change:** For each candidate, run `grep -rn '<basename>' frontend/src` to confirm zero importers; comment out the file's exports first (per session rule: comment-out, don't delete on uncertain blast radius), or git-rm if confident.

### 7.2 Delete unused `errors.rs` and `validation.rs` helpers OR wire them up
**Files:** `api/src/utils/errors.rs:66,80,106,129,139,197,236,245,254,266,279`; `api/src/middleware/validation.rs:78,86,127,132`.
**Change:** Decision required — either remove (if truly dead) or integrate into route handlers. Integration is the better choice; the helpers are well-designed.

### 7.3 Fix Dockerfile pinning
**File:** `api/Dockerfile:5,34`.
**Change:**
- Line 5: `FROM rust:latest` → `FROM rust:1.87-bookworm AS builder` (pin minor).
- Line 34: `FROM debian:trixie-slim` → `FROM debian:bookworm-slim` (stable channel).

### 7.4 Remove dead `wrangler.toml [vars]`
**File:** `frontend/wrangler.toml:10-13`.
**Change:** Remove the `VITE_*` keys (they're build-time, not runtime). If runtime vars are needed, rename to `PUBLIC_*` and read via `$env/dynamic/public`.

### 7.5 Document required Fly.io secrets
**File:** `api/fly.toml`.
**Change:** Add a comment block listing all required Fly secrets: `DATABASE_URL`, `JWT_SECRET`, `CORS_ORIGINS`, `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `STRIPE_*`, etc. Optionally: refuse to start if `CORS_ORIGINS` is unset (paired with Phase 3.1).

### 7.6 Tighten tsconfig
**File:** `frontend/tsconfig.json`.
**Change:** Add `"noUncheckedIndexedAccess": true`, `"exactOptionalPropertyTypes": true`. Remove the stale `exclude` paths (`Do's`, `SimpletTGWatkins`, `blogsample`).
**Verification:** `pnpm --filter revolution-svelte check` — fix any new violations.

### 7.7 Deprecate dead transitive DB types
**File:** `frontend/package.json`.
**Change:** Investigate `@types/{mysql,pg,pg-pool,tedious}` source — likely a vitest dependency. Add `pnpm.overrides` to remove if possible.

### 7.8 Fix Playwright config to use pnpm
**File:** `frontend/playwright.config.ts:43`.
**Change:** `command: 'npm run dev'` → `command: 'pnpm dev'`.

### 7.9 Fix docker-compose redis dependency
**File:** `docker-compose.yml:82`.
**Change:** `condition: service_started` → `condition: service_healthy`.

### 7.10 Stop ESLint from ignoring tests/scripts
**File:** `frontend/eslint.config.js:133-145`.
**Change:** Remove `tests/**` and `scripts/**` from `ignores` array. Run lint, fix or suppress per case.

---

## Phase 8 — Test coverage gaps (effort: L, ~20–40 hours; parallelizable)

### 8.1 Provision local Postgres `test` role for Rust integration tests
**Effort:** small
**Change:** Update `docker-compose.yml` (or new `docker-compose.test.yml`) to provision a Postgres role `test` with database `test_db`. Document in `docs/development/LOCAL_DEV.md`. Optionally: wrap `cargo test` in a Make target that asserts the role exists before invoking.
**Verification:** `cargo test --manifest-path api/Cargo.toml --all-targets` → 115 previously-blocked tests now run.

### 8.2 Move 4 pure-unit tests out of integration binary
**File:** `api/tests/explosive_swings/{stats_test.rs,alerts_test.rs}`.
**Change:** Move the TOS formatter and profit-factor calculation tests into `#[cfg(test)] mod tests` blocks inside `api/src/services/...` so they don't depend on the integration test harness.

### 8.3 Cover security-critical untested utilities
**Files:** `frontend/src/lib/utils/sanitization.ts`, `frontend/src/lib/utils/safe-math-parser.ts`.
**Change:** Add Vitest unit tests with property-based / fuzz tests. These are the highest-priority gaps — they replace `eval()` and guard XSS.

### 8.4 Cover `lib/api/auth.ts`, `client.svelte.ts`
**Effort:** medium
**Change:** Mock `fetch` via Vitest's `vi.spyOn(global, 'fetch')`. Test login, refresh, token-expiry, retry-on-401, queue ordering.

### 8.5 Cover `routes/auth.rs` + `routes/oauth.rs` in Rust integration
**Dependency:** 8.1 done first.
**Change:** Add new integration tests for login, register, password-reset, JWT refresh, OAuth callback flows.

### 8.6 Re-enable or delete the 32 `it.skip` tests
**Files:** `frontend/src/lib/components/cms/blocks/__tests__/{ButtonBlock,GalleryBlock,CalloutBlock,VideoBlock}.test.ts`.
**Change:** For each `it.skip` / `describe.skip`, either fix the underlying issue, document why permanently skipped (e.g. "jsdom can't compute custom properties"), or delete the test.

### 8.7 Investigate the 2 ChartBlock/ColumnsBlock failures
**Files:** `frontend/src/lib/components/cms/blocks/__tests__/{ChartBlock,ColumnsBlock}.test.ts`.
**Change:** Both fail because jsdom doesn't compute CSS custom properties. Either swap `toHaveStyle` for direct attribute checks, or migrate to a real-browser test with Vitest's browser mode / Playwright.

---

## Phase ordering & dependencies summary

```
Phase 1  ──┬─→ Phase 2 (frontend lint+a11y, mostly orthogonal)
           ├─→ Phase 3 (rust correctness, orthogonal)
           ├─→ Phase 4 (contract — depends on Phase 1.1's cookie fix)
           └─→ Phase 5 (lint+format; runs in parallel with anything)

Phase 6 ──→ Phase 7 (dead code; once 6 lands, more code becomes safely deletable)
                    ──→ Phase 8 (test coverage; depends on infra in 7.1)
```

---

## Verification checklist (run after each phase)

```bash
# Frontend
pnpm --filter revolution-svelte check                # 0/0/0
pnpm --filter revolution-svelte test:unit            # all green except 2 known jsdom limits
pnpm --filter revolution-svelte exec eslint . --max-warnings=0
pnpm --filter revolution-svelte exec prettier --check .

# Backend
cargo check --manifest-path api/Cargo.toml --all-targets
cargo clippy --manifest-path api/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path api/Cargo.toml --all -- --check
cargo deny --manifest-path api/Cargo.toml check
cargo test --manifest-path api/Cargo.toml --test utils_test --test stripe_test  # no-DB

# Smoke
# Login → /admin/courses loads real data
# Login → /dashboard/<room>/courses/<slug>/player streams video
# Visit /signup, complete form → no effect_update_depth_exceeded in console
# Visit /account/sessions → no cascade
# Inspect <head> on prod build — no svelteInspector toggle, no devtools-json link
# DevTools Network tab on / — no app.css chunk loaded; marketing.css present
# DevTools cascade probe on / — body bg painted from .marketing-shell, not body inline
```

---

## Estimated total effort

| Phase | Effort | Wall time |
|---|---|---|
| 1 | M | 6–10 h |
| 2 | M | 4–6 h |
| 3 | M | 3–5 h |
| 4 | M | 3–4 h |
| 5 | S | ~1 h |
| 6 | L | 10–15 h |
| 7 | M | 3–5 h |
| 8 | L | 20–40 h |
| **Total** | | **~50–86 h** |

Phases 1–5 (the "make it correct" half) are ~17–26 hours. Phases 6–8 (the "make it conform to standards" half) account for the bulk of remaining effort.
