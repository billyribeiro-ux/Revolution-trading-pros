# UI/UX Master Backlog — single source of truth

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.

**Last updated:** 2026-04-25 (commit `<pending>`)

This is the canonical "what's done / what's pending" tracker for every UI/UX
finding from the four 2026-04-25 audits:

- [`UIUX_FORENSIC_AUDIT_2026-04-25.md`](UIUX_FORENSIC_AUDIT_2026-04-25.md) — admin shell + CMS editor + CSS architecture + admin tables + member dashboard
- [`ADMIN_SIDEBAR_AUDIT_2026-04-25.md`](ADMIN_SIDEBAR_AUDIT_2026-04-25.md) — sidebar deep dive
- [`ANALYTICS_DASHBOARD_AUDIT_2026-04-25.md`](ANALYTICS_DASHBOARD_AUDIT_2026-04-25.md) — analytics dashboards
- [`DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md`](DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md) — codebase-wide

Status legend: ✅ done · 🟡 in-flight · ⬜ pending · ⛔ blocked.

---

## Tier 0 — fixed in the 2026-04-25 PE7 cleanup pass

| # | What | Where | Status |
|---|------|-------|--------|
| 1 | Dashboard dropdown's three dead `href: '#'` → real `/dashboard/<slug>` | `frontend/src/routes/dashboard/+page.svelte:60-76` | ✅ |
| 2 | CMS editor: initialize `BlockStateManager` so ImageBlock / VideoBlock / others don't crash when calling `getBlockStateManager()` | `frontend/src/routes/cms/editor/+page.svelte` | ✅ |
| 3 | CMS editor: mobile `@media (max-width: 640px)` block (smaller toolbar padding, hide labels, larger delete button) | `frontend/src/routes/cms/editor/+page.svelte` | ✅ |
| 4 | CMS editor: delete button visible on touch devices (`@media (hover: none)`) and on `:focus-within` / `--selected` | `frontend/src/routes/cms/editor/+page.svelte` | ✅ |
| 5 | AdminSidebar: active-state via longest-prefix match instead of exact string equality (fixes `/admin/members/123` and `/admin/blog/<slug>/edit`) | `frontend/src/lib/components/layout/AdminSidebar.svelte` | ✅ |
| 6 | AdminSidebar: real sign-out via `$lib/api/auth.logout()` + `goto('/')` (was a passive `<a href="/">` leaving the session intact) | `frontend/src/lib/components/layout/AdminSidebar.svelte` | ✅ |
| 7 | AdminSidebar: derive role label from `$user`, with `isSuperadmin` mapping to "Super Admin" (was hardcoded "Administrator") | `frontend/src/lib/components/layout/AdminSidebar.svelte` | ✅ |
| 8 | AdminSidebar: `aria-label` on `<nav>`, `aria-current="page"` on active link, `aria-hidden="true"` on every decorative icon | `frontend/src/lib/components/layout/AdminSidebar.svelte` | ✅ |
| 8a | Keyboard shortcut handler: don't fire `g→d`/`g→a` sequences while typing in `<input>`/`<textarea>`/contenteditable (typing "good" triggered goto-dashboard); remove duplicate `Shift+?` handler that double-toggled help; `Escape` exits cleanly without preventDefault | `frontend/src/lib/stores/keyboard.svelte.ts:194-275` | ✅ |
| 8b | `KeyboardShortcutsHelp.svelte` — clarified the one-way store→prop sync (was opaque); confirmed no feedback loop | `frontend/src/lib/components/KeyboardShortcutsHelp.svelte:56-60` | ✅ |
| 8c | **Analytics dashboard background — strip animated blob effects.** PE7 mandate: no childish decorations on data dashboards. Removed 3 blob divs + `@keyframes float` + gradient-wash background from `admin/analytics/+page.svelte`; flat `#0f172a` surface. Same treatment to `admin/dashboard/+page.svelte` (3 ambient-blob divs + selectors). Replaced `in:fly`/`in:scale` entrance animations with `in:fade` (200ms). | `frontend/src/routes/admin/analytics/+page.svelte`, `frontend/src/routes/admin/dashboard/+page.svelte` | ✅ |

---

## Tier 1 — high-leverage, target this week

| # | What | Where | Owner | Status |
|---|------|-------|-------|--------|
| 9 | `admin/+page.svelte` blob widths fluid (`clamp(min, vw, max)` + `aspect-ratio: 1`) instead of fixed 600/500/400 px | `frontend/src/routes/admin/+page.svelte:918-941` | — | ✅ |
| 10 | `admin/coupons` search `<input>` gets `<label>` + `aria-label` (a11y) | `frontend/src/routes/admin/coupons/+page.svelte:207` | — | ✅ already wired (audit was outdated) |
| 11 | `admin/coupons` swap browser `confirm()` → `ConfirmationModal` + swap `alert()` → `toastStore` | `frontend/src/routes/admin/coupons/+page.svelte:107-112, 348` | — | ✅ already wired (audit was outdated) |
| 12 | Z-index token unification | `frontend/src/app.css`, `frontend/src/lib/components/admin/ActionsDropdown.svelte` | — | ⬜ no real chaos found in sampled files; revisit if a regression appears |
| 13 | Body-scroll-lock on custom modals — use `Modal.svelte` instead of bespoke overlays | `frontend/src/routes/admin/categories/+page.svelte`, `.../courses/+page.svelte:544` | — | ⬜ |
| 14 | Focus trap on `courses` QuickCreate modal (copy from `Modal.svelte:90-113`) | `frontend/src/routes/admin/courses/+page.svelte:544` | — | ⬜ |
| 15 | Admin table responsive: wrap every admin table in `<div style="overflow-x: auto">`, drop `min-width: 600px` rule | `frontend/src/lib/styles/admin-responsive.css:307`, `frontend/src/routes/admin/users/+page.svelte:540` | — | ⬜ |
| 16 | Dynamic admin page title (per-route, not static "Admin Dashboard") | `frontend/src/routes/admin/+layout.svelte:131` | — | ✅ wired `formatPageTitle(page.url.pathname)` into `<svelte:head>` |
| 17 | AdminSidebar Esc-to-close drawer on mobile | `frontend/src/lib/components/layout/AdminSidebar.svelte` | — | ⬜ |
| 18 | AdminSidebar role-based item filtering (declare `requiredRole?: string[]` per item, gate via `isAdmin`/`isSuperadmin`) | `frontend/src/lib/components/layout/AdminSidebar.svelte` | — | ⬜ |
| 19 | `admin/+page.svelte` render the `error` state | `frontend/src/routes/admin/+page.svelte:405-408` | — | ✅ already wired (audit was outdated) |
| 20 | Empty states + skeleton loaders extracted from `MobileResponsiveTable.svelte` and reused on the 11 admin list pages | `frontend/src/lib/components/ui/MobileResponsiveTable.svelte:174` | — | ⬜ |

---

## Tier 2 — system-level investments (this sprint / next)

| # | What | Where | Status |
|---|------|-------|--------|
| 21 | **CMS autosave** — debounce `updateBlock` and POST through a `command` remote function | `frontend/src/routes/cms/editor/+page.svelte` + new `frontend/src/routes/cms/editor/commands.remote.ts` | ⬜ |
| 22 | **Wire `CommandManager`** for undo/redo (already implemented at `frontend/src/lib/utils/command-manager.ts:1-239`, never instantiated) | `frontend/src/routes/cms/editor/+page.svelte` | ⬜ |
| 23 | **CMS drag-and-drop reorder** — native HTML5 DnD or a small Svelte action | `frontend/src/routes/cms/editor/+page.svelte` | ⬜ |
| 24 | **Adopt `<Icon name="…" />` wrapper** + sweep inline `<svg>` in `admin/dashboard/+page.svelte:233-295` and `KpiCard.svelte:74-86` | new `frontend/src/lib/components/Icon.svelte` | ⬜ |
| 25 | **Adopt Recharts** (or Svelte port) for analytics. Migrate `TimeSeriesChart`, then `FunnelChart`. Keep `CohortMatrix` as a table. | `frontend/src/lib/components/analytics/*` | ⬜ |
| 26 | **`KpiCard3D` variant** — CSS perspective + tilt on hover, optional sparkline slot. Roll out per page. | new `frontend/src/lib/components/analytics/KpiCard3D.svelte` | ⬜ |
| 27 | Hardcoded color sweep (732 instances) → admin design tokens. Worst offenders: `ActionsDropdown.svelte:166-172`, `CalloutBlock.svelte` | repo-wide | ⬜ |
| 28 | Breakpoint normalization — pick 640/768/1024/1280/1536; remove the rogue 360/380/480 instances | `frontend/src/lib/styles/admin-responsive.css:154`, `frontend/src/app.css:78, 143, 215` | ⬜ |
| 29 | Profile self-service backend + frontend (no edit name/email/password/avatar today) | `api/src/routes/users.rs` + new frontend forms | ⬜ |

---

## Tier 3 — strategic improvements

| # | What | Status |
|---|------|--------|
| 30 | CMS preset picker (30+ presets seeded in `api/migrations/041_cms_presets.sql`, not exposed) | ⬜ |
| 31 | CMS AI panel (Claude Sonnet 4 wired in `api/src/routes/cms_ai_assist.rs`, no UI) | ⬜ |
| 32 | Breadcrumb component used across admin (currently absent) | ⬜ |
| 33 | KPI detail-view modal (clickable cards drill into time-series + segments) | ⬜ |
| 34 | Goals / Funnels / Cohorts backend implementation (UIs exist, APIs stubbed) | ⛔ depends on backend work |
| 35 | Client-side error rendering on `admin/+page.svelte` (state captured at line 84-86, never shown) — partial overlap with #19 | ⬜ |

---

## Cross-cutting hygiene (lock-in via CI gates)

These should land as `pnpm check`-verified guard rails so they don't regress.

| What | How | Status |
|------|-----|--------|
| No new hardcoded `<your-api-host>` in `frontend/src/` outside the documented exceptions (`test-backend/+page.svelte`, `store/scanners` preconnect) | ESLint rule + CI grep gate | ⬜ |
| No new `as any` casts without an inline justification comment | ESLint `@typescript-eslint/no-explicit-any` set to `error`, allow per-line via comment | ⬜ |
| No new `unwrap_or_default()` on `Result<T, E>` in `api/src/routes/**` | clippy lint + CI | ⬜ |
| Admin-side a11y enforced via `pnpm check:a11y` (already wired) | pre-existing ✅ | ✅ |

---

## Known follow-ups outside this audit

These were scoped out but flagged by the audits — track separately, not part of this UX cleanup.

| Item | Where | Audit |
|------|-------|-------|
| Stripe Checkout-Session creation is a `// TODO` stub | `api/src/routes/subscriptions.rs:446` | distinguished-engineer §1 |
| WebSocket has no JWT validation | `api/src/routes/websocket.rs:344` | security |
| Favorites proxy reads wrong cookie (`session` instead of `rtp_access_token`) | `frontend/src/routes/api/favorites/+server.ts:32` | integration |
| MFA dormant — `services/mfa.rs` complete, never called from `login` | `api/src/routes/auth.rs` | auth |
| Production Fly Postgres unreachable | infra | repo state |

---

## How to use this file

1. Pick an item; create a topic branch named after the row number (e.g. `git checkout -b ux/9-fluid-blob-widths`).
2. Apply the smallest correct change.
3. Run the four gates locally:
   - `pnpm --filter revolution-svelte run check`
   - `pnpm --filter revolution-svelte test:unit`
   - `cd frontend && pnpm exec playwright test tests/e2e --project=chromium`
   - `cd api && cargo check && cargo test --test utils_test --test stripe_test`
4. Update the row's Status column in this file (`⬜` → `✅`) and reference the commit hash.
5. Commit.

The Tier-0 rows above are the template — note the commit ranges and outcomes captured.
