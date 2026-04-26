# Admin Sidebar Forensic

Audit date: 2026-04-26. Read-only.

## Active component

- **File:** `frontend/src/lib/components/layout/AdminSidebar.svelte`
- **Import site:** `frontend/src/routes/admin/+layout.svelte:26`
  ```ts
  import { AdminSidebar } from '$lib/components/layout';
  ```
  Re-exported by barrel `frontend/src/lib/components/layout/index.ts:11`.
- **Render site:** `frontend/src/routes/admin/+layout.svelte:186`
  ```svelte
  <AdminSidebar isOpen={isSidebarOpen} onclose={closeSidebar} />
  ```
- **Props (declared at `AdminSidebar.svelte:42-47`):**
  | Prop      | Type           | Default     | Source                           |
  |-----------|----------------|-------------|----------------------------------|
  | `isOpen`  | `boolean?`     | `undefined` | `+layout.svelte:47` `$state(false)` |
  | `onclose` | `() => void?`  | `undefined` | `+layout.svelte:136` `closeSidebar` |

  Note: props are read via `props.isOpen` / `props.onclose` (whole-object pattern). They are NOT destructured into individual `$bindable` runes — see "Reactive declaration audit" below.

### Sibling sidebars (NOT active for `/admin`)

- `frontend/src/lib/components/admin/Sidebar.svelte` — **dead code**. No grep hits anywhere in `frontend/src` (verified). Has its own (much smaller) navigation tree (`Dashboard`, `Forms`, `SEO`) and uses the legacy `$authStore` autosubscribe at `Sidebar.svelte:134,138,139`. Recommend deletion or archive.
- `frontend/src/lib/components/dashboard/DashboardSidebar.svelte` — used only by `/dashboard/*` routes, not `/admin`.

## Menu link inventory

Source: `AdminSidebar.svelte:49-105` (`menuSections` array). HTTP status from `curl -s -o /dev/null -w '%{http_code}'` against `localhost:5173` (unauth — every protected route is 303-redirected to `/login?redirect=…` by `hooks.server.ts:32`; following redirects yields 200). Filesystem column verifies each `+page.svelte` exists under `frontend/src/routes/admin/`.

| Section    | Label             | href                          | Route exists? | HTTP (303→200) |
|------------|-------------------|-------------------------------|---------------|----------------|
| (top)      | Overview          | `/admin`                      | yes (`+page.svelte`) | 303→200 |
| Members    | All Members       | `/admin/members`              | yes | 303→200 |
| Members    | Segments          | `/admin/members/segments`     | yes | 303→200 |
| Members    | Subscriptions     | `/admin/subscriptions`        | yes | 303→200 |
| Members    | Products          | `/admin/products`             | yes | 303→200 |
| Members    | Coupons           | `/admin/coupons`              | yes | 303→200 |
| Content    | Blog Posts        | `/admin/blog`                 | yes | 303→200 |
| Content    | Courses           | `/admin/courses`              | yes | 303→200 |
| Content    | Indicators        | `/admin/indicators`           | yes | 303→200 |
| Content    | Trading Rooms     | `/admin/trading-rooms`        | yes | 303→200 |
| Content    | Resources         | `/admin/resources`            | yes | 303→200 |
| Content    | Categories        | `/admin/blog/categories`      | yes | 303→200 |
| Content    | Media Library     | `/admin/media`                | yes | 303→200 |
| Content    | Videos            | `/admin/videos`               | yes | 303→200 |
| Content    | Popups            | `/admin/popups`               | yes | 303→200 |
| Content    | Forms             | `/admin/forms`                | yes | 303→200 |
| Marketing  | Campaigns         | `/admin/email/campaigns`      | yes | 303→200 |
| Marketing  | Email Templates   | `/admin/email/templates`      | yes | 303→200 |
| Marketing  | Email Settings    | `/admin/email/smtp`           | yes | 303→200 |
| Marketing  | SEO               | `/admin/seo`                  | yes | 303→200 |
| Analytics  | Dashboard         | `/admin/analytics`            | yes | 303→200 |
| Analytics  | Behavior          | `/admin/behavior`             | yes | 303→200 |
| Analytics  | CRM               | `/admin/crm`                  | yes | 303→200 |
| System     | Site Health       | `/admin/site-health`          | yes | 303→200 |
| System     | Connections       | `/admin/connections`          | yes | 303→200 |
| System     | Admin Users       | `/admin/users`                | yes | 303→200 |
| System     | Settings          | `/admin/settings`             | yes | 303→200 |

**Result: 27/27 hrefs resolve to a real route.** No 404s, no stale targets.

### Sub-menu / collapse / active-state logic

- **No nested sub-menus.** Sections are flat groups headed by `<div class="nav-section-title">`. The first group has `title: null` so its header is omitted (`AdminSidebar.svelte:171`).
- **Active state** (`AdminSidebar.svelte:117-126`): `bestActiveHref` is the longest matching href against `page.url.pathname` (exact match OR `pathname.startsWith(href + '/')`). This correctly disambiguates `/admin/members/segments` (picks the longer `/segments` href, not the prefix `/members`). `aria-current="page"` is applied at `:181`.
- **Collapsed state** is purely CSS-driven — no `$state` for collapse:
  - `>=1280px` full sidebar (240px, labels visible)
  - `1024–1279px` compact rail (72px, icons only, CSS tooltips via `data-tooltip` attr — `:530-560`)
  - `<1024px` off-canvas drawer toggled via `props.isOpen` adding `.open` class (`:157`)
- **Mobile overlay**: rendered conditionally (`{#if props.isOpen}` at `:211`); clicking it calls `closeSidebar()`.

## Reactive declaration audit

| Location | Declaration | Classification | Notes |
|----------|------------|----------------|-------|
| `AdminSidebar.svelte:47` | `let props: Props = $props();` | READ-ONLY | Whole-object props (no destructuring). Not `$bindable`; `isOpen` mutations from parent re-flow correctly via prop reassignment. |
| `AdminSidebar.svelte:111` | `let currentPath = $derived(page.url.pathname);` | READ-ONLY | Pure derivation from `$app/state` `page` rune. Safe. |
| `AdminSidebar.svelte:117-126` | `let bestActiveHref = $derived.by(() => { … })` | READ-ONLY | Reads `currentPath` + static `menuSections`. No writes. Recomputes O(n) over 27 hrefs per `pathname` change — negligible cost. |
| `AdminSidebar.svelte:129-141` | `let displayRole = $derived.by(() => { const u = $user; … })` | **BORDERLINE — `$store` autosubscribe inside `$derived.by`** | `$user` is the legacy store wrapper exported at `auth.svelte.ts:682-690`. Auto-subscribe inside `$derived.by` synthesizes a `legacy_pre_subscribe` call. Per CLAUDE.md the codebase had to convert several `$effect`s away from `$user` reads to fix `effect_update_depth_exceeded` (`+layout.svelte:53-76`). A `$derived` autosubscribe is NOT in the same severity class as the converted `$effect`s — it's the documented "synchronous-initial-snapshot" pattern (`auth.svelte.ts:243-249`) and only fires reactively on subsequent notifications, which are queueMicrotask-coalesced (`auth.svelte.ts:284-303`). Risk: if `setUser` fires while this derivation is being read inside another effect, the snapshot fan-out could re-enter. Lower-risk alternative: read `authStore.user` (the rune-backed getter at `auth.svelte.ts:309-311`) directly and skip the legacy subscribe layer. |
| `AdminSidebar.svelte:196` | `{$user?.name?.[0]?.toUpperCase() || 'A'}` | BORDERLINE — `$store` autosubscribe in template | Same wrapper, two more reads in the template. Each `$user` reference compiles to a fresh `legacy_pre_subscribe`. Three template reads (`:196,199,200` via `displayRole`) → three subscriptions for the same data. |
| `AdminSidebar.svelte:199` | `{$user?.name \|\| 'Admin'}` | BORDERLINE | Same as above. |
| `AdminSidebar.svelte:107-109` | `function closeSidebar() { props.onclose?.(); }` | READ-ONLY | Pure function. |
| `AdminSidebar.svelte:145-154` | `async function handleSignOut() { await logout(); … await goto('/'); }` | READ+WRITE *external* | Calls `logout()` which mutates `authStore` ($state runes) — those writes are coalesced by `notifySubscribers` microtask. Safe relative to the sidebar's own runes. |

**No `$state`, no `$effect`, no manual `subscribe/unsubscribe`, no `onMount`/`onDestroy` in this file.** Cleanup obligations: none — `$user`/`$isAuthenticated` autosubscribes are managed by the Svelte compiler's generated cleanup.

## Bugs found

### B1 — No role/permission gate on menu items (`AdminSidebar.svelte:49-105`)
**Symptom:** Every authenticated user — including non-admin members — sees all 27 admin links rendered. Click-through hits the route, where access depends entirely on the route page's own data-loading or the API's role check on the data fetch.
**Cause:** The sidebar imports `isSuperadmin` (`AdminSidebar.svelte:13`) but only uses it for the **role label** (`:132`). The `menuSections` array is rendered unconditionally (`:170-189`). There is no `+layout.server.ts` for `/admin` — `frontend/src/routes/admin/+layout.ts:1-4` only sets `ssr = false; prerender = false;`, and `hooks.server.ts:32` guards `/admin` for *authentication only*, not role. The auth guard at `+layout.svelte:71-76` likewise only checks `$isAuthenticated`.
**Impact:** Information disclosure (sitemap of admin features visible to any logged-in user) and worse UX (members get a wall of links that 401/403 on click).

### B2 — Three separate `$user` autosubscribes in one template
**Symptom:** Each `$user` reference inside the component (`AdminSidebar.svelte:130, 196, 199`) compiles to its own `legacy_pre_subscribe` subscription against `authStore`. The component holds 3 active subscriptions where 1 would suffice.
**Cause:** Pattern of using the legacy store wrapper (`auth.svelte.ts:682-690`) instead of the rune-backed getter `authStore.user` (`auth.svelte.ts:309-311`). The wrapper exists for backward compatibility (`auth.svelte.ts:679-681`) but the codebase has been migrating away from it.
**Impact:** Low. `notifySubscribers` is microtask-coalesced (`auth.svelte.ts:284-303`), so a single auth state change still results in one fan-out per subscriber. But this is the pattern the project audit flagged for *effects*; preventatively cleaning up the read sites here removes future cascade risk.

### B3 — `props` whole-object pattern violates project house style
**Symptom:** `let props: Props = $props();` at `AdminSidebar.svelte:47` followed by `props.isOpen` / `props.onclose` reads. CLAUDE.md explicitly calls out the preferred Svelte 5 pattern as `let { foo = $bindable(null) } = $props();`.
**Cause:** Hand-rolled component pre-dating the 41-component migration in commit `05acf3231`.
**Impact:** Style/maintenance only. The whole-object pattern is functionally correct and does NOT trigger `state_referenced_locally` here because `props` itself is not re-assigned. But it's inconsistent with the rest of the codebase.

### B4 — No `aria-expanded` / drawer-state ARIA for mobile drawer
**Symptom:** When the mobile drawer opens (`<aside class:open={props.isOpen}>` at `:157`), no `aria-hidden` is toggled and no `aria-expanded` is exposed on the toggle. The toggle button lives outside this component (`+layout.svelte:192-199`) and similarly lacks `aria-expanded={isSidebarOpen}` / `aria-controls="…"`.
**Cause:** Mobile drawer state is purely visual.
**Impact:** Screen readers can't announce drawer state. Off-screen `<aside>` is still focusable when `transform: translateX(-100%)` is applied (`AdminSidebar.svelte:625`) — focus can leak into the closed drawer via Tab.

### B5 — Notification dot / counter logic
The sidebar itself ships **no** badges or "new"/"beta" indicators on any nav item. Notification badge logic lives in `+layout.svelte:223-235` on the header bell button, sourced from `getUnreadCount()` (`stores/notifications.svelte.ts`). The `<span class="notification-badge">` (`+layout.svelte:233`) is correctly conditional (`{#if unreadCount > 0}`). Nothing in the **sidebar** is stubbed.

### B6 — Sign-out flow handles failure cleanly (no bug, noted for completeness)
`handleSignOut` (`AdminSidebar.svelte:145-154`) calls the API `logout`, swallows any error with a `console.warn` (`:149-151`), then unconditionally `goto('/')`. This is a deliberate pattern, not a bug — even if the API logout 500s, client tokens are cleared by `authStore.logout()` (`auth.svelte.ts:612-663`). However: the redirect target is `/` (homepage), not `/login` like the rest of the codebase (e.g. `auth.svelte.ts:612` defaults to `/login`). Inconsistent UX — clicking "Sign Out" from `/admin` lands on the marketing site, not the login page.

### B7 — Active-state false positive when `currentPath` exactly matches no href
**Symptom:** `bestActiveHref` initial value is `''` (`:120`). If `currentPath` is e.g. `/admin/foo` where `foo` isn't in the sidebar, NO item is highlighted (correct). But for `/admin` itself, the loop's `currentPath === '/admin'` matches the Overview item — so this is fine. However, the `startsWith(href + '/')` check on `:121` for the Overview href `/admin` means **any** `/admin/anything` would match the Overview prefix; only the longest-match logic saves us. This is correct as written but extremely sensitive — adding a future href that is itself a strict prefix of another href without a tighter match could regress silently.

## Sidebar interaction with route transitions

- **Mount lifecycle:** `AdminSidebar` is rendered directly inside `/admin/+layout.svelte` (above `<main>` which renders `props.children()` at `:269`). SvelteKit re-renders only the child page when navigating `/admin/foo → /admin/bar`; the layout (and therefore the sidebar) **stays mounted**.
- **Active-highlight update:** `currentPath` is `$derived(page.url.pathname)` (`:111`); `page` from `$app/state` is reactive. On navigation, `currentPath` changes → `bestActiveHref` recomputes → all 27 `<a>` elements re-evaluate `class:active`. No remount, no animation breakage.
- **`$effect`s in scope:** none in the sidebar itself. The two `onMount` handlers in the layout (`+layout.svelte:71-76, 109-129`) run once at layout mount and not again across child-page navigations — confirmed by the `PRINCIPAL-2026-04-26` and `FIX-2026-04-26` markers explaining the `$effect → onMount` conversion specifically because the prior `$effect`s were trapping in cascade-update.
- **Unexpected effect risk:** zero. The only reactive subscriptions inside the sidebar are the auto-subscribed `$user` reads, which fan out via the auth store's coalesced microtask. Navigation does not trigger an auth state change, so `$user` does not re-emit during route transitions.

## Top 5 sidebar issues

1. **(B1) No role gating on menu items.** Every authenticated user sees the full admin index — both an info-disclosure and UX problem. Highest priority. Fix: filter `menuSections` against `isSuperadmin($user)` / `isAdminUser` / per-item `permission` keys, or render a `+layout.server.ts` guard that redirects non-admins.
2. **(B4) Mobile drawer accessibility.** Missing `aria-expanded`/`aria-controls` on the toggle, no focus trap, no `aria-hidden` on the off-screen `<aside>`. Affects keyboard users on mobile.
3. **(B6) Sign-out lands on `/`, not `/login`.** Inconsistent with `authStore.logout` default. UX confusing — users expect to see the login page after explicit sign-out.
4. **(B2) Three `$user` legacy autosubscribes per render.** Cosmetic but project policy is migrating off this pattern; fix is mechanical (replace with `authStore.user` reads).
5. **(B3) Whole-object `$props()` destructuring.** Inconsistent with the 41-component migration in `05acf3231`. Convert to `let { isOpen = false, onclose } = $props();`.
