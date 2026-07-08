# Local development runbook

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.

This is the canonical guide for running the full stack on your laptop, against
local Docker Postgres + Redis + Rust API + native SvelteKit dev server. It
takes ~10 minutes the first time (Rust release build of the API is the long
step), under 30 seconds on subsequent boots.

## Prerequisites

| Tool           | Version                                                               | macOS install                                                |
| -------------- | --------------------------------------------------------------------- | ------------------------------------------------------------ |
| Docker Desktop | ≥ 4.30                                                                | `brew install --cask docker`                                 |
| Node.js        | 24.18.0 LTS (latest LTS "Krypton")                                    | `nvm install 24 && nvm use 24`                               |
| pnpm           | 11.10.0+                                                               | `corepack enable && corepack prepare pnpm@11.10.0 --activate` |
| Rust           | 1.96.0 (only for native cargo dev, not needed if you only use Docker) | `rustup install stable`                                      |

You do **not** need `flyctl` for local dev. (You'll need it later if you
deploy.)

## First-time boot

### 1. Bring up Postgres + Redis + the API container

```bash
docker compose up -d db redis api
```

The first `up` builds the Rust API image — that's the slow part (~5-7 min
cold; subsequent rebuilds are 30-60 s). Watch progress with:

```bash
docker compose logs -f api
```

Wait for `[INFO] axum: Listening on http://0.0.0.0:8080`. Then sanity-check:

```bash
curl http://localhost:8080/health
# → {"status":"healthy","version":"0.1.0","environment":"development"}

curl http://localhost:8080/health/detailed
# → all three services (database, redis, storage) should report healthy
```

If any service reports unhealthy, see [Troubleshooting](#troubleshooting) below.

### 2. Seed your admin user

The dev DB starts empty. Run the seed script with the credentials you want to
log in with:

```bash
./api/scripts/seed-local-admin.sh "you@example.com" "YourStrongPass1!" "Your Name"
```

The script generates an Argon2id hash inside a one-shot Python container and
upserts the user as `super_admin` with email already verified. Output:

```
✓ argon2id hash generated
INSERT 0 1
✓ user upserted: you@example.com (role=super_admin, email_verified)
```

### 3. Install frontend deps

```bash
pnpm install
```

### 4. Start the frontend natively

```bash
pnpm dev
```

This is faster than running the frontend in Docker on macOS (no filesystem
overhead, snappier hot reload). Vite picks up `frontend/.env.local` (created
on first checkout if missing — see [Frontend env](#frontend-env) below) which
points the SvelteKit server at the local API on port 8080.

Vite will print:

```
VITE v7.x.x  ready in NNN ms
➜  Local:   http://localhost:5173/
```

### 5. Log in

Open [http://localhost:5173/auth/login](http://localhost:5173/auth/login) and
sign in with the credentials you used in step 2.

## Subsequent boots

```bash
docker compose up -d        # all backend services
pnpm dev                    # frontend
```

Containers persist across `docker compose down` (volumes survive). To wipe
data and start fresh:

```bash
docker compose down -v      # removes volumes too
```

…then re-run [step 1](#1-bring-up-postgres--redis--the-api-container) onward.

## Frontend env

`frontend/.env.local` (gitignored) pins the dev URLs. If it's missing, copy:

```bash
cat > frontend/.env.local <<'EOF'
VITE_API_URL=http://localhost:8080
VITE_API_BASE_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080
VITE_SITE_URL=http://localhost:5173
API_BASE_URL=http://localhost:8080
BACKEND_URL=http://localhost:8080
EOF
```

`.env.local` overrides `.env.production` for the dev server. To restore prod
URLs (e.g. for testing the proxies against the live API), delete `.env.local`.

## Common workflows

### Inspect the local DB

```bash
docker exec -it rtp-db psql -U rtp -d revolution_trading_pros
```

Useful queries once you're in:

```sql
\dt                                      -- list all tables
SELECT id, email, role, email_verified_at FROM users;
SELECT name, version, applied_on FROM _sqlx_migrations ORDER BY version DESC LIMIT 5;
```

### Re-seed / promote a user

```bash
./api/scripts/seed-local-admin.sh "you@example.com" "NewPassword!" "Your Name"
# Re-running upserts: it'll update the password hash and force role=super_admin.
```

### Run all four test gates locally

```bash
# Frontend (≈1 minute total)
pnpm check
pnpm test:unit
cd frontend && pnpm exec playwright test tests/e2e --project=chromium

# Backend (≈3 minutes cold, seconds warm)
cd ../api
cargo check
cargo test --test utils_test --test stripe_test    # the no-DB tests
```

### Tail server logs

```bash
docker compose logs -f api db redis
```

### Force a Rust rebuild without losing volumes

```bash
docker compose up -d --build api
```

## Troubleshooting

**`docker compose up` hangs on "Building"** — open Docker Desktop → Builds tab.
The Rust release build with the full dependency graph (axum, sqlx, AWS SDK,
tokio) takes 5-7 minutes cold. The "stuck on aws-sdk-s3" feeling is normal.
Use `docker compose --progress=plain up --build api` to see line-by-line
progress in the terminal.

**API container is `unhealthy`** — check logs:

```bash
docker compose logs api
```

Most common cause is the DB not being ready when the API tried to run
migrations. Restart the API:

```bash
docker compose restart api
```

**`POST /api/auth/login` returns 500 "Database error"** — the API can't
talk to Postgres. Check `docker compose ps`; the `db` container should say
`(healthy)`. If not, restart it (`docker compose restart db`).

**`POST /api/auth/login` returns 401 "Invalid credentials"** — the user
isn't seeded, or the password is wrong. Re-run the seed script with the
exact password you'll type at the login form.

**Login page reports "Account requires email verification"** — the seed
script always sets `email_verified_at = NOW()`. If you're seeing this, you're
hitting a different (production) API. Check `frontend/.env.local` exists and
has `VITE_API_URL=http://localhost:8080`.

**`pnpm dev` fails with `Cannot find module 'svelte-click-to-source'`** —
run `pnpm install` from the repo root, not from `frontend/`.

**Port 5432 conflict** — you have a host Postgres running. Either stop it
(`brew services stop postgresql`) or change the host port in
`docker-compose.yml` (`"5432:5432"` → `"5433:5432"`) and update
`DATABASE_URL` in your env if you connect with a GUI.

**Port 5173 in use** — kill the previous Vite dev server:

```bash
lsof -ti:5173 | xargs kill -9
```

**Posts API still hits Fly URL** — your `+page.server.ts` is using the
hardcoded fallback because the env var didn't reach Vite. Make sure
`frontend/.env.local` exists and that you ran `pnpm dev` from the repo root.

## Production differences

What's stubbed in local dev that works in production (when Fly is up):

- **Stripe** is in test mode by default; webhooks won't fire unless you set
  up the Stripe CLI to forward events to `localhost:8080/api/payments/webhook`.
- **R2 storage** is unavailable; image uploads will fail with a clear error.
  CMS asset list works (read-only) against an empty bucket.
- **Bunny video streaming** is unavailable; video lessons won't play.
- **Postmark email** is unavailable; verification / reset emails are not
  sent. Verification token gets logged to API stdout — copy from
  `docker compose logs api` if you need to test the verify flow.
- **Meilisearch** is unavailable; full-text search returns 503.

All of these are configurable via env vars in `docker-compose.yml`.

## Running Rust integration tests locally

Rust integration tests at `api/tests/` require a Postgres `test` role with a
`test_db` database. To provision that alongside the existing dev DB, use the
test overlay file:

```sh
docker compose -f docker-compose.yml -f docker-compose.test.yml up db -d
# then run integration tests
cargo test --manifest-path api/Cargo.toml --all-targets
```

> **One-time setup:** `docker-entrypoint-initdb.d` scripts only run when the
> Postgres data volume is first created. If you already have a running dev
> volume you will need to `docker compose down -v` (wipes all data) and then
> boot with the overlay, or manually run the SQL in the script:
>
> ```sql
> CREATE ROLE test WITH LOGIN PASSWORD 'test';
> CREATE DATABASE test_db OWNER test;
> GRANT ALL PRIVILEGES ON DATABASE test_db TO test;
> ```

The no-DB tests (per CLAUDE.md) are always safe to run without Docker:

```sh
cd api && cargo test --test utils_test --test stripe_test
```

## Deploying

See [`docs/setup/DEPLOYMENT_GUIDE.md`](../setup/DEPLOYMENT_GUIDE.md) for the
production deploy story (Fly.io API + Cloudflare Pages frontend + R2/Bunny
config).
