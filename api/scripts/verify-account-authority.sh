#!/usr/bin/env bash
# Own one isolated Redis container, then invoke the existing isolated PostgreSQL proof.
set -euo pipefail
fail() { printf 'Error: %s\n' "$*" >&2; exit 2; }
redis_port=16389
pg_args=()
while (($#)); do
  case "$1" in
    --redis-port) (($# >= 2)) || fail 'Missing --redis-port'; redis_port=$2; shift 2 ;;
    --port|--pg-bin) (($# >= 2)) || fail "Missing $1"; pg_args+=("$1" "$2"); shift 2 ;;
    --help|-h) printf 'Usage: api/scripts/verify-account-authority.sh [--redis-port PORT] [--port PGPORT] [--pg-bin DIRECTORY]\nUses only the pinned cached Redis image, never pulls. Retains all owned fixture logs.\n'; exit 0 ;;
    *) fail "Unknown argument: $1" ;;
  esac
done
[[ "$redis_port" =~ ^[1-9][0-9]{3,4}$ ]] || fail 'Redis port must be between 1024 and 65535'
((redis_port >= 1024 && redis_port <= 65535)) || fail 'Invalid Redis port'
case "$redis_port" in 16388|19020|19021|19025|19026) fail 'Refuse existing browser Redis/provider ports' ;; esac
command -v docker >/dev/null || fail 'Docker is required for the owned real Redis fixture'
if [[ -n "${DOCKER_HOST:-}" ]]; then
  daemon_host=$DOCKER_HOST
else
  daemon_host=$(docker context inspect "$(docker context show)" --format '{{.Endpoints.docker.Host}}')
fi
[[ "$daemon_host" = unix:///* ]] || fail 'This fixture requires a local Unix-socket Docker daemon'

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
[[ -x "$script_dir/verify-email-backend.sh" ]] || fail 'Missing executable PostgreSQL fixture runner'
image=sha256:21090af93f8324db2f0f0f4f38a7c60369d47d917e000c3423d89d2ae2883537
docker image inspect "$image" --format '{{.Id}}' >/dev/null || fail 'Pinned cached Redis image is unavailable; no pull was attempted'
umask 077
fixture_dir=$(mktemp -d "${TMPDIR:-/tmp}/revolution-account-redis.XXXXXX")
# The random temp basename supplies a unique container name; the independent UUID labels ownership.
fixture_token=$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')
[[ "$fixture_token" =~ ^[0-9a-f]{32}$ ]] || fail 'Could not create a fixture ownership token'
container_name="revolution-account-proof-$fixture_token"
container_id=
cleanup() {
  local result=$?
  trap - EXIT
  if [[ -z "$container_id" && -r "$fixture_dir/redis.cid" ]]; then container_id=$(cat "$fixture_dir/redis.cid"); fi
  if [[ -n "$container_id" ]]; then
    local identity
    identity=$(docker inspect "$container_id" --format '{{.Id}}|{{.Name}}|{{index .Config.Labels "revolution.fixture.owner"}}|{{range (index .HostConfig.PortBindings "6379/tcp")}}{{.HostIp}}:{{.HostPort}}{{end}}' 2>/dev/null || true)
    if [[ "$identity" = "$container_id|/$container_name|$fixture_token|127.0.0.1:$redis_port" ]]; then
      docker logs "$container_id" >"$fixture_dir/redis.log" 2>&1 || true
      docker rm --force "$container_id" >>"$fixture_dir/cleanup.log" 2>&1 || result=1
    else
      printf 'Refused Redis cleanup: owned container identity/binding changed. Inspect %s\n' "$fixture_dir" >&2
      result=1
    fi
  fi
  printf 'Account fixture exit: %s. Retained Redis logs: %s\n' "$result" "$fixture_dir"
  exit "$result"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
printf 'Owned Redis fixture: %s\n' "$fixture_dir"
# Docker itself atomically refuses an occupied host port. No existing container is stopped.
container_id=$(docker run --detach --pull=never --cidfile "$fixture_dir/redis.cid" --name "$container_name" \
  --label "revolution.fixture.owner=$fixture_token" --network bridge \
  --publish "127.0.0.1:$redis_port:6379" "$image" \
  redis-server --save '' --appendonly no 2>"$fixture_dir/start-error.log")
[[ "$container_id" =~ ^[0-9a-f]{64}$ ]] || fail 'Docker did not return an owned container ID'
ready=0
for attempt in {1..30}; do
  if [[ "$(docker exec "$container_id" redis-cli ping 2>/dev/null || true)" = PONG ]]; then ready=1; break; fi
  sleep 0.1
done
((ready)) || fail 'Owned Redis did not become ready'
docker exec "$container_id" redis-cli SET revolution:fixture:owner "$fixture_token" >/dev/null
printf '%s\n' "$container_id|$container_name|127.0.0.1:$redis_port" >"$fixture_dir/ownership.log"
"$script_dir/verify-email-backend.sh" "${pg_args[@]}" --owned-redis-port "$redis_port" --owned-redis-token "$fixture_token" \
  2>&1 | tee "$fixture_dir/account-proof.log"
