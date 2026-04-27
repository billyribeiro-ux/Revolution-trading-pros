# BLOG / CMS — END-TO-END AUDIT

**Repo:** Revolution-trading-pros
**Generated:** 2026-04-27
**Scope:** Read-only investigation against the live local Docker stack (`rtp-api` + `rtp-db` healthy on port 8080) plus static analysis. Includes one **empirical round-trip test** against the real public API.
**Companion:** Builds on [BLOG_SYSTEM_REPORT.md](BLOG_SYSTEM_REPORT.md). This audit promotes hypotheses from that report to either **CONFIRMED** or **FALSIFIED** with evidence, and adds new findings.

---

## Executive summary

Of the issues called out in the previous report, the empirical tests confirmed **5 critical bugs**, **4 contract mismatches**, and **2 dead code surfaces**. One concern (**Yjs collaboration**) turns out to be worse than reported: the WebSocket endpoint the client connects to does not exist on the server. One concern was **partially refuted in a useful direction**: the legacy `posts` table has been quietly extended with `scheduled_publish_at`, `scheduled_unpublish_at`, `locale`, `is_primary_locale`, `parent_post_id` (added in migration 014), so it's closer to v2 than the README's `001` schema suggested — but no Rust code yet writes those columns, and no scheduler reads them.

**The single most damaging defect:** every block authored in the admin BlockEditor renders as an empty tag on the public blog. Verified end-to-end via SQL insert → public API GET → simulated render. Detail in §1.

---

## Methodology

1. **Static analysis:** grep for callers/definitions, schema introspection via `psql`, type-flow tracing.
2. **Live API check:** `curl` against `http://localhost:8080`. Health endpoint OK; admin login was sandbox-blocked, so admin-mutating tests were performed via direct SQL inserts (which bypass auth but exercise the same row format).
3. **Round-trip test:** SQL INSERT a `posts` row using the editor's exact JSON payload shape → fetch via the same public REST endpoint the blog UI uses → simulate the renderer's switch statement against the response.
4. **Type/lint gates:** `pnpm check` run to confirm the bugs are not type-detectable.

The post used in the round-trip test (`editor-roundtrip-test`) was deleted after the test.

---

## §1 — CRITICAL: block payload shape mismatch (CONFIRMED end-to-end)

### Hypothesis from prior report
The admin BlockEditor saves blocks shaped `{type, content: {...}, settings, metadata}`. The public renderer reads `block.data?.text`. Same data, different keys → loss on render.

### Round-trip evidence

**Step 1.** SQL insert with the editor's exact serialized shape (matches [admin/blog/create/+page.svelte:188-194](frontend/src/routes/admin/blog/create/+page.svelte#L188-L194)):

```json
[
  {"id":"b1","type":"paragraph","content":{"text":"Hello world from the editor"},"settings":{},"metadata":{}},
  {"id":"b2","type":"heading","content":{"text":"A heading","level":2},...},
  {"id":"b3","type":"list","content":{"items":["one","two","three"],"style":"unordered"},...},
  {"id":"b4","type":"image","content":{"mediaUrl":"/img.jpg","mediaCaption":"A caption","mediaAlt":"alt"},...},
  {"id":"b5","type":"callout","content":{"text":"Callout body"},...}
]
```

**Step 2.** `curl http://localhost:8080/api/posts/editor-roundtrip-test` → returns the blocks unchanged (the Rust API stores `content_blocks` as opaque `serde_json::Value` and round-trips it verbatim).

**Step 3.** Apply the renderer's branches from [blog/[slug]/+page.svelte:330-378](frontend/src/routes/blog/[slug]/+page.svelte#L330-L378):

| Authored | Rendered HTML | Why |
|---|---|---|
| paragraph "Hello world from the editor" | `<p></p>` | reads `block.data?.text` (undefined) |
| heading level=2 "A heading" | `<h5></h5>` | level=2 not matched; falls into `else <h5>`. Text empty. |
| list \[one, two, three\] (unordered) | `<ul></ul>` | reads `block.data?.items` (undefined) |
| image /img.jpg + caption "A caption" | `<figure><img src="" alt=""></figure>` | reads `block.data?.url` / `block.data?.caption` |
| callout "Callout body" | (silently dropped) | no branch in renderer for `callout` |

Reading-time calculator at [blog/[slug]/+page.svelte:104-123](frontend/src/routes/blog/[slug]/+page.svelte#L104-L123) returns **0 minutes** for the same reason — it reads `block.data?.text` and `block.data?.items`. `TableOfContents.svelte:98-108` reads `block.data?.level` and `block.data?.text` — produces an empty TOC.

### Type evidence (why the typechecker doesn't catch it)

- Editor-side ([cms/blocks/content/HeadingBlock.svelte:48](frontend/src/lib/components/cms/blocks/content/HeadingBlock.svelte#L48), `ParagraphBlock`, `QuoteBlock`): all read `props.block.content.text` and `props.block.content.html`.
- Public renderer side ([blog/[slug]/+page.svelte:332](frontend/src/routes/blog/[slug]/+page.svelte#L332)): reads `block.data?.text`.
- `Block.data` and `Block.content` are both loosely typed under different `Block` interfaces (one in `BlockEditor/types.ts`, one in `cms/blocks/types.ts`). The renderer page declares its own local `interface ContentBlock { type: string; data?: { text?: string; ... } }` ([blog/[slug]/+page.svelte:95-102](frontend/src/routes/blog/[slug]/+page.svelte#L95-L102)) — there is no compile-time bridge between the two shapes.

`pnpm check` returns **5,261 files / 0 errors / 0 warnings** — so this defect is invisible to CI.

### Severity
**P0.** Every post created through the modern admin editor is silently broken on its public page. The bug is masked locally because there are no real posts; in any environment with content, this would be the most visible problem.

### Fix scope
A one-place compatibility layer (translate `content` → `data` either at API egress or at render ingress) lifts the immediate breakage. The proper fix is to pick one shape and unify; estimated 1–2 days plus a one-shot DB migration if any rows are already in the `content` shape in production.

---

## §2 — CRITICAL: Rust API silently drops admin form fields (CONFIRMED)

### Evidence (Rust DTOs)

[api/src/routes/posts.rs:62-89](api/src/routes/posts.rs#L62-L89):

```rust
pub struct CreatePostRequest {
    pub title: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
}
```

`UpdatePostRequest` is identical except all fields are `Option`.

### Fields the admin UI collects but the API ignores

Both create + edit pages have these in their `post` state:

| Field | Source | Server-side fate |
|---|---|---|
| `featured_media_id` | image upload result | Dropped — no column on `posts`, no field on DTO |
| `featured_image_alt` | UI form | Dropped |
| `featured_image_title` | UI form | Dropped |
| `featured_image_caption` | UI form | Dropped |
| `featured_image_description` | UI form | Dropped |
| `allow_comments` | UI form (default `true`) | Dropped — no DB column, no DTO field |
| `meta_keywords: string[]` | UI SeoMetaFields | Dropped |
| `categories: string[]` | UI category picker (slugs like `"market-analysis"`) | Dropped |
| `tags: number[]` | UI tag picker | Dropped (`POST /admin/posts` does not touch `post_tags`) |

`scheduled_publish_at`, `scheduled_unpublish_at`, `locale`, `is_primary_locale`, `parent_post_id` were added to the `posts` schema in migration 014 but **none of them appear in the Rust DTO either** — no Rust code reads or writes them.

### Severity
**P0** for the SEO/discoverability features the UI suggests are working (meta_keywords, alt text, scheduling). **P1** for `allow_comments` (no comment system is wired anyway). Anyone using the admin trusts the form; nothing surfaces a save error.

### Verified via DB schema
```
api/migrations/014_advanced_cms_features.sql:431  ALTER TABLE posts ...
\d posts shows columns:
  scheduled_publish_at | timestamp with time zone
  scheduled_unpublish_at | timestamp with time zone
  locale | varchar(10) DEFAULT 'en'
  is_primary_locale | boolean DEFAULT true
  parent_post_id | bigint
```

Plus index `idx_posts_scheduled_publish` for `WHERE status='scheduled'` — but **no scheduler exists**: `grep -rnE "scheduled_publish|publish_scheduled|cron|tokio::spawn.*posts"` finds matches only in v2 `cms_*` files, never for `posts`. The schedule-modal in `admin/blog/+page.svelte` is a UI shell with no backend.

---

## §3 — HIGH: hardcoded categories diverge from DB (CONFIRMED)

### Evidence

A `predefinedCategories: BlogCategory[]` array of 18 trading categories with `id: string`, `name`, `color` is **hardcoded and duplicated 3 times**:

- [admin/blog/+page.svelte:50-69](frontend/src/routes/admin/blog/+page.svelte#L50-L69)
- [admin/blog/create/+page.svelte:33-52](frontend/src/routes/admin/blog/create/+page.svelte#L33-L52)
- [admin/blog/edit/[id]/+page.svelte:36-55](frontend/src/routes/admin/blog/edit/[id]/+page.svelte#L36-L55)

The three copies have **inconsistent colors** for the same categories — e.g. `Technical Analysis` is `#E6B800` in two files and `#6366f1` in the third; `Psychology` is `#B38F00` vs `#8b5cf6`. Same `id`/`name`, drift in display.

Meanwhile the live DB has:

```
public.categories  (id BIGSERIAL, name, slug UNIQUE, description, parent_id, ...)
public.post_categories  (post_id BIGINT, category_id BIGINT)
```

…and the admin UI declares `categories: [] as string[]` (storing slugs like `"market-analysis"`). The `categories` table is currently empty (0 rows). The Rust `CreatePostRequest` has no `categories` field.

So:
- Categories live in TypeScript, not the DB.
- The `categories` table is wired into a `categories.rs` route + `/api/admin/categories` proxy but nothing populates it.
- The picked categories are dropped on save anyway (see §2).
- The display colors drift between edit and list views.

### Severity
**P1.** Drift breeds bugs; categories are a foundational taxonomy.

---

## §4 — HIGH: Yjs collaborative editing is broken / disconnected (CONFIRMED, worse than reported)

### Evidence

Client expects:
- [BlockEditor/collaboration/yjs-provider.ts:235-238](frontend/src/lib/components/blog/BlockEditor/collaboration/yjs-provider.ts#L235-L238): `Format: wss://domain/api/ws/collab/{roomId}`
- Default `WS_URL = 'wss://revolution-trading-pros-api.fly.dev'` (from [api/config.ts:13](frontend/src/lib/api/config.ts#L13))

Server exposes:
- [api/src/routes/mod.rs:129](api/src/routes/mod.rs#L129): `.nest("/ws", websocket::router())`
- [websocket.rs:667-669](api/src/routes/websocket.rs#L667-L669): `.route("/ws", get(ws_handler))` — a single trade-rooms broadcaster.
- `grep -rnE "y-websocket|yjs|/collab"` against `api/src/` returns **zero matches** outside markdown comments. There is no Yjs CRDT host.

So the client connects to `wss://.../api/ws/collab/{roomId}`, but the only matching router prefix is the trade-rooms WS. Either it 404s or it connects and exchanges undecodable binary frames with the wrong handler. Either way: collab is not functional.

The `yjs`, `y-websocket`, `y-indexeddb`, `y-protocols` deps are still pulled into the bundle.

### Severity
**P1** for product correctness if collab is meant to be a feature. **P3** for ship-readiness if it was never meant to be live (should be deleted from deps + the editor's collab UI).

---

## §5 — HIGH: dead code (CONFIRMED + new specifics)

### `/cms/editor` standalone demo

- 352 lines at [routes/cms/editor/+page.svelte](frontend/src/routes/cms/editor/+page.svelte).
- In-memory only: state held in `let blocks = $state<Block[]>([])`, no fetch, no save.
- README treats this page as "the CMS editor" — its broken add-block button is listed as production blocker #2.
- Repo-wide search for `/cms/editor`:
  - **Zero links** from any other Svelte page or layout.
  - **Zero references** in API config / routes.
  - **One reference**: [tests/e2e/block-editor.spec.ts:5](frontend/tests/e2e/block-editor.spec.ts#L5) — Playwright suite that navigates to `/cms/editor`. Three of its tests are already `test.fixme`'d with the comment *"toolbar add-block click is not appending a new editor__block on this commit. Follow-up: trace the bits-ui v2 upgrade impact on the toolbar Button component event handlers."*

So the README's "blocker #2 — CMS editor toolbar add-block is broken" is correctly diagnosed but mis-prioritized: this page has no production callers. It's a fixture for a quarantined E2E that exercises a demo that's not the real editor. The real editor (`BlockEditor.svelte`) is used by `/admin/blog/(create|edit)` and is unaffected.

**Recommended action:** delete the page **and** the E2E spec, OR rewire the spec to exercise `/admin/blog/create` (which is the real editor).

### `/api/cms/upload/image` (local-disk image upload)

- [routes/api/cms/upload/image/+server.ts](frontend/src/routes/api/cms/upload/image/+server.ts) writes to `static/uploads/cms/` via `fs.writeFile`.
- Repo-wide search for callers: **zero**, including in tests.
- The real editor uses `/api/admin/media/upload` → Rust → R2.
- Cloudflare Pages has read-only FS, so this would silently fail in production.

**Recommended action:** delete.

### `BlockToolbar` and `MediaLibrary`

The export comments at [BlockEditor/index.ts:29, 54, 90](frontend/src/lib/components/blog/BlockEditor/index.ts#L29) say: *"BlockToolbar retired 2026-01-26 - zero imports found"*, *"MediaLibrary retired 2026-01-26 - zero imports found"*, *"CollaborationPanel retired 2026-01-26 - zero imports found"*. These removals were already done; mentioning here so they aren't reintroduced.

---

## §6 — MEDIUM: `get_related_posts` references a column that doesn't exist (CONFIRMED)

### Evidence

[posts.rs:621-642](api/src/routes/posts.rs#L621-L642) executes this SQL:

```sql
SELECT DISTINCT p.* FROM posts p
LEFT JOIN post_tags pt1 ON p.id = pt1.post_id
LEFT JOIN post_tags pt2 ON pt1.tag_id = pt2.tag_id AND pt2.post_id = $1
WHERE p.id != $1 AND p.status = 'published'
  AND (pt2.post_id IS NOT NULL OR p.category_id = $2)
...
```

The `posts` table has **no `category_id` column** (`\d posts` confirms; categories live in the `post_categories` join). The query is wrapped in `.unwrap_or_default()` with the comment `// Fallback to empty if query fails (e.g., no category_id column)` — i.e. the author knew it was broken and shipped a silent fallback.

### Effect
`/api/posts/:slug/related` always returns the second-pass "recent posts" branch (the `if related.is_empty() { ... }` fallback at line 645). The category-aware logic is dead code.

### Severity
**P2.** Functional but misleading — appears tag/category-aware in the UI, isn't.

---

## §7 — MEDIUM: legacy posts has scheduling columns + index but no scheduler (NEW)

### Evidence

```
posts.scheduled_publish_at        timestamptz
posts.scheduled_unpublish_at      timestamptz
idx_posts_scheduled_publish       btree (scheduled_publish_at)
                                  WHERE scheduled_publish_at IS NOT NULL
                                    AND status::text = 'scheduled'::text
```

But:
- `CreatePostRequest` / `UpdatePostRequest` do not have `scheduled_publish_at`.
- The frontend's `post.published_at` is mapped to a `datetime-local` input but there's no separate "schedule for" field.
- `grep -rnE "scheduled_publish_at|status.*scheduled" api/src/` returns matches only in domain/cms entity code (v2 `cms_content`), not in `posts.rs` or any worker/cron path.
- `admin/blog/+page.svelte` has `showScheduleModal` and `schedulePost` state but I could not find a `POST` to a scheduling endpoint for legacy posts.
- No `tokio::spawn` worker, no cron, no scheduled-task dispatcher reads these columns.

### Severity
**P1** if scheduling is supposed to work. The feature is half-built end-to-end.

---

## §8 — MEDIUM: two parallel storage systems with no bridge (CONFIRMED)

The previous report flagged this; this audit confirms by content-counts and Rust route inspection:

| | Legacy `posts` | `cms_content` |
|---|---|---|
| Rows in local DB | 0 | 0 |
| Public route (`/blog/[slug]`) | ✅ reads from this | ❌ no public reader |
| Admin UI (`/admin/blog/*`) | ✅ writes to this | ❌ |
| Rich-feature DB (revisions, RLS, scheduling worker, comments, AI history, offline queue) | ❌ none reach legacy | ✅ all present |
| Rust route surface | `posts.rs` (808 lines, 11 endpoints) | `cms_v2.rs` + `cms_v2_enterprise.rs` + `cms_revisions.rs` + `cms_assets.rs` + `cms_seo.rs` + `cms_reusable_blocks.rs` + `cms_presets.rs` + `cms_scheduling.rs` + `cms_datasources.rs` + `cms_ai_assist.rs` + `cms_global_components.rs` + `cms_delivery.rs` |

No migration script, no shim, no FK from `posts` to `cms_content`. Pure parallel implementations.

---

## §9 — Audit of the README's "Top blockers" list

README §"Status of the system (2026-04-25 audit)" lists 5 blockers. Re-evaluation:

| # | README claim | Audit result |
|---|---|---|
| 1 | Restore Fly Postgres | Out of scope here (production infra). |
| 2 | CMS editor toolbar add-block click is broken (`frontend/src/routes/cms/editor/+page.svelte:113`) | **Misprioritized.** That file is a 352-line in-memory demo with zero callers. The real editor (`BlockEditor.svelte` used by `/admin/blog/*`) is unaffected. Recommend: delete the demo + the quarantined E2E. The actual P0 is the §1 shape mismatch — far more impactful and not on the README's list. |
| 3 | Stripe Checkout-Session creation is `// TODO` | Confirmed via grep at `api/src/routes/subscriptions.rs:446` — out of scope for blog. |
| 4 | `/admin` frontend has no role gate | Confirmed via inspection of `admin/+layout.ts` (does not check role). Out of scope here. |
| 5 | Favorites proxy reads the wrong cookie | Out of scope here. |

**The README is missing the 5 items in this audit's §1–§3 + §6–§7.**

---

## §10 — Re-prioritized punch list (evidence-ordered)

| Pri | Item | Evidence | Where |
|---|---|---|---|
| **P0** | Block payload shape mismatch — every post renders empty | §1 round-trip + simulator | [blog/[slug]/+page.svelte:330](frontend/src/routes/blog/[slug]/+page.svelte#L330), [admin/blog/create/+page.svelte:188](frontend/src/routes/admin/blog/create/+page.svelte#L188) |
| **P0** | Most editor block types have no public renderer (only 6 of 40+) | §1 callout case + [types.ts:14-69](frontend/src/lib/components/blog/BlockEditor/types.ts#L14-L69) | same |
| **P0** | Admin form fields silently dropped on save (`featured_media_id`, alt/caption/title/desc, `meta_keywords`, `categories`, `tags`, `allow_comments`) | §2 DTO inspection | [posts.rs:62-89](api/src/routes/posts.rs#L62-L89) |
| **P1** | Categories hardcoded in 3 files with color drift; DB taxonomy unused | §3 | 3 admin pages |
| **P1** | Yjs WS endpoint mismatch — collab non-functional | §4 grep + router | client: [yjs-provider.ts:235](frontend/src/lib/components/blog/BlockEditor/collaboration/yjs-provider.ts#L235); server: nothing |
| **P1** | Scheduling columns exist; no scheduler worker | §7 | DB schema + grep |
| **P2** | `get_related_posts` references nonexistent `p.category_id`; logic is silently dead | §6 | [posts.rs:630](api/src/routes/posts.rs#L630) |
| **P2** | `/cms/editor` page + E2E spec are dead — delete both | §5 | [routes/cms/editor/+page.svelte](frontend/src/routes/cms/editor/+page.svelte), [tests/e2e/block-editor.spec.ts](frontend/tests/e2e/block-editor.spec.ts) |
| **P2** | `/api/cms/upload/image` local-disk endpoint, zero callers, Cloudflare-incompatible | §5 | [api/cms/upload/image/+server.ts](frontend/src/routes/api/cms/upload/image/+server.ts) |
| **P3** | Two parallel content systems with no bridge (`posts` ↔ `cms_content`) | §8 | strategic |

A single **3-line fix at [blog/[slug]/+page.svelte:330](frontend/src/routes/blog/[slug]/+page.svelte#L330)** can lift P0/#1 immediately:

```ts
const data = block.data ?? block.content ?? {};
```

…then read `data.text`, `data.level`, `data.items`, `data.url`, `data.caption`. That keeps existing posts (if any are in the legacy shape) rendering and fixes new posts. Same change in `TableOfContents.svelte` and the reading-time computation. Proper unification + adding the missing block branches is a follow-up.

A single **5-line addition to `CreatePostRequest`/`UpdatePostRequest` + UPDATE/INSERT** in `posts.rs` lifts P0/#3 for the columns that already exist on the table (`scheduled_publish_at`, `scheduled_unpublish_at`, `locale`, `parent_post_id`). The `featured_media_id`/alt/caption + categories/tags require schema and join-table writes — bigger.

---

## §11 — Open questions (carried forward + new)

From the prior report (still open):

1. Which content system is canonical going forward — `posts` or `cms_content`?
2. Are there any blog posts in production?
3. What does "WordPress Classic" mean precisely (single TipTap rich-text + Visual/Text tab, or current blocks + a top-level HTML toggle)?

New from this audit:

4. **Was the editor ever round-trip-tested in a real environment?** The §1 bug suggests no. If it was, on what shape — i.e. did some intermediate version of `posts.rs` translate `content` → `data` and got removed?
5. **Are the 18 hardcoded categories canonical, or were they meant to migrate to the DB?** If canonical, factor to `$lib/data/categories.ts` (one source). If transitional, populate `categories` table and delete the array.
6. **Is Yjs collab a feature you want, or scaffolding to delete?** Removing it cuts ~4 deps from the bundle and simplifies the editor.
7. **Should `scheduled_publish_at` work for legacy posts?** If yes, who owns building the scheduler — Rust `tokio::spawn` worker, or a separate cron? Migration 014 is dated; the index has been in place a while with nothing reading it.
8. **What should happen on an unrecognized block type at view time?** Drop silently (current), render a placeholder, or throw a build-time error so authors can't ship unrenderable types?

---

## Appendix A — Commands and outputs (for reproducibility)

```bash
# Confirmed schema drift in posts
docker exec rtp-db psql -U rtp -d revolution_trading_pros -c "\d posts"
# → includes scheduled_publish_at, scheduled_unpublish_at, locale,
#   is_primary_locale, parent_post_id (none in 001_initial_schema.sql)

# Round-trip insert (using editor shape)
docker exec rtp-db psql -U rtp -d revolution_trading_pros -c \
  "INSERT INTO posts (...) VALUES (..., '<editor-shape>'::jsonb) ..."
# → row id 1, status published

curl -sS http://localhost:8080/api/posts/editor-roundtrip-test
# → returns content_blocks faithfully in editor shape
#   {"content":{"text":"..."},"id":"b1","metadata":{},"settings":{},"type":"paragraph"}

# Verify NO category_id column on posts
docker exec rtp-db psql -U rtp -d revolution_trading_pros -tAc \
  "SELECT column_name FROM information_schema.columns
   WHERE table_name='posts' AND column_name LIKE '%categor%';"
# → (no rows)

# Verify NO Yjs/collab in api/src
grep -rnE "y-websocket|yjs|/collab" api/src/  # → no matches outside doc comments

# Typecheck status
pnpm check
# → 5261 FILES 0 ERRORS 0 WARNINGS

# Test post cleaned up
docker exec rtp-db psql -U rtp -d revolution_trading_pros -c \
  "DELETE FROM posts WHERE slug='editor-roundtrip-test';"
```

## Appendix B — Files touched by the round-trip test

- **Inserted:** 1 row in local `posts` table.
- **Deleted:** same row, after confirming the bug.
- **No code modified.** This audit is read-only on the codebase.
