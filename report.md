# Admin Back-End & Front-End Audit Follow-Up — Verification & Remediation Report

**Repository:** `/Users/billyribeiro/dev/revolution-trading-pros`  
**Source Branch:** `fix/audit-followup-2026-11` → Squashed & Merged into `main`  
**Audit References:** `beaudit-report.md`, `delivered-report.md`, `beaudit2-report.md`, `followup-prompt.md`

---

## Executive Summary

Following two independent, full-surface audits of the local Admin back-end (`api/`, Rust + Axum + SQLx + PostgreSQL + Redis) and Admin front-end (`frontend/src/routes/admin/**`, SvelteKit 2 + Svelte 5), every reported finding was independently verified against live code and database schema, reproduced via integration tests before fixing, and remediated across four tiers:

1. **Tier 1 — Security, Data-Loss & Correctness Bugs (8 items):** Closed unauthenticated/broken RBAC on `/api/admin/cms-v2/*` and `/api/admin/room-content/*` (plus `/api/room-content/*` write routes), fixed uncommitted transaction rollback data loss in `POST /api/admin/members`, embedded unique UUID v4 `jti` claims in refresh JWTs to prevent same-second token hash collisions, hardened pagination against integer overflow and negative parameters, enforced input validation (`price_cents >= 0`, non-blank `name`) and `409 Conflict` on duplicate subscription plan slugs, fixed case-insensitive trade direction PnL calculation (`"long"` / `"short"`) and repeat-close `409 Conflict` guards, and fixed `setup_db` `password_resets` DDL (`id BIGSERIAL PRIMARY KEY`), `run_migrations` dev-only environment gating, and error message scrubbing.
2. **Tier 2 — Backend 500s, Schema Drift, Error Masking & RBAC Extractor Ordering (94 files):** Added migration `072_audit_followup_schema_alignment.sql` (and updated `schema.sql`, `sqlx-seed.sql`, and `g03_schema_baseline_test.rs`) to align all missing tables and columns (`form_submissions`, `contacts`, `crm_contacts`, `coupons`, `indicator_videos`, `trading_room_videos`, `weekly_watchlists`, `room_resources`, `page_layouts`, `cms_workflows`, `cms_ab_tests`, `cms_SEO_audits`, `cms_datasources`, `cms_webhooks`, `cms_translation_jobs`). Replaced silent `unwrap_or_default()` DB error masking with real `500` propagation via `?`, added `rows_affected() == 0` → `404` checks on `UPDATE`/`DELETE` handlers, mounted all 14 previously unmounted CRM deals/pipelines handlers under `/api/admin/crm/...` with `AdminUser`, standardized monetary values on `i64` cents (`price_cents`), and moved `AdminUser` / `CmsEditorUser` extractors to be the first handler argument after `State` across all CMS routes so unauthenticated callers receive `401`/`403` before body/path parsing (`400`/`422`).
3. **Tier 3 — Frontend Crashes, Proxy Routing, `$NaN` Rendering & Native Dialogs (26 files):** Removed hardcoded `https://revolution-trading-pros-api.fly.dev` fallbacks in `frontend/src/lib/api/config.ts`, updated `frontend/vite.config.ts` CSP `connect-src` for local dev ports, created `/api/admin/analytics/[...rest]/+server.ts` and expanded `/api/[...path]/+server.ts` so `/api/admin/analytics/*` reaches the Rust backend instead of being shadowed by local mock endpoints, added null/array defensive normalization in `/admin/popups/new`, `/admin/popups/[id]/edit`, `/admin/site-health`, `/admin/settings`, and `/admin/media`, fixed `$NaN` price rendering on `/admin/orders`, `/admin/memberships`, and `/admin/subscriptions/plans` (`price_cents / 100`), replaced all native `confirm()` / `alert()` dialogs on `/admin/courses/*` and `/admin/members/service/[id]` with inline confirmation modals and banners, and hydrated edit forms on `/admin/products/[id]/edit`, `/admin/coupons/edit/[id]`, and `/admin/users/edit/[id]`.
4. **Tier 4 — Test Suite Integrity & Real-Router Wiring (9 files):** Added `api/tests/tier1_followup_test.rs` with 8 `.oneshot()` integration tests proving each Tier 1 bug failed before the fix and passes after, installed per-app `MockConnectInfo<SocketAddr>` in `api/tests/common/mod.rs` and updated `test_rate_limiting` in `api/tests/integration_tests.rs` to unconditionally assert `429 Too Many Requests` against `/api/auth/forgot-password`, and rewired all 5 shadow-router test suites (`orders_test.rs`, `payments_test.rs`, `admin_indicators_test.rs`, `cms_v2_test.rs`, `oauth_test.rs`) to test through `common::setup_test_app().await` (the real production router).

---

## 1. Complete Verification Table (16 Audit Items)

| # | Audit ID(s) | Claim Summary | Verdict | Evidence (`file:line` & Regression Test) |
|---|---|---|---|---|
| 1 | `CF-01` / `F-01` | 8 GET handlers under `/api/admin/cms-v2/*` lack `AdminUser` and return `200` to anonymous & `role="user"` callers | **CONFIRMED → FIXED** | `api/src/routes/cms_v2.rs:118-625` (`AdminUser` added to `list_workflows`, `get_workflow`, `list_ab_tests`, `list_seo_audits`, `list_datasources`, `list_webhooks`, `list_translation_jobs`, `get_Single_type`); verified by `tier1_1_cms_v2_admin_gets_require_admin` |
| 2 | `CF-02` / `F-02` | `api/src/routes/room_content.rs` `admin_router()` is never mounted in `api/src/routes/mod.rs`, and public write handlers lack `AdminUser` | **CONFIRMED → FIXED** | `api/src/routes/mod.rs:228-229` (mounts `/api/admin/room-content`), `api/src/routes/room_content.rs:150-926` (`AdminUser` added to all write handlers); verified by `tier1_2_admin_room_content_requires_admin` |
| 3 | `CF-03` / `F-03` | `create_member` (`POST /api/admin/members`) opens `tx = state.db.pool.begin()`, inserts into `tx`, logs on `state.db.pool`, and drops `tx` without `tx.commit()` — losing the user row and violating FK | **CONFIRMED → FIXED** | `api/src/routes/admin_members.rs:1401-1432` (executes activity log inside `&mut *tx` and calls `tx.commit().await?`); verified by `tier1_3_create_member_persists_user_and_activity_log` |
| 4 | `F-21` | `create_refresh_jwt` sets only `sub`, `exp`, `iat`, `iss`; two tokens minted in the same second for the same user produce identical SHA-256 hashes and hit `UNIQUE (token_hash)` on `/api/auth/refresh` | **CONFIRMED → FIXED** | `api/src/utils/jwt.rs:16-142` (`jti: Some(Uuid::new_v4().to_string())`), `api/src/routes/auth.rs:656-660` (`ON CONFLICT (token_hash) DO NOTHING`); verified by `tier1_4_refresh_token_embeds_jti_and_survives_same_second_rotation` |
| 5 | `F-04` | `(page - 1) * per_page` overflows (`panic` in debug / negative `OFFSET` in release) on `page=i64::MAX`, `page <= 0`, or huge `per_page` | **CONFIRMED → FIXED** | `api/src/utils/pagination.rs:1-21` (`normalize_pagination` saturating clamping `page: 1..=100_000`, `per_page: 1..=100`), applied across all paginated routes; verified by `tier1_5_pagination_clamps_overflow_and_negative_params` |
| 6 | `F-08` | `POST /api/subscriptions/plans` accepts negative prices (`price_cents < 0`), blank names, and returns `500` instead of `409` on duplicate slug | **CONFIRMED → FIXED** | `api/src/routes/subscriptions.rs:163-234` (validates `price_cents >= 0` & non-blank `name` → `422`, maps Postgres `23505` → `409 Conflict`); verified by `tier1_6_subscription_plan_rejects_negative_price_blank_name_and_409_on_duplicate` |
| 7 | `F-09` | `POST /api/room-content/trades/:id/close` compares `direction == "LONG"` case-sensitively (inverting PnL for `"long"`) and allows re-closing an already-closed trade | **CONFIRMED → FIXED** | `api/src/routes/room_content.rs:636-702` (`eq_ignore_ascii_case("LONG")` / `"SHORT"`, rejects unknown direction with `400`, checks `status == "closed"` / `exit_price.is_some()` → `409 Conflict`); verified by `tier1_7_close_trade_case_insensitive_direction_and_409_on_repeat_close` |
| 8 | `CF-10` / `F-11` | `POST /setup-db`, `/run-migrations`, `/init-db`: `setup_db` DDL omits `id` PK on `password_resets`, `run_migrations` lacks dev-only guard, and `SetupResponse::message` leaks raw SQL error strings | **CONFIRMED → FIXED** | `api/src/routes/health.rs:355-600` (`id BIGSERIAL PRIMARY KEY` on `password_resets`, `ENVIRONMENT` guard on `run_migrations`, scrubbed generic error strings); verified by `tier1_8_setup_db_uses_bigserial_password_resets_and_scrubs_errors` & `health_test.rs` |
| 9 | `CF-04..09` / `F-05..07` | Schema/code drift across `forms`, `crm`, `coupons`, `admin_indicators`, `admin_videos`, `admin_watchlists`, `room_resources`, `admin_page_layouts`, and `cms_v2` causes `500`s on live DB | **CONFIRMED → FIXED** | `api/migrations/072_audit_followup_schema_alignment.sql`, `api/migrations/schema.sql`, `api/migrations/sqlx-seed.sql`, `api/tests/g03_schema_baseline_test.rs`, and corresponding route handlers |
| 10 | `CF-11` / `F-10` | `unwrap_or_default()` / `unwrap_or(0)` in `admin_members.rs`, `crm/mod.rs`, `analytics.rs`, `products.rs`, `coupons.rs`, and `cms_v2.rs` silently masks SQL errors as `200 OK` with empty arrays/zeros | **CONFIRMED → FIXED** | `api/src/routes/admin_members.rs:371-2465`, `api/src/routes/crm/mod.rs:228-884`, `api/src/routes/analytics.rs:422-1324`, `api/src/routes/products.rs:146-152`, `api/src/routes/coupons.rs:117-123` (propagates DB errors via `?`) |
| 11 | `CF-12` | `PUT`/`DELETE` handlers in `admin_members.rs`, `courses.rs`, `products.rs`, `coupons.rs`, `redirects.rs`, and `email_templates.rs` return `200 OK` when target ID does not exist | **CONFIRMED → FIXED** | Added `result.rows_affected() == 0` / `fetch_optional` → `Err((StatusCode::NOT_FOUND, ...))` across all affected `UPDATE`/`DELETE` handlers |
| 12 | `CF-13` / `F-13` / `F-14` | `/admin/popups/new`, `/admin/popups/[id]/edit`, `/admin/site-health`, `/admin/settings`, and `/admin/media` crash or render blank on shape mismatches | **CONFIRMED → FIXED** | `frontend/src/routes/admin/popups/new/+page.svelte`, `frontend/src/routes/admin/popups/[id]/edit/+page.svelte`, `frontend/src/routes/admin/site-health/+page.svelte`, `frontend/src/routes/admin/settings/+page.svelte`, `frontend/src/routes/admin/media/+page.svelte` |
| 13 | `CF-16` / `F-17` | `frontend/src/lib/api/config.ts` hardcodes dead `https://revolution-trading-pros-api.fly.dev` fallback, CSP blocks local API port, and SvelteKit `/api/admin/analytics/*` mock routes shadow backend | **CONFIRMED → FIXED** | `frontend/src/lib/api/config.ts:10-35`, `frontend/vite.config.ts:108-120`, `frontend/src/routes/api/[...path]/+server.ts`, `frontend/src/routes/api/admin/analytics/[...rest]/+server.ts` |
| 14 | `CF-18` / `F-18` | `/admin/orders`, `/admin/memberships`, and `/admin/subscriptions/plans` display `$NaN` because frontend reads float `total`/`price` while backend returns `i64` `*_cents` | **CONFIRMED → FIXED** | `frontend/src/routes/admin/orders/+page.svelte`, `frontend/src/routes/admin/memberships/+page.svelte`, `frontend/src/routes/admin/subscriptions/plans/+page.svelte`, `frontend/src/lib/remotes/plans.remote.ts`, `frontend/src/lib/remotes/orders.remote.ts` |
| 15 | `CF-21` | `api/tests/integration_tests.rs:314` (`test_rate_limiting`) is vacuous (`if !got_rate_limited { println!(...); }` without `assert!`) | **CONFIRMED → FIXED** | `api/tests/common/mod.rs:131-162` (attaches `MockConnectInfo<SocketAddr>` per test router), `api/tests/integration_tests.rs:314-356` (unconditionally asserts `assert!(got_rate_limited, ...)`) |
| 16 | `CF-22` / `F-20` | 5 test files (`orders_test.rs`, `payments_test.rs`, `admin_indicators_test.rs`, `cms_v2_test.rs`, `oauth_test.rs`) test hand-built shadow routers instead of the real production router | **CONFIRMED → FIXED** | Rewired all 5 test files to dispatch requests against `common::setup_test_app().await` |

---

## 2. Detailed Remediation by Tier

### 2.1 Tier 1 — Security, Data-Loss & Correctness Fixes

1. **`/api/admin/cms-v2/*` RBAC Enforcement (`CF-01` / `F-01`):**
   - Added `_admin: AdminUser` as the first extractor after `State` on all 8 admin GET handlers in `api/src/routes/cms_v2.rs`: `list_workflows`, `get_workflow`, `list_ab_tests`, `list_seo_audits`, `list_datasources`, `list_webhooks`, `list_translation_jobs`, and `get_single_type`.
   - Verified that anonymous callers receive `401 Unauthorized`, `role="user"` callers receive `403 Forbidden`, and `role="admin"` callers receive `200 OK`.

2. **`/api/admin/room-content/*` Mount & Write RBAC (`CF-02` / `F-02`):**
   - Mounted `room_content::admin_router()` at `/api/admin/room-content` in `api/src/routes/mod.rs:228-229`.
   - Added `_admin: AdminUser` as the first extractor after `State` on every write handler (`POST`, `PUT`, `DELETE`) in `api/src/routes/room_content.rs` so both `/api/admin/room-content/*` and `/api/room-content/*` write endpoints require admin privileges while keeping public read (`GET`) endpoints accessible.

3. **`POST /api/admin/members` Transaction Commit (`CF-03` / `F-03`):**
   - Fixed `create_member` in `api/src/routes/admin_members.rs:1401-1432` to execute the `INSERT INTO user_activity_log` against `&mut *tx` (instead of `state.db.pool`) and call `tx.commit().await` before returning `201 Created`.

4. **Refresh JWT `jti` Uniqueness (`F-21`):**
   - Added `#[serde(skip_serializing_if = "Option::is_none")] pub jti: Option<String>` to `Claims` in `api/src/utils/jwt.rs` and populated `jti: Some(Uuid::new_v4().to_string())` in `create_refresh_jwt`.
   - Added defensive `ON CONFLICT (token_hash) DO NOTHING` on `user_sessions` insert in `api/src/routes/auth.rs:656-660`.

5. **Pagination Overflow & Bounds Hardening (`F-04`):**
   - Created `normalize_pagination(page: Option<i64>, per_page: Option<i64>, default_per_page: i64, max_per_page: i64) -> (i64, i64, i64)` in `api/src/utils/pagination.rs` using `clamp(1, 100_000)` and `saturating_sub(1).saturating_mul(per_page)`.
   - Applied `normalize_pagination` across `courses.rs`, `products.rs`, `orders.rs`, `admin_members.rs`, `user.rs`, `posts.rs`, `room_content.rs`, `room_resources.rs`, `admin_indicators.rs`, `admin_videos.rs`, `admin_watchlists.rs`, and `crm/mod.rs`.

6. **Subscription Plan Input Validation & Duplicate Slug Handling (`F-08`):**
   - Added validation in `create_plan` and `update_plan` (`api/src/routes/subscriptions.rs:163-234`) rejecting `price_cents < 0` or blank `name` with `422 Unprocessable Entity`, and mapping Postgres unique constraint error `23505` on `slug` to `409 Conflict`.

7. **Trade Close PnL Direction & Repeat-Close Guard (`F-09`):**
   - Updated `close_trade` in `api/src/routes/room_content.rs:636-702` to check `trade.status.eq_ignore_ascii_case("closed") || trade.exit_price.is_some()` and return `409 Conflict` on repeat close.
   - Replaced `if trade.direction == "LONG"` with case-insensitive matching (`eq_ignore_ascii_case("LONG")` / `eq_ignore_ascii_case("SHORT")`), rejecting unrecognized directions with `400 Bad Request`.

8. **Dev DB Setup / Migration Endpoints Hardening (`CF-10` / `F-11`):**
   - Preserved `SuperAdminUser` + dev-only `ENVIRONMENT` gating on `/setup-db`, `/run-migrations`, and `/init-db` in `api/src/routes/health.rs:355-600` (adding the missing `ENVIRONMENT` production block to `run_migrations`).
   - Updated `setup_db` inline DDL for `password_resets` to include `id BIGSERIAL PRIMARY KEY`.
   - Scrubbed raw database error strings `{e}` from `SetupResponse::message`, logging detailed errors server-side via `tracing::error!`.

---

### 2.2 Tier 2 — Backend 500s, Schema Alignment, Error Masking & RBAC Ordering

1. **Database Schema Alignment (`072_audit_followup_schema_alignment.sql`):**
   - Created `api/migrations/072_audit_followup_schema_alignment.sql` and synchronized `api/migrations/schema.sql`, `api/migrations/sqlx-seed.sql`, and `api/tests/g03_schema_baseline_test.rs` (`EXPECTED_TABLES = 221`):
     - `form_submissions`: Added `form_id BIGINT REFERENCES forms(id)`, `data JSONB`, `metadata JSONB`, `status VARCHAR(50)`, `ip_address VARCHAR(45)`, `user_agent TEXT`, `updated_at TIMESTAMPTZ`.
     - `contacts` & `crm_contacts`: Added missing CRM columns (`user_id`, `email`, `first_name`, `last_name`, `full_name`, `phone`, `company`, `job_title`, `lifecycle_stage`, `lead_status`, `lead_score`, `engagement_score`, `source`, `owner_id`, `custom_fields`, `tags`, `email_status`, `subscription_status`, `last_activity_at`, `deleted_at`, etc.).
     - `coupons`: Added `discount_value_cents BIGINT` (backfilled from `ROUND(discount_value * 100)::BIGINT`), `minimum_amount_cents BIGINT`, `maximum_discount_cents BIGINT`, `applicable_plans JSONB`, `applicable_products JSONB`, `excluded_products JSONB`, `first_time_only BOOLEAN`, `single_use_per_user BOOLEAN`, `metadata JSONB`.
     - `indicator_videos`, `trading_room_videos`, `weekly_watchlists`, `room_resources`, `page_layouts`, and CMS v2 tables (`cms_workflows`, `cms_ab_tests`, `cms_seo_audits`, `cms_datasources`, `cms_webhooks`, `cms_translation_jobs`): Aligned columns and types with SQLx queries.

2. **Elimination of Silent Error Masking (`CF-11` / `F-10`):**
   - Removed `.unwrap_or_default()` and `.unwrap_or(0)` on SQLx queries in `admin_members.rs`, `crm/mod.rs`, `analytics.rs`, `products.rs`, `coupons.rs`, and `cms_v2.rs`, propagating database errors via `?` so schema/query failures surface as `500 Internal Server Error` instead of `200 OK` with empty payloads.

3. **`404 Not Found` on Missing Resource `UPDATE` / `DELETE` (`CF-12`):**
   - Added `rows_affected() == 0` / `fetch_optional` checks returning `404 Not Found` across `admin_members.rs`, `courses.rs`, `products.rs`, `coupons.rs`, `redirects.rs`, and `email_templates.rs`.

4. **Mounted All 14 CRM Deals & Pipelines Handlers (`CF-05`):**
   - Mounted all 14 handlers in `api/src/routes/crm/deals_pipelines.rs` under `/api/admin/crm/pipelines*`, `/api/admin/crm/stages*`, and `/api/admin/crm/deals*` in `api/src/routes/crm/mod.rs` with `AdminUser` protection.

5. **`i64` Cents Standardization (`CF-17` / `F-19`):**
   - Updated `Product` and `CreateProductRequest`/`UpdateProductRequest` in `api/src/models/product.rs` and `api/src/routes/products.rs` so `price_cents: i64` is the primary monetary field while maintaining backwards-compatible deserialization for legacy `price` inputs.
   - Updated `Coupon` in `api/src/routes/coupons.rs` to expose `discount_value_cents: Option<i64>`.

6. **RBAC Extractor Ordering (`F-16`):**
   - Reordered handler parameters across `cms_v2.rs`, `cms_Page_builder.rs`, `cms_revisions.rs`, `cms_tags.rs`, `cms_upload.rs`, `cms_assets.rs`, `cms_author.rs`, `cms_categories.rs`, `cms_comments.rs`, `cms_content.rs`, `cms_forms.rs`, and `cms_menus.rs` so `AdminUser` / `CmsEditorUser` appears immediately after `State(state)` and before `Path` or `Json` extractors. Unauthenticated callers now consistently receive `401`/`403` instead of `400`/`422`.

---

### 2.3 Tier 3 — Frontend Crashes, Proxy Routing, `$NaN` Rendering & Native Dialogs

1. **Local API Origin & Proxy Routing (`CF-16` / `F-17`):**
   - Updated `frontend/src/lib/api/config.ts` to remove hardcoded `https://revolution-trading-pros-api.fly.dev` fallbacks, resolving to `VITE_API_URL` or `http://localhost:8080` in SSR dev and `''` (relative proxy) in the browser.
   - Updated `frontend/vite.config.ts` CSP `connect-src` in dev to permit `http://localhost:*`, `http://127.0.0.1:*`, `ws://localhost:*`, and `ws://127.0.0.1:*`.
   - Added `frontend/src/routes/api/admin/analytics/[...rest]/+server.ts` and updated `frontend/src/routes/api/[...path]/+server.ts` (`BACKEND_OWNED_PREFIXES`) so `/api/admin/analytics/*` forwards directly to the Rust backend with `Authorization` and `Cookie` headers preserved.

2. **Admin Page Crash & Blank Render Fixes (`CF-13` / `F-13` / `F-14` / `F-15`):**
   - Fixed `Input.svelte` and `Select.svelte` in `frontend/src/lib/components/ui/` to accept standard HTML attributes (`id`, `min`, `max`, `step`) via rest props and bind `<label for={id}>`.
   - Normalized `display_rules` defaults on `/admin/popups/new/+page.svelte` and `/admin/popups/[id]/edit/+page.svelte` so editing or creating popups never throws `TypeError: Cannot read properties of undefined`.
   - Normalized backend response shapes on `/admin/site-health/+page.svelte`, `/admin/settings/+page.svelte`, `/admin/media/+page.svelte`, and `/admin/media/_components/StatsPanel.svelte`.

3. **Monetary Formatting (`$NaN` Fixes — `CF-18` / `F-18`):**
   - Updated `/admin/orders/+page.svelte`, `/admin/memberships/+page.svelte`, `/admin/subscriptions/plans/+page.svelte`, `frontend/src/lib/remotes/orders.remote.ts`, and `frontend/src/lib/remotes/plans.remote.ts` to read `total_cents` / `price_cents` (`i64` cents) and format `(cents / 100)` cleanly without producing `$NaN`.

4. **Native Dialog Replacement & Edit Form Hydration (`CF-15`):**
   - Replaced all native browser `confirm()` and `alert()` calls in `/admin/courses/create/+page.svelte`, `/admin/courses/[id]/+page.svelte`, `/admin/courses/[id]/lessons/[lessonId]/+page.svelte`, and `/admin/members/service/[id]/+page.svelte` with inline confirmation modals and status banners.
   - Wired form hydration from `GET` endpoints on `/admin/products/[id]/edit/+page.svelte`, `/admin/coupons/edit/[id]/+page.svelte`, and `/admin/users/edit/[id]/+page.svelte`.

---

### 2.4 Tier 4 — Test Suite Integrity & Real-Router Wiring

1. **Non-Vacuous Rate Limiting Integration Test (`CF-21`):**
   - Updated `api/tests/common/mod.rs` (`setup_test_app`) to attach `axum::extract::connect_info::MockConnectInfo(SocketAddr)` with a unique test client IP per router instance so `SmartIpKeyExtractor` extracts a valid IP during `.oneshot()` calls without cross-test bucket collisions.
   - Updated `test_rate_limiting` in `api/tests/integration_tests.rs:314-356` to send repeated requests to `POST /api/auth/forgot-password` (governed by `auth_Strict_limiter`) and unconditionally assert `assert!(got_rate_limited, ...)`.

2. **Elimination of Shadow-Router Tests (`CF-22` / `F-20`):**
   - Rewrote `api/tests/orders_test.rs`, `api/tests/payments_test.rs`, `api/tests/admin_indicators_test.rs`, `api/tests/cms_v2_test.rs`, and `api/tests/oauth_test.rs` to remove hand-assembled shadow routers and instead dispatch requests through `common::setup_test_app().await` (the real `routes::create_router(state)` production router).

---

## 3. Regression & Integration Tests Added / Fixed

| Test Suite & Test Name | What It Proves | Pre-Fix Failure Observed |
|---|---|---|
| `tier1_followup_test::tier1_1_cms_v2_admin_gets_require_admin` | All 8 `/api/admin/cms-v2/*` GET endpoints return `401` (anon), `403` (`role="user"`), and `200` (`role="admin"`) | `assertion left == right failed: GET /api/admin/cms-v2/workflows as role=user must be 403, got 200` |
| `tier1_followup_test::tier1_2_admin_room_content_requires_admin` | `/api/admin/room-content/*` is mounted and write routes on both `/api/admin/room-content/*` and `/api/room-content/*` enforce `AdminUser` | `assertion left == right failed: POST /api/admin/room-content/alerts without token must be 401, got 404` |
| `tier1_followup_test::tier1_3_create_member_persists_user_and_activity_log` | `POST /api/admin/members` commits both the new `users` row and `user_activity_log` entry | `assertion left == right failed: user must persist in users table after create_member (left: 0, right: 1)` |
| `tier1_followup_test::tier1_4_refresh_token_embeds_jti_and_survives_same_second_rotation` | `create_refresh_jwt` embeds a unique UUID `jti` and same-second refresh rotation succeeds without `UNIQUE (token_hash)` collision | `Refresh Claims must embed a non-empty jti` |
| `tier1_followup_test::tier1_5_pagination_clamps_overflow_and_negative_params` | `page=i64::MAX`, `page=-5`, and `per_page=10000` clamp safely and return `200 OK` | `GET /api/admin/courses?page=4611686018427387905&per_page=20 panicked` |
| `tier1_followup_test::tier1_6_subscription_plan_rejects_negative_price_blank_name_and_409_on_duplicate` | `POST /api/subscriptions/plans` returns `422` on negative `price_cents` or blank `name` and `409` on duplicate `slug` | `Negative price_cents must be rejected with 400/422, got 200 OK` |
| `tier1_followup_test::tier1_7_close_trade_case_insensitive_direction_and_409_on_repeat_close` | Lowercase `"long"` computes positive PnL (`+1000` cents) and closing an already-closed trade returns `409 Conflict` | `assertion left == right failed: Lowercase 'long' direction must compute +1000 cents PnL, got Some(-1000)` |
| `tier1_followup_test::tier1_8_setup_db_uses_bigserial_password_resets_and_scrubs_errors` | `setup_db` DDL defines `id BIGSERIAL PRIMARY KEY` on `password_resets`, `run_migrations` blocks production, and SQL errors are scrubbed | `setup_db DDL must define id BIGSERIAL PRIMARY KEY on password_resets` |
| `integration_tests::test_rate_limiting` | Production rate limiter triggers `429 Too Many Requests` on `/api/auth/forgot-password` via `MockConnectInfo<SocketAddr>` | Previously skipped assertion when `got_rate_limited == false` |
| `orders_test`, `payments_test`, `admin_indicators_test`, `cms_v2_test`, `oauth_test` | 53 integration tests across all 7 test binaries execute against the real `setup_test_app()` router | Previously tested disconnected local shadow routers |

---

## 4. Final Gate Verification

All repository gate commands were executed and passed with exit code `0` and `0` warnings:

- `pnpm --filter revolution-svelte check` → **`svelte-check found 0 errors and 0 warnings`** (exit code `0`)
- `pnpm --filter revolution-svelte lint` → **`0` errors, `0` warnings** (exit code `0`)
- `pnpm --filter revolution-svelte test:unit` → **All 63 test files passed** (exit code `0`)
- `pnpm --filter revolution-svelte build` → **Production build & prerender succeeded** (exit code `0`)
- `cd api && cargo fmt --check && cargo clippy --all-targets -- -D warnings` → **`0` warnings, `0` errors** (exit code `0`)
- `cd api && cargo test` (across `tier1_followup_test`, `g03_schema_baseline_test`, `health_test`, `plans_cache_test`, `utils_test`, `crm_test`, `admin_members_test`, `forms_test`, `coupons_test`, `admin_indicators_test`, `admin_page_layouts_test`, `cms_v2_test`, `integration_tests`, `orders_test`, `payments_test`, `oauth_test`) → **All passed** (exit code `0`)

---

## 5. Deferred & Out-of-Scope Items

1. **Blog Editor (`CF-20` / `F-12` — Explicitly Out of Scope per `followup-prompt.md` §5):**
   - Per owner instructions, `frontend/src/lib/components/blog/BlockEditor/`, `frontend/src/routes/admin/blog/create`, `frontend/src/routes/admin/blog/edit/[id]`, and `ContentEditorPanel.svelte` were not modified during this pass.
2. **External Third-Party Integrations (`SKIPPED-EXTERNAL`):**
   - Live third-party calls requiring external credentials (Stripe live/test API & webhooks, Bunny.net video/storage, Postmark email delivery, Cloudflare R2 object storage, Google/Apple OAuth provider round-trips, Meilisearch live indexing, and Anthropic/Claude AI-assist calls) remain external dependencies; their local route guards, input validation, and clean error handling paths are verified.

