# Backlog

Forward-looking work captured as it surfaces during shipped batches.
Items here are intentionally not yet scoped to a specific branch — when
a batch picks them up, it pulls them out of this file into a
`MERGE_BATCH<n>.md` plan and removes the entry once shipped.

---

## Batch 4 candidates

### 0. Shrink or box `ApiError` instead of raising clippy's error-size bar
**Severity:** P3 (code health; no runtime impact)

Rust 1.98's clippy extended `result_large_err` to `async fn`, so ~150
route handlers returning `Result<T, ApiError>` began failing the
`-D warnings` gate on 2026-08-30. Re-verified on Rust 1.99.0 (2026-10-09):
removing `api/clippy.toml` still yields exactly 150 `result_large_err` errors,
so the threshold stays until this item lands. `ApiError` is 168 bytes (3×String,
2×Option<String>, Option<HashMap>), over the 128-byte default. The
dependency sweep unblocked CI with `api/clippy.toml`
(`large-error-threshold = 176`) rather than reshaping ~40 modules
mid-upgrade.

The real fix is to make the error small: box it (`Result<T, Box<ApiError>>`
serializes identically, so the JSON contract is unaffected) or collapse
`status`/`code`/`timestamp` into cheaper representations (a status enum
and a derived timestamp). Then drop `clippy.toml` back to the default and
delete the ~12 now-redundant `#[allow(clippy::result_large_err)]`
attributes still scattered through the routes.


### 1. Admin edit-coupon Stripe-coupon recreation flow
**Severity:** P1 (operators can silently desync coupons)

When admin edits discount math (percent_off, amount_off, duration),
backend creates a new Stripe coupon, flips `stripe_coupon_id` pointer on
the DB row, optionally deletes the old Stripe coupon. (Stripe Coupons
are immutable; existing subscriptions keep the discount they redeemed
per Stripe's `duration` semantics, which is the desired behavior.) The
current `update_coupon` only updates DB metadata, so an operator
changing 10% → 15% in our admin UI silently leaves the Stripe coupon at
10% and customers continue redeeming at 10%.

### 2. Coupon backfill admin tool
**Severity:** P2 (currently no rows affected; future-proof)

Add admin endpoint `POST /api/admin/coupons/:id/sync-to-stripe` that
mirrors a DB row whose `stripe_coupon_id IS NULL` into a fresh Stripe
Coupon and stores the id. Useful if a Stripe API outage during create
left a row half-mirrored, or if rows ever get imported from elsewhere.

### 3. Frontend admin coupon form fields for `duration` / `duration_in_months`
**Severity:** P1 (form is now incomplete vs. backend contract)

Add a `duration` dropdown (once / forever / repeating) and a
`duration_in_months` input (visible only when `duration=repeating`) to
the `/admin/coupons` create + edit pages. The backend now accepts and
requires these fields per Migration 062 + Batch 3.5; the frontend form
hasn't caught up yet.

---

## P0 — every SSR route returns 500 on the Cloudflare runtime (pre-existing)

**Severity:** P0 the moment maintenance mode is switched off.
**Found:** 2026-10-09, during end-to-end verification of the dependency sweep.

Running the production build in Cloudflare's own runtime
(`wrangler pages dev .svelte-kit/cloudflare`, workerd) and requesting all 62
top-level routes: 38 prerendered pages return 200, 6 redirect, and **18
server-rendered routes return 500** — `/blog`, `/login`, `/store`, `/tools`,
`/learning-center`, `/live-trading-rooms`, `/my`, `/watchlist`,
`/verify-email`, `/authors`, `/tutorials`, `/guides`, `/daily`, `/crm`,
`/embed`, `/posts`, `/chatroom-archive`, and `/auth`. The same build of
untouched `main` (5457181) produced **identical** status codes on all 62 routes,
so this predates the upgrade. In production it is masked by maintenance mode
(`hooks.server.ts` redirects everything to the prerendered `/maintenance`).

**Root cause.** `isomorphic-dompurify` has no Workers build: its package
exports are `node` (jsdom) and `default` (browser), and the workerd bundle
resolves `default` → `dist/browser.mjs`, which runs
`DOMPurify.sanitize.bind(DOMPurify)` at import time. Without a `window`,
DOMPurify (3.4.14 and 3.4.16 alike — verified) is created with
`isSupported = false` and no `sanitize`, so the module throws
`TypeError: Cannot read properties of undefined (reading 'bind')` during
init. The root layout pulls it in (`+layout` → `consent` → `TemplateEditor` →
`#lib/utils/sanitize` → `isomorphic-dompurify`), so every non-prerendered page
fails before rendering.

**Why it was not patched inside the dependency sweep.** It is a security
decision, not a version bump. DOMPurify needs a DOM; jsdom cannot run in
workerd, DOMPurify's maintainers advise against other DOM shims for
server-side sanitizing, and making `{@html}` content client-only would drop
server-rendered blog HTML (SEO) and fight hydration.

**Recommended fix.** Sanitize at the trust boundary on the server: the Rust
API sanitizes stored/served HTML with `ammonia` (html5ever-based allowlist —
the Rust ecosystem's standard), using the same allowlists as
`src/lib/utils/sanitize.ts`'s profiles, and the frontend keeps DOMPurify in the
browser as defence in depth, imported so that server bundles never evaluate it
(and `TemplateEditor` taken out of the root layout's import graph). Verify by
re-running the 62-route sweep under `wrangler pages dev` — expect zero 5xx.

## 2026-10-09 dependency-upgrade follow-ups

Surfaced while moving everything to its latest release (CHANGELOG.md
2026-10-09). Each is either a decision that needs an owner or pre-existing debt
that was out of scope for a dependency upgrade.

### A. Align the backend Stripe API version with Stripe.js `endive`
**Severity:** P2 (works today; Stripe recommends one release train)

`@stripe/stripe-js` 10 loads Stripe.js `endive`, which makes client-side
requests on API version `2026-09-30.endive`. The Rust API still pins
`2024-12-18.acacia` (`api/src/services/stripe.rs`) and sends
`payment_method_types[0]=card`, a parameter `endive` deletes from
PaymentIntents. Mixed versions are supported, but moving the backend is a
payment-flow change that must be validated end to end in Stripe test mode, so
it was not folded into the dependency bump.

### B. Re-pin Node when 26 becomes LTS (2026-10-28)
**Severity:** P3

Node 24.21.0 is the latest LTS on 2026-10-09; per `nodejs/Release`
`schedule.json`, 24 enters maintenance on 2026-10-20 and 26 becomes Active LTS
on 2026-10-28. Re-pin `.nvmrc`, `.node-version`, `frontend/.node-version`
(read by the Cloudflare Pages build), both `engines.node`, CI's
`NODE_VERSION`, and `@types/node` (24.x → 26.x) together after that date.
**Node ≥ 25 no longer bundles Corepack** (verified: the v26.11.1 tarball
contains no `corepack`; v24.21.0 does), so the `corepack enable` steps in
`ci.yml` and `deploy-cloudflare.yml` must first install it from npm
(`npm install -g corepack@0.36.0`, which supports Node ≥ 26), and
`LOCAL_DEV.md` must say the same.

### C. `@testing-library/jest-dom` types vs Vitest 5
**Severity:** P3 (type-level only; hidden by `skipLibCheck`)

jest-dom 7.0.1 (latest) augments `vitest`'s `Assertion<T>`; Vitest 5 declares
`Assertion<R, T>`, which is a TS2428 conflict that only `skipLibCheck: true`
(Kit's default) hides. Runtime matchers work. Re-check when jest-dom ships a
Vitest 5-aware release.

### D. Test-suite hygiene (pre-existing)
**Severity:** P3

- `pnpm test:coverage` fails its 80/80/75/80 thresholds at ~7% coverage on
  both Vitest 4 and 5. It is not a CI gate; either raise coverage or set
  realistic thresholds.
- `frontend/tests/unit/WeeklyHero.test.ts` imports vitest but sits outside
  `test.include` (`src/**/*.{test,spec}.{js,ts}`), so it never runs.
- `@vitest/browser-playwright` is declared (and kept current at 5.0.3) but has
  zero usages — there is no `test.browser` config. Remove it, or adopt browser
  mode deliberately.

### E. `AIGeneratedBlock.svelte` is a placeholder
**Severity:** P3 (pre-existing)

It simulates output client-side and offers a stale model list (GPT-4,
GPT-3.5, Claude 3, Claude 2). It does not call `/api/cms/ai/generate`, which
was the only live Claude integration and now runs on `claude-sonnet-5-5`.

### F. Rust edition 2021 → 2024 (deliberately not folded into the toolchain bump)
**Severity:** P3 (2021 stays fully supported by every future compiler)

Measured 2026-10-09 on Rust 1.99.0 with
`RUSTFLAGS="-W rust-2024-compatibility" cargo check --all-targets`:

| Lint | Sites | Where | Auto-fixable |
|------|-------|-------|--------------|
| `tail_expr_drop_order` | 26 | `db/mod.rs`, `jobs/{email_reminders,reconcile_stripe}.rs`, `queue/worker.rs` (×7), `routes/{admin_videos/crud,analytics,cms_ai_assist,cms_datasources/import_export,forms/public,media/bulk (×4),media/uploads,subscriptions_admin/price_changes,user/avatar,websocket}.rs`, `services/{mod,redis}.rs` | **No** — temporaries in tail expressions drop earlier; each site needs a semantic review |
| `if_let_rescope` | 5 | `routes/auth/{login,session}.rs`, `routes/forms/public.rs`, `routes/media/crud.rs`, `routes/newsletter.rs` | Yes (`cargo fix` rewrites to `match`) |
| `deprecated_safe_2024` | 8 | `tests/common/mod.rs`, `tests/explosive_swings/fixtures.rs` (`std::env::set_var`) | Wraps in `unsafe {}` — prefer passing config explicitly over mutating process env in multi-threaded tests |
| `edition_2024_expr_fragment_specifier` | 1 | `routes/admin_courses/crud.rs` | Yes |
| `rust_2024_incompatible_pat` | 1 | `routes/websocket.rs` | Yes |

A scratch-copy migration (2026-10-09) compiles and passes clippy
`-D warnings` plus the no-DB tests, but it also needs `cargo clippy --fix` to
collapse 86 nested `if`/`if let` sites into let-chains (clippy's
`collapsible_if` fires on them under 2024), and a decision on rustfmt: pin
`style_edition = "2021"` in `api/rustfmt.toml` or accept a ~280-file
reformat. The drop-order sites sit in auth, websocket, queue and
payment-reconciliation paths, which the no-DB tests do not exercise, so this
belongs in its own reviewed change with the DB-backed integration tests
(`integration_tests`, `explosive_swings_tests`) run against a real Postgres —
not hidden inside a version-bump diff.

### G. Backend dependency hygiene (optional, low risk)
**Severity:** P3

- `aws-config` is used only for `BehaviorVersion`, which `aws-sdk-s3`
  re-exports (`aws_sdk_s3::config::BehaviorVersion`); dropping it removes five
  crates from the tree.
- `api/clippy.toml` could use clippy's type-specific `large-error-ignored`
  (available since 1.93) instead of the crate-wide `large-error-threshold`,
  until item 0 lands.
- `deny.toml` allow-lists `Unicode-DFS-2016`, `MPL-2.0` and `OpenSSL`, which no
  crate in the tree uses any more (`license-not-encountered`).
- Rust 1.99's Cargo turns incremental compilation off by default when `CI` is
  set; `api/.cargo/config.toml` forces `incremental = true`. Decide whether that
  is still wanted for CI cache size and speed.
- AWS SDK for Rust 2.x is announced for November 2026. With
  `default-features = false` the crate already uses the post-2.x default
  client, but the major bump will be its own upgrade.

### H. `goto()` to routes that do not exist (pre-existing)
**Severity:** P3

Kit 3 rejects `goto()` for URLs that resolve to no page
(`navigation_route_missing`) instead of rendering a 404. These targets have no
route: `goto('/workflows/new')` (`routes/workflows/+page.svelte`),
`goto('/posts')` (`routes/posts/[slug]/edit/+page.svelte`), and
`/admin/page-builder?course=…` (a `goto` in `routes/admin/courses/+page.svelte`
plus three `<a href>` links). Each needs a product decision — build the page or
remove the entry point. (`CommandPalette`'s `/admin/blog/new` and
`SocialLoginButtons`' `/api/auth/*` were fixed on 2026-10-09.)

## SvelteKit 3 follow-ups

The repo is on SvelteKit 3 stable (`@sveltejs/kit` 3.0.1,
`@sveltejs/adapter-cloudflare` 8.0.0) as of 2026-10-09. That upgrade shipped
the former items 1 (`invalidateAll` → `refreshAll`), 2 (`json()` →
`Response.json()`; there were no `text()` calls), 3 (`replaceState` →
`replace`), 6 (re-pin off the prerelease line), 8 (the adapter's
"Reading `config.kit`" build warning — gone in adapter 8.0.0) and 9 (`version`
from `$app/env` now works in the service worker), so they are removed here.
See CHANGELOG.md 2026-10-09.

### 4. Pre-existing a11y failure: form inputs without labels — and the suite audits the wrong page
**Severity:** P2 (pre-existing, not a migration regression)

`tests/accessibility/a11y-audit.spec.ts` → "form inputs should have
associated labels" fails identically on `main` and on the 2026-10-09 upgrade
(both run the same day: upgraded 9 passed / 1 failed / 2 skipped). The
"CMS Editor" tests navigate to `/admin/blog/new`, which is not a route (the
editor is `/admin/blog/create`); unauthenticated, the request lands on
`/login`, so the whole "CMS Editor" block actually audits the **login page**,
and the unlabelled input is there. Fix the input on `/login`, then point the
suite at `/admin/blog/create` with an authenticated fixture so it audits the
editor it is named after. (On `main` the "color contrast" test also failed
that day, but as a `waitForLoadState` timeout on a cold dev server, not a
contrast violation.)

### 5. TypeScript 7
**Severity:** P2 (blocked upstream)

TypeScript 7.0.2 is npm `latest` but cannot be the project's `typescript`:
`@sveltejs/kit@3.0.1` peers `typescript ^6.0.0`, `svelte-check@4.7.6` peers
`^5.0.0 || ^6.0.0`, and `typescript-eslint@8.71.1` peers `>=4.8.4 <6.1.0`
(re-verified against the registry 2026-10-09). The only documented TS 7 route
today is side-by-side — keep `typescript@~6` and add
`@typescript/native@npm:typescript@7`, then `svelte-check --tsgo` — which is
opt-in and experimental. Revisit when the Svelte toolchain peers on 7.

### 7. Remaining Svelte-5 modernisation surface
**Severity:** P3 (all currently supported; no warnings fired)

Measured 2026-08-14 against the official [Svelte best practices](https://svelte.dev/docs/svelte/best-practices)
list. The codebase is already fully migrated on the big-ticket items —
**0** `export let`, **0** `on:` event directives, **0** `$:` reactive statements,
**0** `class:` directives, and **0** real `<slot>` or `<svelte:component>` uses
(the few textual matches are comments recording that they were replaced).

What is left, in full:

| Pattern | Count | Recommended replacement | Notes |
|---------|-------|-------------------------|-------|
| `svelte/store` imports | 28 files | classes with `$state` fields | The largest remaining item. |
| `setContext` / `getContext` | 24 calls | `createContext` | Docs prefer it for type safety. |
| `use:tilt` action | 1 | `{@attach ...}` | The other two `use:` directives are not legacy: `use:enhance` is SvelteKit's own API and `use:emblaCarouselSvelte` is the library's public interface. |

None of these emit warnings — `pnpm check:strict` and `pnpm check:a11y` are both
clean at 0 errors / 0 warnings (re-verified 2026-10-09 on Kit 3.0.1 /
Svelte 5.57.2) — so this is voluntary modernisation, not remediation.
