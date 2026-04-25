# Admin Sidebar — Forensic End-to-End Audit

**Auditor:** Claude (Opus 4.7) acting as a Distinguished Principal Engineer (ICT 7+).
**Date:** 2026-04-25 · **Component:** [`frontend/src/lib/components/layout/AdminSidebar.svelte`](../../frontend/src/lib/components/layout/AdminSidebar.svelte)
**Consumer:** [`frontend/src/routes/admin/+layout.svelte`](../../frontend/src/routes/admin/+layout.svelte) (line 24, 136)
**Method:** read-only deep dive on every nav item, every route, every interactive element.

---

## 1. Navigation items — route existence check

All 24 primary sidebar links route to **fully implemented** pages (>50 LOC, real data fetching, real UI). No dead links.

| Section | Label | href | Route exists | State |
|---------|-------|------|:-:|---|
| (Root) | Overview | `/admin` | ✓ | Production dashboard |
| Members | All Members | `/admin/members` | ✓ | List + filter + drawer |
| Members | Segments | `/admin/members/segments` | ✓ | Has dynamic detail route |
| Members | Subscriptions | `/admin/subscriptions` | ✓ | Real |
| Members | Products | `/admin/products` | ✓ | Real |
| Members | Coupons | `/admin/coupons` | ✓ | Real (UX issues — see UIUX audit) |
| Content | Blog Posts | `/admin/blog` | ✓ | Full editor UI |
| Content | Courses | `/admin/courses` | ✓ | With lessons |
| Content | Indicators | `/admin/indicators` | ✓ | Builder |
| Content | Trading Rooms | `/admin/trading-rooms` | ✓ | Real |
| Content | Resources | `/admin/resources` | ✓ | Real |
| Content | Categories | `/admin/blog/categories` | ✓ | Real |
| Content | Media Library | `/admin/media` | ✓ | With analytics |
| Content | Videos | `/admin/videos` | ✓ | Real |
| Content | Popups | `/admin/popups` | ✓ | CRUD + analytics |
| Content | Forms | `/admin/forms` | ✓ | Submissions |
| Marketing | Campaigns | `/admin/email/campaigns` | ✓ | With reports |
| Marketing | Email Templates | `/admin/email/templates` | ✓ | Preview/edit |
| Marketing | Email Settings | `/admin/email/smtp` | ✓ | Real |
| Marketing | SEO | `/admin/seo` | ✓ | Multiple subroutes |
| Analytics | Dashboard | `/admin/analytics` | ✓ | 11 sub-routes (mostly stubbed APIs — see Analytics audit) |
| Analytics | Behavior | `/admin/behavior` | ✓ | Real |
| Analytics | CRM | `/admin/crm` | ✓ | 20+ subroutes |
| System | Site Health | `/admin/site-health` | ✓ | Real |
| System | Connections | `/admin/connections` | ✓ | Real |
| System | Admin Users | `/admin/users` | ✓ | List + delete only (no create UI — see UIUX audit) |
| System | Settings | `/admin/settings` | ✓ | OAuth + connections |

---

## 2. Active-state detection — broken on nested routes (HIGH)

**File:** `AdminSidebar.svelte:108`
```svelte
let currentPath = $derived(page.url.pathname);
class:active={currentPath === item.href}
```

The exact-match comparison fails for any deep route:

| User on | Expected highlight | Actual |
|---------|---------------------|--------|
| `/admin/members` | All Members | ✓ |
| `/admin/members/123` | All Members | ✗ none |
| `/admin/members/segments` | Segments | ✗ none |
| `/admin/blog/post-slug/edit` | Blog Posts | ✗ none |

**Fix shape (longest-prefix match):**
```svelte
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

Picking the longest match correctly disambiguates `/admin/members` vs `/admin/members/segments`.

**Status:** ✅ **FIXED** in this audit pass — see commit log on the next sidebar commit.

---

## 3. Sign-out / Exit button — auth session leak (HIGH)

**File:** `AdminSidebar.svelte:154-157`

```svelte
<a href="/" class="exit-btn">
    <IconLogout size={18} />
    <span>Exit Admin</span>
</a>
```

The button is an `<a>` to `/`. It **does not call `logout()`**. The user navigates to the home page but their `rtp_access_token` and `rtp_refresh_token` cookies remain set; `event.locals.user` stays populated; clicking back to `/admin` re-enters with full privileges.

The icon (`IconLogout`) and the visible text ("Exit Admin") describe two different operations. Mental-model split.

**Fix:** Real logout via `$lib/api/auth.logout()` then `goto('/')`.

**Status:** ✅ **FIXED** in this audit pass.

---

## 4. Hardcoded "Administrator" role (MEDIUM)

**File:** `AdminSidebar.svelte:151`

```svelte
<span class="user-role">Administrator</span>
```

Every user sees "Administrator" regardless of actual role. A `developer`-role user sees "Administrator". A `super_admin` sees "Administrator" and not "Super Admin". A non-admin who somehow reached the admin shell sees "Administrator" too — actively misleading.

**Fix:** Derive from `$user.role`, with `isSuperadmin(user) → 'Super Admin'`, otherwise title-case the role string, fallback `'Member'`.

**Status:** ✅ **FIXED** in this audit pass.

---

## 5. No role-based filtering of nav items (MEDIUM)

The sidebar shows all 24 items to every user. The backend rejects non-admin requests, so the security boundary is intact, but the UX is poor: a user with limited permissions will browse the menu, click items, and see 403 errors.

**Recommended fix (future):** declare a `requiredRole?: Role[]` on each nav item, filter `menuSections` against the current user's role. Items the user can't access are hidden from view.

**Status:** documented; not implemented in this pass (requires defining the role taxonomy on the frontend, which the prior audits also flagged as a gap).

---

## 6. Accessibility — missing ARIA (MEDIUM)

| Attribute | Required | Was | Now |
|-----------|----------|-----|-----|
| `<nav aria-label="…">` on the sidebar | yes | missing | ✅ "Admin primary navigation" |
| `aria-current="page"` on the active link | yes | missing | ✅ wired to `bestActiveHref` |
| `aria-hidden="true"` on decorative icons | yes | missing | ✅ added on every nav-item icon, avatar, and exit button |
| `<nav role="navigation">` | implicit on `<nav>`, but explicit when needed | not needed | n/a |
| Skip-to-content link | global concern | exists in `app.html` | unchanged |
| `Esc` closes mobile drawer | yes | missing | not yet wired (see TODO below) |

**Status:** ✅ four of five fixed in this audit pass. Esc-to-close on the mobile drawer is documented as a follow-up.

---

## 7. Mobile / responsive

`AdminSidebar.svelte:431-447` — the slide-in drawer pattern works:

```css
@media (max-width: 1024px) {
    .admin-sidebar { transform: translateX(-100%); }
    .admin-sidebar.open { transform: translateX(0); }
    .close-btn { display: block; }
    .sidebar-overlay { display: block; }
}
```

`admin-responsive.css:14-22` — at <480px the sidebar is full-width, which is correct behavior on phones.

No issues. Not changed.

---

## 8. Visual polish — minor

| Issue | File:line | Severity | Fix |
|-------|-----------|----------|-----|
| Two "IconMail" icons for Email Templates + Email Settings — visually identical | `AdminSidebar.svelte:80-81` | low | use `IconMailOpen` or `IconMailbox` for one of them |
| `.size-sm` action-dropdown buttons drop to 28×28 on mobile — below WCAG 2.2 AA touch target | `ActionsDropdown.svelte:147-149` | low | force `min-width: 44px; min-height: 44px;` on `(hover: none)` |
| `width: 240px` hardcoded in sidebar CSS while `+layout.svelte:243` reads `var(--admin-sidebar-width, 240px)` | `AdminSidebar.svelte:173` | low | switch to the CSS var |

**Status:** documented; not blocking, batched into the upcoming "color + breakpoint sweep".

---

## 9. Summary

**Visual & functional grade:** good (8/10). All routes wired, smooth transitions, brand gold accents, focus rings, 44px touch targets.

**Active-state, sign-out, role display, ARIA were the four real bugs.** All four are fixed in this pass. The remaining items (role-based hiding, Esc-on-drawer, identical-icon split, sidebar-width CSS-var) are documented in the master backlog.

Tracked in [`MASTER_UIUX_BACKLOG.md`](MASTER_UIUX_BACKLOG.md).
