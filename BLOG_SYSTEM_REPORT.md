# BLOG / CMS SYSTEM REPORT

**Repo:** Revolution-trading-pros
**Generated:** 2026-04-27
**Purpose:** Read-only investigation to plan editor improvements (WordPress-Classic-style visual ↔ HTML editor).
**Scope:** No code modified. Findings only.

---

## TL;DR

There are **two parallel content systems** in this repo:

1. **Legacy `posts` system** (BIGSERIAL ids, JSONB `content_blocks`) — this is what the public blog at `/blog/[slug]` actually reads from, and what the admin UI at `/admin/blog/(create|edit)` writes to via the `/api/admin/posts` proxy → Rust `/admin/posts` route. This is the real working system today.
2. **New `cms_content` v2 system** (UUID, content-type enum, revisions, RLS, scheduling, AI assist, reusable blocks, datasources, presets, global components, Yjs collab provider) — built out across migrations 023–044, with 2 dozen Rust route files (`cms_v2*`, `cms_assets`, `cms_seo`, `cms_revisions`, `cms_reusable_blocks`, etc.) and a fleshed-out `BlockEditor` Svelte component (~2,900 lines) plus 30+ block component types. **The blog admin pages do not yet use this v2 system end-to-end** — they use the v2 `BlockEditor` component, but persist to the **legacy `posts` table**.

The "CMS editor toolbar add-block click is broken" line in [README.md](README.md#L120) refers to [frontend/src/routes/cms/editor/+page.svelte](frontend/src/routes/cms/editor/+page.svelte) — that page is a **standalone in-memory demo**, not the real editor. The real editor used by humans is [frontend/src/routes/admin/blog/create/+page.svelte](frontend/src/routes/admin/blog/create/+page.svelte) and `.../edit/[id]/+page.svelte`, both of which embed the v2 `BlockEditor` component.

The DB is currently empty locally (0 posts, 0 cms_content, 0 categories, 0 tags, 0 cms_assets). Production DB is unreachable per README.

---

## 1. PROJECT STRUCTURE

### Versions / toolchain

| Tool | Version | Source |
|---|---|---|
| Node | `>=20.0.0` | [frontend/package.json:9](frontend/package.json#L9) |
| pnpm | `10.33.2` | [frontend/package.json:7](frontend/package.json#L7) (workspace + filter; root in [package.json:5](package.json#L5)) |
| SvelteKit | `^2.58.0` | [frontend/package.json:110](frontend/package.json#L110) |
| Svelte | `^5.55.5` (runes) | [frontend/package.json:149](frontend/package.json#L149) |
| Vite | `^7` | [frontend/package.json:159](frontend/package.json#L159) |
| TypeScript | `^6.0.3` | [frontend/package.json:156](frontend/package.json#L156) |
| `@sveltejs/adapter-cloudflare` | `^7.2.8` | [frontend/package.json:109](frontend/package.json#L109) |
| Tailwind | `^4.2.4` | [frontend/package.json:154](frontend/package.json#L154) |
| Rust | 1.94 + Axum 0.7 | README |
| Postgres | 16 + sqlx 0.8 | README |

### Folder structure relevant to the blog system

```
frontend/src/
├── routes/
│   ├── blog/                        Public blog index + [slug] (consumer)
│   │   ├── +page.svelte / +page.ts
│   │   ├── [slug]/+page.svelte
│   │   └── [slug]/+page.ts          SSR loader, SEO + JSON-LD
│   ├── posts/[slug]/edit/           Member-edit surface (uses different fetch shape, /api/posts/:slug PATCH)
│   ├── cms/editor/                  Standalone block-editor DEMO (in-memory only, no save/load)
│   ├── admin/blog/                  Real blog admin
│   │   ├── +page.svelte             3,127-line list/grid view (filters, bulk ops, export, schedule modal)
│   │   ├── create/+page.svelte      1,319 lines, embeds <BlockEditor>
│   │   ├── edit/[id]/+page.svelte   1,455 lines, embeds <BlockEditor>
│   │   └── categories/+page.svelte
│   ├── admin/cms/datasources/       Only one v2-CMS subroute landed in admin so far
│   └── api/                         SvelteKit proxies → Rust API
│       ├── admin/posts/             CRUD proxy
│       ├── admin/categories/
│       ├── admin/tags/
│       ├── admin/media/             upload, ai, analytics, [id]
│       └── cms/upload/image/        ⚠ writes to local disk; broken on Cloudflare Pages
├── lib/components/blog/
│   ├── BlockEditor/                 v2 editor (~30+ block types, AI, SEO, revisions, Yjs collab, perf)
│   │   ├── BlockEditor.svelte       2,892 lines
│   │   ├── types.ts                 1,474 lines (BlockType union, BLOCK_DEFINITIONS, SEOAnalysis, …)
│   │   ├── BlockInserter / Renderer / SettingsPanel / ErrorBoundary / VirtualBlockList
│   │   ├── AIAssistant / SEOAnalyzer / RevisionHistory / SchedulingPanel
│   │   ├── KeyboardShortcuts / PerformanceOverlay
│   │   ├── collaboration/           yjs + y-websocket + y-indexeddb
│   │   ├── upload/uploader.ts       client-side upload pipeline → /api/admin/media/upload
│   │   └── performance/, validation.ts, error-handling.ts, offline/
│   ├── SeoMetaFields.svelte
│   ├── TableOfContents / FloatingTocWidget / ReadingProgress / SocialShare
├── lib/components/cms/
│   ├── BlockVirtualizer / PresenceAvatars
│   └── blocks/                      Concrete renderers used by /cms/editor demo
│       ├── content/{Paragraph,Heading,List,Code,Quote,PullQuote,Checklist}Block
│       ├── advanced/HtmlBlock      ← already has edit/preview/split toggle 🔑
│       ├── interactive/, layout/, media/, trading/, ai/, hooks/
└── lib/utils/
    └── sanitize.ts                  DOMPurify wrapper, 4 profiles (strict/standard/rich/minimal)

api/src/routes/
├── posts.rs                         Legacy /posts + /admin/posts (808 lines)
├── categories.rs / tags.rs
├── media.rs                         /admin/media (1,269 lines)
├── cms_v2.rs                        New unified CMS, /admin/cms-v2 + /cms (904 lines)
├── cms_v2_enterprise.rs
├── cms_revisions.rs                 1,209 lines
├── cms_assets.rs                    DAM
├── cms_ai_assist.rs                 Claude-powered AI ops
├── cms_seo.rs                       Server-side SEO validation
├── cms_reusable_blocks.rs
├── cms_presets.rs
├── cms_scheduling.rs
├── cms_datasources.rs
├── cms_global_components.rs
└── cms_delivery.rs

api/migrations/
├── 001_initial_schema.sql           CREATES `posts`, `categories`, `tags`, `post_categories`, `post_tags`
├── 023_custom_cms_implementation.sql CREATES `cms_*` parallel system
├── 024–044                          CMS v2 enhancements (advanced features, scheduling, datasources, …)
```

### Where blog posts live

- **Public blog list:** [frontend/src/routes/blog/+page.svelte](frontend/src/routes/blog/+page.svelte) loads via [+page.ts](frontend/src/routes/blog/+page.ts) → `apiFetch(API_ENDPOINTS.posts.list)` = `/api/posts`.
- **Public blog detail:** [frontend/src/routes/blog/[slug]/+page.svelte](frontend/src/routes/blog/[slug]/+page.svelte) — SSR loader fetches `/api/posts/{slug}` (Rust route `GET /posts/:slug`).
- **Admin list:** [frontend/src/routes/admin/blog/+page.svelte](frontend/src/routes/admin/blog/+page.svelte) (3,127 lines, with grid/list/preview/filter/bulk/export/schedule).
- **Admin create:** [frontend/src/routes/admin/blog/create/+page.svelte](frontend/src/routes/admin/blog/create/+page.svelte).
- **Admin edit:** [frontend/src/routes/admin/blog/edit/[id]/+page.svelte](frontend/src/routes/admin/blog/edit/[id]/+page.svelte).
- **Storage:** `posts.content_blocks` JSONB (legacy `posts` table — see §2).

### Public-facing rendering

Done inline in [frontend/src/routes/blog/[slug]/+page.svelte:308-378](frontend/src/routes/blog/[slug]/+page.svelte#L308-L378) — a giant `{#each post.content_blocks}` switch, branching on `block.type` (`paragraph` / `heading` / `list` / `quote` / `code` / `image`). Uses `{@html sanitizeBlogContent(...)}` for text-bearing blocks. **All other block types from the editor's 40+ vocabulary (callout, cta, columns, video, button, accordion, gallery, ticker, chart, etc.) are NOT rendered here** — they will be silently dropped at view time.

---

## 2. DATABASE / STORAGE LAYER

- **DB:** PostgreSQL 16 (Fly Postgres in prod, Docker `rtp-db` locally; user `rtp` / db `revolution_trading_pros`).
- **Query layer:** raw SQL via `sqlx 0.8` (`sqlx::query_as`, parameterized). No ORM. Migrations auto-run on boot from `api/migrations/*.sql`.

### TWO SYSTEMS

#### A) Legacy `posts` (active — what the blog actually uses)

Defined in [api/migrations/001_initial_schema.sql:159-210](api/migrations/001_initial_schema.sql#L159-L210):

```sql
CREATE TABLE IF NOT EXISTS posts (
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    excerpt TEXT,
    content_blocks JSONB,                     -- ← block array stored here
    featured_image VARCHAR(500),              -- ← URL string only, not FK
    status VARCHAR(20) DEFAULT 'draft',       -- draft | published | archived
    published_at TIMESTAMP,
    meta_title VARCHAR(255),
    meta_description TEXT,
    indexable BOOLEAN DEFAULT true,
    canonical_url VARCHAR(500),
    schema_markup JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS categories (id BIGSERIAL PK, name, slug UNIQUE, description, parent_id);
CREATE TABLE IF NOT EXISTS post_categories (post_id, category_id);
CREATE TABLE IF NOT EXISTS tags (id BIGSERIAL PK, name, slug UNIQUE);
CREATE TABLE IF NOT EXISTS post_tags (post_id, tag_id);
```

Notes / gaps:
- No `featured_media_id` FK — `featured_image` is a free-form string (the admin UI tracks `featured_media_id` in state but it isn't on the legacy table).
- No revision history table for `posts` (revisions exist only for v2 `cms_content`).
- No FK from `posts` to a media table.
- No `category_id` column on `posts` directly — but [api/src/routes/posts.rs:630](api/src/routes/posts.rs#L630) writes a query referencing `p.category_id`, with a comment admitting the column may not exist and a fallback to `unwrap_or_default()`. Real category linkage goes through `post_categories` join.

Format of `content_blocks`: a JSON array of `{ type, data: { text|level|items|style|code|url|caption|width|height } }` objects. The renderer at [blog/[slug]/+page.svelte:330](frontend/src/routes/blog/[slug]/+page.svelte#L330) handles only: `paragraph`, `heading` (level 1–5), `list` (ordered/unordered), `quote`, `code`, `image`. The admin editor produces a richer shape — `{ type, content: { … }, settings: { … }, metadata: { … } }` (see Block type at [BlockEditor/types.ts:75-81](frontend/src/lib/components/blog/BlockEditor/types.ts#L75-L81)) — and [admin/blog/create/+page.svelte:189-194](frontend/src/routes/admin/blog/create/+page.svelte#L189-L194) maps it down to `{ type, content, settings }` before posting. **There is a real shape mismatch between what the editor saves (`content`) and what the renderer reads (`data`).**

#### B) New `cms_content` (parallel; not yet wired into public blog)

Defined in [api/migrations/023_custom_cms_implementation.sql:304-392](api/migrations/023_custom_cms_implementation.sql#L304-L392):

```sql
CREATE TYPE cms_content_type AS ENUM ('page','blog_post','alert_service','trading_room',
    'indicator','course','lesson','testimonial','faq','author','topic_cluster',
    'weekly_watchlist','resource','navigation_menu','site_settings','redirect');
CREATE TYPE cms_user_role AS ENUM ('super_admin','marketing_manager','content_editor',
    'weekly_editor','developer','viewer');
CREATE TYPE cms_content_status AS ENUM ('draft','in_review','approved','scheduled','published','archived');

CREATE TABLE cms_content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type cms_content_type NOT NULL,
    slug VARCHAR(500) NOT NULL,
    locale VARCHAR(10) NOT NULL DEFAULT 'en',
    is_primary_locale BOOLEAN, parent_content_id UUID,
    title, subtitle, excerpt, content TEXT,
    content_blocks JSONB DEFAULT '[]',
    featured_image_id UUID REFERENCES cms_assets(id),
    og_image_id UUID REFERENCES cms_assets(id),
    gallery_ids UUID[],
    meta_title VARCHAR(70), meta_description VARCHAR(160), meta_keywords TEXT[],
    canonical_url, robots_directives, structured_data JSONB,
    author_id UUID REFERENCES cms_users(id), contributors UUID[],
    status cms_content_status DEFAULT 'draft',
    published_at, scheduled_publish_at, scheduled_unpublish_at TIMESTAMPTZ,
    primary_category_id UUID, categories UUID[],
    custom_fields JSONB,
    template VARCHAR(100),
    deleted_at TIMESTAMPTZ,                  -- soft delete
    version INTEGER NOT NULL DEFAULT 1,      -- optimistic locking
    created_at, updated_at, created_by, updated_by,
    UNIQUE (content_type, slug, locale)
);
```

Plus, in the same/follow-on migrations:

| Table | Purpose |
|---|---|
| `cms_users` | CMS profile + role (FK → main `users.id`) |
| `cms_assets` | DAM: filename, mime, size, R2 storage_key, cdn_url, blurhash, dominant_color, variants[], usage_count, alt_text, caption, tags[] |
| `cms_asset_folders` | Hierarchical (depth, path, parent_id) |
| `cms_revisions` | revision_number, is_current, JSONB snapshot, change_summary, changed_fields[], + `change_type`, `word_count`, `diff_stats` (added in 025) |
| `cms_tags` / `cms_content_tags` | UUID taxonomy w/ hierarchy + usage_count |
| `cms_content_relations` | source/target/type (e.g. related_posts) |
| `cms_workflow_log` | Audit trail for status transitions |
| `cms_reusable_blocks` / `cms_reusable_block_usage` | (025) Saved block templates with usage tracking |
| `cms_user_editor_preferences` | (025) Autosave interval, focus mode, daily word goal, custom_shortcuts JSONB, theme |
| `cms_ai_assist_history` | (025) AI ops + token counts + latency + applied? |
| `cms_offline_sync_queue` | (025) Offline edit replay with conflict detection |
| `cms_comment_notifications` | (025) Mention/reply notifications |
| Plus tables in 027/041/042 (datasources, scheduling/releases, presets, enterprise features) |

**Row Level Security** is enabled on `cms_reusable_blocks`, `cms_user_editor_preferences`, `cms_ai_assist_history`, `cms_offline_sync_queue` ([025:489-523](api/migrations/025_blog_editor_enhancements.sql#L489-L523)). Policies use functions `cms_current_user_id()` / `cms_current_user_role()`.

### Media storage

Two paths exist — a clear forking point:

| Endpoint | Frontend route | Storage destination |
|---|---|---|
| `/api/cms/upload/image` | [frontend/src/routes/api/cms/upload/image/+server.ts](frontend/src/routes/api/cms/upload/image/+server.ts) | **Local disk** `static/uploads/cms/` via `fs.writeFile` — broken on Cloudflare Pages (read-only FS). 10 MB limit. JPEG/PNG/GIF/WebP/AVIF. |
| `/api/admin/media/upload` (used by `mediaApi` and BlockEditor's `uploadImage`) | proxied to Rust [api/src/routes/media.rs](api/src/routes/media.rs) | Cloudflare R2 (S3-compatible) per `cms_assets.storage_provider = 'r2'`. |

The block editor's uploader hits `/api/admin/media/upload` (correct path); the older `/api/cms/upload/image` endpoint is the buggy one.

Image enhancements stored: `width`, `height`, `aspect_ratio`, `blurhash`, `dominant_color`, `variants[]` (responsive sizes), `alt_text`, `caption`, `seo_title`, `seo_description`, `tags[]`.

---

## 3. CURRENT EDITOR

### What's actually wired up

- Real editor used in production admin = [frontend/src/lib/components/blog/BlockEditor/BlockEditor.svelte](frontend/src/lib/components/blog/BlockEditor/BlockEditor.svelte) (2,892 lines). Embedded by both `/admin/blog/create` and `/admin/blog/edit/[id]`.
- It is a **block-based editor**, not a single rich-text WYSIWYG. Each block is its own component. Inline text editing happens via `contenteditable` inside `ParagraphBlock.svelte`, `HeadingBlock.svelte`, etc.
- The standalone [`/cms/editor`](frontend/src/routes/cms/editor/+page.svelte) page is a **352-line in-memory demo** that creates blocks but never persists them. It is what the README's "broken add-block click" line refers to (loose `onclick` on a `<button>` inside a focusable parent — accessibility issue, not a functional one). **Not used by any real workflow.**

### Block vocabulary

Defined in [BlockEditor/types.ts:14-69](frontend/src/lib/components/blog/BlockEditor/types.ts#L14-L69) — 40+ types across categories:

- **Text:** paragraph, heading, quote, pullquote, code, preformatted, list, checklist
- **Media:** image, gallery, video, audio, file, embed, gif
- **Layout:** columns, group, separator, divider, spacer, row
- **Interactive:** button, buttons, accordion, tabs, toggle, toc
- **Trading:** ticker, chart, priceAlert, tradingIdea, riskDisclaimer
- **Advanced:** callout, card, testimonial, cta, countdown, socialShare, author, relatedPosts, newsletter
- **AI:** aiGenerated, aiSummary, aiTranslation
- **Custom/Dynamic:** shortcode, **html**, reusable

The Postgres `cms_block_type` enum ([023:97-122](api/migrations/023_custom_cms_implementation.sql#L97-L122)) is a smaller, page-builder-oriented overlap (hero-slider, blog-feed, etc.) — these are not the same vocabularies. **Block content is stored as opaque JSONB**, so the enum mismatch isn't currently fatal.

### Features the editor advertises (per source comments + sibling files)

- 30+ block types, drag & drop reorder
- Undo / redo (unlimited)
- Device preview (desktop / tablet / mobile)
- AI assistant (Claude-powered via `cms_ai_assist.rs` + `@anthropic-ai/sdk`)
- Real-time SEO analyzer (`SEOAnalyzer.svelte`)
- Revision history (`RevisionHistory.svelte` + `cms_revisions`)
- Keyboard shortcuts (`KeyboardShortcuts.svelte`, customizable via `cms_user_editor_preferences.custom_shortcuts`)
- Performance metrics + reporter (web vitals, render p95)
- Block error boundaries + recovery
- Virtual scrolling for huge documents (`VirtualBlockList.svelte`)
- **Yjs collaborative editing scaffolding** ([BlockEditor/collaboration/yjs-provider.ts:17-19](frontend/src/lib/components/blog/BlockEditor/collaboration/yjs-provider.ts#L17-L19) — `yjs` + `y-websocket` + `y-indexeddb`). Whether the WS server endpoint is actually live in production is unverified.
- Offline queue (`cms_offline_sync_queue` + `BlockEditor/offline/`)
- Scheduling panel (`SchedulingPanel.svelte`)
- Asset manager (`AssetManager.svelte`), block inserter (`BlockInserter.svelte`), settings panel
- Reusable blocks library, global components, presets

### How saves work

Admin pages compose `post.content_blocks` from BlockEditor output and POST/PUT through the `/api/admin/posts` proxy. Example from [admin/blog/create/+page.svelte:189-194](frontend/src/routes/admin/blog/create/+page.svelte#L189-L194):

```ts
post.content_blocks = blocks.map((b) => ({
    type: b.type,
    content: b.content,
    settings: b.settings
}));
```

Then a JSON POST to `/api/admin/posts` (the SvelteKit proxy at [api/admin/posts/+server.ts](frontend/src/routes/api/admin/posts/+server.ts)) → adds `Bearer rtp_access_token` cookie/header → forwards to Rust `POST /admin/posts` ([posts.rs:227-305](api/src/routes/posts.rs#L227-L305)) → SQL INSERT into `posts`.

The **edit page also has an unsaved-changes guard** (`beforeNavigate` + `beforeunload` listener, [admin/blog/edit/[id]/+page.svelte:124-150](frontend/src/routes/admin/blog/edit/[id]/+page.svelte#L124-L150)) and a slug-uniqueness probe with debounce.

### Media uploads from the editor

- BlockEditor uses [BlockEditor/upload/uploader.ts](frontend/src/lib/components/blog/BlockEditor/upload/uploader.ts).
- Client-side: 10 MB limit; image type validation; thumbnail / dimensions / dominant color extraction (`upload/image-processor.ts`); `UploadController` for cancellation.
- Posts to `/api/admin/media/upload` → Rust `media.rs` → R2.
- Alt text, caption, title, description are first-class fields on `cms_assets` and on the create-post `featured_*` form state.

### What's missing or broken in the current editor

- **No public renderer for ~35 of the 40+ block types.** The blog detail page only renders 6 (paragraph, heading, list, quote, code, image). Everything else (callout, cta, columns, video, button, accordion, gallery, ticker, chart, accordion, tabs, …) saves fine but disappears on view.
- **Block payload shape mismatch.** Editor saves `{ type, content: {...} }`; public renderer reads `block.data?.text`. The branch at [blog/[slug]/+page.svelte:332](frontend/src/routes/blog/[slug]/+page.svelte#L332) silently shows empty for blocks created with the new editor unless the backend / a layer translates `content` ↔ `data`. Worth verifying with one round-trip.
- **No revision history wired for legacy `posts`.** `cms_revisions` exists for `cms_content` only.
- **No autosave on the active blog admin pages** (the v2 `cms_user_editor_preferences.autosave_interval_seconds` exists in DB but the admin/blog flow uses manual Save buttons).
- **No "view source / HTML" toggle at the editor level** — but `HtmlBlock` ([cms/blocks/advanced/HtmlBlock.svelte:31-115](frontend/src/lib/components/cms/blocks/advanced/HtmlBlock.svelte#L31-L115)) does have `edit | preview | split` view modes per block. So the primitive exists; what's missing is a top-level toggle that converts the entire block list to a flat HTML string (and back).
- **`/cms/editor` page is dead weight** — 352 lines that never persist. Either delete or wire it to `cms_content`.
- **Image upload endpoint duplication.** `/api/cms/upload/image` writes to local disk; will break on Cloudflare Pages. Real editor uses `/api/admin/media/upload` (R2). The local-disk one looks like an early prototype that never got removed.
- **Public renderer falls back to `<p class="no-content">Content coming soon...</p>`** when blocks are absent — there's **no HTML-string fallback**, even though `posts.content` doesn't exist (only `content_blocks` is on the legacy table; `cms_content.content TEXT` exists on the v2 table but isn't the source).

---

## 4. API / BACKEND

### Stack

Backend is **Rust + Axum 0.7**, plus SvelteKit `+server.ts` proxies (every proxy reads `env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'`). The proxies forward auth (`rtp_access_token` cookie or `Authorization` header).

### Blog-related endpoints

Wired in [api/src/routes/mod.rs:95-96, 117-149, 209-234](api/src/routes/mod.rs#L95):

#### Legacy posts (active)

| Method | Path | Auth | Source |
|---|---|---|---|
| GET | `/api/posts` | public | [posts.rs:93](api/src/routes/posts.rs#L93) (status='published' default) |
| GET | `/api/posts/:slug` | public | [posts.rs:201](api/src/routes/posts.rs#L201) |
| GET | `/api/posts/:slug/related` | public | [posts.rs:592](api/src/routes/posts.rs#L592) |
| GET | `/api/admin/posts` | admin (`AdminUser`) | [posts.rs:666](api/src/routes/posts.rs#L666) |
| POST | `/api/admin/posts` | admin | [posts.rs:227](api/src/routes/posts.rs#L227) |
| GET | `/api/admin/posts/:id` | admin | [posts.rs:478](api/src/routes/posts.rs#L478) |
| PUT | `/api/admin/posts/:id` | admin | [posts.rs:308](api/src/routes/posts.rs#L308) |
| DELETE | `/api/admin/posts/:id` | admin | [posts.rs:426](api/src/routes/posts.rs#L426) |
| POST | `/api/admin/posts/:id/publish` | admin | [posts.rs:504](api/src/routes/posts.rs#L504) |
| POST | `/api/admin/posts/:id/unpublish` | admin | [posts.rs:536](api/src/routes/posts.rs#L536) |
| POST | `/api/admin/posts/:id/archive` | admin | [posts.rs:564](api/src/routes/posts.rs#L564) |

#### Categories / tags / media

- `/api/admin/categories/*` ← `categories.rs`
- `/api/admin/tags/*` ← `tags.rs`
- `/api/admin/media/*` ← `media.rs` (1,269 lines — uploads, AI tagging, analytics)

#### v2 CMS

- `/api/admin/cms-v2/*` and `/api/cms/*` ← `cms_v2.rs`
- `/api/admin/cms-v2/enterprise/*` ← `cms_v2_enterprise.rs`
- `/api/cms/revisions/*` ← `cms_revisions.rs`
- `/api/cms/ai/*` ← `cms_ai_assist.rs` (Claude-backed)
- `/api/cms/seo/*` ← `cms_seo.rs`
- `/api/cms/reusable-blocks/*` (admin + public)
- `/api/cms/presets/*` (admin + public)
- `/api/cms/assets/*` ← `cms_assets.rs`
- `/api/cms/scheduling/*`
- `/api/cms/datasources/*` (admin + public)
- `/api/cms/global-components/*` (admin + public)
- `/api/delivery/*` ← `cms_delivery.rs`

### SvelteKit proxies to add to your mental model

| Proxy | Purpose |
|---|---|
| [/api/admin/posts/+server.ts](frontend/src/routes/api/admin/posts/+server.ts) | GET/POST list + create |
| /api/admin/posts/[...rest]/+server.ts | catch-all for /:id, /:id/publish, etc. |
| /api/admin/posts/stats/+server.ts | dashboard counts |
| /api/admin/categories/+server.ts (+ [...rest]) | |
| /api/admin/tags/+server.ts (+ [...rest]) | |
| /api/admin/media/{upload,ai,analytics,[id]}/+server.ts | |
| /api/cms/upload/image/+server.ts | ⚠ writes to local disk; Cloudflare-incompatible |
| /api/cms/ai/+server.ts, /api/cms/newsletter/+server.ts | |

### Authentication

- Custom JWT-based auth (per `auth.rs` + `oauth.rs` + Redis-backed JWT blacklist).
- Frontend cookie: `rtp_access_token` (HttpOnly) — see [api/admin/posts/+server.ts:46](frontend/src/routes/api/admin/posts/+server.ts#L46), recently fixed (FIX-2026-04-26 in comments) so all admin proxies prefer cookie over `Authorization` header.
- Admin gate on Rust side: `AdminUser` extractor (`crate::middleware::admin::AdminUser`). Every admin route requires it.
- Admin gate on **frontend** side: per top README §"Top blockers" #4, "/admin frontend has no role gate" — i.e. anyone authenticated can render the admin UI; Rust will reject mutations, but the UI shell shows up. Worth confirming before treating as a security boundary.
- v2 CMS uses RLS + `cms_user_role` enum (super_admin / marketing_manager / content_editor / weekly_editor / developer / viewer). The legacy `posts` table has only the binary admin gate.

### Auth ↔ blog admin

There is **no per-role permission for blog publishing on the legacy path** — anyone in the `AdminUser` set can `POST /admin/posts`, `:id/publish`, etc. The granular content_editor / weekly_editor / approved-status workflow exists in the v2 schema (`cms_content_status` includes `in_review`, `approved`) but is not wired into the legacy `posts` flow.

---

## 5. PUBLIC RENDERING

### URL → route

- `/blog` → [frontend/src/routes/blog/+page.svelte](frontend/src/routes/blog/+page.svelte) (paginated list).
- `/blog/[slug]` → [frontend/src/routes/blog/[slug]/+page.svelte](frontend/src/routes/blog/[slug]/+page.svelte). SSR enabled (`prerender = false`, but server-rendered for SEO — see [+page.ts:10-11](frontend/src/routes/blog/[slug]/+page.ts#L10-L11)).
- Slug is just a SvelteKit dynamic param; it's resolved server-side via `apiFetch(API_ENDPOINTS.posts.single(slug))` → Rust `GET /posts/:slug`.

### How the body renders

Inline switch in [blog/[slug]/+page.svelte:330-378](frontend/src/routes/blog/[slug]/+page.svelte#L330-L378):

```svelte
{#each post.content_blocks as block}
    {#if block.type === 'paragraph'}
        <p>{@html sanitizeBlogContent(block.data?.text || '')}</p>
    {:else if block.type === 'heading'}
        <h2>{@html sanitizeBlogContent(block.data?.text || '')}</h2>      // h1..h5 by level
    {:else if block.type === 'list'}
        <ul>{#each block.data?.items as item}<li>{@html ...}</li>{/each}</ul>
    {:else if block.type === 'quote'}     <blockquote>...</blockquote>
    {:else if block.type === 'code'}      <pre><code>{block.data?.code}</code></pre>
    {:else if block.type === 'image'}     <figure><img .../></figure>
    {/if}
{/each}
```

### Sanitization layer

[frontend/src/lib/utils/sanitize.ts](frontend/src/lib/utils/sanitize.ts) wraps `isomorphic-dompurify@3.10.0`. Four profiles:

| Profile | Allowed tags | Used for |
|---|---|---|
| `strict` | b, i, em, strong, u, s, br, p, span | form labels |
| `standard` | + h1–h6, lists, blockquote, pre, code, a, img, figure, table | popups, video overlays |
| `rich` | + video, source, audio, iframe (only YouTube/Vimeo via `afterSanitizeAttributes` hook) | **blog content** |
| `minimal` | b, i, em, strong, br | text snippets |

Hooks: forces `rel="noopener noreferrer"` + `target="_blank"` on external links; strips iframes from non-trusted sources.

### SEO + structured data

- `<SEOHead>` component invoked in [blog/[slug]/+page.svelte:184-194](frontend/src/routes/blog/[slug]/+page.svelte#L184-L194), with `title`, `description`, `canonical`, `ogType="article"`, `ogImage`, `author`, `publishedTime`, `schema`.
- Per-post JSON-LD: built in [+page.ts:56-79](frontend/src/routes/blog/[slug]/+page.ts#L56-L79) via `articleSchema()` + `breadcrumbSchema()` from [`$lib/seo/jsonld`](frontend/src/lib/seo/jsonld.ts). `post.schema_markup` (a JSONB column) is used if set, otherwise a default Article schema is constructed.
- meta_title / meta_description / canonical_url / robots all surfaced from the post row.
- og_image: `post.featured_image` (legacy) or v2 `og_image_id` → asset.

### RSS / sitemap / OG image / structured data

- **RSS:** [frontend/src/routes/feed.xml/](frontend/src/routes/feed.xml) and `atom.xml`.
- **Sitemap:** [/sitemap.xml/](frontend/src/routes/sitemap.xml) + `news-sitemap.xml`, `video-sitemap.xml`. Uses `super-sitemap@^1.0.12`.
- **`/robots.txt`** present.
- **OG image generation:** I did not find a dynamic OG image generator (e.g. Satori). Static `featured_image` URL is used.
- **Structured data:** JSON-LD article + breadcrumb only; no FAQ/HowTo/VideoObject schemas wired (despite faq blocks existing in the editor vocabulary).

---

## 6. PUBLISHING WORKFLOW

| Capability | Legacy `posts` | v2 `cms_content` | Surfaced in admin/blog UI? |
|---|---|---|---|
| Draft / published / archived | ✅ | ✅ + `in_review` + `approved` + `scheduled` | Draft/Published/Archived only |
| Per-post publish/unpublish/archive endpoint | ✅ | ✅ | ✅ |
| Scheduled publishing | column missing on `posts.published_at` semantics | `scheduled_publish_at` + `scheduled_unpublish_at` | UI has a "schedulePost" / `showScheduleModal` state ([admin/blog/+page.svelte:99](frontend/src/routes/admin/blog/+page.svelte#L99)) — surface exists, backing for legacy posts is unclear (no scheduler cron found for `posts`) |
| Revision history | ❌ no revisions table | ✅ `cms_revisions` + `cms_compute_revision_diff()` | RevisionHistory component exists (in BlockEditor) but it presumably reads `cms_revisions` keyed by `content_id`, not `posts.id` |
| Autosave | ❌ (manual save) | ✅ `cms_user_editor_preferences.autosave_interval_seconds` (default 10s) | Not wired in the create/edit pages; they have manual Save + unsaved-changes guard |
| Offline queue + conflict resolution | ❌ | ✅ `cms_offline_sync_queue` | not surfaced |
| AI assist | n/a | ✅ `cms_ai_assist_history` + `/api/cms/ai/*` | `AIAssistant.svelte` exists in BlockEditor |
| Reusable blocks | n/a | ✅ `cms_reusable_blocks` | seeded with 6 defaults (CTA primary, trade setup, 2-column, info/warning callouts, perf stats) |
| Comments / @mentions | n/a | ✅ `cms_comments` + `cms_comment_notifications` (025) | not surfaced in blog admin |
| Workflow audit log | n/a | ✅ `cms_workflow_log` | not surfaced |

**What works today:** create/edit/delete/list/publish/unpublish/archive of legacy posts; SSR rendering; SEO; sitemap; tags; categories; media upload to R2; rich block editor for authoring (when its output happens to be one of the 6 renderable types).

**What's broken / unfinished:**
- The `/cms/editor` standalone demo (README's "broken add-block click").
- `/api/cms/upload/image` writes to local disk (Cloudflare-incompatible).
- The block-shape mismatch between editor (`content`) and renderer (`data`) — saves succeed but display loses data for new blocks unless something normalizes.
- Most editor block types have no public renderer.
- No revision history, autosave, or scheduling for legacy posts (only for v2 `cms_content`).
- The two systems aren't bridged: `posts` rows aren't mirrored to `cms_content` and vice versa. v2 features can't reach legacy posts.

---

## 7. EXISTING CONTENT

### Counts (local Docker DB, 2026-04-27)

```
posts                                 0
cms_content WHERE content_type='blog_post'  0
categories                            0
tags                                  0
cms_assets                            0
```

The local DB is empty. No seed migrations insert into `posts` or `cms_content`. **Production DB is unreachable** per [README.md:7](README.md#L7), so no live count is available.

### Format

- Legacy `posts.content_blocks`: JSONB array of `{ type, data: {...} }`. No HTML or Markdown column.
- v2 `cms_content.content_blocks`: JSONB array of editor-shaped blocks (`{ type, content, settings, metadata }`). v2 also has a `content TEXT` column that isn't currently populated by the admin flow.
- Featured images: URL string (legacy) or `cms_assets` UUID FK (v2).

### Sample post content

There are no real posts. The closest representative content is the seed for **reusable blocks** in [025_blog_editor_enhancements.sql:531-555](api/migrations/025_blog_editor_enhancements.sql#L531-L555) — JSONB structure example:

```json
{
  "blockType": "cta-banner",
  "data": {
    "headline": "Ready to Start Trading?",
    "description": "Join thousands of successful traders",
    "buttonText": "Get Started",
    "buttonUrl": "/pricing",
    "variant": "primary"
  }
}
```

```json
{
  "blockType": "trade-setup",
  "data": {
    "ticker": "",
    "direction": "long",
    "entryPrice": null,
    "stopLoss": null,
    "targets": [],
    "rationale": "",
    "timeframe": "swing"
  }
}
```

Note these reusable-block samples use the older `data` shape (matching the public renderer), not the newer `content/settings` shape from `BlockEditor/types.ts`. Two formats coexisting.

---

## 8. DEPENDENCIES & PACKAGES

### Editor / blog / CMS dependencies in `frontend/package.json`

| Package | Version | Notes |
|---|---|---|
| `@anthropic-ai/sdk` | ^0.91.1 | AI assist (Claude) — current |
| `dompurify` | 3.4.1 | HTML sanitizer (pinned) |
| `isomorphic-dompurify` | ^3.10.0 | SSR-safe wrapper |
| `diff-match-patch` | ^1.0.5 | Revision diff |
| `@types/diff-match-patch` | ^1.0.36 | |
| `yjs` | ^13.6.30 | Collaborative CRDT |
| `y-websocket` | ^3.0.0 | WS provider |
| `y-indexeddb` | ^9.0.12 | Offline persistence |
| `y-protocols` | ^1.0.7 | Awareness/sync helpers |
| `blurhash` | ^2.0.5 | Featured-image LQIP |
| `tus-js-client` | ^4.3.1 | Resumable uploads (likely Bunny video) |
| `@vimeo/player` | ^2.30.3 | Video block |
| `lightweight-charts` | ^5.2.0 | Chart block |
| `embla-carousel*` | 8.6.0 | Gallery / slider blocks |
| `super-sitemap` | ^1.0.12 (devDep) | Sitemap generator |
| `dompurify` ↔ `isomorphic-dompurify` | both pinned to 3.x | **DOMPurify 3 is current major. No deprecation.** |

### Notable SDKs not in the editor scope

- `@stripe/stripe-js`, `lottie-web`, `gsap`, `three`, `@threlte/*`, `d3*`, `tailwindcss@4` — unrelated to blog editor.
- `bits-ui@^2.18.0` (devDep, Svelte 5) — used in admin UI.
- `@tabler/icons-svelte-runes@^3.41.1` — Svelte-5 runes-compatible icons (used everywhere in admin/blog).

### Deprecated / abandoned / outdated

- **Nothing major flagged.** The stack is unusually current for the date — Svelte 5.55, SvelteKit 2.58, Vite 7, Tailwind 4, TypeScript 6.0, Yjs 13, DOMPurify 3.4.
- `dompurify` is **pinned exact** to `3.4.1` (no caret) — possibly intentional for security review reproducibility, possibly drifting. Consider whether the pin should track a known-good range.
- `prettier-plugin-svelte@^3.5.1` and `eslint-plugin-svelte@^3.17.1` are aligned with Svelte 5.

### Svelte 4 → Svelte 5 migration concerns

Per [CLAUDE.md](CLAUDE.md):

> 41 components were migrated off the legacy pattern in commit `05acf3231`. The repo's typecheck must stay at 0 errors / 0 warnings.

The README confirms `pnpm check` passes 8,799 files / 0 errors. Concrete signals across the editor surface I read:
- Uses `$state`, `$derived`, `$derived.by`, `$effect`, `$props`, `$bindable` correctly.
- Uses `crypto.randomUUID()` for client block IDs.
- Uses `flip` + `fade` from `svelte/animate` & `svelte/transition`.
- Some files have `// @ts-ignore write-only state` annotations on props that are written but not read — clean if intentional.
- A few legacy patterns I'd review: in [admin/blog/+page.svelte:75](frontend/src/routes/admin/blog/+page.svelte#L75), `let posts = $state<any[]>([])` uses `any` extensively; admin/blog files have several `as any` casts. Tightening those would help the editor refactor land cleanly.

---

## 9. PAIN POINTS & GAPS

### What's solid and works well

- **DB schema design** (v2): UUID PKs, soft deletes, optimistic locking, audit columns, RLS policies, hierarchical taxonomy, scheduling timestamps, multi-locale support — production-grade.
- **Sanitization architecture**: profile-based DOMPurify wrapper, hooks, trusted iframe domains.
- **Auth proxy hardening**: cookie-first with header fallback (FIX-2026-04-26), parameterized SQL throughout `posts.rs`.
- **Legacy CRUD path**: stable, slug-uniqueness handled, status transitions exposed.
- **SEO surface**: structured `articleSchema`, breadcrumbSchema, robots directives column, canonical URL field, meta_title/meta_description in DB and in load functions.
- **Block editor breadth**: 40+ block types, AI, SEO analyzer, revisions, Yjs, virtual scrolling, performance tracking — the bones of an Elementor-class editor are here.

### What's incomplete or hacky

- **Two incompatible block payload shapes coexist** (`{type,data}` vs `{type,content,settings,metadata}`). Editor produces one, public renderer consumes the other, reusable-block seeds use a third hybrid. **Highest-risk piece for any editor change.**
- **Public renderer covers ~6 of 40+ block types.** Most blocks save and disappear.
- **Two upload paths** — `/api/cms/upload/image` (local disk, Cloudflare-incompatible) and `/api/admin/media/upload` (R2). Delete or fix the disk one.
- **`/cms/editor` standalone page** is dead code (no save/load, listed as "broken" in README).
- **Two parallel content systems** (`posts` vs `cms_content`) with no bridge. v2 features (revisions, scheduling, AI history, comments, RLS, workflow log) cannot reach legacy posts.
- **Frontend admin role gate missing** (per README's blocker list).
- **Schedule-modal UI exists for legacy posts but has no scheduler.** No cron / worker found that publishes scheduled drafts on the `posts` table.
- **`p.category_id` referenced** in `get_related_posts` SQL despite the column not existing on `posts`; the query is wrapped in `unwrap_or_default()` so it silently returns empty — masking a bug.
- **Heavy reliance on `as any` casts in admin/blog edit page** (e.g. tag normalization at [edit/[id]/+page.svelte:187](frontend/src/routes/admin/blog/edit/[id]/+page.svelte#L187)) suggests the API contract is in flux.

### What's missing entirely

- A **single-string HTML body** column or rendering path. There is no `posts.body_html` or canonical "render anything we don't recognize as raw HTML" fallback.
- A **public renderer that uses the same `BlockRenderer.svelte`** as the admin (`cms/blocks/BlockRenderer.svelte` exists but isn't called from `/blog/[slug]`).
- **Markdown support** — no Markdown parser dep (no `marked` / `remark` / `mdsvex` / `unified`).
- **Dynamic OG image generation.**
- **Public sitemap entries for `cms_content`** (only legacy `posts` are indexed today, presumably).
- **Bridge / migration script** between the two systems.

### What would need to change to add a WordPress-Classic-style editor with visual ↔ HTML toggle

Two viable architectures, depending on how much of the existing block system you want to keep:

**Option A — Add a top-level "HTML view" mode that flattens blocks ↔ HTML.**
- Re-use the existing **`HtmlBlock`** primitive (already supports edit/preview/split). Build a top-level toggle that:
  1. Serializes the entire block list to a single HTML string (using each block's `BlockRenderer`).
  2. Lets the user edit raw HTML.
  3. Re-parses HTML back into blocks (paragraphs / headings / lists / images / blockquote / pre+code) and treats anything unrecognized as a single `html` block.
- The block payload shape mismatch still has to be resolved first, otherwise round-trips will lose data.

**Option B — Replace the per-block contenteditable model with a single rich-text editor (TipTap / ProseMirror / Lexical) per post.**
- Introduces a real "Visual / Text" toggle the way WordPress Classic does.
- Schema becomes `posts.body_html TEXT` (or stays `content_blocks` as a single `{type:'html'}` block).
- Loses the block-specific UX (drag-reorder, per-block settings panel, AI assist scoped per block) unless TipTap nodes are wrapped to preserve it.
- Yjs already integrates well with TipTap — the existing `yjs-provider.ts` could be repointed.

Either way, the **work is dominated by these prerequisites**, not by the toggle itself:

1. **Decide which content table is canonical** going forward (`posts` or `cms_content`). Bridge or sunset the other.
2. **Unify the block payload shape** end-to-end. Pick `{type, content, settings, metadata}` (editor-native) and update the public renderer + reusable-block seeds, OR pick `{type, data}` and update the editor mapping. Add a server-side migration for any existing rows.
3. **Build a public `BlockRenderer`** that mirrors the admin one, so all 40+ block types render publicly.
4. **Delete or wire `/cms/editor`** and **delete `/api/cms/upload/image`** (the local-disk one).
5. **Add `featured_media_id` FK on `posts`** (already in editor state, nowhere in the table).
6. **Decide whether revisions, autosave, scheduling apply to `posts`** — and either add them or commit to migrating to `cms_content`.

Then the visual ↔ HTML toggle is a 1–2 day feature on top of either Option A or B.

---

## 10. QUESTIONS FOR THE USER

These are ambiguities the code can't resolve:

1. **Which content system is canonical going forward — `posts` (legacy) or `cms_content` (v2)?** The v2 system is far richer but unwired for the public blog; the legacy system is what users actually read. Plan depends entirely on this answer.
2. **Are there any blog posts in production?** Local DB is empty, prod DB is down. If prod has many posts, migration story matters; if it's empty/test data, you have free hands.
3. **Is the Yjs WebSocket server actually running?** I see the client provider in `BlockEditor/collaboration/yjs-provider.ts` but didn't find a server in the Rust API (`y-websocket` typically needs a separate Node WS server). Is collab a real feature or scaffolding?
4. **What does "WordPress Classic" mean here precisely?** (a) A single TipTap/ProseMirror editor with a Visual / Text tab, with no blocks; (b) the current block editor *plus* a global "view source HTML" toggle layered on top; (c) something else (Gutenberg-classic blend)?
5. **Is the editor's existing AI / SEO / Revisions / Yjs / virtual-scroll surface a feature you want to keep**, or are you fine collapsing those if it simplifies a TipTap-based rewrite?
6. **Is `/cms/editor` (the standalone demo) intended as a future home page for the editor, or dead code to delete?** README treats it as a real (broken) feature.
7. **Should `/api/cms/upload/image` be deleted now**, or are there callers I haven't found? It's incompatible with the Cloudflare Pages frontend.
8. **For Markdown support** — is there any plan/desire to support Markdown round-trip (some authors prefer it), or is HTML-only the goal?
9. **Production deploy target for the frontend** — Cloudflare Pages (per README and `adapter-cloudflare`) or Node? This affects the upload-path decision and whether server-side filesystem access is ever an option.
10. **Does `posts.featured_image` (string) need to migrate to `featured_media_id` (FK to `cms_assets`)** so alt text/blurhash/variants come from the DAM rather than ad-hoc?
11. **Block payload format** — when I say "two shapes coexist," is that already known and tracked somewhere, or is this report the first surfacing? It would change the priority of fixing it.
12. **Auth model for blog publishing** — should we adopt the v2 `cms_user_role` workflow (draft → in_review → approved → published, with role-gated transitions), or keep the binary "any admin can publish" model on legacy posts?

---

## Appendix A — File Index (for quick navigation during planning)

| Concern | File |
|---|---|
| Public blog list | [frontend/src/routes/blog/+page.svelte](frontend/src/routes/blog/+page.svelte) |
| Public blog detail (renderer) | [frontend/src/routes/blog/[slug]/+page.svelte](frontend/src/routes/blog/[slug]/+page.svelte) |
| Public blog detail (SSR loader + JSON-LD) | [frontend/src/routes/blog/[slug]/+page.ts](frontend/src/routes/blog/[slug]/+page.ts) |
| Admin blog list | [frontend/src/routes/admin/blog/+page.svelte](frontend/src/routes/admin/blog/+page.svelte) |
| Admin blog create | [frontend/src/routes/admin/blog/create/+page.svelte](frontend/src/routes/admin/blog/create/+page.svelte) |
| Admin blog edit | [frontend/src/routes/admin/blog/edit/[id]/+page.svelte](frontend/src/routes/admin/blog/edit/[id]/+page.svelte) |
| Standalone editor demo (broken/dead) | [frontend/src/routes/cms/editor/+page.svelte](frontend/src/routes/cms/editor/+page.svelte) |
| Real editor component | [frontend/src/lib/components/blog/BlockEditor/BlockEditor.svelte](frontend/src/lib/components/blog/BlockEditor/BlockEditor.svelte) |
| Block type definitions | [frontend/src/lib/components/blog/BlockEditor/types.ts](frontend/src/lib/components/blog/BlockEditor/types.ts) |
| Block registry exports | [frontend/src/lib/components/blog/BlockEditor/index.ts](frontend/src/lib/components/blog/BlockEditor/index.ts) |
| HTML block (per-block source toggle) | [frontend/src/lib/components/cms/blocks/advanced/HtmlBlock.svelte](frontend/src/lib/components/cms/blocks/advanced/HtmlBlock.svelte) |
| Yjs provider | [frontend/src/lib/components/blog/BlockEditor/collaboration/yjs-provider.ts](frontend/src/lib/components/blog/BlockEditor/collaboration/yjs-provider.ts) |
| Sanitizer | [frontend/src/lib/utils/sanitize.ts](frontend/src/lib/utils/sanitize.ts) |
| Upload pipeline | [frontend/src/lib/components/blog/BlockEditor/upload/uploader.ts](frontend/src/lib/components/blog/BlockEditor/upload/uploader.ts) |
| Posts proxy | [frontend/src/routes/api/admin/posts/+server.ts](frontend/src/routes/api/admin/posts/+server.ts) |
| Image upload (local-disk, broken) | [frontend/src/routes/api/cms/upload/image/+server.ts](frontend/src/routes/api/cms/upload/image/+server.ts) |
| Posts route (Rust) | [api/src/routes/posts.rs](api/src/routes/posts.rs) |
| Media route (Rust) | [api/src/routes/media.rs](api/src/routes/media.rs) |
| CMS v2 route (Rust) | [api/src/routes/cms_v2.rs](api/src/routes/cms_v2.rs) |
| Router wiring | [api/src/routes/mod.rs](api/src/routes/mod.rs) |
| Initial schema (legacy posts) | [api/migrations/001_initial_schema.sql](api/migrations/001_initial_schema.sql) |
| Custom CMS v2 schema | [api/migrations/023_custom_cms_implementation.sql](api/migrations/023_custom_cms_implementation.sql) |
| Editor enhancements (reusable blocks, autosave prefs, AI history, offline queue) | [api/migrations/025_blog_editor_enhancements.sql](api/migrations/025_blog_editor_enhancements.sql) |
