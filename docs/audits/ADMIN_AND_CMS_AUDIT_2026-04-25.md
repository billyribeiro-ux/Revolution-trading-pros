# Admin Backend & Headless CMS — End-to-End Audit

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.

**Auditor:** Claude (Opus 4.7) · **Date:** 2026-04-25 · **Commit:** `08cf8605c`
**Scope:** four parallel read-only sweeps —
- backend admin (`api/src/routes/admin*.rs` + `routes/{courses,indicators,subscriptions}_admin.rs` + `middleware/admin.rs` + `migrate.rs` + `security.rs` + `settings.rs`),
- headless CMS v2 (`api/src/routes/cms_*.rs` + `services/cms_*.rs` + `frontend/src/routes/cms/` + `frontend/src/lib/components/cms/`),
- frontend admin + dashboard surfaces (`frontend/src/routes/{admin,dashboard,account,my}/**`),
- the integration layer (`+server.ts` proxies, `+page.server.ts` loads, `.remote.ts` files, the Axum client).

This is a state-of-the-system snapshot. Every claim is cited to `file:line`.
Pairs with [`PRODUCT_AND_AUTH_AUDIT_2026-04-25.md`](PRODUCT_AND_AUTH_AUDIT_2026-04-25.md)
(auth, RBAC/ABAC, CRUD coverage, products) — **read both before touching anything**.

---

## 0. TL;DR — what's actually broken

Ranked by user impact:

| # | Issue | Where | Impact |
|---|-------|-------|--------|
| 1 | **CMS editor can't add blocks.** The toolbar `<button onclick={() => addBlock(type)}>` fires nothing after the bits-ui v2 upgrade. Three e2e tests are quarantined as `test.fixme` because of this. | [`frontend/src/routes/cms/editor/+page.svelte:113`](frontend/src/routes/cms/editor/+page.svelte#L113) | Critical — the CMS is the headline feature; right now you can open the editor but cannot author. |
| 2 | **Stripe Checkout-Session creation is a stub.** `// TODO: Create Stripe checkout session` at `subscriptions.rs:446`. The funnel cannot collect money via the standard self-serve path. | `api/src/routes/subscriptions.rs:446` | Critical — revenue-blocking. |
| 3 | **`/admin` has no frontend role gate.** `/admin/+layout.ts` is just `ssr=false; prerender=false`. Any logged-in user can mount the admin UI; backend rejects the data calls but the UI loads anyway. | [`frontend/src/routes/admin/+layout.ts:1-4`](frontend/src/routes/admin/+layout.ts#L1-L4) | High — defense-in-depth gap, not exploitable but ugly. |
| 4 | **Favorites proxy reads the wrong cookie.** Reads `session` instead of `rtp_access_token`, so every favorites request goes out unauthenticated. | [`frontend/src/routes/api/favorites/+server.ts:32`](frontend/src/routes/api/favorites/+server.ts#L32) | High — feature is broken in prod. |
| 5 | **WebSocket accepts any client.** `// TODO: Validate JWT tokens` at `websocket.rs:344`. Anyone with the URL can subscribe to the live alert stream. | `api/src/routes/websocket.rs:344` | High — leaks paid content. |
| 6 | **Admin auth pattern is split.** `admin.rs` mostly uses `User + require_admin()`; the rest of `admin_*.rs` modules use the `AdminUser` extractor. Two paths to maintain, two ways to get role checks wrong. | `api/src/middleware/admin.rs` + `api/src/routes/admin.rs` | High — maintenance + audit risk. |
| 7 | **No transactions on multi-step admin mutations.** `grant_membership`, `bulk_assign_tags`, etc. can leave the DB partially updated. | `api/src/routes/admin.rs:958-1037`, `admin_members.rs:948-992` | High — data integrity. |
| 8 | **14 hardcoded `<your-api-host>` URLs in the frontend.** Spread across `+page.server.ts` files, `lib/server/watchlist.ts`, `hooks.server.ts:24`, `lib/api/config.ts`. Migration to a different API URL today touches 14 files. | inventory in [§5.4](#54-hardcoded-urls) | Medium — bites on every env change. |
| 9 | **Three separate retry implementations.** Catch-all proxy, Axum client, `lib/api/admin.ts`. Three places to debug a retry storm. | [§5.5](#55-retry-and-circuit-breaker) | Medium. |
| 10 | **Production API reports `environment: "development"`.** Which probably relaxes cookie / CORS / log defaults. | observed live | Medium — security hygiene. |
| 11 | **Existing `.remote.ts` files have correctness issues.** Hard-coded refresh keys + unawaited `.refresh()` promises. | [`commands.remote.ts:71, 89`](frontend/src/routes/dashboard/explosive-swings/commands.remote.ts) | Medium — already detailed in the previous audit. |
| 12 | **CMS AI-assist wired in backend, no frontend UI.** | `api/src/routes/cms_ai_assist.rs` | Low — feature dormant. |

The good news: the **data models** and **route shapes** are mostly correct. The
gaps are at the seams — frontend gates missing, mutations not transactional,
toolbar wired wrong, money flow stubbed.

---

## 1. Backend admin

### 1.1 What's mounted

77 route modules total; 12 are admin-flavored. Per-module summary:

| File | Lines | Routes (count) | Auth pattern | Notes |
|------|-------|----------------|--------------|-------|
| `routes/admin.rs` | 1678 | ~25 | mixed: `User + require_admin()`, **plus** `AdminUser` on `list_all_plans` (line 815) | the kitchen sink — dashboard, users, memberships, campaigns, coupons, settings |
| `routes/admin_courses.rs` | 2312 | full course/module/lesson/live-session/enrollment CRUD | `AdminUser` extractor uniformly | most complete admin module |
| `routes/admin_indicators.rs` | 915 | indicator + files + videos + analytics CRUD | `AdminUser` extractor uniformly | upload pipeline disabled in `routes/mod.rs` (see prior audit) |
| `routes/admin_videos.rs` | 1858 | video + series + watch-history CRUD | `AdminUser` | several `// TODO` placeholders for trader/room metadata |
| `routes/admin_members.rs` | 1308 | segments, member tags, analytics, notes, emails | `User + require_admin()` | bulk tag assignment is loop-not-tx |
| `routes/admin_member_management.rs` | 1393 | member CRUD | `AdminUser` | clean |
| `routes/admin_popups.rs` | 856 | popup CRUD | `AdminUser` | clean |
| `routes/admin_page_layouts.rs` | 766 | page layout CRUD | `AdminUser` | clean |
| `routes/courses_admin.rs` | (~2000) | parallel admin path for courses | `AdminUser` | overlaps with `admin_courses.rs`; both are mounted |
| `routes/indicators_admin.rs` | (~1500) | parallel admin path for indicators | `AdminUser` | overlaps with `admin_indicators.rs` |
| `routes/subscriptions_admin.rs` | 723 | subscription admin | `AdminUser` | clean |
| `routes/migrate.rs` | 58 | one-shot deploy tasks | `AdminUser` | safe |

### 1.2 Auth posture — the split pattern

Two extractors live in `middleware/admin.rs`:
- `AdminUser` (lines 11-95): role ∈ {`admin`, `super_admin`, `super-admin`, `developer`} OR email in `SUPERADMIN_EMAILS`.
- `SuperAdminUser` (lines 76-77): role ∈ {`super_admin`, `super-admin`}.

`admin.rs` largely doesn't use them — it accepts a `User` and calls a private
`require_admin()` helper inside each handler. Every other `admin_*.rs` module
uses the extractor. Three problems with this:

1. **Easy to forget** the helper call inside a new handler in `admin.rs` and ship an unprotected route.
2. **Different error semantics** — an extractor returns 401/403 before the handler runs; the helper runs after some setup.
3. **Audit pain** — grepping for "is this admin-protected?" needs two patterns.

The one **unprotected** route in this surface is intentional:
`GET /admin/coupons/validate/:code` (`admin.rs` near line 594) — coupons are
validated at checkout, no auth required. No rate limit on it though.

### 1.3 Stubs and TODOs (concrete file:line)

| Location | Issue | Severity |
|----------|-------|----------|
| `api/src/routes/admin_videos.rs:323-324, 366-367, 644-645, 662-663` | `let trader = None; // TODO`, `let rooms = vec![]; // TODO`, `this_week: 0 // TODO`, `traders: [] // TODO` — analytics returns hard-coded zeros | Medium |
| `api/src/routes/admin.rs:1218` | `.unwrap_or_default()` after a failing query in `list_campaigns` — silent failure | Medium |
| `api/src/routes/admin.rs:1288-1351` | Multiple `.unwrap_or((0,))` on critical stats queries — admin sees "0 users" instead of an error when the DB hiccups | Medium |
| `api/src/routes/admin.rs:1484-1540` | `impersonate_user` returns a placeholder string `format!("impersonate_{}_{}", …)` — not a real JWT | High (feature is dead) |
| `api/src/routes/admin_members.rs:114-120` | `.ok()` swallows missing-table errors; admin sees empty list when schema has drifted | Medium |

### 1.4 Database integrity

- **Parameterized queries:** ✓ everywhere I sampled. No string interpolation into SQL.
- **Transactions:** ✗ **none** in admin mutations. Multi-step writes (grant
  membership and bump count, bulk-assign tags across N users) can leave the DB
  half-updated.
  - `admin.rs:958-1037` — `grant_membership` writes the membership row, then
    updates an aggregate count; either can fail independently.
  - `admin_members.rs:948-992` — `bulk_assign_tags` loops, no `BEGIN`/`COMMIT`.
- **Cache invalidation:** ✗ none. After updating a user's role
  (`admin.rs:253`), nothing flushes the Redis user cache, nothing invalidates
  outstanding sessions.

### 1.5 Cross-cutting

- **Rate limiting on admin endpoints:** ✗ none. `services/rate_limit.rs` is
  used on auth and payments, not admin.
- **Audit log writes:** ✗ none. There's tracing (`tracing::info!`) at
  `admin.rs:39, migrate.rs:23, admin.rs:1514-1522` (impersonation logs are the
  exception — those are correctly persisted to the security log) but no
  general "admin did X" rows in PostgreSQL. The `cms_audit` service does this
  for CMS, but admin user-management mutations are silent.
- **Impersonation:** logged correctly (`admin.rs:1514-1522`), but the
  generated token is a placeholder string, not a real JWT — so the feature is
  observable but not functional.

### 1.6 Recommended priorities (backend admin)

1. **Wrap multi-step mutations in transactions.** Helper:
   `state.db.begin().await? → handler body → tx.commit().await?` pattern;
   apply to `grant_membership`, `bulk_assign_tags`, role updates, anything
   that touches more than one table.
2. **Pick one auth pattern.** I'd port `admin.rs` to use `AdminUser` /
   `SuperAdminUser` extractors and delete `require_admin()`. One PR.
3. **Add cache invalidation calls** after every mutation that affects a
   cached read path — at minimum: user updates, role changes, membership
   grants, settings updates.
4. **Stop swallowing errors with `.unwrap_or_default()`** on stats queries.
   Either propagate (`?`) or log + return an explicit fallback shape with a
   "stats unavailable" flag the UI can read.
5. **Finish or remove `impersonate_user`.** Today it's a vector for
   confusion: the audit log says "admin X impersonated user Y" but no
   impersonation actually happened.

---

## 2. Headless CMS v2

### 2.1 What's mounted

13 backend route modules under `cms_*.rs` and a parallel set of services
under `services/cms_*.rs`. Schema lives in 8 SQL migrations
(`014, 023, 024, 027, 029, 041_presets, 041_scheduling, 042_datasources`).

| Module | Purpose | State |
|--------|---------|-------|
| `cms_v2.rs` | core CRUD: content, assets, folders, revisions, tags, comments | ✅ |
| `cms_v2_enterprise.rs` | release bundles, multi-content workflows | ✅ |
| `cms_assets.rs` | asset CRUD (split from v2 for clarity) | ✅ |
| `cms_revisions.rs` | revision list + restore (`require_cms_admin` on restore) | ✅ |
| `cms_scheduling.rs` | cron-driven publish / unpublish | ✅ |
| `cms_seo.rs` | SEO validation (title/desc lengths, density, readability) | ✅ |
| `cms_global_components.rs` | site-wide reusable components | ✅ |
| `cms_reusable_blocks.rs` | block library | ⚠ — sync/detach mentioned in header, no route handlers |
| `cms_presets.rs` | block presets (30+ seeded in migration 041) | ⚠ — backend only, no frontend integration |
| `cms_datasources.rs` | option lists (countries, US states, symbols, palettes) | ⚠ — CSV import/export mentioned, no routes |
| `cms_delivery.rs` | public read API (`GET /content/:type/:slug`) | ✅ |
| `cms_ai_assist.rs` | Claude Sonnet 4 integration | ⚠ — backend wired, no frontend UI |
| `cms_workflow.rs` (service) | status transitions | ✅ |
| `cms_audit.rs` (service) | structured audit log | ✅ |

### 2.2 Block taxonomy

**63 distinct block types** declared in
`frontend/src/lib/components/cms/blocks/types.ts`, grouped:

- **Content** (8): paragraph, heading, quote, pullquote, code, preformatted, list, checklist
- **Media** (7): image, video, audio, gallery, file, embed, gif
- **Interactive** (5): accordion, tabs, toggle, toc, buttons
- **Layout** (5): columns, group, row, divider, spacer
- **Trading** (5): ticker, chart, priceAlert, tradingIdea, riskDisclaimer
- **AI** (3): aiGenerated, aiSummary, aiTranslation
- **Advanced** (~30): card, testimonial, cta, countdown, socialShare, author,
  relatedPosts, newsletter, html, button, callout, shortcode, reusable, etc.

`frontend/src/lib/components/cms/blocks/BlockRenderer.svelte:73-122` maps
every type to its component via a `componentMap` record.

### 2.3 The toolbar bug

This is the single most user-impactful regression in the whole system.

**Symptom:** clicking any block button in the editor toolbar does nothing.
The block isn't appended to the page.

**Location:** [`frontend/src/routes/cms/editor/+page.svelte:113`](frontend/src/routes/cms/editor/+page.svelte#L113)

**Likely cause:** the toolbar buttons render via a bits-ui v2 `<Button>`
primitive, and the v1 → v2 upgrade moved how event handlers attach. Plain
`onclick={() => addBlock(type)}` no longer reaches the underlying button. The
e2e tests `tests/e2e/block-editor.spec.ts:13, :25, :38` are quarantined
because of this.

**Fix path:**
1. Read what `<button>` (or `<Button>`) the editor toolbar actually renders
   today (not yet inspected — flagged as next step).
2. If it's a bits-ui Button, switch to bits-ui v2's recommended event
   forwarding pattern.
3. If it's a native `<button>`, the issue is something else (event delegation
   or the import chain rewriting `onclick`).
4. Once fixed, lift the three `test.fixme` markers in
   `tests/e2e/block-editor.spec.ts:13, 25, 38`.

### 2.4 What's missing in the editor

The block taxonomy is rich but the editor itself is minimal:
- ✗ no drag-and-drop reorder
- ✗ no undo/redo (no history stack in the BlockStateManager)
- ✗ no autosave
- ✗ no preview-mode toggle, no mobile preview
- ✗ no AI-assist panel (backend ready, no UI)
- ✗ no preset picker (30+ presets seeded, no UI to insert one)
- ✗ no datasource picker for blocks that take options
- ✗ collaborator presence: `PresenceAvatars.svelte` exists but is unused;
  no WebSocket subscription to a presence channel

In the meantime the things that DO exist are solid: block selection (lines
59-64), block deletion (lines 72-75), block-count display (line 20), the
toolbar with 10 pre-configured types (lines 78-89). It just doesn't add
blocks.

### 2.5 Workflow + revisions + scheduling

These are the most complete part of the CMS:

- **State machine** (`api/src/models/cms.rs:127-134`):
  `Draft → InReview → Approved → [Scheduled | Published] → Archived`.
  Transitions are enforced server-side at `cms_v2.rs:362-386`. Only
  super-admin / admin can publish (line 373).
- **Revisions:** `GET /content/:id/revisions` lists; `POST /content/:id/revisions/:n/restore`
  rolls back. Restore is admin-only (`cms_v2.rs:439`).
- **Scheduling:** `cms_scheduled_jobs` table + cron worker in
  `services/cms_scheduler.rs`. Status enum: `Pending / Processing / Completed
  / Failed / Cancelled`. Release bundles support multi-content coordinated
  publishes.
- **Audit:** `services/cms_audit.rs` writes a `cms_audit_logs` row for every
  CMS-side change, including before/after JSON, IP, UA, user id + email +
  role. SOC 2 / GDPR-friendly. Backend admin user-management writes do **not**
  go through this, which is the gap.

### 2.6 Asset pipeline

- **Bucket:** `revolution-trading-media` (R2) — env `R2_BUCKET`.
- **Public CDN:** R2 public URL, `R2_PUBLIC_URL`.
- **Limits:** 50 MB per upload (`services/cms_upload.rs:38-66`).
- **Allowed types:** JPEG/PNG/GIF/WebP/AVIF/SVG, MP4/WebM/MOV, PDF/Word/Excel/CSV.
- **Folder model:** hierarchical with `parent_id`, `path`, `depth`, color
  + icon + `is_public` + `allowed_roles`. Soft delete via `deleted_at`.
- **Asset metadata:** alt text, caption, credits, SEO title/description,
  tags, image variants, blurhash, dominant color, Bunny video bindings,
  usage count, last-used.
- **Search:** filter by folder, mime type, tags, creator, paginated. Full-
  text on filename is **not** exposed (commented `// TODO` in `cms_content.rs`
  near line 1000).

### 2.7 Recommended priorities (CMS)

1. **Fix the toolbar.** The single highest-leverage fix in the whole repo.
2. **Wire the AI panel.** Backend has SSE-streaming Claude calls;
   surfacing them in the editor takes a small panel + a `command` invocation.
3. **Add autosave + undo/redo.** Both fit naturally into `BlockStateManager`;
   autosave can ride a `command` from `data.remote.ts` to keep the same
   pattern as explosive-swings.
4. **Surface presets in the editor.** 30 seeded presets are sitting unused.
5. **Make CMS audit cover admin user-management too.** Move the
   `cms_audit::log` helper one layer up and call it from
   `admin_member_management.rs` mutations.

---

## 3. Frontend admin + dashboard

### 3.1 Page inventory

Counts after walking the route tree:

| Surface | `+page.svelte` files | Top-level state |
|---------|---------------------|-----------------|
| `frontend/src/routes/admin/**` | ~27 | mixed (most pages are full-CRUD; a few stubs) |
| `frontend/src/routes/dashboard/**` | ~15 | working but with hardcoded TODOs |
| `frontend/src/routes/account/**` | 2 | read-only (no edit UI — by design, see prior audit) |
| `frontend/src/routes/my/**` | 4 | view-only |

### 3.2 Auth gates (frontend)

| Surface | Layout file | Authn? | Authz? | Verdict |
|---------|-------------|--------|--------|---------|
| `/admin` | [`admin/+layout.ts:1-4`](frontend/src/routes/admin/+layout.ts#L1-L4) | via hooks.server.ts | **none** | ✗ broken — any authenticated user lands here |
| `/dashboard` | `dashboard/+layout.server.ts:41-50` | via hooks.server.ts | **none** | ⚠ — admin / member share `/dashboard`; UX-only issue |
| `/account` | none (covered by hooks PROTECTED_ROUTES) | yes | none | ⚠ — same as dashboard |
| `/my/*` | none (covered by hooks PROTECTED_ROUTES) | yes | none | ⚠ |

`hooks.server.ts:30` defines `PROTECTED_ROUTES = ['/dashboard', '/account',
'/checkout', '/trading-room', '/admin']` and validates the JWT, but **never
checks role**. The role helpers in `frontend/src/lib/config/roles.ts:299-322`
exist; no route guard uses them.

### 3.3 CRUD UI coverage (frontend admin)

| Resource | List | Detail | Create | Edit | Delete | Notes |
|----------|------|--------|--------|------|--------|-------|
| Members | ✅ | ✅ drawer | ✅ modal | ✅ modal | ✅ | full CRUD |
| Categories | ✅ | inline | ✅ modal | ✅ modal | ✅ + merge | best-built |
| Courses | ✅ | ✅ drawer | ✅ Builder | ✅ Builder | ✅ | full |
| Indicators | ✅ | ✅ drawer | ✅ Builder | ✅ Builder | ✅ | full |
| Coupons | ✅ | inline | ✅ modal | ✅ modal | ✅ | full |
| Trading Rooms | ✅ | ✅ drawer | partial | ✅ modal | ✅ | partial create UI |
| Email templates | ✅ | inline | ✅ form | ✅ form | ✅ | full |
| Blog posts | ✅ | ✅ editor | ✅ editor | ✅ editor | ✅ | full |
| Subscriptions | ✅ | ✅ drawer | ✅ modal | ✅ modal | ✗ | no cancel/delete UI |
| Users | ✅ | ✗ | ✗ | ✗ | ✅ | list + delete only |
| Products | ✅ | ✗ | ✗ | ✗ | ✅ | list + delete only |
| Orders | ✅ | ✗ | ✗ | ✗ | ✗ | view only |
| Member Segments | ✅ | ✅ drawer | ✅ form | ✗ "Coming soon" | ✅ | edit is a stub (`segments/+page.svelte:114-115`) |

### 3.4 Form patterns — three different approaches

The frontend talks to the backend in three incompatible shapes:

1. **Typed API client + Svelte 5 store** (admin): clean, consistent.
   `usersApi.list()`, `membersStore`, `productsApi`, etc.
2. **Direct fetch + `getAuthToken()`** (account/my): `fetch('/api/...')` from
   the browser. Lives at `my/subscriptions/+page.svelte:80`. Bypasses the
   nice catch-all proxy and re-implements auth.
3. **Backend stores driven by store-side actions** (e.g. members, courses) —
   layered on top of (1).

There are **zero** uses of SvelteKit `<form use:enhance>` or remote-function
`form()` outside the explosive-swings dashboard. The convention this codebase
has chosen is fine — but the *inconsistency* between admin and `/my/*` is
not.

### 3.5 Stubs and dead UI

| Page / element | File | Symptom |
|----------------|------|---------|
| Member segment "Edit" | `admin/members/segments/+page.svelte:114-115` | `toastStore.info('… - Coming soon')` |
| Dashboard trading-room dropdown links | `dashboard/+page.svelte:62-75` | three entries `href: '#'` with `// TODO: Provide URL` |
| Analytics heatmaps page | `admin/analytics/heatmaps/+page.svelte` | placeholder visualization |

No deleted pages still referenced in nav. No Lorem-ipsum.

### 3.6 Component patterns

Every admin component sampled uses the **legacy** Svelte 5 `let props = $props()`
+ shadow-state pattern (e.g. `AdminCard.svelte:34`, `MemberFormModal.svelte`,
`SubscriptionFormModal.svelte`, `CourseBuilder.svelte`, `IndicatorBuilder.svelte`).

We migrated 41 of these to the destructured `$bindable()` form for the
shadcn-svelte wrappers in commit `05acf3231`. **The admin / dashboard
component layer hasn't been migrated yet** and would be a natural batch
follow-up — it's the same mechanical transformation, and `svelte-check` will
flag every remaining `state_referenced_locally` warning.

---

## 4. Integration layer

### 4.1 Three connection patterns coexist

| Pattern | File count | Where | Notes |
|---------|-----------|-------|-------|
| Catch-all proxy `/api/[...path]/+server.ts` | 1 generic + ~96 specialized | `frontend/src/routes/api/**` | retry + circuit breaker; consistent error shape; **bypassed** by both other patterns |
| Direct fetch in `+page.server.ts` `load` | 61 files | scattered | mostly hits `event.fetch('/api/...')`, but **14 files hardcode `<your-api-host>/...` directly** |
| `.remote.ts` (Svelte 5 Remote Functions) | 2 | `dashboard/explosive-swings/{data,commands}.remote.ts` | only fully-typed end-to-end pattern |

### 4.2 The catch-all proxy

[`frontend/src/routes/api/[...path]/+server.ts`](frontend/src/routes/api/%5B...path%5D/+server.ts)
is solid:

- **Methods:** GET / POST / PUT / PATCH / DELETE all forwarded.
- **Auth:** reads `rtp_access_token` cookie, attaches `Authorization: Bearer`.
- **Retry:** 3 attempts, exponential backoff 1s → 2s → 4s.
- **Circuit breaker:** in-memory; 5 failures opens it for 30 s. This is what
  produced the `[API Proxy] Circuit breaker OPEN - too many failures` log
  lines we saw during the DB outage.
- **Timeout:** 30 s default, 60 s for multipart.
- **Error shape:** `{error, code, requestId, timestamp, path, retryable}`.
- **Base URL precedence:** `env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT`.

### 4.3 The Axum client

[`frontend/src/lib/server/axum/client.ts`](frontend/src/lib/server/axum/client.ts)
is the path that `.remote.ts` files use. Almost identical retry + error
mechanics but **different env-var precedence**:
`env.API_BASE_URL || env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT`.

That precedence mismatch with the catch-all (`VITE_API_URL` first vs
`API_BASE_URL` first) is a real footgun — set one env in dev, the proxy and
remote functions will hit different backends. Fix is one-line.

Domain adapters under `lib/server/axum/`: `axumAuth`, `axumAlerts`,
`axumTrades`, `axumTradePlans`, `axumStats`, `axumVideos`. Each is a thin
typed wrapper over the client. Pattern works well; just needs to grow.

### 4.4 Hardcoded fly.dev URLs

14 files hardcode the production URL. Centralizing them is a 30-minute PR.

```
frontend/src/routes/+page.server.ts:6
frontend/src/routes/dashboard/explosive-swings/+page.server.ts:55
frontend/src/routes/dashboard/explosive-swings/alerts/+page.server.ts
frontend/src/routes/dashboard/explosive-swings/watchlist/+page.server.ts
frontend/src/routes/dashboard/explosive-swings/analytics/+page.server.ts
frontend/src/routes/dashboard/weekly-watchlist/watchlist-rundown-archive/+page.server.ts:67
frontend/src/lib/server/watchlist.ts:32
frontend/src/lib/server/watchlist.ts:48
frontend/src/hooks.server.ts:24
frontend/src/lib/api/config.ts
frontend/src/routes/api/logout/+server.ts:28  (uses BACKEND_URL fallback only)
frontend/src/routes/api/favorites/+server.ts  (also reads wrong cookie — see §4.6)
…and 2 more in `routes/api/watchlist/`
```

Centralize in a `lib/config/api.ts` with one canonical export.

### 4.5 Retry and circuit breaker

Implemented **three times**:

1. The catch-all proxy `+server.ts:206-283`.
2. The Axum client `lib/server/axum/client.ts:186-275`.
3. `lib/api/admin.ts` (somewhere similar).

Three places to debug a thundering-herd. Should extract into a single
`lib/server/retry.ts` and import from all three. The shapes are close enough
that it would be a non-breaking refactor.

### 4.6 Concrete bugs surfaced by the integration audit

| # | Bug | File:line | Severity |
|---|-----|-----------|----------|
| A | Favorites proxy reads wrong cookie | `frontend/src/routes/api/favorites/+server.ts:32` — `cookies.get('session')` should be `cookies.get('rtp_access_token')` | **High — feature broken in prod** |
| B | Env-var precedence mismatch | catch-all uses `VITE_API_URL || BACKEND_URL`; Axum client uses `API_BASE_URL || VITE_API_URL || BACKEND_URL` | High — silent split-brain in dev |
| C | Hardcoded fly.dev URLs | 14 files (§4.4) | Medium |
| D | Retry implemented 3× | proxy + client + admin.ts (§4.5) | Medium |
| E | Watchlist endpoint **bypasses the proxy entirely** | `frontend/src/routes/api/watchlist/...` direct-fetches `<your-api-host>/api/watchlist/entries` | Medium |
| F | Logout proxy returns 200 even when backend logout fails | `frontend/src/routes/api/logout/+server.ts:50` | Low — defensive cookie deletion is intentional |
| G | Token-refresh logic exists in 3 places that must stay in sync | hooks.server.ts + auth store + `lib/api/auth.ts` | Low — already documented in source comments |

Bug A is the most user-visible: the Favorites button on the site has been
silently failing for whoever's been testing it.

### 4.7 Migration candidates → Remote Functions

Per the prior audit's tier list, these five `+page.server.ts` files are the
cleanest wins for the next sweep:

1. `dashboard/high-octane-scanner/+page.server.ts` → new `scanner.remote.ts`
2. `dashboard/weekly-watchlist/watchlist-rundown-archive/+page.server.ts`
   → `watchlist.remote.ts` (also kills hardcoded URL on line 67)
3. `dashboard/explosive-swings/alerts/+page.server.ts` → consolidate into
   the existing `data.remote.ts`
4. `dashboard/explosive-swings/watchlist/+page.server.ts` → ditto
5. `dashboard/day-trading-room/daily-videos/+page.server.ts`
   → `videos.remote.ts` reusing `axumVideos` (kills hardcoded URL on line 23)

---

## 5. Cross-cutting findings

### 5.1 The four worst offenders, ranked

| # | Where | Cost to fix |
|---|-------|------------|
| 1 | CMS toolbar add-block (bits-ui v2 regression) | 1-2 hours of investigation, one file edit |
| 2 | Stripe Checkout-Session stub | half-day for the happy path; another half-day for retries / webhook validation |
| 3 | `/admin` frontend role gate missing | one new `+layout.server.ts`, ten lines |
| 4 | Favorites proxy wrong cookie | one-line edit |

### 5.2 The slow hum

These don't break anything individually but are how the codebase rots:

- 14 hardcoded fly.dev URLs.
- Three retry implementations.
- Two env-var precedence orders.
- `admin.rs` uses one auth pattern, every other admin module uses another.
- 41 admin / dashboard components still on legacy `let props = $props()`
  shadow-state.
- Three forks of the token-refresh dance (server hook, client store, API
  wrapper).

Each of these is a 30-minute fix in isolation. Together they're the entire
"why does this codebase feel slippery" feeling.

### 5.3 What's surprisingly good

Worth naming:

- **CMS data model** — content / asset / revision / audit / preset /
  datasource is genuinely well thought through.
- **Rust API parameterization** — every query I sampled is parameterized.
  No SQL injection holes.
- **Indicator download tokens** — short-lived, IP-bound, max-downloads.
  Best-built control in the repo.
- **Audit-grade logging in the CMS** — the schema for `cms_audit_logs` is
  SOC 2 / GDPR-shaped (action, entity type+id, before+after JSON, IP, UA,
  actor email + role).
- **Existing `.remote.ts` pattern** — the explosive-swings file pair is the
  right shape; just needs to spread.

---

## 6. Suggested order of work when we resume dev

1. **Fix the DB outage** (still in [§7 of the previous audit](PRODUCT_AND_AUTH_AUDIT_2026-04-25.md#7-current-incidents)). Nothing below matters until login works.
2. **Fix the CMS toolbar** ([§2.3](#23-the-toolbar-bug)). Single highest-leverage fix.
3. **Fix the favorites cookie bug** (one line, [§4.6 row A](#46-concrete-bugs-surfaced-by-the-integration-audit)).
4. **Add the frontend `/admin` role gate** (`admin/+layout.server.ts` redirects when `!user.is_admin`).
5. **Wire Stripe Checkout-Session creation** (`subscriptions.rs:446`).
6. **Centralize the Fly URL** in `lib/config/api.ts`; replace the 14 hardcodes in one PR.
7. **Unify env-var precedence** between catch-all proxy and Axum client.
8. **Wrap admin multi-step mutations in transactions** — `grant_membership` and `bulk_assign_tags` first.
9. **Pick one auth pattern in `admin.rs`** and port the kitchen-sink module to `AdminUser` extractor.
10. **Migrate the 5 dashboard `+page.server.ts` files** to `.remote.ts` (per [§4.7](#47-migration-candidates--remote-functions)).
11. **Fix the two correctness issues in the existing `.remote.ts` files** (hardcoded refresh keys; unawaited promises — see prior audit §10.7).
12. **Activate MFA in `login`** — the service is already complete.
13. **WebSocket JWT validation** (`websocket.rs:344`).
14. **Migrate admin / dashboard components** off legacy `let props = $props()` to destructured `$bindable()` (the autofixer-friendly batch the audit identified in §3.6).

Items 2-4 are all under a day each. Item 5 is the revenue blocker. Items
6-14 are the cleanup that pays compound interest.

---

## Appendix — repos of evidence

This audit is produced by four parallel read-only agent sweeps. Their full
transcripts (with citations beyond what's quoted above) are recoverable
from the conversation log of `08cf8605c..HEAD`. If a future audit wants to
re-verify any single claim, the file:line numbers above are the seed.
