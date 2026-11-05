#!/usr/bin/env bash
# Run the real, guarded email/CRM proof with a new local PostgreSQL cluster.
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: api/scripts/verify-email-backend.sh [--port PORT] [--pg-bin DIRECTORY]

Creates and stops its own disposable PostgreSQL cluster; retains its data and
logs for inspection. Default PostgreSQL port: 55439. The proof also owns local
provider ports 19020, 19021, 19025 and 19026. All must be available.

Requires PostgreSQL with pg_stat_statements and the pinned Rust toolchain and
dependencies already installed. Cargo runs offline. No .env file is sourced.
USAGE
}
fail() {
  printf 'Error: %s\n' "$*" >&2
  exit 2
}

pg_port=55439
pg_bin=
while (($#)); do
  case "$1" in
    --port)
      (($# >= 2)) || fail '--port requires a value'
      pg_port=$2
      shift 2
      ;;
    --pg-bin)
      (($# >= 2)) || fail '--pg-bin requires a directory'
      pg_bin=$2
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *) fail "Unknown argument: $1" ;;
  esac
done
[[ "$pg_port" =~ ^[1-9][0-9]{3,4}$ ]] || fail 'Use a port between 1024 and 65535'
((pg_port >= 1024 && pg_port <= 65535)) || fail 'Use a port between 1024 and 65535'
case "$pg_port" in
  19020|19021|19025|19026) fail 'PostgreSQL cannot use a provider fixture port' ;;
esac
if [[ -n "$pg_bin" ]]; then
  [[ "$pg_bin" = /* && -d "$pg_bin" ]] || fail '--pg-bin must be an existing absolute directory'
  for tool in initdb pg_ctl createdb psql; do
    [[ -x "$pg_bin/$tool" ]] || fail "Missing executable: $pg_bin/$tool"
  done
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
api_dir=$(cd -- "$script_dir/.." && pwd)
fixture_path=${PATH:?PATH is required}
if [[ -n "$pg_bin" ]]; then
  fixture_path="$pg_bin:$fixture_path"
fi
fixture_home=${HOME:?HOME is required for the installed Rust toolchain}
clean_env=(env -i "PATH=$fixture_path" "HOME=$fixture_home")
if [[ -n "${CARGO_HOME:-}" ]]; then
  clean_env+=("CARGO_HOME=$CARGO_HOME")
fi
if [[ -n "${RUSTUP_HOME:-}" ]]; then
  clean_env+=("RUSTUP_HOME=$RUSTUP_HOME")
fi
for tool in initdb pg_ctl createdb psql cargo; do
  "${clean_env[@]}" /bin/bash -c "command -v \"\$1\" >/dev/null" _ "$tool" || fail "Missing tool: $tool"
done

umask 077
fixture_dir=$(mktemp -d "${TMPDIR:-/tmp}/revolution-email-proof.XXXXXX")
data_dir="$fixture_dir/postgres"
database_name=admin_move_verify_email_backend
database_url="postgresql://admin_verify_fixture@127.0.0.1:$pg_port/$database_name"
cluster_start_attempted=0
cleanup() {
  local result=$?
  trap - EXIT
  if ((cluster_start_attempted)) && "${clean_env[@]}" pg_ctl -D "$data_dir" status >/dev/null 2>&1; then
    if ! "${clean_env[@]}" pg_ctl -D "$data_dir" -m fast -w -t 30 stop >>"$fixture_dir/postgres-stop.log" 2>&1; then
      printf 'Could not stop the owned cluster; inspect %s\n' "$fixture_dir/postgres-stop.log" >&2
      result=1
    fi
  fi
  printf '\nProof exit: %s. Retained fixture and logs: %s\n' "$result" "$fixture_dir"
  exit "$result"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

printf 'Isolated fixture: %s\n' "$fixture_dir"
: >"$fixture_dir/empty-pgpass"
clean_env+=("PGPASSFILE=$fixture_dir/empty-pgpass")
"${clean_env[@]}" initdb -D "$data_dir" -U admin_verify_fixture --auth-local=trust --auth-host=trust --encoding=UTF8 --locale=C >"$fixture_dir/initdb.log" 2>&1
cat >>"$data_dir/postgresql.conf" <<CONFIG
listen_addresses = '127.0.0.1'
port = $pg_port
unix_socket_directories = ''
shared_preload_libraries = 'pg_stat_statements'
CONFIG
cluster_start_attempted=1
"${clean_env[@]}" pg_ctl -D "$data_dir" -l "$fixture_dir/postgres.log" -w -t 30 start
"${clean_env[@]}" createdb -h 127.0.0.1 -p "$pg_port" -U admin_verify_fixture "$database_name"

# No caller provider credentials, database settings or proxy variables enter
# the test process. The ignored test rejects non-loopback adapters and a DB
# that is not empty, and supplies only .test recipient addresses.
cd -- "$api_dir"
"${clean_env[@]}" \
  CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 SQLX_OFFLINE=true \
  ENVIRONMENT=development \
  "DATABASE_URL=$database_url" "ADMIN_VERIFY_DATABASE_URL=$database_url" \
  ADMIN_VERIFY_PG_STAT_STATEMENTS=1 \
  APP_URL=http://127.0.0.1:5274 \
  JWT_SECRET=only-a-disposable-jwt-fixture-secret-at-least-32-chars \
  MEMBER_INDICATOR_SECRET=only-a-disposable-indicator-fixture-secret \
  MEMBER_LICENSE_SECRET=only-a-disposable-license-fixture-secret \
  CREDENTIALS_ENCRYPTION_KEY=only-a-disposable-credential-fixture-secret \
  REDIS_URL=redis://127.0.0.1:1 \
  MEILISEARCH_HOST=http://127.0.0.1:1 MEILISEARCH_API_KEY=fixture-only \
  R2_ENDPOINT=http://127.0.0.1:1 R2_ACCESS_KEY_ID=fixture-only \
  R2_SECRET_ACCESS_KEY=fixture-only R2_BUCKET=fixture-only \
  R2_PUBLIC_URL=http://127.0.0.1:1/fixture-only \
  STRIPE_SECRET=sk_test_fixture_only STRIPE_WEBHOOK_SECRET=whsec_fixture_only \
  STRIPE_PUBLISHABLE_KEY=pk_test_fixture_only \
  POSTMARK_TOKEN=controlled-adapter-token POSTMARK_FROM_EMAIL=sender@example.test \
  POSTMARK_TEST_ENDPOINT=http://127.0.0.1:19020/email \
  POSTMARK_WEBHOOK_TOKEN=only-a-disposable-webhook-fixture-secret \
  POSTMARK_BROADCAST_STREAM=fixture-broadcast \
  CONNECTION_TEST_BASE_URL=http://127.0.0.1:19021 \
  cargo test --locked --offline --test email_completion_test -- \
  transactional_receipts_newsletter_and_email_jobs_use_real_adapter_acknowledgment \
  --exact --ignored --nocapture --test-threads=1 2>&1 | tee "$fixture_dir/proof.log"

"${clean_env[@]}" psql "$database_url" -v ON_ERROR_STOP=1 -Atc \
  'SELECT COUNT(*), MAX(version) FROM _sqlx_migrations WHERE success' \
  | tee "$fixture_dir/migration-summary.log"
