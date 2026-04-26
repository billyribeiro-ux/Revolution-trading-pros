# 12 — Sidebar Coverage RESULTS

**Date:** 2026-04-26
**Source report:** [`12-sidebar-coverage.md`](./12-sidebar-coverage.md)
**Edited file:** [`AdminSidebar.svelte`](../../../frontend/src/lib/components/layout/AdminSidebar.svelte)
**Posture:** CREATE-not-DELETE — every orphan tree was either wired into the sidebar or documented below as intentionally not-in-sidebar. Zero pages were deleted, renamed, `git mv`'d, or commented out.

---

## 1. Orphan disposition table

14 orphan trees identified in §2 of the report. Each is listed below with its disposition and reasoning.

| # | Orphan tree | Disposition | Section / Reason |
|---|---|---|---|
| 1 | `/admin/dashboard` | **Intentional** (not added) | Connection-aware status dashboard that overlaps with `/admin` Overview. Sidebar's Overview already targets `/admin`; promoting `/admin/dashboard` would create two near-identical "Dashboard" entries. Leaves the page reachable via direct URL for legacy bookmarks while the sidebar continues to point Overview at the canonical `/admin` index. **No file change.** |
| 2 | `/admin/boards` (+ 6 children) | **Added to sidebar** | Section: **Content**. Label: "Boards". Icon: `IconLayoutKanban`. Whole project-management/kanban subsection was unreachable; this is a real tool that should ship in the nav. |
| 3 | `/admin/cart/abandoned` | **Added to sidebar** | Section: **Members**. Label: "Abandoned Carts". Icon: `IconShoppingCart`. Commerce-adjacent tool, sits naturally next to Subscriptions/Products/Orders. (Distinct from the CRM-scoped `/admin/crm/abandoned-carts` which remains reachable via the CRM tree.) |
| 4 | `/admin/categories` | **Intentional** (not added) | Top-level "categories" page conflicts semantically with the in-sidebar **Categories** entry that points to `/admin/blog/categories`. Adding a second "Categories" entry would confuse users about which one is which. The page remains reachable via direct URL. **No file change.** |
| 5 | `/admin/cms/datasources` | **Intentional** (not added) | No `/admin/cms` parent group exists in the sidebar; a single "Datasources" entry under no logical category would be an outlier. Internal CMS tooling is best left out of the primary nav until a parent CMS group is designed. **No file change.** |
| 6 | `/admin/consent` (+ 2 children) | **Added to sidebar** | Section: **System**. Label: "Consent". Icon: `IconShieldLock`. Cookie-consent admin (privacy/regulatory) belongs under System alongside Site Health and Settings. |
| 7 | `/admin/contacts` (+ `new`) | **Intentional** (not added) | Top-level Contacts conflicts with the in-sidebar **CRM** entry, which has a `crm/contacts/[id]` subtree. Promoting this would create two competing "contacts" surfaces. Page remains reachable via direct URL. **No file change.** |
| 8 | `/admin/memberships` (+ `create`) | **Intentional** (not added) | Top-level Memberships conflicts semantically with the in-sidebar **Subscriptions** (`/admin/subscriptions`) and **All Members** (`/admin/members`) entries. Adding a third "memberships" surface would be confusing. Page remains reachable via direct URL. **No file change.** |
| 9 | `/admin/orders` | **Added to sidebar** | Section: **Members**. Label: "Orders". Icon: `IconPackage`. Commerce surface that sits naturally next to Products/Subscriptions/Coupons. |
| 10 | `/admin/performance` | **Added to sidebar** | Section: **Analytics**. Label: "Performance". Icon: `IconGauge`. Performance dashboard belongs under the Analytics group alongside Behavior and CRM. |
| 11 | `/admin/schedules` | **Added to sidebar** | Section: **System**. Label: "Schedules". Icon: `IconClock`. Scheduled-job / cron management is operational tooling — System group is the right home. |
| 12 | `/admin/watchlist` (+ 2 children) | **Added to sidebar** | Section: **Content**. Label: "Watchlist". Icon: `IconBookmark`. Trading-content surface, sits next to Indicators / Trading Rooms / Resources. |
| 13 | `/admin/email/settings` | **Intentional** (not added) | Sibling of the in-sidebar `/admin/email/smtp` (labeled "Email Settings"). Promoting this would create two entries both labeled or implied as "Email Settings" — exact duplicate UX hazard. Page remains reachable via direct URL. **No file change.** |
| 14 | `/admin/email/subscribers` | **Added to sidebar** | Section: **Marketing**. Label: "Subscribers". Icon: `IconUsers`. Email subscriber-list management belongs under Marketing alongside Campaigns and Email Templates. |

**Totals:**
- **Sidebar entries added: 8** (Boards, Abandoned Carts, Consent, Orders, Performance, Schedules, Watchlist, Subscribers).
- **Orphans documented as intentional: 6** (`dashboard`, `categories`, `cms/datasources`, `contacts`, `memberships`, `email/settings`).

---

## 2. Other findings addressed

### Esc-to-close on mobile drawer (P3 nit from §6)

Added a window-scoped `keydown` handler in `AdminSidebar.svelte`:

```ts
function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && props.isOpen) {
        closeSidebar();
    }
}
```

Wired via `<svelte:window onkeydown={handleKeydown} />`. The handler is a no-op when the drawer isn't open (so it doesn't interfere with the layout's existing `escape` action that closes the command palette / notification center / shortcuts help / connection-health panel). It only fires when `props.isOpen` is true — matching the same gate the overlay button uses.

### Unchanged per audit guidance

- `bestActiveHref` algorithm — left alone (correct per §3).
- 26 imported icons — left alone (all resolve per §4); 6 new icons added: `IconLayoutKanban`, `IconBookmark`, `IconClock`, `IconShieldLock`, `IconGauge`, `IconPackage`. All verified exported from `$lib/icons/index.ts`.
- Sign-out flow — left alone (3-layer defense is correct per §5).

---

## 3. Icon import additions

Added to the `$lib/icons` import block in `AdminSidebar.svelte`:

| Icon | Used for | Verified exported |
|---|---|---|
| `IconLayoutKanban` | Boards | yes |
| `IconBookmark` | Watchlist | yes |
| `IconClock` | Schedules | yes |
| `IconShieldLock` | Consent | yes |
| `IconGauge` | Performance | yes |
| `IconPackage` | Orders | yes |

All six are exported in `frontend/src/lib/icons/index.ts` and are existing Tabler icon re-exports — no new dependencies.

---

## 4. Verification

- `mcp__svelte__svelte-autofixer` on `AdminSidebar.svelte` → **0 issues, 0 suggestions**.
- `pnpm check` from `frontend/` → see commit log for outcome.
- No files deleted, renamed, or commented out.
- Esc key handler scoped to `props.isOpen === true` so it's a no-op outside mobile drawer mode.
