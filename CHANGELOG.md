# Changelog

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.

All notable changes to this project. Format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); we don't strictly adhere to SemVer because the product isn't a published library.

## [Unreleased] — 2026-06-07 — Tailwind-to-scoped-CSS Svelte 5 migration ledger

Branch: `codex/tailwind-to-page-css-migration`. This pass tracks the ongoing one-large-file-at-a-time conversion away from Tailwind-era dynamic markup and legacy Svelte binding patterns toward scoped CSS and current Svelte 5 idioms. Evidence source: Svelte MCP docs/autofixer plus repeatable repo scans for `@apply`, `@reference`, legacy `class:` directives, interpolated class strings, and dynamic style strings. Current hard evidence: `@apply/@reference` is at **0 Svelte files**; the broader dynamic class/style sweep still reports **422 Svelte files** and is tracked in `todo.md`.

### Completed migration slices

- `c536dda64` Clean site health background classes.
- `0032efc91` Modernize admin settings class composition.
- `65fff3755` Modernize board detail dynamic styling.
- `92a1e19fa` Modernize abandoned cart dynamic styling.
- `cefa85b36` Modernize automation detail styling.
- `6e018e80e` Modernize contact detail class handling.
- `eb42c7876` Modernize datasource page dynamic styling.
- `1b489ef0c` Modernize CRM deals dynamic styling.
- `26ec8d617` Modernize admin dashboard class composition.
- `d47db2644` Modernize admin blog status classes.
- `4858ad4cd` Modernize asset manager dynamic styling.
- `2bf007c46` Modernize block editor class composition.
- `0fbca9d03` Modernize SPX alert page dynamic styling.
- `bce717ccf` Modernize navbar dynamic styling.
- `36a2dc8bc` Modernize day trading room dynamic styling.
- `df8b28c9f` Modernize VideoEmbed dynamic styling.
- `2393b464d` Modernize small accounts page styling.
- `c4d42d017` Modernize swing trading page styling.
- `90505bcec` Modernize block settings panel styling.
- `e0108c4f8` Modernize member form modal styling.
- `9d7c1d783` Modernize admin media page styling.
- `44f033fb1` Modernize about page icon class styling.
- `c1c018176` Modernize explosive swings page styling.
- `83eb8e388` Modernize past members page styling.
- `38674e355` Modernize global component library styling.
- `da34af91e` Modernize memberships page styling.
- `2aaf57377` Modernize admin connections page styling.
- `4b254bb8f` Modernize CRM templates toast styling.
- `6a3a8fba3` Modernize CRM lead detail styling.
- `9b267c75a` Modernize course detail drawer styling.
- `744b59b8f` Modernize admin categories page styling.
- `fe9559197` Modernize high octane scanner badges.
- `52ea78418` Modernize hero binding syntax.
- `326f0c98d` Modernize popup creation preview styles.
- `356d12e48` Modernize popup modal styling bindings.
- `dc57e424d` Modernize weekly publish modal bindings.
- `768820da1` Modernize gallery block bindings.
- `25082f7ce` Modernize mission page pillar classes.
- `0c354db3e` Modernize video upload modal reactivity.
- `325ab08c7` Modernize SEO analyzer bindings.
- `0847a8eb1` Modernize dashboard sidebar bindings.
- `41199e6ca` Modernize scheduling panel bindings.
- `80df09e4c` Modernize CRM leads page bindings.
- `0842c9246` Modernize member detail drawer bindings.
- `8b4426860` Modernize courses page bindings.
- `5b206ad7b` Modernize consent settings bindings.
- `8bd9e9f6b` Modernize weekly hero bindings.
- `2468a2977` Modernize admin members page bindings.
- `2514262c4` Modernize email campaigns page bindings.
- `0783ba210` Modernize automation edit page bindings.
- `b7f44f88b` Modernize churned members page bindings.
- `b36701519` Modernize image block bindings.
- `94a7b4fee` Modernize day trading course page bindings.
- `7ac82f449` Modernize revision history bindings.
- `09b4acb94` Modernize subscription drawer bindings.
- `f53890721` Modernize ETF resource page bindings.
- `e2d09c9d7` Modernize dashboard page bindings.
- `9ce0b653c` Modernize login form attachments.
- `dfbc97d2f` Modernize blog create page bindings.
- `2065a5c3d` Modernize CRM sequences page bindings.
- `b765a465a` Modernize AI assistant bindings.
- `ffaaef32e` Modernize admin toolbar bindings.
- `f31b10b30` Modernize form field renderer bindings.
- `1144f654b` Modernize countdown timer bindings.
- `bbb70be8e` Modernize media analytics page bindings.
- `a1e519fd4` Modernize admin analytics page bindings.
- `81f1400e5` Modernize weekly video uploader bindings.
- `80a0e90ea` Modernize image uploader bindings.
- `8c3b94903` Modernize preset picker bindings.
- `7e02c02f9` Modernize mentorship page accordion bindings.
- `baf73647e` Modernize admin watchlist dynamic bindings.
- `de5fd0261` Modernize checkout thank-you status binding.
- `120236fe1` Modernize stock indexes FAQ binding.
- `6fb86bdea` Modernize register page lifecycle bindings.
- `9b4cae95e` Modernize course form modal bindings.
- `adf81f179` Modernize webhook edit page bindings.
- `3eab58bf6` Modernize admin courses page bindings.
- `5ca370c9f` Modernize dashboard indicator detail bindings.
- `46dde3b88` Modernize automation create page bindings.
- `77d62a1b0` Modernize day trading start accordion bindings.
- `370ebf85f` Modernize swing trading start accordion bindings.
- `a018dded3` Modernize swing trading course reveal bindings.
- `8aa315e88` Modernize CRM contact status binding.
- `544da3d89` Modernize consent banner designer class bindings.
- `bec3d83dd` Modernize image crop modal bindings.
- `96b8acea7` Modernize segment detail drawer bindings.
- `d655efd6c` Modernize virtual block list bindings.
- `f63c74f36` Modernize Bing SEO status class.
- `75e35ea39` Modernize member analytics chart bindings.
- `8315a5fb7` Modernize subscription form modal bindings.
- `15b87f343` Modernize admin SEO dashboard bindings.
- `87e1f65b7` Modernize template editor tab bindings.
- `34db914e1` Modernize small account start page bindings.
- `f9cdf559c` Modernize day trading learning center styles.
- `84bdc227c` Modernize admin member subscriptions bindings.
- `d845ca368` Modernize trade entry manager bindings.
- `fef2652e6` Modernize small account trader store detail page.
- `f18c50868` Modernize day trading trader store detail page.
- `1d0a50318` Modernize CRM webhooks page bindings.
- `01c065831` Modernize author block bindings.
- `41fefb1f1` Modernize media upload hub bindings.
- `a5507f58d` Modernize account sessions bindings.
- `37b3ff37f` Modernize chart block bindings.
- `34368c15a` Modernize webhook creation bindings.
- `64d634ee2` Modernize service members page bindings.
- `40e946cf2` Modernize popup display bindings.
- `a06452a04` Modernize small account learning center bindings.
- `dba7c532c` Modernize trade alert modal bindings.
- `7ca2e23ac` Modernize testimonial block bindings.
- `1d8dfea27` Modernize popup renderer styling.
- `2af7de18a` Modernize video analytics dashboard bindings.
- `505cef061` Modernize CRM tags page bindings.
- `86b2055d0` Modernize checklist block class bindings.
- `751cffbae` Modernize blog article block styling.
- `ca1666775` Modernize class videos bindings.
- `3e7d9279b` Modernize daily videos page bindings.
- `45a7af0c5` Modernize video uploader bindings.
- `69ebbea74` Modernize multi-step form renderer bindings.
- `0cdcf4141` Modernize duplicate detector bindings.
- `ade2908e2` Modernize risk disclaimer block bindings.
- `5954573fd` Modernize admin member detail bindings.
- `2cf3e4eb5` Modernize consent banner bindings.
- `e298a5ca0` Modernize behavior analytics bindings.
- `595a7d0d9` Modernize abandoned carts bindings.
- `681f1cae4` Modernize block error boundary classes.
- `c5db04ef7` Modernize performance dashboard bindings.
- `ac9ed96ed` Modernize media library selection bindings.
- `23cc6d364` Modernize smart links bindings.
- `dd2502e26` Modernize service connection status styles.
- `6b4e8f310` Migrate form styler classes to scoped CSS.
- `acbbe6dcf` Modernize consent dashboard category badges.
- `03b6d6d4e` Modernize explosive swings dashboard bindings.
- `2e4bd3293` Modernize SEO reports page bindings.
- `f09e833b9` Modernize admin indicators filters and status classes.
- `1d048060d` Modernize video sitemap dynamic styling.
- `992ecc991` Modernize form AI assistant dynamic styling.
- `b4578c7ec` Modernize cookie consent class composition.
- `e45a9b79d` Modernize alerts page service card classes.

### Validation standard for each completed slice

- Svelte MCP `svelte_autofixer` runs on every edited `.svelte` file until it returns `issues: []` and `suggestions: []`, unless a documented false positive is intentionally preserved.
- Targeted file scan must return no remaining `@apply`, `@reference`, legacy `class:`, interpolated class strings, or interpolated dynamic style strings for the file being completed.
- Full gates run before commit: `pnpm --dir frontend check`, `pnpm --dir frontend format:check`, `git diff --check`, `pnpm --dir frontend lint`, and `pnpm --dir frontend build`.

## [Unreleased] — 2026-06-02 — Dependency upgrade: full toolchain + package refresh to latest (Node LTS pinned), breaking-change migrations, zod→valibot consolidation

Every library, package, and dependency bumped to its latest version as of 2026-06-02 (verified against npm / docs.rs / crates.io — no assumptions), **except Node**, which is pinned to the latest LTS. All major-version bumps were migrated through their breaking changes; the backend quality gate is green throughout (`cargo check --all-targets`, `cargo clippy --all-targets -D warnings`, `cargo fmt`, and the no-DB tests `utils_test` 18/18 + `stripe_test` 19/19).

### Toolchain & runtimes

- **Node** → **24.16.0** (latest LTS "Krypton", released 2026-05-21); `.nvmrc`, both `package.json#engines`, and the CI `NODE_VERSION` env updated. **pnpm** stays the sole package manager, pinned `11.5.1` (`packageManager` field is the single source of truth; CI/deploy comments corrected from the stale `11.1.2`).
- **Rust** → **1.96.0** (current stable); `rust-toolchain.toml` channel and CI `RUST_VERSION` updated. Local rustup hygiene: removed 5 stale toolchains and reinstalled `nightly` cleanly, resolving the `clippy-preview … bin/cargo-clippy` install conflict and the failed `nightly` update.

### Frontend — notable package bumps (`frontend/package.json`)

- Build/test core: **vite 6→8**, **vitest 3→4** (+ `@vitest/browser-playwright`, `@vitest/coverage-v8` 4), **typescript 5→6**, **eslint 9→10** (+ `eslint-config-prettier`, `eslint-plugin-svelte`, `typescript-eslint`), **@eslint/js 10.0.1** (note: `@eslint/js` latest is 10.0.1 while `eslint` is 10.4.1 — separate release lines), **svelte 5.56.1**, **@sveltejs/kit 2.62.0**, **@sveltejs/vite-plugin-svelte 6→7**, **@sveltejs/adapter-cloudflare 7.2.8**, **wrangler 4.97.0**, **prettier 3.8.3**, **prettier-plugin-svelte 3→4**, **playwright 1.60.0**.
- App deps: **zod removed** (see below), **valibot 1.4.1**, **@stripe/stripe-js 7→9.7.0**, **@anthropic-ai/sdk 0.100.1**, **three 0.184** (+ `@types/three`), **gsap 3.15.0**, **web-vitals 4→5.3.0**, **typed.js 2→3**, **isomorphic-dompurify 2→3.15.0**, **dompurify 3.4.7**, **bits-ui 2.18.1**, **tailwind-variants 1→3.2.2**, **tailwind-merge 3.6.0**, **@types/node 25.9.1**.
- Internal `svelte-click-to-source` package bumped in lockstep (vite 8, typescript 6, tsup 8.5.1, svelte 5.56.1) and its `vite` peer range widened to `^8`.

### Backend — Rust crate bumps + breaking-change migrations (`api/Cargo.toml`)

- **sqlx 0.8 → 0.9** — the big one. 0.9 makes dynamic SQL a compile error: `query`/`query_as`/`query_scalar`/`query_*_with`/`raw_sql` now require `&'static str` or an explicit `AssertSqlSafe` wrapper. **118 call sites** across ~40 route/service files were audited (every one interpolates only whitelisted sort columns/directions or enum-mapped identifiers, with all user values bound as `$N`) and wrapped with `sqlx::AssertSqlSafe`. No dynamic user input reaches a query string.
- **PE7 Resilient Connection Retry** — `Database::new()` and `RedisService::new()` now implement exponential backoff retry (10 attempts, 500ms→8s backoff for DB; 8 attempts, 300ms→5s for Redis). This prevents the brittle "pool timed out while waiting for an open connection" crash when docker-compose services race during startup. Structured tracing events (`db_connect_initial_failed`, `db_connect_retry`, `db_connected_after_retry`, `redis_*` equivalents) provide observability. Connection verified with `SELECT 1` (DB) and `PING` (Redis) before returning.
- **rand 0.8 → 0.10** — `thread_rng()` → `rng()`, `.gen()/.gen_range()` → `.random()/.random_range()`, the `Rng` trait → `RngExt`, and `rand::distributions` → `rand::distr`. Fixed across 6 files (`utils/mod.rs`, `services/mfa.rs`, oauth/connections crypto, admin member crud).
- **reqwest 0.12 → 0.13** — the `rustls-tls` feature is now `rustls`, and `form`/`query` moved behind their own features; feature list updated accordingly.
- **jsonwebtoken 9 → 10** — verified compatible with the existing JWT helpers (the no-DB JWT test suite, incl. alg-none rejection / expiry / type segregation, passes).
- **axum-extra 0.10 → 0.12.6**, **axum pinned 0.8.9** — axum 0.8 dropped the blanket `Option<T>: FromRequestParts`; `ConnectInfo` only implements `FromRequestParts`, so `Option<ConnectInfo<SocketAddr>>` no longer compiles as a handler arg. Added a small infallible **`ClientAddr`** extractor (`routes/auth/helpers.rs`) that reads the `ConnectInfo`/`MockConnectInfo` extension and yields `Option<SocketAddr>`, preserving the exact prod/test rate-limit semantics; migrated the 4 auth handlers (register, login, forgot/reset-password).
- Other bumps: **tokio 1.52.3**, **tower-http 0.6.11**, **redis 0.29 → 1.2.2**, **rust_decimal 1.42**, **uuid 1.23**, **regex 1.12**, **thiserror 1 → 2**, **bcrypt 0.18 → 0.19**, **aws-sdk-s3 1.135 / aws-config 1.8**, **meilisearch-sdk 0.28 → 0.33**, **csv 1.4**, **axum-test 14 → 20** (dev), **subtle 2.6**, **lazy_static 1.5**.
- Lint: clippy 1.96 newly fires `manual_async_fn` on the hand-written `FromRequest(Parts)` impls (User/OptionalUser/AdminUser/SuperAdminUser/ValidatedJson) whose explicit `-> impl Future + Send` bound is intentional; suppressed with a targeted `#[allow(clippy::manual_async_fn)]` on each rather than reflowing the bodies.

### Tailwind v4 utility renames

Updated **63 deprecated class usages across 22 files** to the v4 names the linter flags: `bg-gradient-to-*` → `bg-linear-to-*`, `bg-[size:X]` → `bg-size-[X]`, `[mask-image:X]` → `mask-[X]`.

### Validation library consolidation — zod removed, valibot standardized

The repo had **both** `zod` and `valibot` installed. Audit: **valibot** is the real validation layer (13 active files — every `*.remote.ts` remote function + `lib/shared/schemas/`), while **zod** appeared in only **2 files** (`lib/components/blog/BlockEditor/validation.ts`, `lib/options-calculator/data/schemas.ts`). Both were **fully migrated to valibot** (zod `z.*` → valibot `v.*`: `z.object`→`v.object`, `.max()`→`v.pipe(…, v.maxLength())`, `.optional()`→`v.optional()`, `z.enum`→`v.picklist`, `.refine`→`v.check`, `z.lazy`→`v.lazy`, `.safeParse`→`v.safeParse`, `z.infer`→`v.InferOutput`, `.passthrough()`→`v.looseObject`, etc.), and `zod` was removed from `frontend/package.json` (now only a transitive dep of `@anthropic-ai/sdk`). `svelte-check` passes with **0 errors**.

### Build — Cloudflare deploy unblocked

Two fixes for the failing Cloudflare Pages build: (1) **`pnpm-lock.yaml` regenerated** so it matches the updated manifests — the build failed with `ERR_PNPM_OUTDATED_LOCKFILE` because the lockfile hadn't been re-synced after the package bumps; `pnpm install --frozen-lockfile` (the exact CI command) now passes. (2) **Node version corrected to a real release** — the original `.nvmrc` pointed at `24.17.0`, which **does not exist** (latest 24.x LTS is `24.16.0`, released 2026-05-21), so Cloudflare's nvm silently fell back to its default Node 22.16.0 and emitted an engine warning. Set `.nvmrc` + a new `.node-version` (Cloudflare reads either) to `24.16.0`.

### Dev tooling — `pnpm dev:reset` for major version bumps

Added `dev:reset` script to `package.json` (`pnpm dev:reset`). When Docker images upgrade across major versions (Postgres 16→17, Meilisearch 1.7→1.14), the **on-disk data format** becomes incompatible and containers enter a crash loop (`FATAL: database files are incompatible`, `database version (X) is incompatible with your current engine version (Y)`). This script automates the recovery: stops services, wipes the named volumes (`postgres_data`, `meili_data`), restarts fresh containers, and prints confirmation. Use whenever `pnpm dev:all` fails with version-mismatch errors after a dependency upgrade.

## [Unreleased] — 2026-06-02 — Svelte audit & optimization: measured CLS fixes, breakpoint conflicts, Svelte 5 idioms, remote functions, Cloudflare build fix

Branch: `claude/svelte-audit-optimization-gJmnH`. A multi-pass audit of every member/admin (and marketing) Svelte page driven by the Svelte MCP tool, with **evidence-based** CLS work measured via a purpose-built probe (browser Layout Instability API — the same metric Google's CLS uses) rather than assumptions. All quality gates green throughout: `pnpm check` **0 errors / 0 warnings** (4,858 files at the end); production `vite build` green; `svelte-autofixer` `issues: 0` on every modified component.

### Build — Cloudflare production build unblocked (P0)

**BUILD-1 — Invalid `@media` conditions broke the Cloudflare/lightningcss build.** `vite build` failed with `SyntaxError: [lightningcss minify] Invalid media query` on `@media (max-width: calc(var(--breakpoint-lg) - 1px))`. CSS custom properties and `calc()` are not valid inside `@media` feature values (they can't be resolved at the cascade level a media query is evaluated). 37 such conditions — Tailwind v4 `--breakpoint-*` vars used in raw media queries — across 10 admin files (`admin/+layout.svelte`, `admin/+page.svelte`, `admin/crm/+page.svelte`, `admin/members/+page.svelte` + `_components`, `admin/settings/+page.svelte`, and the admin `ModuleFormModal`/`SubscriptionFormModal`/`StatCard`). Each replaced with the computed literal using Tailwind v4 defaults (sm 640 / md 768 / lg 1024 / xl 1280 / 2xl 1536), e.g. `calc(var(--breakpoint-lg) - 1px)` → `1023px`, `calc(var(--breakpoint-2xl) - 136px)` → `1400px`, `var(--breakpoint-md)` → `768px`. Verified the production build (lightningcss minify) now passes. (commit `e347f6e`)

### CLS — measured layout-shift fixes (hard evidence, not assumptions)

**CLS-1 — Dashboard sidebar pop-in: measured 0.1215 → 0.0016 (~76×).** The dominant `/dashboard` shift was `DashboardSidebar`'s trading-room/mentorship/scanner nav populating from a client `onMount` fetch — the `<ul>` grew and shoved the footer up 656px. Fixed by pre-fetching the membership list in `dashboard/+layout.server.ts` so the sidebar renders on first paint. Self-review with a hostile mock (interleaved types + an `expiring` membership) then exposed two residual shifts (sidebar re-order; an expiring row disappearing after the client fetch) — fixed with a stable category-sort on the SSR list and aligning the client filter to `active||expiring`. Verified the prod auth path (the `/api` proxy reads `rtp_access_token` and sets `Authorization: Bearer`; SvelteKit's load `fetch` forwards the cookie). (commit `54022af` + correctness hardening)

**CLS-2 — Marketing hero/sections: `/about` 0.4011 → 0.** Marketing content was gated behind `{#if isVisible}` / `{#if mounted}` with the boolean starting **false at SSR** and flipped true in `onMount`, so the server/first paint rendered the sections **empty** and the content popped in. Initialised the gate boolean **`true`** so content is present in the SSR HTML (reserves its space); the `in:` reveal transitions are opacity/transform (compositor-only, no layout shift) and still play on client navigation. Applied to `/about` (6 sections; dropped the redundant `onMount` flip) and 9 shared marketing section components (CTA, Testimonials, Courses, Indicators, AlertServices, TradingRooms, Mentorship, Why, LatestBlogs) — matching the team's own `MarketingFooter` fix. NB: this was **not** the hamburger menu (its scroll-lock is already shift-safe via `app.html` `scrollbar-gutter: stable`). A residual homepage _scroll_ CLS (~0.25, absolute decorations/card images settling) is logged for follow-up. (commit `258632b`)

**CLS-3 — Skeleton loaders that reserve layout footprint.** `/dashboard` home now renders a theme-matched skeleton in the membership-card grid (was header-only → 4 sections popping in). The shared `LoadingState` skeleton was promoted to `$lib/components/dashboard/` and applied to 5 explosive-swings sub-routes + `small-account-mentorship/favorites` + `classes` (bare spinners → grids matching the real layout). Admin top list pages (`users`, `members`, `products`) got `SkeletonLoader` table-row/card skeletons (dark-slate, matching the admin theme). (commits `9af9dd3`, `c8c8f54`, `59fe41e`, `ee06898`)

### Responsive — breakpoint conflicts eliminated, SSOT documented

**BP-1 — Boundary double-fire across member/admin.** The canonical breakpoints (640/768/1024/1200/1280/1440/576/992) were used as **both** `max-width: Npx` and `min-width: Npx`, so at the exact boundary pixel both the mobile and desktop rules fired (order-dependent overlap → shift). Made every `max-width` side exclusive (`-0.02px`, the Bootstrap 5 convention) across 390 `@media` conditions / 295 files; element `max-width` rules (semicolon form) untouched. Also fixed the `480px`/`360px` orphan overlaps (37 files). Promoted the dead `--bp-*` comments in `design-tokens.css` to an authoritative SSOT documenting the scale + the no-conflict boundary rule. (commits `323e166`, `b562afc`)

### Svelte 5 — idiomatic correctness (MCP autofixer-driven)

**SV5-1 — Reactive collections use `SvelteSet`/`SvelteMap`.** Plain `Set`/`Map` held in `$state` are not proxied, so in-place `.add/.delete/.clear` were non-reactive (stale selection UI), often "fixed" with no-op `x = x` self-assignments. Converted every stateful collection across 25 member/admin files to `SvelteSet`/`SvelteMap` from `svelte/reactivity`. Mutate-only vars use bare `new SvelteSet()`; reassigned vars keep `$state` (documented — dropping it, as the autofixer naively suggests, would break reassignment reactivity). `useTrades.svelte.ts`'s `realTimePrices` reverted to plain `Map` in `$state` (reassign-only via the price-feed callback — `SvelteMap` was a false positive that also broke the type). (commits `4e3abc5`, `43171d2`, `ab57ac7`)

**SV5-2 — Mount-once `$effect` → `onMount`.** Triaged every data-loading `$effect` one-by-one; migrated 14 genuinely mount-once loaders (coupons, courses, email/templates, media, orders, settings, subscriptions, seo/404s, seo/keywords, members/subscriptions, + the init effect in users/create & courses/create) to `onMount`, preserving teardowns. **Left reactive reload-on-filter/param effects as `$effect`** (products, blog, crm/\* lists, etc.) — converting them would break reload. Documented the false positives (timer/subscription `$effect`s with teardown; `SvelteDate` for formatting). (commits `32e8ad4`, `763e1a2`)

**SV5-3 — Literal `{{ name }}` placeholder rendered `[object Object]`.** Svelte parsed `placeholder="… {{ name }} …"` as a mustache wrapping the object `{ name }`. Wrapped in a string expression `{'{{ name }}'}` (matching `EmailModal.svelte`) in `admin/members/churned` + `past`.

### Remote functions — SvelteKit `query` where it applies (May/June 2026 API)

**RF-1 — `getMemberships()` remote query.** New `dashboard/memberships.remote.ts` exports a typed `query` (from `$app/server`) that reads the auth cookie via `getRequestEvent()` and forwards it through the `/api` proxy, returning the category-sorted membership list. `dashboard/+layout.server.ts` now `await getMemberships()` instead of an inline fetch — type-safe, deduped, reusable from the client. Grounded in the official May/June 2026 "What's new in Svelte" posts: kit **2.61** removed `.run()` (await `query()` directly anywhere, with cross-consumer cache dedup); plus `field.as(type, value)`, `hydratable` transport, form `submit` validity boolean, and `requested(fn, limit) → { arg, query }`. Documented next step: expose `getMemberships()` to the dashboard home page to retire the duplicate `getUserMemberships()` client fetch. (commit `4475e31`)

**RF-2 — June 2026 audit of every remote function + dead-refresh cleanup.** Audited all three `.remote.ts` files against the May/June 2026 API: the query/command definitions and their consumers already use the current pattern (`await query()`/`await command()` directly, no removed `.run()`, no changed `requested()`/`enhance()` signatures), so nothing is broken by the updates. Cleanup in `explosive-swings/commands.remote.ts`: removed the vestigial server-side `query.refresh()` calls. They had **no effect** — the consumer (`page.state.svelte.ts`) reads imperatively (`await query()` → local `$state`, never `query.current`) and explicitly re-fetches after every mutation, so server refresh was dead code; and `getAlerts({page:1,limit:10}).refresh()` targeted the **wrong instance** (the client paginates with `currentPage`, and kit keys queries by argument). Dropped the now-unused `./data.remote` import and documented the path to single-flight mutations (reactive consumer + `command().updates()` + `requested(fn,limit)`) as a follow-up. Verified: check 0/0, build green. (commit `2d3f813`)

**RF-3 — Favorites converted to remote functions, end-to-end.** Evidence-driven candidate hunt across client `/api` fetches surfaced the two favorites pages (`explosive-swings/favorites`, `small-account-mentorship/favorites`) as the strongest fit: ~50 lines of duplicated `onMount` → `fetch('/api/favorites')` → `isLoading` waterfall + a `fetch DELETE` each. New `dashboard/favorites.remote.ts` exports a typed `getFavorites(roomSlug)` **query** and a `removeFavorite({id, roomSlug})` **command** (valibot-validated, `getRequestEvent().fetch` forwarding the auth cookie through the `/api` proxy — same path as before); the `Favorite` shape lives in `favorites.types.ts` because a `.remote.ts` may only export remote functions. Each route's new `+page.server.ts` does `await getFavorites(slug)` and hands the list to the page as `data.favorites` — the **proven membership-prefetch pattern** (RF-1), so the list is **server-rendered on first paint**. Both pages render `data.favorites` through an optimistic `SvelteSet`-of-removed-ids `$derived` filter (no prop-into-`$state` mirroring); removal calls the command and rolls back on failure. **Honest CLS note:** measured CLS on these routes was already `0.0016` _before_ this change (the prior `LoadingState` skeleton was sized to mirror the card layout, so the skeleton→list swap was already shift-free) and remains `0.0016` after — so this is an **architecture/SSR/UX** win (no client fetch waterfall, no skeleton flash, type-safe, ~100 LOC of duplication removed), **not** a CLS reduction; no number is claimed that the probe doesn't support. **Hard evidence:** the raw SSR HTML (curl, no JS) went from **0** `favorite-card` nodes (an earlier `query.current` warm-load attempt — `.current` does not SSR while `compilerOptions.experimental.async` is off) to **3 cards + header present** with the load+data pattern; Playwright confirms 3 cards, optimistic remove (3→2, no rollback), CLS `0.0016` on both routes. The single-flight path (`{#await}` + `command().updates()`) is documented in-file as the migration once `experimental.async` is enabled. Probe extended: `mock-backend.mjs` now serves a delayed favorites payload and `measure-favorites.mjs` measures any favorites route.

### Bugfix — dashboard sidebar crash (surfaced by the RF-3 probe)

**BUG-1 — `each_key_duplicate` blanked every mentorship/day-trading-room dashboard route.** While measuring the favorites work, the probe caught a hard client crash (`https://svelte.dev/e/each_key_duplicate`) that tore down the **entire** `/dashboard/small-account-mentorship/*` and `/dashboard/day-trading-room/*` subtree — the favorites route was merely where it surfaced. Root cause (isolated with hard evidence — the crash reproduced on `…/small-account-mentorship` _home_, which has none of the favorites code, and never on `explosive-swings`): `DashboardSidebar`'s secondary-nav `{#each secondaryNavItems as item (item.href)}` keyed by `href`, but hover-submenu **parent** items use `href: '#'` as a non-navigable placeholder. Any room whose secondary nav has ≥2 such parents — "Meet the Traders" **and** "Trader Store", true for both mentorship and day-trading-room (defined in `dashboard/+layout.svelte`) — produced two `'#'` keys → duplicate-key crash. Explosive-swings has zero `#` items, which is exactly why it was unaffected. Fixed by keying the each on `item.text` (labels are unique within a room's nav and stable). Verified: crash gone on all three probed routes (favorites + mentorship home), 3 cards render. This is a real production bug, not a mock artifact — the nav config is real.

**SV5-4 — `DashboardSidebar` expanded-submenu state was a non-reactive `Set`.** `expandedSubmenus` (toggled on hover, read in the template via `class:is-expanded`, `aria-expanded`, and the `{#if expandedSubmenus.has(item.text)}` that mounts the submenu) was a plain `new Set()`, so `.add()/.delete()` never triggered a re-render — the hover-expand state (and its `aria-expanded`) couldn't update reactively. The file even had a dead `import {  } from 'svelte/reactivity'` betraying the original intent. Converted to `SvelteSet` (svelte-autofixer suggestion, confirmed real by tracing the template reads). Autofixer now returns `issues: 0, suggestions: 0` on the component.

### Runtime — P0: client-side remote queries were silently broken (missing `experimental.async`)

**RUN-1 — `compilerOptions.experimental.async` enabled; fixes `experimental_async_required` repo-wide.** Hunting for the next remote-function conversion turned up a latent **P0 runtime bug**: the SvelteKit remote-functions docs require **both** `kit.experimental.remoteFunctions` _and_ `compilerOptions.experimental.async`, but `svelte.config.js` had only the first. The consequence — invisible to `pnpm check` and `vite build`, which is why three prior remote-function passes (RF-1…RF-3) never caught it — is that any **client-side** remote-query resolution calls Svelte's `hydratable()`, which throws `https://svelte.dev/e/experimental_async_required` when async is off. **Hard evidence:** loading `/dashboard/explosive-swings` (which calls `getAlerts`/`getStats`/… from `onMount → initializeData()`) threw the error **29×** and its queries never resolved. Server-resolved queries (awaited in `load` and serialized — `memberships.remote.ts`, the favorites RF-3 `+page.server.ts`) were unaffected, which is why the dashboard _sidebar_ worked while the _body_ queries didn't. Enabling async fixes it: explosive-swings **29 → 0** errors and its data resolves; `pnpm check` stays **0 errors / 0 warnings** (4,865 files); the production `vite build` (Cloudflare/lightningcss) passes; a 9-route runtime smoke (home, about, pricing, dashboard, explosive-swings, favorites, admin, admin/orders, admin/members) is clean except `admin/members`, whose 2 `undefined.length`/`undefined.total_members` TypeErrors are a _mock-data_ gap (the probe backend doesn't populate the members store) and are **config-independent** — present with async on or off. Reversible in one block (documented inline in `svelte.config.js`). This also unblocks the idiomatic June-2026 patterns (`{#await query()}`, reactive `$derived(await query(arg))`, `command().updates()` single-flight).

**RF-4 — `admin/orders` data layer on typed remote queries.** With RUN-1 in place, converted the orders page off raw `fetch('/api/admin/orders…')` to typed, valibot-validated `query` functions in `admin/orders/orders.remote.ts` (`getOrders({ page, perPage, status, search })` + `getOrderDetail(id)`); shapes live in `orders.types.ts`. Consumed **imperatively** — `loadOrders()` awaits `getOrders(...)` and assigns to local `$state`; the detail awaits `getOrderDetail(...)` on modal open — because the reactive `$derived(getOrders(...)).current` form, while now _possible_ under async, was deliberately avoided here to preserve the page's exact paginate/filter/refresh behaviour and "keep last data while loading" feel (matching the proven `explosive-swings` consumer). Net: end-to-end type safety + server-side arg validation replacing `any` JSON, same UX. CSV export stays a client `fetch` (it streams a `Blob`, not a devalue-serialized value). **Verified live** (probe backend with `MOCK_ROLE=super-admin` + an orders fixture): list renders 25 rows, Next paginates (`Page 1 of 3` → `Page 2 of 3`, first row `RTP-1001` → `RTP-1026`), the detail modal opens and loads, zero page errors. Harness: `mock-backend.mjs` gained an env-gated role (`MOCK_ROLE`, default `member`) and an admin-orders fixture so admin remote functions can be probed.

**RF-5 — `admin/members/analytics` on a single reactive remote query.** The first conversion to use the now-unblocked **idiomatic reactive** pattern. The page's six parallel `fetch('/api/admin/members/analytics/…')` calls + ~90 lines of `Promise.allSettled` result-processing collapse into one typed `query` — `analytics.remote.ts:getMemberAnalytics(range)` runs the six fetches server-side (each degrading independently to its empty default) and returns one combined `MemberAnalytics` payload (`analytics.types.ts`). The page consumes it reactively: `const q = $derived(getMemberAnalytics(dateRange))`, with same-named `$derived` views (`metrics`, `growthData`, `cohortData`, …) so **every chart's markup is unchanged**. Changing the date-range `<select>` (now a plain `bind:value`, the `onchange={loadAnalytics}` removed) re-runs the query automatically; Refresh/Retry call `q.refresh()`. Net: −~120 LOC, end-to-end types replacing six `any` JSON reads, server-side parallelism, and SSR-resolved first paint. **Verified live** (probe backend with a members-analytics fixture): 4 metric cards render with real data (`1,240` total members), the not-connected state correctly absent, **0** `experimental_async_required`; the query is server-resolved on first paint (no client request to the analytics proxy — it's inlined), and changing the range / clicking Refresh each fire one client remote-function request (20 → 21 → 22) while the cards keep rendering. Harness: `mock-backend.mjs` gained an env-gated members-analytics fixture (raw arrays/objects, matching the backend shape the page reads directly).

**RF-6 — `admin/media/analytics` (bandwidth dashboard) on a reactive remote query.** Same reactive pattern as RF-5, applied to the bandwidth-savings dashboard: three parallel `fetch('/api/admin/media/analytics/…')` + `Promise.allSettled` processing → one `getMediaAnalytics(range)` query (`media-analytics.remote.ts`/`.types.ts`) returning `{ overview, bandwidth, formats, hasData }`; the page derives same-named views so the chart/SVG markup is untouched, and the time-range buttons just set `timeRange` (the `handleTimeRangeChange` + `onMount` loader deleted). The one wrinkle — three `tweened` headline numbers (`savingsPercent`/`totalSavings`/`co2Saved`) that were imperatively `.set()` after fetch — is now driven by a single `$effect` that syncs them off the derived `overview`; the svelte-autofixer's "function call in `$effect`" hint is a documented false-positive here (a time-interpolated motion store can't be a `$derived`), so the component stays at `issues: 0`. **Verified live** (media-analytics mock fixture): renders with real data (`savingsPercent 58`, charts present, not-connected absent), **0** `experimental_async_required`, and switching the range flips the active button (`30 Days` → `90 Days`) and fires exactly one `getMediaAnalytics(["90d"])` remote call. Harness gained a media-analytics fixture.

**RF-7 — `admin/seo/404-monitor` on queries + commands with single-flight mutations.** The first conversion to exercise the **full June-2026 mutation idiom**. The page's raw `fetch` + manual `loadLogs()/loadStats()` re-fetch dance becomes `monitor.remote.ts`: two reactive queries (`getLogs(sort)` keyed by the sort `<select>`, `getStats()` no-arg) consumed via `.current`, plus two commands (`ignoreLog`, `bulkDeleteLogs`) that perform **server-driven single-flight refreshes** — after the mutation they call `getLogs(sort).refresh()` + `getStats().refresh()` so the updated table **and** stat cards ride back on the _same_ response, no second round-trip and no manual re-fetch (`sort` is threaded through each command so the refresh hits the exact `getLogs(sort)` instance the client renders). The `$effect`-on-`sortBy` reload is gone (the reactive query replaces it); the redirect-created callback refreshes imperatively (`logsQuery.refresh()`), since that mutation belongs to a child modal. **Verified live** against a _stateful_ 404-logs mock fixture: list renders 3 rows (stats total `3`); clicking **Ignore** flips a row's badge to "Ignored"; deleting a row shrinks the table **3 → 2** _and_ updates the stat card **3 → 2** in lock-step — proving the command refreshed both queries in one flight — with **0** `experimental_async_required`. Harness: `mock-backend.mjs` gained a stateful 404-log store (bulk-delete removes ids, ignore flips status, stats computed live) + request-body reading.

**RF-8 — `admin/seo/redirects` on queries + commands with single-flight mutations.** Same pattern as RF-7, applied to the redirect manager. `redirects.remote.ts`: two no-arg reactive queries (`getRedirects()`, `getRedirectStats()`) + three commands (`removeRedirect`, `toggleRedirectActive`, `bulkRemoveRedirects`) that single-flight refresh both queries after mutating (no `sort` to thread, since the queries are argument-less). The page keeps its handler names (`deleteRedirect`/`toggleRedirect`/`bulkDelete`) — only their bodies + the data source changed — so the table markup is untouched; the editor-saved callback refreshes imperatively (child modal). Notably the old optimistic local toggle (`redirect.is_active = !…; redirects = [...redirects]`) is replaced by the command + single-flight, removing a hand-maintained mutation path. **Verified live** (stateful redirects mock): 4 rows / `total 4` / `active 3`; toggling a redirect flips `active 3 → 2`; deleting one shrinks the table **4 → 3** and `total 4 → 3` in lock-step; **0** `experimental_async_required`. Harness gained a stateful redirects store (toggle/delete/bulk-delete).

**RF-9 — `admin/seo/keywords` on queries + command with single-flight.** The smallest of the SEO CRUD set: `keywords.remote.ts` exports two no-arg queries (`getKeywords()`, `getKeywordStats()`) + one command (`removeKeyword`) that single-flight refreshes both after a delete; Refresh now calls `keywordsQuery.refresh()`/`statsQuery.refresh()`. The `onMount`+`browser` loaders are gone. **Verified live** (stateful keywords mock): deleting a keyword shrinks the table **3 → 2** and the `Total Keywords` stat **3 → 2** in lock-step; **0** `experimental_async_required`. Harness gained a stateful keywords store.

**RF-10 — `admin/schedules` on a typed query + 6 commands (imperative consumer).** The heaviest non-billing CRUD page (8 handlers: load, create, update, delete, toggle, duplicate, bulk-delete, bulk-update). Converted **surgically, imperatively** (like `orders`): `schedules.remote.ts` exports `getSchedules(roomId)` + 6 commands (`postSchedule` backs create _and_ duplicate; `putSchedule` backs edit _and_ the active-toggle; `deleteScheduleById`; `bulkDeleteSchedules`; `bulkUpdateSchedules`) — each handler keeps its exact validation/`error`/`success`/modal flow, only the `fetch` body changed. Commands surface the backend's `{error|message}`. The commands deliberately do **not** single-flight-refresh (no reactive `.current` reader → it'd be dead code per RF-2); the page's `loadSchedules()` is the refresh path. **Two evidence-based fixes surfaced during verification:**

- _Caching bug (caught by hard evidence):_ a delete's server mutation succeeded (mock store `1,2,3 → 2,3`) but the UI stayed at 3 rows — because remote queries are **cached by argument**, so the post-mutation `await getSchedules(sameRoomId)` returned the stale cached list. Fixed by re-reading via `getSchedules(roomId).refresh()` in `loadSchedules` (forces a fresh fetch). `orders` never hit this since its pagination changes the cache key.
- _Dead client redirect removed:_ the old `loadSchedules` `401 → goto('/login')` is dropped — `admin/+layout.server.ts` + `hooks.server.ts` enforce admin auth server-side (the layout's own comment notes the client redirect was _bypassable_ and was superseded), so it was redundant; `goto` import removed.

**Verified live** (stateful schedules mock, keyed by `room_id`): all 3 seed schedules render (**0** `experimental_async_required`); deleting one drops the grid **3 → 2 cards** _and_ the store `1,2,3 → 2,3`; toggling the inactive schedule flips it active (`Activate` buttons **1 → 0**, `Deactivate` **2 → 3**, store all-active). Harness gained a stateful schedules store (full CRUD + bulk + method-aware routing).

**RF-11 — `admin/subscriptions/plans` on typed queries + commands (billing-sensitive, surgical).** The last and most delicate page — its mutations re-price Stripe products and migrate live subscribers. Converted **surgically/imperatively**: `plans.remote.ts` exports `getPlans()` + `getPriceHistory(id)` queries and `changePlanPrice` / `updatePlan` / `setPlanActive` commands. The commands are **thin forwarders** — all money normalization (`Math.round(price * 100)` → `price_cents`/`amount_cents`) and the grandfathering/migration-detail messaging stay in the component **verbatim**; only the `fetch` moved server-side. `changePlanPrice` **returns** the backend payload so the page keeps reporting `subscriptions_migrated`/`subscriptions_failed`. Same `.refresh()` cache fix as RF-10 in `loadPlans` (argument-less query → single cache key → would otherwise go stale after a mutation). **Verified live, including the billing path** (stateful plans mock): stats render (total 3 / active 2 / withStripe 2, **0** `experimental_async_required`); toggling the inactive plan flips `active 2 → 3`; a price change with `apply_to: next_renewal` posts the migration request, the success toast correctly reports **"5 existing subscriber(s) will switch at next renewal"** (i.e. the command's _return value_ is consumed), the list price updates **$149 → $199**, and the store reflects `price: 199` — confirming the cents-normalization and the post-mutation refresh both work. Harness gained a stateful plans store (list / price-history / price-change-with-migration-counts / PUT).

### Hygiene & tooling

**HYG-1 — Noise-comment cleanup.** Removed high-confidence cruft across 178 member/admin files (1,349 lines): commented-out Svelte rune code, `// Old:` dead refs, decorative `@version`/`@author` doc tags, and pure box-drawing separators — while preserving all functional directives (`svelte-ignore`, `eslint-disable`, `@ts-*`), license/purpose headers, and genuine WHY comments. The box-art rule explicitly never touches a `/*`/`*/` delimiter line (an earlier draft left CSS comments unterminated — caught via a `/* */` balance check + build before commit). (commit `72043cf`)

**TOOL-1 — `frontend/.cls-probe/` CLS measurement harness.** Local-only Node scripts that drive the dev server (with a mock backend) through Chromium and report CLS + the shifting DOM nodes via the Layout Instability API — so the CLS fixes above are reproducible before/after numbers, not guesses.

## [Unreleased] — 2026-05-24 23:30 UTC — Google May 2026 SEO updates + repo hygiene + audit refresh

Branch: `claude/google-seo-updates-may-2026-Sp1j7` (off `main`). Single end-to-end pass triggered by "pull up Google's latest May 2026 SEO updates and compare against our repo and implement the changes end to end using the Svelte MCP tool." Followed by a forensic repo audit and a hygiene sweep run under the explicit standing rule "we never delete retired files; move them into the retired folder; act on forensic hard evidence, not assumptions; use both the Svelte and Rust MCP tools."

Every quality gate ran green at the end of the pass: `pnpm check` 4,725 files / **0 errors / 0 warnings**; `pnpm test:unit` **2,255 passed / 32 skipped / 0 failed** across 57 files; `cargo check` clean (full fresh build). `svelte-autofixer` returned zero issues on the modified admin schema component. The Rust MCP server (`rust-mcp-server`) is declared in `.mcp.json` but its binary is not installed in the current cloud-agent environment, so `cargo check` was driven directly via Bash and that limitation is logged here for transparency.

### SEO — Google May 2026 changes wired end-to-end

**SEO-1 — FAQ rich results deprecation (Google 2026-05-07).** Google removed FAQ rich results from Search on 2026-05-07 (Rich Results Test loses support June 2026; Search Console API support removed August 2026). All four FAQ-schema generators in this repo are now marked `@deprecated` and emit a dev-only `console.warn` when invoked: `faqSchema()` in [frontend/src/lib/seo/jsonld.ts](frontend/src/lib/seo/jsonld.ts), `generateFAQSchema()` in [frontend/src/lib/seo/structured-data.ts](frontend/src/lib/seo/structured-data.ts), `generateFAQ()` in [frontend/src/lib/utils/structured-data.ts](frontend/src/lib/utils/structured-data.ts), and `generateFAQStructuredData()` in [frontend/src/lib/options-calculator/utils/seo.ts](frontend/src/lib/options-calculator/utils/seo.ts). Markup is still emitted because it remains useful for AI/voice-search consumers (LLMs, voice assistants) — only the Google rich-result expectation is gone. The admin schema-picker at [frontend/src/routes/admin/seo/schema/+page.svelte](frontend/src/routes/admin/seo/schema/+page.svelte) now shows a "Deprecated" badge plus explanatory note next to `FAQPage`. Twelve existing routes still emit `FAQPage` JSON-LD; none were touched.

**SEO-2 — Speakable schema builder (Google 2026-05-15 generative-AI guide).** New `speakableSchema({ url, cssSelector?, xpath?, name?, id? })` builder in [frontend/src/lib/seo/jsonld.ts](frontend/src/lib/seo/jsonld.ts), backed by new `JsonLdSpeakable` and `JsonLdSpeakableSpec` types in [frontend/src/lib/seo/types.ts](frontend/src/lib/seo/types.ts). Exported through the unified `$lib/seo` barrel. First adoption: [frontend/src/routes/blog/[slug]/+page.ts](frontend/src/routes/blog/[slug]/+page.ts) emits a Speakable node alongside the existing `articleSchema` + `breadcrumbSchema`, selecting `.post-title` and `.post-lead` (the headline + lede already rendered by the page). [frontend/src/routes/our-mission/+page.svelte](frontend/src/routes/our-mission/+page.svelte) attaches `SpeakableSpecification` directly to its existing `Article` `@graph` node and adds `data-speakable` to the hero `<h1>` and lede `<p>` so the selector resolves. This is the active replacement for the AI-surface area lost when FAQ rich results were deprecated.

**SEO-3 — `hasAdultConsideration` on Product schema (Google 2026-05-20).** Added `hasAdultConsideration?: 'yes' | 'no'` to `ProductConfig` and `generateProduct()` in [frontend/src/lib/utils/structured-data.ts](frontend/src/lib/utils/structured-data.ts), defaulting to `'no'` for our trading-education catalogue when omitted. Matches the Merchant Center feed specification alignment Google announced 2026-05-20.

**SEO-4 — May 2026 core update (Google 2026-05-21).** No code action — content-quality focused. README §May 2026 SEO updates documents the four changes and the "prefer Speakable over FAQ for new content" guidance.

### Docs — Forensic audit + CLAUDE.md refresh

**DOC-1 — `docs/audits/FORENSIC_AUDIT_2026-05-24.md` added.** End-to-end audit captured the repo state at HEAD `30cda69`: gates green, two prior P0 release-blockers from [docs/audits/FULL_REPO_AUDIT_2026-05-17.md](docs/audits/FULL_REPO_AUDIT_2026-05-17.md) (P0-1 checkout route mismatch, P0-3 webhook atomicity) mechanically closed via the `api/src/routes/payments/` module split and the `create_subscription_checkout` wiring (both verified by `grep` on this tree; both still owe a live Playwright pass for full sign-off). Repo composition: frontend 76,017 LOC / 1,092 .svelte / 558 routes; Rust API 110,538 LOC. Quality-debt heatmap: 295 `: any | <any>`, 81 `TODO/FIXME/XXX/HACK`, **63 `console.log/debug` (down from 186 in the Apr 25 audit — large reduction)**.

**DOC-2 — CLAUDE.md "Read first" table refreshed.** Promotes `FORENSIC_AUDIT_2026-05-24.md` to row 1 ("what's the state of this codebase?"); promotes `FULL_REPO_AUDIT_2026-05-17.md` to "picking the next thing to fix" in place of the Apr 25 distinguished-engineer audit whose Tier-0 backlog items are now closed or moot post-Fly-strip. The Apr 25 `REPO_STATE` is retained as "history-only context."

### Hygiene — One-shot scripts retired (not deleted) + audit-folder slimmed

**HYG-1 — Six one-shot PE7 Python scripts moved into the retired folder.** `fix_spx.py`, `fix_spx_all.py`, `fix_spx_complete.py`, `fix_spx_now.py` (repo root) and `frontend/src/routes/alerts/spx-profit-pulse/fix_gsap.py`, `fix_plans.py` all hard-code an absolute macOS path (`/Users/billyribeiro/Desktop/...`) — they ran exactly once during the PE7 migration and are unrunnable on any other machine. They were originally added in `ff7f510` and a prior session in this same branch deleted them in `eb91d4c`. That deletion is reversed: the files are now restored under [retiredmay26/scripts-pe7-2026-05-23/](retiredmay26/scripts-pe7-2026-05-23/), with sha256 verified against the deleted versions and a provenance README. `git grep` confirmed zero external references to any of the six.

**HYG-2 — Four root-level audit markdowns relocated into `docs/audits/`.** `GSAP_ANIMATION_AUDIT_2026-05-23.md`, `GSAP_COMPLETE_VERIFICATION_2026-05-23.md`, `RESTORE_AND_MIGRATE_PLAN.md`, and `orphans.md` were sitting at the repo root rather than under `docs/audits/` where CLAUDE.md points readers. They moved into `docs/audits/`; `orphans.md` renamed to `ORPHAN_INVENTORY_2026-05-20.md` to match the dated-audit convention.

**HYG-3 — Twelve orphaned audit docs retired (`retiredmay26/docs/audits-2026-05-24/`).** A forensic cross-reference sweep of `docs/audits/*.md` (`git grep -lE "\\b<filename>\\b"` + `git grep -lFw "<basename>"`, both excluding the file itself) found 53 of 65 audit MDs were referenced from `CHANGELOG.md` / `CLAUDE.md` / `README.md` / other audit docs / **and source code itself** (e.g. `api/src/routes/consent.rs`, `frontend/src/lib/api/abandoned-carts.ts`, `frontend/src/lib/stores/connections.svelte.ts`, `frontend/src/routes/+layout.svelte`, `api/migrations/schema.sql`). Those 53 stay where they are — moving them would break inbound links. The remaining 12 with zero inbound references moved into [retiredmay26/docs/audits-2026-05-24/](retiredmay26/docs/audits-2026-05-24/) with a provenance README that lists every file, every check, and the bash snippet to re-verify on the current tree:

- `ADMIN_CMS_AUDIT_2026-05-10.md`, `CSS_ISOLATION_PLAN_2026-04-25.md`, `GSAP_ANIMATION_AUDIT_2026-05-23.md`, `GSAP_COMPLETE_VERIFICATION_2026-05-23.md`, `OVERSIZED_COMPONENTS_CHANGELOG_2026-05-16.md`, `PORT_FLEXIBILITY_2026-04-27.md`, `RESTORE_AND_MIGRATE_PLAN_2026-05-23.md`, `SVELTE_REMEDIATION_PLAN_2026-05-16.md`, `TASK3_RESULT.md`, `TASK5_RESULT.md`, `TASK6_RESULT.md`, `TRADING_ROOMS_BACKEND_GAPS_2026-05-16.md`.

### Discipline notes

- Svelte MCP `svelte-autofixer` was run on the modified `frontend/src/routes/admin/seo/schema/+page.svelte` per the CLAUDE.md mandate — zero issues returned.
- Rust MCP `rust-mcp-server` is configured in `.mcp.json` but absent from this cloud-agent runtime; `cargo check` was run via Bash as a fallback (clean compile).
- Every move in this pass used `git mv` so file-history follow works.

### Verification

```
$ pnpm check
COMPLETED 4725 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

$ pnpm test:unit
Test Files  57 passed (57)
     Tests  2255 passed | 32 skipped (2287)
  Duration  ~35s

$ cargo check
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2m 51s
```

## [Unreleased] — 2026-04-29 16:07 EDT — Security hardening pass: 13 of 14 audit gaps closed

Branch: `security-hardening-2026-04-29` (off `main`). Single audit-and-fix pass driven by [docs/audits/SECURITY_GAPS_2026-04-29.md](docs/audits/SECURITY_GAPS_2026-04-29.md). Every claim grounded in direct source read; every fix verified by `cargo check` clean, `cargo test --lib config` 13/13 passing, `pnpm check` 0 errors / 0 warnings / 5217 files.

13 code-fixable gaps closed. Only C-3 (R2 credential rotation) remains — it requires logging into the Cloudflare console and cannot be fixed from code.

### Security — CRITICAL fixes

**C-1 — OAuth callback no longer leaks JWTs in the URL.** Google and Apple callback handlers in [api/src/routes/oauth.rs](api/src/routes/oauth.rs) previously redirected to `/auth/callback?token=<JWT>&refresh_token=<JWT>&session_id=...`, leaking credentials into Cloudflare logs, browser history, and Referer headers. New `oauth_callback_response_with_cookies()` helper sets `rtp_access_token`, `rtp_refresh_token`, `rtp_session_id` as `HttpOnly; SameSite=Lax; Path=/` cookies on the redirect Response (with `Secure` in production), and the redirect URL carries only `?provider=google` or `?provider=apple`. Frontend [src/routes/auth/callback/+page.svelte](frontend/src/routes/auth/callback/+page.svelte) rewritten — no longer reads `?token=...` from the URL; calls `GET /api/auth/me` to confirm the cookie session.

**C-2 — Frontend `hooks.server.ts` no longer decodes JWT payloads without verifying signatures.** The transient-failure fallback at lines 263-306 of [frontend/src/hooks.server.ts](frontend/src/hooks.server.ts) ran `JSON.parse(Buffer.from(parts[1], 'base64').toString())` and trusted the resulting `payload.role`. An attacker who could induce an API failure (slow request, transient 500) and plant a forged JWT in the cookie could land on admin pages. The block is deleted; transient API failure now sets `event.locals.user = null` and forces re-authentication.

### Security — HIGH fixes

**H-1 — `JWT_SECRET` production assertion.** [api/src/config/mod.rs](api/src/config/mod.rs) `validate_production_secrets()` now panics at boot if `ENVIRONMENT=production` AND (`JWT_SECRET.len() < 32` OR `JWT_SECRET` contains the strings `replace-me`, `placeholder`, `changeme`, or `your-secret-here`). Catches the `.env.example` placeholder leaking into a prod deploy. Five new `#[should_panic]` tests cover short / empty / `.env.example` literal / generic placeholder / `changeme` cases. All 13 config tests pass.

**H-2 — Password reset now invalidates all sessions.** [api/src/routes/auth.rs](api/src/routes/auth.rs) `reset_password` handler tail rewritten: after the password update + token delete, looks up `user_id` by email, calls `redis.invalidate_all_user_sessions()` + `redis.invalidate_user_cache()`, and inserts a `security_events` row of type `password_reset` / category `authentication` / severity `high`. A thief holding a stolen access or refresh token can no longer survive a password reset.

**H-3 — JWT blacklist now fails closed.** [api/src/middleware/auth.rs](api/src/middleware/auth.rs) previously let the request through on `redis.is_token_blacklisted` `Err`. Now returns 401 "Authentication temporarily unavailable" — frontend's existing 401-then-refresh-then-re-auth flow handles UX. Eliminates the window where a Redis fault during logout/ban kept the just-revoked token usable.

**H-4 — Coupon `usage_count` no longer double-incremented.** Removed the duplicate increment block in [api/src/routes/checkout.rs](api/src/routes/checkout.rs) (was running at order-create, including for abandoned carts). Single source of truth is now [api/src/routes/payments.rs](api/src/routes/payments.rs) `handle_checkout_completed`, idempotent at the `webhook_events(event_id)` UNIQUE level. New migration [api/migrations/065_backfill_coupon_usage.sql](api/migrations/065_backfill_coupon_usage.sql) recomputes historical counts from `COUNT(orders.id) WHERE status = 'completed'`.

**H-5 — `impersonate_user` admin endpoint deleted.** The placeholder stub at [api/src/routes/admin.rs](api/src/routes/admin.rs) returned a non-functional `impersonate_{id}_{timestamp}` string but contained the comment "In a real implementation, you would generate a JWT token for the target user" — a footgun for the next person who took it literally and minted a backdoor JWT for arbitrary users. Handler body removed (replaced with explanatory FIX-H-5 comment), route registration removed, frontend wrappers in [src/lib/api/admin.ts](frontend/src/lib/api/admin.ts) and [src/lib/api/enterprise/admin-adapter.ts](frontend/src/lib/api/enterprise/admin-adapter.ts) removed. No UI page called either, no end-user-visible regression.

### Security — MEDIUM fixes

**M-1 — CSP tightened, both API and frontend.** [api/src/main.rs](api/src/main.rs): dropped `'unsafe-inline'` from `script-src` and `style-src` (API serves only JSON), removed `http://localhost:*` and `ws://localhost:*` from `connect-src`, added explicit `default-src 'none'`, `frame-ancestors 'none'`, `base-uri 'self'`, `form-action 'self'`. [frontend/svelte.config.js](frontend/svelte.config.js): added Stripe domains (`https://js.stripe.com` to script-src and frame-src, `https://api.stripe.com` to connect-src, `https://hooks.stripe.com` to frame-src) so checkout will work; removed duplicated always-on `http://localhost:8080` from connect-src (still in the `NODE_ENV === 'development'` branch). Kept `style-src 'unsafe-inline'` on frontend — required for Svelte component-scoped `style=""` attributes; SvelteKit's nonce mode covers `<style>` blocks but cannot rewrite inline attributes.

**M-2 — Dead `RateLimitService` removed.** Deleted [api/src/services/rate_limit.rs](api/src/services/rate_limit.rs) (452 lines never instantiated anywhere). Login rate limiting goes through `state.services.redis.check_login_rate_limit` directly with fail-closed behavior. Module declaration removed from [api/src/services/mod.rs](api/src/services/mod.rs). New migration [api/migrations/067_drop_unused_login_rate_limits.sql](api/migrations/067_drop_unused_login_rate_limits.sql) drops the empty backing table.

**M-3 — Unused `oauth_tokens` table dropped.** Schema declared `access_token_encrypted` and `refresh_token_encrypted` columns implying capability that never existed in code (no encrypt helper anywhere in `api/src/`; OAuth handler never wrote to the table). New migration [api/migrations/066_drop_unused_oauth_tokens.sql](api/migrations/066_drop_unused_oauth_tokens.sql) drops it. Future provider-token storage feature must build encryption-at-rest from scratch and ship the schema alongside the writer.

**M-4 — Bcrypt-to-argon2id silent rehash on successful login.** [api/src/routes/auth.rs](api/src/routes/auth.rs) login handler now detects `password_hash` starting with `$2` (Laravel bcrypt legacy) and re-hashes the verified plaintext with argon2id, overwriting the column. Best-effort — a DB fault logs but doesn't fail the login. Audit event `bcrypt_rehashed_to_argon2id` emitted on each migration; query progress with `SELECT COUNT(*) FROM users WHERE password_hash LIKE '$2%'`.

### Security — LOW fixes

**L-1 — Email PII stripped from `tracing` logs.** 28 call sites across [api/src/routes/auth.rs](api/src/routes/auth.rs), [api/src/middleware/admin.rs](api/src/middleware/admin.rs), and [api/src/routes/oauth.rs](api/src/routes/oauth.rs) included `email = %user.email` (or `%input.email`) in `security`/`security_audit` log events. All removed via batch perl pass — `user_id = %user.id` is retained, so correlation is one DB lookup away (and `security_events` table still holds the email under admin auth).

**L-2 — Verification tokens cleared at register.** [api/src/routes/auth.rs](api/src/routes/auth.rs) register handler now does `DELETE FROM email_verification_tokens WHERE user_id = $1` before the new INSERT, inside the same transaction. Defensive; matches the `resend_verification` pattern. Idempotent.

### Outstanding

**C-3 — R2 credential rotation.** Cannot be fixed from code. The values currently in `api/.env` are real Cloudflare R2 credentials. Rotation steps in [docs/audits/SECURITY_GAPS_2026-04-29.md](docs/audits/SECURITY_GAPS_2026-04-29.md) §C-3.

### Verification

```
$ cargo check
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.43s

$ cargo test --lib config --test utils_test --test stripe_test
running 13 tests
... 13 passed; 0 failed

$ pnpm check
COMPLETED 5217 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

### Files

15 modified, 1 deleted, 4 created. Net: −240 lines.

| Modified                                         | Deleted                        | Created                                              |
| ------------------------------------------------ | ------------------------------ | ---------------------------------------------------- |
| api/src/config/mod.rs                            | api/src/services/rate_limit.rs | docs/audits/SECURITY_GAPS_2026-04-29.md              |
| api/src/main.rs                                  |                                | api/migrations/065_backfill_coupon_usage.sql         |
| api/src/middleware/admin.rs                      |                                | api/migrations/066_drop_unused_oauth_tokens.sql      |
| api/src/middleware/auth.rs                       |                                | api/migrations/067_drop_unused_login_rate_limits.sql |
| api/src/routes/admin.rs                          |                                |                                                      |
| api/src/routes/auth.rs                           |                                |                                                      |
| api/src/routes/checkout.rs                       |                                |                                                      |
| api/src/routes/oauth.rs                          |                                |                                                      |
| api/src/services/mod.rs                          |                                |                                                      |
| frontend/src/hooks.server.ts                     |                                |                                                      |
| frontend/src/lib/api/admin.ts                    |                                |                                                      |
| frontend/src/lib/api/enterprise/admin-adapter.ts |                                |                                                      |
| frontend/src/routes/auth/callback/+page.svelte   |                                |                                                      |
| frontend/svelte.config.js                        |                                |                                                      |

---

## [Unreleased] — 2026-04-28 (k) — Comprehensive Svelte audit: autofixer round 2, security, accessibility

A full second pass of the Svelte MCP autofixer plus targeted security and accessibility hardening across the codebase. Four discrete improvements landed in this checkpoint, each verified against `pnpm check` (0 errors / 0 warnings / 5217 files).

### Fixed — Svelte autofixer round 2 (41 issues across 35 files)

Parallel autofixer audit across all four major folders. Found and fixed issues missed by round 1.

| Folder                  | Files audited | Files fixed | Issues                         |
| ----------------------- | ------------- | ----------- | ------------------------------ |
| `lib/components/`       | 408           | 5           | 7 missing `{#each}` keys       |
| `lib/` (non-components) | 29            | 1           | 1 dead self-assignment removed |
| `routes/admin/`         | 162           | 24          | 28 missing `{#each}` keys      |
| `routes/` (non-admin)   | 239           | 5           | 5 missing `{#each}` keys       |

Confirmed absent across the entire codebase: legacy `on:event` directives, `export let` props, `createEventDispatcher`, `beforeUpdate`/`afterUpdate`, `<slot>`/`$$props`/`$$restProps`/`$$slots`, `<svelte:component>` deprecation, and `state(props.foo)` shadow-state anti-pattern.

### Security — Wrapped 19 unsafe `{@html}` usages with `sanitizeHtml`

Every `{@html}` site that rendered user-controlled or API-sourced content now passes through DOMPurify via `$lib/sanitize`. Files updated include `CountdownTimer`, `AccordionTabField`, `PostContentField`, `PlatformDownloads`, `VideoTranscript`, `BannerRenderer`, `view-order/[id]`, `[room_slug]/video/[slug]`, `learning-center/[slug]`, `swing-trading-room/video/[slug]`, `small-account-mentorship/video/[slug]`, `dashboard/indicators/[id]`, and the explosive-swings search result cards (`TradePlanResultCard`, `TradeResultCard`, `AlertResultCard`).

Verified safe-as-is and untouched: hardcoded `Icons.*` SVG strings (mentorship, our-mission), JSON-LD escape pattern (SEOHead, options-calculator, store/scanners), already-sanitized renderers (`HtmlBlock.safeHtml`, `MobileResponsiveTable.getValue`).

### Accessibility — Added `aria-hidden="true"` to ~282 decorative SVGs across 129 files

Single-line `<svg>` openings without `aria-hidden`, `aria-label`, or `role` got `aria-hidden="true"` so screen readers skip them. Eight lone-icon buttons (where the SVG was the only child of a `<button>`) got `aria-label` on the button + `aria-hidden="true"` on the SVG instead, since the parent control needed the label: `VideoTranscript` copy button, `RepeaterField` move/remove buttons, `FileUploadField` remove button, `ImageUploader` retry/remove buttons.

### Infrastructure — Upgraded `lib/components/Icon.svelte`

The Tabler icon wrapper now passes `class`, `stroke`, and `color` through to the underlying Tabler component — making it a true drop-in replacement for inline `<svg>` markup that uses Tailwind sizing/coloring classes (`w-4 h-4 text-emerald-400`, etc.). Existing call sites unaffected (props are optional, defaults match the prior behavior). This unlocks the upcoming inline-SVG → Tabler migration documented in `INLINE_SVG_AUDIT.md`.

### Documentation

New report at [INLINE_SVG_AUDIT.md](INLINE_SVG_AUDIT.md): inventory of all 850 inline icon SVGs across 217 files (excluding `routes/dashboard/`), grouped by area (admin, marketing, lib/components), with file paths, route URLs, and the top 30 path signatures matched to Tabler equivalents. The members dashboard (`routes/dashboard/`) is permanently excluded from icon migration per product decision.

### Verification

`pnpm check` after all changes: **0 errors / 0 warnings / 5217 files**.

---

## [Unreleased] — 2026-04-28 (j) — Svelte MCP autofixer pass: payments-fix-2026-04 branch

### Fixed

Full Svelte autofixer + grep sweep on the `payments-fix-2026-04` branch. 98 missing `{#each}` key expressions across 42 files — all keys chosen by reading the actual data shape (`.id`, `.value`, `.href`, `.name`, primitive identity, or index as last resort).

Folders fixed:

| Folder                                    | Files  | Instances |
| ----------------------------------------- | ------ | --------- |
| `lib/components/forms/` (+ `pro/`)        | 10     | 27        |
| `lib/components/auth/`                    | 3      | 4         |
| `lib/components/blog/BlockEditor/`        | 2      | 3         |
| `lib/components/cms/blocks/`              | 2      | 3         |
| `lib/components/courses/`                 | 1      | 3         |
| `lib/components/dashboard/` (+ `alerts/`) | 6      | 13        |
| `lib/consent/components/`                 | 1      | 4         |
| `lib/consent/templates/`                  | 1      | 6         |
| `lib/seo/`                                | 1      | 2         |
| `routes/my/subscriptions/`                | 1      | 1         |
| `routes/admin/seo/`                       | 14     | 32        |
| **Total**                                 | **42** | **98**    |

Final grep for unkeyed `{#each}` blocks: **zero results.**
`pnpm check` after all changes: **0 errors / 0 warnings / 5217 files**.

---

## [Unreleased] — 2026-04-27 (i) — Svelte MCP autofixer pass: straggler fixes

### Fixed

Two `lib/components/` files missed in pass (e): `BatchOperations.svelte` and `forms/ThemeCustomizer.svelte` — both missing `{#each}` key expressions. Added `(action.id)` and `(theme.id)` respectively.

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-28 (i) — Svelte MCP autofixer pass: straggler fixes

### Fixed

Six files missed in earlier passes — all missing `{#each}` key expressions:

- `lib/components/BatchOperations.svelte` — added `(action.id)`
- `lib/components/forms/FormBuilder.svelte` — added `(type)` key on field types loop
- `lib/components/forms/MultiStepFormRenderer.svelte` — added `(step.id)` on steps loop
- `lib/components/forms/ThemeCustomizer.svelte` — added `(theme.id)`
- `routes/admin/seo/keywords/+page.svelte` — added `(keyword.keyword)` on 3 loops
- `routes/admin/seo/redirects/+page.svelte` — added `(type)` and `(redirect.id)`

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-27 (h) — Svelte MCP autofixer pass: routes/ (non-admin)

### Fixed

Ran `svelte-autofixer` on all `.svelte` files under `frontend/src/routes/` outside of `routes/admin/`. All issues were missing `{#each}` key expressions. Folders fixed: `(dev)/workbench`, `about`, `account`, `alerts`, `analytics`, `behavior`, `blog`, `cart`, `checkout`, `classes`, `cookie-policy`, `courses`, `crm`, `dashboard` (including all nested explosive-swings, spx-profit-pulse, swing-trading-room, day-trading-room, small-account-mentorship components), `email`, `indicators`, `learning-center`, `live-trading-rooms`, `media`, `mentorship`, `my`, `our-mission`, `resources`, `store`, `tutorials`, `watchlist`, `workflows`.

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-27 (g) — Svelte MCP autofixer pass: routes/admin/

### Fixed

Ran `svelte-autofixer` on all `.svelte` files under `frontend/src/routes/admin/`. All issues were missing `{#each}` key expressions. Subfolders covered: `analytics`, `behavior`, `blog`, `boards`, `cart`, `categories`, `cms`, `connections`, `consent`, `contacts`, `coupons`, `courses`, `crm`, `email`, `forms`, `indicators`, `media`, `members`, `memberships`, `orders`, `performance`, `popups`, `products`, `resources`, `schedules`, `seo`, `settings`, `site-health`, `subscriptions`, `trading-rooms`, `users`, `videos`, `watchlist`.

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-27 (f) — Svelte MCP autofixer pass: lib/ (non-components)

### Fixed

Ran `svelte-autofixer` on all `.svelte` files under `frontend/src/lib/` outside of `lib/components/` (already covered in pass (e)). Folders audited:

| Folder                                               | Files audited | Files fixed                                                                                                             |
| ---------------------------------------------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `lib/consent/components/` + `lib/consent/templates/` | 8             | 2 (`ConsentPreferencesModal`, `BannerRenderer` — missing `{#each}` keys)                                                |
| `lib/icons/`                                         | 24            | 0                                                                                                                       |
| `lib/monitoring/`                                    | 1             | 0                                                                                                                       |
| `lib/options-calculator/components/` (all subdirs)   | 48            | 1 (`StrategyBuilder` — missing `{#each}` key on breakevens loop)                                                        |
| `lib/seo/`                                           | 1             | 0 (autofixer flags `{@html}` JSON-LD as XSS warning — content is sanitized via `safeJsonLdSerialize`, accepted as safe) |

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-27 (e) — Svelte MCP autofixer full pass: lib/components/

### Fixed

Ran `svelte-autofixer` (Svelte 5, official MCP) on every `.svelte` file under `frontend/src/lib/components/`. All issues were missing `{#each}` key expressions — Svelte requires keys for correct DOM diffing, stable component state, and working transitions. Folders audited and fixed:

| Folder                                                                                                       | Files audited | Files fixed                               |
| ------------------------------------------------------------------------------------------------------------ | ------------- | ----------------------------------------- |
| `lib/components/auth/`                                                                                       | 9             | 1 (`MobileBackground` — 2 `{#each}` keys) |
| `lib/components/admin/`                                                                                      | 28            | 9                                         |
| `lib/components/analytics/`                                                                                  | 16            | 8                                         |
| `lib/components/blog/` + `blog/BlockEditor/`                                                                 | 21            | 10 (26 keys)                              |
| `lib/components/cart/` + `charts/` + `checkout/` + `classes/`                                                | 7             | 1                                         |
| `lib/components/cms/` (all subdirs)                                                                          | 47            | 9 (15 keys)                               |
| `lib/components/consent/` + `core/` + `courses/` + `crm/` + `dashboard/`                                     | 29            | 9                                         |
| `lib/components/dev/` + `forms/` + `icons/` + `indicators/` + `layout/`                                      | ~50           | 16                                        |
| `lib/components/media/` + `nav/` + `patterns/` + `popups/` + `resources/`                                    | 31            | 15                                        |
| `lib/components/sections/` + `seo/` + `ssr/` + `traders/` + `trading-room/` + `ui/` + `video/` + `workflow/` | ~120          | 31                                        |
| **Total**                                                                                                    | **~358**      | **~109**                                  |

`pnpm check` after all changes: **0 errors / 0 warnings / 5215 files**.

---

## [Unreleased] — 2026-04-27 (d) — Svelte autofixer pass: keyed `{#each}` blocks

### Fixed

- **21 components missing `{#each}` keys (Svelte best-practice violation).** Every unkeyed `{#each}` block forces Svelte to rebuild DOM nodes from the end of the list on any update rather than moving existing nodes by identity — causes stale component state, broken transitions/animations, and unnecessary re-renders. The Svelte MCP autofixer was run on the 21 affected components; keys were added using the most specific available property (`item.id`, `item.service`, `item.src`, primitive value, or index for animation-ticker spreads). Components fixed: `AdminToolbar`, `ApiNotConnected`, `CommandPalette`, `ConnectionHealthPanel`, `CountdownTimer`, `ExportButton`, `NotificationCenter`, `PopupModal`, `RateLimitIndicator`, `SEOHead`, `SkeletonLoader`, `VideoEmbed`, `CourseDetailDrawer`, `RetentionCurve`, `BlockInserter`, `BlockSettingsPanel`, `PerformanceOverlay`, `RevisionHistory`, `SchedulingPanel`, `CourseReviews`, `DashboardSidebar`.
- **`pnpm check` remains 0 errors / 0 warnings** after all changes.

---

## [Unreleased] — 2026-04-28 (c) — Blog admin: tag CSP + type-mismatch fixes

### Fixed

- **CSP blocked tags dropdown in dev (`connect-src`).** `loadTags()` called `api.get('/api/admin/tags')` which routed to `http://localhost:8080` (direct backend) — blocked by CSP before the request left the browser. Root cause: `apiFetch` routes `/api/*` to `API_BASE_URL` (localhost:8080), bypassing the SvelteKit proxy. Fix (two parts): (1) `svelte.config.js` now extends `connect-src` with `http://localhost:8080`, `http://localhost:5173`, `http://localhost:5174` when `process.env.NODE_ENV === 'development'` — production CSP unchanged; (2) `loadTags()` in both create and edit pages now calls `fetch('/api/admin/tags', { credentials: 'include' })` (same-origin SvelteKit proxy, cookie-authed) instead of `api.get(...)`.
- **Tags sent as `number[]` instead of `string[]` → HTTP 422.** `post.tags` stored tag IDs (numbers); `CreatePostRequest.tags: Option<Vec<String>>` expects names. serde rejects integers with 422 "expected a string". Fix: `savePost()` in both create and edit pages now maps `post.tags` (IDs) → tag names via `availableTags.find(t => t.id === id)?.name` before building the request body.
- **`published_at` format rejected by Rust serde (`NaiveDateTime`).** `new Date().toISOString()` produces `"YYYY-MM-DDTHH:MM:SS.mmmZ"`; `datetime-local` inputs produce `"YYYY-MM-DDTHH:MM"` (no seconds). Both fail serde's `NaiveDateTime` parser (rejects milliseconds, Z suffix, and missing seconds). Fix: new `toNaiveDateTime()` helper in both pages strips ms/Z and pads missing seconds to `:00`.
- **`loadPost()` fails on fresh page load (`Failed to fetch`).** `adminFetch` relies on an in-memory auth token that is null until the auth store restores from the refresh cookie. On a full page navigation the token is gone, so `adminFetch` sends no Authorization header; the backend returns 401; the plain-text 401 body causes `response.json()` to throw inside `fetchFromBackend`, which returns `{data:null, status:500}`. Fix: `loadPost()` now calls `fetch('/api/admin/posts/${postId}', { credentials: 'include' })` (same-origin, cookie-authed SvelteKit proxy) instead of `adminFetch`. Unused `adminFetch` import removed.
- **`createProxyShim` response caused `ERR_CONTENT_DECODING_FAILED`.** The shim streamed the upstream response with all original headers including `content-encoding: gzip`. Node's `fetch` (server-side) already decompresses the body, so the browser received uncompressed bytes labelled as gzip and failed to decode them. Fix: `createProxyShim.ts` now strips `content-encoding` and `transfer-encoding` before returning the response to the browser.

### Added

- **`frontend/tests/e2e/tags_csp_fix.spec.ts`** — end-to-end verification spec: API-based login (no UI flake), confirms tags dropdown populates (CSP fix), tags sent as strings (type-mismatch fix), POST/PUT both return 200, public `/blog/<slug>` returns 200, edit page title loads correctly (loadPost fix). Cleanup in `finally` block.

---

## [Unreleased] — 2026-04-28 (b) — Infrastructure & Dev Stack

Migration system reconciliation, Meilisearch local stack, and Fly.io config cleanup.

### Fixed

- **Migration checksum mismatch (permanent startup abort).** `030_room_fulltext_search_indexes.sql` was alphabetically prior to `030_room_search_indexes.sql`, causing sqlx to compare the wrong SHA-384 on every API boot and abort all pending migrations silently. Root cause: duplicate version numbers across 4 pairs/triples (030, 037, 040, 041). Fix: renamed the 5 intruding files to unique version numbers (050–054). All renamed files' DDL was already applied; renaming only affects sqlx's compile-time embedding, not the DB schema.
- **Migrations 031–045 unregistered in `_sqlx_migrations`.** All 15 migrations were applied to the DB (via psql or dump restoration) but never recorded in `_sqlx_migrations`, so sqlx would attempt to re-apply them after any fix to the v30 mismatch. Fix: inserted 19 rows directly into `_sqlx_migrations` with correct SHA-384 checksums and `success=true`. Max registered version is now 54.
- **Meilisearch startup warning on every API boot.** `SearchService::setup_indexes()` fires in a background tokio task at startup; with no Meilisearch container in the local stack it logged `WARN Failed to setup search indexes: HTTP request failed: builder error` on every start. Fix: added `getmeili/meilisearch:v1.7` as a first-class service in `docker-compose.yml` with a healthcheck; the API `depends_on` it as `service_healthy`. Zero warnings on startup after fix.
- **`fly.toml` Meilisearch secret names wrong.** Comments listed `MEILI_URL` / `MEILI_MASTER_KEY` but `api/src/config/mod.rs` reads `MEILISEARCH_HOST` / `MEILISEARCH_API_KEY`. Corrected to match the code. Also corrected `RUST_LOG` (was `revolution_api=debug` — overly verbose for prod; changed to `sqlx=warn,tower_http=info`).
- **`fly.toml` missing required non-secret env vars.** `HOST`, `PORT`, `ENVIRONMENT`, and `JWT_EXPIRES_IN` were only set in `docker-compose.yml`. Added to `fly.toml [env]` block so production picks them up without a `fly secrets set` call.

### Added

- **Meilisearch service in local dev stack** (`docker-compose.yml`): `getmeili/meilisearch:v1.7`, dev master key `dev-meili-master-key`, persistent `meili_data` volume, port 7700. The api service `environment:` block injects `MEILISEARCH_HOST=http://meili:7700` and `MEILISEARCH_API_KEY=dev-meili-master-key` — these take precedence over the intentionally blank values in `api/.env`, which are left blank so prod secrets aren't needed for local dev.
- **`fly.toml` prod migration repair note.** Inline comment references `docs/audits/MIGRATION_REPAIR_2026-04-28.md` with the exact `INSERT` block to run against the production DB before the next `fly deploy`.

### Changed

- **Bring-up command in `docker-compose.yml` header** updated: `docker compose up -d db redis api` → `docker compose up -d db redis meili api`.
- **`api/.env.example`** Meilisearch comment updated to explain that blank values are intentional for local dev (docker-compose `environment:` block provides them).
- **5 migration files renamed** (no SQL content changed):
  - `030_room_fulltext_search_indexes.sql` → `050_room_fulltext_search_indexes.sql`
  - `037_video_system_ict7_complete.sql` → `051_video_system_ict7_complete.sql`
  - `040_crm_deals_pipelines.sql` → `052_crm_deals_pipelines.sql`
  - `040_subscription_notifications_ict7.sql` → `053_subscription_notifications_ict7.sql`
  - `041_cms_scheduling_releases.sql` → `054_cms_scheduling_releases.sql`

---

## [Unreleased] — 2026-04-28 (a) — Auth Security Hardening

Comprehensive auth/authorization security hardening. All 7 findings verified live against the running Docker stack. Full audit in `docs/audits/AUTH_AUDIT.md`.

### Security

- **C-1 — Banned user auth bypass closed.** `is_active` added to `User` struct (`api/src/models/user.rs`) and to both SELECT queries (login handler + auth middleware). Login blocks banned accounts before JWT issuance; middleware rejects every subsequent request from a banned user regardless of token validity.
- **H-1 — JWT access token TTL reduced from 24h to 1h.** Default changed in `api/src/config/mod.rs`; `docker-compose.yml` and `api/.env.example` updated to match. `exp - iat = 3600` confirmed live. Production: set `JWT_EXPIRES_IN=1` in Fly secrets (or rely on `fly.toml [env]` after this release).
- **H-2 — Login rate limiter now fails closed on Redis outage.** Both the per-IP and per-email rate limit blocks in `api/src/routes/auth.rs` previously passed all logins through on Redis error. Both now return HTTP 503. Also fixed a pre-existing type error in `api/src/services/redis.rs::incr()`: the Redis pipeline result was typed as `i64` but the crate returns `Vec<i64>`; changed to `Vec<i64>` + `.into_iter().next()`. This bug had been silently masked by the fail-open code.
- **H-4 — Admin `create_user` now validates password strength.** `api/src/routes/admin.rs` calls `validate_password` before `hash_password`; weak passwords now return HTTP 400 instead of being hashed and stored.
- **H-5 — `resend_verification` email enumeration closed.** The already-verified branch previously returned a distinct message confirming both account existence and verification state. Now returns the same generic message as the unknown-email branch with no email sent.
- **H-6 — `ban_user` now invalidates active sessions immediately.** After the DB `is_active = false` update, `ban_user` calls `redis.invalidate_all_user_sessions` and `redis.invalidate_user_cache` (tolerating Redis failure with a warning log). Combined with C-1's middleware check, banned users lose access on their next request rather than after the cache TTL.
- **H-7 — Developer/super_admin role assignment restricted to super_admin actors.** `update_user` in `api/src/routes/admin.rs` now rejects role changes to `developer`, `super_admin`, or `super-admin` from any non-super_admin actor with HTTP 403. All role changes are logged to `security_events` with `event_type='role_change'`, `old_role`, `new_role`, and `actor_id`.

### Fixed

- **`redis::incr()` pipeline deserialization** (`api/src/services/redis.rs`): `redis::pipe().atomic().incr().expire()` returns `Vec<i64>`, not a bare `i64`. The broken type annotation caused `check_ip_rate_limit` to always return `Err(...)`, which the old fail-open login handler silently swallowed. Fixed by deserializing as `Vec<i64>` and taking the first element.

### Security

- **C-1 — Banned user auth bypass closed.** `is_active` added to `User` struct (`api/src/models/user.rs`) and to both SELECT queries (login handler + auth middleware). Login blocks banned accounts before JWT issuance; middleware rejects every subsequent request from a banned user regardless of token validity.
- **H-1 — JWT access token TTL reduced from 24h to 1h.** Default changed in `api/src/config/mod.rs`; `docker-compose.yml` and `api/.env.example` updated to match. `exp - iat = 3600` confirmed live. Production: set or remove `JWT_EXPIRES_IN` in Fly secrets.
- **H-2 — Login rate limiter now fails closed on Redis outage.** Both the per-IP and per-email rate limit blocks in `api/src/routes/auth.rs` previously passed all logins through on Redis error. Both now return HTTP 503. Also fixed a pre-existing type error in `api/src/services/redis.rs::incr()`: the Redis pipeline result was typed as `i64` but the crate returns `Vec<i64>`; changed to `Vec<i64>` + `.into_iter().next()`. This bug had been silently masked by the fail-open code.
- **H-4 — Admin `create_user` now validates password strength.** `api/src/routes/admin.rs` calls `validate_password` before `hash_password`; weak passwords (e.g. `"a"`) now return HTTP 400 instead of being hashed and stored.
- **H-5 — `resend_verification` email enumeration closed.** The already-verified branch previously returned `"Your email is already verified. You can log in."` — a distinct message that confirmed both account existence and verification state. Now returns the same generic message as the unknown-email branch. No email is sent for verified accounts. Verified byte-identical across all three branches live.
- **H-6 — `ban_user` now invalidates active sessions immediately.** After the DB `is_active = false` update, `ban_user` calls `redis.invalidate_all_user_sessions` and `redis.invalidate_user_cache` (tolerating Redis failure with a warning log). Combined with C-1's middleware check, banned users lose access on their next request rather than after the cache TTL.
- **H-7 — Developer/super_admin role assignment restricted to super_admin actors.** `update_user` in `api/src/routes/admin.rs` now rejects role changes to `developer`, `super_admin`, or `super-admin` from any non-super_admin actor with HTTP 403. All role changes (privileged or not) are logged to `security_events` with `event_type='role_change'`, `old_role`, `new_role`, and `actor_id`.

### Fixed

- **`redis::incr()` pipeline deserialization** (`api/src/services/redis.rs`): `redis::pipe().atomic().incr().expire()` returns `Vec<i64>`, not a bare `i64`. The broken type annotation caused `check_ip_rate_limit` to always return `Err(...)`, which the old fail-open login handler silently swallowed. Fixed by deserializing as `Vec<i64>` and taking the first element.

---

## [Unreleased] — 2026-04-27

End-to-end repair of the blog/CMS path: public renderer, admin write path, R2 media routing, scheduling-claim cleanup, analytics ingestion, and dev-stack port consistency. Verified live with a real admin login and a real R2 round-trip.

### Fixed

- **Public blog renderer** rendered every block as an empty tag (e.g. `<p></p>`, `<h5></h5>` for level=2 headings, empty lists, `<img src="">`). Cause: editor saves `{type, content: {...}}`, renderer read `block.data?.X`. Added a `blockData()` shape-normalizer in [`frontend/src/routes/blog/[slug]/+page.svelte`](frontend/src/routes/blog/[slug]/+page.svelte) and [`frontend/src/lib/components/blog/TableOfContents.svelte`](frontend/src/lib/components/blog/TableOfContents.svelte); both data shapes now render correctly.
- **Public renderer extended** to handle `paragraph`, `heading` (h1-h5), `list`, `checklist`, `quote`, `pullquote`, `code`, `preformatted`, `image`, `video`, `embed`, `callout`, `html`, `separator`, `divider`, `spacer`, `button`. Reading time and TOC populate correctly (were 0 / empty).
- **`POST /api/admin/posts` silently dropped** 9 admin-form fields (`featured_media_id`, alt/caption/title/description, `meta_keywords`, `allow_comments`, `categories`, `tags`). DTO + INSERT updated in [`api/src/routes/posts.rs`](api/src/routes/posts.rs); `sync_post_categories` and `sync_post_tags` helpers handle the join tables; tag names create-on-demand via `ON CONFLICT (slug) DO NOTHING`.
- **`PUT /api/admin/posts/:id`** mirrored — clear-and-replace semantics on join tables when the input array is `Some`, preserve when `None`.
- **`published_at` was null** on `status='published'` posts because the create handler bound `input.published_at` directly (almost always `None`). Auto-stamps `Utc::now().naive_utc()` now in both `create_post` and `update_post` (only on draft→published transition for updates).
- **`effect_update_depth_exceeded`** in browser console on every blog post page. Cause was inside [`TableOfContents.svelte`](frontend/src/lib/components/blog/TableOfContents.svelte): `buildHierarchy` mutated the `flatItems` `$state` array via `.push()` from inside the `$effect` that owned it. Refactored to build a local array and assign once. Also added defensive id-tracked gates to the two `$effect`s in [`+page.svelte`](frontend/src/routes/blog/[slug]/+page.svelte) so they don't re-fire on unrelated reactive churn.
- **`/api/analytics/reading` returned 415** for every request. Cause was Axum's `Json<T>` extractor refusing `Content-Type: text/plain`, which is the only content-type `navigator.sendBeacon` can ship without triggering a CORS preflight. Replaced with a `Bytes` extractor + manual `serde_json::from_slice`. Pre-fix, `analytics_events` had 0 `reading` rows ever.
- **`/api/analytics/reading` body shape mismatch** (would have produced 422 once 415 was fixed): client sends camelCase `{event, postId, slug, …}`, server expected snake_case `{post_id, scroll_depth, time_on_page}`. Widened `ReadingTrackRequest` with `#[serde(rename_all = "camelCase")]` + `#[serde(alias = "post_id")]`, a `FlexibleId` enum for the TS `string|number` ambiguity, and `#[serde(flatten)] extras` to preserve any future field into JSONB.
- **`event_name` for reading events was hardcoded** `'page_read'` for every row in `analytics_events`, defeating the column's purpose. Now set from the client's `event` field (`reading_milestone`, `reading_completion`, etc.).
- **`get_related_posts` SQL referenced `p.category_id`** that doesn't exist on the `posts` table; the query was wrapped in `unwrap_or_default()` masking the bug — flagged for follow-up; the noisy code path is now never the only thing standing between the user and their related posts since the renderer fix surfaces this whenever the page loads.
- **`/cms/editor` standalone demo** was a 352-line in-memory toy with no save/load that the README listed as a production blocker. The real editor (`BlockEditor.svelte` used by `/admin/blog/(create|edit)`) was unaffected; the demo + its quarantined Playwright spec are removed.

### Added

- **`api/migrations/045_blog_post_metadata_and_categories.sql`**: 7 new columns on `posts` (`featured_media_id`, `featured_image_alt`, `featured_image_caption`, `featured_image_title`, `featured_image_description`, `meta_keywords TEXT[]`, `allow_comments`); seeds the 18 predefined blog categories so the new server-side slug-resolution logic can find them.
- **`frontend/src/lib/data/predefined-categories.ts`**: single source of truth for the 18 hardcoded blog categories. The `BlogCategory` interface and `getPredefinedCategoryById()` helper moved here; the same colors (canonical: list+create page palette) are now used by all admin views.
- **Dev-mode config fallback** ([`api/src/config/mod.rs`](api/src/config/mod.rs) `required_or_dev` helper): when `ENVIRONMENT=development`, missing `R2_ENDPOINT` / `R2_ACCESS_KEY_ID` / `R2_SECRET_ACCESS_KEY` / `R2_BUCKET` / `STRIPE_SECRET` / `STRIPE_WEBHOOK_SECRET` / `MEILISEARCH_API_KEY` no longer hard-fail boot; uses a `tracing::warn!` placeholder and lets dev sessions run without prod creds. Production behaviour unchanged.
- **Startup environment-mismatch panic** ([`api/src/config/mod.rs`](api/src/config/mod.rs)): when `ENVIRONMENT=development` and `APP_URL` contains `revolution-trading-pros.pages.dev`, `<your-api-host>`, or `revolutiontradingpros.com`, the API panics with a FATAL message instead of silently using the dev placeholders. Defence-in-depth against an environment-name typo in production secrets.
- **R2 storage live** for media uploads. `docker-compose.yml` now loads `api/.env` via `env_file:`; populated R2 credentials in `api/.env`. End-to-end round-trip verified against the real bucket: `POST /api/admin/media/upload` → HTTP 200 with a real CDN URL; `GET <url>` returns the same bytes.

### Changed

- **`api/Dockerfile`**: bumped builder image from `rust:1.87-bookworm` to `rust:1.94-bookworm` because transitive deps (`home@0.5.12`, `time@0.3.47`, `time-core@0.1.8`, `time-macros@0.2.27`) now require Rust ≥ 1.88.
- **`docker-compose.yml`**: `api` service gains `env_file: ./api/.env` so secrets in `api/.env` reach the container; the explicit `environment:` block still wins on conflict for DB/Redis/JWT/HOST/PORT/ENVIRONMENT/CORS so dev defaults stay authoritative.
- **`api/src/routes/posts.rs` `PostRow`** struct now includes the 7 new metadata columns so `SELECT *` deserialization keeps working and read endpoints (`get_post`, `get_post_by_id`) surface them.
- **`get_post` and `get_post_by_id`** return type changed from `Json<PostRow>` to `Json<serde_json::Value>` so the response can flatten in the joined `categories: [...]` (slugs) and `tags: [...]` (names) arrays alongside the post columns.
- **Analytics `track_reading` writes the full client payload** to `analytics_events.properties` JSONB (was only `post_id`/`scroll_depth`/`time_on_page`); future client-side fields are captured without another schema change.
- **Schedule UI hidden** in [`frontend/src/routes/admin/blog/+page.svelte`](frontend/src/routes/admin/blog/+page.svelte): the schedule modal had no backing scheduler worker (`posts.scheduled_publish_at` and the index exist, but no `tokio::spawn` polls them). Re-enable here when a worker is wired in.

### Removed

- **`/cms/editor` page** (`frontend/src/routes/cms/editor/+page.svelte`, 352 lines, in-memory demo with no persistence). The README's "blocker #2" was pointing at this dead code, not at the real editor.
- **`/api/cms/upload/image` endpoint** (`frontend/src/routes/api/cms/upload/image/+server.ts`, 105 lines) — wrote uploads to local disk; Cloudflare-incompatible; zero callers anywhere in source. Real path is `/api/admin/media/upload` → Rust → R2.
- **Yjs collab scaffolding**: 4 files in `frontend/src/lib/components/blog/BlockEditor/collaboration/` (2,455 lines) + `frontend/src/lib/collaboration/yjs-provider.ts` (200 lines) + `frontend/src/lib/components/cms/PresenceAvatars.svelte` (187 lines). Client tried to connect to `wss://.../api/ws/collab/{roomId}`; that path doesn't exist on the Rust router. Removed the four `yjs` / `y-websocket` / `y-indexeddb` / `y-protocols` deps too (12 transitive packages dropped).
- **Schedule modal markup + state + CSS** in [`frontend/src/routes/admin/blog/+page.svelte`](frontend/src/routes/admin/blog/+page.svelte) (~97 lines): modal only fired a "coming soon" toast on submit. TODO marker in place pointing at the missing scheduler worker.
- **`frontend/tests/e2e/block-editor.spec.ts`** (93 lines): targeted the deleted `/cms/editor` demo. Three of its four tests were already `test.fixme`'d.
- **`predefinedCategories` array duplications** across 3 admin pages with color drift (`Technical Analysis` and `Psychology` had different hex codes between edit vs list/create). Single source now lives in `lib/data/predefined-categories.ts`.

### Changed (dev stack — port consistency)

- **`frontend/vite.config.ts`**: added `server: { port: 5173, strictPort: true, host: 'localhost' }`. Dev server now fails fast if 5173 is taken rather than silently drifting to 5174 and breaking CORS.
- **`frontend/package.json`**: added `ports:check`, `dev:clean`, and `dev:fresh` scripts for diagnosing and recovering from port conflicts.
- **`frontend/.env.example`**: added `FRONTEND_URL=http://localhost:5173` as the canonical dev-origin variable consumed by Playwright and preview scripts.
- **`frontend/playwright.config.ts`**: `BASE_URL` reads `FRONTEND_URL || E2E_BASE_URL || localhost:5173`; `webServer.timeout` set to 120 s to accommodate cold Vite starts.
- **`frontend/tests/e2e/verify_tag_fix.spec.ts`** and **`frontend/scripts/preview-component.js`**: hardcoded `localhost:5173` strings replaced with env-var reads matching the pattern above.

### Test evidence (this date)

- `pnpm check`: 5,215 files / 0 errors / 0 warnings.
- `pnpm build`: succeeds.
- `cargo check`: clean.
- `docker compose ps`: `rtp-api`, `rtp-db`, `rtp-redis` all healthy.
- Real admin login (`POST /api/auth/login` → JWT, role `super_admin`).
- Real `POST /api/admin/posts` with full payload → HTTP 200, all 9 new fields surfaced.
- Real `GET /api/posts/:slug` → joined `categories` and `tags` arrays returned.
- Real `PUT /api/admin/posts/:id` → fields updated, joins replaced (categories 2 → 1, tags 3 → 1 on test).
- Real `DELETE /api/admin/posts/:id` → HTTP 200, cascade.
- Real `POST /api/admin/media/upload` (real R2 credentials) → HTTP 200; `GET <returned URL>` → 200, byte-identical PNG.
- Real `POST /api/analytics/reading` (text/plain, sendBeacon shape) → 200; 4 reading rows landed in `analytics_events` during a real Chromium scroll session.
- Renderer browser test (real Chromium, headed) → 8/8 block-type render checks pass; reading time non-zero; TOC populated; no `effect_update_depth_exceeded`.
- `docker run` with `ENVIRONMENT=development` + prod-looking `APP_URL` → panics with the FATAL message.

### Net change

`+490` / `−3,756` lines across 21 files (5 modified, 2 created, 9 deleted, 1 migration added, 4 npm deps removed). Codebase is in a verified known-good state with R2 storage live.

---

## [Unreleased] — 2026-04-25

### Local development

- **Added** Docker Compose stack (`db` + `redis` + `api`) so the platform runs locally without Fly.io.
- **Added** `api/scripts/seed-local-admin.sh` — Argon2id-hashed admin upsert via parameterized psql; safe against email/name containing quotes.
- **Added** `frontend/.env.local` template + auto-loading by Vite so `pnpm dev` points at the local API out of the box.
- **Added** `docs/development/LOCAL_DEV.md` runbook (boot from a fresh clone in 10 minutes, with troubleshooting).
- **Added** `docs/development/ENV_VARS.md` — canonical matrix of all 28 backend + 20+ frontend env vars.

### Tooling / quality

- **Changed** every SvelteKit `+server.ts` proxy under `frontend/src/routes/api/` (76 files) to read `env.API_BASE_URL || env.BACKEND_URL` from `$env/dynamic/private`, with the Fly URL only as a fallback. The previous hardcoded constants made local dev impossible.
- **Changed** `frontend/src/routes/+page.server.ts` (homepage SSR posts loader) to the same env-driven pattern.
- **Migrated** to pnpm 10.33 from npm; added `pnpm-workspace.yaml`.
- **Bumped** all dependencies to current latest (Vite 7, TypeScript 6, ESLint 10, Vitest 4, bits-ui 2, isomorphic-dompurify 3, zod 4, etc.).
- **Migrated** 41 shadcn-svelte wrapper components from the legacy `let props = $props(); let foo = $state(props.foo)` shadowing pattern to idiomatic Svelte 5 destructured `$bindable()` props (eliminates 50 `state_referenced_locally` warnings).

### Repo organization

- **Added** canonical `README.md` at the root.
- **Added** `CONTRIBUTING.md`, `.editorconfig`, `CHANGELOG.md`.
- **Moved** 20+ accumulated audit/setup/forensic markdowns into a structured `docs/` taxonomy (`docs/audits/`, `docs/architecture/`, `docs/setup/`, `docs/forensics/`, `docs/frontend/`, `docs/development/`).
- **Moved** stray shell scripts from the root into `scripts/`.
- **Removed** 9 one-shot frontend migration scripts that had completed their job (recoverable from git history).
- **Removed** stale `.build-trigger` placeholder.
- **Removed** `pa11y-ci` (sole source of glob@7 / inflight / whatwg-encoding deprecation warnings; superseded by Svelte's compile-time a11y rules + `@axe-core/playwright` for runtime checks). −122 transitive packages.
- **Confirmed** Storybook is not installed and never was; no story files, no `.storybook/` config. Nothing to remove.

### Documentation

- **Added** `SYSTEM_ARCHITECTURE_AND_AUTH.md` reference doc.
- **Added** `REPO_STATE_2026-04-25.md` snapshot of the post-pnpm-migration state.
- **Added** four end-to-end audits (in `docs/audits/`):
  - `PRODUCT_AND_AUTH_AUDIT_2026-04-25.md` — auth, RBAC/ABAC, CRUD coverage, products, Remote Functions migration plan
  - `ADMIN_AND_CMS_AUDIT_2026-04-25.md` — backend admin, CMS v2, frontend admin/dashboard, integration layer
  - `DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` — anti-patterns, test gaps, dead code, perf, security, a11y/SEO/observability/i18n
  - 39-item prioritized remediation backlog

### Test gates

All passing as of 2026-04-25:

| Gate                                | Result                                                   |
| ----------------------------------- | -------------------------------------------------------- |
| `pnpm check`                        | 8799 files / 0 errors / 0 warnings                       |
| `pnpm test:unit`                    | 1442 passed / 32 skipped / 0 errors                      |
| `playwright (chromium)`             | 85 passed / 8 skipped (3 fixme + 5 API-gated) / 0 failed |
| `cargo check`                       | clean                                                    |
| `cargo test --no-run`               | all 5 binaries compile                                   |
| `cargo test utils_test stripe_test` | 17 / 17                                                  |

### Known broken (not regressed; documented)

- Production Fly Postgres is unreachable while the subscription is renewed.
- CMS editor toolbar add-block click is a no-op (bits-ui v2 regression at `frontend/src/routes/cms/editor/+page.svelte:113`).
- Stripe Checkout-Session creation is a `// TODO` stub (`api/src/routes/subscriptions.rs:446`); funnel cannot take money via self-serve.
- Frontend `/admin` has no role gate (any logged-in user can mount the UI; backend rejects data calls).
- Favorites proxy reads the wrong cookie (`frontend/src/routes/api/favorites/+server.ts:32`).

See `docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` §9 for the prioritized backlog.
