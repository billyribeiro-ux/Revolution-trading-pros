# Principal Engineer Audit — 2026-07-19

**Scope:** entire repository — frontend (SvelteKit/Svelte 5), api (Rust/axum/sqlx), workspace
tooling, CI/CD, and documentation.
**Method:** every mechanical gate re-run on the current tree; official Svelte 5 / SvelteKit
documentation pulled via the Svelte MCP server to set the idiom bar (`svelte/best-practices`,
`kit/state-management`); six parallel deep-read subsystem auditors (Opus) covering ~200
directly-opened files plus census greps over the full tree; **every P0/P1 claim in this
document independently re-verified at the cited file:line before inclusion**. The Rust MCP
server (`rust-mcp-server`) was installed during this audit and is configured in `.mcp.json`
for future sessions; this run used the equivalent cargo commands directly.

**Repository scale:** 1,102 Svelte components · 793 frontend TS/JS files (~731k LOC) ·
356 Rust files (~111k LOC + ~35k test LOC) · 65 SQL migrations · 2 CI workflows.

---

## Executive verdict

**Overall grade: B− — excellent bones, two production-blocking defects, and consistency
enforced by culture rather than tooling.**

The paradox of this codebase: individual subsystems are written to a genuinely high
standard — the Svelte 5 runes migration is essentially complete (zero legacy patterns in
1,102 components), SQL is injection-free across ~600 query sites, payments are
HMAC-verified + idempotent + transactional, secrets are AES-256-GCM encrypted, and the
migration governance docs are model-grade. But the repo currently ships **two P0s** (the
API panics on boot; paid file downloads are unauthenticated), and the same five
engineering standards are re-implemented 3–5 different ways because nothing mechanical
enforces convergence.

| Subsystem | Grade | P0 | P1 | P2 | P3 |
|---|---|---|---|---|---|
| Rust routes/handlers | C | 2 | 2 | 3 | 6 |
| Rust services/models/DB | B+ | 0 | 2 | 6 | 8 |
| Frontend lib layer | B− | 0 | 4 | 9 | ~14 |
| Public routes + proxies | B− | 0 | 3 | 6 | 7 |
| Admin + CRM routes | B− | 0 | 2 | 6 | 5 |
| Workspace / CI / docs | C+ | 0 | 4 | 11 | 16 |
| **Total** | **B−** | **2** | **17** | **41** | **~56** |

---

## P0 — production-blocking (fix before anything else)

### P0-1 · The API cannot boot: unfinished axum 0.7→0.8 route-syntax migration
`api/Cargo.toml` pins `axum = 0.8.9`, but **233 route definitions across 58 files** still
use axum-0.7 `/:param` syntax (`grep -rE '\.route\("[^"]*/:' api/src/routes` → 233;
`without_v07_checks` appears **0** times). axum 0.8's `Router::route()` **panics at router
construction** on the old syntax; `main.rs` eagerly builds `routes::api_router()` at
startup, so the process aborts on launch. This was empirically reproduced against the
pinned crate. It predates the 2026-07-19 dependency top-up (axum 0.8.9 was already pinned
on main).

Why every gate missed it: the old syntax is valid Rust, so `cargo check` and
`clippy` pass; the two no-DB test files (`utils_test`, `stripe_test`) never construct the
routers; CI has no boot smoke test.

**Fix:** migrate `/:param` → `/{param}` (and `/*rest` → `/{*rest}`) across the 58 files,
then add a permanent gate: a `#[test]` that constructs `api_router()` (this class of
failure is invisible to type-checking and will recur on every framework bump otherwise).

### P0-2 · Paywall bypass: premium file URLs served unauthenticated
`api/src/routes/room_resources/download.rs:23` — `generate_secure_download`
(POST `/room-resources/:id/secure-download`) has **no auth extractor** and returns the raw
`resource.file_url` for any resource id, premium included. `download.rs:79` —
`download_resource` validates the download token **only if one is supplied**
(`if let Some(token) = params.get("token")`); omit the param and the handler falls through
to return `file_url`. Both are mounted on the public router (`room_resources/mod.rs:309,312`).
The code's own comment concedes: "In production, you'd also verify user session/membership
here."

**Fix:** require the `User` extractor + membership check on both handlers; make the token
mandatory on `download_resource`.

---

## P1 — high severity (17)

### Rust API
1. **IDOR via spoofable `x-user-id` header** — `room_resources/public.rs:502,554,597,630,667,700`
   trusts a client-supplied header for per-user favorites/access-log reads *and writes*
   (nothing anywhere sets that header; verified). Any caller can write or delete another
   user's data. `routes/favorites.rs` shows the correct pattern (`User` extractor).
2. **Unauthenticated watch-progress upsert** — `admin_videos/analytics.rs:145`
   (POST `/video-advanced/analytics/progress/:id`) takes no `User`/`AdminUser` and writes
   `video_watch_progress` with a `user_id` from the request **body**; siblings in the same
   file are properly gated.
3. **Postmark client has no HTTP timeout** — `services/email.rs:93,128` (`Client::new()`;
   reqwest has no default timeout). A hung SMTP relay stalls password-reset/verification/
   receipt sends indefinitely. `services/stripe.rs` (30s + fallback) is the house reference.
4. **Bunny client has no HTTP timeout** — `services/bunny.rs:30`; worse,
   `upload_thumbnail_from_url` (bunny.rs:263) fetches a **caller-supplied URL** with no
   timeout (also an SSRF-adjacent surface).

### Frontend
5. **`lib/api/client.svelte.ts` is SSR-unsafe** — module-load singleton (`:1626`) starts two
   unguarded `setInterval`s (`:1473`, `:1523`) that run in the Node SSR process, and holds
   user/membership state in process-global stores (`:1633-1637`) — a cross-request
   user-data-leak surface (latent; no server load currently calls its user methods).
6. **Same client retries non-idempotent mutations** — `executeRequest` (`:634-729`)
   catches the non-retryable throw at `:703` without re-throwing, so failed POST/PUT/PATCH
   (including 4xx) are retried up to 3× with no idempotency key — double-submit risk on
   anything that 500s/times out after committing.
7. **Same client persists auth + refresh tokens to `localStorage`** (`:497-511`),
   contradicting the deliberate memory-only posture of `stores/auth.svelte.ts`; one XSS
   exfiltrates long-lived tokens.
8. **API-client fragmentation: 5 implementations, 3 colliding `api` exports** with
   divergent origin/auth behavior (`config.ts:475` cross-origin bearer vs
   `client.ts`/`client.svelte.ts` same-origin cookie) and three distinct `ApiError` types.
   Importing `{ api }` from the wrong path silently changes security posture.
9. **Unauthenticated paid-LLM endpoint** — `routes/api/cms/ai/generate/+server.ts:21-43`
   forwards a fully user-controlled prompt to Anthropic with caller-supplied `maxTokens`,
   unclamped, and no session check (`/api/cms/*` is not in `PROTECTED_ROUTES`).
   Budget-drain the moment maintenance mode lifts.
10. **Fabricated stock prices served as real quotes** — `routes/api/prices/+server.ts:112-174`
    silently returns `Math.random()`-generated prices with no `simulated` marker when the
    upstream key is absent/failing, and caches them. On a trading-education platform this
    is an integrity/liability problem, not a cosmetic one.
11. **Two analytics proxies still on the pre-fix env shape** —
    `api/analytics/batch/+server.ts:9-10` and `api/analytics/performance/+server.ts:22-23`
    read `env.VITE_API_URL || env.BACKEND_URL || 'http://localhost:8080'`; the 2026-04-26
    repo-wide fix to `env.API_BASE_URL || …` missed exactly these two (their siblings carry
    the fix comment). If prod sets only `API_BASE_URL`, analytics silently posts to
    localhost.
12. **The CRM create/edit surface is 21 non-functional stub pages** (~2,635 lines, one
    placeholder duplicated 21×, 1–6 lines of real variance each; verified by normalized
    diff). List pages ship New/Edit buttons that dead-end into them. Tracked in
    `docs/audits/admin-2026-04-26/06-crm-DEFERRED.md`, but the affordances mislead users
    and the copies will rot. Consolidate into one `ComingSoonStub.svelte` (~90% reduction)
    or hide the buttons.
13. **`admin/crm/leads` does a full-table fetch with client-side pagination**
    (`leads/+page.svelte:250,267,279-291`) — unbounded as the table grows; `templates` and
    `members` already demonstrate the correct server-paginated pattern.

### Workspace / CI / docs
14. **The canonical test gate is fiction** — `pnpm exec playwright test tests/e2e` is
    mandated by CLAUDE.md, README (×2), CONTRIBUTING, and LOCAL_DEV, but `frontend/tests/e2e/`
    was deleted 2026-05-19. Every contributor following the docs hits "no tests found."
    README's status table still claims "e2e ✅ 85 passed."
15. **`pnpm test:unit` is documented as a root command but only exists in `frontend/`** —
    four docs give an invocation that fails from the repo root.
16. **README's stack table is wrong on 6 rows** — "Vite 7" (actual 8.1.5), "Rust 1.94 +
    Axum 0.7" (1.96.0 + 0.8.9), "PostgreSQL 16 + sqlx 0.8" (18.4 + 0.9), "Redis 7"
    (8.8.0), Fly.io listed as prod host (removed 2026-04-28).
17. **`.github/workflows/` documentation describes a pipeline that doesn't exist** —
    README/TECHNICAL_NOTES/SECRETS document `deploy-fly.yml`, production/staging deploy
    jobs, Lighthouse, Slack, cache purge, and cite line numbers past EOF of the real
    107-line `deploy-cloudflare.yml`. CI/CD docs are currently fiction.

---

## P2 — significant (41, thematic roll-up)

**Rust:** two multi-table writes without transactions (`cms_content.rs:211-224` folder
delete; `mfa.rs:240-256` verify-and-enable — its sibling `disable_mfa` does it right); MFA
rate-limit **fails open** on DB error (`mfa.rs:508-523`, `.unwrap_or(0)`); ~106
`unwrap_or_default()` sites swallow DB `Result`s (house-rule violation; reads only); 585
sites return raw internal error strings to clients (`e.to_string()` — constraint names/SQL
fragments leak); no global request-body limit (the 10 MB middleware in
`middleware/validation.rs` is written but never wired; `bunny_upload` buffers up to ~10 GB
per admin upload); money representation is split between NUMERIC-dollars (orders,
membership_plans) and BIGINT-cents (courses, indicators) with scale-casts at the boundary;
`schema.sql`↔prod drift remains owner-gated (documented); the old `rustls-webpki 0.101`
stack rides along via `aws-smithy-http-client` (triaged in deny.toml).

**Frontend:** iframe origin check uses `src.includes(domain)` — bypassable
(`utils/sanitize.ts:198`); canonical SITE_URL is hardcoded to the **staging** `.pages.dev`
domain in `seo/defaults.ts:15` and the sitemap/feeds while options-calculator hardcodes the
apex domain — search engines are being canonicalized to a preview host; `PopupModal.svelte`
leaks listeners from an `$effect` with no cleanup (`:125`) and mutates `$state` inside
`$derived` (`:119,568`); a hand-rolled JSON-LD serializer bypasses the audited
`serializeJsonLd` (`dashboard/Breadcrumbs.svelte:69`); ~1,068 lines of dead client code
(`enhanced-client*.ts`, imported nowhere); two `.svelte.ts` modules contain zero runes
(legacy stores); formatting helpers are re-implemented per-file (`formatDate` ×20,
`formatFileSize` ×15, `formatCurrency` ×13); the admin `/api/admin/crm/*` SvelteKit proxies
are **dead code for the browser path** (both clients target the backend origin directly, so
the carefully-`requireAdmin`ed proxies never run — keep one path, delete the other);
`products/create` vs `products/[id]/edit` share 6 copy-pasted components already 41 lines
diverged; icon-only modal close buttons systematically lack `aria-label`; the catch-all
proxy `api/[...path]/+server.ts` UTF-8-decodes binary upload bodies (`request.text()`) and
drops backend `Set-Cookie` headers; unvalidated `ticker`/`expiration` are interpolated into
Polygon URLs (param smuggling with the site's paid key); public-site SSR `/me` fetch has no
timeout (`hooks.server.ts:170-177` — the protected path got the 10s abort, this one
didn't); three stale self-redirecting `+server.ts` files 301 to their own path (infinite
loop for non-HTML requests); dashboard mock endpoints hotlink a competitor's CDN
(`api/dashboard/spx-profit-pulse/+server.ts` → `cdn.simplertrading.com`).

**Workspace/CI:** CI hardcodes `corepack prepare pnpm@11.5.2` while `packageManager` says
11.15.0 (and LOCAL_DEV says 11.10.0) — three live pnpm versions and the CI comment claims
"single source of truth"; `cargo deny`/`cargo machete` run locally only, never in CI;
vitest coverage thresholds (80/80/75/80) are configured but CI never runs `--coverage` —
decorative; ~90 Rust integration test files exist but CI runs 2 (no Postgres service; the
`docker-compose.test.yml` overlay for it already exists); `deploy-cloudflare.yml:72-73`
injects the literal placeholder `<your-api-host>` as `API_BASE_URL` at build time; no
`concurrency:` guards on either workflow; 3 of 4 `minimumReleaseAgeExclude` pins reference
versions no longer in the lockfile; `svelte-click-to-source` is an orphaned workspace
package (built, imported by nothing; `inspector: false` in svelte.config.js); broken doc
pointers (`backups/fly-io-removed-2026-04-28.md` ×5, `docs/forensics/` ×4);
`scripts/pe7_gate.sh` tells you to wire it into CI and never was. **Prettier drift: 156
files fail `format:check`** (not a CI gate; ESLint passes clean, svelte-check is 0/0 —
formatting is the one hygiene gate with no enforcement).

---

## P3 — notable (~56, highlights)

Rust: `redirects.rs:334` hit-counter endpoint unauthenticated among `AdminUser` siblings;
~2,700 lines of dead unmounted routes (`settings.rs`, `indicators_admin.rs`);
`reqwest::Client::new()` per-request at 10 sites (share one in `AppState`); DB pool has no
`idle_timeout`/`max_lifetime` (never recycles after failover); `panic = "abort"` raises the
stakes of any latent panic; sqlx macro vs runtime split is 27 : 595 (all parameterized, but
the "macros only" house rule is effectively inverted — no compile-time schema checking);
legacy `users` table is `timestamp` while newer tables are `timestamptz` (Rust types track
each correctly — verified — but it's a join footgun); stale Fly.io comments persist in
Cargo.toml and `.cargo/audit.toml`.

Frontend: login response body still echoes tokens that were just set as httpOnly cookies
(`api/auth/login/+server.ts:75`); `maintenance/notify` GET exposes subscriber count with no
auth and its in-memory `Set` is meaningless per-isolate on Cloudflare; `locals.consent` is
typed and read but never written (dead wiring); root `+layout.js` sets `prerender = true`
tree-wide (footgun for future authed routes); `logger.error` is a no-op in production (no
telemetry forwarding — errors vanish); `theme` store's `auto` mode goes stale when the OS
theme flips; 668 `console.*` calls in frontend src; 81 TODO/FIXME markers; `.npmrc` carries
dead justifications (`camera-controls`, `@fortawesome` — neither is a dependency;
`legacy-peer-deps` is an npm flag in a pnpm repo); `WeeklyHero.test.ts` sits where neither
vitest nor playwright will run it; eslint config ignores directories that don't exist;
CHANGELOG stopped 6 weeks ago; `docs/audits/README.md` index omits ~10 newer audits;
1.8 MB `retiredmay26/` + 31 MB `frontend/Implementation/` of dead weight; audit docs cite
commit `05acf3231` which no longer resolves.

---

## Mechanical gates (this tree, this audit)

| Gate | Result |
|---|---|
| `svelte-check` (typecheck) | ✅ 0 errors / 0 warnings (4,745 files) |
| ESLint (frontend, full) | ✅ clean |
| Prettier `format:check` | ❌ 156 files drifted |
| Vitest unit | ✅ 2,272 passed / 32 skipped |
| Frontend production build | ✅ |
| `svelte-autofixer` (root layout, PopupDisplay, maintenance page) | ✅ 0 issues; `client.svelte.ts` flagged legacy stores |
| `cargo check` | ✅ (but cannot catch P0-1) |
| `cargo clippy --locked --all-targets -- -D warnings` | ✅ clean |
| `cargo test` (utils, stripe, crypto) | ✅ 22 passed |
| `cargo machete` | ✅ no unused dependencies |
| `cargo deny` | ⚠️ not runnable in this container; **not in CI either** (P2) |
| API boot | ❌ **panics — P0-1** |

---

## What is genuinely excellent (keep and standardize on these)

- **Svelte 5 discipline:** 0 `export let` / `on:` / `createEventDispatcher` /
  `$app/stores` / `<slot>` across 1,102 components; `$derived` (301 files) dominates
  `$effect` (46); the banned shadow-state pattern appears nowhere; the `seed*` +
  `untrack()` init-once idiom is deliberate and documented.
- **Security engineering:** nonce CSP with no `unsafe-inline`; JWT pinned to HS256 with
  token-epoch revocation (fail-closed); spoof-resistant client IP via trusted-proxy CIDRs;
  Stripe HMAC + single-tx idempotent webhooks; AES-256-GCM secrets-at-rest with legacy
  read-back; constant-time comparisons; every `{@html}` sanitizer-wrapped; the audit trail
  of past fixes is commented with provenance.
- **SQL:** zero injection across ~600 query sites; every `AssertSqlSafe` inspected
  interpolates only whitelisted structural tokens; pagination capped `.min(100)` almost
  universally; full-text search via `plainto_tsquery`.
- **Migration governance:** guarded destructive ops, written rollbacks, an honest README
  documenting the non-reproducible historical chain and the baseline strategy.
- **Reference implementations that the rest of the code should converge on:**
  `frontend/src/lib/api/client.ts` + `errors.ts` + `hooks.svelte.ts` (client stack),
  `seo/serializeJsonLd.ts`, `stores/auth.svelte.ts` (memory-only tokens),
  `services/stripe.rs` (timeout + fallback), `routes/favorites.rs` (per-user auth),
  `admin/+layout.server.ts` (server-side gate sharing one predicate with the proxies).

---

## 10-year sustainability plan (prioritized)

1. **Fix the two P0s and gate the class.** Route-syntax migration + a router-construction
   smoke test in CI; auth on the download endpoints. Nothing else matters until the
   binary boots and the paywall holds.
2. **Make consistency mechanical, not cultural.** Every systemic finding here is a
   correct pattern that exists in-repo but isn't enforced: one API client (delete 4), one
   error type per side, one `resolveBackendUrl()` for all ~150 proxies, one formatting
   module, one logger, shared `AssertSqlSafe` sort/filter helpers, a lint for icon-only
   buttons. Codify via lint rules + a small `lib/server` helper layer; delete the losers.
3. **Close the CI/reality gap.** One gate definition (docs say "run `pe7_gate.sh`", CI runs
   the same script): add format:check, coverage, cargo deny/machete, the Postgres-service
   integration job (compose overlay exists), concurrency guards, and drop the hardcoded
   corepack version so `packageManager` is the single truth. Add a docs-lint step that
   fails on referenced-but-missing paths — it would have caught 8 of the P1/P2s here
   automatically.
4. **Make version-truth generated.** README stack table, pnpm/Node/Rust versions, and the
   audit index should be rendered from lockfiles/toolchain files, not hand-transcribed.
   Wire dependabot/renovate — the repo's monthly manual "top up everything" ritual is
   exactly what automation removes.
5. **Kill the dead weight** — 21 CRM stubs → 1 component; `enhanced-client*` (1,068 lines);
   unmounted Rust routes (2,700 lines); orphaned `svelte-click-to-source`; `retiredmay26/`
   + `Implementation/` (33 MB); self-redirecting endpoints; the bypassed CRM proxies.
   Every orphan costs a future engineer an hour of "is this load-bearing?"
6. **Truth in data:** never serve simulated prices unmarked; move canonical URLs to one
   env-driven origin (currently canonicalizing to a staging domain); finish the
   cents-everywhere money migration; make `logger.error` actually report in production.

---

*Per-subsystem detail (including the full P3 inventories and file-review lists) was
produced by six parallel auditors during this audit; the findings above are the complete
P0/P1 set and the consolidated P2/P3 themes, each spot-verified at the cited locations.*
