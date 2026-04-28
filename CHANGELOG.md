# Changelog

All notable changes to this project. Format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); we don't strictly adhere to SemVer because the product isn't a published library.

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

Comprehensive auth/authorization security hardening. All 7 findings verified live against the running Docker stack. Full audit in `AUTH_AUDIT.md`.

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
- **Startup environment-mismatch panic** ([`api/src/config/mod.rs`](api/src/config/mod.rs)): when `ENVIRONMENT=development` and `APP_URL` contains `revolution-trading-pros.pages.dev`, `revolution-trading-pros-api.fly.dev`, or `revolutiontradingpros.com`, the API panics with a FATAL message instead of silently using the dev placeholders. Defence-in-depth against an environment-name typo in production secrets.
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

| Gate | Result |
|------|--------|
| `pnpm check` | 8799 files / 0 errors / 0 warnings |
| `pnpm test:unit` | 1442 passed / 32 skipped / 0 errors |
| `playwright (chromium)` | 85 passed / 8 skipped (3 fixme + 5 API-gated) / 0 failed |
| `cargo check` | clean |
| `cargo test --no-run` | all 5 binaries compile |
| `cargo test utils_test stripe_test` | 17 / 17 |

### Known broken (not regressed; documented)

- Production Fly Postgres is unreachable while the subscription is renewed.
- CMS editor toolbar add-block click is a no-op (bits-ui v2 regression at `frontend/src/routes/cms/editor/+page.svelte:113`).
- Stripe Checkout-Session creation is a `// TODO` stub (`api/src/routes/subscriptions.rs:446`); funnel cannot take money via self-serve.
- Frontend `/admin` has no role gate (any logged-in user can mount the UI; backend rejects data calls).
- Favorites proxy reads the wrong cookie (`frontend/src/routes/api/favorites/+server.ts:32`).

See `docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` §9 for the prioritized backlog.
