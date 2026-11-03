# Independent Verification Report — `report.md` & Commit `7f6a0f9df`

**Target Commit (`HEAD` / `origin/main`):** `7f6a0f9dfa4dbdd5e5675426b340674bffcd941c` ("fix(admin): resolve backend & frontend audit findings across all 4 tiers", 137 files changed, +4,644 / −2,700)  
**Pre-Fix Baseline Commit (`HEAD~1`):** `60288659d8d3540a527789d4bb2da4fc0eb17f6e`  
**Dropped Audit-Artifact Commit (in reflog `HEAD@{16}`):** `ca98481be`  
**Verification Mode:** Read-only repository inspection + throwaway PostgreSQL databases (`rtp_verify_db`, `rtp_verify_mig_db`, `rtp_live_before`, `rtp_live_after`) and Redis DBs (`9`, `10`, `11`). Zero tracked files modified.

---

## 1. Verdict per `report.md` Row (§1 Rows 1–16) & Claim-vs-Code Analysis (§1.5)

### 1.1 Row-by-Row Verdict Table (`report.md` §1)

| Row | Audit ID(s) | `report.md` Claim Summary | Verdict | Forensic Summary (`file:line` & Evidence) |
| :-: | :--- | :--- | :---: | :--- |
| **1** | `CF-01` / `F-01` | 8 GET handlers under `/api/admin/cms-v2/*` (`list_workflows`, `get_workflow`, `list_ab_tests`, `list_seo_audits`, `list_datasources`, `list_webhooks`, `list_translation_jobs`, `get_Single_type`) had `AdminUser` added | **`PARTIAL` / `MISLOCATED`** | None of the 8 handler names claimed in `report.md` §1 / §2.1.1 exist in `api/src/routes/cms_v2.rs` (`GET /api/admin/cms-v2/workflows` returns `404` both before and after). In reality, commit `7f6a0f9df` added `_admin: AdminUser` to **3** previously unauthenticated GET handlers in `api/src/routes/cms_v2.rs:561-566` (`get_site_settings`), `592-597` (`list_navigation_menus`), and `609-615` (`get_navigation_menu`), and replaced `user: User` with `CmsEditorUser`/`AdminUser` on 21 handlers (`cms_v2.rs:78-719`). Test `tier1_1_cms_v2_admin_gets_require_admin` (`api/tests/tier1_followup_test.rs:33-71`) only tests those 3 paths (`settings`, `menus`, `menus/{id}`) with anon (`401`) and member (`403`) tokens; it does not test 8 routes or assert `200` for admin. |
| **2** | `CF-02` / `F-02` | Mounted `room_content::admin_router()` in `api/src/routes/mod.rs:228-229` and added `AdminUser` to all write handlers in `api/src/routes/room_content.rs:150-926` | **`PARTIAL` / `MISLOCATED`** | `api/src/routes/room_content.rs` does not exist (`api/src/routes/room_content/` is a directory), and `api/src/routes/mod.rs` was **not modified** in `7f6a0f9df` (`room_content::admin_router()` was already mounted at `api/src/routes/mod.rs:171`). Write handlers already required `AdminUser`; `7f6a0f9df` added `AdminUser` wrapper functions to the **6 GET handlers** on `admin_router()` in `api/src/routes/room_content/mod.rs:477-576`. Furthermore, `report.md:144` fabricates a pre-fix failure (`POST /api/admin/room-content/alerts ... got 404`) that `tier1_2` (`api/tests/tier1_followup_test.rs:76-121`) never tests. |
| **3** | `CF-03` / `F-03` | Fixed `create_member` (`POST /api/admin/members`) in `api/src/routes/admin_members.rs:1401-1432` to insert `user_activity_log` on `&mut *tx` and call `tx.commit().await` before returning `201 Created` | **`MISLOCATED`** | `api/src/routes/admin_members.rs` does not exist, and `POST /api/admin/members` returns `405 Method Not Allowed`. The actual route is `POST /api/admin/member-management` returning `200 OK`, and the real fix (`&mut *tx` + `tx.commit().await`) is at `api/src/routes/admin_member_management/crud.rs:411-432` (tested at `api/tests/tier1_followup_test.rs:126-169`). |
| **4** | `F-21` | Added `jti: Option<String>` in `api/src/utils/jwt.rs:16-142` and `ON CONFLICT (token_hash) DO NOTHING` on `user_sessions` in `api/src/routes/auth.rs:656-660` | **`PARTIAL` / `MISLOCATED`** | Neither `api/src/utils/jwt.rs` nor `api/src/routes/auth.rs` exists, and no file in `api/src/routes/auth/` was touched in `7f6a0f9df`. `ON CONFLICT (token_hash) DO NOTHING` does not exist anywhere in `api/src/`. The real fix added non-optional `#[serde(default)] pub jti: String` at `api/src/utils/mod.rs:69-70` and `jti: uuid::Uuid::new_v4().to_string()` at `api/src/utils/mod.rs:325, 363`, which prevents same-second SHA-256 token hash collisions in Redis blacklist rotation (`api/src/routes/auth/refresh.rs:43-176`). |
| **5** | `F-04` | Created `normalize_pagination` in `api/src/utils/pagination.rs:1-21` (`page: 1..=100_000`, `per_page: 1..=100`) and applied it across all paginated routes | **`PARTIAL` / `MISLOCATED`** | `api/src/utils/pagination.rs` and `normalize_pagination` do not exist (`0` matches in repo). Instead, ad-hoc inline `.clamp(1, 10_000)` / `.max(1)` and `.saturating_mul(per_page)` were added across 26 route files. `GET /api/admin/members` (`api/src/routes/members.rs:88-100`) was missed and still returns `500` on `?per_page=-5` and `?search=%00`. |
| **6** | `F-08` | `POST /api/subscriptions/plans` (`api/src/routes/subscriptions.rs:163-234`) rejects `price_cents < 0` and blank `name` with `422` and maps SQLSTATE `23505` to `409 Conflict` | **`MISLOCATED`** | `api/src/routes/subscriptions.rs` does not exist, and `POST /api/subscriptions/plans` returns `405 Method Not Allowed`. The real endpoint is `POST /api/admin/subscriptions/plans` in `api/src/routes/subscriptions_admin/plans.rs:133-185` (and `update_plan` at `lines 214-229`), where the validation and `23505` → `409 Conflict` mapping are confirmed. |
| **7** | `F-09` | `POST /api/room-content/trades/:id/close` (`api/src/routes/room_content.rs:636-702`) uses case-insensitive `"LONG"` / `"SHORT"`, rejects unknown direction with `400 Bad Request`, and checks `status == "closed"` / `exit_price.is_some()` → `409` | **`PARTIAL` / `MISLOCATED`** | Real handler is `PUT /api/admin/room-content/trades/{id}/close` in `api/src/routes/room_content/trades.rs:314-396`. It checks `existing.status != "open"` (`lines 331-336`) and `WHERE id = $1 AND status = 'open' AND deleted_at IS NULL` (`line 367`) → `409`, and uses `existing.direction.eq_ignore_ascii_case("long")` (`line 339`). However, it is a **read-then-write** (not a single atomic `UPDATE`), never checks `exit_price.is_some()`, and **does not validate `"short"` or return `400`** on unknown direction (`else` branch at `line 341` treats any non-`"long"` string as short). |
| **8** | `CF-10` / `F-11` | `api/src/routes/health.rs:355-600`: `setup_db` defines `id BIGSERIAL PRIMARY KEY` on `password_resets`, `run_migrations` has dev-only `ENVIRONMENT` guard, and raw SQL errors are scrubbed | **`CONFIRMED` (`MISLOCATED` lines)** | `api/src/routes/health.rs` has 394 lines total; the changes are confirmed at `api/src/routes/health.rs:196-201, 215, 248, 272, 304-313, 329` and tested at `api/tests/tier1_followup_test.rs:512-522`. (Note: the owner never answered the audit question on whether `/setup-db` should be removed or gated under `/api/admin/*`; `7f6a0f9df` unilaterally kept it mounted at root.) |
| **9** | `CF-04..09` / `F-05..07` | `072_audit_followup_schema_alignment.sql` aligns `form_submissions`, `contacts`, `crm_contacts`, `coupons` (`discount_value_cents` backfill), `indicator_videos`, `trading_room_videos`, `weekly_watchlists`, `room_resources`, `page_layouts`, and CMS v2 tables | **`PARTIAL`** | `api/migrations/072_audit_followup_schema_alignment.sql:1-147` only creates 5 tables (`user_course_enrollments_enhanced`, `organization_profile`, `department_members`, `team_members`, `cms_asset_usage`) and alters 10 tables (`courses`, `course_modules_v2`, `lessons`, `user_course_enrollments`, `unified_videos`, `member_filters`, `newsletter_subscribers`, `categories`, `indicator_files`, `indicator_videos`). It does **not** touch `form_submissions`, `contacts`, `crm_contacts`, `coupons` (`discount_value_cents` does not exist in `schema.sql`), `trading_room_videos`, `weekly_watchlists`, `room_resources`, `page_layouts`, or CMS v2 tables (those were adjusted in Rust SQL queries instead). Also, `072` was copy-pasted raw onto the bottom of `api/migrations/schema.sql:21926-22070`. |
| **10** | `CF-11` / `F-10` | Eliminated silent error masking (`unwrap_or_default()` / `unwrap_or(0)`) in `admin_members.rs`, `crm/mod.rs`, `analytics.rs`, `products.rs`, `coupons.rs`, and `cms_v2.rs` | **`PARTIAL` / `MISLOCATED`** | `api/src/routes/analytics.rs` was **not modified at all** in `7f6a0f9df`; `api/src/routes/products.rs` had zero `unwrap_or` changes; `api/src/routes/crm/mod.rs` still has **7** `.await.unwrap_or((0,))` queries at lines `63, 71, 79, 85, 92, 98, 104` (where line `104` queries non-existent `crm_campaigns.emails_sent` and silently masks the SQL error to `0`). Across `api/src/routes/`, **137 SQL queries still use `.await.unwrap_or(...)` and 45 use `.await.ok()`**. |
| **11** | `CF-12` | Added `rows_affected() == 0` → `404` across `admin_members.rs`, `courses.rs`, `products.rs`, `coupons.rs`, `redirects.rs`, and `email_templates.rs` | **`PARTIAL` / `MISLOCATED`** | `courses.rs` does not exist, `redirects.rs` was not modified in `7f6a0f9df`, and `email_templates.rs` had no `rows_affected()` change. Real `rows_affected() == 0` → `404` checks were added in `admin/coupons.rs:283-299`, `coupons.rs:663-668`, `products.rs:349-365`, `contacts.rs:266-305`, `admin/campaigns_stats.rs:111-116`, `crm/*.rs`, `admin_indicators/*.rs`, and `admin_members/{filters,segments,tags}.rs`. Soft-delete `POST /api/admin/products/{id}/archive` (`products.rs:443-462`) and `DELETE /api/admin/room-content/alerts/{id}` (`room_content/alerts.rs:285-315`) still return `200` on repeat calls. |
| **12** | `CF-13` / `F-13` / `F-14` | Fixed crashes/blank renders on `/admin/popups/new`, `/admin/popups/[id]/edit`, `/admin/site-health`, `/admin/settings`, and `/admin/media` | **`PARTIAL`** | None of the 5 pages throw uncaught `TypeError`s in Playwright, **but `/admin/popups/1/edit` still renders blank form fields (`name`, `title`, `content`, `cta_text`, `cta_url`)** because `popupsApi.get(id)` (`frontend/src/lib/api/popups.ts:1739`) reads from an empty client-side Svelte store `get(popups)` instead of fetching `/api/admin/popups/${id}`! Also, `Input.svelte:23` only added `value = $bindable()`, not the `id`/`min`/`max`/`step` or `<label for={id}>` changes claimed in `report.md` §2.3.2. |
| **13** | `CF-16` / `F-17` | Removed hardcoded `fly.dev` fallback in `config.ts`, updated `vite.config.ts` CSP, and updated `api/[...path]/+server.ts` (`BACKEND_OWNED_PREFIXES`) + `api/admin/analytics/[...rest]/+server.ts` | **`PARTIAL` / `MISLOCATED`** | `config.ts` was modified at lines `316-335` (not `10-35`), did **not** remove any `fly.dev` URL in `7f6a0f9df`, and still sets `API_BASE_URL = 'http://localhost:8080'` (`config.ts:12-17`), sending all non-admin browser `/api/*` requests cross-origin (`config.ts:330-331`). `api/[...path]/+server.ts:71` only removed `500` from `RETRYABLE_STATUS_CODES` (`BACKEND_OWNED_PREFIXES` does not exist). `api/admin/analytics/[...rest]/+server.ts:62` was modified (1-line path rewrite to `/api/analytics/${rest}`), not newly created. |
| **14** | `CF-18` / `F-18` | Fixed `$NaN` rendering on `/admin/orders`, `/admin/memberships`, and `/admin/subscriptions/plans` (citing `frontend/src/lib/remotes/{plans,orders}.remote.ts`) | **`MISLOCATED` (`CONFIRMED` behavior)** | `frontend/src/lib/remotes/plans.remote.ts` and `frontend/src/lib/remotes/orders.remote.ts` do not exist; the real files are `frontend/src/routes/admin/subscriptions/plans/plans.remote.ts:44-71` and `frontend/src/routes/admin/orders/orders.remote.ts:48-96`. Browser pass confirms zero `NaN` text across `/admin/orders`, `/admin/memberships`, and `/admin/subscriptions/plans`. |
| **15** | `CF-21` | Attached `MockConnectInfo<SocketAddr>` in `api/tests/common/mod.rs:131-162` and made `test_rate_limiting` (`api/tests/integration_tests.rs:314-356`) unconditionally assert `429` | **`PARTIAL` / `MISLOCATED`** | Real lines are `api/tests/common/mod.rs:105-115` and `api/tests/integration_tests.rs:314-345`. `test_rate_limiting` tests `POST /api/auth/login` (not `/api/auth/forgot-password` as claimed in `report.md` Executive Summary & §2.4.1), uses a single hardcoded IP `127.0.0.1:40000` (not unique per router instance), and cites non-existent identifiers `SmartIpKeyExtractor` and `auth_Strict_limiter`. |
| **16** | `CF-22` / `F-20` | Rewired 5 shadow-router test files (`orders_test.rs`, `payments_test.rs`, `admin_indicators_test.rs`, `cms_v2_test.rs`, `oauth_test.rs`) to `common::setup_test_app().await` | **`PARTIAL`** | All 5 files (`10` async router tests + `21` sync unit tests = `31` tests, not `53` or `29` as claimed in `report.md:152, 133`) were rewired to `common::setup_test_app().await`. However, `cms_v2_test.rs:344-348` (`public_router_is_get_only_and_rejects_mutations`) **removed** `assert_ne!(resp.status(), StatusCode::NOT_FOUND)`, weakening the test so unmounted public GET routes returning `404` now pass undetected. |

---

### 1.2 Specific Claim-vs-Code Verification (`counter-prompt.md` §1.5)

1. **Trade Close (`api/src/routes/room_content/trades.rs:314-396`):**
   - **Read-then-write, NOT a single atomic `UPDATE`:**
     - Line `315`: Executes `SELECT * FROM room_trades WHERE id = $1 AND deleted_at IS NULL` via `.fetch_one(&state.db.pool).await`.
     - Lines `331-336`: Checks `if existing.status != "open"` in Rust and returns `409 Conflict`.
     - Lines `339-352`: Computes `price_delta`, `pnl`, `pnl_percent`, `holding_days`, and `result` in Rust from the `SELECT` snapshot:
       ```rust
       let price_delta = if existing.direction.eq_ignore_ascii_case("long") {
           input.exit_price - existing.entry_price
       } else {
           existing.entry_price - input.exit_price
       };
       ```
     - Lines `354-395`: Executes `UPDATE room_trades SET status = 'closed', ... WHERE id = $1 AND status = 'open' AND deleted_at IS NULL RETURNING ...` via `.fetch_optional(&state.db.pool).await` and returns `409 Conflict` if `None`.
   - **False Claims & Remaining TOCTOU Window:**
     - `report.md` (§1 row 7 & §2.1.7) claims `close_trade` matches `"SHORT"` case-insensitively, rejects unknown direction values with `400 Bad Request`, and checks `exit_price.is_some()`. **None of those exist in the code**: any non-`"long"` string falls into the `else` branch and is calculated as a short trade.
     - While `WHERE status = 'open'` prevents two concurrent `close_trade` calls from both succeeding, the separate `SELECT` (`line 315`) and `UPDATE` (`line 354`) outside a transaction leave a TOCTOU window if `update_trade` (`trades.rs:193-307`) modifies `entry_price`, `quantity`, or `direction` between `line 315` and `line 354`.

2. **Refresh Token `jti` (`api/src/utils/mod.rs:69-70`, `api/src/routes/auth/refresh.rs:25-186`, `api/tests/tier1_followup_test.rs:174-255`):**
   - `Claims.jti` at `api/src/utils/mod.rs:69-70` is `#[serde(default)] pub jti: String` (non-optional), not `Option<String>` in `api/src/utils/jwt.rs` as claimed in `report.md:59`.
   - `refresh()` (`api/src/routes/auth/refresh.rs:43-176`, untouched in `7f6a0f9df`) hashes the raw refresh JWT via `hash_token_for_blacklist(&input.refresh_token)` (`api/src/routes/auth/helpers.rs:255-260`), checks `redis.is_token_blacklisted(&token_hash)`, mints a new refresh token via `create_refresh_token_versioned` (`api/src/utils/mod.rs:354-370`, which now sets `jti: uuid::Uuid::new_v4().to_string()`), and blacklists the old `token_hash` in Redis (`refresh.rs:166`).
   - Because each minted token now has a unique UUID v4 `jti`, two tokens minted in the same integer second produce distinct JWT strings and distinct SHA-256 hashes, preventing the newly minted token from self-blacklisting.
   - `tier1_4_refresh_token_embeds_jti_and_survives_same_second_rotation` (`api/tests/tier1_followup_test.rs:174-255`) genuinely performs `login` → `refresh(r1) -> r2` → `refresh(r2)` within one second and asserts `r1 != r2` (`line 225`) and `status2 == StatusCode::OK` (`line 250`). (However, `report.md:146`'s claimed pre-fix assertion string `"Refresh Claims must embed a non-empty jti"` does not exist in the test.)

3. **`tier1_5` Pagination Test (`api/tests/tier1_followup_test.rs:260-287`):**
   - Only tests 4 URLs (`lines 263-268`):
     - `/api/admin/courses?page=999999999&per_page=100`
     - `/api/admin/indicators?page=999999999&per_page=100`
     - `/api/admin/popups?page=999999999&per_page=100`
     - `/api/admin/popups?page=-1&per_page=-5`
   - Does **not** test `page=-1&per_page=-5` on `/api/admin/courses` or `/api/admin/indicators`, and does **not** test `search=%00` on any endpoint.
   - Furthermore, `GET /api/admin/members` (`api/src/routes/members.rs:88-100`) was missed in `7f6a0f9df`: passing `?per_page=-5` causes `LIMIT -5` → `500 Internal Server Error`, and passing `?search=%00` causes PostgreSQL invalid UTF-8 `0x00` → `500 Internal Server Error`.

4. **Migration `072_audit_followup_schema_alignment.sql` & `api/migrations/schema.sql`:**
   - **Forward-only & idempotent:** Confirmed. All 5 `CREATE TABLE` statements (`lines 4-70`) use `IF NOT EXISTS` and all 52 `ALTER TABLE` statements (`lines 72-147`) use `ADD COLUMN IF NOT EXISTS`. Running `072_audit_followup_schema_alignment.sql` twice on top of `schema.sql` succeeded with `0` errors (`MIG072_RUN1_OK=0`, `MIG072_RUN2_OK=0`).
   - **Why `target_audience` appears at `schema.sql:21885` AND `schema.sql:22003`:**
     - `api/migrations/schema.sql:21885` is `target_audience jsonb` inside `CREATE TABLE IF NOT EXISTS public.campaigns` (`lines 21875-21896`).
     - `api/migrations/schema.sql:22003` is `ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS target_audience JSONB;` because the author copy-pasted the entire 145-line body of `072_audit_followup_schema_alignment.sql` verbatim onto the end of `api/migrations/schema.sql` (`lines 21926-22070`).
     - Out of the 15 `ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS ...` statements appended at `schema.sql:21994-22008`, **12 columns already existed** in `CREATE TABLE public.courses` (`schema.sql:5880-5918`). Worse, `courses.avg_rating` at `schema.sql:5912` is `numeric(3,2) DEFAULT 0.00`, so `ADD COLUMN IF NOT EXISTS avg_rating DOUBLE PRECISION` at `schema.sql:22005` / `072:83` is a **silent no-op** that leaves `avg_rating` as `numeric(3,2)`.
   - **Claimed `coupons.discount_value_cents` backfill:** Fabricated (`NO-SUCH-CHANGE`). `discount_value_cents` does not exist in `072` or `schema.sql` (`coupons` at `schema.sql:5362-5382` still has `discount_value numeric(10,2) NOT NULL`); `api/src/routes/coupons.rs` computes `(discount_value * 100)::BIGINT AS value_cents` in SQL `SELECT` projections.

5. **Shadow-Router Tests (`orders_test.rs`, `payments_test.rs`, `admin_indicators_test.rs`, `cms_v2_test.rs`, `oauth_test.rs`):**
   - Total diff across the 5 files: `+216 / -519` lines. The `-217` lines in `cms_v2_test.rs` (`340` lines changed) came from deleting the 155-line `mod probe` shadow router (`lines 58-213` in `60288659d`).
   - Total test functions kept: `31` before → `31` after (`10` async `#[tokio::test]` router tests + `21` sync `#[test]` unit tests):
     - `orders_test.rs`: `6` → `6` (`2` async router tests strengthened from `!= 404` to `== 401`).
     - `payments_test.rs`: `6` → `6` (`1` async router test checking `!= 404` and `!= 405`).
     - `admin_indicators_test.rs`: `6` → `6` (`2` async router tests strengthened from `!= 404` to `== 401` / `== 405`).
     - `cms_v2_test.rs`: `6` → `6` (`4` async router tests; the 3 admin router tests were strengthened to `== 401`, **but** `public_router_is_get_only_and_rejects_mutations` at `cms_v2_test.rs:344-348` dropped `assert_ne!(resp.status(), StatusCode::NOT_FOUND)`).
     - `oauth_test.rs`: `7` → `7` (`1` async router test checking `!= 404` and `!= 405`).

6. **`MockConnectInfo` in `api/tests/common/mod.rs:105-115` vs Production IP Resolution:**
   - Before `7f6a0f9df`, `tower::ServiceExt::oneshot` bypassed `into_make_service_with_connect_info::<SocketAddr>()` (`api/src/main.rs:328`), causing `ClientAddr` (`api/src/routes/auth/helpers.rs:29-42`) to fail with `500 Internal Server Error` on every `POST /api/auth/login` call in tests.
   - Attaching `MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000)))` in `api/tests/common/mod.rs:105-115` allows `ClientAddr` to extract `127.0.0.1:40000` and proves that 6 consecutive `POST /api/auth/login` requests trip Redis `check_rate_limit("rl:auth_login_ip:127.0.0.1", 5, 300)` (`429 Too Many Requests` at `api/tests/integration_tests.rs:314-345`).
   - **What it masks / does not prove:** It bypasses TCP `into_make_service_with_connect_info`, shares a single static loopback IP (`127.0.0.1:40000`, where `is_trusted_proxy` is always `true`), and never exercises `resolve_client_ip` (`api/src/routes/auth/helpers.rs:59-93`) with `trust_proxy_headers: true` or proxy headers (`Fly-Client-IP`, `X-Forwarded-For`, `X-Real-IP`).

---

## 2. Corrected Locator Table (§1.2)

### 2.1 Non-Existent Paths & Identifiers Cited in `report.md`

- **6 Primary Non-Existent Paths:** `api/src/utils/pagination.rs`, `api/src/utils/jwt.rs`, `api/src/routes/room_content.rs`, `api/src/routes/auth.rs`, `api/src/routes/admin_members.rs`, `api/src/routes/subscriptions.rs`.
- **11 Non-Existent CMS Paths (`report.md:103`):** `api/src/routes/cms_Page_builder.rs`, `api/src/routes/cms_revisions.rs`, `api/src/routes/cms_tags.rs`, `api/src/routes/cms_upload.rs`, `api/src/routes/cms_assets.rs`, `api/src/routes/cms_author.rs`, `api/src/routes/cms_categories.rs`, `api/src/routes/cms_comments.rs`, `api/src/routes/cms_content.rs`, `api/src/routes/cms_forms.rs`, `api/src/routes/cms_menus.rs`.
- **7 Additional Non-Existent Backend Route Files:** `api/src/routes/courses.rs`, `api/src/routes/orders.rs`, `api/src/routes/user.rs`, `api/src/routes/admin_indicators.rs`, `api/src/routes/admin_videos.rs`, `api/src/routes/admin_watchlists.rs`, `api/src/routes/room_resources.rs`.
- **2 Non-Existent Frontend Remote Files (`report.md:37, 120`):** `frontend/src/lib/remotes/plans.remote.ts`, `frontend/src/lib/remotes/orders.remote.ts`.
- **Non-Existent Symbols (`0` matches in repo):** `normalize_pagination`, `ON CONFLICT (token_hash) DO NOTHING`, `SmartIpKeyExtractor`, `auth_Strict_limiter`, `BACKEND_OWNED_PREFIXES`.
- **SvelteKit Version Error (`report.md:11`):** Claims `"SvelteKit 2 + Svelte 5"`; `frontend/package.json:88` is `"@sveltejs/kit": "^3.1.0"`.

### 2.2 Complete Corrected Locator Table for `report.md` §1 & §2

| `report.md` Ref | Cited Locator / Claim in `report.md` | Real `file:line` in Commit `7f6a0f9df` (or `NO-SUCH-CHANGE`) |
| :--- | :--- | :--- |
| **Row 1 / §2.1.1** | `api/src/routes/cms_v2.rs:118-625` (8 GET handlers: `list_workflows`, `get_workflow`, `list_ab_tests`, `list_seo_audits`, `list_datasources`, `list_webhooks`, `list_translation_jobs`, `get_single_type`) | **8 named GET handlers:** `NO-SUCH-CHANGE`.<br>**Real `AdminUser` GET additions:** `api/src/routes/cms_v2.rs:561-566` (`get_site_settings`), `592-597` (`list_navigation_menus`), `609-615` (`get_navigation_menu`), `872-881` (`public_router`).<br>**`CmsEditorUser`/`AdminUser` replacements:** `api/src/routes/cms_v2.rs:78-719`.<br>**Test:** `api/tests/tier1_followup_test.rs:33-71`. |
| **Row 2 / §2.1.2** | `api/src/routes/mod.rs:228-229` & `api/src/routes/room_content.rs:150-926` | **`api/src/routes/mod.rs`:** `NO-SUCH-CHANGE` (already mounted at `api/src/routes/mod.rs:171`).<br>**Write handlers RBAC:** `NO-SUCH-CHANGE` (already had `AdminUser`).<br>**Real change:** `api/src/routes/room_content/mod.rs:477-576` (6 `AdminUser` GET wrappers on `admin_router`).<br>**Test:** `api/tests/tier1_followup_test.rs:76-121`. |
| **Row 3 / §2.1.3** | `api/src/routes/admin_members.rs:1401-1432` (`POST /api/admin/members`) | `api/src/routes/admin_member_management/crud.rs:411-432` (`create_member` on `POST /api/admin/member-management`).<br>**Test:** `api/tests/tier1_followup_test.rs:126-169`. |
| **Row 4 / §2.1.4** | `api/src/utils/jwt.rs:16-142` (`jti: Option<String>`) & `api/src/routes/auth.rs:656-660` (`ON CONFLICT (token_hash) DO NOTHING`) | `api/src/utils/mod.rs:69-70` (`pub jti: String`), `325` (`create_jwt_versioned`), `363` (`create_refresh_token_versioned`).<br>**`api/src/routes/auth.rs` / `ON CONFLICT`:** `NO-SUCH-CHANGE`.<br>**Test:** `api/tests/tier1_followup_test.rs:174-255`. |
| **Row 5 / §2.1.5** | `api/src/utils/pagination.rs:1-21` (`normalize_pagination`) + route call-sites | **`api/src/utils/pagination.rs` / `normalize_pagination`:** `NO-SUCH-CHANGE`.<br>**Real inline pagination locators (26 files):** `api/src/routes/admin/abandoned_carts.rs:123-128`, `api/src/routes/admin_courses/crud.rs:30-33`, `api/src/routes/admin_indicators/crud.rs:28-32, 54-59`, `api/src/routes/admin_member_management/activity.rs:25-27`, `api/src/routes/admin_members/filters.rs:62-69`, `api/src/routes/admin_members/segments.rs:62-69`, `api/src/routes/admin_members/tags.rs:70-77`, `api/src/routes/admin_page_layouts.rs:34-38`, `api/src/routes/admin_popups.rs:47-49`, `api/src/routes/admin_videos/crud.rs:29-31, 46`, `api/src/routes/categories.rs:118-120`, `api/src/routes/contacts.rs:79-83`, `api/src/routes/coupons.rs:360-362`, `api/src/routes/courses_admin/enrollments.rs:97-103`, `api/src/routes/crm/contacts.rs:84-91`, `api/src/routes/crm/deals_pipelines.rs:543-545`, `api/src/routes/crm/leads.rs:87-89`, `api/src/routes/email_templates.rs:79-81`, `api/src/routes/forms/crud.rs:23-25`, `api/src/routes/forms/public.rs:24-26`, `api/src/routes/forms/submissions.rs:28-30`, `api/src/routes/posts.rs:214-216, 987-989`, `api/src/routes/products.rs:57-59, 532-534`, `api/src/routes/subscriptions_admin/plans.rs:30-32`, `api/src/routes/trading_rooms/admin_rooms.rs:291-293`, `api/src/routes/trading_rooms/admin_videos.rs:45-47, 68, 129-131`.<br>**Test:** `api/tests/tier1_followup_test.rs:260-287`. |
| **Row 6 / §2.1.6** | `api/src/routes/subscriptions.rs:163-234` (`POST /api/subscriptions/plans`) | `api/src/routes/subscriptions_admin/plans.rs:133-153` (`create_plan` validation), `178-185` (`23505` → `409`), `214-229` (`update_plan` validation).<br>**Test:** `api/tests/tier1_followup_test.rs:292-407`. |
| **Row 7 / §2.1.7** | `api/src/routes/room_content.rs:636-702` (`POST /api/room-content/trades/:id/close`) | `api/src/routes/room_content/trades.rs:331-395` (`PUT /api/admin/room-content/trades/{id}/close`).<br>**`exit_price.is_some()` & `"SHORT"` / `400 Bad Request` on invalid direction:** `NO-SUCH-CHANGE`.<br>**Test:** `api/tests/tier1_followup_test.rs:413-507`. |
| **Row 8 / §2.1.8** | `api/src/routes/health.rs:355-600` | `api/src/routes/health.rs:196-201, 215, 248, 272, 304-313, 329`.<br>**Test:** `api/tests/tier1_followup_test.rs:512-522`. |
| **Row 9 / §2.2.1** | `api/migrations/072_audit_followup_schema_alignment.sql`, `schema.sql`, `sqlx-seed.sql`, `g03_schema_baseline_test.rs` | `api/migrations/072_audit_followup_schema_alignment.sql:1-147`, `api/migrations/schema.sql:21926-22070`, `api/migrations/sqlx-seed.sql:201-203`, `api/tests/g03_schema_baseline_test.rs:41, 141, 206`.<br>**DDL in `072` for `form_submissions`, `contacts`, `crm_contacts`, `coupons` (`discount_value_cents`), `trading_room_videos`, `weekly_watchlists`, `room_resources`, `page_layouts`, `cms_*`:** `NO-SUCH-CHANGE` (handled via SQL query edits in `api/src/routes/forms/*.rs`, `coupons.rs`, `admin_page_layouts.rs`, `trading_rooms/admin_videos.rs`, `room_resources/public.rs`, `crm/*.rs`). |
| **Row 10 / §2.2.2** | `admin_members.rs:371-2465`, `crm/mod.rs:228-884`, `analytics.rs:422-1324`, `products.rs:146-152`, `coupons.rs:117-123`, `cms_v2.rs` | **`analytics.rs` & `products.rs`:** `NO-SUCH-CHANGE`.<br>**Real locators:** `api/src/routes/coupons.rs:426-432, 475-481`, `api/src/routes/cms_v2.rs:655-686`, `api/src/routes/crm/mod.rs:112`, `api/src/routes/crm/{automations,companies,contacts,deals_pipelines,leads,lists_tags,segments,sequences_campaigns,smart_links,templates,webhooks}.rs`, `api/src/routes/admin_members/{analytics,filters,notes_emails,segments,tags}.rs`, `api/src/routes/admin_member_management/{activity,crud}.rs`, `api/src/routes/admin_courses/{analytics_pricing,enrollments}.rs`, `api/src/routes/courses_admin/{enrollments,utility}.rs`, `api/src/routes/forms/{analytics,crud,public,submissions}.rs`, `api/src/routes/email_templates.rs:119`, `api/src/routes/posts.rs:373, 775`. |
| **Row 11 / §2.2.3** | `admin_members.rs`, `courses.rs`, `products.rs`, `coupons.rs`, `redirects.rs`, `email_templates.rs` | **`courses.rs`, `redirects.rs`, `email_templates.rs`:** `NO-SUCH-CHANGE`.<br>**Real locators:** `api/src/routes/admin/coupons.rs:283-299`, `api/src/routes/coupons.rs:663-668`, `api/src/routes/products.rs:349-365`, `api/src/routes/contacts.rs:266-305`, `api/src/routes/admin/campaigns_stats.rs:111-116`, `api/src/routes/crm/{deals_pipelines,companies,automations,lists_tags,segments,sequences_campaigns,smart_links,templates,webhooks}.rs`, `api/src/routes/admin_indicators/{crud,files,videos}.rs`, `api/src/routes/admin_members/{filters,segments,tags}.rs`. |
| **§2.2.4** | Mounted 14 CRM Deals & Pipelines handlers in `api/src/routes/crm/mod.rs` | **`api/src/routes/crm/mod.rs`:** `NO-SUCH-CHANGE` (already merged `deals_pipelines::router()`).<br>**Real locator:** `api/src/routes/crm/deals_pipelines.rs:1233-1261` (mounted 15 handlers across 9 `.route(...)` paths). |
| **§2.2.5** | `api/src/models/product.rs`, `api/src/routes/products.rs`, `api/src/routes/coupons.rs`, `api/src/routes/subscriptions.rs` | **`api/src/models/product.rs` & `subscriptions`:** `NO-SUCH-CHANGE`.<br>**Real locators:** `api/src/routes/products.rs:377-381`, `api/src/routes/coupons.rs:198, 206, 228, 305, 330, 395, 434, 463`, `api/src/routes/admin/abandoned_carts.rs:30-56, 86-190, 294`, `api/src/routes/crm/{deals_pipelines,sequences_campaigns,leads,contacts,companies}.rs`. |
| **§2.2.6** | Reordered `AdminUser` / `CmsEditorUser` across 11 `cms_*.rs` files | **11 cited `cms_*.rs` file paths:** `NO-SUCH-CHANGE` (files do not exist).<br>**Real locators:** `api/src/middleware/admin.rs:109-144`, `api/src/routes/cms_v2.rs:158-735`, `api/src/routes/cms_v2_enterprise.rs:109-301`, `api/src/routes/cms_ai_assist.rs:475-693`, `api/src/routes/cms_assets/usage.rs:28, 129`, `api/src/routes/cms_datasources/{datasources,entries,import_export}.rs`, `api/src/routes/cms_global_components/{crud,sync,usage,versions}.rs`, `api/src/routes/cms_presets/{actions,crud,listing,retrieval}.rs`, `api/src/routes/cms_reusable_blocks/{crud,usage}.rs`, `api/src/routes/cms_revisions/handlers.rs:22-216`, `api/src/routes/cms_scheduling/{calendar_history,cron,releases,schedules}.rs`, `api/src/routes/cms_seo/handlers.rs:26, 70`. |
| **Row 12 / §2.3.2** | `Input.svelte`, `Select.svelte`, `popups/new`, `popups/[id]/edit`, `site-health`, `settings`, `media`, `StatsPanel.svelte` | `frontend/src/lib/components/ui/Input.svelte:23`, `frontend/src/lib/components/ui/Select.svelte:18`, `frontend/src/routes/admin/popups/new/+page.svelte:38-41`, `frontend/src/routes/admin/popups/[id]/edit/+page.svelte:28-30, 111-121`, `frontend/src/routes/admin/site-health/+page.svelte:164-187`, `frontend/src/routes/admin/settings/+page.svelte:931-938`, `frontend/src/routes/admin/media/+page.svelte:200-204`, `frontend/src/routes/admin/media/_components/StatsPanel.svelte:32-37, 47, 57, 67, 91`. |
| **Row 13 / §2.3.1** | `frontend/src/lib/api/config.ts:10-35`, `frontend/vite.config.ts:108-120`, `frontend/src/routes/api/[...path]/+server.ts`, `frontend/src/routes/api/admin/analytics/[...rest]/+server.ts` | `frontend/src/lib/api/config.ts:316-335`, `frontend/vite.config.ts:132-142`, `frontend/src/routes/api/[...path]/+server.ts:71`, `frontend/src/routes/api/admin/analytics/[...rest]/+server.ts:62`. |
| **Row 14 / §2.3.3** | `orders/+page.svelte`, `memberships/+page.svelte`, `subscriptions/plans/+page.svelte`, `frontend/src/lib/remotes/{plans,orders}.remote.ts` | `frontend/src/routes/admin/orders/+page.svelte:94-100`, `frontend/src/routes/admin/orders/orders.remote.ts:48-96`, `frontend/src/routes/admin/memberships/+page.svelte:222-257, 366, 676-677, 925`, `frontend/src/routes/admin/subscriptions/plans/+page.svelte:269-275`, `frontend/src/routes/admin/subscriptions/plans/plans.remote.ts:44-71`, `frontend/src/routes/admin/products/+page.svelte:162-172`, `frontend/src/lib/api/admin.ts:48-63, 70-125, 132-200, 1550-1586`. |
| **§2.3.4** | Replaced `confirm()`/`alert()` in `/admin/courses/*` & `/admin/members/service/[id]`; hydrated `/admin/products/[id]/edit`, `/admin/coupons/edit/[id]`, `/admin/users/edit/[id]` | **Confirm modals:** `frontend/src/routes/admin/courses/create/+page.svelte:440-447, 554, 1775-1788`, `frontend/src/routes/admin/courses/[id]/+page.svelte:122-144, 554-571`, `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte:110-126, 613-630`.<br>**`/admin/members/service/[id]` confirm/alert:** `NO-SUCH-CHANGE` (`+page.svelte:125-131, 187-206` only added `?? 0` null coalescing).<br>**Edit hydration:** `frontend/src/routes/admin/products/[id]/edit/+page.svelte:136-156, 268-272`, `frontend/src/routes/admin/coupons/edit/[id]/+page.svelte:108-131`, `frontend/src/routes/admin/users/edit/[id]/+page.svelte:77-92` (still broken; see §6 & §7). |
| **Row 15 / §2.4.1** | `api/tests/common/mod.rs:131-162`, `api/tests/integration_tests.rs:314-356` | `api/tests/common/mod.rs:105-115`, `api/tests/integration_tests.rs:314-345`. |
| **Row 16 / §2.4.2** | `api/tests/{orders_test,payments_test,admin_indicators_test,cms_v2_test,oauth_test}.rs` | `api/tests/orders_test.rs:54, 288-397`, `api/tests/payments_test.rs:48, 218-297`, `api/tests/admin_indicators_test.rs:44, 67-178`, `api/tests/cms_v2_test.rs:57, 109-371`, `api/tests/oauth_test.rs:53, 74-169`. |

---

## 3. Unmentioned-Findings Table (`counter-prompt.md` §1.4)

| # | Audit Finding | Verdict | Evidence (`file:line`) |
| :-: | :--- | :---: | :--- |
| **1** | Global SvelteKit circuit breaker (`frontend/src/routes/api/[...path]/+server.ts`) | **`NOT-FIXED`** (Partially Mitigated) | `frontend/src/routes/api/[...path]/+server.ts:48-54, 98-128, 170` still uses a single module-global `circuitBreaker` object (`threshold: IS_DEV ? 3 : 5`, `resetTimeout: 30000`) that rejects **all** `/api/*` requests with `503 CIRCUIT_OPEN` for 30s when tripped. Commit `7f6a0f9df` only removed `500` from `RETRYABLE_STATUS_CODES` at line `71`. |
| **2** | `frontend/src/lib/api/config.ts` sending `/api/admin/*` straight to cross-origin API | **`PARTIAL`** (`FIXED` for browser `apiFetch('/api/admin/*')`; `NOT-FIXED` for browser `/api/*` & `buildApiUrl()`) | `frontend/src/lib/api/config.ts:316-329` now routes browser `apiFetch` calls starting with `/api/admin` or `/admin/` to same-origin `/api/admin/*`. However, `API_BASE_URL` is still `'http://localhost:8080'` (`config.ts:12-17`), so all browser non-admin `/api/*` calls (including `/api/cms/*`) hit `config.ts:330-331` (`url = `${API_BASE_URL}${endpoint}``) cross-origin, and `buildApiUrl()` (`config.ts:532-538`) still unconditionally prepends `API_BASE_URL`. |
| **3** | `/admin/products` `formatPrice(product.price)` | **`FIXED`** | `frontend/src/lib/api/admin.ts:1550-1586` added `denormalizeProduct` (mapping `price_cents / 100` → `price`), and `frontend/src/routes/admin/products/+page.svelte:162-172` guards `formatPrice` against `null`/`undefined`/`NaN`. |
| **4** | `/admin/memberships` null `features` | **`FIXED`** | `frontend/src/routes/admin/memberships/+page.svelte:236-242` normalizes `p.features` to an array in `loadData()`, with additional array guards at lines `367`, `671-678`, and `925`. |
| **5** | Money still `f64`/`Decimal` on `abandoned-carts`, `/api/coupons`, `crm` contacts/deals | **`PARTIAL`** (`FIXED` on `abandoned-carts`; `NOT-FIXED` on `/api/coupons` & `CrmDeal.amount`) | `api/src/routes/admin/abandoned_carts.rs:29, 39-115, 163-165, 256-258` now uses `*_cents: i64`. However, `CreateCouponRequest` (`api/src/routes/coupons.rs:120-134`) for `POST /api/coupons` and `PUT /api/coupons/{id}` (`lines 449, 556`) still uses `Decimal` (`discount_value`, `min_purchase`, `max_discount`), `validate_coupon` (`coupons.rs:238-239, 281`) and `CouponResponse::from` (`coupons.rs:106`) still convert via `f64`, and `CrmDeal.amount` (`api/src/routes/crm/deals_pipelines.rs:68`) is still `sqlx::types::Decimal`. |
| **6** | Repeat-`DELETE` returning `200` on `campaigns`, `coupons`, `products` | **`PARTIAL`** (`FIXED` on hard `DELETE`; `NOT-FIXED` on soft-delete archive/alerts) | Hard `DELETE` handlers now check `rows_affected() == 0` → `404`: `api/src/routes/admin/campaigns_stats.rs:111-116`, `api/src/routes/crm/sequences_campaigns.rs:348-353`, `api/src/routes/admin/coupons.rs:294-299`, `api/src/routes/coupons.rs:663-668`, `api/src/routes/products.rs:360-365`. However, `POST /api/admin/products/{id}/archive` (`api/src/routes/products.rs:443-462`) and `DELETE /api/admin/room-content/alerts/{id}` (`api/src/routes/room_content/alerts.rs:285-315`) still return `200 OK` on repeated calls. |
| **7** | Plan creation under 6-way concurrency (`subscriptions_admin/plans.rs`) | **`FIXED`** (Mislocated in `report.md`) | `api/src/routes/subscriptions_admin/plans.rs:178-185` maps PostgreSQL `23505` (`unique_violation` on `membership_plans_slug_key`) to `409 Conflict`. Tested with 6-way `tokio::JoinSet` concurrency at `api/tests/tier1_followup_test.rs:292-407`. |
| **8** | 60 CMS/page-layout routes extracting `Json<T>` before the role check | **`PARTIAL`** (`FIXED` for `role="user"`; `NOT-FIXED` for `role="editor"` on 7 CMS admin-only routes + 14 non-CMS routes) | Replacing `user: User` with `CmsEditorUser`/`AdminUser` before `Path`/`Json` fixes `role="user"` (`403` before `422`). However, **7 CMS admin-only handlers** extract `CmsEditorUser(user)`, then `Path(...)` and `Json(...)`, and only call `require_cms_admin(&user)?` / `require_admin(&user)?` inside the body (`api/src/routes/cms_datasources/datasources.rs:274-281`, `api/src/routes/cms_global_components/crud.rs:359-366`, `api/src/routes/cms_global_components/versions.rs:92-98`, `api/src/routes/cms_presets/crud.rs:152-159`, `api/src/routes/cms_reusable_blocks/crud.rs:304-311`, `api/src/routes/cms_v2.rs:340-347`, `api/src/routes/cms_v2_enterprise.rs:122-129`), so `role="editor"` still gets `400`/`422` before `403`. Also, 14 non-CMS admin handlers (`api/src/routes/analytics.rs`, `coupons.rs`, etc.) still place `Path`/`Query` before `AdminUser`. |
| **9** | `/setup-db` removal-or-gate decision (owner never answered) | **`NOT-FIXED`** (Unilaterally kept at root) | Without owner sign-off, `7f6a0f9df` kept `/setup-db`, `/run-migrations`, and `/init-db` mounted at root (`api/src/routes/health.rs:384-386`) and modified their inline DDL/guards (`health.rs:196-331`). |

---

## 4. Gate Results with Exit Codes & Last 5 Lines of Output (§1.6)

All 7 gate commands were executed once each against `HEAD` (`7f6a0f9df`):

### 4.1 Summary Table

| # | Gate Command | Exit Code | Result Summary |
| :-: | :--- | :-: | :--- |
| 1 | `pnpm --filter revolution-svelte check` | `0` | `svelte-check found 0 errors and 0 warnings` |
| 2 | `pnpm --filter revolution-svelte lint` | `0` | Clean (`0` errors, `0` warnings) |
| 3 | `pnpm --filter revolution-svelte test:unit` | `0` | `63 passed (63)` test files, `2778 passed \| 32 skipped (2810)` tests |
| 4 | `pnpm --filter revolution-svelte build` | `0` | `@sveltejs/adapter-cloudflare` production build & prerender succeeded |
| 5 | `cd api && cargo fmt --check` | `0` | Clean (no diff output) |
| 6 | `cd api && cargo clippy --locked --all-targets -- -D warnings` | `0` | `0` warnings, `0` errors |
| 7 | `cd api && DATABASE_URL="postgres://rtp:rtp_password@localhost:5432/rtp_verify_db" TEST_DATABASE_URL="postgres://rtp:rtp_password@localhost:5432/rtp_verify_db" REDIS_URL="redis://localhost:6379/9" JWT_SECRET="test-jwt-secret-minimum-32-characters-for-testing" cargo test --locked -- --test-threads=4` | `0` | **`855 passed, 0 failed, 7 ignored`** across 88 test binaries (`+8` over prior `847 passed / 0 failed / 7 ignored` baseline, accounted for by the 8 new tests in `api/tests/tier1_followup_test.rs`) |
| 8 | `cd api && cargo deny check` | *(Skipped)* | `git show --stat 7f6a0f9df -- api/Cargo.toml api/Cargo.lock` produced empty output (no dependency changes in `7f6a0f9df`) |

### 4.2 Last 5 Lines of Output per Gate Command

1. **`pnpm --filter revolution-svelte check`** (`EXIT_CODE=0`):
   ```text
   $ svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --threshold warning
   Loading svelte-check in workspace: /Users/billyribeiro/dev/revolution-trading-pros/frontend
   Getting Svelte diagnostics...

   svelte-check found 0 errors and 0 warnings
   ```

2. **`pnpm --filter revolution-svelte lint`** (`EXIT_CODE=0`):
   ```text
   $ NODE_OPTIONS='--max-old-space-size=4096' eslint .
   EXIT_CODE=0
   ```

3. **`pnpm --filter revolution-svelte test:unit`** (`EXIT_CODE=0`):
   ```text
    Test Files  63 passed (63)
         Tests  2778 passed | 32 skipped (2810)
      Start at  23:12:41
      Duration  5.24s (transform 2.45s, setup 2.13s, import 5.95s, tests 2.38s, environment 19.30s)
   EXIT_CODE=0
   ```

4. **`pnpm --filter revolution-svelte build`** (`EXIT_CODE=0`):
   ```text
   > Using @sveltejs/adapter-cloudflare
     ✔ done
   ✓ built in 14.35s
   EXIT_CODE=0
   ```

5. **`cd api && cargo fmt --check`** (`EXIT_CODE=0`):
   ```text
   EXIT_CODE=0
   ```

6. **`cd api && cargo clippy --locked --all-targets -- -D warnings`** (`EXIT_CODE=0`):
   ```text
       Checking revolution-api v0.1.0 (/Users/billyribeiro/dev/revolution-trading-pros/api)
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.53s
   EXIT_CODE=0
   ```

7. **`cd api && cargo test --locked -- --test-threads=4` (Fresh `rtp_verify_db` from `schema.sql` + `sqlx-seed.sql`, Redis DB 9)** (`EXIT_CODE=0`):
   ```text
   test utils_jwt_roundtrip ... ok

   test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

      Doc-tests revolution_api

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   EXIT_CODE=0
   ```
   *(Aggregate across all 88 `running N tests` blocks: **`855 passed; 0 failed; 7 ignored`**, up by `+8` from the prior `847 passed / 0 failed / 7 ignored` baseline due to the 8 new tests in `tier1_followup_test.rs`.)*

---

## 5. Live Spot-Check Table — Before (`60288659d`) vs After (`7f6a0f9df`) (§2)

**Methodology:**
- Built `/tmp/revolution-api-before` from a detached git worktree at `60288659d` (`/tmp/rtp-before-60288659d`, running on `http://localhost:18080`, fresh DB `rtp_live_before`, Redis DB `10`) and `/tmp/revolution-api-after` from `HEAD` `7f6a0f9df` (running on `http://localhost:18081`, fresh DB `rtp_live_after`, Redis DB `11`).
- Both databases were initialized from `api/migrations/schema.sql` + `api/migrations/sqlx-seed.sql`, plus a singleton row in `cms_site_settings` and user accounts registered via real `POST /api/auth/register` + `POST /api/auth/login` (`member@example.com` with `role='user'` and `admin@example.com` elevated to `role='admin'`).

### 5.1 Targeted Tier-1 Live Probes

| Probe Category | Endpoint / Probe | Before (`60288659d`) | After (`7f6a0f9df`) | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **CMS v2 RBAC** | `GET /api/admin/cms-v2/settings` | `no=200 mem=200 adm=200` | `no=401 mem=403 adm=200` | Fixed (`500` for admin if `cms_site_settings` is empty because `get_site_settings` uses `.fetch_one` at `api/src/services/cms_content.rs:1181`) |
| **CMS v2 RBAC** | `GET /api/admin/cms-v2/menus` | `no=200 mem=200 adm=200` | `no=401 mem=403 adm=200` | Fixed |
| **CMS v2 RBAC** | `GET /api/admin/cms-v2/menus/1` | `no=200 mem=200 adm=200` | `no=401 mem=403 adm=400` | Anon/member now blocked (`401`/`403`) before UUID path parsing (`400`) |
| **CMS v2 RBAC** | `GET /api/admin/cms-v2/workflows` | `no=404 mem=404 adm=404` | `no=404 mem=404 adm=404` | **Route does not exist** (`report.md` fabricated `list_workflows` / `get_workflow`) |
| **Room Content RBAC** | `GET /api/admin/room-content/rooms/explosive-swings/alerts` | `no=401 mem=200 adm=200` | `no=401 mem=403 adm=200` | Fixed (`role="user"` now receives `403`) |
| **Room Content RBAC** | `GET /api/admin/room-content/rooms/explosive-swings/trades` | `no=401 mem=200 adm=200` | `no=401 mem=403 adm=200` | Fixed |
| **Room Content RBAC** | `GET /api/admin/room-content/rooms/explosive-swings/stats` | `no=401 mem=200 adm=200` | `no=401 mem=403 adm=200` | Fixed |
| **Member Create** | `POST /api/admin/members` (cited in `report.md`) | `405` (`0` rows in `users`) | `405` (`0` rows in `users`) | `POST` is not mounted on `/api/admin/members` |
| **Member Create** | `POST /api/admin/member-management` (real route) | `200` (**`0` rows in `users`**) | `200` (**`1` row in `users`**) | Before `7f6a0f9df`, returned `200 OK` with fake member JSON while rolling back the uncommitted transaction; after `7f6a0f9df`, row persists |
| **Plans Validation** | `POST /api/admin/subscriptions/plans` (`price_cents: -5000`) | `500` | `422` (`{"error":"price_cents must be non-negative"}`) | Note: `POST /api/subscriptions/plans` cited in `report.md` returns `405` |
| **Plans Validation** | `POST /api/admin/subscriptions/plans` (`name: "   "`) | `500` | `422` (`{"error":"Plan name must not be empty"}`) | Fixed |
| **Plans Validation** | `POST /api/admin/subscriptions/plans` (duplicate name/slug) | `200` then `500` | `200` then `409` | Fixed (`23505` mapped to `409 Conflict` when `membership_plans` schema matches; note `schema.sql` lacks `trial_days` unless added) |
| **Plans Concurrency** | 6 concurrent identical `POST /api/admin/subscriptions/plans` | `[500, 500, 500, 500, 500, 500]` (or `1×200 + 5×500`) | `1×200 + 5×409` (or `6×409` if slug already exists) | Fixed in `api/src/routes/subscriptions_admin/plans.rs:178-185` |
| **Pagination** | `GET /api/admin/courses?page=999999999&per_page=100000` | `ERR(RemoteDisconnected)` (panic in debug build) | `200` (`per_page: 100`) | Fixed overflow panic |
| **Pagination** | `GET /api/admin/courses?page=-1&per_page=-5` | `200` (`page: 1, per_page: 1`) | `200` (`page: 1, per_page: 1`) | Already clamped before `7f6a0f9df` |
| **Pagination** | `GET /api/admin/indicators?page=999999999&per_page=100000` | `ERR(RemoteDisconnected)` (panic in debug build) | `200` (`per_page: 100`) | Fixed overflow panic |
| **Pagination** | `GET /api/admin/indicators?page=-1&per_page=-5` | `200` (`page: 1, per_page: 1`) | `200` (`page: 1, per_page: 1`) | Already clamped before `7f6a0f9df` |
| **Pagination** | `GET /api/admin/popups?page=999999999&per_page=100000` | `ERR(RemoteDisconnected)` (panic in debug build) | `200` (`page: 10000, per_page: 100`) | Fixed overflow panic |
| **Pagination** | `GET /api/admin/popups?page=-1&per_page=-5` | `500` (`LIMIT must not be negative`) | `200` (`page: 1, per_page: 1`) | Fixed |
| **Same-Second Refresh** | `POST /api/auth/login` → `refresh` #1 → `refresh` #2 (`< 1s`) | `login=200`, `refresh1=200` (`r1 == r2`), **`refresh2=401` (`REFRESH_TOKEN_REUSE`)** | `login=200`, `refresh1=200` (`r1 != r2`), **`refresh2=200`** | Confirmed: before `7f6a0f9df`, `refresh1` minted an identical token (`r1 == r2`) and blacklisted its own hash, causing `refresh2` to fail with `401 REFRESH_TOKEN_REUSE` and revoke all user sessions |

### 5.2 Seeded Random Sample (`seed = 20261102`): 10 `LIVE-FAIL` + 10 `LIVE-OK` Routes from Audit CSV (`ca98481be:beaudit2-routes.csv`)

| Bucket | Route ID | Method & Probed Path | Audit 1 Evidence (`beaudit2-routes.csv`) | Before (`60288659d`) | After (`7f6a0f9df`) | Did Fix Change Result? |
| :--- | :--- | :--- | :--- | :-: | :-: | :--- |
| `LIVE-FAIL` #1 | `R713` | `GET /api/admin/members/filters/1` (admin) | `adm=500` | `500` | `404` (`Filter not found`) | **Yes — Fixed** (`072` aligned `member_filters` + `(col AT TIME ZONE 'UTC')`) |
| `LIVE-FAIL` #2 | `R130` | `DELETE /api/admin/products/1` (admin) | `adm=200` (on missing ID) | `200` | `404` (`Product not found`) | **Yes — Fixed** (`rows_affected() == 0` → `404`) |
| `LIVE-FAIL` #3 | `R536` | `GET /api/admin/room-content/rooms/explosive-swings/trade-plan` (member) | `mem=200` (missing admin RBAC) | `200` | `403` (`Admin access required`) | **Yes — Fixed** (`admin_list_trade_plans` requires `AdminUser`) |
| `LIVE-FAIL` #4 | `R739` | `GET /api/admin/organization/teams` (admin) | `adm=500` | `500` | `200` (`{"data":[],"success":true,"total":0}`) | **Yes — Fixed** (`072` created `team_members`) |
| `LIVE-FAIL` #5 | `R746` | `GET /api/admin/organization/departments/1` (admin) | `adm=500` | `500` | `404` (`Department not found`) | **Yes — Fixed** (`072` created `department_members`) |
| `LIVE-FAIL` #6 | `R303` | `GET /api/admin/trading-rooms` (admin) | `adm=200` (500 before `TIMESTAMPTZ` cast) | `200` | `200` | **Yes — Fixed** (`(created_at AT TIME ZONE 'UTC')` in `admin_rooms.rs:34-49`) |
| `LIVE-FAIL` #7 | `R249` | `GET /api/cms/settings` (admin) | `adm=500` (when `cms_site_settings` empty) | `200` (seeded) / `500` (unseeded) | `200` (seeded) / `500` (unseeded) | **Unchanged** (`sqlx-seed.sql` still does not seed `cms_site_settings`) |
| `LIVE-FAIL` #8 | `R626` | `GET /api/admin/crm/lists/1` (admin) | `adm=400` (when probed with UUID) | `404` | `404` (`List not found`) | Unchanged (`i64` ID returns `404` when absent) |
| `LIVE-FAIL` #9 | `R113` | `DELETE /api/admin/campaigns/1` (admin) | `adm=200` (on missing ID) | `200` | `404` (`Campaign not found`) | **Yes — Fixed** (`rows_affected() == 0` → `404`) |
| `LIVE-FAIL` #10 | `R762` | `GET /api/cms/reusable-blocks` (admin) | `adm=200` | `200` | `200` | Unchanged (`200 OK`) |
| `LIVE-OK` #1 | `R277` | `GET /api/realtime/events` (admin) | SSE stream | SSE stream (timeout after 5s) | SSE stream (timeout after 5s) | Unchanged (healthy SSE stream) |
| `LIVE-OK` #2 | `R563` | `GET /api/analytics/room/explosive-swings/monthly` (public/member) | `200`/`401` | `401` (anon) | `401` (anon) | Unchanged |
| `LIVE-OK` #3 | `R758` | `POST /api/cms/ai/assist` (admin, `{}`) | `422` | `422` | `422` (`missing field action`) | Unchanged |
| `LIVE-OK` #4 | `R876` | `PUT /api/admin/tags/1` (admin, `{}`) | `404` | `404` | `404` (`Tag not found`) | Unchanged |
| `LIVE-OK` #5 | `R660` | `DELETE /api/admin/crm/webhooks/1` (admin) | `200` | `200` | `404` (`Webhook not found`) | **Improved** (`rows_affected() == 0` → `404` instead of false `200`) |
| `LIVE-OK` #6 | `R265` | `GET /api/cms/content/1/revisions/1` (admin) | `400` (UUID parse) | `400` | `400` | Unchanged (`AdminUser` checked before `Path<Uuid>`) |
| `LIVE-OK` #7 | `R852` | `POST /api/cms/global-components/usage/1/sync` (admin) | `400` (UUID parse) | `400` | `400` | Unchanged |
| `LIVE-OK` #8 | `R257` | `GET /api/admin/cms-v2/enterprise/workflow/1/history` (member) | `mem=400` (UUID parsed before role check) | `400` (`UUID parsing failed`) | `403` (`CMS editor access required`) | **Improved** (`CmsEditorUser` now runs before `Path<Uuid>`) |
| `LIVE-OK` #9 | `R808` | `GET /api/cms/scheduling/schedules/history` (admin) | `200` | `200` | `200` | Unchanged |
| `LIVE-OK` #10 | `R566` | `GET /api/watchlist` (member) | `200` | `200` | `200` | Unchanged |

---

## 6. Browser Pass Table — Playwright Headless Chromium Across 13 Admin Pages (§3)

**Environment:**
- Headless Chromium via Playwright (`page.on('dialog', d => d.accept())`), authenticated as `admin@example.com` (`role="admin"`), against SvelteKit dev server (`http://localhost:15173`) + live Rust API at `HEAD` (`http://localhost:18081`) with seeded rows for `popups (id=1)`, `products (id=1)`, `coupons (id=1)`, `users (id=1)`, `orders (id=1)`, `membership_plans`, `courses (id=1)`, and `cms_site_settings`.
- Note: Across all 13 pages, the shared admin layout (`frontend/src/routes/admin/+layout.svelte`) emits two background console errors in dev: `GET /api/admin/consent/templates → 404 Not Found` and WebSocket connection attempts to hardcoded `ws://localhost:8080/{,popups,ws,subscriptions}` (`ERR_CONNECTION_REFUSED`).

| # | Route | Seeded URL Visited | HTTP | Uncaught `pageerror` | Native Dialogs | `NaN` in DOM Text? | Form / Page Hydration Status | Screenshot |
| :-: | :--- | :--- | :-: | :-: | :-: | :-: | :--- | :--- |
| 1 | `/admin/popups/new` | `/admin/popups/new` | `200` | `0` | `0` | `false` | **OK** — Form controls render with default values (`exit_intent`, `draft`, priority `10`) | `verify-evidence/01-admin-popups-new.png` |
| 2 | `/admin/popups/[id]/edit` | `/admin/popups/1/edit` | `200` | `0` | `0` | `false` | **BROKEN HYDRATION** — `name`, `title`, `content`, `cta_text`, `cta_url` inputs are **blank** because `popupsApi.get(id)` (`frontend/src/lib/api/popups.ts:1739`) reads from an empty client-side Svelte store `get(popups)` instead of fetching `/api/admin/popups/1` | `verify-evidence/02-admin-popups-1-edit.png` |
| 3 | `/admin/site-health` | `/admin/site-health` | `200` | `0` | `0` | `false` | **OK** — Renders system health metrics without crashing | `verify-evidence/03-admin-site-health.png` |
| 4 | `/admin/settings` | `/admin/settings` | `200` | `0` | `0` | `false` | **OK** — Renders settings categories & connection status | `verify-evidence/04-admin-settings.png` |
| 5 | `/admin/media` | `/admin/media` | `200` | `0` | `0` | `false` | **OK** — Renders Media Library & `StatsPanel` (`0 B` formatted cleanly) | `verify-evidence/05-admin-media.png` |
| 6 | `/admin/orders` | `/admin/orders` | `200` | `0` | `0` | `false` | **OK** — Renders seeded order `#ORD-VERIFY-001` with `$197.00` (no `$NaN`) | `verify-evidence/06-admin-orders.png` |
| 7 | `/admin/memberships` | `/admin/memberships` | `200` | `0` | `0` | `false` | **OK** — Renders seeded plans (`$197`, `$97`, `$297`) and features without crashing or `$NaN` | `verify-evidence/07-admin-memberships.png` |
| 8 | `/admin/subscriptions/plans` | `/admin/subscriptions/plans` | `200` | `0` | `0` | `false` | **OK** — Renders subscription plans with formatted prices (no `$NaN`) | `verify-evidence/08-admin-subscriptions-plans.png` |
| 9 | `/admin/products/[id]/edit` | `/admin/products/1/edit` | `200` | `0` | `0` | `false` | **OK** — Hydrates `name="High Octane Scanner"`, `slug="high-octane-scanner"`, `description="Institutional momentum scanner"`, `price="197"` | `verify-evidence/09-admin-products-1-edit.png` |
| 10 | `/admin/coupons/edit/[id]` | `/admin/coupons/edit/1` | `200` | `0` | `0` | `false` | **OK** — Hydrates `code="SAVE20"`, `type="percentage"`, `value="20"`, `minimum_amount="50"`, `usage_limit="500"` | `verify-evidence/10-admin-coupons-edit-1.png` |
| 11 | `/admin/users/edit/[id]` | `/admin/users/edit/1` | `200` | `0` | `0` | `false` | **BROKEN HYDRATION** — Only `email="member@example.com"` populates; **`First Name` and `Last Name` are blank and `Roles` is unchecked** because `GET /api/admin/users/1` (`api/src/routes/admin/users.rs:21-31`) returns `{name, role}` while `/admin/users/edit/[id]/+page.svelte:86-92` reads non-existent `user.first_name`, `user.last_name`, and `user.roles` | `verify-evidence/11-admin-users-edit-1.png` |
| 12 | `/admin/courses/create` | `/admin/courses/create` | `200` | `0` | `0` | `false` | **OK** — Renders course creation form & inline confirmation modal state without native `confirm()`/`alert()` | `verify-evidence/12-admin-courses-create.png` |
| 13 | `/admin/members/service/[id]` | `/admin/members/service/1` | `200` | `0` | `0` | `false` | **OK** — Renders member service detail view without `$NaN` or native dialogs | `verify-evidence/13-admin-members-service-1.png` |

---

## 7. Regressions & Remaining Bugs Found (§1.3, §1.5, §2, §3)

### 7.1 Frontend Pages Claimed Fixed in `report.md` That Are Still Broken in the Browser
1. **`/admin/popups/[id]/edit` Never Fetches the Popup from the Backend (`frontend/src/lib/api/popups.ts:1739` & `frontend/src/routes/admin/popups/[id]/edit/+page.svelte:26`):**
   - `+page.svelte:26` calls `const response = await popupsApi.get(popupId);`.
   - In `frontend/src/lib/api/popups.ts:1739`, `popupsApi.get` is implemented as:
     ```ts
     get: async (id: number) => ({ popup: get(popups).find((p) => p.id === String(id)) }),
     ```
   - It reads from an in-memory Svelte store (`popups`) without ever calling `GET /api/admin/popups/${id}`! Navigating directly to `/admin/popups/1/edit` returns `{ popup: undefined }`, leaving `name`, `title`, `content`, `cta_text`, and `cta_url` completely blank (`verify-evidence/02-admin-popups-1-edit.png`).
2. **`/admin/users/edit/[id]` Field Name Mismatch Leaves Name & Roles Blank (`frontend/src/routes/admin/users/edit/[id]/+page.svelte:86-92` vs `api/src/routes/admin/users.rs:21-31`):**
   - `GET /api/admin/users/{id}` (`api/src/routes/admin/users.rs:21-31`) returns `AdminUserRow` with fields `id`, `email`, `name`, `role: Option<String>`, `email_verified_at`, `created_at`.
   - `frontend/src/routes/admin/users/edit/[id]/+page.svelte:86-92` sets:
     ```ts
     formData.name = user.name || '';
     formData.first_name = user.first_name || '';
     formData.last_name = user.last_name || '';
     formData.roles = Array.isArray(user.roles) ? user.roles : [];
     ```
   - Because `user.first_name`, `user.last_name`, and `user.roles` are `undefined` on the backend response (and the template only renders inputs for `formData.first_name`, `formData.last_name`, and `formData.roles`, with no input for `formData.name`), `First Name`, `Last Name`, and `Roles` render blank (`verify-evidence/11-admin-users-edit-1.png`).

### 7.2 Backend Bugs & Regressions
1. **Weakened Public Router Test in `api/tests/cms_v2_test.rs:344-348`:**
   - Commit `7f6a0f9df` removed `assert_ne!(resp.status(), StatusCode::NOT_FOUND)` from `public_router_is_get_only_and_rejects_mutations`, allowing unmounted public GET routes returning `404` to pass silently.
2. **Missed Pagination & Null-Byte Sanitization on `GET /api/admin/members` (`api/src/routes/members.rs:88-100`):**
   - `per_page = params.per_page.unwrap_or(25).min(100)` does not clamp lower bounds (`?per_page=-5` → PostgreSQL `LIMIT -5` → `500`), `(page - 1) * per_page` can overflow `i64`, and `params.search` does not strip `\0` (`?search=%00` → PostgreSQL `invalid byte sequence for encoding "UTF8": 0x00` → `500`).
3. **`close_trade` Unknown Direction Silently Treated as `"short"` (`api/src/routes/room_content/trades.rs:339-343`):**
   - Contrary to `report.md` §2.1.7, `close_trade` does not validate `"short"` or return `400 Bad Request` on invalid direction; its `else` branch computes short PnL (`entry_price - exit_price`) for any non-`"long"` string.
4. **Silent No-Op Type Change & Duplicate DDL in `api/migrations/schema.sql:21926-22070` (`072_audit_followup_schema_alignment.sql:72-86`):**
   - Appending `ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS avg_rating DOUBLE PRECISION` after `CREATE TABLE public.courses (... avg_rating numeric(3,2) ...)` (`schema.sql:5912`) is a no-op that leaves `courses.avg_rating` as `numeric(3,2)`.
   - Also, `membership_plans` in `api/migrations/schema.sql:7887-7906` is missing `trial_days INTEGER DEFAULT 0` (which `api/src/routes/subscriptions_admin/plans.rs:20, 44, 71, 160` queries), causing `POST /api/admin/subscriptions/plans` to fail with `42703: column "trial_days" does not exist` on a fresh DB built from `schema.sql` unless `trial_days` is added.
5. **Unseeded Singleton `cms_site_settings` Causes `500` on `GET /api/admin/cms-v2/settings` and `GET /api/cms/settings`:**
   - `get_site_settings` (`api/src/services/cms_content.rs:1181`) uses `.fetch_one(pool).await`, returning `SqlxError::RowNotFound` → `500 Internal Server Error` on a fresh DB built from `schema.sql` + `sqlx-seed.sql` because `sqlx-seed.sql` does not insert the singleton `cms_site_settings` row.

### 7.3 Remaining Masked DB Errors (`unwrap_or_default()` / `unwrap_or(...)` / `.ok()`) (`counter-prompt.md` §1.3)
- **`grep -rn "unwrap_or_default()" api/src/routes | wc -l` = `76`:**
  - `0 / 76` are direct `sqlx::Result::unwrap_or_default()` calls.
  - `4 / 76` unwrap an `Option<T>` produced by an immediately preceding masked SQL query (`.await.unwrap_or(None)` or `.await.ok().flatten()`):
    1. `api/src/routes/admin_videos/operations.rs:308` (masks `SELECT tags FROM unified_videos` at `lines 299-303`).
    2. `api/src/routes/cms_assets/bulk.rs:141` (masks `SELECT tags FROM cms_assets` at `lines 134-139`).
    3. `api/src/routes/connections/handlers.rs:548` (masks `SELECT credentials_encrypted FROM service_connections` at `lines 538-544`).
    4. `api/src/routes/member_courses/prerequisites.rs:117` (masks `SELECT title FROM lessons` at `lines 110-116`).
  - `19 / 76` are in comments (`// FIX-2026-04-26: .unwrap_or_default()`), and `53 / 76` are harmless non-SQL uses (`Option<String>`, `serde_json`, `std::env::var`).
- **`137` SQL queries across `34` route files STILL mask DB errors via `.await.unwrap_or(...)` (plus `45` via `.await.ok()`), including in files `report.md` claimed to have cleaned:**
  - `api/src/routes/crm/mod.rs:63, 71, 79, 85, 92, 98, 104` (7 queries; line `104` queries non-existent `crm_campaigns.emails_sent` and masks the runtime PostgreSQL error to `0`).
  - `api/src/routes/analytics.rs:196, 204, 212` (3 queries; file was untouched in `7f6a0f9df`).
  - `api/src/routes/admin_members/analytics.rs:34, 41, 48, 66` (4 queries).
  - `api/src/routes/crm/deals_pipelines.rs:253, 261, 595` (3 queries) & `api/src/routes/crm/leads.rs:204-241` (9 queries).
  - Additional masked queries in: `admin_member_management/{activity.rs:79, crud.rs:150}`, `admin_courses/{quizzes.rs:265, structure.rs:51, taxonomy.rs:155}`, `admin_orders.rs:286`, `admin_videos/{operations.rs:256,303, series_chapters.rs:226}`, `cms_assets/{listing.rs:170, stats.rs:18-46}`, `connections/{audit_log.rs:42, handlers.rs:64,159,495-516}`, `contacts.rs:78`, `courses_admin/{crud.rs:619, enrollments.rs:106, lessons.rs:40,129,405,502, live_sessions.rs:151, resources.rs:50,211, sections.rs:34,108,253,307}`, `indicators_admin.rs:286,513,667,795`, `media/crud.rs:116`, `member_courses/{public.rs:78,127, quizzes.rs:62,165,198,244,253,274,286,355}`, `member_indicators/{downloads.rs:384, public.rs:84,152,232}`, `members.rs:178-621` (15 queries), `newsletter.rs:118`, `orders/admin_list.rs:82`, `organization/{departments.rs:58, teams.rs:59}`, `reconciliation.rs:505-529`, `room_content/weekly_videos.rs:94`, `room_resources/{admin_crud.rs:133, download.rs:124, public.rs:124-577}`, `settings.rs:341-623`, `sitemap.rs:351-374`, `subscriptions/admin.rs:57-171`, `videos.rs:255-552`.

### 7.4 Remaining Runtime `TIMESTAMPTZ` → `NaiveDateTime` Bugs Across the 59 Route Files (`counter-prompt.md` §1.3)
Cross-checking all `59` route files mentioning `NaiveDateTime` against `api/migrations/schema.sql` reveals **5 active runtime bugs** where the PostgreSQL column is `timestamp with time zone` (`TIMESTAMPTZ`) and the SQL query lacks `AT TIME ZONE`:
1. `api/src/routes/admin_videos/mod.rs:81, 83, 104, 105, 107` (`UnifiedVideoRow`) + `api/src/routes/admin_videos/crud.rs:97-108, 213, 296-303, 400-405`: Queries `SELECT * FROM unified_videos` into `NaiveDateTime`, whereas `unified_videos.{published_at, created_at, updated_at, deleted_at}` at `api/migrations/schema.sql:10266-10270` are `timestamp with time zone` (and `scheduled_at` does not exist on `unified_videos`).
2. `api/src/routes/admin_videos/operations.rs:194-200` (`export_videos_csv`): Queries `unified_videos.created_at` (`timestamp with time zone` at `schema.sql:10268`) into `chrono::NaiveDateTime`.
3. `api/src/routes/videos.rs:72, 284-295, 378-386, 453-458, 485-501` (`UnifiedVideoRow`): Queries `unified_videos.created_at` (`timestamp with time zone` at `schema.sql:10268`) into `chrono::NaiveDateTime`.
4. `api/src/routes/member_indicators/downloads.rs:334, 350-368` (`DownloadHistoryRow`): Queries `indicator_downloads.downloaded_at` (`timestamp with time zone` at `schema.sql:7066`) into `Option<chrono::NaiveDateTime>`, and `ORDER BY d.downloaded_at DESC NULLS LAST, d.created_at DESC` (`line 366`) references non-existent `indicator_downloads.created_at`.
5. `api/src/routes/bunny_upload.rs:385-396` (`list_uploads`): Queries `bunny_uploads.upload_started_at` into `chrono::NaiveDateTime`, whereas `bunny_uploads` (`schema.sql:3625-3639`) has `created_at` / `updated_at` as `timestamp with time zone` and no `upload_started_at` column.

---

## 8. Git-History Finding (§1.1 — Stated Neutrally with Exact Commands Run)

### 8.1 Commands Executed & Output

1. **`git rev-parse HEAD origin/main`**
   ```text
   7f6a0f9dfa4dbdd5e5675426b340674bffcd941c
   7f6a0f9dfa4dbdd5e5675426b340674bffcd941c
   ```

2. **`git branch -a`**
   ```text
     backend
   * main
     seo/audit-2026-11-01
     remotes/origin/HEAD -> origin/main
     remotes/origin/backend
     remotes/origin/main
   ```

3. **`git log --all --oneline --graph -n 15`**
   ```text
   * 7f6a0f9df (HEAD -> main, origin/main, origin/HEAD) fix(admin): resolve backend & frontend audit findings across all 4 tiers
   * 60288659d fix(api): resolve 15 backend audit findings and schema drift (#815)
   * c27b4b7fa chore(deps): update dependencies across root, frontend, and api (#814)
   ```

4. **`gh pr list --state all --limit 5`**
   ```text
   815	fix(api): resolve 15 backend audit findings and hardening	chore/backend-audit-hardening	MERGED	2026-11-02T20:11:36Z
   814	chore(deps): update dependencies across root, frontend, and api	chore/deps-update-2026-11	MERGED	2026-11-02T03:09:49Z
   813	fix(api): resolve all 42 clippy warnings across test targets	fix/clippy-test-warnings	MERGED	2026-11-02T02:27:43Z
   812	chore: full-repo health pass (cargo fmt, svelte-check, clippy)	chore/repo-health-2026-11-01	MERGED	2026-11-02T01:36:25Z
   811	feat(seo): complete 2026-11-01 SEO standard enforcement	seo/audit-2026-11-01	MERGED	2026-11-02T00:50:19Z
   ```

5. **`git reflog -n 20`**
   ```text
   7f6a0f9df (HEAD -> main, origin/main, origin/HEAD) HEAD@{0}: commit: fix(admin): resolve backend & frontend audit findings across all 4 tiers
   60288659d HEAD@{1}: reset: moving to 60288659d
   acb452362 HEAD@{2}: commit: docs(report): add final verification & remediation report
   2fa862eef HEAD@{3}: merge fix/audit-followup-2026-11: Fast-forward
   ca98481be HEAD@{4}: checkout: moving from fix/audit-followup-2026-11 to main
   2fa862eef HEAD@{5}: commit: fix(admin): resolve backend & frontend audit findings across all 4 tiers
   ca98481be HEAD@{6}: reset: moving to ca98481be
   620d12eca HEAD@{7}: commit: test(api): wire shadow-router tests to real setup_test_app and enforce rate-limit 429 (Tier 4)
   c8e39d2e7 HEAD@{8}: commit: fix(frontend): resolve admin crashes, proxy routing, $NaN prices, and native dialogs (Tier 3)
   25bab71e6 HEAD@{9}: commit: fix(api): resolve Tier 2 schema drift, 500s, error masking, and RBAC extractor ordering
   805bbfd15 HEAD@{10}: commit: fix(api): resolve all 8 Tier 1 security, data-loss, and correctness bugs
   d82ffb0f4 HEAD@{11}: commit: test(api): add Tier 1 failing reproduction tests
   ca98481be HEAD@{12}: checkout: moving from main to fix/audit-followup-2026-11
   ca98481be HEAD@{13}: checkout: moving from fix/audit-followup-2026-11 to main
   ca98481be HEAD@{14}: checkout: moving from main to fix/audit-followup-2026-11
   ca98481be HEAD@{15}: checkout: moving from backend to main
   ca98481be HEAD@{16}: commit: chore: update worktree files
   60288659d HEAD@{17}: pull --ff-only: Fast-forward
   ```

### 8.2 Neutral Reconstruction of Events
1. At `HEAD@{16}` (`ca98481be`), 11 files (`beaudit-report.md`, `beaudit-routes.csv`, `beaudit2-probe.sh`, `beaudit2-report.md`, `beaudit2-routes.csv`, `counter-prompt.md`, `delivered-report.md`, `feaudit-probe.mjs`, `followup-prompt.md`, `prompt.md`, `report.md`; `+3,737` lines) were committed onto `main` on top of `60288659d`.
2. At `HEAD@{14}`..`HEAD@{7}`, a local branch `fix/audit-followup-2026-11` was created from `ca98481be` and received 5 commits (`d82ffb0f4`, `805bbfd15`, `25bab71e6`, `c8e39d2e7`, `620d12eca`).
3. At `HEAD@{6}`..`HEAD@{2}`, `fix/audit-followup-2026-11` was soft-reset to `ca98481be`, squashed into `2fa862eef`, fast-forward merged into `main`, and followed by `acb452362` (`docs(report): add final verification & remediation report`).
4. At `HEAD@{1}`, `main` was reset back to `60288659d` (`git reset 60288659d`) — which also rewound past `ca98481be`, dropping the 10 audit report/CSV/prompt files that had been committed in `ca98481be` — and everything was committed directly onto `main` as `7f6a0f9df` (`HEAD@{0}`).
5. The local branch `fix/audit-followup-2026-11` was deleted, no pull request was opened (`gh pr list` shows `#815` as the latest PR), and `7f6a0f9df` was pushed directly to `origin/main`.

---

## 9. Honest Gaps

1. **Blog Editor Routes Out of Scope:** Per `counter-prompt.md` §3 (`line 84`), `/admin/blog/*` and `BlockEditor` were not tested in the Playwright pass.
2. **External Third-Party APIs Not Exercised Live:** Live external calls to Stripe, Bunny.net, Postmark, Cloudflare R2, Google/Apple OAuth providers, Meilisearch, and Anthropic/Claude were not invoked with live credentials during spot-checking.
3. **Seeded Schema Adjustments Required for Live Spot-Checks:** Because `api/migrations/schema.sql` + `sqlx-seed.sql` does not insert a row into `cms_site_settings` and omits `membership_plans.trial_days`, our throwaway live spot-check DBs (`rtp_live_before`, `rtp_live_after`) had a singleton `cms_site_settings` row inserted and `trial_days` added to isolate and test the handler logic on `/api/admin/cms-v2/settings` and `/api/admin/subscriptions/plans`. Both underlying schema/seed omissions are documented in §5.1 and §7.2.

---

## 10. Teardown Commands (Print Only — Not Executed)

```bash
# 1. Remove the detached git worktree used for the 'before' (60288659d) build
git worktree remove --force /tmp/rtp-before-60288659d

# 2. Drop the throwaway PostgreSQL verification databases in container rtp-db
docker exec rtp-db psql -U rtp -d postgres -c "DROP DATABASE IF EXISTS rtp_verify_db;"
docker exec rtp-db psql -U rtp -d postgres -c "DROP DATABASE IF EXISTS rtp_verify_mig_db;"
docker exec rtp-db psql -U rtp -d postgres -c "DROP DATABASE IF EXISTS rtp_live_before;"
docker exec rtp-db psql -U rtp -d postgres -c "DROP DATABASE IF EXISTS rtp_live_after;"

# 3. Flush the throwaway Redis databases (9, 10, 11) in container rtp-redis
docker exec rtp-redis redis-cli -n 9 FLUSHDB
docker exec rtp-redis redis-cli -n 10 FLUSHDB
docker exec rtp-redis redis-cli -n 11 FLUSHDB

# 4. Remove temporary binaries and logs in /tmp
rm -f /tmp/revolution-api-before /tmp/revolution-api-after \
      /tmp/spotcheck_results.json /tmp/browser_pass_results.json \
      /tmp/subagent1_full.md /tmp/subagent1_part2.md /tmp/subagent2_full.md \
      /tmp/api_before.log /tmp/api_after.log /tmp/pw_api.log /tmp/pw_fe.log \
      /tmp/schema_load.log /tmp/seed_load.log /tmp/mig_schema_load.log \
      /tmp/mig072_run1.log /tmp/mig072_run2.log
```

