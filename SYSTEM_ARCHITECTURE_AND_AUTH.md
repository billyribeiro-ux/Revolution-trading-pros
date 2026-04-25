# System Architecture & Authentication Reference

**Last verified against source: 2026-04-25, commit `66040b420` (post-pnpm-migration).**
**Read this first when picking the project back up.** Everything below is grounded
in the actual code at this commit; if a section disagrees with what you find, the
code wins and this doc needs updating.

---

## 1. Stack at a glance

| Layer | Technology | Hosting | Path in repo |
|-------|-----------|---------|--------------|
| Frontend | SvelteKit 2 + Svelte 5 (runes) + Vite 7 | Cloudflare Pages | [`frontend/`](frontend/) |
| Backend API | Rust + Axum 0.7 | Fly.io | [`api/`](api/) |
| Workspace package mgr | pnpm 10.33.2 | — | root [`pnpm-workspace.yaml`](pnpm-workspace.yaml) |
| Database | PostgreSQL (Fly.io managed) | Fly.io | accessed via `sqlx` 0.8 |
| Cache + sessions / rate limit / blacklist | Redis (Upstash) | Upstash | accessed via `redis` 0.27 |
| Object storage | Cloudflare R2 (S3-compatible) | Cloudflare | `aws-sdk-s3` 1.65 |
| Search | Meilisearch | Meilisearch Cloud | `meilisearch` SDK |
| Email | Postmark | Postmark | direct HTTPS |
| Payments | Stripe + PayPal | external | `stripe` Rust + `@stripe/stripe-js` on web |
| Video / large file CDN | Bunny.net (Stream + Storage) | Bunny | direct HTTPS |
| Realtime | Axum WebSockets + Server-Sent Events | same Fly.io app | `routes/websocket.rs`, `routes/realtime.rs` |
| Error tracking | Sentry | Sentry SaaS | `@sentry/sveltekit` (frontend) |

The two ends communicate over HTTPS only. The frontend never embeds API
credentials at build time — it talks to the API through SvelteKit `+server.ts`
proxy routes ([§4](#4-frontend--backend-handshake)) that attach cookies and
forward bodies.

### Public service URLs

| Purpose | URL |
|---------|-----|
| Production frontend | `https://revolution-trading-pros.pages.dev` |
| Production API | `https://revolution-trading-pros-api.fly.dev` |
| WebSocket | `wss://revolution-trading-pros-api.fly.dev` |
| R2 public bucket | `https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev` |

These are pinned in [`frontend/wrangler.toml`](frontend/wrangler.toml) and
[`.env.example`](.env.example).

---

## 2. Backend (Rust API) layout

Entry point: [`api/src/main.rs`](api/src/main.rs) → router composition in
[`api/src/lib.rs`](api/src/lib.rs).

```
api/src/
├── routes/         77 route modules, each owns one Axum sub-router
│   ├── auth.rs     email/password, JWT issue/refresh, password reset, MFA
│   ├── oauth.rs    Google + Apple sign-in (server-driven OAuth 2.0 / OIDC)
│   ├── admin.rs    + admin_*.rs — admin-only endpoints (auth + role-gated)
│   ├── cms_v2.rs   + cms_*.rs — CMS endpoints (assets, pages, drafts, scheduling)
│   ├── trading_rooms.rs, room_*.rs   live-room features
│   ├── members.rs, indicators.rs, courses.rs, payments.rs, …
│   ├── websocket.rs    real-time push
│   └── realtime.rs     SSE channels
├── services/       business logic — keep handlers thin
│   ├── mfa.rs              TOTP (RFC 6238) + backup codes
│   ├── stripe.rs           Stripe API client + webhook verification
│   ├── email.rs            Postmark wrapper
│   ├── storage.rs / bunny.rs   R2 + Bunny abstractions
│   ├── redis.rs            shared Redis pool + helpers
│   ├── rate_limit.rs       sliding window in Redis
│   ├── search.rs           Meilisearch indexing + query
│   ├── subscription_service.rs / order_service.rs
│   └── …
├── middleware/
│   ├── auth.rs             Bearer JWT verification + revocation check
│   ├── admin.rs            role check (runs after auth)
│   ├── content_type.rs     enforce JSON / multipart
│   └── validation.rs       request body validation hooks
├── models/         sqlx structs + DTOs
├── domain/         pure types (no DB), enums, value objects
├── db/             pool + migrations runner
├── monitoring/     metrics, health, tracing wiring
└── queue/          background job dispatch
```

The `routes::router()` functions return an `axum::Router<AppState>`. They are
composed in `lib.rs` and mounted under `/api`. CORS, compression, tracing, and
the security-header layer wrap the whole router via `tower-http`.

### Datastores reached from the API

| Store | Used for |
|-------|----------|
| PostgreSQL | source of truth: users, content, orders, audit log, MFA secrets |
| Redis (Upstash) | sessions, rate limits, JWT revocation list, real-time room state |
| R2 | uploaded media, CMS assets, exports |
| Bunny Stream + Storage | course videos, large downloads |
| Meilisearch | full-text search across CMS, indicators, posts |

---

## 3. Frontend (SvelteKit) layout

Entry: [`frontend/src/hooks.server.ts`](frontend/src/hooks.server.ts) (auth
gate) and [`frontend/src/routes/+layout.server.ts`](frontend/src/routes/+layout.server.ts)
(initial user hydration).

```
frontend/src/
├── routes/
│   ├── +layout.server.ts           load user from locals → page data
│   ├── auth/                       login / signup / OAuth callback UI
│   │   └── callback/+page.svelte   handles Google + Apple return
│   ├── api/                        SvelteKit endpoints — server-side proxy layer
│   │   ├── auth/                   13 endpoints: login, register, refresh, me,
│   │   │                            forgot-password, reset-password, set-session,
│   │   │                            resend-verification, google[+/callback],
│   │   │                            apple[+/callback]
│   │   └── (other proxies for posts, payments, etc.)
│   ├── dashboard/, admin/, account/, checkout/, trading-room/   protected
│   └── (marketing routes)                                        public
├── lib/
│   ├── components/auth/    LoginForm.svelte, SocialLoginButtons.svelte,
│   │                        TestimonialCarousel.svelte, Scene3D.svelte, …
│   ├── components/ui/      shadcn-svelte wrappers (card, dialog, dropdown-menu,
│   │                        select, table, …) — all Svelte 5 idiomatic
│   ├── stores/             $state-based stores (subscriptions, behavior,
│   │                        analytics, audit log, widgets, …)
│   ├── server/             code that only runs server-side (token utils,
│   │                        backend HTTP client)
│   └── utils/              sanitization (DOMPurify), validation, formatting
└── hooks.server.ts         protected-route gate, security headers, CORS
```

### Build / dev commands (run from `frontend/`)

| Command | Purpose |
|---------|---------|
| `pnpm dev` | Vite dev server on `:5173` |
| `pnpm build` | Production build (`.svelte-kit/cloudflare`) |
| `pnpm check` | `svelte-kit sync && svelte-check` (typecheck) |
| `pnpm test:unit` | Vitest |
| `pnpm exec playwright test tests/e2e --project=chromium` | E2E |

---

## 4. Frontend ↔ backend handshake

**Pattern: server-side proxy.** The Svelte client never calls the Fly API
directly. Instead it calls a same-origin SvelteKit endpoint
(`/api/auth/login`, etc.) that lives at
[`frontend/src/routes/api/`](frontend/src/routes/api/). That endpoint forwards
the request to the Rust API and, on success, sets `httpOnly` cookies.

Reasons documented in the code:

1. Avoids CORB / CORS friction on Cloudflare Pages.
2. Keeps tokens out of JS-accessible storage — the frontend bundle never
   touches `Authorization` headers.
3. Lets `hooks.server.ts` apply security headers + rate limiting uniformly.

---

## 5. Authentication & authorization flow

### 5.1 Token strategy

| Token | Issued by | Stored as | Lifetime | Purpose |
|-------|-----------|-----------|----------|---------|
| Access JWT | API `POST /api/auth/login`, `/refresh`, OAuth callbacks | `httpOnly` cookie `rtp_access_token` (`SameSite=Lax`, `Secure` in prod) | `expires_in` from API (~1h default) | Bearer to API |
| Refresh JWT | API on login + every refresh | `httpOnly` cookie `rtp_refresh_token` | 30 days | Mint new access tokens |
| MFA-pending JWT | API when MFA is required mid-login | returned in body, **not** set as cookie | short | Tied to a single MFA verification |

JWT signing is HMAC, secret in `JWT_SECRET`. Library: `jsonwebtoken` 9.2 on the
API side. Verification helper: [`api/src/middleware/auth.rs`](api/src/middleware/auth.rs).

### 5.2 Email + password flow

```
Browser  →  POST /api/auth/login  (frontend SvelteKit endpoint)
             ↓ proxy
            POST https://revolution-trading-pros-api.fly.dev/api/auth/login
             ↓ Rust handler login() in api/src/routes/auth.rs:465
                - argon2 compare against users.password_hash
                - on bcrypt-prefixed legacy hashes, fall back to bcrypt
                  (Laravel-compat) and rehash with argon2 on success
                - issue access JWT + refresh JWT
                - if user.mfa_enabled: return 200 with mfa_required=true,
                  no cookies; client redirects to /auth/mfa
            ← 200 { token, refresh_token, expires_in, user }
        ←   SvelteKit endpoint sets:
              rtp_access_token  (httpOnly, Lax, maxAge=expires_in)
              rtp_refresh_token (httpOnly, Lax, maxAge=30d)
            returns user JSON to client
Browser keeps no token in JS; subsequent navigation hits hooks.server.ts which
reads the access cookie and populates event.locals.user.
```

Endpoints involved (see also [§5.7](#57-endpoint-map)):

- Frontend proxy: [`frontend/src/routes/api/auth/login/+server.ts`](frontend/src/routes/api/auth/login/+server.ts)
- Rust handler: `login` at [`api/src/routes/auth.rs:465`](api/src/routes/auth.rs#L465)

### 5.3 Registration + email verification

1. `POST /api/auth/register` — argon2-hashes the password, inserts the user,
   queues a Postmark verification email (token is a signed JWT scoped to
   `verify_email`).
2. User clicks the link → `GET /api/auth/verify-email?token=…` →
   `verify_email` handler at [`api/src/routes/auth.rs:229`](api/src/routes/auth.rs#L229)
   marks `users.email_verified_at`.
3. `POST /api/auth/resend-verification` re-mints the JWT and emails it.

### 5.4 Token refresh

1. Frontend `+server.ts` proxy detects 401 on a downstream call.
2. Calls `POST /api/auth/refresh` with the `rtp_refresh_token` cookie.
3. Rust `refresh` ([`auth.rs:719`](api/src/routes/auth.rs#L719)) verifies the
   refresh JWT against the **revocation set in Redis**, rotates it, and
   returns a new pair.
4. `hooks.server.ts` writes the new access cookie (visible at
   [`frontend/src/hooks.server.ts`](frontend/src/hooks.server.ts) around
   line 189) and continues the original request.

### 5.5 Logout

- `POST /api/auth/logout` — adds the access JWT's hash to the Redis blacklist
  (TTL = remaining JWT lifetime), clears the refresh chain.
- `POST /api/auth/logout-all` — invalidates **every** active session for the
  user (revokes all outstanding refresh tokens). Mounted on a separate
  `logout_router()` so it can require fresh auth.
- The frontend always pairs the API call with `cookies.delete('rtp_access_token')`
  and `cookies.delete('rtp_refresh_token')`.

### 5.6 OAuth (Google + Apple)

Server-driven OAuth 2.0 / OIDC flow — the Rust API is the Relying Party. PKCE
code challenges are generated server-side; nothing leaks into the browser.

- `GET /api/oauth/google` — builds an authorize URL with PKCE + state +
  nonce, sets short-lived signed cookies for state/PKCE verifier, 302s to
  Google.
- `GET /api/oauth/google/callback` — verifies state, exchanges code for
  tokens, validates the ID token JWT against Google's JWKS, upserts the user
  by `email`, and issues the same access + refresh JWT pair as
  email/password login.
- `GET /api/oauth/apple` + `POST /api/oauth/apple/callback` — same pattern.
  Apple posts the form to the callback (form-post mode), so the callback is
  POST while Google's is GET.
- Frontend trampolines: `/auth/callback/+page.svelte` handles the user-facing
  return, and the SvelteKit `api/auth/google/+server.ts` /
  `api/auth/apple/+server.ts` endpoints are thin proxies that match the
  email/password proxy shape — they receive the API's token pair and set the
  same `rtp_*` cookies.

Routes:
- Rust: [`api/src/routes/oauth.rs`](api/src/routes/oauth.rs)
- Frontend proxies: [`frontend/src/routes/api/auth/google/`](frontend/src/routes/api/auth/google/),
  [`frontend/src/routes/api/auth/apple/`](frontend/src/routes/api/auth/apple/)

### 5.7 Endpoint map

| Method | Path | Purpose | Where |
|--------|------|---------|-------|
| POST | `/api/auth/register` | create account | `auth.rs:62` |
| POST | `/api/auth/login` | password login → JWT pair | `auth.rs:465` |
| POST | `/api/auth/refresh` | rotate refresh + access JWT | `auth.rs:719` |
| GET  | `/api/auth/me` | current user (requires Bearer) | `auth.rs:787` |
| POST | `/api/auth/logout` | revoke current session | `auth.rs:809` |
| POST | `/api/auth/logout-all` | revoke every session | `auth.rs:866` |
| POST | `/api/auth/forgot-password` | email a reset token | `auth.rs` |
| POST | `/api/auth/reset-password` | consume reset token | `auth.rs` |
| GET  | `/api/auth/verify-email` | confirm email | `auth.rs:229` |
| POST | `/api/auth/resend-verification` | re-issue verify email | `auth.rs` |
| GET  | `/api/oauth/google` | start Google flow | `oauth.rs` |
| GET  | `/api/oauth/google/callback` | Google return | `oauth.rs` |
| GET  | `/api/oauth/apple` | start Apple flow | `oauth.rs` |
| POST | `/api/oauth/apple/callback` | Apple return (form_post) | `oauth.rs` |

Frontend mirror (each is a thin proxy):

```
frontend/src/routes/api/auth/{login,register,refresh,me,forgot-password,
                              reset-password,resend-verification,set-session,
                              google[+/callback],apple[+/callback]}/+server.ts
```

### 5.8 Multi-factor auth (TOTP)

Implemented in [`api/src/services/mfa.rs`](api/src/services/mfa.rs).

- RFC 6238 TOTP, 30-second period, 6 digits, HMAC-SHA1.
- Secret stored encrypted in `users.totp_secret`; verified via
  `totp_verified_at`.
- Setup flow: `setup_mfa(user_id, email)` returns a `MfaSetupResponse` with
  the otpauth URL (so the frontend can render a QR via
  [`api/src/routes/auth.rs`](api/src/routes/auth.rs)) plus 10 single-use
  backup codes (Argon2-hashed before storage).
- During login, when `users.mfa_enabled` is true the API short-circuits and
  returns `mfa_required: true` instead of the JWT pair. The frontend then
  prompts for a TOTP code or backup code and POSTs to `/api/auth/mfa/verify`
  (route lives next to `auth.rs`); on success the API issues the standard
  JWT pair and the proxy sets cookies as usual.

### 5.9 Route protection (frontend)

Implemented in [`frontend/src/hooks.server.ts`](frontend/src/hooks.server.ts).

```ts
const PROTECTED_ROUTES = ['/dashboard', '/account', '/checkout',
                          '/trading-room', '/admin'];
```

For every request:
1. Read `rtp_access_token` cookie (and `Authorization` header for direct API
   hits).
2. If present, validate via API (`GET /api/auth/me`) and populate
   `event.locals.user`.
3. If absent or invalid, attempt silent refresh using `rtp_refresh_token`.
4. If the request path matches a protected prefix and there is still no user
   after step 3, redirect to `/auth/login`.

Admin pages additionally check `event.locals.user.role === 'admin'` inside
`+layout.server.ts`. Same gate is enforced on the API side via the
`middleware::admin::admin_only` layer mounted on `admin_*` routers.

### 5.10 Route protection (backend)

Two layers in [`api/src/middleware/`](api/src/middleware/):

1. **`auth.rs`** — extracts `Authorization: Bearer <jwt>`, verifies signature
   + expiry, hashes the token and checks the **Redis revocation list**, then
   inserts the resolved `User` into request extensions. Routes typed as
   `async fn handler(user: User, …)` get auth automatically by virtue of the
   `FromRequestParts` impl.
2. **`admin.rs`** — runs after `auth` and rejects any non-admin user with
   `403`.

JWT revocation: `auth.rs` hashes the raw JWT with SHA-256 and looks up the
hex digest in a Redis set (`jwt:blacklist:*` keys with TTL = remaining JWT
lifetime). Logout adds entries; refresh rotation also adds the old refresh
token to a parallel set.

---

## 6. Environment variables

Master template: [`.env.example`](.env.example). Backend reads from `api/.env`
and the frontend uses `VITE_*` prefixed vars at build time.

### Required for the API to boot

```
DATABASE_URL=postgres://…
REDIS_URL=rediss://…
JWT_SECRET=<>=32 chars>
JWT_EXPIRES_IN=24                       # hours
R2_ENDPOINT, R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY, R2_BUCKET, R2_PUBLIC_URL
STRIPE_SECRET, STRIPE_PUBLISHABLE_KEY, STRIPE_WEBHOOK_SECRET
POSTMARK_TOKEN, FROM_EMAIL
CORS_ORIGINS=https://revolution-trading-pros.pages.dev,http://localhost:5173
MEILISEARCH_HOST, MEILISEARCH_API_KEY
BUNNY_STREAM_LIBRARY_ID, BUNNY_STREAM_API_KEY,
BUNNY_STORAGE_ZONE, BUNNY_STORAGE_API_KEY,
BUNNY_STORAGE_HOSTNAME, BUNNY_CDN_URL
```

OAuth + MFA additions also live in `.env.example` (Google / Apple client
IDs and secrets, optional Sentry DSN, etc.).

### Required for the frontend

```
VITE_API_URL=https://revolution-trading-pros-api.fly.dev
VITE_API_BASE_URL=…    # same as VITE_API_URL
VITE_WS_URL=wss://…
VITE_CDN_URL=https://pub-….r2.dev
VITE_SITE_URL=…
VITE_SITE_NAME=…
```

The `+server.ts` proxies hardcode `https://revolution-trading-pros-api.fly.dev`
as the fallback when `env.API_BASE_URL` is unset — this is fine for production
but means a custom local API needs `API_BASE_URL` set in
`frontend/.env`.

---

## 7. Deployment

### Frontend (Cloudflare Pages)

- **Source of truth:** Cloudflare Pages dashboard's Git integration.
  Production branch: `main`. Push → auto-deploy.
- **Build settings (in dashboard):**
  - Root directory: `frontend`
  - Build command: `pnpm install && pnpm build` (must use pnpm — the
    repo's `npm ci` workflow files are stale; see follow-ups in
    [`REPO_STATE_2026-04-25.md`](REPO_STATE_2026-04-25.md))
  - Build output directory: `.svelte-kit/cloudflare`
- **Adapter:** `@sveltejs/adapter-cloudflare` (configured in `svelte.config.js`).
- A **parallel** GitHub Actions workflow exists at
  [`.github/workflows/deploy-cloudflare.yml`](.github/workflows/deploy-cloudflare.yml).
  It still uses `npm ci` + `npm run build` and will fail until migrated to
  pnpm. The Cloudflare dashboard integration deploys independently of this
  workflow.

### Backend (Fly.io)

- **Source of truth:** [`api/fly.toml`](api/fly.toml) +
  [`api/Dockerfile`](api/Dockerfile).
- Database (PostgreSQL) is a separate Fly app reached via Flycast
  (`revolution-db.flycast`).
- Deploy is currently manual: `fly deploy` from `api/` (or via the
  `deploy-fly.yml` workflow when configured).
- Migrations live in [`api/migrations/`](api/migrations/) and run via
  [`api/run-migrations.js`](api/run-migrations.js) on app start.

---

## 8. Security posture (highlights)

- **Passwords:** argon2id (current), bcrypt accepted for legacy migration,
  rehashed to argon2 on next successful login.
- **Tokens:** httpOnly + SameSite=Lax cookies; never exposed to JS; rotation
  on every refresh; revocation list in Redis; `subtle::ConstantTimeEq` used
  for sensitive comparisons.
- **MFA:** TOTP RFC 6238 + Argon2-hashed backup codes.
- **CSRF:** the proxy pattern + `SameSite=Lax` cookies eliminates the typical
  cross-origin form vector. State-changing API calls additionally require
  the `Authorization` header (or the cookie) and `Content-Type: application/json`,
  enforced by `middleware::content_type`.
- **Rate limiting:** Redis sliding-window in `services::rate_limit`, applied
  per-route in `lib.rs`. Defaults are tighter on `/auth/*` and `/payments/*`.
- **CORS:** `tower-http::cors` with an explicit allowlist from `CORS_ORIGINS`.
  No wildcard.
- **CSP / security headers:** added by `hooks.server.ts` and replicated in
  `frontend/static/_headers` for static assets. Allowlists Stripe, Google
  OAuth/GA4, Bunny, PayPal, R2 CDN, Iconify.
- **Sanitization:** any user-supplied HTML rendered via `{@html}` goes through
  `frontend/src/lib/utils/sanitization.ts` (DOMPurify with a strict policy).
- **Audit log:** every admin write goes through `services::cms_audit` and is
  also persisted in PostgreSQL `audit_log`.

---

## 9. What to read next

1. [`REPO_STATE_2026-04-25.md`](REPO_STATE_2026-04-25.md) — current commit's
   verification gates and known follow-ups.
2. [`DEPLOYMENT_GUIDE.md`](DEPLOYMENT_GUIDE.md) — step-by-step deploy
   instructions (older but still mostly accurate).
3. [`api/RUNNING_MIGRATIONS.md`](api/RUNNING_MIGRATIONS.md) — DB migration
   procedure.
4. [`CLAUDE.md`](CLAUDE.md) — repo-specific Svelte MCP usage rules.
5. [`api/src/routes/auth.rs`](api/src/routes/auth.rs) and
   [`frontend/src/hooks.server.ts`](frontend/src/hooks.server.ts) — when in
   doubt, the auth code is short enough to read end-to-end.
