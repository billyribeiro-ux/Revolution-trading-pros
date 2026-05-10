# Cleanup result — 2026-04-27

## What was deleted

| File | Lines | Tracking | How |
|---|---|---|---|
| `BLOG_SYSTEM_REPORT.md` | ~890 | tracked (committed by user during the session) | `git rm` |
| `BLOG_SYSTEM_AUDIT.md` | ~440 | tracked | `git rm` |
| `BLOG_FIXES_RESULT.md` | ~600 | tracked | `git rm` |
| `BLOG_FIXES_RESULT_V2.md` | ~270 | tracked | `git rm` |
| `BLOG_FIXES_RESULT_V3.md` | ~290 | tracked | `git rm` |
| `/tmp/analytics-415-diagnosis.md` | ~75 | untracked | `rm` |

Total: 5 tracked files removed via `git rm`, 1 untracked tmp file removed via `rm`. ~2,565 lines of working-arc documentation excised; the substance is now in `CHANGELOG.md`.

No ad-hoc test scripts remained (the `_blog-roundtrip.mjs`, `_effect-warning-check.mjs`, `_analytics-check.mjs` etc. were each cleaned up at the end of their respective passes). No `/tmp/*` working files survived.

## Notable observation during cleanup

Two commits appeared on `main` while the cleanup work proceeded — `c647146a2` and `affc8fab1` — they aren't from this session. They committed the BLOG report files into history, which is why `git rm` was needed instead of plain `rm`. No commit was made in this cleanup pass.

## New CHANGELOG.md entry (added in place — pasted here for review)

```md
## [Unreleased] — 2026-04-27

End-to-end repair of the blog/CMS path: public renderer, admin write path, R2 media routing, scheduling-claim cleanup, and analytics ingestion. Verified live with a real admin login and a real R2 round-trip.

### Fixed

- **Public blog renderer** rendered every block as an empty tag (e.g. `<p></p>`, `<h5></h5>` for level=2 headings, empty lists, `<img src="">`). Cause: editor saves `{type, content: {...}}`, renderer read `block.data?.X`. Added a `blockData()` shape-normalizer in `frontend/src/routes/blog/[slug]/+page.svelte` and `frontend/src/lib/components/blog/TableOfContents.svelte`; both data shapes now render correctly.
- **Public renderer extended** to handle `paragraph`, `heading` (h1-h5), `list`, `checklist`, `quote`, `pullquote`, `code`, `preformatted`, `image`, `video`, `embed`, `callout`, `html`, `separator`, `divider`, `spacer`, `button`. Reading time and TOC populate correctly (were 0 / empty).
- **`POST /api/admin/posts` silently dropped** 9 admin-form fields (`featured_media_id`, alt/caption/title/description, `meta_keywords`, `allow_comments`, `categories`, `tags`). DTO + INSERT updated in `api/src/routes/posts.rs`; `sync_post_categories` and `sync_post_tags` helpers handle the join tables; tag names create-on-demand via `ON CONFLICT (slug) DO NOTHING`.
- **`PUT /api/admin/posts/:id`** mirrored — clear-and-replace semantics on join tables when the input array is `Some`, preserve when `None`.
- **`published_at` was null** on `status='published'` posts because the create handler bound `input.published_at` directly (almost always `None`). Auto-stamps `Utc::now().naive_utc()` now in both `create_post` and `update_post` (only on draft→published transition for updates).
- **`effect_update_depth_exceeded`** in browser console on every blog post page. Cause was inside `TableOfContents.svelte`: `buildHierarchy` mutated the `flatItems` `$state` array via `.push()` from inside the `$effect` that owned it. Refactored to build a local array and assign once. Also added defensive id-tracked gates to the two `$effect`s in `+page.svelte`.
- **`/api/analytics/reading` returned 415** for every request. Axum's `Json<T>` extractor refused `Content-Type: text/plain`, the only content-type `navigator.sendBeacon` can ship without a CORS preflight. Replaced with a `Bytes` extractor + manual `serde_json::from_slice`. Pre-fix, `analytics_events` had 0 `reading` rows ever.
- **`/api/analytics/reading` body shape mismatch** (would have produced 422 once 415 was fixed): client sends camelCase, server expected snake_case. Widened `ReadingTrackRequest` with `#[serde(rename_all = "camelCase")]` + `#[serde(alias = "post_id")]`, a `FlexibleId` enum for `string|number`, and `#[serde(flatten)] extras` to preserve future fields into JSONB.
- **`event_name` for reading events was hardcoded** `'page_read'`. Now set from the client's `event` field (`reading_milestone`, `reading_completion`, etc.).
- **`/cms/editor` standalone demo** was a 352-line in-memory toy with no persistence that the README listed as a production blocker. The real editor (`BlockEditor.svelte` used by `/admin/blog/(create|edit)`) was unaffected; the demo + its quarantined Playwright spec are removed.

### Added

- **`api/migrations/045_blog_post_metadata_and_categories.sql`**: 7 new columns on `posts` + seeds the 18 predefined blog categories.
- **`frontend/src/lib/data/predefined-categories.ts`**: single source of truth for the 18 hardcoded blog categories.
- **Dev-mode config fallback** (`api/src/config/mod.rs` `required_or_dev` helper): missing R2/Stripe/Meili envs no longer hard-fail boot when `ENVIRONMENT=development`.
- **Startup environment-mismatch panic** (`api/src/config/mod.rs`): `ENVIRONMENT=development` + production-looking `APP_URL` panics with a FATAL message instead of using dev placeholders.
- **R2 storage live**. `docker-compose.yml` loads `api/.env` via `env_file:`. End-to-end round-trip verified: `POST /api/admin/media/upload` → 200 with R2 CDN URL; `GET <url>` returns the same bytes.

### Changed

- **`api/Dockerfile`**: bumped builder from `rust:1.87-bookworm` to `rust:1.94-bookworm`.
- **`docker-compose.yml`**: `api` service gains `env_file: ./api/.env`.
- **`api/src/routes/posts.rs` `PostRow`** struct now includes the 7 new metadata columns.
- **`get_post` and `get_post_by_id`** return type changed to `Json<serde_json::Value>` so responses include joined `categories: [...]` and `tags: [...]` arrays.
- **Analytics `track_reading` writes the full client payload** to `analytics_events.properties` JSONB.
- **Schedule UI hidden** in `frontend/src/routes/admin/blog/+page.svelte`: no backing scheduler worker exists (DB columns + index are in place but unused).

### Removed

- **`/cms/editor` page** (352 lines, in-memory demo).
- **`/api/cms/upload/image` endpoint** (105 lines, local-disk uploader, Cloudflare-incompatible, zero callers).
- **Yjs collab scaffolding**: 4 files in `BlockEditor/collaboration/` (2,455 lines) + `lib/collaboration/yjs-provider.ts` (200 lines) + `cms/PresenceAvatars.svelte` (187 lines). Client connected to a path the server never exposed. 4 npm deps removed (12 transitive packages dropped).
- **Schedule modal markup + state + CSS** in `admin/blog/+page.svelte` (~97 lines).
- **`frontend/tests/e2e/block-editor.spec.ts`** (93 lines, targeted the deleted demo).
- **`predefinedCategories` array duplications** across 3 admin pages with color drift.

### Test evidence (this date)

- `pnpm check`: 5,215 files / 0 errors / 0 warnings.
- `pnpm build`: succeeds.
- `cargo check`: clean.
- `docker compose ps`: `rtp-api`, `rtp-db`, `rtp-redis` all healthy.
- Real admin CRUD over HTTP (`POST/GET/PUT/DELETE`) verified.
- Real R2 upload + fetch round-trip verified byte-identical.
- Real reading-analytics rows persisted to `analytics_events`.
- Renderer browser test (real Chromium): 8/8 block-type checks pass; reading time non-zero; TOC populated; no `effect_update_depth_exceeded`.
- `docker run` with prod-looking `APP_URL` panics with the FATAL message.

### Net change

`+490` / `−3,756` lines across 21 files. Codebase is in a verified known-good state with R2 storage live.
```

## git status

```
D  BLOG_FIXES_RESULT.md
D  BLOG_FIXES_RESULT_V2.md
D  BLOG_FIXES_RESULT_V3.md
D  BLOG_SYSTEM_AUDIT.md
D  BLOG_SYSTEM_REPORT.md
 M CHANGELOG.md
 M docker-compose.yml
```

5 deletions staged via `git rm`, 2 modifications unstaged. Nothing committed (per the standing rule).

## Final gates

| Gate | Result |
|---|---|
| `pnpm check` | 5,215 files / 0 errors / 0 warnings |
| `cargo check` (api) | clean |
| `docker compose ps` | rtp-api / rtp-db / rtp-redis all healthy |
| Working tree | dirty as expected — review and commit yourself |

## Next step

Review the diff (`git diff CHANGELOG.md docker-compose.yml`), then commit when you're satisfied. After confirming, this `CLEANUP_RESULT.md` can be deleted too — it's just the receipt for this pass.
