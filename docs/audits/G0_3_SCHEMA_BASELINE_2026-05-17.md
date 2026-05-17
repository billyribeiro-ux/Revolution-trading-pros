# G0.3 — Schema reproducibility: reconstructed baseline + fresh-env mechanism

**Date:** 2026-05-17
**Audit item:** G0.3 (schema not reproducible from zero — P0 decision gate)
**Status:** Reconstructed baseline **shipped & proven**. One owner-gated
production cutover step remains (exact runbook below).

---

## 1. Summary of evidence (pasted)

```
REPLAY 60/60 OK on db=g03final          # all 60 migrations replay on a
                                        # fresh DB w/ reconstructed baseline

# schema.sql loaded into a brand-new EMPTY database:
schema.sql load rc=0
--- error count: 0 ---
TABLE COUNT: 204

# fresh-env end-to-end (schema.sql + sqlx-seed.sql into a clean DB):
schema.sql   rc=0 err=0
sqlx-seed.sql rc=0 err=0
sqlx_rows=60  tables=205        # 204 app tables + _sqlx_migrations

# the EXACT production call proven a no-op (gated Rust test):
G0.3 PROOF OK: schema.sql + seed -> embedded migrator no-op;
               60 versions applied, 204 tables.
test schema_sql_plus_seed_makes_embedded_migrator_a_noop ... ok

# constraint: NO committed migration was edited:
$ git diff --stat -- api/migrations/0*.sql
(empty)
```

The "8/60 fail" headline from the original audit was a **cascade**, not 8
independent bugs. Re-derived empirically by iterating fresh-DB replays.

---

## 2. Proven gap inventory & the reconstructed objects

The historical sqlx chain assumes a Laravel-era substrate that **no
migration creates**. Each reconstructed object's column types are derived
from the application contract (Rust models / `sqlx` queries in
`api/src/`) and cross-checked against how the migrations use the column.

### 2.1 `user_sessions` — vestigial table (in `schema.sql` baseline)

- **Failure:** `035_ict7_member_system_complete.sql:180` —
  `ALTER TABLE user_sessions ADD COLUMN …` (guarded) but table absent.
- **Code evidence:** **No SQL query in `api/src/` targets a
  `user_sessions` table.** Sessions are 100 % Redis:
  `api/src/services/redis.rs` builds `"user_sessions:{id}"` Redis keys;
  `auth.rs`/`admin.rs` only call `redis.invalidate_all_user_sessions()`.
- **Reconstruction:** minimal Laravel-shape stub. The 3 columns 035 adds
  (`device_fingerprint`, `is_mfa_verified`, `mfa_verified_at`) are
  intentionally **not** in the baseline so 035 owns them. No app
  contract constrains the stub's types → each marked
  `-- RECONSTRUCTED: verify vs prod`.

### 2.2 `traders` — vestigial FK target (in `schema.sql` baseline)

- **Failure (the cascade root):** `036_room_resources_ict7_complete.sql`
  fails at line 192 `CREATE INDEX … (trading_room_id)`. Real cause:
  line 105 `ALTER TABLE room_resources ADD COLUMN trader_id BIGINT
  REFERENCES traders(id)` lives inside a
  `DO $$ … EXCEPTION WHEN others THEN NULL; END $$;` block. With
  `traders` absent that one FK fails, the **whole DO block silently
  rolls back**, so `trading_room_id`, `content_type`, `section`,
  `resource_date`, `trader_id` (and every other 036 column add) never
  happen — which is why the audit saw 5 "missing column" errors. They
  are **one** root cause.
- **Code evidence:** the app's real trader data is `room_traders`
  (`api/src/routes/trading_rooms.rs` `admin_list_traders`:
  `FROM room_traders t`). The public `list_traders` handler returns a
  hardcoded empty array. **No `sqlx` query targets a bare `traders`
  table.** Its only consumer is 036's FK.
- **Reconstruction:** minimal `traders` table with **`id BIGSERIAL`** so
  the FK target type matches `room_resources.trader_id`
  (`api/src/routes/room_resources.rs:58` `pub trader_id: Option<i64>` →
  `BIGINT`). Other columns marked `-- RECONSTRUCTED: verify vs prod`.

#### Resulting `room_resources` gap-column types (now created by 036 itself once the cascade is unblocked — code-verified)

| Column | Type | Code evidence (`api/src/routes/room_resources.rs`) |
|---|---|---|
| `trading_room_id` | `BIGINT` | struct `RoomResource` L57 `pub trading_room_id: i64` (NOT NULL); joins `trading_rooms.id` (BIGSERIAL). 036 also: `ADD COLUMN trading_room_id BIGINT`. |
| `content_type` | `VARCHAR/TEXT` | L42 `pub content_type: String` (NOT NULL); bound in INSERT L796. 036: `ADD COLUMN content_type VARCHAR(50)`. |
| `section` | `TEXT` | L43 `pub section: Option<String>`. 036: `ADD COLUMN section VARCHAR(50)`. |
| `resource_date` | `DATE` | L59 `pub resource_date: NaiveDate` (NOT NULL); parsed `%Y-%m-%d` L767. 036: `ADD COLUMN resource_date DATE`. |
| `trader_id` | `BIGINT` | L58 `pub trader_id: Option<i64>`. 036: `ADD COLUMN trader_id BIGINT REFERENCES traders(id)`. |

These columns end up in `schema.sql` via 036 (once `traders` exists), so
they carry 036's exact types — fully consistent with the app contract
above. No guessing.

### 2.3 `unified_videos.tags` — JSONB (scratch patch on 051)

- **Failure:** `051_video_system_ict7_complete.sql:87`
  `CREATE INDEX … USING GIN (tags jsonb_path_ops)`. `unified_videos` is
  created by `015` **without** `tags`, and **no migration adds it**.
- **Type proof — MUST be JSONB:**
  - `jsonb_path_ops` rejects TEXT (`operator class "jsonb_path_ops" does
    not accept data type text` — observed).
  - App contract: `api/src/routes/videos.rs:24` struct `UnifiedVideoRow`,
    `:41 pub tags: Option<serde_json::Value>`, selected via
    `SELECT v.* FROM unified_videos`; `:221`
    `v.tags @> {}::jsonb` containment.
- **Reconstruction:** scratch 051 self-adds the column **before** the GIN
  index — `ALTER TABLE unified_videos ADD COLUMN IF NOT EXISTS tags JSONB
  DEFAULT '[]'::jsonb` — mirroring how 036 self-adds its own columns.
  Idempotent; prod's `ADD COLUMN IF NOT EXISTS` semantics unaffected.

### 2.4 `user_memberships.trial_ends_at` / `grace_period_end` — TIMESTAMP (scratch patch on 053)

- **Failure:** `053_subscription_notifications_ict7.sql:19/23` index
  `trial_ends_at` / `grace_period_end`, but those columns are first added
  by migration **060** — 7 migrations later (ordering bug; prod's
  Laravel-era table already had them).
- **Type proof — `TIMESTAMP` (no tz):**
  - `api/src/routes/subscriptions.rs:221`
    `pub trial_ends_at: Option<chrono::NaiveDateTime>`
  - `:230 pub grace_period_end: Option<chrono::NaiveDateTime>`,
    selected `SELECT um.trial_ends_at, um.grace_period_end FROM
    user_memberships`; `payments.rs:1354` binds `.naive_utc()`.
  - `chrono::NaiveDateTime` ↔ Postgres `TIMESTAMP` — the **exact** type
    migration 060 also uses (`ADD COLUMN … trial_ends_at TIMESTAMP`).
- **Reconstruction:** scratch 053 self-adds both idempotently before its
  indexes (`ADD COLUMN IF NOT EXISTS … TIMESTAMP`). 060's
  `ADD COLUMN IF NOT EXISTS` stays a no-op.

### 2.5 Genuine intra-chain SQL defects (scratch patches)

| File | Defect | Scratch fix |
|---|---|---|
| `041_cms_presets.sql` | `CONSTRAINT … UNIQUE (block_type, slug) WHERE deleted_at IS NULL` — a table UNIQUE constraint cannot carry `WHERE` (invalid SQL, state-independent). Later `ON CONFLICT (block_type, slug) WHERE deleted_at IS NULL` upsert needs the matching partial unique index. | Converted to a partial `CREATE UNIQUE INDEX … WHERE deleted_at IS NULL`, **placed in the indexes section above the seed INSERT** so the `ON CONFLICT` resolves. |
| `050_room_fulltext_search_indexes.sql` | Re-defines `search_room_content(...)` with a changed signature → "function is not unique" on replay. | Scratch prepends `DROP FUNCTION IF EXISTS search_room_content(TEXT,TEXT,TEXT[],INT,INT);`. |
| `054_cms_scheduling_releases.sql` | References `performed_by` column not present. | Scratch adds `performed_by BIGINT`. |

> All scratch patches live ONLY in throwaway copies under `/tmp/g03work/`
> during generation. **No committed `api/migrations/0*.sql` was edited**
> (`git diff --stat -- api/migrations/0*.sql` is empty). The patches are
> the documented procedure for *regenerating* `schema.sql`, never applied
> to production.

---

## 3. Prod-safety mechanism

| | Production | Fresh env (CI / local / DR) |
|---|---|---|
| Migration source | committed `0*.sql`, replayed by `sqlx::migrate!("./migrations")` (unchanged) | `api/migrations/schema.sql` (one shot) |
| `_sqlx_migrations` | as the chain wrote it | seeded from `api/migrations/sqlx-seed.sql` (real SHA-384 of each committed file) |
| App startup | `sqlx::migrate!` replays normally | `sqlx::migrate!` sees all 60 versions applied w/ matching checksums → **no-op** (proven) |

Nothing about how the app loads migrations changed. The fresh-env path
is additive: `schema.sql` + `sqlx-seed.sql`, both shipped in
`api/migrations/`. Bootstrap steps and seed-generation are in
`api/migrations/README.md`.

**Proof the production call itself is satisfied:** the gated test
`api/tests/g03_schema_baseline_test.rs` loads `schema.sql`, seeds
`_sqlx_migrations` with SHA-384 of every committed file, then runs the
**identical** `sqlx::migrate!("./migrations").run(&pool)` the app runs in
`src/db/mod.rs`. It completes `Ok` (no `VersionMismatch` — all 60
embedded checksums equal the seeded ones) with 60 versions applied and
204 tables. Gated on `G03_VERIFY_DB_URL`; skips instantly when unset so
the no-DB test set is unaffected.

---

## 4. Reproducing `schema.sql` from scratch

1. Copy all 60 committed `0*.sql` to a scratch dir.
2. Apply the three scratch content-patches in §2.5 and the §2.3/§2.4
   self-add patches to the scratch copies only.
3. Write the reconstructed pre-sqlx baseline (§2.1 `user_sessions` +
   §2.2 `traders`).
4. Fresh DB → apply baseline → replay the 60 scratch migrations in
   filename order. Iterate until `REPLAY 60/60 OK`.
5. `pg_dump --schema-only --no-owner --no-privileges <db>`; strip the
   pg_dump 16 `\restrict`/`\unrestrict` lines (psql-only; break raw
   protocol loaders); prepend the header; write `api/migrations/schema.sql`.
6. Regenerate `api/migrations/sqlx-seed.sql` (procedure in
   `api/migrations/README.md`).
7. Verify: load `schema.sql` into a brand-new empty DB → 0 errors;
   `SELECT count(*) FROM information_schema.tables WHERE
   table_schema='public'` → 204. Run
   `api/tests/g03_schema_baseline_test.rs` with `G03_VERIFY_DB_URL`.

---

## 5. The ONE remaining owner-gated step (production cutover)

The fresh-env mechanism is fully shipped & proven. **Not done, owner-
gated:** confirming the reconstructed baseline matches the **live
production** schema and aligning prod's lineage. The reconstruction is
type-derived from the app contract — until diffed against prod it is
*believed*, not *verified*, equal.

```bash
# 1. Capture canonical prod schema (maintenance window).
pg_dump --schema-only --no-owner --no-privileges "$PROD_DATABASE_URL" \
  > /tmp/prod_schema.sql

# 2. Normalise (strip psql meta-cmds + pg_dump version banner) and diff.
norm() { grep -vE '^\\(un)?restrict |^-- Dumped (from|by)' "$1"; }
diff <(norm /tmp/prod_schema.sql) <(norm api/migrations/schema.sql)

# 3. Reconcile each diff hunk:
#    - prod-only object the reconstruction missed  -> add to schema.sql, commit
#    - reconstruction-only (a scratch-patch artifact) -> confirm harmless / adjust
#    - type drift on a reconstructed column          -> correct schema.sql to prod's type
# 4. Confirm prod _sqlx_migrations holds exactly the 60 historical
#    versions with disk-matching checksums (also resolves audit P2-I).
#    Prod KEEPS replaying the historical chain; schema.sql stays the
#    fresh-env source of truth only.
```

With the reconstructed `schema.sql` in hand this is a ~30-minute
owner task (a bounded diff-and-reconcile), **not** the open-ended
research project it was before G0.3. Do not run any prod
`_sqlx_migrations` operation without explicit owner sign-off
(repo `CLAUDE.md`).

---

## 6. Files delivered / modified

| Path | Status |
|---|---|
| `api/migrations/schema.sql` | **new** — reconstructed baseline (60/60 replay, 0-err from-zero, 204 tables) |
| `api/migrations/sqlx-seed.sql` | **new** — `_sqlx_migrations` seed (60 rows, real SHA-384) |
| `api/migrations/README.md` | **new** — fresh-env bootstrap + owner runbook |
| `api/tests/g03_schema_baseline_test.rs` | **new** — gated proof the prod migrator call is a no-op |
| `docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md` | **new** — this note |
| `docs/audits/REMEDIATION_PLAN_2026-05-17.md` | **modified** — G0.3 row/section/carry-over updated |
| `api/migrations/0*.sql` (60 files) | **UNTOUCHED** — `git diff --stat` empty |
