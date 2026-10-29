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

## 2026-10-11 options-calculator follow-ups

These are findings from the adversarial review of the Svelte 5 / SvelteKit 3
migration that were judged real but not worth folding into it. All are LOW, and
none is user-visible today unless noted. Paths are relative to
`frontend/src/lib/options-calculator/`.

### S. Cache: notifier misses read-path mutations; `schema` is optional

- **Stale stats:** `data/cache.ts` mutates the memory tier on two read paths
  without calling `notifyChange`:
  - promoting a localStorage hit (`getStorage`);
  - evicting an expired entry (`getMemory`).

  As a result, `DataSourceBadge`'s `getStats()` can under-count until the next
  `set()`.
  - **Fix:** route every memory-tier mutation through one helper.
  - **Constraint:** on read paths, notify with `queueMicrotask`. `get()` runs
    inside awaited `$derived`s, so a synchronous write there would throw
    `state_unsafe_mutation`.
- **Optional schema:** `get` and `getStorage` still accept no schema and return
  `envelope.output.data as T`. Every caller passes one today. Make the parameter
  required and delete the unchecked branch.

### T. Chain loading state belongs in `MarketDataService`

`OptionsChainViewer`'s `isLoadingChain` / `latestLoad` duplicate the service's
latest-wins `#chain` slot. The panel unmounts on tab change, so switching away
and back while a slow chain is in flight shows no spinner. Instead, expose a
reactive `isLoadingChain` from the service and set it in `fetchOptionsChain`.

### U. 3D charts: boundary instead of hand-caught WebGL errors; keyboard orbit

- **Error handling:** `charts/Surface3D.svelte` and
  `charts/GreeksOrbit3D.svelte` catch the `WebGLRenderer` constructor error
  and write `$state` from inside the attachment's effect. Instead, let it throw
  into a `<svelte:boundary>`. `GreeksPanel` already wraps the orbit in one.
- **Pointer-only controls:** both canvases are `aria-hidden` and not
  focusable, and `OrbitControls.listenToKeyEvents` is never called, so
  rotate/zoom works only with a pointer. To fix:
  - make the canvas the focus target (`tabindex="0"`, `role="application"`,
    the label moved off the `role="img"` wrapper);
  - call `controls.listenToKeyEvents(canvas)`;
  - mention Shift+arrows in the hint.

### V. `MetricCard`: impure `$derived`; description is `title`-only

- **Impure derived:** the `history` `$derived.by` appends to an outer
  `buffer`, which is a side effect. It is latent under `experimental.async`,
  since a derived can be recomputed per batch. To fix, move the buffer into the
  `bounceOnChange` attachment's nested `$effect` and draw the polyline there.
- **`title`-only description:** each Greek's description is only a `title` on
  a non-focusable div, so keyboard and touch users never see it. Render
  `InfoTooltip` beside the label, or use `aria-describedby` with an `sr-only`
  span.

### W. `InfoTooltip`: WCAG 1.4.13

A tooltip opened by hover cannot be dismissed with Escape, because the handler
is on the button. Moving the pointer onto the tooltip also hides it, because of
`pointer-events-none` and `mouseleave` on the button. To fix:

- move the Escape handler to `<svelte:window>`;
- put hover handling on the wrapper and drop `pointer-events-none`.

### X. Dialogs nothing opens yet

- **`ui/ConfirmDialog.svelte`** (no importers): it now traps Tab, but its
  `danger` variant still focuses the destructive button first. Use
  `{@attach variant === 'danger' && takeFocus}` on Cancel.
- **`education/ConceptTooltip.svelte`** (`EducationOverlay.showEntry` has no
  callers): it is presented modally but has `role="tooltip"` with focusable
  buttons and never takes focus. Make it a `role="dialog"` with
  `aria-labelledby`, the focus-on-attach pattern and `trapFocus`, like the
  export dialogs.

### Y. Smaller accessibility gaps

- **`export/ExportPNG.svelte`:** the "Capture Zone", "Aspect Ratio" and
  "Branding" headings are not tied to their controls. Add
  `role="group" aria-labelledby` with `$props.id()` ids.
- **`charts/GreeksHeatmap.svelte`:** cell values are still hover-only. A
  screen-reader summary (range and current value) was added; a full ARIA grid
  with arrow-key navigation was not.

### Z. Not yet reviewed

The adversarial review covered these clusters:

- data;
- core;
- strategy;
- 2D and 3D charts;
- `ui/`;
- education/export.

It did not cover:

- `saved/`, `growth/` and `power-user/`;
- `hero/`, the styles and `utils/animations.ts`;
- the route files and the other `utils/`.

They passed the same gates and the Chromium run, but nobody has read them
adversarially.

## Resolved 2026-10-10 — P0: every SSR route returned 500 on the Cloudflare runtime

Fixed; see CHANGELOG.md "2026-10-10". `isomorphic-dompurify` (whose browser
build throws at import time in workerd) is gone. A single sanitizer engine
(`frontend/src/lib/utils/html-sanitizer/`) runs the same allowlist on the
server (parse5) and in the browser (`DOMParser`). The root layout no longer
imports the consent UI. The Rust API sanitizes author HTML with `ammonia` on
its public read endpoints. The 62-route `wrangler pages dev` sweep that found
the 18 500s is the regression check, and `pnpm test:sanitizer:browser` checks
server/browser output parity against real Chromium.

## 2026-10-10 security follow-ups (found while fixing the P0)

The API inventory behind the P0 fix turned up these. None blocks the fix;
each is small and independent.

### I. Public `GET /api/posts` honours `?status=`

`list_posts` (`api/src/routes/posts.rs`) takes `status` from the query string
and defaults it to `published`, so `/api/posts?status=draft` lists unpublished
posts to anonymous callers. Admins already have `admin_list_posts`. Fix: pin
the public route to `published`. First check that no admin page calls the
public route with `status=`.

### J. Room search highlights do not escape the source text

`api/src/services/room_search.rs` (`ts_headline` at :349, :436, :520) wraps
matches in `<mark>` but leaves the rest of the alert/trade text raw, and the
dashboard renders it with `{@html}`. The frontend sanitizer contains it today
(standard profile). Fix at the source: HTML-escape the column before
`ts_headline`, so the only markup in the result is the `<mark>` the server adds.

### K. `GET /api/cms/settings` is public and returns the injected scripts

`head_scripts`, `body_start_scripts`, `body_end_scripts` and `custom_css` are
admin-entered and served to anyone (`api/src/routes/cms_v2.rs:605`). The
frontend reads only `maintenance_mode`. Return a public projection of the
settings instead.

### L. Other public HTML endpoints with no renderer yet

CMS v2 content (`/api/cms/content/{type}/{slug}`, `/api/delivery/content/…`),
public reusable blocks and global components, and `/api/videos/{id_or_slug}`
(`unified_videos.description`) all return author HTML unsanitized. No frontend
page renders them with `{@html}` today. Route them through `utils::html`
(`sanitize_html` / `sanitize_content_blocks`) when a renderer is added.

### M. Field/route mismatches that leave sinks rendering nothing

- Email template preview reads `body_html`; the API returns `data.html`.
- CRM template preview reads `content`; the API field is `body`.
- Popups: `PopupModal` expects `content.{type,data}`; the API sends a string.
  None of the three popup components is mounted in a route.
- The trading-room video pages call `/api/trading-rooms/{room}/videos/{slug}`,
  which the Rust API does not have.
- `/api/admin/invoice-settings` has a SvelteKit proxy but no Rust route.
- The CMS revisions router is mounted at `/api/api/cms` (double prefix,
  `api/src/routes/mod.rs:123`).
- `cms_scheduling/cron.rs:195` writes `cms_content.data`, a column the
  `CmsContent` struct does not have. Verify it against the schema.

### N. Editor-side regex sanitizer

`BlockEditor/validation.ts` exports a pattern-based `sanitizeHtml` /
`sanitizeBlock`. Nothing outside the module calls them. Delete them, or route
them through `#lib/utils/sanitize.js`, so there is one sanitizer.

### O. Prerendering depends on the API being reachable at build time

`hooks.server.ts` asks `GET /api/cms/settings` for `maintenance_mode` and fails
closed: if the API is unreachable or errors, the result is "maintenance on".
That also applies during `vite build`'s prerender. With no API, every
prerendered page is written as a 102-byte redirect stub to `/maintenance`.
This was reproduced on 2026-10-10, and it is what CI's build step produces,
since CI has no API. Skip the maintenance gate while `building` (from
`$app/env`), so static pages never depend on API state at build time; the
runtime check still guards SSR. Related: a fresh database bootstrapped from
`schema.sql` + `sqlx-seed.sql` has no `cms_site_settings` singleton row
(migration 023 inserts it). `/api/cms/settings` then returns 500, which also
trips the fail-closed path. Add the seed rows that migrations insert to the
fresh-environment bootstrap.

### P. Blog post bodies are not server-rendered

`/blog/[slug]` loads the post in `+page.ts` (used only for SEO) and then
fetches it again in the component, rendering the body inside
`{#await postPromise}`. SSR therefore emits only the pending state. Render the
body from `data` (sanitized on the server by both the API and the isomorphic
sanitizer) and drop the second fetch. That brings back server-rendered article
HTML for search engines.

### Q. Re-check `<select>` parity when parse5 adopts the relaxed select parser

Chromium already parses arbitrary elements inside `<select>`/`<option>`
(the WHATWG customizable-select parser change); parse5 8.0.1 does not. The
sanitizer drops `select` with its content, but a *malformed* select can still
end in a different place on the two parsers. Server and browser output then
differ: both sanitized, with a dev-mode hydration warning at worst. When parse5
ships the change, bump it, then delete the `<select>` exemption in
`frontend/scripts/sanitizer-differential.js` and the caveat in
`html-sanitizer/engine.ts`. `pnpm test:sanitizer:browser` must then report 0
differences with 0 exempt.

### R. The RSS/Atom feeds ignore the block shapes the editor writes

`/feed.xml` and `/atom.xml` read only fields on the block itself (`block.text`,
`block.content`, `block.url`, …). Posts saved by the editor nest their payload
under `block.content` (an object) or `block.data`. For those posts the feeds
emit empty paragraphs or `[object Object]`. The blog page handles both shapes
through `blockData()`. Reuse that mapping in both feed generators (the output
is sanitized either way).

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

### B. (Resolved 2026-10-28) Re-pinned Node to 26.13.0 Active LTS ("Lithium")

Completed on 2026-10-28 (`v26.13.0`, promoted to Active LTS on 2026-10-28):
`.nvmrc`, `.node-version`, `frontend/.node-version`, both `engines.node`
(`>=26.13.0`), CI and Cloudflare deploy workflows (`NODE_VERSION: '26.13.0'`
plus `npm install -g corepack@0.36.0` before `corepack enable`), `LOCAL_DEV.md`,
and `@types/node` (`26.6.6`). See CHANGELOG.md 2026-10-28.

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
- `@vitest/browser-playwright` is declared (and kept current at 5.0.5) but has
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
**Severity:** P3, except the first bullet (P2)

- **CI runs no `cargo deny check`.** CLAUDE.md makes it a gate, but only
  `pnpm api:lint` runs it locally; RUSTSEC-2026-0258 (published 2026-08-18) sat
  unnoticed on `main` until the 2026-10-09 sweep. Add a `cargo deny check` step
  to the backend job (e.g. `EmbarkStudios/cargo-deny-action`, pinned).
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

The repo is on SvelteKit 3 stable (`@sveltejs/kit` 3.1.0,
`@sveltejs/adapter-cloudflare` 8.0.1) as of 2026-10-28. The 2026-10-09 upgrade
shipped the former items 1 (`invalidateAll` → `refreshAll`), 2 (`json()` →
`Response.json()`; there were no `text()` calls), 3 (`replaceState` →
`replace`), 6 (re-pin off the prerelease line), 8 (the adapter's
"Reading `config.kit`" build warning — gone in adapter 8.0.0) and 9 (`version`
from `$app/env` now works in the service worker), so they are removed here.
See CHANGELOG.md 2026-10-09 and 2026-10-28.

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

TypeScript 7.0.2 is npm `latest` (`6.0.3` is the latest 6.x release) but cannot
be the project's `typescript`: `@sveltejs/kit@3.1.0` peers `typescript ^6.0.0`,
`svelte-check@4.7.7` peers `^5.0.0 || ^6.0.0`, and `typescript-eslint@8.72.1`
peers `>=4.8.4 <6.1.0` (re-verified against the registry 2026-10-28). The only
documented TS 7 route today is side-by-side — keep `typescript@~6` (`6.0.3`)
and add `@typescript/native@npm:typescript@7`, then `svelte-check --tsgo` —
which is opt-in and experimental. Revisit when the Svelte toolchain peers on 7.

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
clean at 0 errors / 0 warnings (re-verified 2026-10-28 on Kit 3.1.0 /
Svelte 5.57.3 / svelte-check 4.7.7) — so this is voluntary modernisation, not remediation.
