# Demolition & Rebuild Plan — Revolution Trading Pros

**Version:** 1.0
**Date:** 2026-04-28
**Owner:** Billy Ribeiro
**Strategy:** Option A — Delete most scaffolding. Build canonical replacements against PAYMENTS_ARCHITECTURE_STANDARD.md.

---

## 0. Pre-Flight (do once, before Phase 1)

### 0.1 Read these documents and confirm understanding

- `PAYMENTS_ARCHITECTURE_STANDARD.md` — the canonical payments spec
- `AUTH_AUDIT.md` — security findings, mostly fixed
- `SUBSCRIPTION_AUDIT.md` and `SUBSCRIPTION_DISCOVERY.md` — current payment system bugs
- `ADMIN_SYSTEM_DISCOVERY.md` — full admin/CMS surface inventory
- `MIGRATION_ISSUES.md` — known migration system corruption
- This document

### 0.2 Hard branch protection

Before any deletion happens, ensure:

```bash
git status
# working tree should be clean or have only intentional changes

git checkout -b demolition-rebuild-2026-04
git push -u origin demolition-rebuild-2026-04
```

All Phase 1-12 work happens on this branch. Main stays untouched until everything is verified and ready to merge.

### 0.3 Database backup

Before Phase 1 runs:

```bash
mkdir -p backups/demolition-$(date +%Y%m%d-%H%M%S)
docker exec rtp-db pg_dump -U rtp -d revolution_trading_pros -F c -f /tmp/rtp-pre-demolition.dump
docker cp rtp-db:/tmp/rtp-pre-demolition.dump ./backups/demolition-$(date +%Y%m%d-%H%M%S)/
```

Verify the dump is non-empty and restorable. If anything in Phase 1-12 destroys data, this is the recovery point.

### 0.4 What we are KEEPING (do not touch in any phase)

These subsystems are working and stay:

- **Auth** — `api/src/middleware/auth.rs`, `api/src/routes/auth.rs`, `api/src/models/user.rs`. All H-fixes from AUTH_AUDIT applied. Do not modify.
- **Blog** — `api/src/routes/posts.rs` admin_router, `frontend/src/routes/admin/blog/`, `frontend/src/routes/blog/`. CSP fix and tag-shape fix applied. Working end-to-end.
- **Media upload to R2** — `api/src/routes/media.rs` upload endpoint, R2 wiring. Working.
- **Trading rooms data layer** — `trading_rooms`, `trading_room_schedules`, `room_alerts`, `room_trade_plans`, `room_weekly_videos`, `room_stats_cache`, `room_content`, `room_resources`. Real data, working code.
- **Categories and tags** — `categories`, `tags`, `post_categories`, `post_tags`. Working.
- **Application settings** — `application_settings` table. Working.
- **Indicator platforms** — `indicator_platforms` table (3 rows: MT4, MT5, TradingView). Reference data, keep.
- **Admin user management** — list/create/edit/ban/role-change in `api/src/routes/admin.rs`. All AUTH_AUDIT fixes applied. Do not modify.

Anything not on this list is a candidate for deletion or rebuild.

### 0.5 What we are DELETING

- CMS v2 (entire subsystem — 20+ tables, 30+ endpoints)
- CRM (11 tables, 34 frontend pages, all empty)
- Email templates UI (no Postmark wiring; will be replaced by real implementation)
- Old subscription / checkout / payments code (incoherent, two parallel flows, exploits)
- Old products / courses / indicators tables (will be replaced by canonical schema)
- Frontend admin pages with no backend: boards, behavior, cart, performance, resources
- Duplicate tables: user_status, lessons, indicators_enhanced, user_indicator_access, user_indicators, user_indicator_ownership
- Orphan endpoints with no frontend caller (full list in Phase 1)
- All `service_connections` references (table doesn't exist anyway)

### 0.6 What we are BUILDING FRESH

- Canonical product catalog (`products` + `product_prices` per spec)
- Canonical entitlements (`user_entitlements` per spec)
- Canonical webhook event tracking (`webhook_events` per spec)
- One canonical checkout endpoint (`POST /api/checkout`)
- Real Stripe webhook handler with idempotency, fail-closed signature verification
- Stripe Customer Portal integration
- Daily reconciliation job
- Real Postmark email integration with all 10 transactional triggers
- Admin UI for product management (Stripe-IDs only, no local prices)
- Admin price-change feature with three options (per spec section 8)

### 0.7 Migration system caveat

`MIGRATION_ISSUES.md` documents that the migration system is partially corrupted (duplicate version numbers at 030, 037, 040, 041; migrations 031-035 applied via psql but not registered in `_sqlx_migrations`).

**This plan does NOT attempt to fix the migration system.** New migrations created during demolition/rebuild use the next available unused number (currently 046+) and proceed forward. The corruption stays where it is. After the rebuild is complete and verified, a separate session addresses the migration system reconciliation.

If a phase requires a new table, the migration runs and is registered cleanly because we're past the corruption point. Verify after each migration: `_sqlx_migrations` table should show the new entry.

---

## 1. Phase Index

| Phase | Title | Risk | Reversible? | Estimated time |
|---|---|---|---|---|
| 1 | Pre-flight verification & dead-code inventory | Low | N/A (read-only) | 1-2 hours |
| 2 | Delete frontend admin pages with no backend | Low | Yes (git revert) | 1-2 hours |
| 3 | Delete CMS v2 subsystem | Medium | Yes (git revert) | 2-3 hours |
| 4 | Delete CRM subsystem | Low | Yes (git revert) | 1-2 hours |
| 5 | Delete duplicate / legacy tables | Medium | Yes (DB dump + migrations) | 2-3 hours |
| 6 | Delete old subscription / checkout / payments code | High | Yes (git revert; no users yet) | 3-4 hours |
| 7 | Build canonical product schema | Medium | Yes (new migrations only) | 3-4 hours |
| 8 | Build canonical checkout endpoint + webhook handler | High | Yes (new code only) | 1 day |
| 9 | Build product admin UI | Medium | Yes | 1 day |
| 10 | Build price-change feature | Medium | Yes | 0.5 day |
| 11 | Wire up Postmark email | Medium | Yes | 1 day |
| 12 | End-to-end verification + reconciliation job | Low | N/A | 0.5 day |

Total: ~3 weeks of focused work, broken into 12 sessions.

---

## 2. Phase 1: Pre-Flight Verification & Dead-Code Inventory

**Goal:** Confirm the working baseline before any deletion. Produce a precise inventory of every file and table targeted for deletion in subsequent phases.

**Risk:** Low (read-only)

### Prompt

```
You are a senior engineer doing the final read-only verification pass before a major demolition + rebuild project on Revolution Trading Pros (SvelteKit + Rust/Axum).

DO NOT modify any code. DO NOT modify the database. Output is a single document: PHASE1_PREFLIGHT.md at the project root.

=== TASK 1: VERIFY BASELINE WORKS ===

Confirm these subsystems are currently functional. For each, run a real test (real browser, real HTTP, real DB query — not synthetic curl on fixtures):

1. Auth: Log in as admin via the real frontend login form. Capture HTTP responses.
2. Blog admin: Navigate to /admin/blog/create, create a test post WITH AT LEAST TWO TAGS SELECTED, publish it, view the public URL. Verify HTTP 200 throughout. Delete the test post.
3. Media: Upload a test image via /admin/media. Verify R2 URL works.
4. Trading rooms: List rooms in admin. Confirm 6 rooms exist.
5. Admin user management: List users. Create a test user. Ban the test user. Verify they cannot log in. Unban. Delete.

If ANY of these fails, STOP and report. The baseline must be solid before demolition.

=== TASK 2: INVENTORY DELETION TARGETS ===

Produce precise file:line lists for each deletion target. For each item, capture: file path, byte count, last commit date, what calls it (if anything).

A. Frontend admin pages with no backend:
   - frontend/src/routes/admin/boards/
   - frontend/src/routes/admin/behavior/
   - frontend/src/routes/admin/cart/
   - frontend/src/routes/admin/performance/
   - frontend/src/routes/admin/resources/
   List every .svelte file inside each, plus any +layout.svelte or +server.ts files. Confirm no Rust route is mounted at the matching API path.

B. CMS v2 backend:
   - api/src/routes/cms_v2.rs
   - api/src/routes/cms_v2_enterprise.rs
   - api/src/routes/cms_ai.rs
   - api/src/routes/cms_seo.rs
   - api/src/routes/cms_reusable_blocks.rs
   - api/src/routes/cms_presets.rs
   - api/src/routes/cms_assets.rs
   - api/src/routes/cms_scheduling.rs
   - api/src/routes/cms_datasources.rs
   - api/src/routes/cms_global_components.rs
   - api/src/routes/cms_revisions.rs
   For each file, capture: line count, mount path in mod.rs.
   List all CMS v2 frontend pages (frontend/src/routes/admin/cms/ if exists).
   Database: list every table starting with `cms_` that's NOT `categories`, `tags`, `post_*`. Capture row counts.

C. CRM subsystem:
   - api/src/routes/crm.rs (line count)
   - frontend/src/routes/admin/crm/ (every page file)
   - All `crm_*` tables (row counts)

D. Email templates UI (will be replaced):
   - api/src/routes/email_templates.rs
   - frontend/src/routes/admin/email/ (every page file)
   - email_templates, email_logs, email_campaigns, newsletter_subscribers tables (row counts)
   NOTE: We will keep the schema for email_logs and newsletter_subscribers (they will be reused). We delete email_templates and email_campaigns.

E. Old payment / subscription / checkout code:
   - api/src/routes/payments.rs
   - api/src/routes/checkout.rs
   - api/src/routes/subscriptions.rs (the user-facing one with the buggy reactivate)
   - api/src/routes/coupons.rs (the custom coupon system that violates the standard)
   - api/src/routes/connections.rs (service_connections is a phantom)
   - frontend/src/routes/admin/coupons/
   - frontend/src/routes/admin/subscriptions/
   - frontend/src/routes/admin/orders/ (will be replaced with Stripe-driven version)
   - frontend/src/routes/checkout/
   - frontend/src/routes/my/subscriptions/ (the user-facing buggy reactivate page)
   Database: orders, order_items, coupons, user_coupons, user_memberships, payment_disputes if exists.
   Capture row counts for all.

F. Duplicate / legacy tables:
   - user_status (duplicates users.is_active)
   - lessons (duplicates course_lessons)
   - indicators_enhanced
   - user_indicator_access, user_indicators, user_indicator_ownership (three duplicates)
   - service_connections (referenced by code but doesn't exist — confirm it actually doesn't exist)
   - products (the empty stub — will be REPLACED with new canonical version, but old table is dropped first)
   - membership_plans (will be REPLACED by products + product_prices canonical schema)
   - courses (current schema with price_cents — will be replaced)
   - indicators (current schema with price column — will be replaced)
   - course_modules, course_lessons, course_sections, course_resources, course_live_sessions, course_downloads, user_course_enrollments, user_lesson_progress (all 0 rows; will be rebuilt cleanly)
   - indicator_files, indicator_videos, indicator_documentation, indicator_download_log, indicator_downloads, indicator_tradingview_access (all 0 rows)
   - user_products

G. Orphan backend endpoints:
   For each endpoint mounted in api/src/routes/mod.rs that is NOT in the KEEP list, search frontend/src/ for any fetch/api call that references it. List endpoints with zero callers.

=== TASK 3: VERIFY KEEP LIST ===

Confirm these are working and present (do NOT modify them):

- api/src/middleware/auth.rs
- api/src/routes/auth.rs (login, register, refresh, logout, password reset)
- api/src/routes/admin.rs (user management endpoints only — list, get, create, update, ban, role)
- api/src/routes/posts.rs (admin_router for blog)
- api/src/routes/media.rs (upload endpoint)
- api/src/routes/trading_rooms.rs
- api/src/routes/room_content.rs
- api/src/routes/room_resources.rs
- api/src/routes/categories.rs
- api/src/routes/tags.rs
- frontend/src/routes/admin/+layout.svelte (auth gate)
- frontend/src/routes/admin/blog/
- frontend/src/routes/admin/media/
- frontend/src/routes/admin/users/ (excluding the create page that calls non-existent endpoints — will be cleaned up in Phase 2)
- frontend/src/routes/admin/trading-rooms/
- frontend/src/routes/admin/categories/
- frontend/src/routes/admin/tags/
- frontend/src/routes/blog/

=== OUTPUT ===

Write PHASE1_PREFLIGHT.md with:

1. Baseline verification results (each test passed/failed with HTTP evidence)
2. Inventory tables for sections A-G above
3. KEEP list verification
4. Total file count to delete (across all phases)
5. Total table count to drop
6. Estimated lines of code to remove (rough)
7. STOP recommendation if any baseline verification failed

DO NOT modify code. DO NOT modify DB.
```

**Verification gate before Phase 2:**
- All baseline tests passed
- PHASE1_PREFLIGHT.md exists and lists every deletion target with file:line precision
- Total estimate: how many files, how many tables, how many LOC

If anything looks wrong, stop. Don't proceed to Phase 2.

---

## 3. Phase 2: Delete Frontend Admin Pages With No Backend

**Goal:** Remove the 5 frontend admin subdirectories that call non-existent backend endpoints.

**Risk:** Low (frontend-only, easily reversed via git)

### Prompt

```
Delete the following frontend admin directories. Each contains pages that call backend endpoints that don't exist.

Pre-checked in Phase 1; no backend calls them; no other frontend page imports from them.

Targets:
- frontend/src/routes/admin/boards/
- frontend/src/routes/admin/behavior/
- frontend/src/routes/admin/cart/
- frontend/src/routes/admin/performance/
- frontend/src/routes/admin/resources/

Also clean up any references in:
- frontend/src/lib/components/admin/Sidebar.svelte (or wherever admin nav lives) — remove any links to these routes
- frontend/src/lib/api/ — delete any TS files that reference only these routes' endpoints

Steps:
1. Read frontend/src/lib/components/admin/Sidebar.svelte (or equivalent navigation). Identify any link or import targeting boards, behavior, cart, performance, or resources. List them.
2. Show me the diff of what you'll change in the navigation file. Do NOT apply yet.
3. Delete the 5 directories: rm -rf each one.
4. Apply the navigation changes from step 2.
5. Run pnpm check. Must be 0/0.
6. Run pnpm build. Must succeed.

If pnpm check or build fails after deletion, identify which file imports from a deleted path and either:
- Remove that import (preferred), or
- STOP and report what needs human review.

After completion, write PHASE2_RESULT.md with:
- List of files deleted (count + total bytes removed)
- Diff of nav changes
- pnpm check output
- pnpm build output

Do NOT commit.
```

**Verification gate before Phase 3:**
- pnpm check 0/0
- pnpm build succeeds
- Site still loads (manually visit the home page and admin home in a real browser)
- Sidebar/nav doesn't show the deleted pages

---

## 4. Phase 3: Delete CMS v2 Subsystem

**Goal:** Remove ~3,000 lines of orphaned CMS v2 backend and any associated tables.

**Risk:** Medium (large change; many files)

### Prompt

```
Delete the CMS v2 subsystem. It has no frontend wiring, 0 rows in all tables, and is pure scaffolding maintained at significant cost.

Backend files to delete entirely:
- api/src/routes/cms_v2.rs
- api/src/routes/cms_v2_enterprise.rs
- api/src/routes/cms_ai.rs
- api/src/routes/cms_seo.rs
- api/src/routes/cms_reusable_blocks.rs
- api/src/routes/cms_presets.rs
- api/src/routes/cms_assets.rs
- api/src/routes/cms_scheduling.rs
- api/src/routes/cms_datasources.rs
- api/src/routes/cms_global_components.rs
- api/src/routes/cms_revisions.rs

Mount-point cleanup (api/src/routes/mod.rs):
Remove every line that references these modules (use, mod, .nest with /admin/cms-v2 or /cms/...).

KEEP:
- api/src/routes/posts.rs (the working v1 blog admin router)
- api/src/routes/categories.rs
- api/src/routes/tags.rs
- frontend/src/routes/admin/blog/ (the working blog UI)
- categories, tags, post_categories, post_tags, posts, post_versions tables (these are blog v1, not CMS v2)

Database tables to drop (all 0 rows except cms_reusable_blocks=6, cms_navigation_menus=3, cms_site_settings=1, but these are unreachable):
- cms_reusable_blocks
- cms_navigation_menus
- cms_site_settings
- cms_sites (if exists)
- cms_pages
- cms_page_versions
- cms_components
- cms_component_versions
- cms_page_blocks
- cms_block_data
- cms_assets
- cms_asset_folders
- cms_audit_log (and all 13 monthly partitions)
- cms_workflows
- cms_workflow_states
- cms_scheduled_publishes
- cms_data_sources
- cms_global_components
- Any other table starting with cms_ that's not in the KEEP list

Steps:

1. Verify Phase 1 inventory shows zero callers from frontend/src for any /api/cms/* or /api/admin/cms-v2/* endpoint. If any callers exist, STOP.

2. Delete the Rust source files:
   git rm api/src/routes/cms_v2.rs api/src/routes/cms_v2_enterprise.rs api/src/routes/cms_ai.rs api/src/routes/cms_seo.rs api/src/routes/cms_reusable_blocks.rs api/src/routes/cms_presets.rs api/src/routes/cms_assets.rs api/src/routes/cms_scheduling.rs api/src/routes/cms_datasources.rs api/src/routes/cms_global_components.rs api/src/routes/cms_revisions.rs

3. Edit api/src/routes/mod.rs:
   - Remove every `pub mod cms_*;` line
   - Remove every `.nest("/admin/cms-v2", ...)` and `.nest("/cms/...", ...)` line
   - Show me the diff before applying.

4. Delete any frontend/src/routes/admin/cms/ directory if it exists.

5. Run cargo check. Must compile cleanly. Fix any leftover imports or references that compile-error.

6. Create a new migration file in api/migrations/ with the next available number (check existing migrations; use 046 or whatever's next). The migration drops all the cms_* tables listed above.
   IMPORTANT: Migration must use IF EXISTS clauses so it's safe even if some tables aren't in the corrupted DB.
   Example:
   DROP TABLE IF EXISTS cms_reusable_blocks CASCADE;
   DROP TABLE IF EXISTS cms_navigation_menus CASCADE;
   ...

7. Apply the migration manually via psql (since the migration system is corrupted, do not rely on the runner):
   docker exec rtp-db psql -U rtp -d revolution_trading_pros -f /path/to/046_drop_cms_v2.sql
   
   Then INSERT a row into _sqlx_migrations registering this manual application:
   INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time)
   VALUES (046, 'drop_cms_v2_tables', NOW(), true, '\\x' || encode(sha384('the actual file contents'), 'hex'), 0);

8. Run pnpm check (frontend must still compile because nothing depended on CMS v2 frontend that doesn't exist).

9. Run cargo check (backend must compile cleanly).

10. Restart the API: docker compose up -d --build api. Verify /health returns 200.

11. Smoke test: Log in as admin, navigate to /admin/blog, create a test post, view it on /blog/<slug>. Confirm blog still works.

Write PHASE3_RESULT.md with:
- List of Rust files deleted
- Diff of mod.rs changes
- Migration file content
- DB tables dropped (verified by querying \dt and confirming they're gone)
- cargo check output
- pnpm check output
- Blog smoke test result

Do NOT commit yet.
```

**Verification gate before Phase 4:**
- cargo check clean
- pnpm check 0/0
- API healthy
- Blog still works end-to-end
- All CMS v2 tables gone from DB

---

## 5. Phase 4: Delete CRM Subsystem

**Goal:** Remove the entire CRM (11 tables, 34 frontend pages, ~2,000 lines of Rust). Zero rows, zero usage.

**Risk:** Low

### Prompt

```
Delete the CRM subsystem. Zero data, zero connections to other features.

Backend:
- api/src/routes/crm.rs — delete entirely
- Remove `pub mod crm;` and `.nest("/admin/crm", crm::router())` from api/src/routes/mod.rs

Frontend:
- frontend/src/routes/admin/crm/ — delete entire directory

Navigation:
- frontend/src/lib/components/admin/Sidebar.svelte (or equivalent) — remove CRM section

Database tables to drop:
- crm_campaigns
- crm_lists
- crm_segments
- crm_sequences
- crm_automations
- crm_templates
- crm_companies
- crm_tags
- crm_smart_links
- crm_webhooks
- crm_recurring_campaigns
- Any other crm_* table

Steps:
1. Same pattern as Phase 3 — delete files, update mod.rs, delete frontend directory, write migration with next available version (047), apply manually via psql, register in _sqlx_migrations.
2. cargo check must compile cleanly.
3. pnpm check 0/0.
4. Smoke test: blog still works, admin home page still loads.

Write PHASE4_RESULT.md with the same structure as Phase 3.

Do NOT commit yet.
```

**Verification gate before Phase 5:**
- All gates from Phase 3
- /admin/crm routes return 404 in browser
- No `crm_` tables remain

---

## 6. Phase 5: Delete Duplicate / Legacy Tables

**Goal:** Remove the duplicate user_indicator_*, lessons-vs-course_lessons, indicators_enhanced, user_status duplicates.

**Risk:** Medium (touching DB schema; need to verify nothing reads from these tables)

### Prompt

```
Delete the duplicate and legacy tables that have no canonical role.

Tables to drop (verified 0 rows in Phase 1):
- user_status (duplicates users.is_active)
- lessons (legacy; course_lessons is canonical)
- indicators_enhanced (duplicates indicators)
- user_indicator_access (1 of 3 duplicates)
- user_indicators (2 of 3 duplicates)  
- user_indicator_ownership (3 of 3 duplicates)
- user_products (will be replaced by user_entitlements in Phase 7)

Pre-deletion verification:
1. Search api/src/ for any reference to each table name.
   grep -rn "user_status" api/src/
   grep -rn "FROM lessons\b" api/src/
   grep -rn "indicators_enhanced" api/src/
   grep -rn "user_indicator_access\|user_indicators\b\|user_indicator_ownership\|user_products" api/src/
   
2. For every match, decide:
   - If the code is in a file we're keeping (auth, blog, etc.): the reference must be removed because we're dropping the table.
   - If the code is in a file we're deleting in a later phase (subscriptions.rs, payments.rs): the reference will be deleted with the file. Note it.

3. For each reference in a kept file, show me the line and the proposed change. STOP for review before applying.

4. After review approval, apply the code changes.

5. Write a new migration (next available version, ~048):
   DROP TABLE IF EXISTS user_status CASCADE;
   DROP TABLE IF EXISTS lessons CASCADE;
   DROP TABLE IF EXISTS indicators_enhanced CASCADE;
   DROP TABLE IF EXISTS user_indicator_access CASCADE;
   DROP TABLE IF EXISTS user_indicators CASCADE;
   DROP TABLE IF EXISTS user_indicator_ownership CASCADE;
   DROP TABLE IF EXISTS user_products CASCADE;

6. Apply manually via psql, register in _sqlx_migrations.

7. cargo check clean. pnpm check 0/0.

8. Smoke test.

Write PHASE5_RESULT.md.

Do NOT commit yet.
```

**Verification gate before Phase 6:**
- All gates from Phase 4
- Code compiles cleanly with all references to dropped tables removed
- All listed tables gone from DB

---

## 7. Phase 6: Delete Old Payment / Subscription / Checkout Code

**Goal:** Remove every line of the broken payment system. The biggest single phase.

**Risk:** High (largest deletion; many interdependencies)

### Prompt

```
Delete the entire old payment system. We have no users, no real orders, no real subscriptions. The code is incoherent (two parallel checkout flows, exploit-ridden reactivate path, custom coupon system, base64 "encryption", placeholder Stripe URLs). Will be rebuilt fresh in Phase 7-10.

Backend files to delete entirely:
- api/src/routes/payments.rs
- api/src/routes/checkout.rs
- api/src/routes/subscriptions.rs
- api/src/routes/subscriptions_admin.rs
- api/src/routes/coupons.rs
- api/src/routes/connections.rs
- api/src/routes/admin_orders.rs
- api/src/routes/admin_courses.rs (will be rebuilt in Phase 9 against canonical schema)
- api/src/routes/admin_indicators.rs (same)
- api/src/routes/courses_admin.rs
- api/src/routes/admin_videos.rs (course-related; rebuilt in Phase 9)
- api/src/services/credential_resolver.rs (services service_connections, which doesn't exist)
- api/src/services/stripe.rs (will be rebuilt with proper signature verification using bytes)

Mount-point cleanup in api/src/routes/mod.rs:
- Remove every line referencing the deleted modules
- Remove every .nest() pointing to their routers
- Show me the full diff before applying

Frontend pages to delete:
- frontend/src/routes/admin/coupons/
- frontend/src/routes/admin/subscriptions/
- frontend/src/routes/admin/orders/
- frontend/src/routes/admin/memberships/
- frontend/src/routes/admin/courses/
- frontend/src/routes/admin/indicators/
- frontend/src/routes/admin/videos/
- frontend/src/routes/checkout/
- frontend/src/routes/my/subscriptions/
- frontend/src/lib/api/cart.ts
- frontend/src/lib/api/subscriptions.ts (if exists)
- frontend/src/lib/api/coupons.ts (if exists)
- Any other frontend/src/lib/api/*.ts that calls deleted endpoints

Navigation cleanup in admin sidebar — remove links to all deleted admin pages.

Database tables to drop (verified 0 rows):
- orders
- order_items
- coupons
- user_coupons
- user_memberships
- payment_disputes (if exists)
- products (the old empty stub — will be replaced)
- membership_plans (will be replaced by canonical products + product_prices)
- courses
- course_modules
- course_lessons
- course_sections
- course_resources
- course_live_sessions
- course_downloads
- user_course_enrollments
- user_lesson_progress
- indicators
- indicator_files
- indicator_videos
- indicator_documentation
- indicator_download_log
- indicator_downloads
- indicator_tradingview_access
- service_connections (if exists; if not, skip)
- webhooks (the 5-col stub)
- webhook_deliveries

KEEP:
- indicator_platforms (3 rows: MT4, MT5, TradingView — reference data)
- All trading_room_* and room_* tables
- All auth tables
- All blog tables
- media table
- categories, tags

Steps:
1. List every code reference (grep) to each table name being dropped in files we're KEEPING (auth, blog, media). Show diffs of removals.
2. List every code reference to deleted modules (e.g., posts.rs uses anything from coupons.rs?). Show diffs.
3. Apply code changes.
4. Delete the Rust source files.
5. Edit mod.rs to remove references.
6. Delete the frontend pages and api lib files.
7. Update sidebar nav.
8. Write a new migration (next available, ~049):
   - DROP TABLE for each table listed above with IF EXISTS CASCADE
9. Apply manually via psql, register in _sqlx_migrations.
10. cargo check clean. pnpm check 0/0. cargo build clean. pnpm build clean.
11. Smoke test:
    - Log in as admin
    - /admin/blog still works
    - /admin/media still works
    - /admin/users still works
    - /admin/trading-rooms still works
    - Public site still loads
    - Anything to do with payments/checkout/subscriptions/courses/indicators returns 404 (expected; will be rebuilt)

Write PHASE6_RESULT.md with:
- All files deleted (Rust + frontend) with line counts
- Migration file
- DB verification (\dt output showing only kept tables remain)
- cargo + pnpm check output
- Smoke test results

Do NOT commit yet.
```

**Verification gate before Phase 7:**
- All gates from Phase 5
- API healthy after major deletion
- Blog, media, users, rooms still work
- Total LOC removed: should be ~10,000+ across phases 2-6

---

## 8. Phase 7: Build Canonical Product Schema

**Goal:** Create `products`, `product_prices`, `user_entitlements`, `webhook_events`, `payment_disputes`, `refunds`, `reconciliation_log` tables per PAYMENTS_ARCHITECTURE_STANDARD.md sections 3, 4, 6.

**Risk:** Medium (new schema; must match spec exactly)

### Prompt

```
Build the canonical product schema per PAYMENTS_ARCHITECTURE_STANDARD.md.

Read the standard document first, especially sections 3 (Product Catalog Model) and 4 (Access State Model) and 6 (Webhook Handler Architecture).

Create a new migration in api/migrations/ (next available version, ~050) with these tables exactly per the standard. Use the SQL from the standard document verbatim — do not invent variations.

Tables to create:
1. products (per Standard §3)
2. product_prices (per Standard §3)
3. user_entitlements (per Standard §4)
4. webhook_events (per Standard §6)
5. payment_disputes (chargeback audit table)
6. refunds (refund audit table; structure: id, stripe_charge_id, stripe_refund_id, amount_cents, currency, reason, refunded_at)
7. reconciliation_log (id, run_at, discrepancies_found, log JSONB)

Also add:
- A `stripe_customer_id TEXT` column to the existing `users` table (UNIQUE constraint; nullable since not all users have purchased)

Steps:
1. Write the migration SQL. Show me the full file before applying.
2. Apply manually via psql.
3. Register in _sqlx_migrations.
4. Verify each table exists with \d <table>.
5. Verify users table now has stripe_customer_id column.
6. cargo check clean (no Rust changes yet, but compile to confirm).

Then in Rust:
1. Create api/src/models/product.rs with structs for Product, ProductPrice, UserEntitlement matching the schema.
2. Create api/src/models/payment.rs with structs for WebhookEvent, PaymentDispute, Refund, ReconciliationLog.
3. Update api/src/models/user.rs User struct to include `stripe_customer_id: Option<String>`.
4. Update api/src/middleware/auth.rs SELECT query to include stripe_customer_id.
5. cargo check clean.

Write PHASE7_RESULT.md with:
- Migration SQL
- Schema verification (\d output for each new table)
- New Rust model files
- cargo check output

Do NOT commit yet.
```

**Verification gate before Phase 8:**
- All new tables exist matching spec
- users.stripe_customer_id column added
- cargo check clean

---

## 9. Phase 8: Build Canonical Checkout Endpoint + Webhook Handler

**Goal:** Implement the single canonical checkout flow (Standard §5) and webhook handler (Standard §6).

**Risk:** High (real payment code; must be exactly right)

### Prompt

```
Build the canonical checkout and webhook system per PAYMENTS_ARCHITECTURE_STANDARD.md sections 5 and 6.

Read the standard sections carefully. Note especially the hard rules:
- Use axum::body::Bytes for webhook body (NOT String)
- Webhook signature verification mandatory in ALL environments (no dev bypass)
- Webhook events idempotent via DB unique constraint
- All money is integer cents
- Server-side price validation against product_prices.is_active
- success_url and cancel_url built server-side from APP_URL only
- One endpoint POST /api/checkout for all product types

Files to create:
1. api/src/routes/checkout.rs (NEW, fresh)
   - POST /api/checkout endpoint
   - GET /api/checkout/session/:id endpoint (for frontend success polling)
   - Single canonical pattern per Standard §5

2. api/src/routes/webhooks.rs (NEW)
   - POST /api/webhooks/stripe endpoint
   - All 9 required event handlers per Standard §6

3. api/src/services/stripe.rs (NEW, replacing the deleted one)
   - Real signature verification using HMAC-SHA256 with raw bytes
   - Real Stripe SDK calls (use the official `stripe` crate or `async-stripe`; pick one and pin it)
   - No "dev bypass" — fail closed always
   - Methods: create_checkout_session, retrieve_session, retrieve_subscription, list_active_subscriptions, create_billing_portal_session

4. api/src/handlers/payments/ — directory for individual webhook handlers
   - checkout_completed.rs
   - subscription_created.rs
   - subscription_updated.rs
   - subscription_deleted.rs
   - invoice_paid.rs
   - payment_failed.rs
   - charge_refunded.rs
   - dispute_created.rs
   - trial_will_end.rs

Mount points in api/src/routes/mod.rs:
- pub mod checkout;
- pub mod webhooks;
- .nest("/api/checkout", checkout::router())
- .nest("/api/webhooks", webhooks::public_router())

Code requirements per Standard §6:
- Webhook handler reads body as Bytes
- Verify signature → reject 400 if invalid
- INSERT INTO webhook_events ... ON CONFLICT (event_id) DO NOTHING RETURNING id; if returns nothing, return 200 immediately (idempotency)
- Dispatch to handler based on event_type
- Each handler propagates errors (no .ok() swallowing)
- After handler success, UPDATE webhook_events SET processed_at = NOW()

Frontend:
1. Create frontend/src/routes/checkout/+page.svelte (success/cancel landing pages)
2. Create frontend/src/routes/checkout/success/+page.svelte (polls /api/checkout/session/:id)
3. Create frontend/src/lib/api/checkout.ts (single client function: startCheckout(stripe_price_id))

Verification at end of phase:
1. cargo check clean
2. pnpm check 0/0
3. Manual test using Stripe CLI:
   - stripe listen --forward-to localhost:8080/api/webhooks/stripe
   - In test mode, manually call POST /api/checkout with a real Stripe price_id (pre-create one in Stripe test dashboard)
   - Complete checkout with test card 4242 4242 4242 4242
   - Verify webhook arrives, signature verifies, entitlement is created in user_entitlements
4. Idempotency test: stripe events resend <event-id> and verify the second delivery does not create a duplicate entitlement

Write PHASE8_RESULT.md with:
- All new files
- Stripe CLI test session output
- Idempotency test result
- cargo + pnpm check output

Do NOT commit yet.
```

**Verification gate before Phase 9:**
- Real Stripe test purchase works end-to-end (test card, real webhook, real entitlement)
- Idempotency verified (duplicate webhook doesn't double-grant)
- All 9 webhook events handle correctly (test with `stripe trigger`)

---

## 10. Phase 9: Build Product Admin UI

**Goal:** Admin UI for creating products that reference Stripe IDs (no local prices).

**Risk:** Medium

### Prompt

```
Build the admin UI for product management per PAYMENTS_ARCHITECTURE_STANDARD.md section 12.

Backend:
1. api/src/routes/admin/products.rs (NEW)
   - GET /api/admin/products (list)
   - POST /api/admin/products (create — requires stripe_product_id, slug, type, name)
   - GET /api/admin/products/:id (detail)
   - PUT /api/admin/products/:id (update metadata; price changes go through Phase 10 endpoint)
   - DELETE /api/admin/products/:id (soft delete via is_active = false)
   - POST /api/admin/products/:id/prices (add a Stripe price reference)
   - DELETE /api/admin/products/:id/prices/:price_id (remove a price reference)

   All endpoints require AdminUser extractor.
   All endpoints log to security_events (admin actions are audited).

2. api/src/services/stripe_catalog.rs (NEW)
   - fetch_price_by_id(stripe_price_id) — for verifying a price exists in Stripe before adding to product_prices
   - fetch_product_by_id(stripe_product_id) — for verifying a product exists before adding to products
   - get_price_amount(stripe_price_id) — for the pricing page (cached briefly)

Frontend:
1. frontend/src/routes/admin/products/+page.svelte (list)
2. frontend/src/routes/admin/products/create/+page.svelte
   - Form: type (course/indicator/room/alert), slug, name, description, stripe_product_id, sort_order, is_featured
   - NO price field (must be entered in Stripe directly)
   - On submit: validates stripe_product_id exists in Stripe; creates row.
3. frontend/src/routes/admin/products/[id]/+page.svelte (edit)
   - Edit metadata
   - Manage attached prices (add/remove stripe_price_id references)
   - NO direct price editing
4. frontend/src/lib/api/products.ts

Verification:
1. cargo check clean
2. pnpm check 0/0
3. Manual test:
   - Create a product in Stripe dashboard test mode (e.g., a course)
   - Note stripe_product_id and stripe_price_id
   - In admin UI: create a product referencing that ID; add the price reference
   - Verify the row appears in products and product_prices tables with correct Stripe IDs
   - Try to checkout for that product (using checkout endpoint from Phase 8)
   - Test card 4242 4242 4242 4242 should grant entitlement

Write PHASE9_RESULT.md.

Do NOT commit yet.
```

**Verification gate before Phase 10:**
- Admin can create a product with Stripe IDs
- Admin UI shows NO local price field anywhere
- End-to-end checkout works with newly-created product
- All admin endpoints audit-logged

---

## 11. Phase 10: Build Price-Change Feature

**Goal:** The three-option price change feature per Standard §8.

**Risk:** Medium

### Prompt

```
Build the price-change feature per PAYMENTS_ARCHITECTURE_STANDARD.md section 8.

Backend endpoint:
POST /api/admin/products/:id/change-price

Body:
{
  "stripe_price_id_to_replace": "price_OLD123",  // required
  "new_amount_cents": 12700,
  "currency": "usd",
  "interval": "month",  // or "quarter", "year", or null for one-time
  "apply_to": "new_only" | "next_renewal" | "immediate_proration"
}

Logic per Standard §8:
1. Auth: AdminUser, ideally super_admin for "immediate_proration"
2. Validate the old stripe_price_id exists in product_prices for this product
3. Call Stripe to create new Price with new_amount_cents on the same Stripe product
4. Call Stripe to deactivate old Price (active = false)
5. Update product_prices: insert new row, mark old row as is_active = false

If apply_to == "next_renewal":
6a. List all active subscriptions on the old price (via Stripe)
6b. For each, call subscription.update with new price + proration_behavior=none
6c. Log each subscription updated to security_events

If apply_to == "immediate_proration":
6a. Same as above but proration_behavior=create_prorations

If apply_to == "new_only":
   No subscription updates. Existing subs continue on the old price.

7. Audit log: write security_events row with the operation summary

Frontend:
- Modal triggered from /admin/products/[id]/+page.svelte
- Three radio buttons matching the spec
- Confirm button
- Shows preview: "X subscribers currently on this price will be affected"

Verification:
1. cargo check + pnpm check
2. Manual test in Stripe test mode:
   - Create 2 test subscriptions on a price (use 2 test users + test cards)
   - Run "new_only" — verify existing subs unchanged, new price created
   - Run "next_renewal" — verify old subs scheduled to switch
   - Run "immediate_proration" — verify Stripe applies prorations on the test data

Write PHASE10_RESULT.md.

Do NOT commit yet.
```

**Verification gate before Phase 11:**
- All three apply_to modes work correctly with Stripe test data
- Audit logging works
- Existing subscribers correctly affected per the chosen mode

---

## 12. Phase 11: Wire Up Postmark Email

**Goal:** Real email integration with the 10 transactional triggers per Standard §15 (and ADMIN_SYSTEM_DISCOVERY §9).

**Risk:** Medium

### Prompt

```
Build real email sending via Postmark.

Setup:
1. Confirm POSTMARK_TOKEN exists in api/.env (test server token from Postmark dashboard)
2. Confirm POSTMARK_FROM_EMAIL is set
3. Add the postmark crate to Cargo.toml (postmark = "0.x") OR use reqwest directly with the Postmark HTTP API

Backend:
1. api/src/services/email.rs (NEW)
   - struct EmailService with from_email and Postmark client
   - async fn send_transactional(to: &str, template_alias: &str, model: serde_json::Value) -> Result<()>
   - The function calls Postmark's "Send With Template" API
   - On failure: log error, write to email_logs table with status='failed', RETURN error (do not silently swallow)

2. Create email templates in Postmark dashboard with these aliases:
   - welcome (sent on registration)
   - email-verification (sent on registration)
   - password-reset (sent on password reset request)
   - subscription-confirmation (sent after checkout.session.completed for subscriptions)
   - subscription-canceled (sent on customer.subscription.deleted)
   - subscription-renewal-reminder (sent 7 days before renewal)
   - trial-ending (sent on customer.subscription.trial_will_end)
   - failed-payment (sent on invoice.payment_failed)
   - receipt (sent on invoice.paid)
   - refund-confirmation (sent on charge.refunded)

3. Wire the triggers:
   - api/src/routes/auth.rs register handler → call send_transactional("welcome", ...) and ("email-verification", ...)
   - api/src/routes/auth.rs forgot_password handler → call send_transactional("password-reset", ...)
   - api/src/handlers/payments/checkout_completed.rs → if subscription, send "subscription-confirmation"; if one-time (course/indicator), send "receipt"
   - api/src/handlers/payments/subscription_deleted.rs → send "subscription-canceled"
   - api/src/handlers/payments/payment_failed.rs → send "failed-payment"
   - api/src/handlers/payments/trial_will_end.rs → send "trial-ending"
   - api/src/handlers/payments/charge_refunded.rs → send "refund-confirmation"
   - For renewal reminders: write a daily scheduled job (api/src/jobs/renewal_reminders.rs) that queries upcoming renewals and sends emails

4. Use the existing email_logs table to record every send attempt (success or failure).

Verification:
1. cargo check clean
2. Use Postmark's test server token; emails go to your real inbox.
3. Manual test each trigger:
   - Register a user → welcome + verification emails arrive
   - Click verification link → user verified
   - Request password reset → email arrives
   - Test mode purchase a course → receipt arrives
   - Test mode purchase a subscription → confirmation arrives
   - Cancel subscription via Customer Portal → cancellation email arrives
   - Use Stripe trigger to fire payment_failed → failed-payment email arrives

Write PHASE11_RESULT.md with each trigger's verification.

Do NOT commit yet.
```

**Verification gate before Phase 12:**
- All 10 email triggers send real email
- All sends logged to email_logs
- Failures don't crash anything; they log

---

## 13. Phase 12: End-to-End Verification & Reconciliation Job

**Goal:** Verify the complete system works. Build the daily reconciliation job per Standard §14.

**Risk:** Low

### Prompt

```
Final verification phase + reconciliation job.

Reconciliation job (per Standard §14):
1. Create api/src/jobs/reconcile_stripe.rs
2. Function: reconcile_subscriptions() that:
   - Fetches all active subscriptions from Stripe (paginated)
   - For each, looks up user_entitlements by stripe_subscription_id
   - Compares state, logs discrepancies to reconciliation_log table
   - Auto-corrects if Stripe says canceled but DB says active (revoke entitlement)
   - Lists DB entitlements not in Stripe; revokes them
3. Wire to a scheduler (use tokio_cron_scheduler or similar; runs daily at 03:00 UTC)
4. Add admin endpoint GET /api/admin/reconciliation/log to view results

End-to-end test scenarios (use Stripe test mode + test cards):

A. Course purchase (one-time, lifetime access):
   - Create test course in Stripe + admin UI
   - Buy with 4242 4242 4242 4242
   - Verify entitlement created with expires_at = NULL
   - Verify receipt email arrived
   - Verify access granted via user_has_access_to_product()

B. Subscription purchase (monthly):
   - Create test trading room product with monthly price in Stripe + admin UI
   - Buy with 4242 4242 4242 4242
   - Verify entitlement created with expires_at = current_period_end
   - Verify confirmation email
   - Verify access

C. Subscription renewal:
   - Use Stripe CLI to advance time
   - Verify invoice.paid webhook arrives, expires_at updated

D. Subscription cancellation:
   - Click "Cancel" in Customer Portal
   - Verify cancel_at_period_end=true; access continues
   - Click "Resume" — verify access continues uninterrupted
   - Cancel and let period end — verify entitlement revoked

E. Refund (course):
   - Issue refund via Stripe dashboard
   - Verify charge.refunded webhook
   - Verify entitlement.revoked_at set
   - Verify refund email sent
   - Verify user no longer has access

F. Failed payment:
   - Use Stripe trigger invoice.payment_failed
   - Verify webhook handled
   - Verify entitlement metadata reflects past_due
   - Verify failed-payment email sent

G. Coupon (Stripe Promotion Code):
   - Create promo code in Stripe dashboard
   - Run checkout with allow_promotion_codes=true
   - Apply code on Stripe Checkout page
   - Verify discount applied, entitlement granted at correct amount

H. Price change "new_only":
   - Have a test subscription on $97/mo
   - Run price change with new_only to $127/mo
   - Verify existing sub still on $97
   - New checkout uses $127

I. Idempotency:
   - Re-send a checkout.session.completed event via Stripe CLI
   - Verify only one entitlement exists

J. Admin actions audited:
   - Ban a user → security_events row exists
   - Change a role → security_events row exists
   - Issue a refund → security_events row exists

K. Reconciliation:
   - Manually break sync: change a Stripe subscription to canceled but leave entitlement active in DB
   - Run reconciliation job manually
   - Verify entitlement revoked

Write PHASE12_RESULT.md with all 11 scenarios documented as PASS/FAIL with HTTP / DB evidence.

Final gates:
- cargo check clean (across entire codebase)
- pnpm check 0/0
- All migrations registered in _sqlx_migrations
- docker compose ps all healthy
- All emails arriving
- All webhooks idempotent

If all 11 scenarios PASS, the system is ready for live mode promotion.

Do NOT commit. Final commit happens after this report is reviewed.
```

**Final verification gate:**
- All 11 scenarios PASS with HTTP + DB evidence
- Live mode promotion checklist (Standard §13) reviewed and ready

---

## 14. Post-Demolition Cleanup

After all 12 phases pass, do the following in a final session:

1. Squash and merge `demolition-rebuild-2026-04` to main with a single coherent commit message:

```
refactor: full demolition + canonical rebuild per PAYMENTS_ARCHITECTURE_STANDARD

- Removed CMS v2 subsystem (~3,000 lines, 20 tables, no users)
- Removed CRM scaffolding (~2,000 lines, 11 tables, no data)
- Removed orphaned admin pages (boards/behavior/cart/performance/resources)
- Removed duplicate tables (user_status, lessons, indicators_enhanced, user_indicator_*, user_products)
- Removed broken payment system (incoherent two-checkout flows, exploits, base64 "encryption")
- Removed legacy product/course/indicator schemas with local prices

- Built canonical product schema (products + product_prices + user_entitlements per spec)
- Built canonical webhook handler (idempotent, fail-closed signature, axum::body::Bytes)
- Built single canonical checkout endpoint (POST /api/checkout)
- Built admin product management UI (Stripe IDs only, no local prices)
- Built three-option price change feature
- Wired Postmark for 10 transactional emails
- Built daily Stripe reconciliation job

References:
- PAYMENTS_ARCHITECTURE_STANDARD.md (canonical spec)
- AUTH_AUDIT.md, SUBSCRIPTION_AUDIT.md, ADMIN_SYSTEM_DISCOVERY.md (audit trail)
- PHASE{1-12}_RESULT.md (verification evidence)
```

2. Update CLAUDE.md to reference PAYMENTS_ARCHITECTURE_STANDARD.md as the canonical spec for all future payment work.

3. Save the standard to memory:

```
memory_user_edits add: Payments and subscription systems must follow PAYMENTS_ARCHITECTURE_STANDARD.md (Stripe is source of truth for prices/coupons/products; app stores entitlements and Stripe IDs; webhooks are the only access-granting path; idempotent via webhook_events table; integer cents always; Stripe Customer Portal for billing UI; Stripe Promotion Codes for coupons; one canonical /api/checkout endpoint for all product types). Apply to RTP and all future projects.
```

4. Delete intermediate PHASE*_RESULT.md files (the canonical history is in git commit messages and the standard doc).

5. Tag the commit: `git tag v0.2.0-canonical-payments`

---

## 15. Failure Recovery

If any phase fails verification:

- **Stop immediately.** Do not proceed to the next phase.
- **Do not try to repair the failed phase forward.** Roll back via git reset --hard or restore the DB backup from Phase 0.
- **Re-plan.** Identify what went wrong, write a corrected prompt, run again on a fresh git branch.

If the migration system corruption causes problems mid-phase:

- Skip the affected migration registration.
- Apply the schema change directly via psql.
- Continue forward.
- Address migration system reconciliation as a separate post-rebuild task.

---

## 16. Out of Scope (Post-Rebuild Backlog)

These don't block the rebuild but should be addressed after:

1. Migration system reconciliation (the 030/037/040/041 duplicates and unregistered 031-035)
2. Test inventory and triage (per the test inventory prompt from prior session)
3. Localhost port flexibility (per the prompt from prior session)
4. Admin "comp subscription" feature
5. Admin-triggered password reset
6. SEO endpoints (verify Search Console wiring)
7. Bounce rate analytics (currently a stub)
8. Webhook retention / archival policy
9. Customer Portal configuration in live Stripe
10. Live mode promotion (Standard §13 checklist)

---

*End of Demolition & Rebuild Plan v1.0.*
