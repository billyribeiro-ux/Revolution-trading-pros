#!/usr/bin/env bash
# Rebuild the LOCAL dev database from the documented fresh-environment baseline
# (api/migrations/README.md): schema.sql + sqlx-seed.sql, then copy the existing rows
# across. The old database is RENAMED, never dropped, so nothing is lost.
#
# Why: a dev DB that was loaded from an older snapshot drifts from schema.sql and the app's
# sqlx migrator then stops at the first conflict ("Migration error ... continuing without
# migrations"), silently leaving later migrations unapplied.
#
# LOCAL DEV ONLY. Requires the docker compose `rtp-db` container. Stop the API first.
#   usage: api/scripts/rebaseline-local-db.sh [db_name]   (default: revolution_trading_pros)
set -euo pipefail

DB="${1:-revolution_trading_pros}"
BACKUP="${DB}_backup_$(date +%Y%m%d%H%M%S)"
HERE="$(cd "$(dirname "$0")" && pwd)"
MIG="$HERE/../migrations"
PSQL=(docker exec -i rtp-db psql -U rtp -v ON_ERROR_STOP=1)

if [[ "${DB}" == *prod* ]]; then echo "refusing: '$DB' looks like production" >&2; exit 1; fi

echo "→ disconnecting clients and renaming $DB -> $BACKUP"
"${PSQL[@]}" -d postgres -Atq -c "select pg_terminate_backend(pid) from pg_stat_activity where datname='$DB' and pid<>pg_backend_pid()" >/dev/null
"${PSQL[@]}" -d postgres -q -c "ALTER DATABASE \"$DB\" RENAME TO \"$BACKUP\"" -c "CREATE DATABASE \"$DB\""

echo "→ loading schema.sql + sqlx-seed.sql"
"${PSQL[@]}" -d "$DB" -q < "$MIG/schema.sql"
"${PSQL[@]}" -d "$DB" -q < "$MIG/sqlx-seed.sql"

echo "→ copying rows from $BACKUP (matching tables, matching columns only)"
python3 - "$BACKUP" "$DB" <<'PY'
import subprocess, sys
old, new = sys.argv[1], sys.argv[2]
def q(db, sql):
    r = subprocess.run(["docker","exec","-i","rtp-db","psql","-U","rtp","-d",db,"-Atq","-c",sql],capture_output=True,text=True)
    return r.stdout
def cols(db):
    d = {}
    for l in q(db, "select table_name||'|'||column_name from information_schema.columns c join information_schema.tables t using (table_schema,table_name) where table_schema='public' and t.table_type='BASE TABLE' and table_name not like '\\_%' order by table_name,ordinal_position").splitlines():
        t, c = l.split("|"); d.setdefault(t, []).append(c)
    return d
o, n = cols(old), cols(new)
done, skipped = [], []
for t in sorted(o):
    if t not in n or q(old, f'select count(*) from "{t}"').strip() in ("", "0"): continue
    common = [c for c in o[t] if c in n[t]]; cl = ",".join(f'"{c}"' for c in common)
    out = subprocess.run(["docker","exec","rtp-db","psql","-U","rtp","-d",old,"-q","-c",f'COPY (select {cl} from "{t}") TO STDOUT'],capture_output=True)
    sql = f'SET session_replication_role=replica; COPY "{t}" ({cl}) FROM STDIN;\n'.encode() + out.stdout + b"\\.\n"
    r = subprocess.run(["docker","exec","-i","rtp-db","psql","-U","rtp","-d",new,"-q","-v","ON_ERROR_STOP=1"],input=sql,capture_output=True)
    (done if r.returncode == 0 else skipped).append(t)
    if r.returncode == 0:
        q(new, f"select setval(pg_get_serial_sequence('\"{t}\"','id'), coalesce((select max(id) from \"{t}\"),1)) where pg_get_serial_sequence('\"{t}\"','id') is not null")
print("copied:", ", ".join(done) or "none"); print("skipped:", ", ".join(skipped) or "none")
PY
echo "✓ done. Old data kept in database: $BACKUP"
