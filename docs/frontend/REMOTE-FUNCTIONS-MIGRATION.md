# Remote Functions Migration — Discovery & Inventory

## Date: Feb 8, 2026 | Status: PHASE 1 COMPLETE — AWAITING APPROVAL

---

## CRITICAL PREREQUISITE

Remote Functions are **experimental** in SvelteKit 2.27+. Requires opt-in:

```js
// svelte.config.js
kit: {
  experimental: {
    remoteFunctions: true
  }
},
compilerOptions: {
  experimental: {
    async: true // optional, for await in components
  }
}
```

Current SvelteKit version: `^2.50.2` ✅ (supports remote functions)

---

## ARCHITECTURE OVERVIEW (AS-IS)

```
┌─────────────────────────────────────────────────────────────────┐
│ Browser (Svelte 5 Components)                                   │
│                                                                 │
│  $lib/api/*.ts domain modules                                   │
│  ├── auth.ts        → raw fetch('/api/auth/*')                  │
│  ├── admin.ts       → raw fetch('/api/admin/*')                 │
│  ├── crm.ts         → apiClient (client.svelte.ts)              │
│  ├── members.ts     → apiClient + raw fetch                     │
│  ├── subscriptions  → raw fetch(API_BASE_URL + '/api/*')        │
│  ├── courses.ts     → raw fetch('/api/admin/courses/*')         │
│  ├── watchlist.ts   → local apiFetch('/api/watchlist/*')        │
│  ├── forms.ts       → apiClient                                 │
│  ├── popups.ts      → apiClient                                 │
│  ├── boards.ts      → apiClient                                 │
│  ├── analytics.ts   → apiClient                                 │
│  └── client.ts      → ApiClient class (retries, circuit breaker)│
│                                                                 │
│  +page.svelte files → direct fetch('/api/*') calls              │
│  page.api.ts files  → api.get() from $lib/api/client            │
├─────────────────────────────────────────────────────────────────┤
│ SvelteKit Server                                                │
│                                                                 │
│  src/routes/api/[...path]/+server.ts  ← CATCH-ALL PROXY        │
│    → Proxies ALL /api/* to Axum with retries, circuit breaker   │
│                                                                 │
│  src/routes/api/auth/*/+server.ts     ← DEDICATED AUTH PROXIES  │
│    → login, register, refresh, me, set-session, etc.            │
│                                                                 │
│  src/routes/api/admin/*/+server.ts    ← ADMIN PROXIES           │
│    → courses, schedules, bunny, CRM, email, etc.                │
│                                                                 │
│  src/routes/api/*/+server.ts          ← DOMAIN PROXIES          │
│    → analytics, consent, behavior, favorites, watchlist, etc.   │
│                                                                 │
│  +page.server.ts load functions                                 │
│    → Direct fetch to Axum (env.API_BASE_URL)                    │
│    → $lib/server/watchlist.ts                                   │
├─────────────────────────────────────────────────────────────────┤
│ Rust/Axum Backend (Fly.io)                                      │
│  revolution-trading-pros-api.fly.dev                            │
│  → /api/auth/*, /api/posts/*, /api/products/*, /api/admin/*     │
│  → /api/subscriptions/*, /api/courses/*, /api/contacts/*        │
│  → /api/videos/*, /api/alerts/*, /api/trades/*                  │
│  → /api/room-resources, /api/watchlist, etc.                    │
└─────────────────────────────────────────────────────────────────┘
```

**Key finding:** The entire SvelteKit API layer is essentially a **proxy layer** — every `+server.ts` endpoint forwards requests to Axum. The client-side `$lib/api/*.ts` modules call these SvelteKit proxy endpoints (same-origin), which then forward to Axum. There is **no business logic** in the SvelteKit `+server.ts` endpoints — they are pure proxies with retry/circuit-breaker logic.

---

## A) INTERNAL FLOWS TO CONVERT TO REMOTE FUNCTIONS

### A1. Dashboard — Explosive Swings (HIGH PRIORITY)

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/alerts/{slug}')` | Axum via proxy | `query` | `dashboard/explosive-swings/data.remote.ts` |
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/trade-plans/{slug}')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/stats/{slug}')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/trades/{slug}')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/weekly-video/{slug}')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/page.api.ts` | `api.get('/api/auth/me')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/page.state.svelte.ts` | `fetch('/api/admin/trades/{id}', DELETE)` | Axum via proxy | `command` | `dashboard/explosive-swings/commands.remote.ts` |
| `dashboard/explosive-swings/components/AddTradeModal.svelte` | `fetch('/api/trades/{slug}', POST)` | Axum via proxy | `command` | same |
| `dashboard/explosive-swings/components/ClosePositionModal.svelte` | `fetch('/api/trades/{slug}/{id}', PATCH)` | Axum via proxy | `command` | same |
| `dashboard/explosive-swings/components/UpdatePositionModal.svelte` | `fetch('/api/trades/{slug}/{id}', PATCH)` | Axum via proxy | `command` | same |
| `dashboard/explosive-swings/components/InvalidatePositionModal.svelte` | `fetch('/api/trades/{slug}/{id}', PATCH)` | Axum via proxy | `command` | same |
| `dashboard/explosive-swings/components/VideoUploadModal.svelte` | `fetch('/api/weekly-video/{slug}/upload', POST)` | Axum via proxy | `command` | same |
| `dashboard/explosive-swings/components/TradeEntryModal.svelte` | `fetch('/api/trade-plans/{slug}', POST/PUT)` | Axum via proxy | `form` | `dashboard/explosive-swings/forms.remote.ts` |
| `dashboard/explosive-swings/favorites/+page.svelte` | `fetch('/api/favorites/*')` | Axum via proxy | `query`/`command` | `dashboard/explosive-swings/data.remote.ts` |
| `dashboard/explosive-swings/watchlist/+page.svelte` | `fetch('/api/watchlist/*')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/video-library/+page.svelte` | `fetch('/api/videos/*')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/search/search.state.svelte.ts` | `fetch('/api/alerts/*')` | Axum via proxy | `query` | same |
| `dashboard/explosive-swings/trades/api.ts` | `fetch('/api/trades/*')` | Axum via proxy | `query`/`command` | same |

### A2. Dashboard — SPX Profit Pulse

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `dashboard/spx-profit-pulse/+page.server.ts` | `fetch('/api/dashboard/spx-profit-pulse')` | Axum via proxy | `query` | `dashboard/spx-profit-pulse/data.remote.ts` |
| `dashboard/spx-profit-pulse/alerts/[slug]/+page.server.ts` | `fetch('/api/dashboard/spx-profit-pulse/alerts/{slug}')` | Axum via proxy | `query` | same |

### A3. Dashboard — Other Trading Rooms

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `dashboard/day-trading-room/+page.server.ts` | `fetch(API_BASE_URL + '/api/room-resources')` | Axum direct | `query` | `dashboard/shared/room-data.remote.ts` |
| `dashboard/small-account-mentorship/+page.server.ts` | same pattern | Axum direct | `query` | same |
| `dashboard/swing-trading-room/+page.server.ts` | same pattern | Axum direct | `query` | same |
| `dashboard/high-octane-scanner/+page.server.ts` | same pattern | Axum direct | `query` | same |
| `dashboard/[room_slug]/daily-videos/+page.server.ts` | `fetch(API_BASE_URL + '/api/...')` | Axum direct | `query` | same |
| `dashboard/[room_slug]/learning-center/+page.server.ts` | same | Axum direct | `query` | same |
| `dashboard/[room_slug]/video/[slug]/+page.server.ts` | same | Axum direct | `query` | same |
| `dashboard/[room_slug]/trading-room-archive/+page.server.ts` | same | Axum direct | `query` | same |

### A4. Dashboard — Account

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `dashboard/account/+page.server.ts` | `apiFetch` from `$lib/api/account` | Axum via proxy | `query` | `dashboard/account/data.remote.ts` |
| `dashboard/account/edit-account/+page.server.ts` | `fetch('/api/auth/me', PATCH)` | Axum via proxy | `form` | `dashboard/account/forms.remote.ts` |
| `dashboard/account/edit-address/+page.server.ts` | `fetch('/api/auth/me', PATCH)` | Axum via proxy | `form` | same |
| `dashboard/account/subscriptions/+page.server.ts` | `fetch('/api/subscriptions/my')` | Axum via proxy | `query` | `dashboard/account/data.remote.ts` |
| `dashboard/account/orders/+page.server.ts` | `fetch('/api/checkout/orders')` | Axum via proxy | `query` | same |
| `dashboard/account/payment-methods/+page.server.ts` | `fetch('/api/...')` | Axum via proxy | `query`/`command` | same |
| `dashboard/account/coupons/+page.server.ts` | `fetch('/api/...')` | Axum via proxy | `query` | same |
| `dashboard/account/add-payment-method/+page.svelte` | `fetch('/api/...')` | Axum via proxy | `command` | `dashboard/account/commands.remote.ts` |

### A5. Admin Panel (~40 pages)

| Domain | Files | Current Call | Proposed RF Type | Remote File |
|--------|-------|-------------|-----------------|-------------|
| Users | `admin/users/*.svelte` | `fetch('/api/admin/users/*')` | `query`/`command` | `admin/users/data.remote.ts` |
| Members | `admin/members/*.svelte` | `$lib/api/members` | `query`/`command` | `admin/members/data.remote.ts` |
| Posts/Blog | `admin/blog/*.svelte` | `$lib/api/admin` | `query`/`form`/`command` | `admin/blog/data.remote.ts` |
| Courses | `admin/courses/*.svelte` | `fetch('/api/admin/courses/*')` | `query`/`form`/`command` | `admin/courses/data.remote.ts` |
| CRM | `admin/crm/*.svelte` (15+ pages) | `$lib/api/crm` | `query`/`form`/`command` | `admin/crm/data.remote.ts` |
| Schedules | `admin/schedules/*.svelte` | `fetch('/api/admin/schedules/*')` | `query`/`command` | `admin/schedules/data.remote.ts` |
| Trading Rooms | `admin/trading-rooms/*.svelte` | `fetch('/api/admin/trading-rooms/*')` | `query`/`command` | `admin/trading-rooms/data.remote.ts` |
| Media | `admin/media/*.svelte` | `$lib/api/media` | `query`/`command` | `admin/media/data.remote.ts` |
| Popups | `admin/popups/*.svelte` | `$lib/api/popups` | `query`/`form`/`command` | `admin/popups/data.remote.ts` |
| Forms | `admin/forms/*.svelte` | `$lib/api/forms` | `query`/`form` | `admin/forms/data.remote.ts` |
| SEO | `admin/seo/*.svelte` | `$lib/api/seo` | `query`/`command` | `admin/seo/data.remote.ts` |
| Email | `admin/email/*.svelte` | `$lib/api/email` | `query`/`form`/`command` | `admin/email/data.remote.ts` |
| Analytics | `admin/analytics/*.svelte` | `$lib/api/analytics` | `query` | `admin/analytics/data.remote.ts` |
| Subscriptions | `admin/subscriptions/*.svelte` | `$lib/api/subscriptions` | `query`/`command` | `admin/subscriptions/data.remote.ts` |
| Products | `admin/products/*.svelte` | `$lib/api/admin` | `query`/`form`/`command` | `admin/products/data.remote.ts` |
| Coupons | `admin/coupons/*.svelte` | `$lib/api/coupons` | `query`/`form`/`command` | `admin/coupons/data.remote.ts` |
| Boards | `admin/boards/*.svelte` | `$lib/api/boards` | `query`/`command` | `admin/boards/data.remote.ts` |
| Watchlist | `admin/watchlist/*.svelte` | `$lib/api/watchlist` | `query`/`form`/`command` | `admin/watchlist/data.remote.ts` |
| Videos | `admin/videos/*.svelte` | `$lib/api/video-advanced` | `query`/`command` | `admin/videos/data.remote.ts` |

### A6. Public/User-Facing Pages

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `blog/+page.ts` | `$lib/api/articles` | Axum via proxy | `query` | `blog/data.remote.ts` |
| `blog/[slug]/+page.ts` | `$lib/api/articles` | Axum via proxy | `query` | same |
| `cart/+page.svelte` | `$lib/api/cart` | Axum via proxy | `query`/`command` | `cart/data.remote.ts` |
| `checkout/+page.ts` | `$lib/api/checkout` | Axum via proxy | `query`/`form` | `checkout/data.remote.ts` |
| `my/courses/+page.svelte` | `fetch('/api/courses/*')` | Axum via proxy | `query` | `my/data.remote.ts` |
| `my/indicators/+page.svelte` | `fetch('/api/indicators/*')` | Axum via proxy | `query` | same |
| `my/subscriptions/+page.svelte` | `fetch('/api/subscriptions/*')` | Axum via proxy | `query` | same |
| `watchlist/[slug]/+page.ts` | `fetch('/api/watchlist/*')` | Axum via proxy | `query` | `watchlist/data.remote.ts` |
| `dashboard/classes/+page.svelte` | `fetch('/api/courses/*')` | Axum via proxy | `query` | `dashboard/classes/data.remote.ts` |
| `classes/[slug]/+page.server.ts` | server-side fetch | Axum direct | `query` | `classes/data.remote.ts` |
| `indicators/[id]/+page.svelte` | `fetch('/api/indicators/*')` | Axum via proxy | `query` | `indicators/data.remote.ts` |

### A7. Auth Flows (Client-Side)

| File | Current Call | Target | Proposed RF Type | Remote File |
|------|-------------|--------|-----------------|-------------|
| `login/+page.svelte` | `$lib/api/auth.login()` | Axum via proxy | `form` | `auth/auth.remote.ts` |
| `register/+page.svelte` | `$lib/api/auth.register()` | Axum via proxy | `form` | same |
| `signup/+page.svelte` | `$lib/api/auth.register()` | Axum via proxy | `form` | same |
| `forgot-password/+page.svelte` | `$lib/api/auth.forgotPassword()` | Axum via proxy | `form` | same |
| `reset-password/+page.svelte` | `$lib/api/auth.resetPassword()` | Axum via proxy | `form` | same |
| `verify-email/+page.svelte` | `$lib/api/auth.verifyEmail()` | Axum via proxy | `command` | same |
| `account/+page.svelte` | `$lib/api/account` | Axum via proxy | `query` | `account/data.remote.ts` |
| `account/sessions/+page.svelte` | `$lib/api/account` | Axum via proxy | `query`/`command` | same |

---

## B) FLOWS TO KEEP AS HTTP ENDPOINTS

| File | Owner | Reason |
|------|-------|--------|
| `api/[...path]/+server.ts` | SvelteKit | **KEEP TEMPORARILY** — catch-all Axum proxy. Will be gradually replaced as remote functions take over individual routes. Remove once all internal flows migrated. |
| `api/auth/login/+server.ts` | SvelteKit | **KEEP** — Sets httpOnly cookies (refresh token). Cookie manipulation requires HTTP endpoint. Remote functions can call this internally. |
| `api/auth/register/+server.ts` | SvelteKit | **KEEP** — Same cookie-setting pattern. |
| `api/auth/refresh/+server.ts` | SvelteKit | **KEEP** — Token refresh with httpOnly cookie rotation. Critical security boundary. |
| `api/auth/set-session/+server.ts` | SvelteKit | **KEEP** — Server-side cookie management for SSR auth. |
| `api/auth/apple/+server.ts` | SvelteKit | **KEEP** — OAuth callback (external contract). |
| `api/auth/apple/callback/+server.ts` | SvelteKit | **KEEP** — OAuth callback (external contract). |
| `api/auth/google/+server.ts` | SvelteKit | **KEEP** — OAuth initiation (external contract). |
| `api/auth/google/callback/+server.ts` | SvelteKit | **KEEP** — OAuth callback (external contract). |
| `api/sse/+server.ts` | SvelteKit | **KEEP** — SSE requires raw HTTP streaming. |
| `api/analytics/batch/+server.ts` | SvelteKit | **KEEP** — Fire-and-forget analytics proxy. Silent failure pattern. |
| `api/analytics/track/+server.ts` | SvelteKit | **KEEP** — Same silent-failure analytics pattern. |
| `api/analytics/performance/+server.ts` | SvelteKit | **KEEP** — Performance metrics collection. |
| `api/analytics/pageview/+server.ts` | SvelteKit | **KEEP** — Pageview tracking. |
| `api/behavior/events/+server.ts` | SvelteKit | **KEEP** — Behavior tracking stub. |
| `api/consent/*/+server.ts` | SvelteKit | **KEEP** — GDPR/CCPA compliance. In-memory storage currently. |
| `api/experiments/config/+server.ts` | SvelteKit | **KEEP** — A/B test config endpoint. |
| `api/cms/newsletter/subscribe/+server.ts` | SvelteKit | **KEEP** — Public newsletter subscription (no auth). |
| `api/cms/ai/generate/+server.ts` | SvelteKit | **KEEP** — AI content generation (may be called externally). |
| `api/cms/upload/image/+server.ts` | SvelteKit | **KEEP** — File upload requires multipart/form-data HTTP. |
| `api/admin/bunny/upload/+server.ts` | SvelteKit | **KEEP** — Video upload to Bunny.net CDN. |
| `api/admin/bunny/create-video/+server.ts` | SvelteKit | **KEEP** — Bunny.net API integration. |
| `api/export/watchlist/+server.ts` | SvelteKit | **KEEP** — CSV/file download requires raw HTTP response. |
| `api/popups/active/+server.ts` | SvelteKit | **KEEP** — Public popup config (no auth, cacheable). |
| `api/prices/+server.ts` | SvelteKit | **KEEP** — Public pricing data. |
| `api/products/+server.ts` | SvelteKit | **KEEP** — Public product listing. |
| `api/schedules/+server.ts` | SvelteKit | **KEEP** — Public schedule data. |

---

## C) AXUM INTEGRATION MAP

### Current State
- **No centralized Axum adapter exists.** Only `$lib/server/watchlist.ts` (1 file).
- Every `+server.ts` and `+page.server.ts` independently constructs Axum URLs and headers.
- API base URL is duplicated across 30+ files: `env.VITE_API_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'`
- Auth token extraction (`cookies.get('rtp_access_token')`) is duplicated everywhere.

### Proposed Centralization

```
src/lib/server/axum/
├── client.ts          # Centralized Axum HTTP client
│                      # - Base URL from env (single source)
│                      # - Auth header injection from cookies/locals
│                      # - Retry with exponential backoff
│                      # - Circuit breaker
│                      # - Correlation IDs
│                      # - Timeout handling
│                      # - Typed error mapping
│
├── auth.ts            # Auth domain: login, register, refresh, me, sessions
├── alerts.ts          # Alerts domain: fetchAlerts, createAlert, deleteAlert
├── trades.ts          # Trades domain: fetchTrades, createTrade, closeTrade, etc.
├── trade-plans.ts     # Trade plans: fetch, create, update, delete
├── stats.ts           # Stats domain: fetchStats
├── videos.ts          # Videos domain: fetchVideos, uploadVideo, weeklyVideo
├── room-resources.ts  # Room resources: tutorials, daily videos, documents
├── watchlist.ts       # Watchlist domain (replace existing $lib/server/watchlist.ts)
├── courses.ts         # Courses domain
├── subscriptions.ts   # Subscriptions domain
├── products.ts        # Products domain
├── members.ts         # Members/users domain
├── admin.ts           # Admin-specific operations
├── crm.ts             # CRM domain
├── media.ts           # Media library domain
└── checkout.ts        # Checkout/orders domain
```

---

## D) SHARED SERVICE EXTRACTION PLAN

### Duplicated Orchestration Candidates

1. **Auth token handling** — duplicated in every `+server.ts` and `$lib/api/*.ts`
2. **API URL construction** — duplicated across 30+ files
3. **Error mapping** — each file has its own try/catch → JSON error pattern
4. **Retry logic** — duplicated in `[...path]/+server.ts`, `$lib/api/client.ts`, `$lib/api/config.ts`
5. **Room resource fetching** — identical pattern in 5+ `+page.server.ts` files

### Proposed Shared Files

```
src/lib/shared/
├── schemas/
│   ├── auth.ts          # Valibot schemas for auth inputs
│   ├── alerts.ts        # Valibot schemas for alert inputs
│   ├── trades.ts        # Valibot schemas for trade inputs
│   ├── trade-plans.ts   # Valibot schemas for trade plan inputs
│   ├── videos.ts        # Valibot schemas for video inputs
│   ├── watchlist.ts     # Valibot schemas for watchlist inputs
│   └── common.ts        # Shared schemas (pagination, slug, id)
│
├── errors/
│   ├── domain.ts        # Typed domain errors (NotFound, Unauthorized, etc.)
│   └── mapping.ts       # Axum error → domain error mapping
│
└── types/
    ├── alerts.ts        # Shared alert types (used by both client and server)
    ├── trades.ts        # Shared trade types
    └── common.ts        # Shared pagination, filter types
```

---

## BOUNDARY MATRIX

| Feature / Call Path | Boundary Type | Axum? | Rationale |
|---|---|---|---|
| Dashboard alerts read | `query` | ✅ | Internal UI read, no secrets exposed |
| Dashboard trade plan read | `query` | ✅ | Internal UI read |
| Dashboard stats read | `query` | ✅ | Internal UI read |
| Dashboard trades read | `query` | ✅ | Internal UI read |
| Dashboard weekly video read | `query` | ✅ | Internal UI read |
| Dashboard admin check | `query` | ✅ | Internal UI read |
| Dashboard room resources (SSR) | `query` | ✅ | Replace +page.server.ts load |
| Dashboard favorites read/toggle | `query`/`command` | ✅ | Internal UI CRUD |
| Dashboard watchlist read | `query` | ✅ | Internal UI read |
| Create/update trade entry | `form` | ✅ | Form submission with validation |
| Close/update/invalidate position | `command` | ✅ | Mutation with auth |
| Delete position | `command` | ✅ | Mutation with auth |
| Add trade | `command` | ✅ | Mutation with auth |
| Upload weekly video | `+server.ts` (KEEP) | ✅ | File upload, multipart |
| Auth login | `form` + `+server.ts` | ✅ | Form + cookie setting |
| Auth register | `form` + `+server.ts` | ✅ | Form + cookie setting |
| Auth refresh | `+server.ts` (KEEP) | ✅ | httpOnly cookie rotation |
| OAuth callbacks | `+server.ts` (KEEP) | ✅ | External contract |
| SSE stream | `+server.ts` (KEEP) | ❌ | Raw HTTP streaming |
| Analytics tracking | `+server.ts` (KEEP) | ✅ | Fire-and-forget, silent failure |
| Newsletter subscribe | `+server.ts` (KEEP) | ✅ | Public endpoint, no auth |
| File uploads (images) | `+server.ts` (KEEP) | ✅ | Multipart form data |
| CSV/file exports | `+server.ts` (KEEP) | ✅ | Raw file download |
| Admin CRUD (all domains) | `query`/`form`/`command` | ✅ | Internal admin UI |
| Blog read | `query` | ✅ | Internal UI read |
| Cart operations | `query`/`command` | ✅ | Internal UI CRUD |
| Checkout | `form` | ✅ | Form submission |
| My courses/indicators/subs | `query` | ✅ | Internal UI read |
| CRM operations | `query`/`form`/`command` | ✅ | Internal admin UI |
| Consent recording | `+server.ts` (KEEP) | ❌ | GDPR compliance, in-memory |
| Public prices/products | `+server.ts` (KEEP) | ✅ | Public cacheable data |

---

## MIGRATION PRIORITY ORDER

### Wave 1: Explosive Swings Dashboard (highest test coverage, most active)
- `data.remote.ts` — queries for alerts, trades, stats, trade plans, weekly video, admin check
- `commands.remote.ts` — delete position, close position, update position, invalidate position, add trade
- `forms.remote.ts` — create/update trade entry

### Wave 2: Other Trading Room Dashboards
- Shared `room-data.remote.ts` for room resources pattern (5 rooms)
- SPX Profit Pulse specific queries

### Wave 3: Account & Auth
- Account data queries
- Account edit forms
- Auth forms (login, register, forgot-password, reset-password)

### Wave 4: Admin Panel
- Start with highest-traffic admin pages
- CRM, Members, Blog, Courses, Schedules

### Wave 5: Public Pages
- Blog, Cart, Checkout, My pages

### Wave 6: Cleanup
- Remove catch-all proxy `api/[...path]/+server.ts`
- Remove individual proxy `+server.ts` files that are no longer needed
- Remove `$lib/api/*.ts` client-side domain modules (replaced by `.remote.ts`)

---

## RISK ASSESSMENT

1. **Remote Functions are experimental** — API may change. Mitigate: pin SvelteKit version, wrap in thin abstraction.
2. **Cookie handling** — Remote functions use `getRequestEvent()` for cookies. Auth token extraction pattern changes.
3. **File uploads** — Cannot use remote functions for multipart uploads. Keep `+server.ts` for these.
4. **SSE/WebSocket** — Cannot use remote functions. Keep `+server.ts`.
5. **Existing tests** — 1,434 tests pass. Migration must not break them. Add new tests for remote functions.
6. **Cloudflare Pages deployment** — Verify remote functions work with `@sveltejs/adapter-cloudflare`.
7. **148 files import from `$lib/api/`** — Large blast radius. Migrate incrementally by wave.
