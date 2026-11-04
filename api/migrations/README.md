# Database migrations — production chain vs. fresh-environment baseline

This directory holds two **different** things. Read this before touching
either.

| Artifact            | Used by                                                                                                                                     | Never                                                                                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0*.sql` (60 files) | **Production** & any already-bootstrapped DB. `sqlx::migrate!("./migrations")` in `src/db/mod.rs` embeds & replays them, checksum-verified. | **Never edit/rename/delete a committed `0*.sql`.** sqlx stores a SHA-384 of each; changing one makes the running app refuse to boot (`VersionMismatch`). |
| `schema.sql`        | **Fresh environments only** — CI DB-tests, local first-boot, disaster recovery. One-shot full current schema.                               | Not loaded by the app. Not applied to production.                                                                                                        |

## Why `schema.sql` exists (audit G0.3)

The historical `0*.sql` chain is **not reproducible from zero**. It
assumes a Laravel-era substrate that no migration creates, and carries
three intra-chain SQL/ordering defects. A from-scratch `sqlx migrate
run` aborts mid-chain. Full root-cause + per-column type evidence:
[`docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`](../../docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md).

`schema.sql` is a `pg_dump --schema-only --no-owner --no-privileges` of a
database built by replaying **all 60** migrations on top of a
reconstructed pre-sqlx baseline. Proven: 60/60 replay; loads into an
empty DB with **0 errors / 204 tables**.

## Automatic empty-database initialization

`Database::migrate()` initializes an empty `public` schema at API startup:

1. Take a transaction-scoped advisory lock to serialize first boot.
2. Load `schema.sql` and `sqlx-seed.sql` atomically, only when there are no application tables.
3. Restore `search_path`, commit the baseline, and execute the embedded migrator.
4. Run every forward migration absent from the baseline ledger, including 073's structural defaults and the media/behavior tables.

No existing application schema is automatically marked as migrated. A non-empty, unbaselined historical database must be reconciled separately. Checksum mismatches and failed migrations abort startup before workers or the HTTP listener start. `SKIP_MIGRATIONS` no longer bypasses readiness. The migration connection is closed even on failure so SQLx cannot retain a session advisory lock in the application pool.

A new local database therefore needs only `DATABASE_URL` plus the ordinary API configuration; no manual SQL baseline step is required. Do not run this against production during development.

For explicit baseline tooling, these two steps still work:

```bash
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f api/migrations/schema.sql
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f api/migrations/sqlx-seed.sql
```

Then start the API to execute all forward migrations. The ledger represents **only** the migrations embodied in `schema.sql` (currently through 072); never seed later migrations without applying their actual SQL. The full schema/defaults proof is `api/tests/admin_completion_test.rs`. `api/tests/g03_schema_baseline_test.rs` independently verifies baseline + ledger + forward migrations when `G03_VERIFY_DB_URL` is provided.

> SQLx ignores `schema.sql` and `sqlx-seed.sql` because their names have no numeric underscore-separated migration prefix. Preserve these names.

### Generating `sqlx-seed.sql`

The seed must carry the **real SHA-384** of each committed `0*.sql`
(exactly what `sqlx::migrate!` embeds at compile time, sqlx 0.8). It is
generated mechanically from the committed files — never hand-written:

```bash
{
  echo "CREATE TABLE IF NOT EXISTS _sqlx_migrations ("
  echo "  version BIGINT PRIMARY KEY, description TEXT NOT NULL,"
  echo "  installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),"
  echo "  success BOOLEAN NOT NULL, checksum BYTEA NOT NULL,"
  echo "  execution_time BIGINT NOT NULL);"
  for f in api/migrations/[0-9]*.sql; do
    b=$(basename "$f")
    ver=$(echo "$b" | sed -E 's/^0*([0-9]+)_.*/\1/'); [ -z "$ver" ] && ver=0
    desc=$(echo "$b" | sed -E 's/^[0-9]+_//; s/\.sql$//')
    sum=$(sha384sum "$f" | awk '{print $1}')
    echo "INSERT INTO _sqlx_migrations (version,description,success,checksum,execution_time)"
    echo "VALUES ($ver,'$desc',true,'\\x$sum'::bytea,0) ON CONFLICT (version) DO NOTHING;"
  done
} > api/migrations/sqlx-seed.sql
```

`version` = leading digits of the filename (e.g. `068_…` → `68`);
`description` = filename minus the numeric prefix and `.sql`. These match
sqlx 0.8's own filename parsing. Regenerate the seed **and** `schema.sql` together only when intentionally rebuilding the baseline; do not add forward migrations to the baseline ledger alone. See the G0.3 audit note for the full regeneration procedure using scratch content patches.

## Regenerating `schema.sql`

`schema.sql` is **not** a raw replay — three committed migrations have
defects (`041` invalid `UNIQUE … WHERE` table constraint; `050`
duplicate function; `053`/`060` column-ordering) and the chain assumes a
pre-sqlx baseline. The exact, reproducible build procedure (reconstructed
baseline + the scratch content-patches, applied to throwaway copies — the
committed files are never touched) is documented step-by-step in
[`docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`](../../docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md).

`schema.sql` carries no psql meta-commands (the pg_dump 16
`\restrict`/`\unrestrict` lines are stripped) so it loads via both `psql
-f` and the raw Postgres protocol (`sqlx::raw_sql`).

## The ONE remaining owner step (production cutover)

The fresh-env mechanism above is fully shipped and proven. **What is
_not_ done** — and is owner-gated — is migrating the **production**
database onto this baseline so prod and fresh envs share one source of
truth. Until then, prod and the reconstructed baseline are only _believed_
to match (the reconstruction is type-derived from the app contract, not
diffed against the live prod schema).

Owner runbook (≈30 min, maintenance window):

```bash
# 1. Capture the canonical production schema.
pg_dump --schema-only --no-owner --no-privileges "$PROD_DATABASE_URL" \
  > /tmp/prod_schema.sql

# 2. Normalise both (strip psql meta-cmds + dump-version banner) and diff.
norm() { grep -vE '^\\(un)?restrict |^-- Dumped (from|by)' "$1"; }
diff <(norm /tmp/prod_schema.sql) <(norm api/migrations/schema.sql)

# 3. Reconcile any prod-only objects into schema.sql (commit the result),
#    OR document them as intentional prod drift.
# 4. In the maintenance window, on prod: confirm _sqlx_migrations holds
#    exactly the 60 historical versions with disk-matching checksums
#    (resolves audit P2-I), then leave prod as-is — prod KEEPS replaying
#    the historical chain. schema.sql is only the fresh-env source.
```

Do **not** run step 4's `_sqlx_migrations` operations against prod
without explicit owner sign-off (per repo `CLAUDE.md`).
