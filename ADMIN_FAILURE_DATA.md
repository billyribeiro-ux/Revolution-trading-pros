# Admin / Dashboard / Sidebar — Failure Data

**Date:** 2026-04-26
**Method:** Live HTTP probes against http://localhost:5173 + source code cross-reference
**Scope:** /admin/*, /dashboard/*, sidebar components, admin shell
**Companion doc:** This sweep extends `ADMIN_DASHBOARD_REPORT.md`. Findings already documented there are referenced as "(B-N — see ADMIN_DASHBOARD_REPORT.md)" rather than re-derived.

---

## Executive summary

- Admin `+page.svelte` files scanned: **141** (12 more than the 129 in the prior report; new ones are mostly under `/admin/crm/*` and `/admin/courses/[id]/lessons/[lessonId]`)
- Dashboard `+page.svelte` files scanned: **88**
- 5xx responses on a 19-route sample probe: **0** (every probed route 303→login as expected)
- New (not previously documented) bug classes identified: **8** — see §10
- Frontend proxies (`+server.ts`) total: **40** (39 admin + 1 my)
- Admin proxies still on legacy `Authorization`-header pattern instead of `cookies.get('rtp_access_token')`: **23 of 39** (59%)
- Backend route files (`api/src/routes/*.rs`): **73**
- Top-priority new finding: **`/logout/+page.server.ts:39-42` deletes the wrong cookie names** — session survives logout for any user who lands there from the dashboard sidebar.
- Second-priority new finding: **POST to any sub-path under `api/admin/posts/`, `api/admin/categories/`, `api/admin/tags/`, `api/admin/coupons/`, `api/admin/users/`, `api/admin/email/templates/`, `api/admin/crm/contacts/` returns 405 from SvelteKit's dev resolver** (not 404 from the backend). This is a structural failure of the frontend routing tree, not a missing-proxy issue.

---

## 1. Backend admin route inventory

The prior report listed 35 sample probes. Adding a new 19-row sample below covering all top-level dashboard surfaces and three admin subroutes; status patterns are identical to the prior report (303→login). No 5xx, no slow >2s observed.

| Route | HTTP | Latency | Notes |
|---|---|---|---|
| `/admin` | 303 | 0.0016s | OK |
| `/admin/dashboard` | 303 | 0.0011s | OK (duplicate of /admin per prior report) |
| `/dashboard` | 303 | 0.0015s | OK |
| `/dashboard/account` | 303 | 0.0026s | OK |
| `/dashboard/account/orders` | 303 | 0.0029s | OK |
| `/dashboard/account/subscriptions` | 303 | 0.0023s | OK |
| `/dashboard/account/payment-methods` | 303 | 0.0024s | OK |
| `/dashboard/account/coupons` | 303 | 0.0021s | OK |
| `/dashboard/account/edit-account` | 303 | 0.0025s | OK |
| `/dashboard/account/edit-address` | 303 | 0.0024s | OK |
| `/dashboard/classes` | 303 | 0.0010s | OK |
| `/dashboard/indicators` | 303 | 0.0010s | OK |
| `/dashboard/explosive-swings` | 303 | 0.0068s | Slowest in sample, still well under 2s |
| `/dashboard/spx-profit-pulse` | 303 | 0.0046s | OK |
| `/dashboard/day-trading-room` | 303 | 0.0034s | OK |
| `/dashboard/swing-trading-room` | 303 | 0.0020s | OK |
| `/dashboard/small-account-mentorship` | 303 | 0.0032s | OK |
| `/dashboard/weekly-watchlist` | 303 | 0.0009s | OK |
| `/dashboard/high-octane-scanner` | 303 | 0.0047s | OK |

(See `ADMIN_DASHBOARD_REPORT.md` §1 for the prior 35-route admin sample.)

---

## 2. Dashboard route inventory

88 `+page.svelte` files under `frontend/src/routes/dashboard/`. Notable structure points discovered during the sweep:

- `dashboard/[room_slug]/` is a dynamic catch directory (`daily-videos`, `learning-center`, `trading-room-archive`, `video/[slug]`) with **no top-level `+page.svelte`**. The DashboardSidebar at `frontend/src/lib/components/dashboard/DashboardSidebar.svelte:172,190,211` generates `href: /dashboard/${membership.slug}/` for every membership type. If the membership.slug is anything other than the seven hard-coded directory names (`day-trading-room`, `swing-trading-room`, `small-account-mentorship`, `explosive-swings`, `spx-profit-pulse`, `high-octane-scanner`, `weekly-watchlist`), the click lands on a `dashboard/[room_slug]/` directory that has no index page — **404 once auth resolves**. (Auth guard hides this on a logged-out probe.)
- `dashboard/spx-profit-pulse/` ships `billy-ribeiro/`, `learning-center/`, `premium-videos/`, `start-here/`, `alerts/`, but the secondary sidebar at `frontend/src/routes/dashboard/+layout.svelte:330,332,337,344` links to `/dashboard/spx-profit-pulse/jonathan-mckeever[/...]` — three broken submenu links plus one broken top-level "Trader Store" link. **No `jonathan-mckeever/` directory exists.** (NEW — not in prior report.)
- `dashboard/[room_slug]/video/[slug]/+page.svelte:32,37,42` and `dashboard/[room_slug]/daily-videos/+page.svelte:39,44,49` ship six `tradingRooms` dropdown entries with `href: '#', // TODO: Provide URL`. (NEW — not in prior report.)

---

## 3. Orphan API proxies (frontend → no backend) and frontend → 405 cliffs

### 3a. Frontend routing 405-trap (NEW, severe)

When a directory under `frontend/src/routes/api/admin/` has its own `+server.ts` AND no `[id]/` or `[...rest]/` dynamic child, **POST requests to any sub-path one level deep return `405 Method Not Allowed` directly from SvelteKit** without ever reaching the backend or the catch-all proxy at `frontend/src/routes/api/[...path]/+server.ts`. GET, PUT, DELETE on the same paths fall through to the catch-all → backend (returning 401 / 404 from the backend as appropriate).

Curl-confirmed cliffs (each row probed live, all return POST=405 GET=401):

| Path probed | GET | POST | PUT | DELETE | Affected page |
|---|---|---|---|---|---|
| `/api/admin/posts/bulk-delete` | 401 | **405** | n/a | 401 | `admin/blog/+page.svelte:302` |
| `/api/admin/posts/bulk-status` | 401 | **405** | n/a | 401 | `admin/blog/+page.svelte:324` |
| `/api/admin/posts/123` | 401 | **405** | n/a | 401 | `admin/blog/+page.svelte:354` (DELETE works, but pattern is broken) |
| `/api/admin/posts/export` | 401 | **405** | n/a | 401 | `admin/blog/+page.svelte:420` |
| `/api/admin/posts/import` | 401 | **405** | n/a | 401 | `admin/blog/+page.svelte:447` |
| `/api/admin/categories/{anything}` | 401 | **405** | n/a | 401 | `admin/blog/categories/+page.svelte` |
| `/api/admin/tags/{anything}` | 401 | **405** | n/a | 401 | (used by tag editors) |
| `/api/admin/coupons/{anything}` | 401 | **405** | n/a | 401 | `admin/coupons/+page.svelte` |
| `/api/admin/users/{anything}` | 401 | **405** | n/a | 401 | `admin/users/create/+page.svelte` |
| `/api/admin/email/templates/{anything}` | 401 | **405** | n/a | 401 | template editors |
| `/api/admin/crm/contacts/{anything}` | 401 | **405** | n/a | 401 | CRM tag/note adders |

For comparison, paths whose parent dir has **no** `+server.ts` proxy (e.g. `/api/admin/anything-no-parent/bulk-delete`, `/api/admin/seo/metrics`, `/api/admin/orders`) correctly reach the catch-all → backend (404).

**Symptom:** Every "bulk delete", "duplicate", "status toggle", "featured toggle" action on the blog admin returns 405 with an empty body. The user sees a generic toast error and the action silently fails. Same shape on the categories/tags editors.

**Why:** SvelteKit's route resolver appears to treat the parent `+server.ts` as claiming all sub-paths for routing purposes. Whatever the underlying SvelteKit-2 routing rule is, the empirical behavior is consistent and reproducible.

**Suggested fix (cheapest):** Add a sibling `[...rest]/+server.ts` next to each parent `+server.ts` that re-delegates to the catch-all behavior — or convert each parent file into `posts/[...rest]/+server.ts` that explicitly handles unknown sub-paths. Alternative: inline-create dedicated `bulk-delete/+server.ts`, `[id]/duplicate/+server.ts`, etc.

### 3b. Endpoints called by frontend pages with no backend Rust route

Cross-referenced against `api/src/routes/*.rs` (73 files). The following endpoints called by admin pages have **no backend route**:

| Frontend caller | Path called | Backend route exists? | Result on call |
|---|---|---|---|
| `admin/dashboard/+page.svelte:98` | `/api/analytics/realtime` | **No** (`api/src/routes/realtime.rs` is unrelated WS realtime) | 404 from backend → "Not Connected" card |
| `admin/dashboard/+page.svelte:107` | `/api/payments/summary` | **No** (`payments.rs` has no `/summary`) | 404 → "Not Connected" |
| `admin/dashboard/+page.svelte:120` | `/api/admin/rooms/stats` | **No** | 404 → empty stats card |
| `admin/seo/+page.svelte:282` | `/api/admin/seo/metrics` | **No** (and the call is commented out per B13) | n/a |
| `admin/orders/+page.svelte:83,103,177` | `/api/admin/orders` | **No** (`orders.rs` exists but does not register `/admin/orders` mount — verify) | 404 (B5 — see ADMIN_DASHBOARD_REPORT.md) |
| `admin/consent/settings/+page.svelte:215,229,249` | `/api/admin/consent/settings*` | **No** | 404 → toast "Failed to load settings" → page renders default state forever (NEW) |
| `admin/site-health/+page.svelte:154,261` | `/api/admin/site-health[/run-tests]` | Verified 401 from backend (route exists) | OK auth-only |

### 3c. Orphan-proxy cleanup status

The prior report (B6, B10–B12) flagged 9 + 6 + 1 + 1 = 17 missing proxies. Verifying with the catch-all `/api/[...path]/+server.ts` in mind: the catch-all WOULD service these for GET/PUT/DELETE, but POST is 405-trapped per §3a for any path that has a parent +server.ts. The functional impact stands.

---

## 4. Wrong-cookie proxies still using legacy auth-header pattern

The prior report flagged `api/admin/users/+server.ts` as inconsistent. The full list as of 2026-04-26 is **23 of 39 admin proxies** still reading `request.headers.get('Authorization')` only:

```
api/admin/posts/+server.ts                         (line 43, 66, 90, 119)
api/admin/posts/stats/+server.ts                    (line 18)
api/admin/trading-rooms/+server.ts                  (line 43, 64)
api/admin/trading-rooms/traders/+server.ts          (line 116, 147)
api/admin/trading-rooms/videos/+server.ts           (line 156, 209)
api/admin/trading-rooms/videos/[slug]/+server.ts    (line 170)
api/admin/schedules/+server.ts                      (line 371, 424)
api/admin/schedules/bulk-delete/+server.ts          (line 55)
api/admin/schedules/bulk-update/+server.ts          (line 65)
api/admin/schedules/[id]/+server.ts                 (line 90, 129, 179)
api/admin/tags/+server.ts                           (line 19)
api/admin/coupons/+server.ts                        (line 33, 72)
api/admin/users/+server.ts                          (line 43, 64)
api/admin/users/[id]/+server.ts                     (line 90, 134, 174)
api/admin/member-management/+server.ts              (line 40)
api/admin/member-management/[id]/+server.ts         (line 22, 61, 100)
api/admin/categories/+server.ts                     (line 42, 64)
api/admin/analytics/dashboard/+server.ts            (line 82)
api/admin/crm/contacts/+server.ts                   (line 23)
api/admin/crm/deals/+server.ts                      (line 23)
api/admin/crm/stats/+server.ts                      (line 18)
api/admin/members/stats/+server.ts                  (line 17)
api/admin/email/templates/+server.ts                (line 42, 61)
```

The 16 already on the canonical `cookies.get('rtp_access_token')` pattern (correct):

```
api/admin/bunny/create-video/+server.ts
api/admin/bunny/upload/+server.ts
api/admin/bunny/uploads/+server.ts
api/admin/bunny/video-status/[guid]/+server.ts
api/admin/connections/+server.ts
api/admin/connections/[key]/connect/+server.ts
api/admin/connections/[key]/disconnect/+server.ts
api/admin/connections/status/+server.ts
api/admin/courses/+server.ts                        (still has commented-out auth_token line, see line 17)
api/admin/courses/[id]/+server.ts
api/admin/courses/[id]/publish/+server.ts
api/admin/courses/[id]/unpublish/+server.ts
api/admin/email/settings/+server.ts                 (no auth — verify intentional)
api/admin/membership-plans/+server.ts
api/admin/products/stats/+server.ts
api/admin/subscriptions/plans/stats/+server.ts
api/my/subscriptions/+server.ts
```

Pattern impact: the catch-all proxy at `api/[...path]/+server.ts:178,191-192` reads `cookies.get('rtp_access_token')` and forwards `Authorization: Bearer <token>`. The 23 dedicated proxies above bypass this and rely solely on the client `adminFetch` setting an `Authorization` header from `localStorage`. Net effect: a fresh login that has the cookie but cleared localStorage 401s on those 23 proxies.

---

## 5. New inert UI / stub data (not in prior report)

| File:line | Symptom | Suggested fix |
|---|---|---|
| `frontend/src/lib/components/CommandPalette.svelte:191-194` | "Clear Cache" Quick Action calls `localStorage.clear()` then reloads — wipes the access token, instantly logs the operator out. Misleading label. | Either rename to "Clear Cache & Logout" or scope the clear to non-auth keys (e.g. `Object.keys(localStorage).filter(k => !k.startsWith('rtp_'))`) |
| `frontend/src/routes/logout/+page.server.ts:39-42` | Deletes `auth_token`, `session_id`, `refresh_token`, `access_token` — none of which are real cookie names in this codebase. The actual `rtp_access_token` and `rtp_refresh_token` are NOT cleared. Session survives the dashboard "Log out" link at `dashboard/+layout.svelte:400` and `dashboard/account/+page.svelte:70`. The internal `fetch('/api/logout', POST)` does delete the right cookies on its own response, but those Set-Cookie headers don't propagate to the user-facing redirect response. | Replace cookie names with `rtp_access_token` and `rtp_refresh_token`, or have the page.server.ts delegate entirely to `goto('/api/auth/set-session', {method:'DELETE'})` then redirect. |
| `frontend/src/lib/components/admin/CourseDetailDrawer.svelte:512` | "Recent Enrollments" empty-state literally reads "Enrollment data coming soon" | Either fetch enrollments or hide the section |
| `frontend/src/lib/components/admin/CourseDetailDrawer.svelte:524,528,532,542,550,588` | Analytics tab shows literal `-` for Total Views, Unique Visitors, Conversion Rate, Avg Time on Course, Drop-off Rate; "Coming soon" for Revenue. 6 dead metrics. | Wire to backend or hide tab |
| `frontend/src/routes/dashboard/[room_slug]/video/[slug]/+page.svelte:32,37,42` | Three trading-rooms dropdown entries with `href: '#', // TODO: Provide URL` | Provide real hrefs or hide the dropdown |
| `frontend/src/routes/dashboard/[room_slug]/daily-videos/+page.svelte:39,44,49` | Same three `href: '#'` TODO entries | Same fix |
| `frontend/src/routes/dashboard/+layout.svelte:330,332,337,344` | SPX Profit Pulse "Meet Jonathan" submenu links to `/dashboard/spx-profit-pulse/jonathan-mckeever*` paths — only `billy-ribeiro/` exists in the directory tree. 4 broken sidebar links per logged-in SPX member. | Either rename `billy-ribeiro/` → `jonathan-mckeever/` or fix the layout's hardcoded slugs |
| `frontend/src/lib/components/PopupDisplay.svelte:432` | Shows literal text "Form integration coming soon..." | Wire popup → form mapping or hide form-typed popups |
| `frontend/src/routes/admin/seo/store-locator/+page.svelte:209` | `alert(JSON.stringify(schema, null, 2))` — debug alert shipped to admin operators | Replace with a Modal that shows formatted JSON |
| `frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte:1134` | `onclick={() => {}}` — second known empty handler in repo (B14 was the first) | Wire the action or remove the button |

Native `alert()` count under `frontend/src/routes/admin/`: **50** total uses. Native `confirm()` count: **11**. Most use cases are "save/delete confirmation" where the repo elsewhere has `ConfirmationModal.svelte`. (See ADMIN_DASHBOARD_REPORT.md B20 for a concrete cluster in `courses/[id]/+page.svelte`.)

---

## 6. Sidebar broken links

Re-verified the prior report's AdminSidebar audit (38 hrefs) — every `+page.svelte` exists. ✅

DashboardSidebar (`frontend/src/lib/components/dashboard/DashboardSidebar.svelte`) hrefs are membership-driven (lines 172, 190, 211), so the link health depends on whether the user's `membership.slug` matches one of the 7 hard-coded directories. The 8th-and-beyond memberships will all dead-end at `/dashboard/<slug>/` which has no `+page.svelte`. Static hrefs in the same file (158, 159, 160, 222, 232) all resolve. ✅

Dashboard `+layout.svelte` secondary sidebar hard-codes 4 broken `jonathan-mckeever` links (covered in §5).

TradingRoomSidebar (`frontend/src/lib/components/dashboard/TradingRoomSidebar.svelte:126,131,137,142`) — 4 hrefs, all resolve. ✅

`frontend/src/lib/components/admin/Sidebar.svelte` — dead component (B1), 3 hrefs that all resolve to real routes if anyone re-mounted the file. ✅

---

## 7. Reactivity smell verification (Phase 7)

| Site | Expected | Found | Status |
|---|---|---|---|
| `frontend/src/routes/admin/+layout.svelte:71-77` auth guard converted to onMount | yes | onMount at line 78, old $effect at lines 66-70 commented per FIX-2026-04-26 marker | ✅ |
| `frontend/src/lib/components/layout/AdminSidebar.svelte:130, 196, 199` uses `user.current` not `$user` | yes | `user.current` at lines 138, 204, 207 (line numbers shifted slightly; no `$user` autosubscribe in file) | ✅ |
| `frontend/src/routes/signup/+page.svelte:25-30` uses onMount not $effect | yes | onMount at line 36 (line shifted from 25-30 → 36); old $effect commented at lines 31-35. **Note:** line 37 still uses `$authStore` legacy autosubscribe — `legacy_pre_subscribe` still wires at component init, mitigating but not eliminating the original cascade. Low-severity smell. | ⚠ partial |
| `frontend/src/routes/account/sessions/+page.svelte:33-50` uses onMount | yes | onMount at line 50; old $effect commented 40-49. **Note:** lines 52, 56 still use `$isAuthenticated` and `$authStore` legacy autosubscribe inside onMount. Same caveat as signup. Low-severity smell. | ⚠ partial |
| `frontend/src/lib/stores/keyboard.svelte.ts:295-330` `init` doesn't self-spread; `destroy` doesn't reset `isInitialized` | yes | `init` at 295 — no `storeState = { ...storeState, ... }`; `destroy` at 345 has `// Old: isInitialized = false;` commented out. | ✅ |
| New $effect blocks reading `$store` legacy autosubscribe | none | Audited 152 `$effect` blocks across `routes/admin/`, `routes/dashboard/`, `lib/components/`. Only stale matches were the COMMENTED-OUT `$effect` blocks in `admin/+layout.svelte:66-70` (kept per FIX-2026-04-26 marker). No active offenders found. | ✅ |

**One-shot $effect anti-pattern (CLAUDE.md flag):** these still ship and should be migrated to `onMount`:

| File:line | $effect body |
|---|---|
| `frontend/src/routes/admin/users/+page.svelte:14` | `loadUsers()` |
| `frontend/src/routes/admin/blog/+page.svelte:136` | `loadPosts()` after browser guard |
| `frontend/src/routes/admin/courses/+page.svelte:276` | `if (browser) fetchCourses()` |
| `frontend/src/routes/admin/connections/+page.svelte:282` | async init wrapper, reads `connections` and `categories` runes |

(Last three are reiterations from the prior report's "honorable mentions"; first three were called out in §3 deep-dives.)

---

## 8. Static-asset 404s

Probed the assets referenced in the marketing-home and login HTML payloads:

| URL | Status | Referenced from | Notes |
|---|---|---|---|
| `/favicon.png` | 200 | layout `<link rel="icon">` | OK |
| `/manifest.json` | 200 | `<link rel="manifest">` | OK |
| `/atom.xml` | 200 | feed link | OK |
| `/feed.xml` | 200 | feed link | OK |
| `/revolution-trading-pros.png` | 200 | brand logo (admin sidebar, login) | OK |

**Zero static-asset 404s observed in the dev SSR HTML.** The login page does reference `/register` (which exists at `frontend/src/routes/register/+page.svelte`, 1237 LOC) — confirmed reachable; SEE §10 for the `/register` vs `/signup` duplication finding.

---

## 9. Other new findings

### 9.1 `/register` and `/signup` are duplicate signup pages

- `frontend/src/routes/register/+page.svelte` — 1237 LOC, branded "Join Revolution" / "register-card" / "register-title"
- `frontend/src/routes/signup/+page.svelte` — 296 LOC, plain Tailwind form

Both routes are live (`/usr/bin/curl` returns 200 on each). The login page (`frontend/src/routes/login/+page.svelte`, rendered HTML at line ~538k bytes) links to `/register`. Other pages (`courses/options-trading/+page.svelte:101,165`, `courses/swing-trading-pro/+page.svelte:232,390`, `verify-email/[id]/[hash]/+page.svelte:107`) also link to `/register`.

**Symptom:** Two signup paths with different feature surfaces and different validation, both hitting the same `authStore.signup()`. `/signup` is at risk of going stale; auditors will get inconsistent UX based on which path they took. NEW finding.

### 9.2 Dashboard layout's `setUser` reads `$isAuthenticated` inside onMount but writes `authStore.setUser` synchronously

`frontend/src/routes/dashboard/+layout.svelte:457` — onMount body branches on `$isAuthenticated` (legacy autosubscribe) before calling `authStore.setUser(serverUser)`. The current onMount-not-$effect form is correct (no reactive cascade). But the `legacy_pre_subscribe` registered at component init still subscribes and re-runs `getSnapshot()` on every store mutation that follows, including the `setUser(serverUser)` write below. No depth-error because onMount is untracked, but it's an unnecessary subscription that the FIX-2026-04-26 sweep set out to eliminate. Low-severity smell, same pattern as signup/sessions in §7.

### 9.3 `/admin/dashboard` (the duplicate) calls 3 backend endpoints that don't exist

`frontend/src/routes/admin/dashboard/+page.svelte:98,107,120` — `/api/analytics/realtime`, `/api/payments/summary`, `/api/admin/rooms/stats`. None of these have backend routes (verified by grep across `api/src/routes/*.rs`). The page degrades to "Not Connected" cards which is the documented behavior. But the duplicate `/admin` dashboard at `frontend/src/routes/admin/+page.svelte` doesn't share this codepath; the AdminSidebar points to `/admin` not `/admin/dashboard`. The `/admin/dashboard` page is effectively orphan, AND its KPI cards are all reading endpoints that don't exist.

### 9.4 No `finally` on `loading=true` in `admin/dashboard/+page.svelte:155-157` and `admin/consent/settings/+page.svelte:212-223`

Both pages have a `loading = true` set followed by `await`, with the `loading = false` reset placed AFTER the catch (consent) or AFTER `Promise.all()` resolve (dashboard) — not in `finally`. If an unrecognized throw escapes the catch (e.g. `JSON.parse` fail in adminFetch's parse layer), the spinner sticks. Low-severity in practice because the inner fetchers each have their own try/catch that fail-soft, but fragile.

### 9.5 `admin/consent/settings/+page.svelte` is wholly broken

All three calls (`/api/admin/consent/settings`, `.../bulk`, `.../reset`) hit a backend route that doesn't exist (no `consent.rs` under `api/src/routes/`). Catch-all proxy forwards GET → backend 404 → page error toast → page renders default state. Save/reset always fail. NEW finding.

### 9.6 `dashboard/account/view-subscription/[id]/+page.server.ts:60,73` ships `// TODO: Call actual API when backend is ready`

The page server-loader returns hard-coded subscription detail data. Live page renders the same placeholder for any subscription ID. NEW finding.

### 9.7 `dashboard/[room_slug]/learning-center/+page.server.ts:75`, `daily-videos/+page.server.ts:45`, `trading-room-archive/+page.server.ts:93` all ship `// TODO: Implement new video fetching approach`

Three room-scoped server loaders return placeholder/static data. Same pattern in `dashboard/day-trading-room/learning-center/+page.server.ts:54`, `dashboard/small-account-mentorship/learning-center/+page.server.ts:54`, `dashboard/small-account-mentorship/trading-room-archive/+page.server.ts:63`, `dashboard/spx-profit-pulse/learning-center/+page.server.ts:54`. **At least 7 dashboard server loaders are not yet wired to the new video API.** NEW.

### 9.8 `frontend/src/lib/api/forms.ts:62` constant `API_BASE = '/api'` (not `/api/forms` or proxy-aware)

The forms client builds URLs as `${API_BASE}/forms/...` → `/api/forms/...`. There's no dedicated `/api/forms/+server.ts` proxy; calls fall through to the catch-all `/api/[...path]/+server.ts`. That's fine for GET (catch-all forwards). But `POST /api/forms/<id>/submissions/bulk-update-status` (line 1553), `POST /api/forms/<id>/submissions/bulk-delete` (line 1567), and `PATCH /api/forms/<id>/submissions/<id>/status` (line 1523) are new POSTs/PATCHes — verified live: `/api/forms` POST returns 200 with empty data, GET returns 200 with valid response, so the catch-all is OK here. **No 405 trap because there's no parent `+server.ts` at `/api/forms/`.** Just want to confirm it's not affected by §3a.

---

## 10. Top 10 fix priorities (ranked by user impact × ease of fix)

1. **Fix `/logout/+page.server.ts:39-42` cookie names** — replace 4 wrong cookie names with `rtp_access_token` + `rtp_refresh_token`. Trivial 4-line edit, eliminates a security-grade silent bug (session survives "Log out" click on dashboard). NEW.
2. **Resolve the SvelteKit POST=405 cliff for `api/admin/posts/*`, `api/admin/categories/*`, `api/admin/tags/*`, `api/admin/coupons/*`, `api/admin/users/*`, `api/admin/email/templates/*`, `api/admin/crm/contacts/*`** — easiest path is a per-folder `[...rest]/+server.ts` that delegates to a shared proxy helper. Unblocks blog admin's 9 broken bulk/mutation actions (B6) AND the same shape on 6 other admin areas. NEW.
3. **Migrate the 23 admin proxies still on `Authorization`-header-only auth to `cookies.get('rtp_access_token')`** — single mechanical refactor across 23 files. Eliminates the cookie-fresh-but-localStorage-empty 401s flagged in the prior report's priority 9.
4. **Fix `dashboard/+layout.svelte:330,332,337,344` SPX submenu hardcoded slugs** (`jonathan-mckeever` → `billy-ribeiro` or vice-versa, depending on which is canonical). 4 broken sidebar links for SPX members. NEW.
5. **Fix CourseDetailDrawer dead metrics tab** — `lib/components/admin/CourseDetailDrawer.svelte:524,528,532,542,550,588`. Either fetch real values or hide the tab. NEW.
6. **Fix `/admin/consent/settings`** — either ship a backend `consent.rs` route or remove the page from the admin nav. Currently 100% broken on every operation. NEW.
7. **Wire the 7 dashboard server loaders that ship `// TODO: Implement new video fetching approach`** — at least three room-scoped pages (learning-center, daily-videos, trading-room-archive) render stale/empty data right now. NEW.
8. **Fix `/admin/dashboard` orphan endpoints** — either remove the duplicate page or stub the 3 missing backend routes. NEW.
9. **Replace CommandPalette "Clear Cache" semantics** — `lib/components/CommandPalette.svelte:191-194` currently logs the user out. Rename or scope. NEW.
10. **Replace native `alert()` (50 sites) and `confirm()` (11 sites) under `/admin/`** with the existing `ConfirmationModal.svelte` / `Modal.svelte`. The most embarrassing instance is `admin/seo/store-locator/+page.svelte:209` `alert(JSON.stringify(schema, null, 2))`. Volume cleanup; can be split across many small PRs.

Honorable mentions still pending from the prior report: B1 (delete dead `lib/components/admin/Sidebar.svelte`), B2 (page-builder route), B3 (CommandPalette wrong nav targets), B7/B8/B14 (Coming-soon toasts), B16/B17/B18 (ConnectionHealthPanel mock metrics).
