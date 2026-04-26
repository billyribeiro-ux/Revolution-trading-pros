# 01 — Admin Shell + Dashboard Audit

Auditor: Principal-engineer pass, read-only.
Date: 2026-04-26.
Scope: admin layout/error/index, `/admin/dashboard`, `AdminSidebar`, `AdminToolbar`, every file under `frontend/src/lib/components/admin/`.

---

## Files reviewed

Shell + dashboard:

- [+layout.svelte](frontend/src/routes/admin/+layout.svelte)
- [+layout.ts](frontend/src/routes/admin/+layout.ts)
- [+page.svelte](frontend/src/routes/admin/+page.svelte) (admin index — the "Analytics Dashboard" page)
- [+error.svelte](frontend/src/routes/admin/+error.svelte)
- [dashboard/+page.svelte](frontend/src/routes/admin/dashboard/+page.svelte) (the secondary "Admin Dashboard" page)

Sidebar / toolbar:

- [layout/AdminSidebar.svelte](frontend/src/lib/components/layout/AdminSidebar.svelte)
- [AdminToolbar.svelte](frontend/src/lib/components/AdminToolbar.svelte)

`frontend/src/lib/components/admin/` (29 files):

- ActionsDropdown, AdminCard, BulkEditModal, BulkUploadQueue, BunnyVideoUploader, ConfirmationModal, ConnectionGate, CourseBuilder, CourseDetailDrawer, CourseFormModal, IndicatorBuilder, MediaUploadHub, MemberDetailDrawer, MemberFormModal, ModuleFormModal, RoomSelector, ScheduledPublishing, SegmentDetailDrawer, ServiceConnectionStatus, Sidebar, StatCard, SubscriptionDetailDrawer, SubscriptionFormModal, TabPanel, TemplateForm, VideoAnalyticsDashboard, VideoChaptersEditor, VideoUploader.

---

## Critical bugs (P0)

### P0-1 — Two parallel "Admin Dashboard" routes ship simultaneously

There are two competing dashboard pages that both render inside the admin shell and both fetch overlapping data:

- [+page.svelte:391](frontend/src/routes/admin/+page.svelte#L391) — "Analytics Dashboard" (centered hero, period picker, ~1500 lines).
- [dashboard/+page.svelte:212](frontend/src/routes/admin/dashboard/+page.svelte#L212) — "Admin Dashboard" (Tailwind-classed grid, no period picker, ~1100 lines).

Only the second one is reachable via the nav (sidebar `Overview` points to `/admin`, no link points to `/admin/dashboard`; the top-level layout-title map at [+layout.svelte:153](frontend/src/routes/admin/+layout.svelte#L153) doesn't even know about `dashboard`). So `/admin/dashboard` is a discoverability dead-end that still pulls `/api/admin/connections/summary`, `/api/analytics/realtime`, `/api/payments/summary`, `/api/admin/rooms/stats` every 60 s once any user lands on it. Pick one canonical page; either delete the orphan or wire the nav to it.

### P0-2 — Admin shell has *zero* role enforcement; only checks "any session"

[+layout.svelte:78-83](frontend/src/routes/admin/+layout.svelte#L78) only verifies `isAuthenticated.current`:

```ts
onMount(() => {
    if (!browser) return;
    if (!isAuthenticated.current) {
        goto('/login?redirect=/admin');
    }
});
```

Any authenticated regular user can render every admin page. The check that the user is an admin or superadmin happens *only* inside `AdminToolbar` (used elsewhere) at [AdminToolbar.svelte:194-216](frontend/src/lib/components/AdminToolbar.svelte#L194), which gates the toolbar UI but not the routes. The admin index (`+page.svelte`), the secondary dashboard, and every page under `/admin/*` will fetch admin endpoints with whatever token the user holds — relying entirely on the Rust API to 403. This is a defense-in-depth gap. Add an `isSuperadmin(user.current)` (or `isAdmin(...)`) guard in the layout's onMount, with a 403 redirect to `/`.

Bonus: the load function at [+layout.ts:1-4](frontend/src/routes/admin/+layout.ts#L1) is just `ssr=false; prerender=false;` — no `parent()` call, no auth check at the load level, so even pages that *do* `await parent()` get nothing useful from the admin layout.

### P0-3 — `+error.svelte` flex-layout breaks on any error

[+error.svelte:232-238](frontend/src/routes/admin/+error.svelte#L232):

```css
.admin-error-page {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-base);
    padding: 2rem;
}
```

There is no `min-height` / `height` set, so the flex container collapses to its child's intrinsic height and the centered card sits flush at the top of the admin content viewport — looks broken whenever an admin route throws. Add `min-height: calc(100dvh - var(--admin-header-height, 70px))` or an explicit min-height.

Also: when the user lands here with a 401 the layout's onMount has already redirected to `/login`, so the 401 branch's "Sign In" button is dead code. With `clearAuth()` followed by `goto(...)` at [+error.svelte:127-130](frontend/src/routes/admin/+error.svelte#L127) the user briefly sees the error card before the layout's redirect fires — small UX wart, not a P0 by itself.

### P0-4 — `MediaUploadHub.svelte` and `BunnyVideoUploader.svelte` bypass the SvelteKit proxy and hit the Fly.io API host directly with `credentials: 'include'`

[MediaUploadHub.svelte:312](frontend/src/lib/components/admin/MediaUploadHub.svelte#L312), [:326](frontend/src/lib/components/admin/MediaUploadHub.svelte#L326), [:386](frontend/src/lib/components/admin/MediaUploadHub.svelte#L386):

```ts
xhr.open('POST', `${API_BASE_URL}${API_ENDPOINTS.admin.media.upload}`);
xhr.withCredentials = true;
…
const createResponse = await fetch(`${API_BASE_URL}${API_ENDPOINTS.admin.bunny.createVideo}`, {
    …
    credentials: 'include',
    …
});
```

`API_BASE_URL` resolves to `https://revolution-trading-pros-api.fly.dev` ([api/config.ts:11-17](frontend/src/lib/api/config.ts#L11)). The auth state in this app is held in the `rtp_access_token` cookie scoped to the *frontend* host; cross-origin XHR with `withCredentials` does not transmit that cookie, and even if you set up CORS+SameSite=None the cookie is HttpOnly to the frontend domain, not the API domain. Net: media + bunny uploads from the admin will silently 401 in production unless the backend skips auth for these routes (which would be its own P0). Mirror the pattern used elsewhere — proxy via `frontend/src/routes/api/admin/bunny/...` and `…/media/...` `+server.ts` shims that pull `env.API_BASE_URL || env.BACKEND_URL` from `$env/dynamic/private`.

[BunnyVideoUploader.svelte:116](frontend/src/lib/components/admin/BunnyVideoUploader.svelte#L116) and [:164](frontend/src/lib/components/admin/BunnyVideoUploader.svelte#L164) are okay by default (the prop defaults to `/api/admin/bunny`, a relative path), but the `apiBase` prop is `string`-typed and any caller can override it with the cross-origin URL — same shape of bug.

### P0-5 — `dashboard/+page.svelte` uses an `$effect` for one-shot init; same anti-pattern the `+layout.svelte` was just fixed for

[dashboard/+page.svelte:184-194](frontend/src/routes/admin/dashboard/+page.svelte#L184):

```ts
$effect(() => {
    if (!browser) return;
    loadDashboard();
    refreshInterval = setInterval(loadDashboard, 60000);
    return () => {
        if (refreshInterval) clearInterval(refreshInterval);
    };
});
```

`loadDashboard()` writes to `connections`, `metrics`, `roomStats`, `lastUpdated`, `isLoading` — all `$state` runes used elsewhere in the component, including by `$derived` (`connectedCount`, `totalServices`). The cleanup function is only valid for an effect that the runtime *re-runs*; a `$effect` with no tracked deps acts like a self-loop hazard since the runtime can re-evaluate it. The PRINCIPAL-2026-04-26 comment in [+layout.svelte:53-77](frontend/src/routes/admin/+layout.svelte#L53) calls this exact pattern "the classic write-while-reading-tracked-dep pattern that trips effect_update_depth_exceeded" and converts it to `onMount`. This page was missed in that sweep — convert to `onMount` with the same reasoning.

---

## High-severity issues (P1)

### P1-1 — `AdminToolbar.svelte` still uses legacy `$authStore` / `$userStore` autosubscribes everywhere

[AdminToolbar.svelte:190](frontend/src/lib/components/AdminToolbar.svelte#L190), [:197](frontend/src/lib/components/AdminToolbar.svelte#L197), [:250](frontend/src/lib/components/AdminToolbar.svelte#L250), [:587](frontend/src/lib/components/AdminToolbar.svelte#L587), [:593](frontend/src/lib/components/AdminToolbar.svelte#L593):

```ts
const currentUser = $derived($userStore as AdminUser | null);
const isAdmin = $derived(
    (() => {
        if (!$authStore.isAuthenticated) return false;
        …
    })()
);
…
if (!browser || !$authStore.isAuthenticated) return;
…
if (token && !$userStore) {
```

These are exactly the autosubscribes that the recent FIX-2026-04-26 pass eliminated from `AdminSidebar.svelte` and `+layout.svelte` (they cite the same hazard in the inline comments at [+layout.svelte:71-77](frontend/src/routes/admin/+layout.svelte#L71) and [AdminSidebar.svelte:128-136](frontend/src/lib/components/layout/AdminSidebar.svelte#L128)). `AdminToolbar` is the last hold-out and is rendered above NavBar at z-index 10100 ([AdminToolbar.svelte:883](frontend/src/lib/components/AdminToolbar.svelte#L883)), so it participates in every authenticated render. Convert to `authStore.isAuthenticated` / `user.current` rune-getters.

### P1-2 — Admin Sidebar's legacy `Sidebar.svelte` ships in parallel with the canonical `layout/AdminSidebar.svelte`

[admin/Sidebar.svelte:1-82](frontend/src/lib/components/admin/Sidebar.svelte#L1) is a fully-built nav sidebar that also reads `$authStore` directly ([:134](frontend/src/lib/components/admin/Sidebar.svelte#L134), [:138](frontend/src/lib/components/admin/Sidebar.svelte#L138), [:139](frontend/src/lib/components/admin/Sidebar.svelte#L139)), defines its own `navigation` array (only Dashboard / Forms / SEO — outdated), and calls `authStore.logout()` directly in `handleLogout` at [:64-67](frontend/src/lib/components/admin/Sidebar.svelte#L64). The active sidebar at [`layout/AdminSidebar.svelte`](frontend/src/lib/components/layout/AdminSidebar.svelte) has 6 sections and 26 items; this one has 3. Nothing in the audited scope imports `admin/Sidebar.svelte`. Either delete it or document why both exist; right now it's an orphan that drifts further every release.

### P1-3 — `+page.svelte` admin index ignores the auth token in some headers and sends `Bearer …` *and* `credentials: 'include'`

[+page.svelte:62-95](frontend/src/routes/admin/+page.svelte#L62):

```ts
async function localFetch<T = any>(endpoint: string): Promise<T> {
    …
    const token = getAuthToken();
    const headers: Record<string, string> = { … };
    if (token) headers['Authorization'] = `Bearer ${token}`;
    …
    const response = await fetch(url, {
        method: 'GET',
        headers,
        credentials: 'include',
        …
    });
    …
}
```

The path here is always relative (`/api/...` → same-origin proxy), so `credentials: 'include'` already attaches the `rtp_access_token` cookie. Adding a `Bearer` header on top is harmless if the proxy ignores it but creates two race conditions:

1. If `getAuthToken()` returns a stale token from `localStorage`/store while the cookie has been refreshed, the proxy may prefer one over the other — a cause of "logged out but still rendering admin" reports.
2. The new convention (per the recent commit `e2356fa46` cited in repo history: "prefer rtp_access_token cookie") is to drop the bearer header for admin proxies. This file predates that.

Drop `headers['Authorization']` here.

### P1-4 — `localFetch` swallows error bodies; the dashboard error banner says only "Failed to load some statistics"

[+page.svelte:88-90](frontend/src/routes/admin/+page.svelte#L88) throws `new Error(\`HTTP ${response.status}\`)`, not the response body. Every promise-rejection is then caught at [+page.svelte:347-348](frontend/src/routes/admin/+page.svelte#L347) and replaced with the generic banner. Admins debugging a 502 from `/api/admin/coupons` see "Failed to load some statistics" with no path, status, or body. At the very least include `response.statusText` and the failing endpoint in the thrown error.

### P1-5 — Dashboard period change races with concurrent fetches

[+page.svelte:368-372](frontend/src/routes/admin/+page.svelte#L368) calls `fetchDashboardStats({ userInitiated: true })` synchronously after setting `selectedPeriod`. There's no abort of the in-flight `Promise.allSettled` from a prior period; the older response can land *after* the newer one and overwrite it (especially when one of the 5 endpoints stalls near `FETCH_TIMEOUT_MS = 10_000`). Add an `AbortController` per call and abort the previous one when `changePeriod` fires.

### P1-6 — `analyticsConnected = true; seoConnected = true;` is overwritten by `onMount`, but the `fetchDashboardStats` path still hard-codes `true` first

[+page.svelte:188-189](frontend/src/routes/admin/+page.svelte#L188):

```ts
analyticsConnected = true; // Platform has built-in analytics
seoConnected = true; // Platform has built-in SEO
```

Then [+page.svelte:380-388](frontend/src/routes/admin/+page.svelte#L380):

```ts
onMount(() => {
    mounted = true;
    analyticsConnected = getIsAnalyticsConnected();
    seoConnected = getIsSeoConnected();
    …
    fetchDashboardStats();
});
```

`fetchDashboardStats()` (called inside `onMount`) re-asserts `true` *after* `getIsAnalyticsConnected()` / `getIsSeoConnected()` were just used to set the real values. Net: the "Connect Analytics" CTA path at [:467-484](frontend/src/routes/admin/+page.svelte#L467) is unreachable because `analyticsConnected` is always `true` by the time the template re-runs. Either remove the hard-coded assignment in `fetchDashboardStats` or compute the flag once at the top of the function from the connections store.

### P1-7 — `IconChevronDown` `class` prop usage is unsupported on Tabler runes icons

[AdminToolbar.svelte:697](frontend/src/lib/components/AdminToolbar.svelte#L697), [:766](frontend/src/lib/components/AdminToolbar.svelte#L766), [:809](frontend/src/lib/components/AdminToolbar.svelte#L809):

```svelte
<IconChevronDown size={16} class={showQuickMenu ? 'rotate' : ''} aria-hidden="true" />
<IconRefresh size={18} class="spin" aria-hidden="true" />
```

`@tabler/icons-svelte-runes` icon components don't accept a `class` prop in their public types; the rotation/spin styling here may be a no-op (and the file relies on `:global(.admin-toolbar .rotate)` / `:global(.admin-toolbar .spin)` which only matches if Tabler forwards the class to the rendered `<svg>`). Verify with the `svelte-autofixer` MCP before shipping; if the class isn't being forwarded, wrap the icons in a `<span class="rotate">…</span>`.

### P1-8 — `dashboard/+page.svelte` mutates a `$state` object's keys via `as any` indexer

[dashboard/+page.svelte:80-86](frontend/src/routes/admin/dashboard/+page.svelte#L80):

```ts
if (data.connections) {
    for (const conn of data.connections) {
        if (conn.is_connected) {
            (connections as any)[conn.key] = true;
        }
    }
}
```

Three problems: (1) `as any` discards the `ConnectionStatus` interface — any malformed key from the API silently inserts a new property; (2) deep mutation on a Svelte 5 `$state` object works at runtime but is brittle if the proxy semantics change; (3) once `connections.google_analytics = true`, the code never *un*-sets it on the next refresh, so once-connected services appear connected forever even after an admin disconnects them. Replace with `connections = { ...connections, [conn.key]: conn.is_connected }` filtered by the known ConnectionStatus keys.

### P1-9 — `getStatsForRoom` warns on every render for missing rooms

[dashboard/+page.svelte:138-151](frontend/src/routes/admin/dashboard/+page.svelte#L138):

```ts
function getStatsForRoom(roomId: string): RoomStats {
    const roomExists: Room | undefined = ROOMS.find((r) => r.id === roomId);
    if (!roomExists) console.warn(`Room ${roomId} not found in ROOMS config`);
    …
}
```

Called inside `{#each ROOMS as room}` so `roomId` is always from `ROOMS` — the warn is dead code. But if `roomStats` from the API contains a stale id, the function is also called from the render loop and the warn fires every paint. Either remove the warn or restructure.

### P1-10 — `+layout.svelte` keyboard shortcuts hard-code `goto('/admin/blog')` etc. but the sidebar canonical href is `/admin/blog`

OK — those match. Not a bug. But [+layout.svelte:120-126](frontend/src/routes/admin/+layout.svelte#L120) hard-codes a closed list of 4 destinations (`dashboard, analytics, blog, settings`) for keyboard nav while the sidebar exposes 26. Inconsistent. Either keep the shortcut list intentionally short and document it, or pull it from the same source as the sidebar.

---

## Medium issues (P2)

### P2-1 — Init-as-effect anti-pattern repeated across 8 admin components

These all do `$effect(() => loadFn())` for one-time init, mirror of the dashboard P0-5:

- [CourseBuilder.svelte:97-99](frontend/src/lib/components/admin/CourseBuilder.svelte#L97) — `$effect(() => { loadCourse(); });`
- [VideoChaptersEditor.svelte:50-52](frontend/src/lib/components/admin/VideoChaptersEditor.svelte#L50) — `$effect(() => { loadChapters(); });`
- [TemplateForm.svelte:30-41](frontend/src/lib/components/admin/TemplateForm.svelte#L30) — sync on prop change (this one is legit — it tracks `template`).
- [ServiceConnectionStatus.svelte:251-255](frontend/src/lib/components/admin/ServiceConnectionStatus.svelte#L251) — `if (isConnected && onConnected) onConnected();` — fires `onConnected` on every isConnected toggle including initial render; consumers may receive duplicate events.
- All `*-FormModal.svelte` and `*-DetailDrawer.svelte` files have a `$effect(() => { if (isOpen) document.body.style.overflow = 'hidden'; … })`. This is fine *except* the cleanup `() => { document.body.style.overflow = ''; }` runs on every re-evaluation, including on `isOpen` going from false→false (no-op) — harmless but noisy. The body-scroll-lock should live in the modal close handler, not effect cleanup.

### P2-2 — `*-FormModal.svelte` "reset on isOpen" pattern overwrites user-entered drafts on re-render

Example, [CourseFormModal.svelte:90-124](frontend/src/lib/components/admin/CourseFormModal.svelte#L90):

```ts
$effect(() => {
    if (isOpen && mode === 'edit' && course) { … overwrite all fields … }
    else if (isOpen && mode === 'create') { … reset all fields … }
    error = '';
    activeSection = 'basic';
});
```

Tracked deps: `isOpen`, `mode`, `course`. If the parent re-creates the `course` object reference (very common with SvelteKit invalidation), the effect re-runs and *blows away the half-typed form*. This pattern repeats in `MemberFormModal.svelte:147`, `ModuleFormModal.svelte:60`, `SubscriptionFormModal.svelte:71`. Use a key-on-id `{#key course?.id}…{/key}` mount/remount pattern, or gate with `untrack(course)` on the field-population branch.

### P2-3 — `MemberFormModal.svelte` defines extensive profile fields it never sends

[MemberFormModal.svelte:65-91](frontend/src/lib/components/admin/MemberFormModal.svelte#L65) declares `phone`, `company`, `jobTitle`, `bio`, `timezone`, `subscriptionTier`, `accountStatus`, `expirationDate`, `tags`, `enableTwoFactor`, `emailNotifications`, `smsNotifications`, `marketingEmails`, `adminNotes` — all `$state`-tracked, all rendered into the form. The `// Extended fields would be populated from member data` comment on [:153](frontend/src/lib/components/admin/MemberFormModal.svelte#L153) and the absence of these in the `CreateMemberRequest` payload (the file is 2202 lines, see header at [:1-102](frontend/src/lib/components/admin/MemberFormModal.svelte#L1)) mean the admin types data and clicks Save and *nothing happens to those fields*. Either gut the unused fields with a comment or wire them through. Right now this is data loss.

### P2-4 — `dashboard/+page.svelte` interface omits `sendgrid` (declared) and the API returns it but it's compared against `connections.stripe` only

[dashboard/+page.svelte:29-36](frontend/src/routes/admin/dashboard/+page.svelte#L29) declares `sendgrid: boolean` but the UI never reads it — every email-related metric is gated on `connections.stripe`. Either remove `sendgrid` from the interface or wire it up.

### P2-5 — Loading/no-data flicker on the dashboard index

[+page.svelte:107](frontend/src/routes/admin/+page.svelte#L107) `let isLoading = $state(true)` plus [:152](frontend/src/routes/admin/+page.svelte#L152) `let seoMetrics = { error404Count: { value: null, hits: 0 }, redirections: { count: null, hits: 0 } }` — but `seoMetrics` is *not* a `$state` rune (it's a plain object). Re-rendering does not update the SEO metric panel after `fetchDashboardStats` mutates fields like `seoMetrics.searchTraffic.value = data.seo.search_traffic`. Reactivity is "accidentally" achieved because the same render also flips `isLoading=false`, which *is* a `$state` and triggers the re-render. If `isLoading` is short-circuited (e.g. cached), the SEO panel stays blank. Wrap `seoMetrics` in `$state({...})`.

Same issue applies to `stats` ([:121-128](frontend/src/routes/admin/+page.svelte#L121)), `analytics` ([:131-138](frontend/src/routes/admin/+page.svelte#L131)), and `deviceBreakdown` ([:154](frontend/src/routes/admin/+page.svelte#L154)).

### P2-6 — `formatPageTitle` segments use `.slice(1)` which mutates trailing path segments unexpectedly

[+layout.svelte:147-184](frontend/src/routes/admin/+layout.svelte#L147): for an unlisted segment like `/admin/foo-bar/baz`, `formatPageTitle` returns `'Baz'` ignoring `foo-bar` entirely. For `/admin/members/123` it returns `'123'` — capitalised — so the browser tab reads "123 · Admin · Revolution Trading Pros". Use the parent segment when the leaf is numeric or absent.

### P2-7 — `AdminSidebar.svelte` hardcodes RTP gold (`#e6b800`) in 5 places to "ensure reliability"

[AdminSidebar.svelte:344](frontend/src/lib/components/layout/AdminSidebar.svelte#L344), [:356](frontend/src/lib/components/layout/AdminSidebar.svelte#L356), [:362](frontend/src/lib/components/layout/AdminSidebar.svelte#L362), [:367](frontend/src/lib/components/layout/AdminSidebar.svelte#L367), [:374](frontend/src/lib/components/layout/AdminSidebar.svelte#L374). The comment "RTP Gold - hardcoded for reliability" suggests prior debugging trauma but the variable `--admin-accent-primary` exists and is gold. Replace with the variable so future re-themes don't miss this file.

### P2-8 — `ConnectionGate.svelte` `feature` prop typed as `string` but `FEATURE_SERVICES` is keyed

[ConnectionGate.svelte:35](frontend/src/lib/components/admin/ConnectionGate.svelte#L35) defines `feature?: string;` then matches against a `switch` of literal strings. Typo in any consumer is a silent fall-through to `default` returning `false`. Type as `keyof typeof FEATURE_SERVICES | 'analytics' | 'seo' | …`.

### P2-9 — `ServiceConnectionStatus.svelte` fires `onConnected` callback inside an effect with no de-bounce / one-shot guard

[ServiceConnectionStatus.svelte:251-255](frontend/src/lib/components/admin/ServiceConnectionStatus.svelte#L251):

```ts
$effect(() => {
    if (isConnected && onConnected) {
        onConnected();
    }
});
```

`isConnected` is derived from `connections.isConnected(serviceKey)` — any change to the connections store (e.g. another service connecting) re-runs the effect; if `isConnected` was true on both sides of the change, `onConnected` fires twice. Add a `let calledOnce = false` flag or move the call into the explicit "service just connected" event in the store.

### P2-10 — `BulkUploadQueue.svelte` `$effect(() => () => stopPolling())` is a confusing way to register an unmount hook

[BulkUploadQueue.svelte:251-255](frontend/src/lib/components/admin/BulkUploadQueue.svelte#L251) is `onMount(() => stopPolling)` written as `$effect`. Use `onDestroy` from svelte for clarity.

---

## Low / nits (P3)

### P3-1 — `+error.svelte` uses 80% of its visual real-estate on a debug panel only shown in dev

[+error.svelte:157-181](frontend/src/routes/admin/+error.svelte#L157) — fine in dev, but the layout was clearly designed around the debug panel; in prod the card looks awkwardly tall.

### P3-2 — `+page.svelte:386` debug `console.debug` only logs `activeConnectionCount` (1 number)

[+page.svelte:386](frontend/src/routes/admin/+page.svelte#L386). Low value, gated on `dev` so harmless, but worth removing.

### P3-3 — `AdminToolbar.svelte:541-551` writes a `gtag` analytics call inline; if `gtag` isn't loaded, swallows error silently

[AdminToolbar.svelte:541](frontend/src/lib/components/AdminToolbar.svelte#L541). Low severity, but the empty try/catch means analytics outages are invisible.

### P3-4 — `ActionsDropdown.svelte` hardcodes background `#1e293b` instead of `var(--admin-popover-bg)`

[ActionsDropdown.svelte:166](frontend/src/lib/components/admin/ActionsDropdown.svelte#L166). Out-of-theme tone.

### P3-5 — `CourseDetailDrawer.svelte` hits `/api/admin/courses/${id}/analytics` with raw `fetch`, no auth header

[CourseDetailDrawer.svelte:146](frontend/src/lib/components/admin/CourseDetailDrawer.svelte#L146). The other places use `adminCoursesApi` or `adminFetch` which set headers; this one path skips them. Fine for cookie-based auth but inconsistent.

### P3-6 — `dashboard/+page.svelte` uses raw emoji in section titles (🔍, 🖥️, 📊)

[dashboard/+page.svelte:368](frontend/src/routes/admin/dashboard/+page.svelte#L368), [:464](frontend/src/routes/admin/dashboard/+page.svelte#L464), [:543](frontend/src/routes/admin/dashboard/+page.svelte#L543). Repo recently migrated other places to Tabler icons; this file got the `<svg>` purge for `IconServer` etc. but kept emoji headers. Pick one.

### P3-7 — `Sidebar.svelte` (admin variant) `handleLogout` calls `authStore.logout()` then `goto('/login')` — bypasses the API logout endpoint

[admin/Sidebar.svelte:64-67](frontend/src/lib/components/admin/Sidebar.svelte#L64). The canonical `AdminSidebar.svelte` calls `await logout()` from `$lib/api/auth` ([AdminSidebar.svelte:155](frontend/src/lib/components/layout/AdminSidebar.svelte#L155)), which hits the server. The orphaned admin/Sidebar leaves the access-token cookie on the server. Symptom of P1-2.

### P3-8 — Quick Actions tile destinations alias missing routes

[+page.svelte:885-903](frontend/src/routes/admin/+page.svelte#L885) — the "Filters" tile points at `/admin/categories`, "Tags" at `/admin/categories`, "Links" at `/admin/seo`, "Broadcast" at `/admin/email`, "Global" at `/admin/settings`. The comment at [:886-889](frontend/src/routes/admin/+page.svelte#L886) acknowledges this and points to `docs/audits/ADMIN_QUICK_ACTIONS_BACKLOG.md`. Acceptable interim state, but they're confusing to admins who click "Filters" expecting a filters management page and land in Categories.

### P3-9 — `IconBrowser` used as the "Tablet" icon

[+page.svelte:776](frontend/src/routes/admin/+page.svelte#L776). Should be `IconDeviceTablet` (or similar). Cosmetic.

### P3-10 — `AdminToolbar.svelte:325` file-download trick uses `link.click()` without DOM cleanup

[AdminToolbar.svelte:323-327](frontend/src/lib/components/AdminToolbar.svelte#L323). `document.createElement('a')` is never appended/removed — harmless in modern browsers but historically flaky on Firefox. Append-then-remove if anyone reports issues.

### P3-11 — `dashboard/+page.svelte:255` `<a>` styled as button has no `role="button"` and no keyboard semantics

[dashboard/+page.svelte:255](frontend/src/routes/admin/dashboard/+page.svelte#L255). It's an actual link to `/admin/settings` so this is fine — but the visual treatment is button-like which can confuse screen-reader users about whether activation triggers navigation. Add an `aria-label` or rephrase.

### P3-12 — `MediaUploadHub.svelte` declares `tweened`, `slide`, `fly` etc. — most are unused

Quick scan suggests imports outpace usage. Typecheck will warn. Low priority.

---

## Notable patterns worth keeping

- The **PRINCIPAL-2026-04-26** / **FIX-2026-04-26** comment markers throughout `+layout.svelte` and `AdminSidebar.svelte` are excellent — they tell the next reader *why* the code uses `onMount` instead of `$effect`, with citations. This pattern should be extended to the migrations that still need to happen (AdminToolbar, dashboard/+page).
- [AdminSidebar.svelte:117-126](frontend/src/lib/components/layout/AdminSidebar.svelte#L117) — `bestActiveHref` longest-prefix selection is a clean, elegant solution to the nested-route active-state problem and beats the typical "if pathname.startsWith(href) and href !== '/'" hack.
- [+page.svelte:78-94](frontend/src/routes/admin/+page.svelte#L78) — `AbortController` + per-request `setTimeout` for the dashboard fetch is the right idea. (Just re-use the pattern at the *changePeriod* level too — see P1-5.)
- [+layout.svelte:557-617](frontend/src/routes/admin/+layout.svelte#L557) — the three-tier responsive layout (full sidebar / compact rail / drawer) in pure CSS with `--admin-sidebar-width` driving the offset is well-engineered and matches the Linear / Vercel pattern the codebase aims for.
- The split of `connections.svelte` getters (`getIsAnalyticsConnected()`, `getIsSeoConnected()`, …) used by [ConnectionGate.svelte:60-90](frontend/src/lib/components/admin/ConnectionGate.svelte#L60) is a clean abstraction worth preserving.
- [ConfirmationModal.svelte:50-79](frontend/src/lib/components/admin/ConfirmationModal.svelte#L50) — `variantConfig` table for danger/warning/info/success is a tidy way to keep four icon/color combos consistent across the app.

---

## Summary

- **5 P0** — both blocking bugs (orphan dashboard, missing role guard, broken error layout, cross-origin uploads, init-as-effect) and serious correctness issues that need fixes before this admin shell is recommended for production scaling.
- **10 P1** — Svelte 5 migration debt concentrated in `AdminToolbar.svelte`, plus an orphan duplicate sidebar, a duplicate-auth-header inconsistency, races, dead branches, and type-safety gaps.
- **10 P2** — repeated init-as-effect across 8 admin components, "reset on isOpen" pattern that loses unsaved drafts, ghost form fields that aren't persisted, and `$state` rune coverage gaps in the dashboard's metric objects.
- **12 P3** — cosmetic, theming, accessibility, and dead-code nits.

Highest-leverage fix: do the `$effect → onMount` sweep on `dashboard/+page.svelte` and the rune-getter conversion on `AdminToolbar.svelte`. Both are mechanical and remove an entire category of `effect_update_depth_exceeded` risk while shipping <50 lines of diff.

Second-highest: add an `isAdmin(user.current)` guard to `+layout.svelte`'s onMount and let the `+error.svelte` 403 branch handle non-admins. Defense in depth is cheap here.
