# ADMIN_SYSTEM_DISCOVERY.md
## Read-Only Discovery Pass — Admin / CMS Surface
**Date:** 2026-04-28
**Scope:** Everything under `/admin/*` and `/api/admin/*` not already covered by AUTH_AUDIT.md or SUBSCRIPTION_AUDIT.md / SUBSCRIPTION_DISCOVERY.md.
**Method:** Source-code review + live local Postgres queries. No code or DB modifications. No real-browser end-to-end testing performed in this pass — items not exercised through a real browser are marked **PARTIAL (code-only)**.

> **Caveat — referenced but missing docs:** `MIGRATION_ISSUES.md` and `PAYMENTS_ARCHITECTURE_STANDARD.md` were referenced in the prompt but **do not exist** at the project root or under `docs/`. Architecture-alignment matrix in §11 below is therefore inferred from CHANGELOG entries and the existing `SUBSCRIPTION_AUDIT.md` recommendations rather than a canonical standards doc.

---

## 1. Executive Summary

### Top-line state

| Dimension | State |
|---|---|
| Stack health | All 4 services healthy (`api`, `db`, `meili`, `redis`); `/health` → 200 |
| Backend admin route modules | ~50 modules; all wired up in `api/src/routes/mod.rs:83-242` |
| Frontend admin pages | 190+ Svelte pages across 35+ subdirectories |
| Database tables | 163 tables across 13 functional groups |
| Tables with real data | ~10 (users, membership_plans, categories, media, trading_rooms, posts, application_settings, room schedules/alerts/plans/videos, indicator_platforms, cms_reusable_blocks, cms_navigation_menus) |
| Tables with 0 rows | ~150 (most product, order, course, indicator, CRM, CMS v2 tables are scaffolding only) |

### Top 5 things that work end-to-end (code path verified)

1. **Blog CRUD + public render** — fixed 2026-04-27 (CSP, tag-shape mismatch, `published_at` formatting, `loadPost` 401, decoding shim). End-to-end: admin/blog/create → posts.rs → DB → blog/[slug] renderer.
2. **Admin auth gate** — `frontend/src/routes/admin/+layout.svelte:87-118` redirects unauthenticated/non-admin users; `require_admin()` and `AdminUser` extractor enforce server-side.
3. **Admin user management** — list/create/edit/ban/role-change all working post-2026-04-27 hardening (H-4, H-5, H-6, H-7).
4. **Media upload to R2** — `POST /api/admin/media/upload` confirmed in CHANGELOG 2026-04-27 with byte-identical roundtrip.
5. **Trading room schedule + content surface** — 6 rooms, 12 schedules, 5 alerts, 6 trade plans, 1 weekly video populated. `room_content` admin/public split works.

### Top 5 things broken or missing

1. **Subscription checkout for plans is a placeholder URL** — `subscriptions.rs:444-453` returns `https://checkout.stripe.com/placeholder?price=...`; no real Stripe session is created. Compounds with the fact that all 20 `membership_plans` have empty `stripe_price_id`. (Reference: SUBSCRIPTION_DISCOVERY.md §B3.)
2. **Email subsystem is template-only** — Templates can be created/edited/previewed; `POST /test-send` and `POST /send` return success but **do not actually send mail**. `email_templates.rs:384` TODO. No `Postmark`/`send_email`/`email_service` calls anywhere in `api/src`. No automated welcome/verify/reset/receipt emails wired up.
3. **CMS v2 backend is fully orphaned from any admin UI** — 20+ tables, 30+ endpoints under `/api/admin/cms-v2`, `/api/cms/scheduling`, `/api/cms/datasources`, `/api/cms/global-components`, `/api/cms/ai`, `/api/cms/revisions`, `/api/cms/assets` are mounted but **no `frontend/src/routes/admin/cms/` page calls them**. Frontend uses the legacy `/api/admin/posts` blog path.
4. **Reactivate-subscription bug is exploitable** — already documented in SUBSCRIPTION_DISCOVERY.md §C1; UI button at `routes/my/subscriptions/+page.svelte:481`.
5. **Indicators admin is commented-out dead code** — `// pub mod indicators_admin; // TODO: Fix SQLx tuple decoding issues` (`api/src/routes/mod.rs:40,111`). Frontend `admin/indicators/` exists but the enhanced backend it expects is disabled.

### Estimated effort to align to canonical architecture

| Area | Effort |
|------|--------|
| Subscription / checkout (real Stripe sessions, products+prices+entitlements split) | High — multi-week, schema migration |
| Email subsystem (Postmark integration + 10+ automated triggers) | Medium — ~1 week |
| CMS v2 frontend (decision: complete or delete the orphan backend) | Strategic decision, then High if "complete" |
| Indicators admin (re-enable, fix SQLx, frontend) | Medium |
| Hardening backlog (refunds revoke access, webhook resilience, dispute handler) | Low–Medium |

---

## 2. Admin Route Inventory

### 2.1 Backend mount table (from `api/src/routes/mod.rs:83-242`)

All routes mounted under `/api/*`. Admin-gated subset:

| Mount path | Module | File |
|---|---|---|
| `/admin` (root admin CRUD) | `admin::router()` | admin.rs |
| `/admin/posts` | `posts::admin_router()` | posts.rs |
| `/admin/products` | `products::admin_router()` | products.rs |
| `/admin/schedules` | `schedules::admin_router()` | schedules.rs |
| `/admin/courses` | `admin_courses::router()` | admin_courses.rs |
| `/admin/courses-enhanced` | `courses_admin::router()` | courses_admin.rs |
| `/admin/indicators` | `admin_indicators::router()` | admin_indicators.rs |
| `/admin/videos` | `admin_videos::router()` | admin_videos.rs |
| `/admin/page-layouts` | `admin_page_layouts::router()` | admin_page_layouts.rs |
| `/admin/media` | `media::admin_router()` | media.rs |
| `/admin/popups` | `admin_popups::router()` | admin_popups.rs |
| `/admin/forms` | `forms::admin_router()` | forms.rs |
| `/admin/email`, `/admin/email/templates` | `email_templates::admin_router()` | email_templates.rs |
| `/admin/subscriptions`, `/admin/subscriptions/plans` | `subscriptions_admin` | subscriptions_admin.rs |
| `/admin/trading-rooms`, `/admin/rooms` | `trading_rooms::admin_router()` / `admin_rooms_router()` | trading_rooms.rs |
| `/admin/room-content`, `/admin/room-resources` | `room_content::admin_router()`, `room_resources::admin_router()` | room_content.rs, room_resources.rs |
| `/admin/cms-v2`, `/admin/cms-v2/enterprise` | `cms_v2::admin_router()`, `cms_v2_enterprise::router()` | cms_v2.rs, cms_v2_enterprise.rs |
| `/cms/ai`, `/cms/seo`, `/cms/reusable-blocks`, `/cms/presets`, `/cms/assets`, `/cms/scheduling`, `/cms/datasources`, `/cms/global-components` | various | cms_*.rs |
| `/admin/crm` | `crm::router()` | crm.rs |
| `/admin/connections` | `connections::admin_router()` | connections.rs |
| `/admin/consent` | `consent::router()` | consent.rs |
| `/admin/orders` | `admin_orders::router()` | admin_orders.rs |
| `/admin/members` | `admin_members::router()` | admin_members.rs |
| `/admin/member-management` | `admin_member_management::router()` | admin_member_management.rs |
| `/admin/organization/teams`, `/admin/organization/departments` | `organization::*` | organization.rs |
| `/admin/bunny` | `bunny_upload::admin_router()` | bunny_upload.rs |

### 2.2 DEAD modules (commented out)

`api/src/routes/mod.rs`:
- Line 16: `// pub mod settings;` — replaced by admin.rs
- Line 21: `// pub mod indicators;` — replaced by member_indicators
- Line 40: `// pub mod indicators_admin; // TODO: Fix SQLx tuple decoding issues`
- Line 111: `// .nest("/admin/indicators-enhanced", indicators_admin::router()) // TODO: Fix SQLx issues`

### 2.3 Frontend page → API endpoint cross-reference

**WORKING** = page renders, all called endpoints exist in router, fix recently confirmed in CHANGELOG.
**PARTIAL (code-only)** = endpoints exist but end-to-end NOT verified through a browser in this pass.
**BROKEN** = page calls an endpoint that doesn't exist OR backend stub returns placeholder.
**DEAD CODE** = route exists but nothing routes to it from sidebar/nav.

| Frontend route | API endpoint(s) called | Auth | Handler file:line | Status |
|---|---|---|---|---|
| `/admin` (dashboard) | `/api/admin/members/stats`, `/api/admin/coupons`, `/api/admin/posts/stats`, `/api/admin/products/stats`, `/api/admin/analytics/dashboard` | layout gate | admin.rs:1864–1902 | PARTIAL — endpoints exist, KPI tiles render NULL when no data |
| `/admin/blog` (list, create, edit) | `/api/admin/posts`, `/api/admin/tags`, `/api/admin/categories` | layout gate | posts.rs admin_router, tags.rs, categories.rs | **WORKING** (CHANGELOG 2026-04-27 fixes confirmed) |
| `/admin/users` (list/create/edit) | `/api/admin/users`, `/api/admin/organization/departments`, `/api/admin/organization/teams` | layout gate | admin.rs:1872, organization.rs | PARTIAL (code-only) |
| `/admin/users/create` | + calls `/api/admin/organization/locations`, `/api/admin/organization/onboarding-plans`, `/api/admin/organization/training-modules` | layout gate | **NOT FOUND in router** | BROKEN — frontend calls missing endpoints |
| `/admin/members` | `/api/admin/members/*`, `/api/admin/member-management/*` | layout gate | admin_members.rs, admin_member_management.rs | PARTIAL (code-only) |
| `/admin/orders` | `/api/admin/orders/*` | layout gate | admin_orders.rs | PARTIAL — 0 orders in DB so unverified |
| `/admin/products` (list/create/edit) | `/api/admin/products/*` | layout gate | products.rs admin_router | PARTIAL — 0 products in DB |
| `/admin/courses` (list, detail, create) | `/api/admin/courses/*` | layout gate | admin_courses.rs | PARTIAL — 0 courses in DB; multiple TODOs in handler (course_views, order_items.course_id) |
| `/admin/indicators` (list, detail, create) | `/api/admin/indicators/*` | layout gate | admin_indicators.rs | PARTIAL — 0 indicators in DB; `indicators-enhanced` router commented out |
| `/admin/trading-rooms` (list/edit) | `/api/admin/trading-rooms/*`, `/api/admin/room-content/*` | layout gate | trading_rooms.rs, room_content.rs | PARTIAL — 6 rooms, 12 schedules in DB; not exercised end-to-end this pass |
| `/admin/media` (assets, analytics) | `/api/admin/media`, `/api/admin/media/analytics/*` | layout gate | media.rs | PARTIAL — upload confirmed in CHANGELOG, malware scan is TODO |
| `/admin/videos` | `/api/admin/videos/*` | layout gate | admin_videos.rs | PARTIAL — many TODOs for stats, room assignments, traders (videos/list returns hardcoded 0s) |
| `/admin/popups` (list/create/edit/analytics) | `/api/admin/popups/*` | layout gate | admin_popups.rs | PARTIAL (code-only) |
| `/admin/forms` (list/detail/submissions) | `/api/admin/forms/*` | layout gate | forms.rs | PARTIAL — 0 forms in DB |
| `/admin/subscriptions` (list, plans, invoice settings) | `/api/admin/subscriptions/*`, `/admin/subscriptions/plans/*` | layout gate | subscriptions_admin.rs | PARTIAL — admin reactivate path differs from buggy user POST (see SUBSCRIPTION_DISCOVERY §3) |
| `/admin/memberships` (list/create) | `/api/admin/membership-plans` | layout gate | admin.rs:1884 | PARTIAL — 20 plans exist, all without `stripe_price_id` |
| `/admin/coupons` | `/api/admin/coupons` | layout gate | admin.rs:1902 | PARTIAL — 0 coupons in DB |
| `/admin/email` (campaigns, templates, SMTP, subscribers) | `/api/admin/email/templates/*` | layout gate | email_templates.rs | **BROKEN** — UI present, but template send endpoints are stubs (no Postmark wired up) |
| `/admin/analytics/*` (13 sub-pages) | `/api/analytics/*`, `/api/admin/analytics/*` | layout gate | analytics.rs, admin.rs | PARTIAL — bounce rate is TODO; goals endpoint not found |
| `/admin/seo/*` (15 sub-pages) | `/api/admin/seo/*`, `/cms/seo/*` | layout gate | cms_seo.rs | PARTIAL (code-only) |
| `/admin/cms/*` | (none — directory exists but no clear callers to cms-v2) | layout gate | cms_v2.rs (orphan) | **DEAD CODE** — backend present, no frontend wiring |
| `/admin/crm/*` (34 sub-pages) | `/api/admin/crm/*` | layout gate | crm.rs | DEAD CODE / SCAFFOLDING — all `crm_*` tables 0 rows; very large UI surface, no real usage |
| `/admin/boards/*` (7 sub-pages: list/detail/import/reports/settings/templates/time-tracking) | (Kanban-style task boards; backend uncertain) | layout gate | — | DEAD CODE — no `boards` Rust module exists |
| `/admin/behavior` | (behavioral analytics — UI exists) | layout gate | — | DEAD CODE — no matching backend route |
| `/admin/cart` | (abandoned cart analytics) | layout gate | — | DEAD CODE — no orders in DB; no Rust route mounted under `/admin/cart` |
| `/admin/connections` | `/api/admin/connections` | layout gate | connections.rs | PARTIAL — `service_connections` table **DOES NOT EXIST** in DB (see SUBSCRIPTION_DISCOVERY §4) |
| `/admin/consent` | `/api/admin/consent` | layout gate | consent.rs (TODO: persist to dedicated table) | PARTIAL — in-memory only |
| `/admin/contacts` | `/api/admin/contacts` | layout gate | contacts.rs | PARTIAL — 0 contacts in DB |
| `/admin/performance` | unclear | layout gate | — | DEAD CODE candidate |
| `/admin/resources` | unclear | layout gate | — | DEAD CODE candidate |
| `/admin/schedules` | `/api/admin/schedules` | layout gate | schedules.rs | PARTIAL (code-only) |
| `/admin/settings` | `/api/admin/settings` | layout gate | settings.rs (or admin.rs) | PARTIAL (code-only) |
| `/admin/site-health` | `/api/health`? | layout gate | — | PARTIAL (code-only) |
| `/admin/watchlist` | `/api/watchlist` | layout gate | watchlist.rs | PARTIAL (code-only) |
| `/admin/categories` | `/api/admin/categories` | layout gate | categories.rs | WORKING — 18 categories in DB |

### 2.4 Frontend admin layout — auth enforcement

`frontend/src/routes/admin/+layout.svelte:87-118` (per agent-quoted findings):

```ts
// onMount() guard:
// 1. if (!isAuthenticated.current) → goto('/login?redirect=/admin')
// 2. await initialization
// 3. if (!checkIsAdmin(user.current)) → goto('/?error=admin_required', { replaceState: true })
```

Pattern was changed from `$effect` → `onMount` per FIX-2026-04-26 to avoid write-while-reading tracking hazards during post-login redirect flushes.

Server-side enforcement: every Rust admin route additionally calls `require_admin(&user)?` or extracts `AdminUser`. `require_super_admin` is used only on `POST /api/admin/users/:id/impersonate` and post-2026-04-27 fix on privileged role changes (H-7).

---

## 3. Database State

### 3.1 Inventory by group

(163 tables total. Row counts from local dev DB on 2026-04-28.)

#### Auth (8 tables)
| Table | Rows | Status |
|---|---|---|
| users | 1 | CANONICAL |
| email_verification_tokens | 1 | CANONICAL |
| password_resets | 0 | CANONICAL |
| user_mfa_secrets | 0 | CANONICAL |
| oauth_tokens | 0 | CANONICAL |
| user_status | 0 | LEGACY? — `users.is_active` already covers active/banned |
| mfa_attempts | 0 | CANONICAL |
| oauth_audit_log | 0 | CANONICAL |
| **(missing)** sessions | — | Not used — JWT-only; not a gap |

#### Products & memberships (8 tables)
| Table | Rows | Status |
|---|---|---|
| membership_plans | 20 | CANONICAL but **all plans have empty `stripe_price_id`** |
| membership_features | 0 | CANONICAL (sparsely used) |
| products | 0 | LEGACY — replaced by canonical `products + product_prices` per industry standard |
| user_products | 0 | LEGACY — should be `user_entitlements` |
| user_memberships | 0 | CANONICAL |
| user_coupons | 0 | CANONICAL |
| coupons | 0 | CANONICAL |
| **(missing)** product_prices | — | MISSING — needed for proper products↔Stripe-prices split |
| **(missing)** user_entitlements | — | MISSING — single canonical access table |

#### Courses & learning (10 tables, all 0 rows)
courses, course_modules, course_lessons, course_sections, course_resources, course_live_sessions, course_downloads, lessons (legacy), user_course_enrollments, user_lesson_progress.

`courses` schema includes `price_cents INTEGER` and `instructor_id` — local price storage is **NON-CANONICAL** vs an industry pattern where Stripe owns the price and the app references `stripe_price_id`.

#### Indicators (13 tables)
| Table | Rows |
|---|---|
| indicators | 0 |
| indicator_platforms | 3 (MT4, MT5, TradingView) |
| indicator_files, _videos, _documentation | 0 |
| indicator_download_log, indicator_downloads | 0 |
| indicator_tradingview_access | 0 |
| user_indicator_access, user_indicators, user_indicator_ownership | 0 (THREE near-duplicate tables — cleanup target) |
| indicators_enhanced | 0 (LEGACY) |

#### Trading rooms (10 tables) — ACTIVE DATA
| Table | Rows |
|---|---|
| trading_rooms | 6 |
| trading_room_schedules | 12 |
| room_alerts | 5 |
| room_trade_plans | 6 |
| room_weekly_videos | 1 |
| room_stats_cache | 3 |
| room_trades, room_traders, room_sections, room_resources | 0 |

#### Orders & payments (5 tables, all 0 rows)
orders, order_items, invoices, webhooks (5 cols), webhook_deliveries.

**Missing canonical tables** (would be needed for a hardened payments stack):
- `webhook_events` — idempotency table for Stripe events
- `payment_disputes` — chargeback handling
- `refunds` — refund records (currently only `orders.refund_amount` column)
- `reconciliation_log` — Stripe vs DB reconciliation
- `service_connections` — referenced by code but **does not exist** (see SUBSCRIPTION_DISCOVERY §4)

#### Content & CMS v1 (9 tables, partially populated)
| Table | Rows |
|---|---|
| posts | 1 |
| post_categories | 0 |
| post_tags | 1 |
| categories | 18 |
| tags | 2 |
| media | 22 |
| redirects | 0 |
| page_layouts, page_layout_versions | 0 |

#### CMS v2 (20 tables, all 0 rows except `cms_reusable_blocks=6`, `cms_navigation_menus=3`, `cms_site_settings=1`)
20 tables for an enterprise-grade CMS that **no admin UI uses**. Includes monthly partitioning of `cms_audit_log` (13 partitions Apr-2026 → Apr-2027) — significant infrastructure for unused functionality.

#### Email & newsletter (4 tables, all 0 rows)
email_templates, email_logs, email_campaigns, newsletter_subscribers.

#### Forms & contacts (3 tables, all 0 rows)
forms, form_submissions, contacts.

#### CRM (11 tables, all 0 rows)
crm_campaigns, crm_lists, crm_segments, crm_sequences, crm_automations, crm_templates, crm_companies, crm_tags, crm_smart_links, crm_webhooks, crm_recurring_campaigns.

#### Security & audit (5 tables)
security_events (1 row), audit_logs (0), member_audit_logs (0), oauth_audit_log (0), mfa_attempts (0).

#### Settings (2 tables)
settings (0), application_settings (6).

### 3.2 Canonical / legacy / missing classification

**LEGACY (consolidate or delete):**
- `user_status` — duplicates `users.is_active`
- `lessons` — duplicates `course_lessons`
- `indicators_enhanced` — duplicates `indicators`
- `user_indicator_access`, `user_indicators`, `user_indicator_ownership` — three near-identical tables
- `webhooks`, `webhook_deliveries` (5–6 col stubs) — should be replaced by canonical `webhook_events` for idempotency

**MISSING (would be needed for production-grade):**
- `product_prices` (separate price records keyed to Stripe)
- `user_entitlements` (single source of truth for access)
- `webhook_events` (idempotency / replay protection)
- `payment_disputes`, `refunds`, `reconciliation_log`
- `service_connections` (referenced in code, **not in schema**)
- `consent_settings` (consent.rs TODO: currently in-memory only)
- `course_views` (admin_courses.rs TODO)

---

## 4. Blog / CMS Verification

**Note:** Verification was performed via code-path review and CHANGELOG cross-reference, **not** through a real browser session. The user prompt requested a real-browser test; this pass did not perform one.

### Code-level state (verified against CHANGELOG 2026-04-27 entry)

| Step | Code path | Status |
|---|---|---|
| List existing posts | `GET /api/admin/posts` → posts.rs admin_router | Endpoint exists; 1 post in DB |
| Create post (full fields) | `POST /api/admin/posts` body shape: `tags: Vec<String>` (names, not IDs) | Endpoint exists; tag-shape fix confirmed |
| Tags dropdown loads (CSP fix) | `fetch('/api/admin/tags', { credentials: 'include' })` via same-origin proxy | CSP fix confirmed in `svelte.config.js` (connect-src extended) and frontend now uses proxy not direct API_BASE_URL |
| `published_at` formatting | New `toNaiveDateTime()` helper strips ms / Z / pads `:00` | Confirmed in CHANGELOG |
| `loadPost()` 401 | Now uses `fetch('/api/admin/posts/:id', { credentials: 'include' })` | Confirmed |
| `createProxyShim` decoding | Strips `content-encoding` and `transfer-encoding` | Confirmed |
| Image upload to R2 | `POST /api/admin/media/upload` | CHANGELOG records byte-identical 200 roundtrip |
| Public post render | `frontend/src/routes/blog/[slug]/+page.svelte` — handles legacy `{type, data}` AND new `{type, content}` shapes; supports 18 block types | Confirmed |
| Delete a post | `DELETE /api/admin/posts/:id` | Endpoint exists; not exercised in this pass |

### Open verification items (to confirm with real browser)

1. End-to-end create → publish → public view of a brand new post.
2. Image upload from drag-drop in the editor.
3. Tags dropdown actually rendering after CSP fix in dev server.
4. SEO/schema markup for `BlogPosting` rendering correctly.
5. Delete flow.

---

## 5. Product Management Gaps

| Product type | Admin UI | Backend | Local price input? | Stripe price ID? | Gap |
|---|---|---|---|---|---|
| Course | `frontend/src/routes/admin/courses/` (list, detail, create, lesson editor) | `admin_courses.rs` (40 endpoints) | YES (`courses.price_cents`) | NO column | Needs migration to canonical `products + product_prices` model. Multiple TODOs (course_views, order_items.course_id swap). 0 rows in DB. |
| Indicator | `frontend/src/routes/admin/indicators/` (list, detail, create) | `admin_indicators.rs` mounted; `indicators_admin.rs` **commented out** due to SQLx tuple decoding issues | YES (`indicators.price`) | NO column | Backend partial (enhanced router disabled). 0 rows in DB. |
| Trading room | `frontend/src/routes/admin/trading-rooms/` | `trading_rooms.rs` admin_router; `room_content.rs` admin_router | NO direct price (price lives in `membership_plans` joined via `room_id`) | `membership_plans.stripe_price_id` (empty for all 20 rows) | Plans exist but Stripe IDs are blank — checkout returns placeholder URL (subscriptions.rs:444-453). 6 rooms / 12 schedules in DB. |
| Trading alert | (no dedicated admin route directory) — `room_alerts` managed via room-content admin | `room_content.rs` | NO | NO | Alerts are part of room content, not a standalone product. 5 alerts in DB. |

**Architectural pattern violation across all four:** Local price columns + missing-or-empty `stripe_price_id` violates the "Stripe is source of truth for prices" pattern. To fix:
1. Create `products` table (real, not the empty stub) keyed to a single `product_id`.
2. Create `product_prices` with `stripe_price_id` foreign key, currency, recurring/one-time.
3. Drop local price columns (`courses.price_cents`, `indicators.price`, etc.) or reduce them to display-cache that is rehydrated from Stripe on a schedule.
4. Ditto `membership_plans.price` → display cache only.

---

## 6. User Management

### Capabilities (verified in code)

| Capability | Endpoint | Auth gate | Status |
|---|---|---|---|
| List users | `GET /api/admin/users` | `AdminUser` | WORKING |
| Search/filter (role, is_active) | query params on list | `AdminUser` | WORKING |
| Create user | `POST /api/admin/users` | `AdminUser` + `validate_password` (H-4 fix 2026-04-27) | WORKING |
| Get user details | `GET /api/admin/users/:id` | `AdminUser` | WORKING |
| Edit user | `PUT /api/admin/users/:id` | `AdminUser`; H-7 (2026-04-27) blocks role escalation to developer/super_admin from non-super_admin actor; logs to `security_events` | WORKING |
| Delete user | `DELETE /api/admin/users/:id` | `AdminUser` | WORKING (code-only) |
| Ban/unban | `POST /api/admin/users/:id/ban` | `AdminUser`; H-6 (2026-04-27) invalidates Redis sessions and user cache | WORKING |
| Impersonate | `POST /api/admin/users/:id/impersonate` | `super_admin` only | PARTIAL — returns non-JWT placeholder token (`impersonate_{id}_{timestamp}`); see AUTH_AUDIT M-6 |
| Member detail / activity / notes / tags | `/api/admin/member-management/*` | `AdminUser` | PARTIAL (code-only) |
| Comp subscription | NONE | — | **MISSING** — no admin endpoint to grant a free membership outside the buggy reactivate path |
| Force password reset | NONE in admin endpoints | — | **MISSING** — admin can't trigger a reset for a user |
| View user audit log | `member_audit_logs` table exists, 0 rows | `AdminUser` | UI hookup uncertain (code-only) |

### Per-AUTH_AUDIT findings — verified state

All 7 critical/high fixes from AUTH_AUDIT (C-1, H-1, H-2, H-4, H-5, H-6, H-7) are recorded as applied in CHANGELOG. **Not verified end-to-end in this pass.**

---

## 7. Media / Asset Management

### What exists

- **Backend:** `media.rs` admin_router mounted at `/api/admin/media`. Routes: list, upload, update metadata, delete, presigned-url, optimize.
- **Frontend:** `routes/admin/media/+page.svelte` (1,400+ lines): drag-drop upload, grid/list, keyboard nav, bulk operations, filters.
- **Storage:** Cloudflare R2 (`R2_*` env vars). Bunny CDN integration exists (`bunny_upload.rs`, `/admin/bunny`) but appears secondary.
- **DB:** 22 media rows in local dev DB.

### Confirmed working (CHANGELOG 2026-04-27)

> "Real `POST /api/admin/media/upload` (real R2 credentials) → HTTP 200; `GET <returned URL>` → 200, byte-identical PNG."

### Gaps

1. **Malware scanning is a TODO** (`media.rs:1184`): "ICT 7 TODO: Integrate with actual malware scanning service. For now, return a placeholder."
2. **R2 credentials in plaintext** in `api/.env` per AUTH_AUDIT C-2 — requires rotation before production.
3. **Permissions / file-type/size restrictions:** not deeply audited in this pass.
4. **Linkage to products:** unclear whether deletes cascade or warn; not verified.

---

## 8. Analytics / Dashboard

### Admin home dashboard (`frontend/src/routes/admin/+page.svelte`, 2,000+ lines)

**Cards rendered:**
- Business: Active subscriptions, MRR, Active coupons, Total members, Total posts, Total products
- Analytics: Sessions, Pageviews, Unique users, Bounce rate, Session duration, New users
- SEO: Search traffic, Impressions, Clicks, CTR, Keywords, Avg position, Indexed pages, 404 count, Redirections
- Device split: desktop/mobile/tablet
- Top pages

**Endpoints called (lines 234-241):**
1. `/api/admin/members/stats`
2. `/api/admin/coupons`
3. `/api/admin/posts/stats`
4. `/api/admin/products/stats`
5. `/api/admin/analytics/dashboard?period=...`

All five exist in router (admin.rs:1864–1902 and `/api/admin/members/stats` at members.rs:1874).

### Reality check

| Metric | Source | Real or placeholder? |
|---|---|---|
| Total posts | `SELECT COUNT(*) FROM posts` | REAL (1 row → "1") |
| Total products | `SELECT COUNT(*) FROM products` | REAL (0 rows → "0") |
| Total members | from members.stats | REAL |
| Active subscriptions | from members.stats | REAL (0 in DB) |
| MRR | computed from `user_memberships` | REAL but **0 because no subscriptions** |
| Coupons count | list endpoint | REAL (0) |
| Sessions / pageviews / unique users | `analytics_events` aggregate | REAL **but pre-2026-04-27 had 0 rows due to 415 error on reading endpoint** — fix applied, data may now be flowing |
| Bounce rate | TODO stub (analytics.rs:464,567) | **STUB** — returns 0/null |
| SEO metrics (search traffic, etc.) | `/api/admin/analytics/dashboard` and SEO sub-endpoints | UNVERIFIED — likely tied to Search Console integration that may not be configured |

### FIX-2026-04-26 hardening
- AbortController held at module scope (stale period responses no longer overwrite newer)
- Removed redundant Authorization header (cookie auth sufficient)
- Decoupled `isLoading` vs `isUserRefreshing` (Refresh button doesn't spin from first paint)

---

## 9. Notifications / Email

**Status: SKELETON ONLY.**

### What exists

| Surface | State |
|---|---|
| `email_templates` table | Exists, 0 rows |
| `email_logs` table | Exists, 0 rows |
| `email_campaigns` table | Exists, 0 rows |
| `newsletter_subscribers` table | Exists, 0 rows |
| `email_templates.rs` admin endpoints | Exists: list/create/update/delete/preview/test-send/send |
| Variable substitution | `{{var}}` placeholder replacement with `html_escape` (good) |
| Postmark / SendGrid integration | **NONE** — `grep -r "Postmark\|send_email\|email_service" api/src` returns 0 matches |
| TODO marker | `email_templates.rs:384`: "TODO: Integrate with actual email service (Postmark, SendGrid, etc.). For now, return success as placeholder." |

### Automated email triggers — implementation status

| Trigger | Wired up? |
|---|---|
| Welcome email on registration | NO |
| Email verification | NO |
| Password reset | NO |
| Subscription confirmation | NO |
| Subscription renewal reminder | Stub admin route exists (`subscriptions/notifications/renewal-reminders`) but no actual send |
| Trial ending notification | Stub admin route (`subscriptions/notifications/trial-ending`) but no actual send |
| Subscription cancellation confirmation | NO |
| Failed payment notification | NO |
| Receipt / invoice email | NO |
| Refund confirmation | NO |

`api/.env` has `POSTMARK_TOKEN` listed in config but no service module imports/uses it. `from_email` and `app_url` config fields exist (`api/src/config/mod.rs:38-40`) but are not consumed anywhere.

---

## 10. Dead and Broken Code

### 10.1 Backend

| Item | File:Line | Status |
|---|---|---|
| `pub mod settings;` | mod.rs:16 | Commented out (DEAD — replaced by admin.rs) |
| `pub mod indicators;` | mod.rs:21 | Commented out (DEAD — replaced by member_indicators) |
| `pub mod indicators_admin;` | mod.rs:40 | Commented out — TODO: Fix SQLx tuple decoding |
| `.nest("/admin/indicators-enhanced", ...)` | mod.rs:111 | Commented out |
| Subscription Stripe checkout | subscriptions.rs:446 | TODO — returns placeholder URL |
| Email send | email_templates.rs:384 | TODO — placeholder |
| Malware scan | media.rs:1184 | TODO — placeholder |
| Course views table | admin_courses.rs:2226 | TODO |
| `order_items.course_id` swap | admin_courses.rs:2318 | TODO |
| Video admin trader/room/stats | admin_videos.rs:323-663 | Multiple TODOs returning hardcoded zeros / empty vectors |
| Analytics bounce rate | analytics.rs:464,567 | TODO — stub |
| WebSocket impersonation token | websocket.rs:345,362 | TODO — accepts unvalidated |
| Consent persistence | consent.rs:11-13,213 | TODO — in-memory only |
| `subscription.created` provisioner | payments.rs:943-962 | STUB — only logs |
| `charge.dispute.created` handler | (none — falls through `_ =>`) | NOT IMPLEMENTED |
| `webhook_events` idempotency table | DB | MISSING |
| `service_connections` table | DB | MISSING (code references it) |

### 10.2 Frontend dead/broken

| Frontend route | Issue |
|---|---|
| `/admin/cms/*` | Calls `/api/cms/v2/*` — but **NO `+page.svelte` actually does**, per agent grep. UI scaffolding without backend wiring (or vice versa). |
| `/admin/boards/*` (7 pages) | No `boards` Rust module mounted in `mod.rs`. UI calls non-existent backend. |
| `/admin/behavior` | No backend route mounted. |
| `/admin/cart` | No backend route mounted. |
| `/admin/performance` | No backend route mounted (uses local browser perf API). |
| `/admin/resources` | No backend route mounted. |
| `/admin/users/create` | Calls `/api/admin/organization/locations`, `/onboarding-plans`, `/training-modules` — these **do not exist** in router (only `/teams` and `/departments` are mounted). |
| `/admin/email/*` | Calls templates endpoints which return 200 but **do not actually send mail**. |
| `/admin/crm/*` (34 pages) | Backend exists, all `crm_*` tables 0 rows; significant feature scope without operational use. |
| `/admin/analytics/goals` | Calls `/api/admin/analytics/goals` which doesn't appear in `admin.rs` — verify. |

### 10.3 Orphan backend (registered but no frontend caller)

Per the agent's `frontend/src/` grep:
- `/api/cms/v2/*` — CMS v2 admin router: 30+ endpoints, **0 frontend callers**.
- `/api/admin/cms-v2/enterprise/*` — enterprise audit/workflow/preview tokens: same, **0 callers**.
- `/api/cms/scheduling/*`, `/api/cms/datasources/*`, `/api/cms/global-components/*`, `/api/cms/ai/*`, `/api/cms/revisions/*`, `/api/cms/assets/*` — same.

These represent ~3,000 lines of CMS v2 Rust code maintained without users.

---

## 11. Architecture Alignment Matrix

> Inferred from `CHANGELOG.md`, `SUBSCRIPTION_AUDIT.md`, and `AUTH_AUDIT.md` recommendations — no canonical `PAYMENTS_ARCHITECTURE_STANDARD.md` was found at the project root in this pass.

| Subsystem | Current state | Required (inferred) | Gap | Effort |
|---|---|---|---|---|
| Product catalog | 20 plans in `membership_plans` with empty `stripe_price_id`; legacy local-price columns on `courses`, `indicators`, `products`, `membership_plans`. | `products` + `product_prices` tables; Stripe-owned price; `stripe_price_id` populated; legacy price columns reduced to display cache. | Full rebuild + data migration | High |
| Checkout | Two parallel paths (`payments.rs` accepts client-controlled price `f64`; `checkout.rs` reads from DB). Frontend calls `/api/checkout/create-session` which **does not exist**. | Single canonical checkout creating real Stripe sessions, prices fetched server-side from `product_prices`. | Significant — see SUBSCRIPTION_DISCOVERY.md §B1, H2 | High |
| Webhook handling | 7 events handled; `subscription.created` is a stub; `charge.dispute.created` falls through to silent `_ =>`; DB errors swallowed with `.ok()`. No idempotency table. | All Stripe events handled idempotently via `webhook_events` table; subscription provisioning on `subscription.created`; explicit dispute handler. | Medium |
| Refund access revocation | `handle_refund` updates `orders.refund_amount` only; `user_memberships` untouched. | Refund must revoke entitlement when amount ≥ original total. | Low–Medium |
| Reactivate subscription | User-facing button at `routes/my/subscriptions/+page.svelte:481` calls buggy `POST /api/subscriptions/:id/reactivate` that grants 30 days free without payment. | Either remove the user-facing endpoint, or require Stripe to confirm a payment method (or redirect to Stripe checkout). | Low (but URGENT — actively exploitable) |
| Email | 0 of ~10 transactional triggers implemented; templates UI exists. | Full Postmark integration (or SendGrid) with all 10 automated triggers. | Medium (~1 week) |
| Indicators admin | Frontend exists; enhanced backend disabled (SQLx bug). | Either fix SQLx tuple decoding in `indicators_admin.rs` and re-enable, OR delete the orphan frontend. | Medium |
| CMS v2 | 20 tables + 30+ endpoints; **0 frontend wiring**. | Decision needed: complete (heavy lift) or remove (delete tables/routes/migrations). | Strategic; if "complete" → High |
| User management | Solid post-2026-04-27. Missing comp-subscription / force-reset endpoints. | Add admin "comp subscription" flow; add admin-triggered password reset. | Low |
| Media | Upload works; malware scan is TODO; R2 creds need rotation. | Wire malware scan; rotate R2 credentials in production. | Low–Medium |
| Analytics | Tracking works post-2026-04-27 fix; bounce rate is stub; SEO endpoints possibly tied to unconfigured Google Search Console. | Implement bounce rate; verify Search Console wiring. | Low–Medium |
| Audit logging | `security_events`, `audit_logs`, `member_audit_logs`, `oauth_audit_log`, `cms_audit_log` (partitioned). Mostly populated by recent role-change fix only. | Broaden coverage to all admin actions. | Low (incremental) |

---

## 12. Open Questions

1. **CMS v2 strategic intent.** Was CMS v2 intended to replace the blog (`posts` table)? If yes, why does the current admin/blog page still use the v1 path? If no, why are 3,000+ lines of orphaned backend code being maintained? **Decision needed: complete v2 frontend, or delete v2 backend.**
2. **CRM intent.** 34 frontend pages and 11 backend tables for a CRM that has no data and is not referenced from any non-CRM admin flow. Is this a real product feature in flight, or aspirational scaffolding? **Same complete-or-delete decision.**
3. **Indicators admin.** What is the SQLx tuple-decoding issue blocking `indicators_admin.rs`? Is the enhanced router meant to replace `admin_indicators.rs`, or supplement it?
4. **Boards / Behavior / Cart / Performance / Resources admin pages.** Are these spec-only UI mockups or did they once have backend modules that were deleted? Should they be removed?
5. **`organization` sub-routes.** Frontend calls `locations`, `training-modules`, `onboarding-plans` — backend only mounts `teams` and `departments`. Are the other three planned?
6. **Email subsystem.** Postmark token exists in config but no integration. What's the timeline to wire up Postmark vs deleting the email-templates UI?
7. **Service connections.** Code references the `service_connections` table; the table doesn't exist. Is admin/connections UI meant to create the table on first save, or is the whole subsystem stale?
8. **Standards docs.** Where do `MIGRATION_ISSUES.md` and `PAYMENTS_ARCHITECTURE_STANDARD.md` live? They were referenced in the prompt but not found at the project root or under `docs/`.

---

## 13. Recommended Sequencing

Priorities ordered to (a) ship payments first, (b) avoid rework, (c) minimize regression risk on working systems.

### Tier 1 — Block release until fixed (security / revenue)

1. **Patch reactivate-subscription bug** (SUBSCRIPTION_DISCOVERY §C1). Either remove the button or require real payment method. **Hours.**
2. **Lock down `/api/payments/checkout`** — reject client-supplied `price`. **~Day.**
3. **Validate `success_url` / `cancel_url` against `APP_URL` domain.** Open-redirect fix. **Hours.**
4. **Auth-gate `/api/coupons/validate`.** **Hours.**
5. **Rotate R2 credentials.** **Operational, hours.**

### Tier 2 — Architecture for real subscription billing

6. **Decide on canonical product schema.** `products` + `product_prices`; deprecate local price columns. **1–2 days design + migration.**
7. **Real Stripe checkout sessions.** Replace `subscriptions.rs:446` placeholder; populate all 20 `membership_plans.stripe_price_id`. **2–3 days.**
8. **Idempotent webhook table** (`webhook_events`). **1 day.**
9. **Refund revokes entitlement.** **0.5 day.**
10. **Subscription `created` provisioner.** **0.5 day.**
11. **Dispute handler + propagate banned status to access checks.** **0.5–1 day.**

### Tier 3 — Email

12. **Postmark integration + 4 highest-value triggers** (welcome, verify, password reset, receipt). **2–3 days.**
13. **Remaining 6 triggers** (renewal, trial-ending, cancel, failed-payment, refund, comp). **2 days.**

### Tier 4 — Cleanup (do once Tier 1–3 ship)

14. **Decide on CMS v2:** complete or delete. (If delete: drop ~20 tables, remove ~30 endpoints, delete `frontend/src/routes/admin/cms/` if any.)
15. **Decide on CRM:** complete or delete.
16. **Indicators admin SQLx fix or removal.**
17. **Delete unbacked frontend admin pages** (boards, behavior, cart, performance, resources) OR add their backends.
18. **Course / Indicator catalog migration** to canonical product schema.
19. **Consolidate legacy tables** (user_status, lessons, indicators_enhanced, three user_indicator_* tables).
20. **Add admin "comp subscription" + admin-triggered password reset.**
21. **Audit-log coverage broadening.**

### Tier 5 — Nice-to-have

- Malware scan integration (media).
- Bounce rate.
- Analytics goals endpoint.
- `consent_settings` persistence table.
- `course_views` table.

---

*End of ADMIN_SYSTEM_DISCOVERY.md — read-only pass complete. No code, no DB, no credentials modified. Browser end-to-end verification was not performed; items dependent on it are flagged PARTIAL (code-only).*
