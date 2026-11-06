#!/usr/bin/env bash
set -euo pipefail

# Fresh PostgreSQL only; no caller database/provider credentials are inherited.
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
binary="$root/api/target/debug/examples/marketing_public_fixture"
[[ -x "$binary" ]] || { echo 'Build the marketing_public_fixture example first.' >&2; exit 1; }
for port in 55485 19085; do
  if lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
    echo "Fixture port $port is already occupied; refusing to replace it." >&2; exit 1
  fi
done
fixture_dir=$(mktemp -d "${TMPDIR:-/tmp}/revolution-marketing.XXXXXX")
owner=$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')
clean_env=(env -i "PATH=$PATH" "HOME=$HOME")
api_pid=
pg_started=0
cleanup() {
  if [[ -n "$api_pid" ]]; then kill "$api_pid" 2>/dev/null || true; wait "$api_pid" 2>/dev/null || true; fi
  if [[ "$pg_started" == 1 ]]; then "${clean_env[@]}" pg_ctl -D "$fixture_dir/postgres" -m fast -w -t 30 stop >>"$fixture_dir/postgres-stop.log" 2>&1; fi
  echo "Retained owned fixture: $fixture_dir"
}
trap cleanup EXIT
trap 'exit 130' INT TERM
"${clean_env[@]}" initdb -D "$fixture_dir/postgres" -U marketing_fixture --auth-local=trust --auth-host=trust --encoding=UTF8 --locale=C >"$fixture_dir/initdb.log" 2>&1
cat >>"$fixture_dir/postgres/postgresql.conf" <<'CONFIG'
listen_addresses = '127.0.0.1'
port = 55485
unix_socket_directories = ''
shared_preload_libraries = 'pg_stat_statements'
CONFIG
"${clean_env[@]}" pg_ctl -D "$fixture_dir/postgres" -l "$fixture_dir/postgres.log" -w -t 30 start
pg_started=1
"${clean_env[@]}" createdb -h 127.0.0.1 -p 55485 -U marketing_fixture marketing_fixture
"${clean_env[@]}" psql -X -h 127.0.0.1 -p 55485 -U marketing_fixture -d marketing_fixture -v ON_ERROR_STOP=1 -c "COMMENT ON DATABASE marketing_fixture IS '$owner'" >"$fixture_dir/owner.log"
echo "Owned marketing fixture: $fixture_dir"
cd -- "$fixture_dir"
"${clean_env[@]}" ENVIRONMENT=development PORT=19085 \
  DATABASE_URL=postgresql://marketing_fixture@127.0.0.1:55485/marketing_fixture \
  MARKETING_FIXTURE_OWNER="$owner" MARKETING_FIXTURE_SEED="${MARKETING_FIXTURE_SEED:-empty}" REDIS_URL=redis://127.0.0.1:1 \
  APP_URL=http://127.0.0.1:4185 \
  JWT_SECRET=disposable-marketing-fixture-jwt-secret-at-least-32-characters \
  MEMBER_INDICATOR_SECRET=disposable-marketing-fixture-indicator-secret \
  MEMBER_LICENSE_SECRET=disposable-marketing-fixture-license-secret \
  CREDENTIALS_ENCRYPTION_KEY=disposable-marketing-fixture-encryption-secret \
  "$binary" >"$fixture_dir/api.log" 2>&1 &
api_pid=$!
wait "$api_pid"
