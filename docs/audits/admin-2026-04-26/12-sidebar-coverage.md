# 12 — Sidebar Coverage + Dead Routes

**Date:** 2026-04-26
**Scope:** `/admin/*` reachability — sidebar ↔ filesystem
**Sidebar source of truth:** [`AdminSidebar.svelte`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) `menuSections` (lines 49–105)

---

## 1. Sidebar links coverage

The sidebar declares **27 unique leaf hrefs** across 6 sections. Every one resolves to a `+page.svelte`. Zero dead links.

### Working

All 27 sidebar `href`s resolve to a real `+page.svelte` and the file is non-empty / non-trivial:

| Section | Label | href | Page |
|---|---|---|---|
| (top) | Overview | `/admin` | [`admin/+page.svelte`](../../../frontend/src/routes/admin/+page.svelte) (60 KB) |
| Members | All Members | `/admin/members` | [`members/+page.svelte`](../../../frontend/src/routes/admin/members/+page.svelte) (57 KB) |
| Members | Segments | `/admin/members/segments` | [`members/segments/+page.svelte`](../../../frontend/src/routes/admin/members/segments/+page.svelte) (45 KB) |
| Members | Subscriptions | `/admin/subscriptions` | [`subscriptions/+page.svelte`](../../../frontend/src/routes/admin/subscriptions/+page.svelte) (29 KB) |
| Members | Products | `/admin/products` | [`products/+page.svelte`](../../../frontend/src/routes/admin/products/+page.svelte) (18 KB) |
| Members | Coupons | `/admin/coupons` | [`coupons/+page.svelte`](../../../frontend/src/routes/admin/coupons/+page.svelte) (23 KB) |
| Content | Blog Posts | `/admin/blog` | [`blog/+page.svelte`](../../../frontend/src/routes/admin/blog/+page.svelte) (75 KB) |
| Content | Courses | `/admin/courses` | [`courses/+page.svelte`](../../../frontend/src/routes/admin/courses/+page.svelte) (28 KB) |
| Content | Indicators | `/admin/indicators` | [`indicators/+page.svelte`](../../../frontend/src/routes/admin/indicators/+page.svelte) (18 KB) |
| Content | Trading Rooms | `/admin/trading-rooms` | [`trading-rooms/+page.svelte`](../../../frontend/src/routes/admin/trading-rooms/+page.svelte) (8 KB) |
| Content | Resources | `/admin/resources` | [`resources/+page.svelte`](../../../frontend/src/routes/admin/resources/+page.svelte) (61 KB) |
| Content | Categories | `/admin/blog/categories` | [`blog/categories/+page.svelte`](../../../frontend/src/routes/admin/blog/categories/+page.svelte) (33 KB) |
| Content | Media Library | `/admin/media` | [`media/+page.svelte`](../../../frontend/src/routes/admin/media/+page.svelte) (83 KB) |
| Content | Videos | `/admin/videos` | [`videos/+page.svelte`](../../../frontend/src/routes/admin/videos/+page.svelte) (93 KB) |
| Content | Popups | `/admin/popups` | [`popups/+page.svelte`](../../../frontend/src/routes/admin/popups/+page.svelte) (12 KB) |
| Content | Forms | `/admin/forms` | [`forms/+page.svelte`](../../../frontend/src/routes/admin/forms/+page.svelte) (2 KB — small but valid; renders `<FormList>`) |
| Marketing | Campaigns | `/admin/email/campaigns` | [`email/campaigns/+page.svelte`](../../../frontend/src/routes/admin/email/campaigns/+page.svelte) (37 KB) |
| Marketing | Email Templates | `/admin/email/templates` | [`email/templates/+page.svelte`](../../../frontend/src/routes/admin/email/templates/+page.svelte) (12 KB) |
| Marketing | Email Settings | `/admin/email/smtp` | [`email/smtp/+page.svelte`](../../../frontend/src/routes/admin/email/smtp/+page.svelte) (8 KB) |
| Marketing | SEO | `/admin/seo` | [`seo/+page.svelte`](../../../frontend/src/routes/admin/seo/+page.svelte) (30 KB) |
| Analytics | Dashboard | `/admin/analytics` | [`analytics/+page.svelte`](../../../frontend/src/routes/admin/analytics/+page.svelte) (30 KB) |
| Analytics | Behavior | `/admin/behavior` | [`behavior/+page.svelte`](../../../frontend/src/routes/admin/behavior/+page.svelte) (19 KB) |
| Analytics | CRM | `/admin/crm` | [`crm/+page.svelte`](../../../frontend/src/routes/admin/crm/+page.svelte) (30 KB) |
| System | Site Health | `/admin/site-health` | [`site-health/+page.svelte`](../../../frontend/src/routes/admin/site-health/+page.svelte) (56 KB) |
| System | Connections | `/admin/connections` | [`connections/+page.svelte`](../../../frontend/src/routes/admin/connections/+page.svelte) (45 KB) |
| System | Admin Users | `/admin/users` | [`users/+page.svelte`](../../../frontend/src/routes/admin/users/+page.svelte) (11 KB) |
| System | Settings | `/admin/settings` | [`settings/+page.svelte`](../../../frontend/src/routes/admin/settings/+page.svelte) (56 KB) |

### Dead

**None.** All 27 hrefs resolve.

### Erroring

**None observed at static-inspection level.** The smallest page (`/admin/forms`, 2 KB) is intentionally a lightweight shell that delegates to `<FormList>` from `$lib/components/forms/FormList.svelte`. The other "small" pages (smtp 8 KB, trading-rooms 8 KB, users 11 KB, popups 12 KB, templates 12 KB) all parse and have a default export.

---

## 2. Orphaned admin pages (reachable code, no sidebar entry)

There are **138 `+page.svelte` files** under `frontend/src/routes/admin/`. Subtracting the 27 sidebar leaves and ~85 detail-page descendants of those leaves leaves a clear orphan set.

### Genuinely orphaned (no sidebar ancestor — these ship but are unreachable from the nav)

| Path | Notes |
|---|---|
| [`/admin/dashboard`](../../../frontend/src/routes/admin/dashboard/+page.svelte) | Duplicate of `/admin` (Overview). Likely dead — nav points all `goto('/admin')` at the parent. |
| [`/admin/boards`](../../../frontend/src/routes/admin/boards/+page.svelte) + 6 children (`/[id]`, `/import`, `/reports`, `/settings`, `/templates`, `/time-tracking`) | Whole subsection unreachable from sidebar. Looks like a project-mgmt feature wired into the codebase but never linked. |
| [`/admin/cart/abandoned`](../../../frontend/src/routes/admin/cart/abandoned/+page.svelte) | CRM has `/admin/crm/abandoned-carts` (also orphaned at this level — only reachable as a CRM descendant). The `/admin/cart/abandoned` path is a separate, isolated tool. |
| [`/admin/categories`](../../../frontend/src/routes/admin/categories/+page.svelte) | Distinct from `/admin/blog/categories` (the sidebar entry). The sidebar's "Categories" label is wired to the blog-scoped page; this top-level `/admin/categories` is unreachable. |
| [`/admin/cms/datasources`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte) | No `/admin/cms` sidebar group; this lives under no parent. |
| [`/admin/consent`](../../../frontend/src/routes/admin/consent/+page.svelte) + 2 children (`/settings`, `/templates`) | Cookie-consent admin tools — no sidebar entry. |
| [`/admin/contacts`](../../../frontend/src/routes/admin/contacts/+page.svelte) + `/admin/contacts/new` | Sidebar's CRM points at `/admin/crm` (which has its own `crm/contacts/[id]` subtree). The top-level `/admin/contacts` is parallel and unlinked. |
| [`/admin/memberships`](../../../frontend/src/routes/admin/memberships/+page.svelte) + `/admin/memberships/create` | Distinct from `/admin/subscriptions` and `/admin/members`. Unreachable. |
| [`/admin/orders`](../../../frontend/src/routes/admin/orders/+page.svelte) | No sidebar entry. |
| [`/admin/performance`](../../../frontend/src/routes/admin/performance/+page.svelte) | No sidebar entry — likely a Lighthouse / Core Web Vitals dashboard. |
| [`/admin/schedules`](../../../frontend/src/routes/admin/schedules/+page.svelte) | No sidebar entry. |
| [`/admin/watchlist`](../../../frontend/src/routes/admin/watchlist/+page.svelte) + 2 children (`/[slug]/edit`, `/create`) | No sidebar entry. |
| [`/admin/email/settings`](../../../frontend/src/routes/admin/email/settings/+page.svelte) | Sibling of `/admin/email/smtp` (which IS in the sidebar as "Email Settings"). Likely dead duplicate or a finer-grained settings page. **bestActiveHref note:** because the sidebar contains `/admin/email/campaigns` / `/admin/email/templates` / `/admin/email/smtp` but no plain `/admin/email`, navigation here highlights *nothing* — neither path is a prefix-match. |
| [`/admin/email/subscribers`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte) | Same as above — orphaned at the `/admin/email/subscribers` path because no sidebar item is a prefix-match. Likely a subscribers/list page that should be wired into the Marketing section. |

### Intentionally unrouted (detail/edit/create pages reachable via in-sidebar list)

These are NOT orphans — they're detail/edit pages reachable through the list page that IS in the sidebar (longest-prefix match still highlights the parent):

- All `/admin/members/{[id], analytics, churned, past, service/[id], subscriptions}` — descendants of in-sidebar `/admin/members`.
- `/admin/blog/{create, edit/[id]}` — descendants of `/admin/blog`.
- `/admin/coupons/{create, edit/[id]}`.
- `/admin/courses/{[id], [id]/lessons/[lessonId], create}`.
- `/admin/products/{[id]/edit, create}`.
- `/admin/popups/{[id]/analytics, [id]/edit, new}`.
- `/admin/forms/{[id]/analytics, [id]/edit, [id]/submissions, create, entries}`.
- `/admin/indicators/{[id], create}`.
- `/admin/users/{create, edit/[id]}`.
- `/admin/videos`, `/admin/media/analytics`.
- `/admin/trading-rooms/[slug]`.
- `/admin/subscriptions/{invoice-settings, plans}`.
- All `/admin/seo/*` (16 sub-pages — descendants of in-sidebar `/admin/seo`).
- `/admin/analytics/{attribution, cohorts, events, funnels, goals, heatmaps, recordings, reports, segments}` — descendants of `/admin/analytics`.
- `/admin/email/campaigns/[id]/report`, `/admin/email/templates/{edit/[id], new, preview/[id]}` — descendants of in-sidebar email pages.
- All `/admin/crm/*` (~25 sub-pages — descendants of `/admin/crm`).

**Orphan count: 14 distinct orphan trees comprising ~22 unreachable `+page.svelte` files.**

---

## 3. bestActiveHref highlighting walkthrough

The algorithm — [`AdminSidebar.svelte:117–126`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — flattens every sidebar href, picks any href that is `currentPath === href || currentPath.startsWith(href + '/')`, and keeps the longest match:

```ts
let bestActiveHref = $derived.by(() => {
    const allHrefs = menuSections.flatMap((s) => s.items.map((i) => i.href));
    let best = '';
    for (const href of allHrefs) {
        if (currentPath === href || currentPath.startsWith(href + '/')) {
            if (href.length > best.length) best = href;
        }
    }
    return best;
});
```

| Test path | Matches found | Longest winner | Highlighted item | Correct? |
|---|---|---|---|---|
| `/admin` | `/admin` | `/admin` | **Overview** | yes |
| `/admin/members/segments` | `/admin`, `/admin/members`, `/admin/members/segments` | `/admin/members/segments` | **Segments** | yes |
| `/admin/members/segments/foo` | `/admin`, `/admin/members`, `/admin/members/segments` (via `startsWith('/admin/members/segments/')`) | `/admin/members/segments` | **Segments** | yes — sub-detail of a segment correctly stays on Segments |
| `/admin/blog/categories` | `/admin`, `/admin/blog`, `/admin/blog/categories` | `/admin/blog/categories` | **Categories** | yes |
| `/admin/blog/edit/123` | `/admin`, `/admin/blog` | `/admin/blog` | **Blog Posts** | yes — edit page is a child of blog, no edit href in sidebar |
| `/admin/email/campaigns` | `/admin`, `/admin/email/campaigns` | `/admin/email/campaigns` | **Campaigns** | yes |
| `/admin/email/templates` | `/admin`, `/admin/email/templates` | `/admin/email/templates` | **Email Templates** | yes |
| `/admin/email/smtp` | `/admin`, `/admin/email/smtp` | `/admin/email/smtp` | **Email Settings** | yes |
| `/admin/crm/contacts/456` | `/admin`, `/admin/crm` | `/admin/crm` | **CRM** | yes — CRM contact-detail page falls through to top-level CRM |

**The algorithm is correct for every test.** The only edge case worth flagging is that the empty path `''` is the seed (`let best = ''`) — if `currentPath === '/foo'` matches no href, no item gets `class="active"` (sidebar shows no active state). For pages like `/admin/email/settings` and `/admin/email/subscribers` (orphans, see §2), this is the actual behavior: nothing is highlighted because no sidebar `href` is a prefix of those paths.

There is also a minor footgun: `/admin` is a prefix of every other admin path, so `currentPath.startsWith('/admin/')` is true universally. The longest-match logic handles this — `/admin` only wins when nothing more specific matches — but a path like `/admin/foo` (where `foo` is orphaned and no other sidebar prefix applies) will incorrectly highlight **Overview**. This is technically wrong UX for orphan pages, but harmless since orphans aren't reachable from the nav anyway.

---

## 4. Icon import sanity

Sidebar icon imports — [`AdminSidebar.svelte:14–41`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — list **26 icons**:

```ts
import {
    IconDashboard, IconReceipt, IconTicket, IconUsers, IconUserCircle,
    IconSettings, IconLogout, IconX, IconForms, IconSeo, IconNews,
    IconMail, IconChartBar, IconSend, IconFilter, IconEye, IconPhoto,
    IconTag, IconVideo, IconShoppingCart, IconBellRinging, IconHeartbeat,
    IconPlugConnected, IconBook, IconFileText, IconTrendingUp
} from '$lib/icons';
```

Cross-checked against [`$lib/icons/index.ts`](../../../frontend/src/lib/icons/index.ts) `export {…}` block (lines 497–943):

| Icon | Exported? | Source line in `index.ts` |
|---|---|---|
| `IconDashboard` | yes | 636 |
| `IconReceipt` | yes | 808 |
| `IconTicket` | yes | 871 |
| `IconUsers` | yes | 896 |
| `IconUserCircle` | yes | 891 |
| `IconSettings` | yes | 826 |
| `IconLogout` | yes | 738 |
| `IconX` | yes | 912 |
| `IconForms` | yes | 683 |
| `IconSeo` | yes | 824 |
| `IconNews` | yes | 768 |
| `IconMail` | yes | 740 |
| `IconChartBar` | yes | 591 |
| `IconSend` | yes | 823 |
| `IconFilter` | yes | 673 |
| `IconEye` | yes | 657 |
| `IconPhoto` | yes | 779 |
| `IconTag` | yes | 861 |
| `IconVideo` | yes | 901 |
| `IconShoppingCart` | yes | 832 |
| `IconBellRinging` | yes | 538 |
| `IconHeartbeat` | yes | 697 |
| `IconPlugConnected` | yes | 792 |
| `IconBook` | yes | 542 |
| `IconFileText` | yes | 669 |
| `IconTrendingUp` | yes | 882 |

**All 26 icons resolve. No missing exports.**

---

## 5. Sign-out flow trace

The sidebar's [`handleSignOut`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) (lines 153–162):

```ts
async function handleSignOut() {
    try {
        await logout();
    } catch (err) {
        console.warn('[AdminSidebar] logout API failed; signing out client-side anyway', err);
    }
    closeSidebar();
    await goto('/');
}
```

Tracing `logout()` → [`auth.ts:1402`](../../../frontend/src/lib/api/auth.ts) → `authService.logout()` ([`auth.ts:713–739`](../../../frontend/src/lib/api/auth.ts)):

```ts
async logout(): Promise<void> {
    try {
        await this.apiRequest<MessageResponse>(API_ENDPOINTS.auth.logout, { method: 'POST' });
        // ...trackEvent...
    } catch (error) {
        console.error('[AuthService] Logout API call failed:', error);
    } finally {
        if (browser) {
            try {
                await fetch('/api/auth/set-session', { method: 'DELETE' });
            } catch (cookieError) {
                console.warn('[AuthService] Failed to clear session cookies:', cookieError);
            }
        }
        this.clearAuth();
    }
}
```

`API_ENDPOINTS.auth.logout` = `'/api/logout'` ([`config.ts:48`](../../../frontend/src/lib/api/config.ts)) → maps to [`api/logout/+server.ts`](../../../frontend/src/routes/api/logout/+server.ts):

```ts
export const POST: RequestHandler = async ({ request, cookies }) => {
    const token = cookies.get('rtp_access_token');
    // ...
    try {
        await fetch(`${API_URL}/logout`, { method: 'POST', headers: { ...token && { Authorization: `Bearer ${token}` } } });
    } catch (error) {
        console.error('[Logout Proxy] Backend logout failed:', error);
    }
    // ALWAYS clear cookies regardless of backend response
    cookies.delete('rtp_access_token', { path: '/' });
    cookies.delete('rtp_refresh_token', { path: '/' });
    return json({ success: true, message: 'Logged out successfully' });
};
```

### Verification matrix

| Claim | Status | Evidence |
|---|---|---|
| Clears `rtp_access_token` cookie | **YES** | [`api/logout/+server.ts:48`](../../../frontend/src/routes/api/logout/+server.ts) — `cookies.delete('rtp_access_token', { path: '/' })`. Also a belt-and-braces second clear via `DELETE /api/auth/set-session` ([`auth.ts:730`](../../../frontend/src/lib/api/auth.ts) → [`set-session/+server.ts:67–72`](../../../frontend/src/routes/api/auth/set-session/+server.ts)) which also clears `rtp_refresh_token`. |
| Calls a backend logout endpoint | **YES** | [`api/logout/+server.ts:33`](../../../frontend/src/routes/api/logout/+server.ts) — `fetch(\`${API_URL}/logout\`, { method: 'POST', ... })` (Rust API). Forwards `Authorization: Bearer <rtp_access_token>` and `X-Session-ID` if present. |
| Handles failures gracefully | **YES (3 layers)** | (1) `api/logout/+server.ts:32–45` wraps the upstream Rust call in `try/catch` and logs. (2) `auth.ts:713–739` wraps `apiRequest` in `try/catch/finally` and ALWAYS runs `clearAuth()` + cookie-DELETE regardless. (3) `AdminSidebar.svelte:154–159` wraps `logout()` itself and falls through to `goto('/')` even if all of the above throw. |

**No auth issues found.** The sign-out flow is correctly defense-in-depth: even if the Rust API is down, the proxy still deletes both cookies; even if the proxy throws, `set-session DELETE` still runs; even if that throws, `goto('/')` still fires.

One minor observation, **not a bug**: the `apiRequest` path inside `auth.ts` will retry up to 3 times on 5xx before propagating the error. In practice that means a flaky logout API can delay the cookie-clear by up to ~7 seconds (`1s + 2s + 4s` exponential backoff). The `handleSignOut` UX would benefit from a parallel local-clear that doesn't wait for the network — but functionally the cookies always get cleared.

---

## 6. Mobile drawer wiring

### Three breakpoint modes (defined in [`AdminSidebar.svelte:484–647`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte))

| Width | Mode | Behavior |
|---|---|---|
| `>= 1280px` | Full sidebar | 240 px wide, labels visible, persistent |
| `1024–1279px` | Compact rail | 72 px wide, icons only, CSS tooltips on hover (lines 503–620) |
| `< 1024px` | Off-canvas drawer | `transform: translateX(-100%)`; `.open` slides it back to `translateX(0)`; overlay button shown (lines 631–647) |

### Toggle wiring — [`+layout.svelte`](../../../frontend/src/routes/admin/+layout.svelte)

| Concern | Wired? | Where |
|---|---|---|
| Hamburger button exists | **YES** | [`+layout.svelte:201–206`](../../../frontend/src/routes/admin/+layout.svelte) — `<button class="mobile-menu-btn" onclick={toggleSidebar} aria-label="Toggle sidebar">` |
| Hamburger calls open toggle | **YES** | `toggleSidebar()` at line 139–141 flips `isSidebarOpen = !isSidebarOpen` |
| Hamburger only shows on mobile | **YES** | `+layout.svelte:351` — `.mobile-menu-btn { display: none }`; line 562–564 — `@media (max-width: calc(var(--breakpoint-lg) - 1px)) { .mobile-menu-btn { display: block } }` |
| `isOpen` prop passed to `<AdminSidebar>` | **YES** | [`+layout.svelte:193`](../../../frontend/src/routes/admin/+layout.svelte) — `<AdminSidebar isOpen={isSidebarOpen} onclose={closeSidebar} />` |
| Sidebar uses `isOpen` to slide in | **YES** | [`AdminSidebar.svelte:165`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — `class:open={props.isOpen}`; CSS `.admin-sidebar.open { transform: translateX(0) }` (line 636) |
| Close button (the X) inside sidebar | **YES** | [`AdminSidebar.svelte:171–173`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — `<button class="close-btn" onclick={closeSidebar}>`. Also only shows below 1024 px (`.close-btn { display: none }` line 268, `display: block` line 642). |
| `onclose` callback wires back to layout | **YES** | `props.onclose?.()` at AdminSidebar:108 → `closeSidebar()` in layout:143–145 → `isSidebarOpen = false` |
| Overlay closes on click | **YES** | [`AdminSidebar.svelte:219–221`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — `{#if props.isOpen} <button class="sidebar-overlay" onclick={closeSidebar} aria-label="Close sidebar"></button> {/if}`. `display: block` only at `<1024px` (line 644–646). |
| Nav items also close drawer on click | **YES** | [`AdminSidebar.svelte:191`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte) — `<a href={item.href} ... onclick={closeSidebar}>` so tapping a link closes the drawer. |

**Drawer is fully wired and works correctly across all three modes.** No bugs.

One UX nit: there is no `Escape` key handler on the drawer. The keyboard-shortcut bus in `+layout.svelte:127–131` does have an `escape` action — but it closes the command palette / notification center / shortcuts help / connection-health panel, NOT the sidebar drawer. A user on mobile-with-keyboard cannot dismiss the drawer with Esc. Low priority.

---

## Summary

| Metric | Value |
|---|---|
| Sidebar leaf hrefs | 27 |
| Dead sidebar links | **0** |
| Erroring (empty / broken) target pages | **0** |
| Total `+page.svelte` under `/admin/` | 138 |
| Genuinely orphaned pages (no sidebar prefix) | **~22 files across 14 trees** |
| Detail/edit descendants (correctly reachable via parent) | ~89 |
| Icons imported by sidebar | 26 |
| Missing icon exports | **0** |
| Sign-out cookie-clear correctness | **3-layer defense, all verified** |
| Mobile drawer toggle wiring | **Complete (hamburger, X-close, overlay, link-tap, prop callback)** |

**Top orphans worth a product call (kill or wire):**

1. `/admin/dashboard` — duplicate of `/admin`. Delete or redirect.
2. `/admin/boards/*` (7 files) — entire project-mgmt subsection unrouted.
3. `/admin/email/{settings, subscribers}` — siblings of in-sidebar email pages, no nav entry.
4. `/admin/contacts`, `/admin/categories`, `/admin/memberships` — top-level pages that conflict semantically with the in-sidebar CRM/blog/subscriptions entries; either retire or rename.
5. `/admin/{cart/abandoned, cms/datasources, consent/*, orders, performance, schedules, watchlist/*}` — each is a stand-alone tool with no nav.

**No auth issues found in the sign-out flow.** Cookies (`rtp_access_token` + `rtp_refresh_token`) are deleted in two redundant proxies, the backend revocation is fire-and-forget, and the UI navigates home regardless of network outcome.
