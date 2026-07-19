# Environment variables — canonical matrix

This is the single source of truth for what variables exist, where they're
read, and what happens when they're missing. The four `.env*` files in the
repo are templates; this doc is the contract.

> Files touched by this matrix:
> - `.env.example` (root, informational)
> - `api/.env.example`
> - `frontend/.env.example`
> - `frontend/.env.production` (committed)
> - `frontend/.env.local` (gitignored, dev override)

## Reading discipline

| Side | How to read | What it means |
|------|-------------|---------------|
| Rust API | `std::env::var("X")` in `api/src/config/mod.rs` and friends | Read at boot; missing values are either fatal (REQUIRED) or fall back (OPTIONAL). |
| SvelteKit server | `import { env } from '$env/dynamic/private'` | Read per request; never bundled into the client. Use this for `API_BASE_URL`, `JWT_SECRET`, anything sensitive. |
| SvelteKit static private | `import { X } from '$env/static/private'` | Read at build time; baked into server bundle. Use for values that don't change at runtime. |
| SvelteKit dynamic public | `import { env } from '$env/dynamic/public'` (must be `PUBLIC_` prefixed) | Per-request, browser-visible. |
| SvelteKit static public | `import.meta.env.VITE_X` | Build-time inlined into the client bundle. **Always** browser-visible. Never put a secret here. |

## Backend (Rust API)

Set in `api/.env` for local, <deploy target TBD — set via your platform secrets manager> for production.

| Var | Required? | Default | Purpose |
|-----|-----------|---------|---------|
| `PORT` | optional | 8080 | Listen port |
| `ENVIRONMENT` | optional | `production` | Tags log lines, enables dev-only paths when `development` |
| `APP_URL` | optional | `https://revolution-trading-pros.pages.dev` | Used in transactional emails as link host |
| `DATABASE_URL` | **required** | — | Postgres connection string. sqlx 0.9, native-tls. |
| `REDIS_URL` | **required** | — | Sessions, rate limit, JWT blacklist, hot-content cache |
| `JWT_SECRET` | **required** | — | HS256 signing secret. ≥ 32 chars. |
| `JWT_EXPIRES_IN` | optional | 24 | Access-token TTL in hours |
| `DEVELOPER_EMAILS` | optional | (empty) | Comma-separated. These users get email-verify bypass at login. |
| `SUPERADMIN_EMAILS` | optional | (empty) | Comma-separated. Treated as super-admin role regardless of DB role. |
| `DEVELOPER_MODE` | optional | false | Toggles a few dev-only middlewares |
| `DEVELOPER_BOOTSTRAP_EMAIL` | optional | — | Seed a developer user on first boot (uses the `_HASH` below). For local dev use `api/scripts/seed-local-admin.sh` instead. |
| `DEVELOPER_BOOTSTRAP_NAME` | optional | — | Display name for the bootstrap user |
| `DEVELOPER_BOOTSTRAP_PASSWORD_HASH` | optional | — | Argon2id hash. **Hash, not plaintext.** |
| `GOOGLE_CLIENT_ID` | optional | — | OAuth. Without it, `/api/oauth/google` returns 503. |
| `GOOGLE_CLIENT_SECRET` | optional | — | OAuth |
| `APPLE_CLIENT_ID` | optional | — | Sign in with Apple service ID |
| `APPLE_TEAM_ID` | optional | — | Apple developer team ID |
| `APPLE_KEY_ID` | optional | — | Apple Sign in with Apple key ID |
| `APPLE_PRIVATE_KEY` | optional | — | Contents of the .p8 file. Keep `\n` line breaks intact. |
| `CORS_ORIGINS` | **required** | — | Comma-separated allowlist for `Access-Control-Allow-Origin`. |
| `R2_ENDPOINT` | required for media | — | Cloudflare R2 endpoint |
| `R2_ACCESS_KEY_ID` | required for media | — | R2 access key |
| `R2_SECRET_ACCESS_KEY` | required for media | — | R2 secret |
| `R2_BUCKET` | required for media | `revolution-trading-media` | Bucket name |
| `R2_PUBLIC_URL` | required for media | — | Public-read URL for served assets |
| `STRIPE_SECRET` | required for checkout | — | `sk_test_...` or `sk_live_...` |
| `STRIPE_PUBLISHABLE_KEY` | required for checkout | — | `pk_test_...` or `pk_live_...` (not strictly needed server-side, kept for parity) |
| `STRIPE_WEBHOOK_SECRET` | required for webhooks | — | `whsec_...` |
| `POSTMARK_TOKEN` | required for email | — | Postmark server token |
| `FROM_EMAIL` | required for email | — | Sender address (must be verified in Postmark) |
| `MEILISEARCH_HOST` | optional | — | Without this, search returns 503 |
| `MEILISEARCH_API_KEY` | optional | — | Search master key |
| `BUNNY_STREAM_LIBRARY_ID` | optional | — | Bunny Stream library for course videos |
| `BUNNY_STREAM_API_KEY` | optional | — | Bunny Stream API key |
| `BUNNY_STORAGE_ZONE` | optional | — | Bunny Storage zone for downloads |
| `BUNNY_STORAGE_API_KEY` | optional | — | Bunny Storage API key |
| `BUNNY_STORAGE_HOSTNAME` | optional | — | e.g. `ny.storage.bunnycdn.com` |
| `BUNNY_CDN_URL` | optional | — | Public CDN URL |

## Frontend (SvelteKit)

Set in `frontend/.env.local` (dev), `frontend/.env.production` (committed,
non-secret), or Cloudflare Pages dashboard env vars (prod secrets).

### Browser-visible (`VITE_*`, `PUBLIC_*`)

| Var | Purpose |
|-----|---------|
| `VITE_API_URL` | Browser-visible API URL (e.g. for client-side fetch / WebSocket) |
| `VITE_API_BASE_URL` | Same as `VITE_API_URL`, kept for legacy import sites |
| `VITE_WS_URL` | WebSocket endpoint |
| `VITE_CDN_URL` | Public R2 URL |
| `VITE_SITE_URL` | Canonical site URL (used in OG tags) |
| `VITE_SITE_NAME` | "Revolution Trading Pros" |
| `VITE_SITE_DESCRIPTION` | OG description |
| `VITE_DEFAULT_OG_IMAGE` | Default OG image path |
| `VITE_TWITTER_HANDLE` | `@RevTradingPros` |
| `VITE_FACEBOOK_APP_ID` | Optional |
| `VITE_GTM_ID` / `VITE_GTAG_ID` | Google Tag Manager + GA4 |
| `PUBLIC_GA4_MEASUREMENT_ID` | GA4 ID readable via `$env/dynamic/public` |
| `PUBLIC_META_PIXEL_ID`, `PUBLIC_TIKTOK_PIXEL_ID`, … | Marketing pixels |
| `VITE_SENTRY_DSN` | Public Sentry DSN |
| `VITE_DEVELOPER_EMAILS` / `VITE_SUPERADMIN_EMAILS` | Frontend role-helper allowlists. **The backend enforces the real check.** |
| `VITE_ANALYTICS_DEBUG` | Verbose analytics logging |

### Server-only

Read via `$env/dynamic/private` from `+server.ts`, `+page.server.ts`,
`hooks.server.ts`. Never bundled into the browser.

| Var | Purpose |
|-----|---------|
| `API_BASE_URL` | Server-side API URL (the backend the proxies hit) |
| `BACKEND_URL` | Same. Both names exist; `API_BASE_URL` wins. |

## Local docker-compose

`docker-compose.yml` provides defaults for `DATABASE_URL`, `REDIS_URL`,
`JWT_SECRET`, `CORS_ORIGINS`, `RUST_LOG`. Override any of them via your
shell environment or by writing a `.env` next to `docker-compose.yml`:

```bash
# Example override (root .env)
JWT_SECRET=use-a-stable-secret-so-tokens-survive-restarts
STRIPE_SECRET=sk_test_xxxxx
```

## Common mistakes

- **Putting a secret in `.env.production`** — it's committed; everything
  there is public.
- **Putting a secret in `VITE_X`** — it's bundled into the browser. Use a
  non-`VITE_`/`PUBLIC_` name and read it via `$env/dynamic/private`.
- **Hardcoding a production API URL in a proxy** — the 2026-04-25 audit found
  14 of these. Use `env.API_BASE_URL || env.BACKEND_URL` with the
  `http://localhost:8080` fallback.
- **Forgetting `.env.local`** — the homepage will keep hitting the remote
  production API even though you ran `docker compose up`.
- **Stale `node_modules`** after switching env mode — Vite caches; restart
  `pnpm dev`.

## Where each file is loaded

| File | Loaded by | Persisted to git? |
|------|-----------|-------------------|
| `.env.example` (root) | Documentation only | Yes |
| `api/.env.example` | Documentation only | Yes |
| `api/.env` | `dotenv` from `api/src/config/mod.rs` | No (gitignored) |
| `frontend/.env.example` | Documentation only | Yes |
| `frontend/.env.production` | Vite, when building/serving in production mode | Yes (no secrets) |
| `frontend/.env.local` | Vite, in any mode (overrides others) | No (gitignored) |
| `frontend/.env.[mode]` | Vite, in that mode | Yes/No depending on `.gitignore` |
| API host secrets (deploy target TBD — Fly.io removed 2026-04-28) | API process env in production | Not in repo |
| Cloudflare Pages env | Frontend build + runtime in production | Not in repo |
