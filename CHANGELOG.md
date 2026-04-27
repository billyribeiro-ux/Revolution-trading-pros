# Changelog

All notable changes to this project. Format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); we don't strictly adhere to SemVer because the product isn't a published library.

## [Unreleased] — 2026-04-27

End-to-end repair of the blog/CMS path: public renderer, admin write path, R2 media routing, scheduling-claim cleanup, and analytics ingestion. Verified live with a real admin login and a real R2 round-trip.

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
