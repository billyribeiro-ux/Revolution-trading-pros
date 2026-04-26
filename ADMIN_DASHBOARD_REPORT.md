# Admin Dashboard Forensic — End-to-End

Audit date: 2026-04-26
Read-only. Server: localhost:5173 (SvelteKit dev).

> **SSR is disabled for the entire `/admin/*` tree** — `frontend/src/routes/admin/+layout.ts:3` sets `export const ssr = false`. Every unauthenticated curl returns the same `303 → /login?redirect=/admin` from the auth-guard hook (~1ms). Without a session cookie, SSR latency is meaningless for triage; the Step-2 timing column reflects only the redirect, not real page work. The audit therefore relies on source-level analysis for surface health.

---

## Route inventory

129 admin `+page.svelte` files. Curl sample below confirms that every top-level admin route 303-redirects to login; this is expected (auth guard) and not a bug. Only 1 admin server-loader exists: `admin/trading-rooms/[slug]/+page.server.ts`.

| Route | Purpose | SSR status (unauth) | Notes |
|---|---|---|---|
| `/admin` | "Analytics Dashboard" KPI overview (2280 LOC) | 303 (1.6ms) | Quick-action tile `/admin/email` is broken (404) — see broken surfaces |
| `/admin/dashboard` | Second dashboard (1246 LOC) | 303 | Duplicate-looking entry point; sidebar points to `/admin` instead of `/admin/dashboard` |
| `/admin/analytics` | KPI / funnels / cohorts / attribution shell | 303 | Has nested layout `analytics/+layout.svelte`; tabs `funnels`, `cohorts`, `attribution`, etc. exist as siblings |
| `/admin/courses` | Course list + quick-create modal | 303 | Quick-create redirects to `/admin/page-builder` which **does not exist** |
| `/admin/courses/[id]` | Course editor | 303 | Three `alert()` and two `prompt()` calls (lines 125, 128, 146, 151, 187, 224, 239, 260, 264) — UX regression vs. modal-based pattern used elsewhere in the repo |
| `/admin/users` | Admin users CRUD | 303 | `$effect` for one-shot fetch (anti-pattern per CLAUDE.md). Proxy uses request-header `Authorization` not the cookie pattern (FIX-2026-04-26 inconsistency) |
| `/admin/blog` | Blog posts list (3092 LOC) | 303 | Calls 8 endpoints that have no proxy file — see broken surfaces |
| `/admin/blog/create` | New blog post | 303 | Raw `fetch('/api/admin/posts', POST)` without cookie auth helper |
| `/admin/connections` | Service connection grid | 303 | "Test connection" button calls `/api/admin/connections/[key]/test` which **does not exist** |
| `/admin/orders` | Orders list | 303 | Hits `/api/admin/orders` which **does not exist** as a SvelteKit proxy → 404 → empty grid |
| `/admin/subscriptions` | Subscription mgmt | 303 | Uses `$lib/api/subscriptions` helpers; OK |
| `/admin/settings` | Settings (1825 LOC) | 303 | Many credential fields with placeholders; submit path not audited end-to-end |
| `/admin/members/segments` | Member segments | 303 | "Edit segment", "Analytics", "Export CSV" all toast `Coming soon` |
| `/admin/seo/+page.svelte` | SEO overview | 303 | Comment shows `// const response = await fetch('/api/admin/seo/metrics');` — fetch is commented out, page renders synthetic state |
| `/admin/cart/abandoned` | Abandoned cart list | 303 | No `api/admin/cart/*` proxy exists |
| `/admin/members/[id]` | Member detail | 303 | Hits `/api/admin/user-memberships` which **does not exist** |
| `/admin/users/create` | New admin user | 303 | Hits 6 `/api/admin/organization/*` endpoints that **do not exist** as proxies |
| ... 110+ more | (course CRUD, CRM, email, SEO, popups, forms, etc.) | All 303 | No SSR latency measurable while logged out |

Curl sample of 35 top-level routes — all returned `303` in 0.9–1.7ms (auth redirect):

```
admin                    303 0.0016s
admin/dashboard          303 0.0011s
admin/analytics          303 0.0016s
admin/courses            303 0.0017s
admin/users              303 0.0013s
admin/blog               303 0.0013s
admin/subscriptions      303 0.0010s
admin/connections        303 0.0009s
admin/members            303 0.0011s
admin/settings           303 0.0012s
admin/orders             303 0.0011s
admin/products           303 0.0010s
admin/coupons            303 0.0014s
admin/seo                303 0.0015s
admin/crm                303 0.0016s
admin/email/campaigns    303 0.0015s
admin/contacts           303 0.0015s
admin/popups             303 0.0016s
admin/forms              303 0.0013s
admin/videos             303 0.0013s
admin/media              303 0.0014s
admin/indicators         303 0.0010s
admin/site-health        303 0.0015s
admin/performance        303 0.0013s
admin/watchlist          303 0.0013s
admin/categories         303 0.0011s
admin/memberships        303 0.0010s
admin/schedules          303 0.0011s
admin/resources          303 0.0010s
admin/boards             303 0.0009s
admin/consent            303 0.0014s
admin/cms/datasources    303 0.0011s
admin/cart/abandoned     303 0.0009s
admin/trading-rooms      303 0.0014s
admin/behavior           303 0.0010s
```

---

## Broken surfaces

### B1 — Dead admin sidebar component
`frontend/src/lib/components/admin/Sidebar.svelte:35-62` defines a 3-item nav (Dashboard, Forms, SEO) with handLogout that doesn't call the API. **It is never imported anywhere** — the live shell uses `$lib/components/layout/AdminSidebar.svelte` (38 nav items). Symptom: confusing for engineers, can drift further out of date. Suggested fix: comment-out per CLAUDE.md "comment-out, verify, delete in follow-up" rule, or delete with the next sweep.

### B2 — Quick-create course → 404
`frontend/src/routes/admin/courses/+page.svelte:114` and `:410`, plus `courses/[id]/+page.svelte:358` `goto('/admin/page-builder?course=...')`. **There is no `/admin/page-builder` route.** Every successful course creation fires the redirect into a 404. Suggested fix: redirect to `/admin/courses/[id]` (course editor exists) and ship `page-builder` later.

### B3 — Command-palette nav targets that 404
`frontend/src/lib/components/CommandPalette.svelte:156` `goto('/admin/blog/new')` — real path is `/admin/blog/create`. `:201` `goto('/logout')` — no `/logout` route. Both are reachable from ⌘K → Quick Actions. Suggested fix: change to `/admin/blog/create` and call `authStore.logout(); goto('/')`.

### B4 — Connection "test" button hits nonexistent proxy
`frontend/src/routes/admin/connections/+page.svelte:178` POST `/api/admin/connections/${key}/test`. The proxy tree under `frontend/src/routes/api/admin/connections/[key]/` only contains `connect/` and `disconnect/`. The "Test" button always pops a network-error toast. Suggested fix: either implement the proxy or hide the button until backend exists.

### B5 — Orders page calls nonexistent proxy
`frontend/src/routes/admin/orders/+page.svelte:83, 103, 177` raw `fetch('/api/admin/orders...')`. **No `/api/admin/orders/+server.ts` exists.** Page always shows error banner "Failed to load orders". Suggested fix: create proxy mirroring the existing courses/users pattern.

### B6 — Blog page calls 7 missing proxies
`frontend/src/routes/admin/blog/+page.svelte` calls (each of these routes is absent under `frontend/src/routes/api/admin/posts/`):
- `:302` `/api/admin/posts/bulk-delete`
- `:324` `/api/admin/posts/bulk-status`
- `:354` `/api/admin/posts/${id}` (DELETE)
- `:366` `/api/admin/posts/${id}/duplicate`
- `:379` `/api/admin/posts/${id}/status`
- `:395` `/api/admin/posts/${id}/featured`
- `:420` `/api/admin/posts/export`
- `:447` `/api/admin/posts/import`
- `:505` `/api/admin/posts/${id}/analytics`

All bulk actions, single-post mutations, import/export, and analytics modal will fail. Stats endpoint (`posts/stats`) and base `posts` GET *do* exist. Suggested fix: ship the missing proxies or hide the controls.

### B7 — Schedule modal is a no-op
`frontend/src/routes/admin/blog/+page.svelte:1407-1416` "Schedule" button toasts `Scheduling feature coming soon` and dismisses the modal without scheduling.

### B8 — Member segments page is half-built
`frontend/src/routes/admin/members/segments/+page.svelte:389-390, 516, 521` — Edit, Analytics, and CSV Export are all `Coming soon` toasts with `// TODO` markers.

### B9 — Quick-action tile `/admin/email`
`frontend/src/routes/admin/+page.svelte:890` lists `/admin/email` and `/admin/email` again (Email + Broadcast). **`admin/email/+page.svelte` does not exist** — only `email/campaigns`, `email/templates`, `email/smtp`, `email/settings`, `email/subscribers`. Both tiles 404. Comment at line 886-889 acknowledges this exists but the alias hasn't been written. Suggested fix: change href to `/admin/email/campaigns`.

### B10 — `users/create` page calls 6 nonexistent proxies
`frontend/src/routes/admin/users/create/+page.svelte:390-395` fans out to:
- `/api/admin/organization/departments`
- `/api/admin/organization/teams`
- `/api/admin/users?role=manager&limit=50` (this one exists)
- `/api/admin/organization/locations`
- `/api/admin/organization/training-modules`
- `/api/admin/organization/onboarding-plans`

The whole `api/admin/organization/` tree is absent. Form submit will receive 5 of 6 fail-soft empty arrays.

### B11 — `members/[id]` calls nonexistent membership proxy
`frontend/src/routes/admin/members/[id]/+page.svelte:397` POST `/api/admin/user-memberships` — no such proxy. Assigning a membership from a member detail is silently broken.

### B12 — Cart abandoned page has no proxy
`frontend/src/routes/admin/cart/abandoned/+page.svelte` — entire `api/admin/cart/` tree is missing.

### B13 — `seo/+page.svelte` fetch is commented out
`frontend/src/routes/admin/seo/+page.svelte:282` `// const response = await fetch('/api/admin/seo/metrics');`. Page renders default state forever; comment never resolved.

### B14 — `media/analytics/+page.svelte` empty onclick
`frontend/src/routes/admin/media/analytics/+page.svelte:255` `<button class="btn-export" onclick={() => {}}>` — Export button is inert.

### B15 — RateLimitIndicator sole-source endpoint mismatch
`frontend/src/lib/components/RateLimitIndicator.svelte:45` calls `adminFetch('/api/admin/connections', { skipAuthRedirect: true })` and reads `data.connections[*].rate_limit`. The connections proxy returns whatever the Rust backend returns; if the backend doesn't include `rate_limit` per connection, the indicator stays "No services connected" even when services *are* connected. This is a silent UX failure; verify backend contract.

### B16 — ConnectionHealthPanel mock metrics
`frontend/src/lib/components/ConnectionHealthPanel.svelte:48-55` ships hard-coded `mockMetrics` for response time, uptime, error rate. Every "Metrics" tab card is fake. Comment on line 47 even reads `// Mock response time data (would come from real API)`. **High-visibility surface (lives in admin header)** that lies to operators.

### B17 — ConnectionHealthPanel "Last checked: Just now" is a literal
`frontend/src/lib/components/ConnectionHealthPanel.svelte:226` renders the string `Last checked: Just now` regardless of when load() last ran.

### B18 — ConnectionHealthPanel `refreshAll` ignores the argument
`frontend/src/lib/components/ConnectionHealthPanel.svelte:135-139` loops `for (service of Object.keys(serviceLabels))` calling `refreshConnection(service)` which itself just does `await connections.load(true)` — i.e. the same global call N times. Spinner flickers across services as `refreshingService` cycles, but the per-service refresh semantic implied by the UI is fictional.

### B19 — AdminToolbar `performance.measure` throws
`frontend/src/lib/components/AdminToolbar.svelte:557-564` calls `performance.measure('admin-toolbar-logout', 'admin-toolbar-logout-start', 'admin-toolbar-logout-end')`. **The `-end` mark is never created** (only `-start` at line 281). `measure()` will throw `SyntaxError`. Wrapped in `try/catch`? No — error bubbles to `trackPerformance` caller, gets swallowed by the outer `try` in `handleLogout`'s `finally` (line 305 already after the try) so logout still completes, but a console error fires every logout. Cosmetic, but fix is trivial: call `performance.mark('admin-toolbar-logout-end')` before `measure`.

### B20 — Dashboard `[id]` route uses `prompt()` for module/lesson titles
`frontend/src/routes/admin/courses/[id]/+page.svelte:151, 187` and `lessons/[lessonId]/+page.svelte:161, 164` use `window.prompt()` to capture title/URL. Native prompts are blocked or stripped on iframes/mobile webviews and can't be styled. Repo has `ModuleFormModal.svelte` and `ConfirmationModal.svelte` for this purpose elsewhere.

---

## Stub data sites

| File:line | Stub | Should fetch from |
|---|---|---|
| `frontend/src/lib/components/ConnectionHealthPanel.svelte:48-55` | `mockMetrics` (response time, uptime, error rate per service) | Backend `/api/admin/connections` should expose per-service health metrics, or new endpoint `/api/admin/connections/[key]/health` |
| `frontend/src/lib/components/ConnectionHealthPanel.svelte:226` | Literal `Last checked: Just now` | Track `lastCheckedAt` in `connections.load()` and format relative |
| `frontend/src/lib/components/CommandPalette.svelte:152-205` | Hard-coded `quickActions` (new-post, new-campaign, upload-media, export-data, refresh-all, clear-cache, logout) | Most are static-OK, but `new-post` points at `/admin/blog/new` which 404s; `logout` should call `authStore.logout()` not `goto('/logout')` |
| `frontend/src/lib/components/CommandPalette.svelte:289-295` | `saveRecentSearch` writes to localStorage but UI has no "Recent Searches" section. Comment line 294: `// TODO: When UI displays recent searches, add state tracking here` | Add UI block bound to `localStorage.recentSearches` |
| `frontend/src/lib/components/NotificationCenter.svelte:212-217` | Empty state. The store (`stores/notifications.svelte.ts`) only fills via WebSocket (`websocketService.subscribeToNotifications`). If WS is unconfigured (per blog page line 207, default), the bell badge is **always zero** in dev and operators never see real notifications | Wire to a REST `/api/admin/notifications` endpoint as a fallback to WebSocket |
| `frontend/src/routes/admin/seo/+page.svelte:282` | `// const response = await fetch('/api/admin/seo/metrics');` — fetch commented out | Either uncomment + ship `seo/metrics` proxy or document why metrics are local-only |
| `frontend/src/routes/admin/+page.svelte:153-158` | `deviceBreakdown = { desktop: 0, mobile: 0, tablet: 0 }` is initialized to zeros and **is never written** by `fetchDashboardStats()`. The Site Health → Device Breakdown panel always shows 0%/0%/0% bars | Read `data.kpis.devices` (or wherever the backend returns the split) and assign to `deviceBreakdown` |
| `frontend/src/routes/admin/+page.svelte:147-150` | `seoMetrics.error404Count.hits` and `redirections.{count,hits}` start at 0 / null and are never set in `fetchDashboardStats` either | Same fix — wire to backend response shape |
| `frontend/src/routes/admin/blog/+page.svelte:48-67` | `predefinedCategories` hard-coded list of 18 trading categories | If categories live in DB, fetch them; otherwise document as intentional |
| `frontend/src/routes/admin/members/segments/+page.svelte:389-390, 516, 521` | "Coming soon" toasts for Edit, Analytics, Export | Implement the three handlers |
| `frontend/src/lib/components/AdminToolbar.svelte:528-553` | `showNotification`, `trackEvent` are stubs (console.log + gtag check); `trackPerformance` calls a broken `performance.measure` | Replace `console.log` with `toastStore` and remove the broken measure |

---

## Buttons going nowhere

| File:line | Button |
|---|---|
| `frontend/src/routes/admin/media/analytics/+page.svelte:255` | `<button class="btn-export" onclick={() => {}}>` — Export Report |
| `frontend/src/routes/admin/blog/+page.svelte:1407-1416` | "Schedule" in Schedule modal — toasts `Scheduling feature coming soon` and closes the modal |
| `frontend/src/routes/admin/members/segments/+page.svelte:389` | Edit Segment — `toastStore.info("Editing 'X' - Coming soon")` |
| `frontend/src/routes/admin/members/segments/+page.svelte:516` | Export CSV — `// TODO: Implement actual CSV export` |
| `frontend/src/routes/admin/members/segments/+page.svelte:521` | Segment Analytics — `Coming soon` toast |
| `frontend/src/lib/components/AdminToolbar.svelte:792-799` | "Profile Settings" → `/account` — that route exists outside admin (`routes/account/`); functionally OK but escapes the admin shell with no warning |
| `frontend/src/lib/components/admin/Sidebar.svelte:64-67` | `handleLogout` calls `authStore.logout()` then `goto('/login')` but never invalidates server-side session (the live AdminSidebar at `layout/AdminSidebar.svelte:153-162` does it correctly with `await logout()` from `$lib/api/auth`) — moot since component is dead, but worth flagging |

---

## Refresh buttons & their loading state owners

| File:line | Button | State | Writers |
|---|---|---|---|
| `frontend/src/routes/admin/+page.svelte:414-423` | "Refresh" KPI dashboard | `isUserRefreshing: $state(false)` | `fetchDashboardStats({userInitiated:true})` flips on entry (line 184), `finally` flips off (line 351). Decoupled from initial load via `isLoading` (line 107) per FIX-2026-04-26 — good pattern |
| `frontend/src/routes/admin/blog/+page.svelte:691-700` | "Refresh" header icon | `loading: $state(false)` | `loadPosts()` sets true on entry, `finally` false (line 187). Auto-refresh `setInterval(30s)` calls both `loadStats()` and conditionally `loadPosts()` (line 145-148) |
| `frontend/src/lib/components/RateLimitIndicator.svelte:165-168` | Caret toggle (not refresh, but periodic) | `isLoading: $state(true)` | `onMount` line 142 sets, finishes after `fetchRateLimits`; auto-refresh every 60s (line 147) |
| `frontend/src/lib/components/ConnectionHealthPanel.svelte:193-200` | "Refresh All" | `refreshingService: $state<string \| null>(null)` | `refreshAll` (line 135) loops services, sets to current service, `connections.load(true)`, restores. Disabled state correct, but per-service refresh is illusory (B18) |
| `frontend/src/lib/components/ConnectionHealthPanel.svelte:295-305` | Per-service refresh icon | Same `refreshingService` | `refreshConnection` (line 126) — but always reloads all connections globally |
| `frontend/src/routes/admin/orders/+page.svelte:73-97` | (page-level reload via `loadOrders`) | `loading: $state(true)` | Set on entry, false in `finally`. Currently always errors due to B5 → spinner stops, but error banner shows |
| `frontend/src/routes/admin/connections/+page.svelte:126-136` | (page-level via `fetchConnections`) | `isLoading: $state(true)` | `finally` flips false. No user-facing refresh button on this page; only initial load |
| `frontend/src/routes/admin/courses/+page.svelte:143-169` | (page-level via `fetchCourses`) | `loading: $state(true)` | Standard try/finally; URL-driven re-fetch on filter change (line 263) |
| `frontend/src/routes/admin/users/+page.svelte:18-41` | (page-level via `loadUsers`) | `loading: $state(true)` | `$effect(() => { loadUsers(); })` (line 14) — anti-pattern per CLAUDE.md; should be `onMount` |
| `frontend/src/routes/admin/subscriptions/+page.svelte:60-83` | (page-level) | `loading + connectionLoading` | `Promise.allSettled` keeps spinner from sticking on a single 500 — good pattern |

No "always-spinning" cases found post-FIX-2026-04-26 (the dashboard `isLoading=true` at first paint is now gated by `isUserRefreshing` for the button per the comment block at lines 97-108). The issue is now confined to the timeout-bound `localFetch` (10s) blocking cards' skeletons, which is acceptable.

---

## High-traffic page deep-dives

### 1) `/admin/courses` — `frontend/src/routes/admin/courses/+page.svelte` (1362 LOC)

**Data flow:**
- `fetchCourses()` line 143 → `adminFetch('/api/admin/courses?page=&per_page=&search=&status=')` (line 155)
- Proxy: `frontend/src/routes/api/admin/courses/+server.ts:18` reads `cookies.get('rtp_access_token')` ✅ (post-FIX-2026-04-26 confirmed)
- POST quick-create line 99 → `adminFetch('/api/admin/courses', POST)` — proxy line 67-69 also reads `rtp_access_token` ✅
- DELETE line 189, publish/unpublish line 205 — both proxies under `courses/[id]/` and `courses/[id]/publish|unpublish/+server.ts` use `rtp_access_token` ✅

**Bugs:**
- B2: After successful create, `goto('/admin/page-builder?course=${id}')` — that route does not exist.
- Initial fetch trigger uses `$effect` (line 262) instead of `onMount`. CLAUDE.md flags this pattern repo-wide.
- `fetchCourses` doesn't propagate HTTP error to the user when the proxy fail-softs to `{ courses: [], total: 0 }` (proxy line 32-39 returns `success: true` with empty data on 401/404). User can't distinguish "no courses" from "auth failure" or "backend down".

**No stub data on the page itself.** Course list is fully backend-driven.

### 2) `/admin/blog` — `frontend/src/routes/admin/blog/+page.svelte` (3092 LOC)

**Data flow:**
- `loadPosts()` line 162 → `adminFetch('/api/admin/posts?...')` (line 174)
- `loadStats()` line 191 → `adminFetch('/api/admin/posts/stats')`
- Both proxies (`api/admin/posts/+server.ts`, `api/admin/posts/stats/+server.ts`) forward Authorization header from request — *not* via `cookies.get('rtp_access_token')`. The page relies on `adminFetch` (`$lib/utils/adminFetch`) attaching the token client-side.

**Bugs:**
- B6: 9 endpoints called from this page have no proxy file. All bulk operations, individual mutations, and import/export are network-broken.
- B7: Schedule modal is a non-functional stub.
- Auto-refresh `setInterval` (line 145) keeps `loadStats` running every 30s even when the tab is hidden. No `visibilitychange` guard.
- WebSocket setup at line 203-242 silently no-ops when `VITE_WS_URL` is unset (which is default per the comment "Fly.io doesn't support WebSockets by default") → all the "real-time" view-count / engagement updates never arrive in production. This is documented in code, but the UI doesn't communicate that "Last updated" timestamp at line 688 is stale by definition.
- `predefinedCategories` (lines 48-67) is local-only — if the DB has different categories, the filter dropdown shows wrong values.

**Stub data on page:** The 18-entry `predefinedCategories` constant.

### 3) `/admin/users` — `frontend/src/routes/admin/users/+page.svelte` (552 LOC)

**Data flow:**
- `loadUsers()` line 18 → `usersApi.list()` from `$lib/api/admin.ts:864` → fetch `/api/admin/users`
- Proxy `api/admin/users/+server.ts` does **not** read `cookies.get('rtp_access_token')`. It forwards `request.headers.get('Authorization')` (line 41-42, 62). This relies on the client's `adminFetch`/`usersApi` setting `Authorization: Bearer …` from `getAuthToken()` localStorage.
- This is **inconsistent with the FIX-2026-04-26 cookie pattern** that the courses, connections, posts/stats, products/stats, membership-plans, and bunny proxies all use. After hard-refresh with cookie present but localStorage cleared, this proxy returns 401.

**Bugs:**
- `$effect(() => { loadUsers(); })` line 14 — one-shot effect anti-pattern. Should be `onMount`.
- Delete UX uses `ConfirmationModal` ✅
- No pagination, no search — list grows unbounded.
- 3 `bg-blob-N` decorative animated divs (lines 79-81) — repo policy `MASTER_UIUX_BACKLOG.md item 8c` per dashboard comment forbids "childish blob effects"; the dashboard removed them but `/admin/users` and `/admin/connections` and `/admin/orders` still ship them.

**Stub data on page:** None. Backend-driven.

### 4) `/admin/connections` — `frontend/src/routes/admin/connections/+page.svelte` (1813 LOC)

**Data flow:**
- `fetchConnections()` line 126 → `adminFetch('/api/admin/connections')`
- Proxy `api/admin/connections/+server.ts:17` reads `cookies.get('rtp_access_token')` ✅ — and `error(401)` if missing (better than the courses fail-soft pattern).
- `connectService` line 146 → POST `/api/admin/connections/${key}/connect` (proxy exists)
- `disconnectService` line 195 → POST `/api/admin/connections/${key}/disconnect` (proxy exists)
- `testConnection` line 178 → POST `/api/admin/connections/${key}/test` — **proxy does not exist (B4)**.

**Bugs:**
- B4 — Test button always errors.
- Init effect (line 282) uses `$effect` not `onMount`; reads `connections` and `categories` runes inside a tracked context. Hasn't blown up because the inner `init` is wrapped in an `async` IIFE, but pattern matches the ones FIX-2026-04-26 already migrated elsewhere.
- Service icons line 244-278 are emoji literals — fine for a dev-grade tool, but the design comment up top advertises "Apple/Netflix-grade" UX.

**Stub data on page:** Service icon emojis (innocuous). Real connection data flows from backend.

---

## Top 10 fix priorities

Ranked by user-visible impact and effort.

1. **B6 — Ship the 9 missing `/api/admin/posts/*` proxies** (or feature-flag the bulk-actions/schedule/import-export controls off). The blog admin has the largest LOC of any admin page and 60% of its action surface is broken.
2. **B5 — Ship `/api/admin/orders/+server.ts`** (or remove the orders nav entry until the backend is wired). It's a top-level navigation item that always errors.
3. **B16 — Replace `mockMetrics` in `ConnectionHealthPanel.svelte:48-55` with real backend metrics**, or remove the "Metrics" tab. This component is rendered from the global admin header — the lie is on every admin page.
4. **B2 — Either ship `/admin/page-builder` or change the post-create redirect** in `courses/+page.svelte:114, 410`. Course creation is a primary admin task that today dead-ends in a 404.
5. **B4 — Implement `/api/admin/connections/[key]/test/+server.ts`** OR remove the Test button. Currently misleads operators into thinking credentials don't work.
6. **B19 — Fix `AdminToolbar.svelte:557-564` `performance.measure` SyntaxError on every logout.** Add the missing `performance.mark('admin-toolbar-logout-end')` or wrap in try/catch.
7. **B3 — Fix CommandPalette nav targets**: `/admin/blog/new` → `/admin/blog/create`, `/logout` → `authStore.logout()`. ⌘K is the power-user surface; broken targets erode trust.
8. **B9 — Alias `/admin/email` to `/admin/email/campaigns`** (or remove from quick-actions in `routes/admin/+page.svelte:890`). Two of the twelve quick-action tiles are dead.
9. **Auth-pattern unification — Migrate `api/admin/users/+server.ts` to the `cookies.get('rtp_access_token')` pattern** that courses/connections/posts/stats already use. Today the users page silently 401s if localStorage is wiped but cookie is fresh.
10. **B17/B18 — Fix `ConnectionHealthPanel` "Last checked" timestamp and the per-service-refresh illusion.** Both mislead operators about freshness; tiny fixes (track `lastFetchedAt`, plumb a per-service refresh through the connections store).

Honorable mentions (cluster-of-cheap-wins, can be one PR each):
- B1 — Delete `lib/components/admin/Sidebar.svelte` (dead code).
- B7, B8, B14 — Replace "Coming soon" toasts and empty `onclick={() => {}}` with feature-flags or hide controls.
- B20 — Replace `prompt()` in `courses/[id]/+page.svelte` with the existing `ModuleFormModal.svelte` / `ConfirmationModal.svelte`.
- Migrate one-shot `$effect(() => loadX())` patterns (`users/+page.svelte:14`, `blog/+page.svelte:136`, `connections/+page.svelte:282`, `courses/+page.svelte:262`) to `onMount` per CLAUDE.md.
