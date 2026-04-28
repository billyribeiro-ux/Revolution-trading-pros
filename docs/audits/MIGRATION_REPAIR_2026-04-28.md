# MIGRATION_REPAIR_PLAN.md

Generated: 2026-04-28  
Author: Claude Code (Phase 2 read-only inventory)  
Status: **AWAITING USER APPROVAL — do not execute Phases 4-7**

---

## Phase 1 — Backup (COMPLETE ✅)

| Item | Result |
|------|--------|
| Dump file | `/tmp/rtp-backup-pre-migration-repair.dump` (729 KB, custom format) |
| Restore test | Restored to `rtp_scratch`, dropped after verify |
| User count pre/post | 1 / 1 ✅ |

---

## Phase 2 — Inventory

### 2A. Migration files on disk (47 files total)

| Version | Filename | Bytes | MD5 |
|---------|----------|-------|-----|
| 000 | 000_bootstrap_users.sql | 737 | b6aa4eeaa02d2a4c1139ccc354fcd2cc |
| 001 | 001_initial_schema.sql | 23592 | c661396c36b7d4fbe69c786208082c2a |
| 002 | 002_fix_password_column.sql | 1920 | 6fb6fc418e8ca3a82373d00732230b58 |
| 003 | 003_fix_jobs_schema.sql | 2369 | 6e0c003c18aa73a772fce76515ad4ef4 |
| 004 | 004_add_mfa_enabled.sql | 981 | 8f8b6e18e6a95adcf1fbc4041090a773 |
| 007 | 007_email_verification_standalone.sql | 1448 | 0990f33f3ebe71adbc9c92399d585663 |
| 008 | 008_seed_membership_plans.sql | 5185 | 9420ff0298413f18d0366d9b95ca6cee |
| 009 | 009_add_performance_indexes.sql | 9674 | efc9b0ac5b4a7215fee7479295150208 |
| 010 | 010_fix_coupons_schema.sql | 376 | f605bb6c783db4e26459d7a1f4e0794f |
| 012 | 012_add_redirects_table.sql | 1340 | eba9392ad943e51042d6de9a1de0e9c3 |
| 013 | 013_trading_room_schedules.sql | 12847 | 08a64449ea5b984fbf4e4d47d3887b27 |
| 014 | 014_advanced_cms_features.sql | 22112 | d0c303953b08631549629bc9cfdac98a |
| 015 | 015_consolidated_schema.sql | 45540 | ffdd56f98a16c519328116537004b69d |
| 016 | 016_fix_course_schema.sql | 12132 | 1cc23d02b87ab643df79d190eb8b4f7d |
| 017 | 017_fix_schema_mismatches.sql | 5565 | ad25e0b888d65306b09ae978c8b5f405 |
| 018 | 018_explosive_swings_complete.sql | 22133 | 29abdf0a295ef1d9ba9975f9132ad3da |
| 019 | 019_subscription_variants.sql | 29020 | e5326de40d6b4b453b898b4121de6df8 |
| 020 | 020_add_runner_stop_column.sql | 424 | 94a606a8137499247b211a75cdbd7ffc |
| 021 | 021_add_room_slug_to_bunny_uploads.sql | 763 | 88e326679c861480c9749dca20155f30 |
| 022 | 022_position_invalidate_fields.sql | 3846 | 8c706ba66f38ee0e2e90e36dffc37ed6 |
| 023 | 023_custom_cms_implementation.sql | 48900 | 1acdd9c79c9f8264c1024fb9c7afae26 |
| 024 | 024_advanced_cms_features.sql | 50039 | 550601fcfca3f72fc3b146884ca341fe |
| 025 | 025_blog_editor_enhancements.sql | 25306 | d9c6dcf2b6d5a6ba3fdd1b9488f75772 |
| 027 | 027_cms_v2_enterprise_features.sql | 15256 | 9fed7dc0bc4009cea4fcdd5d9e5e983c |
| 028 | 028_user_favorites_and_stats.sql | 12147 | 60f9d972e8c86b09eaed552d099f7520 |
| 029 | 029_cms_reusable_blocks_schema_fix.sql | 866 | c8ae3d799919855d0ed42e2a661da8c5 |
| 030 | **030_room_fulltext_search_indexes.sql** | 17218 | 06a1e2ea148f9956b46a90e2240477df |
| 030 | **030_room_search_indexes.sql** | 6902 | 154a59dd2f9782be8db35af4cf185ed1 |
| 031 | 031_analytics_functions.sql | 6246 | 2bb4a50f25f9d84053e64ac247b55fde |
| 032 | 032_search_function.sql | 4218 | 01daa68459d16697211b5ad1017160e3 |
| 033 | 033_oauth_providers.sql | 4548 | 1e514fcafff4195ef9f8c46b9a761618 |
| 035 | 035_ict7_member_system_complete.sql | 9734 | 44b7ac71ec21af4f8b6188c4e7460006 |
| 036 | 036_room_resources_ict7_complete.sql | 17654 | 6b378bf2d2695fceaf2bdcfacd9a45c5 |
| 037 | **037_forms_ict7_enhancements.sql** | 1766 | e93eb6489ec41e1a3df13cac816245b4 |
| 037 | **037_video_system_ict7_complete.sql** | 9535 | d7012f5ac00c45c5e579acfb320a3320 |
| 038 | 038_popups_system_complete.sql | 10596 | bebe1bad0d8df5ab270ff5319b31bca1 |
| 039 | 039_resources_ict7_enhanced.sql | 6068 | f0130d7d7f5a78d6f8be af2bdcfacd9a45c5 |
| 040 | **040_courses_ict7_complete.sql** | 18359 | 46e53989ce5db02f11df9175821c2e2b |
| 040 | **040_crm_deals_pipelines.sql** | 18184 | f4f815efa03ed6d47ea47ff50ca73a36 |
| 040 | **040_subscription_notifications_ict7.sql** | 1645 | 271ed4d336975c98951193e50f685b91 |
| 041 | **041_cms_presets.sql** | 23810 | 0ffc460d24f063689f8f6937377d223b |
| 041 | **041_cms_scheduling_releases.sql** | 25309 | c20645ace41904ed8b36bf9395668355 |
| 042 | 042_cms_datasources.sql | 23396 | ff47fbd68cf664610cfa91a38bf0db6e |
| 043 | 043_membership_plan_price_history.sql | 3056 | 1f55f4c908c53c2b254ec07ee860874f |
| 044 | 044_service_connections.sql | 4695 | ce3ce1b1f462f394bd4489e34c2f2bdd |
| 045 | 045_blog_post_metadata_and_categories.sql | 4340 | 976c6c0b548d83317edc753e0e2e5b9d |

**Missing version numbers (gaps):** 005, 006, 011, 026, 034

**Duplicate version numbers (8 pairs/triples):**
- 030: `030_room_fulltext_search_indexes.sql` AND `030_room_search_indexes.sql`
- 037: `037_forms_ict7_enhancements.sql` AND `037_video_system_ict7_complete.sql`
- 040: `040_courses_ict7_complete.sql` AND `040_crm_deals_pipelines.sql` AND `040_subscription_notifications_ict7.sql`
- 041: `041_cms_presets.sql` AND `041_cms_scheduling_releases.sql`

---

### 2B. `_sqlx_migrations` table (what the DB thinks it has)

Max registered version: **30** (30 rows total, versions 0-30, skipping 5, 6, 11, 26)

| Version | Description | Checksum (SHA-384 hex) | Success |
|---------|-------------|------------------------|---------|
| 0 | bootstrap users | 2e8193df...05319 | t |
| 1 | initial schema | 0e5566c8...c999 | t |
| 2 | fix password column | 9e50ea02...fd3d3 | t |
| 3 | fix jobs schema | a6d78d09...e58aae | t |
| 4 | add mfa enabled | 16eaf3d1...b8c9b | t |
| 7 | email verification standalone | fe87361...eaaf | t |
| 8 | seed membership plans | 5fd96ebd...a925 | t |
| 9 | add performance indexes | fcb26086...04cbd | t |
| 10 | fix coupons schema | adc5bce6...4d61 | t |
| 12 | add redirects table | 38c2abcb...6906 | t |
| 13 | trading room schedules | 3e8c44a5...ed64 | t |
| 14 | advanced cms features | 0f1014aa...7f33 | t |
| 15 | consolidated schema | 5e145873...57ef | t |
| 16 | fix course schema | 5bcd11d6...59b7 | t |
| 17 | fix schema mismatches | f8cad8d1...cf80 | t |
| 18 | explosive swings complete | 24928020...594 | t |
| 19 | subscription variants | 7ed50a45...5de4 | t |
| 20 | add runner stop column | 9c0647dd...df3a | t |
| 21 | add room slug to bunny uploads | 921dccf5...ccf | t |
| 22 | position invalidate fields | 4967d5ba...813 | t |
| 23 | custom cms implementation | e04b84fc...933 | t |
| 24 | advanced cms features | a8f018a0...202 | t |
| 25 | blog editor enhancements | 1cc51240...da1 | t |
| 27 | cms v2 enterprise features | b2de54a1...2ed | t |
| 28 | user favorites and stats | c66c71dc...af2 | t |
| 29 | cms reusable blocks schema fix | b6ddc5af...b02 | t |
| 30 | room search indexes | **5314a8d2...289** | t |

**The version 30 checksum in DB matches `030_room_search_indexes.sql`**, not `030_room_fulltext_search_indexes.sql`.

---

### 2C. Duplicate version analysis

#### v030 — THE ROOT CAUSE OF MIGRATION FREEZE

| File | SHA-384 | In DB? |
|------|---------|--------|
| `030_room_fulltext_search_indexes.sql` | `b0855fbb...1229` | ❌ NO |
| `030_room_search_indexes.sql` | `5314a8d2...289` | ✅ YES (matches DB) |

**sqlx loads migrations alphabetically.** At version 30, sqlx picks `030_room_fulltext_search_indexes.sql` (f comes before s). Its SHA-384 is `b0855fbb...` but the DB has `5314a8d2...`. **Checksum mismatch → sqlx aborts the entire migration run on every API startup.** All migrations 031–045 are silently skipped.

#### v037 — Two competing files, neither registered in DB

| File | SHA-384 | In DB? |
|------|---------|--------|
| `037_forms_ict7_enhancements.sql` | `12f9f7ed...128` | ❌ (v37 not in `_sqlx_migrations`) |
| `037_video_system_ict7_complete.sql` | `6db3ae19...c62` | ❌ |

Both files' content is already applied to the DB (tables `forms`, `videos` etc. exist). These would conflict even if the v030 mismatch were fixed.

#### v040 — Three competing files, none registered

sqlx alphabetically picks `040_courses_ict7_complete.sql`. All three files' DDL is already applied.

#### v041 — Two competing files, none registered

sqlx alphabetically picks `041_cms_presets.sql`. Both files' DDL is already applied.

---

### 2D. Tables from migrations 031–045

All tables from all migration files 031–045 **already exist in the DB** (applied manually via psql or inherited from an earlier dump). None are registered in `_sqlx_migrations`.

Key tables verified present:
- 031: `analytics_events` ✅
- 032: `cms_search_content` function ✅
- 033: `oauth_tokens`, `oauth_states` ✅
- 035: `user_mfa_secrets`, `mfa_attempts`, `login_rate_limits`, `member_audit_logs` ✅
  - `security_events.event_category`, `security_events.severity` ✅ (needed for H-7)
  - `user_sessions` ❌ DOES NOT EXIST (no migration creates it; 035's DO block silently bypassed in psql)
- 036–045: all representative tables verified ✅

**`user_sessions` is absent from the DB and from every migration file.** It is referenced only in 035's DO block (guarded by IF NOT EXISTS checks), which ran silently past the gap in psql.

---

### 2E. Current API behavior at startup

Every time the API container starts, sqlx runs `migrate!` and hits:

```
Migration checksum mismatch for version 30!
  Expected: b0855fbb... (030_room_fulltext_search_indexes.sql)
  Got:      5314a8d2... (030_room_search_indexes.sql)
```

sqlx **aborts** and returns an error. The API then either:
- (a) continues with whatever schema is already in the DB (the API itself doesn't crash on migration error — it logs and continues), OR
- (b) panics at startup depending on how the migration call is handled

The existing schema is complete enough for the API to function, which is why the stack works. But no future migrations can ever be applied automatically.

---

## Phase 3 — Reconciliation Plan

### Root causes (ordered by severity)

1. **`030_room_fulltext_search_indexes.sql` is the wrong file at version 030.** The DB was initialized with `030_room_search_indexes.sql`. The fulltext file was added later (presumably a renamed replacement), and its alphabetical priority breaks sqlx.

2. **Migrations 031–045 were applied outside sqlx** (via psql or dump restoration) and are not recorded in `_sqlx_migrations`. sqlx will try to re-apply them if the v030 mismatch is ever fixed.

3. **Four version numbers are reused** (030, 037, 040, 041). Each duplicate will cause another checksum mismatch the moment the v030 mismatch is resolved.

4. **`user_sessions` table is missing** — referenced in 035 but never created by any migration. This is a schema gap; the API currently has no session table.

---

### Fix strategy

The safest approach is **register-what's-applied** rather than re-run anything. All DDL is already in the DB. The goal is to make `_sqlx_migrations` reflect reality, and eliminate the duplicate-version files.

#### Step A — Rename the intruder file for v030

`030_room_fulltext_search_indexes.sql` → `050_room_fulltext_search_indexes.sql`

Why: `030_room_search_indexes.sql` matches the DB. The fulltext file is the newer/replacement that was given the wrong number. Moving it to an unused high version (050) resolves the alphabetical priority conflict without losing the DDL.

**Risk (assessed, LOW):** Both files share the same three GIN index names (`idx_room_alerts_fts`, `idx_room_trades_fts`, `idx_room_trade_plans_fts`). The fulltext file uses `DROP INDEX IF EXISTS` before recreating them, so if it runs after `030_room_search_indexes.sql` it will safely replace those three GIN indexes. It also adds 7 additional B-tree indexes with distinct names not present in the simpler file. No destructive overlap.

#### Step B — Rename duplicate files for 037, 040, 041

All duplicate-numbered files whose content is already in the DB need unique version numbers. Proposed renaming:

| Old name | New name | Already applied? |
|----------|----------|-----------------|
| `037_video_system_ict7_complete.sql` | `051_video_system_ict7_complete.sql` | ✅ yes |
| `040_crm_deals_pipelines.sql` | `052_crm_deals_pipelines.sql` | ✅ yes |
| `040_subscription_notifications_ict7.sql` | `053_subscription_notifications_ict7.sql` | ✅ yes |
| `041_cms_scheduling_releases.sql` | `054_cms_scheduling_releases.sql` | ✅ yes |

The alphabetically-first file in each duplicate group stays at its current number (037, 040, 041).

#### Step C — Register migrations 031–049 in `_sqlx_migrations`

For each migration that is already applied but not registered, insert a row directly into `_sqlx_migrations`:

```sql
INSERT INTO _sqlx_migrations (version, description, installed_on, checksum, success, execution_time)
VALUES (31, 'analytics functions', now(), decode('<sha384-hex>', 'hex'), true, 0);
-- ... repeat for 32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45
-- Use the alphabetically-first file's SHA-384 for duplicate-numbered versions
```

After Step B renames, the surviving first-alphabetical file for each version is the canonical one. Insert its checksum.

**Critical:** For version 30, the registered checksum must remain `5314a8d2...` (matches `030_room_search_indexes.sql`). Do NOT update it.

#### Step D — Create `user_sessions` stub

Since no migration creates `user_sessions` and the API has Redis-based session invalidation code that references it:

Option 1: Create a minimal `user_sessions` table in a new migration (e.g. `046_user_sessions.sql`)  
Option 2: Confirm the Rust API doesn't actually SELECT from `user_sessions` directly (it may only use Redis for sessions), and leave the table absent.

**Needs API code audit before deciding.**

#### Step E — Verify and test

After Steps A-D:
1. `docker compose down && docker compose up -d` — API must start without migration errors
2. `docker exec rtp-api curl -s http://localhost:8080/health` — must return healthy
3. Check API logs for any `checksum mismatch` or `relation does not exist` errors

---

### What will NOT be done

- Re-running any migration SQL against the DB (all DDL already applied)
- Dropping or recreating any table
- Touching production (Fly.io) — this plan is local only
- Merging or deduplicating SQL content between duplicate files

---

## STOP — Phase 4-7 require explicit user approval

**DO NOT EXECUTE any of the steps above until the user replies with explicit approval.**

The repair involves:
- Renaming 5 migration files (`git mv` equivalent)
- Direct INSERT into `_sqlx_migrations` (bypasses sqlx safety)
- Potentially creating a new `user_sessions` migration

These are irreversible once committed. Reply with:
> "Approved — execute Phases 4-7"

to proceed.
