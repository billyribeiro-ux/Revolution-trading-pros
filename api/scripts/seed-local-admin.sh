#!/usr/bin/env bash
# Seed (or upsert) a super_admin user into the local Docker Postgres.
#
# Usage:
#   ./api/scripts/seed-local-admin.sh <email> <password> [name]
#
# Requires:
#   - docker compose stack running (`docker compose up -d db redis api`)
#   - the rtp-api container available (we exec into it for argon2 hashing)
#
# This is LOCAL-DEV ONLY. Never run against production. The password lands
# in the SQL string for the duration of one psql call inside the rtp-db
# container — the hash is what's persisted; the plaintext is discarded.

set -euo pipefail

EMAIL="${1:-}"
PASSWORD="${2:-}"
NAME="${3:-Admin}"

if [[ -z "$EMAIL" || -z "$PASSWORD" ]]; then
  echo "usage: $0 <email> <password> [name]" >&2
  exit 1
fi

# Sanity: the API container must be up so we can use its argon2 binary.
# Falls back to a one-shot rust container if rtp-api isn't running.
if docker ps --format '{{.Names}}' | grep -q '^rtp-api$'; then
  HASH_CONTAINER="rtp-api"
  HASH_CMD=(docker exec -i rtp-api ./revolution-api --hash-password)
  # The API binary doesn't expose --hash-password; fall through to the python path below.
fi

# We hash via Python's argon2-cffi inside a one-shot container because that's
# the smallest dependency surface that produces a $argon2id$ string compatible
# with the Rust `argon2` crate's PasswordHash::new() parser.
#
# Parameters match bootstrap_dev.rs (OWASP 2024 for financial apps):
#   memory_cost: 65536 KiB (64 MiB)
#   time_cost:   3 iterations
#   parallelism: 4
#   hash_len:    32 bytes
#   salt_len:    16 bytes (default)

HASH=$(docker run --rm \
  -e RTP_PWD="$PASSWORD" \
  python:3.12-alpine sh -c '
    pip install --quiet argon2-cffi >/dev/null 2>&1
    python - <<EOF
import os
from argon2 import PasswordHasher
from argon2.profiles import RFC_9106_HIGH_MEMORY
ph = PasswordHasher(time_cost=3, memory_cost=65536, parallelism=4, hash_len=32)
print(ph.hash(os.environ["RTP_PWD"]), end="")
EOF
')

if [[ -z "$HASH" || "${HASH:0:10}" != '$argon2id$' ]]; then
  echo "ERROR: argon2 hash generation failed (got: ${HASH:0:30}…)" >&2
  exit 1
fi

echo "✓ argon2id hash generated"

# Upsert into the local Postgres. The users table is created by migration
# 000_bootstrap_users.sql. role = super_admin gives admin + superadmin both.
docker exec -i rtp-db psql -U rtp -d revolution_trading_pros -v ON_ERROR_STOP=1 <<SQL
INSERT INTO users (name, email, password_hash, role, email_verified_at, created_at, updated_at)
VALUES ('$NAME', '$EMAIL', '$HASH', 'super_admin', NOW(), NOW(), NOW())
ON CONFLICT (email) DO UPDATE
SET name          = EXCLUDED.name,
    password_hash = EXCLUDED.password_hash,
    role          = 'super_admin',
    email_verified_at = COALESCE(users.email_verified_at, NOW()),
    updated_at    = NOW();
SQL

echo "✓ user upserted: $EMAIL (role=super_admin, email_verified)"
echo
echo "Login at http://localhost:5173/auth/login with:"
echo "  email:    $EMAIL"
echo "  password: (the one you just provided)"
