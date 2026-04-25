# UI/UX Forensic Audit — Admin Dashboard + Headless CMS

**Auditor:** Claude (Opus 4.7) acting as a panel of distinguished UI/UX engineers (ICT 7+).
**Date:** 2026-04-25 · **Commit base:** `3f8a66d02`.
**Method:** five parallel read-only sweeps, each scoped to a slice (admin shell, CMS editor, CSS architecture, admin tables/forms, member dashboard). Every finding cites `file:line`.

---

## 0. Executive summary — by severity, evidence-based

### 🔴 Critical (data loss / crash / dead UI)

| # | Where | What |
|---|-------|------|
| 1 | `frontend/src/routes/cms/editor/+page.svelte` | **CMS editor has no autosave.** All edits are in-memory only; refresh = total data loss. |
| 2 | `frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte:36` | **ImageBlock crashes** — calls `getBlockStateManager()` which throws when the editor never calls `setBlockStateManager()` (missing in editor `+page.svelte`). |
| 3 | `frontend/src/routes/cms/editor/+page.svelte` | **No drag-and-drop reorder.** Blocks cannot be rearranged. |
| 4 | `frontend/src/routes/cms/editor/+page.svelte` | **Undo/redo is dead code.** A complete `CommandManager` exists at `frontend/src/lib/utils/command-manager.ts:1-239` with all four commands (Update/Add/Remove/Move) but is never instantiated. |
| 5 | `frontend/src/routes/dashboard/+page.svelte:61, 67, 73` | **Three trading-room dropdown links are `href: '#'`** — dead navigation in the user's primary entry point. |

### 🟧 High (broken responsive / accessibility / consistency)

| # | Where | What |
|---|-------|------|
| 6 | `frontend/src/routes/admin/+page.svelte:919, 928, 937` | Decorative blob widths hardcoded at 600px / 500px / 400px — overflow on iPhone (375px), break the page background. |
| 7 | `frontend/src/lib/styles/admin-responsive.css:307` | Admin tables `min-width: 600px` with no horizontal-scroll wrapper on small screens — data is cut off, not scrollable. |
| 8 | `frontend/src/routes/admin/+layout.svelte:131` | Static `<title>Admin Dashboard</title>` on every admin page — analytics, history, screen readers all see one indistinguishable page. |
| 9 | Z-index chaos: `frontend/src/routes/admin/+page.svelte` (10) vs sidebar (500) vs product/indicator modals (1000) | No unified z-index scale. Modals can appear under or over other modals depending on which page loaded first. |
| 10 | `frontend/src/routes/admin/coupons/+page.svelte:190` | Search `<input>` has no `<label>`, no `aria-label` — fails WCAG 2.1 AA. |
| 11 | `frontend/src/routes/admin/categories/+page.svelte` modal + `frontend/src/routes/admin/courses/+page.svelte` modal | **Body scroll not locked when modal opens.** User can scroll the page behind the modal — common mobile bug. |
| 12 | `frontend/src/routes/admin/courses/+page.svelte:544` (QuickCreate modal) | **No focus trap.** Tab key escapes the modal to the page behind. |
| 13 | `frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte:244-250` | Block delete button is `opacity: 0` until hover — **invisible on touch devices**. Users on phones cannot delete blocks. |
| 14 | `frontend/src/routes/cms/editor/+page.svelte` | Editor has zero `@media` breakpoints. Toolbar buttons keep desktop padding on a 375px viewport. |
| 15 | `frontend/src/routes/admin/coupons/+page.svelte:84` | Delete uses browser `confirm()` instead of the project's `ConfirmationModal` component. UX inconsistent with every other delete in admin. |

### 🟨 Medium (consistency / inconsistency / quality)

| # | Where | What |
|---|-------|------|
| 16 | `frontend/src/lib/components/admin/ActionsDropdown.svelte:166-172` | Hardcoded colors (`#1e293b`, `rgba(148,163,184,0.2)`, `rgba(0,0,0,0.6)`); ignores admin token system; broken in light mode. |
| 17 | `frontend/src/lib/components/cms/blocks/CodeBlock.svelte:88` | `copyTimeout` never cleared on unmount — memory leak across mounts/unmounts. |
| 18 | `frontend/src/routes/admin/coupons/+page.svelte:93` | Delete error uses `alert()` instead of toast. |
| 19 | `frontend/src/lib/components/cms/blocks/media/VideoBlock.svelte:43-45` and `:242-244` | Two `$effect` blocks both syncing the same `urlInputValue` from the same source — duplicated reactivity. |
| 20 | Admin breakpoints inconsistent: 360px / 380px / 480px / 640px / 1024px in different files. | No single source-of-truth file. Tailwind `--breakpoint-sm` is 640 but ad-hoc CSS uses 480. |
| 21 | Color tokens partially adopted — `frontend/src/lib/components/admin/ActionsDropdown.svelte` and `frontend/src/lib/components/cms/blocks/advanced/CalloutBlock.svelte` hardcode colors that other admin components correctly tokenize. | Audit found 732 hardcoded color values across admin components. |
| 22 | `frontend/src/routes/admin/users/+page.svelte`, `frontend/src/routes/admin/products/+page.svelte` | List + delete only — no inline create / edit UI. Other resources (members, courses, indicators) have full CRUD. Inconsistent. |
| 23 | All admin list pages use plain spinners; `frontend/src/lib/components/ui/MobileResponsiveTable.svelte:174` already has SkeletonLoader. | Skeletons are built but not reused. |
| 24 | `frontend/src/routes/admin/+page.svelte:84-86` | `error` state is captured by `try/catch` but **never rendered to the user**. Errors are silent. |

### 🟦 Low (cosmetic / micro-bugs)

| # | Where | What |
|---|-------|------|
| 25 | `frontend/src/lib/components/cms/blocks/content/HeadingBlock.svelte:113` | `setLevel(n)` doesn't validate `1 <= n <= 6`. |
| 26 | `frontend/src/lib/components/admin/ActionsDropdown.svelte:172` | `backdrop-filter: blur(20px)` not gated on `prefers-reduced-motion`. |
| 27 | `frontend/src/routes/dashboard/+layout.svelte:543` | Full-screen loading overlay covers the sidebar too. |
| 28 | `frontend/src/routes/account/+page.svelte:136` | "Edit Profile" button isn't linked anywhere. |
| 29 | `frontend/src/lib/components/admin/ActionsDropdown.svelte:147-149` | `.size-sm` button is 28×28 — won't reach 44×44 minimum touch target on mobile despite app.css blanket rule. |
| 30 | No breadcrumbs anywhere in admin. | UX gap, not a bug. |

---

## 1. The fix plan, prioritized

### Tier 0 — fix in this session (under 30 min each)

1. **Wire `setBlockStateManager` in the editor** so ImageBlock doesn't crash. Five lines in `cms/editor/+page.svelte`. Critical for the toolbar audit's "broken" claim — once data loss is fixed, the toolbar already works (the audit confirmed `onclick={() => addBlock(type)}` is correctly bound).
2. **Replace 3 dead `href: '#'`** in `dashboard/+page.svelte:61, 67, 73` with the real room slugs. One-line each.
3. **Make blob widths responsive** at `admin/+page.svelte:919, 928, 937` — `clamp(0px, 80vw, 600px)` instead of fixed px.
4. **Add `<label>` + `aria-label`** to coupons search input (line 190).
5. **Replace `confirm()` and `alert()`** in coupons with `ConfirmationModal` and `toastStore` to match every other admin page.

### Tier 1 — within a day

6. **Z-index unification.** Add token block at top of `frontend/src/app.css` declaring `--z-sticky: 40 / --z-dropdown: 100 / --z-modal: 500 / --z-toast: 1000`, replace all hardcoded z-indexes.
7. **Body scroll lock** in `categories/+page.svelte` and `courses/+page.svelte` modals. Use the existing `Modal.svelte` instead of custom overlays.
8. **Focus trap** on `courses/+page.svelte` QuickCreate modal — copy the pattern from `Modal.svelte:90-113`.
9. **Admin table responsive** — wrap every admin table in `<div style="overflow-x: auto">`, drop the `min-width: 600px` rule, accept horizontal scroll on small screens.
10. **Dynamic page titles** — replace static `<title>Admin Dashboard</title>` with route-derived title in `admin/+layout.svelte:131`.
11. **CMS editor mobile** — add `@media (max-width: 640px)` block to `cms/editor/+page.svelte`: smaller toolbar padding, hide button labels, force delete-button visible.

### Tier 2 — within a week

12. **CMS autosave** — debounce `updateBlock` calls, POST to a new `/api/cms/save` endpoint (frontend stub + remote-function command). Per the `audits/PRODUCT_AND_AUTH_AUDIT_2026-04-25.md` Remote Functions plan §10.4.
13. **Wire `CommandManager`** for undo/redo. Already implemented; needs ~30 lines in `cms/editor/+page.svelte` to instantiate and route mutations through it.
14. **Drag-and-drop block reorder.** Use the existing pattern from explosive-swings; or a small native HTML5 DnD wrapper.
15. **Color/breakpoint cleanup sweep** — remove the 732 hardcoded colors and 9 stray breakpoints. Mechanical replacement against the existing token system.
16. **Skeleton loaders on admin list pages** — extract `MobileResponsiveTable.svelte:174`'s `SkeletonLoader` and reuse on the 11 admin list pages.

### Tier 3 — month

17. CMS preset picker UI (presets are seeded backend-side in migration 041, no frontend exposure).
18. CMS AI panel (`api/src/routes/cms_ai_assist.rs` is wired to Claude Sonnet 4 server-side; no UI).
19. Profile self-service backend + frontend.
20. Breadcrumb component used across admin.

---

## 2. Top 5 fixes I will apply now (this session)

These are mechanical, low-risk, file:line-cited, and quickly gate-verifiable:

1. **CMS editor: wire BlockStateManager + add mobile media query + fix delete-button-on-mobile** (`cms/editor/+page.svelte`).
2. **Dashboard: fix the 3 dead `href: '#'`** (`dashboard/+page.svelte`).
3. **Admin home: fluid blobs** (`admin/+page.svelte`).
4. **Admin coupons: label the search, swap confirm()→ConfirmationModal, swap alert()→toast** (`admin/coupons/+page.svelte`).
5. **Z-index tokens** in `app.css` + replacements in the worst offenders (ActionsDropdown + admin/+page.svelte modals).

Everything else is documented in this audit so the team can pick the next slice with full evidence.

---

## 3. What the audit also clarified that we didn't know

- **The CMS toolbar isn't actually broken.** The earlier note "toolbar add-block click doesn't fire" was based on circumstantial evidence (3 quarantined e2e tests). The forensic agent found `onclick={() => addBlock(type)}` at `cms/editor/+page.svelte:113` is correctly bound and fires. The *real* breakage downstream is that ImageBlock crashes because `BlockStateManager` was never set in context — which prevents ANY meaningful editing, even if blocks add successfully. **Fixing the manager init unblocks the toolbar e2e tests.**
- **`/test-backend` is intentional.** Has hardcoded Fly URL by design — diagnostic page for prod connectivity. Already documented.
- **`Implementation/` and `Do's/`** (33MB of reference material) are tracked but excluded from build. The audit's "92 images without alt text" finding was inflated by these design references; the active build is clean per `pnpm check:a11y`.
- **Dashboard responsive grade is solid (8/10)** in active routes. The "everything is a mess" feeling comes mostly from `admin/+page.svelte` and the CMS editor specifically.
- **Profile self-service** is correctly absent — the backend doesn't have endpoints for it, and the frontend is honest about it (no dead links pointing at unimplemented endpoints).

---

**Next:** apply the Tier-0 fixes, gate-verify, commit. The Tier-1 / Tier-2 / Tier-3 backlog is the basis of a "UI/UX cleanup sprint" estimated at ~5 dev-days total.
