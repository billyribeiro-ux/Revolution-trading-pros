# Product & Authentication End-to-End Audit

**Auditor:** Claude (Opus 4.7) · **Date:** 2026-04-25 · **Commit:** `9e4a86eb6`
**Scope:** authentication, RBAC, ABAC, CRUD coverage, and the four product
verticals (indicators, courses, trading rooms, services / pricing).
**Method:** four parallel read-only sweeps of `api/src/` and `frontend/src/`.
Every claim is cited to `file:line`. This is a state-of-the-system snapshot,
not a refactor plan — the goal is to give the next dev (you, after a coffee
break) an accurate map before they pick the work back up.

---

## 0. TL;DR for the dev meeting

| Area | State | One-line |
|------|-------|----------|
| Auth — email/password login | ✅ working | Argon2id, JWT pair, httpOnly cookies, Redis revocation |
| Auth — refresh + rotation | ✅ working | Refresh rotates on every use |
| Auth — OAuth (Google + Apple) | ⚠ partial | PKCE on Google ✓, Apple uses state+nonce only; tokens travel in URL on callback |
| Auth — MFA (TOTP) | 🟥 dormant | Service is implemented end-to-end but never called from `login` |
| Auth — email verification | ⚠ tracked-but-not-enforced | `email_verified_at` is fetched and ignored except at login gate |
| Auth — password reset | ✅ working | Hashed reset token, 1h TTL |
| RBAC — backend gate | ✅ enforced | `AdminUser` extractor on every `/admin/*` route |
| RBAC — frontend admin gate | 🟥 missing | `/admin/+layout.ts` has **no role check** |
| ABAC — subscription gating | 🟥 not enforced server-side | Public course list serves all published courses regardless of plan |
| ABAC — banned/inactive users | 🟥 not enforced | `is_active` only inspected by admin list endpoint |
| CRUD — indicators / courses / cms / payments | ✅ full | three-tier (public/auth/admin) with revisions, Stripe wired |
| CRUD — trading rooms | ⚠ partial | seed data + read-only routes; no admin CREATE/UPDATE/DELETE |
| CRUD — user profile / settings | 🟥 missing | no self-service `PUT /users/:id` or change-password endpoint |
| Products — indicators | ⚠ partial | model + delivery tokens complete; admin upload route disabled in mod.rs |
| Products — courses | ✅ complete | three-tier, drip, prereqs, quizzes, Bunny Stream, progress |
| Products — trading rooms | ✅ complete (read) | 6 rooms seeded; alerts/trades/plans + WebSocket real-time |
| Products — pricing / Stripe | ⚠ partial | `// TODO: Create Stripe checkout session` in `subscriptions.rs:446` |

The DB outage that's blocking login right now ([§7](#7-current-incidents)) is
**infrastructure**, not code. The `/api/health` route name was also wrong —
the live one is `/health` (no `/api` prefix).

---

## 1. Authentication system

### 1.1 Login flow (email + password)

```
LoginForm.svelte (POST /api/auth/login)
  → frontend/src/routes/api/auth/login/+server.ts:26-32   proxy
    → api/src/routes/auth.rs:465-715   handler
        - rate limit (Redis sliding window)            line 479-514
        - users lookup by email                        line 517-530
        - timing-attack dummy hash                     line 538
        - Argon2id verify, bcrypt fallback for legacy  line 557-563
        - email_verified_at gate                       line 623-642
                (developer/superadmin emails bypass)
        - issue access JWT  (HS256, 1h default)        line 662-673
        - issue refresh JWT (HS256, 7d)                line 676-682
        - persist session_id in Redis                  line 685-695
    ← { token, refresh_token, session_id, user, expires_in }
  ← frontend/src/routes/api/auth/login/+server.ts:54-76
        - cookies.set('rtp_access_token',  …, httpOnly, Lax, expires_in)
        - cookies.set('rtp_refresh_token', …, httpOnly, Lax, 30d)
```

Subsequent navigation: `frontend/src/hooks.server.ts:125-152` reads the
access cookie and validates against `GET /api/auth/me`.

### 1.2 Tokens

- **Algorithm:** HS256 (`api/src/utils/mod.rs:252`).
- **Claims:** `sub`, `exp`, `iat`, `token_type` ("access" | "refresh") —
  `utils/mod.rs:29-36`.
- **Access TTL:** `state.config.jwt_expires_in`, default 1 h.
- **Refresh TTL:** 7 days (`utils/mod.rs:235`).
- **Storage:** httpOnly cookies named `rtp_access_token` and
  `rtp_refresh_token`; never in JS-readable storage.
- **Rotation:** every refresh issues a new refresh token
  (`auth.rs:752-783`); old one is added to the Redis blacklist.
- **Revocation:** `middleware/auth.rs:54-78` SHA-256-hashes the bearer token
  and queries Redis. **Fails open** if Redis is unreachable
  (`auth.rs:68-75`). Worth weighing — see [§9](#9-recommended-followups).

### 1.3 Refresh

Triggered by `hooks.server.ts:171-195` when `/api/auth/me` returns 401 and
a `rtp_refresh_token` cookie exists. Frontend hits
`/api/auth/refresh` proxy → `api/src/routes/auth.rs:719`. Backend rotates,
returns a new pair, frontend rewrites both cookies.

### 1.4 Logout

- `POST /api/auth/logout` — `auth.rs:809-862`. Invalidates the Redis
  session (line 818) and blacklists the access token's SHA-256 hash with
  `min(remaining_ttl, 24h)`.
- `POST /api/auth/logout-all` — `auth.rs:866-893`. Calls
  `redis.invalidate_all_user_sessions(user.id)`.

### 1.5 OAuth (Google, Apple)

Both flows live in `api/src/routes/oauth.rs`.

- **Google** uses PKCE: verifier generated `oauth.rs:585-586`, challenge
  embedded in the authorize URL `oauth.rs:616`, verifier sent in the token
  exchange `oauth.rs:735`. State validated at `oauth.rs:668-695`.
- **Apple** uses **state + nonce only** (`oauth.rs:877-884`), no PKCE.
  Apple's form_post POST callback verified at `oauth.rs:962-996`.
- **Callback handoff:** the API's callback redirects back to the SPA with
  the token pair in the **URL query string**. The author flagged this as a
  known concern (`oauth.rs:840-844`). The frontend trampoline at
  `/auth/callback/+page.svelte` then immediately POSTs them to
  `/api/auth/set-session` which sets the httpOnly cookies and redirects
  to a clean URL. Window between issue and cookie-set is short but
  non-zero, and the URL still ends up in browser history / Cloudflare access
  logs.

### 1.6 MFA (TOTP) — dormant

`api/src/services/mfa.rs` is fully implemented:

- `generate_totp_secret` / `generate_totp` / `verify_totp`
  (`mfa.rs:53-126`) — RFC 6238, 30 s period, 6 digits, HMAC-SHA1.
- `setup_mfa` returns the otpauth URL + 10 Argon2-hashed backup codes
  (`mfa.rs:167-201`).
- `verify_mfa` handles TOTP and backup codes with rate limiting at
  5 failures / 15 min (`mfa.rs:257-432`).

But the login handler **never calls it**. It selects `mfa_enabled` in every
user query (`auth.rs:94, 133, 262, 354, 518, 903`) and never branches on
the value. There is no `/api/auth/mfa/verify` route in the auth router
(`auth.rs:1086-1103`). MFA setup likely lives in an admin/account flow
that's not mounted yet either.

### 1.7 Password reset

Standard chain — `forgot_password` → email with hashed token (`auth.rs:897-966`),
1 h expiry; `reset_password` consumes it once and rehashes with Argon2id
(`auth.rs:970-1083`). Strength check (`utils/mod.rs:56-135`) requires 12-128 chars,
upper/lower/digit/special, no weak patterns.

---

## 2. RBAC

### 2.1 Roles in the system

| Role | Source of truth | Notes |
|------|-----------------|-------|
| `super_admin` / `super-admin` | `api/src/middleware/admin.rs:76-77`; `frontend/src/lib/config/roles.ts:68` | Both hyphen and underscore variants are accepted everywhere |
| `admin` | `api/src/middleware/admin.rs:27-30`; `roles.ts:70` | |
| `developer` | `admin.rs:30`; `roles.ts:69` | Counts as admin in the backend extractor |
| `member` | `roles.ts:71` (frontend only) | Not a backend role — derived via membership table |
| `user` | `api/src/models/user.rs:126` (default) | Anyone authenticated |

The backend stores `role` as a single nullable string on `users`
(`models/user.rs:20`). The API response converts that into a `roles: string[]`
(`models/user.rs:105, 145`), which is what `frontend/src/lib/config/roles.ts`
expects.

### 2.2 Backend RBAC enforcement

Per-extractor pattern. Handlers that need admin declare `AdminUser` (or
`SuperAdminUser`) as a parameter and the extractor at
`api/src/middleware/admin.rs:11-95` runs before the body. It returns 401 when
the user isn't authenticated, 403 when their role isn't in the allowlist, and
also honors a `SUPERADMIN_EMAILS` config-based bypass
(`admin.rs:33, 80`).

Admin-only routers: `admin.rs`, `admin_courses.rs`, `admin_indicators.rs`,
`admin_videos.rs`, `admin_member_management.rs`, `admin_members.rs`,
`admin_popups.rs`, `admin_page_layouts.rs`, `subscriptions_admin.rs`,
`courses_admin.rs`, `indicators_admin.rs`, plus the bulk of `cms_*` routes.
All confirmed to use `AdminUser`.

### 2.3 Frontend RBAC enforcement

`frontend/src/hooks.server.ts:30` defines:

```ts
const PROTECTED_ROUTES = ['/dashboard', '/account', '/checkout',
                          '/trading-room', '/admin'];
```

The hook ensures **authentication** before any protected route's load
function runs (`hooks.server.ts:41-310`). It does **not** check role.

`frontend/src/routes/admin/+layout.ts:1-4` is the only admin-side gate, and
it consists of `export const ssr = false; export const prerender = false;` —
**no role check, no redirect**. This is a real gap: any logged-in user can
hit `/admin` and the backend will reject the data calls with 403, but the UI
will mount first.

`frontend/src/routes/dashboard/+layout.server.ts` accepts `event.locals.user`
unconditionally — admins, members, and unverified users all land on the
same `/dashboard`.

### 2.4 Helpers exist, just aren't used

`frontend/src/lib/config/roles.ts` ships well-built helpers — `isAdmin`,
`isSuperadmin`, `hasPermission` (lines 253-352). They're called by
`AdminToolbar.svelte` and a handful of other components for *UI* gating
("hide this button"), but **no route guard uses them**.

---

## 3. ABAC

### 3.1 What attributes exist on `users`

From `api/src/models/user.rs:12-29`:

- `role: Option<String>` — primary RBAC field
- `email_verified_at: Option<NaiveDateTime>` — verification gate
- `mfa_enabled: Option<bool>` — MFA flag
- (derived in `UserResponse`) `is_admin`, `permissions`

Missing or off-model:
- `is_active` exists in the SQL (`admin.rs:82` queries it) but is **not
  loaded into the `User` struct** that flows through middleware. Banned
  users keep working tokens until expiry.
- Subscription tier / plan id is **not on `users`** — it's a join into
  `user_memberships` (`admin.rs:869-875`). Middleware never performs that
  join, so route handlers don't know a caller's plan unless they ask.

### 3.2 Where ABAC is and isn't enforced

| Surface | Check that exists | Status |
|---------|-------------------|--------|
| Login | email_verified_at must be set (unless dev/superadmin) | ✅ enforced (`auth.rs:623-642`) |
| Public course list | required_plan_id field exists on courses | 🟥 not consulted by `member_courses.rs:54-86` |
| Course detail / video URL | Bunny Stream URL served directly | 🟥 no membership check on the read path |
| Trading rooms | `is_public` on the room | ⚠ checked for visibility, not for join/alert push |
| Indicator download | Token-based, expiring, IP-bound | ✅ proper ABAC via `IndicatorDownload` (`models/indicator.rs:156-173`) — best-built control in the repo |
| Banned user | `is_active=false` on row | 🟥 never read by middleware |
| Email-unverified user | `email_verified_at` NULL | 🟥 only checked at login; cached tokens stay valid |

So ownership for paid digital downloads (indicators) is solid. Streamed
content (videos, course lessons, room alerts) is currently **trust-the-frontend**.

---

## 4. CRUD coverage

Inventoried 13 resource groups across the 77 route modules. The full
matrix lives in this doc's [Appendix A](#appendix-a-crud-matrix); the
summary:

**Fully built (10):** indicators · courses · CMS v2 · orders / payments /
subscriptions · categories · tags · search · videos · forms / newsletter ·
media library.

**Partial (4):**
- **Trading rooms** — read-only public + admin reads. No admin
  CREATE/UPDATE/DELETE for rooms themselves. Alerts within a room are full
  CRUD (`room_content.rs:232-280`).
- **Members / Users** — admin can read and ban; **no profile self-service.**
  No `PUT /users/:id`, no avatar upload route, no change-password endpoint
  (password reset only via the email-forgot flow).
- **Watchlist / Favorites / Connections** — `GET` only.
- **Popups / Schedules** — admin popups are full CRUD; user-facing schedule
  CRUD is missing.

**Routes that look stubbed:**
- `admin_indicators.rs` upload routes are referenced but the wiring in
  `routes/mod.rs:40, 109` looks disabled — admin can manage metadata, can't
  push the actual file.
- `subscriptions.rs:446` — `// TODO: Create Stripe checkout session`. The
  Stripe **types** are present, the **price IDs** are stored, but the
  function that builds a Checkout session and returns its URL is a stub.
  This is the missing piece for "user clicks Subscribe → goes to Stripe."
- `websocket.rs:344` — `// TODO: Validate JWT tokens`. WebSocket
  connections are currently accepted without checking the bearer.
- `media.rs:1174` — `// TODO: Integrate scanning service` (malware scan on
  upload).

---

## 5. Products

### 5.1 Indicators — partial

- Model: `api/src/models/indicator.rs:24-44` covers id / name / slug /
  price / platform / version / download_url / thumbnail / features /
  requirements / screenshots / SEO meta / `is_active`.
- Multi-platform delivery: `IndicatorFile` (`indicator.rs:66-94`) —
  TradingView, ThinkorSwim, MT4, etc., with SHA-256 checksums and
  storage_provider (R2 / Bunny / S3).
- Ownership: `UserIndicatorOwnership` (`indicator.rs:128-148`) tracks
  expiration and lifetime access.
- Secure download: `IndicatorDownload` (`indicator.rs:156-173`) issues
  short-lived tokens with `max_downloads`, IP, and UA tracking.
- Frontend: `frontend/src/routes/indicators/+page.svelte` (browse),
  `[id]/+page.svelte` (detail), `rsi/`, `macd/` (pre-rendered examples),
  `/my/indicators/` (member library).
- **Gap:** the upload pipeline that fills `cdn_url` and the Bunny-Stream
  preview videos isn't wired. `admin_indicators.rs` exists but the upload
  routes appear disabled in `routes/mod.rs`. So today an indicator can be
  *modeled* but not *delivered*.

### 5.2 Courses — complete

- Model: `Course` (`course.rs:14-74`), `CourseModule` with drip support
  (`course.rs:107-121`), `Lesson` with Bunny GUID + prerequisites
  (`course.rs:128-162`), `UserCourseEnrollment` (`course.rs:219-241`),
  `UserLessonProgress` (`course.rs:254-268`), `CourseReview`
  (`course.rs:275-290`), `Quiz` / `QuizQuestion` / `QuizAnswer`
  (`course.rs:516-561`).
- Routes: `courses.rs` (public read), `courses_admin.rs` (admin CRUD with
  modules / lessons / live sessions / enrollments), `member_courses.rs`
  (enrollment + progress).
- Frontend: `/courses/`, plus pre-rendered course pages
  `/courses/day-trading-masterclass/`, `/courses/options-trading/`,
  `/courses/swing-trading-pro/`, `/courses/risk-management/`, and the
  member dashboard at `/my/courses/`.
- Player payload: `CoursePlayerData` (`course.rs:493-500`) ships course +
  modules + downloads + enrollment + progress + current_lesson in one
  shot. Drip and prereq fields are honored.
- **Tech debt:** completion-cert email and quiz-grade-notification emails
  are stubs (see §6).

### 5.3 Trading rooms — complete on the read path

Six rooms seeded in `trading_rooms.rs:87-100`+:
1. Day Trading Room
2. Swing Trading Room
3. Small Account Mentorship (`room_type: "mentorship"`)
4. Explosive Swings
5. SPX Profit Pulse
6. High Octane Scanner

Per-room data:
- `RoomAlert` (`room_content.rs:67-89`) — alert_type, symbol,
  entry/exit alert linkage, expandable notes.
- `TradePlan` (`room_content.rs:156-177`) — entry / target / stop /
  rationale.
- `RoomResource` — generic attachments.
- Full alert CRUD (`room_content.rs:232-280`).
- Real-time fan-out via `services/event_broadcaster.rs`, exposed through
  `routes/websocket.rs` (primary) and `routes/realtime.rs` (SSE fallback).
- Search (`room_search.rs`) and analytics (`room_analytics.rs`) per room.

Frontend: `/live-trading-rooms/`, `/live-trading-rooms/day-trading/`,
`/swing-trading/`, `/small-accounts/`. Alert UI consumes the WebSocket.

### 5.4 Other offerings

| Offering | Where it lives | State |
|----------|----------------|-------|
| Alert Services (Explosive Swings, SPX Profit Pulse) | `RoomAlert` rows on the relevant rooms; frontend `/alerts/explosive-swings`, `/alerts/spx-profit-pulse` | ✅ wired through trading-room model |
| Mentorship | `room_type: "mentorship"` on the Small Account room | ⚠ exists as a flag; no scheduling / 1-on-1 model |
| Newsletter | `models/newsletter.rs:12-37` (with GDPR consent fields), `routes/newsletter.rs` | ⚠ subscribe / token-confirm / unsubscribe wired; `newsletter.rs:251` and `:318` have `// TODO: actually send confirmation email` |
| Blog / Posts | `models/post.rs:24-48` (rich SEO), `routes/posts.rs`, `frontend/src/routes/blog/`, `frontend/src/routes/posts/`, homepage `LatestBlogsSection` | ✅ complete |
| Options Calculator | None on `main` at this commit | 🟥 referenced in older git history, not present here |

### 5.5 Pricing & Stripe

- Plans: `MembershipPlan` (`subscription.rs:33-48`) with price, billing
  cycle (Monthly / Quarterly / Annual — `subscription.rs:13-17`),
  `stripe_price_id`, features JSON, trial_days. Per-room plans supported
  via `MembershipPlanExtended` (`subscriptions.rs:34-57`).
- Service: `services/stripe.rs` defines the SDK shape (Checkout sessions,
  subscriptions, customers, refunds, payment methods); pinned to
  Stripe API version `2024-12-18.acacia`.
- Subscription lifecycle: statuses `Active / Cancelled / Expired / Pending
  / PastDue / Paused` (`subscription.rs:23-30`); user-facing CRUD in
  `subscriptions.rs` (POST create, GET /my, GET /my/active, POST
  change-plan, POST cancel).
- Coupons: `models/order.rs:104-119` — percentage and fixed, max_uses,
  expiry, per-product/plan applicability.
- Checkout: `routes/checkout.rs:62-80` calculates totals from cart +
  coupon and creates an `Order`. **The Stripe Checkout-Session creation
  call itself is a stub** (`subscriptions.rs:446` — `// TODO: Create Stripe
  checkout session`). So the order is created locally; the user is not
  yet redirected to Stripe to actually pay.

This is the most economically important gap in the entire backend — until
this is filled, the site cannot collect money through the standard self-serve
funnel.

---

## 6. Tech-debt index

Found in source via grep:

| Location | Note |
|----------|------|
| `api/src/routes/subscriptions.rs:446` | `// TODO: Create Stripe checkout session` |
| `api/src/routes/websocket.rs:344` | `// TODO: Validate JWT tokens` |
| `api/src/routes/newsletter.rs:251, :318` | `// TODO: Actually send confirmation email` |
| `api/src/routes/email_templates.rs:384` | `// TODO: Integrate (Postmark/SendGrid/etc.)` |
| `api/src/routes/admin_videos.rs:323-324, 644-645` | `// TODO` fetch trader/room metadata; this_week/this_month aggregations |
| `api/src/routes/media.rs:1174` | `// TODO: Integrate scanning service` (malware) |
| `api/src/routes/oauth.rs:840-844, 1070-1084` | Tokens travel in URL query string; comment acknowledges concern |
| `api/src/middleware/auth.rs:68-75` | Redis blacklist check **fails open** if Redis is unreachable |
| `api/src/routes/mod.rs:40, 109` | Indicator file-upload routes appear commented/disabled |
| `frontend/src/routes/admin/+layout.ts:1-4` | No role check — only disables SSR |
| `frontend/src/routes/dashboard/+layout.server.ts` | No role-aware redirect |

---

## 7. Current incidents

- **Production database is unreachable from the API.** Probed at 2026-04-25
  14:30 PT: `GET /health` → 200 healthy; `GET /api/posts?per_page=6` →
  500 with body `{"error":"error communicating with database: expected to
  read 5 bytes, got 0 bytes at EOF"}`; `POST /api/auth/login` → 500 with
  body `{"error":"Database error"}`. Diagnosis: Fly.io Postgres app is
  stopped, out of connections, or the Flycast tunnel is wedged. The API
  process itself is fine.
  - **Action:** `flyctl status -a revolution-db`,
    `flyctl logs -a revolution-db | tail -200`. Then
    `flyctl machines restart -a revolution-db`, then
    `flyctl apps restart revolution-trading-pros-api` to reset the API's
    connection pool.
- **CSP blocks `cloudflareinsights.com/cdn-cgi/rum`.** Cosmetic, but worth
  cleaning up. Either widen `connect-src` to include the bare host or
  remove the RUM beacon. Currently only `static.cloudflareinsights.com` is
  allowed.
- **Production API reports `environment: "development"`.** Set
  `ENVIRONMENT=production` on the Fly app — this almost certainly relaxes
  cookie / CORS / log-level defaults.

---

## 8. Where to focus when we go back to building

Ordered by user-impact × effort:

1. **Fix the DB outage** ([§7](#7-current-incidents)). Until this is solved
   nothing else matters.
2. **Wire the Stripe Checkout-Session creation call**
   (`subscriptions.rs:446`). Without it the funnel can't take money.
3. **Add the frontend admin role gate** in
   `frontend/src/routes/admin/+layout.server.ts` (new file): redirect to
   `/dashboard` if `event.locals.user?.is_admin !== true`.
4. **Activate MFA at login.** Branch on `user.mfa_enabled` after password
   verify, return `mfa_required: true` without setting cookies, expose
   `POST /api/auth/mfa/verify` (call into `services/mfa.rs::verify_mfa`).
5. **Add ABAC on streamed content.** Course-detail and room-content
   handlers should join `user_memberships` and 403 when the caller's plan
   doesn't cover the resource. The data model is already there.
6. **Validate JWTs on the WebSocket** (`websocket.rs:344`). Without this,
   anyone with the URL can subscribe to live alerts.
7. **`is_active` enforcement.** Add a column to the `User` row loaded by
   middleware and return 401 when false. One-line per layer.
8. **Indicator upload route.** Re-enable the disabled mounts in
   `routes/mod.rs`, finish whatever is unfinished, exercise it with a
   small TradingView script.
9. **User profile self-service** — `PUT /users/:id`, change-password,
   avatar. The data model is there; routes aren't.
10. **Newsletter / quiz email sending.** Use the same Postmark client as
    the verification emails.

Lower-priority but worth tracking: trading-room admin CRUD (5.3 is read-only),
malware scanning on uploads (`media.rs:1174`), room scheduling, options
calculator (currently absent on this branch).

---

## 9. Recommended follow-ups (security-flavored)

- Move Redis blacklist to **fail-closed** for `/admin/*` (and arguably for
  any state-changing call). The current "fail open for availability" is a
  reasonable default for SaaS but is the wrong default for an admin panel.
- Stop returning OAuth tokens in the URL on callback. Either set the cookie
  on the API side and `Set-Cookie: Domain=.pages.dev`-style across the
  apex (requires same eTLD+1), or POST them to a frontend endpoint via a
  tiny self-submitting form.
- Tighten the email-verification gate so subsequent calls (not just the
  initial login) re-check `email_verified_at`. Today, a token issued for a
  later-revoked-and-unverified user keeps working until expiry.
- Apple sign-in: PKCE is supported by Apple now. Adding it makes the
  client-secret leak less catastrophic.
- Document the `SUPERADMIN_EMAILS` env list and ensure both ends agree on
  it (`admin.rs:33` vs `roles.ts:48-62`).

---

## 10. Svelte Remote Functions — migration plan

### 10.1 Status check

**Already opted in.** `frontend/svelte.config.js:13-16`:

```js
kit: {
  experimental: {
    remoteFunctions: true
  },
  …
}
```

`compilerOptions.experimental.async` is **not** set — meaning we can use
the `query.loading / .error / .current` accessor form, but we cannot
`{#await getPosts()}` directly inside templates yet. Turning on `async` is
a one-line follow-up if/when we want top-level `await` in markup.

**Already in use** in two files (the canonical pattern to copy):

- `frontend/src/routes/dashboard/explosive-swings/data.remote.ts`
  — `query` functions for alerts, trade plan, stats, trades, weekly video.
- `frontend/src/routes/dashboard/explosive-swings/commands.remote.ts`
  — `command` functions for save / close / update / delete alerts and
  trades, with `requireAuth()` and `requireAdmin()` guards built on
  `getRequestEvent()`.

So this isn't green-field — there's a house style already. New migrations
should match it.

### 10.2 The four primitives (quick reference)

| Primitive | When to use | Imports |
|-----------|-------------|---------|
| `query` | Read dynamic data; deduped + cached per page | `import { query } from '$app/server'` |
| `query.batch` | Multiple instances on the same page (n+1 killer) | same |
| `prerender` | Read static-ish data (changes ≤ once per deploy) | `import { prerender } from '$app/server'` |
| `form` | Mutations, must work without JS | `import { form } from '$app/server'` |
| `command` | Mutations from event handlers (no `<form>`) | `import { command } from '$app/server'` |

All accept a Valibot/Zod schema as the first argument (we already use
Valibot — see `frontend/src/lib/shared/schemas/`). Skip with `'unchecked'`
only when you really mean it.

### 10.3 What this replaces

The repo currently uses **SvelteKit `+server.ts` proxy endpoints** and
**`+page.server.ts` `load` functions** to talk to the Rust API. Remote
functions replace both with type-safe, code-colocated calls. Net wins:

1. **Type safety end-to-end** — no more `as TypedResponse` casts on the
   client.
2. **Auto-deduplication** — calling `getPost('foo')` from three components
   on one page = one network round trip.
3. **Single-flight mutations** — `command` can refresh queries server-side
   in the same response, eliminating the standard "POST then GET" pattern.
4. **Server-only modules stay server-only** — the Axum client (which
   carries the JWT secret-bearing cookies) physically cannot leak into the
   bundle, because `.remote.ts` files are split at build time.

### 10.4 Where remote functions are the right call

Ranked by ROI in this codebase:

#### Tier 1 — clear wins (do these first)

| Target | Replaces | Notes |
|--------|----------|-------|
| **All `frontend/src/routes/api/auth/*/+server.ts` proxies (12 files)** | The 12 manually-coded fetch proxies that re-stringify cookies and forward bodies | Most fit naturally as `command` (login, register, refresh, logout, forgot-password, reset-password, resend-verification, set-session). `me` becomes a `query`. The proxy logic lives once, in one file (`$lib/server/auth.remote.ts`), instead of 12. |
| **`/api/posts` listing on the homepage** | The SSR `fetch` in `+page.server.ts` that's currently retry-looping during the DB outage | Convert to `prerender` (rebuild on deploy) — also fixes the homepage hanging when the API is down. |
| **CMS asset list / page list / categories / tags** | Multiple `+server.ts` proxies under `frontend/src/routes/api/` for read paths | All read-mostly, all heavily deduped on admin pages, all benefit from `query.batch`. |
| **Indicator + course detail pages** | `+page.server.ts` `load` + `+server.ts` proxy combo | Use `query` with a slug schema; pre-rendered example pages (rsi, macd, day-trading-masterclass, etc.) become `prerender` with `inputs`. |
| **Any `<form>` that currently submits to `+server.ts`** | Manual fetch-then-redirect handlers | `form` upgrades them with progressive enhancement, schema validation, and field-level error display "for free". |

#### Tier 2 — solid wins, more invasive

| Target | Replaces | Notes |
|--------|----------|-------|
| **Trading-room alerts feed** | The current SvelteKit endpoint that proxies `/api/rooms/:slug/alerts` plus a separate WebSocket subscription | Use `query` for the initial load + an `$effect` that calls `.refresh()` on WebSocket message. Pairs well with `query.batch` if the same page shows alerts for multiple rooms. |
| **Course player progress updates** | A hand-rolled fetch-and-PUT on every video tick | `command` with debounce; on success use `requested(getEnrollment, 1).refreshAll()` to single-flight-refresh the dashboard. |
| **Admin CMS page editor** | The big editor page that shotguns reads through several proxies on mount | `query.batch` for the parallel reads + `command` for saves. The autosave story becomes "submit; SvelteKit refreshes the queries; UI re-renders" instead of manual cache busting. |
| **Watchlist / Favorites / Connections** | The currently-read-only routes | When we add CREATE/UPDATE/DELETE (currently a [§4](#4-crud-coverage) gap) build them as `command`s from day one. |

#### Tier 3 — keep what you have

- **Stripe webhook receiver** (`/api/payments/webhook`). Stripe signs to a
  fixed URL, so it must remain a regular `+server.ts` POST handler.
- **OAuth callbacks** (`/api/auth/google/callback`, etc.). The Rust API
  redirects to these by URL; they have to be real routes.
- **Long-lived streams** (SSE, WebSocket). Remote functions are
  request/response — keep `realtime.rs` and the WebSocket client as-is.
- **Anything that proxies multipart uploads >5 MB.** Form handlers can
  accept `File` inputs but the request still flows through SvelteKit;
  keep R2 presigned-URL flows direct from browser to R2.

### 10.5 The house pattern (copy this for every new file)

Verified in `dashboard/explosive-swings/{data,commands}.remote.ts`:

```ts
// data.remote.ts  (queries — reads)
import * as v from 'valibot';
import { query, getRequestEvent } from '$app/server';
import { axumPosts } from '$lib/server/axum';                // server-only API client
import { FetchPostsInputSchema } from '$lib/shared/schemas/posts';

export const getPosts = query(FetchPostsInputSchema, async (input) => {
  return await axumPosts.list(input);
});

export const getPost = query(v.string(), async (slug) => {
  const post = await axumPosts.bySlug(slug);
  if (!post) error(404, 'Not found');
  return post;
});
```

```ts
// commands.remote.ts  (mutations — writes)
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, requested } from '$app/server';
import { axumPosts, axumAuth } from '$lib/server/axum';
import { CreatePostInputSchema } from '$lib/shared/schemas/posts';
import { getPosts } from './data.remote';

async function requireAdmin() {
  const event = getRequestEvent();
  const token = event.locals.accessToken ?? event.cookies.get('rtp_access_token');
  if (!token) error(401, 'Authentication required');
  if (!(await axumAuth.checkAdminStatus())) error(403, 'Admin access required');
}

export const createPost = command(CreatePostInputSchema, async (input) => {
  await requireAdmin();
  const post = await axumPosts.create(input);

  // Single-flight: refresh any active getPosts(...) instances in the same response
  await requested(getPosts, 5).refreshAll();
  return post;
});
```

Use site (any `+page.svelte`):

```svelte
<script lang="ts">
  import { getPosts, getPost } from './data.remote';
  import { createPost } from './commands.remote';

  const posts = getPosts({ limit: 10 });
</script>

{#if posts.error}
  <p>Couldn't load posts.</p>
{:else if posts.loading}
  <p>Loading…</p>
{:else}
  <ul>
    {#each posts.current as post (post.id)}
      <li><a href="/blog/{post.slug}">{post.title}</a></li>
    {/each}
  </ul>
{/if}
```

### 10.6 Concrete next steps when we resume dev

1. **Pick one slice end-to-end** as the second case study (after explosive-swings).
   Recommendation: **homepage `LatestBlogsSection`** → `getPosts.prerender(...)`.
   That immediately fixes the current SSR retry-storm seen in
   [§7](#7-current-incidents).
2. **Carve out `$lib/server/axum.ts`** if it doesn't already centralize
   token forwarding; remote functions should always go through it so the
   cookie-to-Bearer translation lives in one file.
3. **Migrate the auth proxies** as one PR. They're identical in shape and
   their tests already exist (vitest stubs the fetch). This deletes 12
   files and produces ~2 (`auth.remote.ts` + `auth.commands.remote.ts`).
4. **Convert the indicator detail page** as the first dynamic-content
   case (slug-keyed `query`).
5. **Validate every new `.remote.ts` file** with the Svelte MCP
   `svelte-autofixer` tool and re-run `pnpm run check` + `vitest` after
   each migration. CLAUDE.md mandates this loop.
6. **Once a few are migrated**, enable `compilerOptions.experimental.async`
   and switch the call sites to top-level `{#await getPosts()}` form,
   which is more readable than `.loading / .error / .current`.

### 10.7 Issues in the existing `.remote.ts` files

While reviewing `dashboard/explosive-swings/{data,commands}.remote.ts`
against the official docs, two patterns came up that **don't match what the
SvelteKit team recommends** and should be cleaned up the next time we
touch those files:

1. **Hard-coded refresh keys in `commands.remote.ts`.**
   Lines like
   `getAlerts({ roomSlug, page: 1, limit: 10 }).refresh()` (e.g.
   `commands.remote.ts:71, :89`) only refresh that exact cache key. If the
   user is on page 3 with `limit=20`, their on-screen list won't update.
   Per the docs (§ "Single-flight mutations / Client-requested
   refreshes"), the right shape is one of:
   - `await requested(getAlerts).refreshAll()` — refresh every active
     `getAlerts(...)` regardless of arguments;
   - have the client opt-in via `submit().updates(getAlerts)`; or
   - if the server can compute the new value, `getAlerts(args).set(value)`
     to single-flight-push the result back without a second round trip.
2. **`.refresh()` results aren't `void`'d / `await`'d.** Bare
   `getAlerts(...).refresh();` calls return a `Promise` that
   the runtime expects to either be awaited or explicitly discarded with
   `void`. The docs are clear: the framework only awaits the refresh
   before sending the response if you tell it to. Untracked promises
   could race the command's HTTP response in the wild.

Both fixes are mechanical and should land as part of the first follow-up
PR that touches these files — they don't change behavior in the happy
path but make the unhappy paths correct.

### 10.8 Risks & sharp edges

- **Experimental flag.** SvelteKit may change the API. Pin
  `@sveltejs/kit` exactly (currently `^2.x`) and read the changelog before
  upgrading.
- **`getRequestEvent()` is partially stubbed** in remote-function context
  — `route`, `params`, `url` refer to the *page that called the remote*,
  not the remote-function endpoint itself. Don't use them for authz
  decisions. Use cookies / locals / the function arguments.
- **Don't put a `.remote.ts` file in `src/lib/server/`.** SvelteKit
  refuses (the file has to be reachable from the server bundle's HTTP
  router). Use `src/lib/remote/` or colocate next to the route.
- **`commands` cannot redirect.** Returning `{ redirect: '/somewhere' }`
  and calling `goto()` on the client is the documented workaround, but
  prefer `form` if the natural UX is "submit and navigate".
- **Cache-key sorting.** Queries deduplicate on a stable serialization of
  their argument — `{ limit: 10, offset: 0 }` and `{ offset: 0, limit: 10 }`
  hit the same cache. If order matters in your input, use an array, not
  an object.

---

## Appendix A. CRUD matrix

(Brief; full citations in the audit-agent transcript that produced this
doc.)

| Resource | File | C | R | U | D | Auth |
|----------|------|---|---|---|---|------|
| Indicators (public) | `routes/indicators.rs` | ❌ | ✓ | ❌ | ❌ | public |
| Indicators (admin) | `routes/indicators_admin.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Indicators (member) | `routes/member_indicators.rs` | ❌ | ✓ | ❌ | ❌ | auth |
| Indicator videos | `routes/indicators_admin.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Platform files | `routes/indicators_admin.rs` | ✓ | ✓ | ✓ | ✓ | admin (mounts may be disabled) |
| Courses (public) | `routes/courses.rs` | ❌ | ✓ | ❌ | ❌ | public |
| Courses (admin) | `routes/courses_admin.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Courses (member) | `routes/member_courses.rs` | ❌ | ✓ | ✓ (progress) | ❌ | auth |
| Modules / Lessons / Live sessions / Enrollments / Quizzes | `routes/courses_admin.rs`, `routes/member_courses.rs` | ✓ | ✓ | ✓ | ✓ | admin / auth |
| Trading rooms | `routes/trading_rooms.rs` | ❌ | ✓ | ❌ | ❌ | public |
| Room alerts / trades / plans / resources | `routes/room_content.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Posts | `routes/posts.rs` | ✓ | ✓ | ✓ | ✓ | admin (write) / public (read) |
| Users (admin) | `routes/admin.rs`, `admin_members.rs` | ✓ | ✓ | ✓ | ✓ (ban) | admin |
| Users (self) | — | ❌ | ❌ | 🟥 missing | ❌ | n/a |
| Subscription plans | `routes/subscriptions.rs` | ❌ | ✓ | ❌ | ❌ | public |
| Member subscriptions | `routes/subscriptions.rs` | ✓ | ✓ | ✓ (change) | ✓ (cancel) | auth |
| Orders | `routes/orders.rs` | ❌ | ✓ | ✓ (status / refund) | ❌ | auth / admin |
| Coupons | `routes/coupons.rs` | ✓ | ✓ | ✓ | ✓ (deactivate) | admin |
| Stripe | `routes/payments.rs` | ✓ (checkout — STUB), ✓ (webhook) | ✓ (config) | ❌ | ❌ | public / webhook |
| CMS pages / assets / revisions / tags / blocks / presets / SEO | `routes/cms_v2*.rs`, `cms_*.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Categories | `routes/categories.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Tags | `routes/tags.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Search | `routes/search.rs` | ❌ | ✓ | ❌ | ❌ | public |
| Forms / Submissions | `routes/forms.rs` | ✓ | ✓ | ✓ | ✓ | admin (public submit) |
| Newsletter | `routes/newsletter.rs` | ✓ (subscribe) | ✓ | ❌ | ✓ | public + admin |
| Email templates | `routes/email_templates.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Videos (admin) | `routes/admin_videos.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| Media library | `routes/media.rs` | ✓ | ✓ | ✓ | ✓ | admin |
| WebSocket / SSE | `routes/websocket.rs`, `realtime.rs` | n/a | ✓ (subscribe) | n/a | n/a | public (auth TODO) |
| Watchlist / Favorites / Connections | resp. `.rs` | ❌ | ✓ | ❌ | ❌ | auth |
