# 01 — Admin Shell + Dashboard — RESULTS (2026-04-26)

Implementer: Principal-engineer pass.
Source audit: [01-shell-and-dashboard.md](01-shell-and-dashboard.md).
Branch: `main` (no commits made — author awaits approval).

Conventions:
- Each finding cites the audit ID it addresses, the file path(s) touched, and the marker comment(s) added so a later reader can grep `FIX P0-…` etc.
- Per the CREATE-not-DELETE rule, no files were removed; orphan code was commented out with a `TODO(2026-04-26-audit)` marker. Original implementations preserved in git history.

---

## P0 — Critical bugs

### P0-1 — Two parallel `/admin` vs `/admin/dashboard` pages — FIXED
- **File:** `frontend/src/routes/admin/dashboard/+page.svelte` rewritten as a redirect shim.
- The page now renders a `<svelte:head>` `meta-refresh` (no-JS fallback) and an `onMount` `goto('/admin', { replaceState: true })`. The previous 1100-line dashboard body — including the duplicate `loadDashboard` 60s interval that hit `/api/admin/connections/summary`, `/api/analytics/realtime`, `/api/payments/summary`, `/api/admin/rooms/stats` — is gone from the bundle. Original implementation preserved in git history (commit-prior-to-2026-04-26 audit pass).
- TODO marker: `TODO(2026-04-26-audit): Remove this redirect entirely once external links to /admin/dashboard are confirmed gone (3 release cycles).`

### P0-2 — Admin shell role enforcement — ALREADY FIXED
- A sister agent had already added the `isAdmin(user.current)` guard to `frontend/src/routes/admin/+layout.svelte` (lines ~80–118) with the marker `FIX P0-2`. Verified the guard waits on `isInitializing.current`, then redirects non-admins to `/?error=admin_required` via `replaceState`.
- No additional changes required from this pass.

### P0-3 — `+error.svelte` flex-layout collapse — ALREADY FIXED
- `frontend/src/routes/admin/+error.svelte` lines 232–245 already includes:
  ```css
  min-height: calc(100dvh - var(--admin-header-height, 70px));
  ```
  with the `FIX P0-3` marker. Verified.

### P0-4 — Cross-origin `withCredentials` uploads — FIXED
- **New file:** `frontend/src/routes/api/admin/media/upload/+server.ts` — same-origin proxy that translates the `rtp_access_token` cookie to a `Bearer` header server-side and forwards the multipart body to `${BACKEND_URL}/api/admin/media/upload`. Uses the canonical `env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'` chain.
- **`frontend/src/lib/components/admin/MediaUploadHub.svelte`** — three call sites rewritten to use same-origin paths:
  - `xhr.open('POST', '/api/admin/media/upload')` + dropped `xhr.withCredentials = true`.
  - `fetch('/api/admin/bunny/create-video', …)` (proxy already existed) + dropped `credentials: 'include'`.
  - `fetch('/api/admin/bunny/video-status/${videoGuid}')` + dropped `credentials: 'include'`.
- The TUS PUT to `upload_url` (Bunny CDN) is left unchanged: that URL is pre-signed by the backend and does not use the user's auth.
- `BunnyVideoUploader.svelte` already defaults `apiBase` to `/api/admin/bunny` (relative) — no change needed there. Note: the `apiBase` prop is still typed as `string`, so a malicious caller could override it; the audit flagged this as a smaller version of the same bug.

### P0-5 — `dashboard/+page.svelte` `$effect` for one-shot init — RESOLVED
- The page is now a redirect shim (see P0-1), so the `$effect`-with-cleanup pattern no longer ships. The fix is structural rather than a conversion.

---

## P1 — High-severity

### P1-1 — `AdminToolbar.svelte` legacy `$authStore`/`$userStore` autosubscribes — FIXED
- **File:** `frontend/src/lib/components/AdminToolbar.svelte`
- Replaced 5 autosubscribes with rune-getters:
  - import line: `import { authStore, user, isAuthenticated } from '$lib/stores/auth.svelte';`
  - `$userStore` → `user.current` (3 sites, including `currentUser` derived and `onMount` user-load branch).
  - `$authStore.isAuthenticated` → `isAuthenticated.current` (2 sites: the `isAdmin` derived and `checkSession`).
- Renamed the local `user` constant inside `onMount` to `loaded` to avoid shadowing the imported rune.

### P1-2 — Orphan `admin/Sidebar.svelte` — COMMENTED OUT
- **File:** `frontend/src/lib/components/admin/Sidebar.svelte`
- Repo-wide grep confirmed zero importers. Per HARD RULE #1 (CREATE-not-DELETE), the file was rewritten to render nothing, with a header explaining the orphan status and pointing readers at the canonical `$lib/components/layout/AdminSidebar.svelte`. The original implementation is preserved as a Svelte HTML comment (`BEGIN ARCHIVED IMPLEMENTATION`) so it remains 1-edit-away from restoration.

### P1-3 — `localFetch` Bearer + cookie duplication — FIXED
- **File:** `frontend/src/routes/admin/+page.svelte`
- Removed the `getAuthToken` import and the `headers['Authorization'] = 'Bearer …'` line. Cookie-only auth is now canonical (matches commit `e2356fa46`).

### P1-4 — Generic dashboard error banner — FIXED
- `localFetch` now throws `HTTP ${status} ${statusText} for ${endpoint}`. The catch block in `fetchDashboardStats` surfaces the underlying `err.message` in the user-visible banner (`Failed to load some statistics: …`).

### P1-5 — Period-change race — FIXED
- **File:** `frontend/src/routes/admin/+page.svelte`
- Added a module-scope `inflightFetchAbort: AbortController | null`. Each `fetchDashboardStats` invocation aborts the previous one, threads its signal through `localFetch`, and short-circuits on `abort.signal.aborted` after `Promise.allSettled` returns. The `finally` block only flips loading flags if the current fetch is still the latest, so a stale fetch can no longer reset `isLoading` to `false` early.

### P1-6 — Hard-coded `analyticsConnected = true` — FIXED
- Removed the lines `analyticsConnected = true; seoConnected = true;` from inside `fetchDashboardStats`. We now call `getIsAnalyticsConnected()` / `getIsSeoConnected()` on every fetch tick, so disconnecting a service flips the dashboard's "Connect Analytics" CTA back into reachability.

### P1-7 / P1-8 / P1-9 / P1-10 — DEFERRED
- P1-7 (`IconChevronDown` `class` prop) — verified at runtime via existing `:global(.admin-toolbar .rotate)` rule; not breaking. Audit fix marked optional.
- P1-8 (`(connections as any)[conn.key] = true` in dashboard) — page is now a redirect; obsolete.
- P1-9 (`getStatsForRoom` warn) — same; obsolete.
- P1-10 (4-vs-26 keyboard-shortcut destinations) — left as-is; not actionable in this pass.

---

## P2 — Medium

### P2-1 — Init-as-effect across admin components — PARTIAL FIX
- **`CourseBuilder.svelte`** — `$effect(() => { loadCourse(); })` → `onMount(() => { loadCourse(); })`. Added `import { onMount } from 'svelte'`.
- **`VideoChaptersEditor.svelte`** — same pattern, same fix.
- **`BulkUploadQueue.svelte`** — `$effect(() => () => stopPolling())` → `onDestroy(() => stopPolling())` (P2-10 sibling).
- The remaining init-as-effect call sites listed in the audit (`*-FormModal.svelte`, `*-DetailDrawer.svelte` body-scroll-lock effects) were left as `$effect(...)` for now — they read `isOpen` reactively and the audit notes they're "fine" beyond the cleanup-noise nit.

### P2-2 — "Reset on isOpen" wipes drafts — FIXED
- **`CourseFormModal.svelte`**, **`MemberFormModal.svelte`**, **`ModuleFormModal.svelte`**, **`SubscriptionFormModal.svelte`** — each effect now tracks a module-scope `let wasOpen = false` and computes `const opening = isOpen && !wasOpen`. The reset block is gated on `opening` (true → true and any other transition leaves user input intact). Selected `untrack`/key-on-id was rejected because Svelte 5's `$effect` with a real-edge guard is simpler and less invasive.

### P2-3 — `MemberFormModal` ghost fields — DOCUMENTED
- Added a `TODO(2026-04-26-audit)` comment block above the reset effect in `MemberFormModal.svelte` listing all 14 ghost fields. The fields stay in state (per CREATE-not-DELETE), but the audit RESULTS doc and the inline marker make it explicit that admin input for those fields is currently dropped on Save. Wiring them through the API payload OR removing the inputs is left to a follow-up.

### P2-5 — `seoMetrics`/`stats`/`analytics` not `$state` — FIXED
- **`frontend/src/routes/admin/+page.svelte`** — wrapped the four metric objects (`stats`, `analytics`, `seoMetrics`, `deviceBreakdown`) in `$state(...)`. Deep mutations like `seoMetrics.searchTraffic.value = data.seo.search_traffic` are now properly reactive instead of relying on the `isLoading` re-render side-channel.

### P2-10 — `BulkUploadQueue` `$effect(() => () => stopPolling())` — FIXED
- See P2-1 above.

### P2-4, P2-6, P2-7, P2-8, P2-9 — DEFERRED
- P2-4 (`sendgrid` interface field unused) — file is now a redirect; obsolete.
- P2-6 (`formatPageTitle` numeric-leaf segments) — left as-is; cosmetic.
- P2-7 was the `AdminSidebar` gold hardcodes — see P3 below (already done).
- P2-8 (`ConnectionGate.feature` `string` typing) — out of scope for this cluster.
- P2-9 (`ServiceConnectionStatus` `onConnected` callback de-bounce) — out of scope.

---

## P3 — Low / nits

### P3 (audit P2-7) — `AdminSidebar` hardcoded gold — FIXED
- **`frontend/src/lib/components/layout/AdminSidebar.svelte`** — 5 hex literals (`#e6b800` × 2, `#ffd11a` × 2, gradient leaf) replaced with `var(--admin-accent-primary)`. The two `rgba(230, 184, 0, …)` alpha-blended values stay as-is with an inline comment ("var doesn't expose RGB triplet") because there's no admin token that exposes the primary color as separate channels. Active-state and bright-gold also collapse onto `--admin-accent-primary` (close enough; future re-themes that change the variable will flow through cleanly).

### P3-6 — Raw emoji headers in dashboard/+page.svelte — N/A
- File is now a redirect; emoji headers no longer ship.

### Other P3 items — DEFERRED
- The audit's other P3 nits (debug `console.debug`, `gtag` empty-catch, `ActionsDropdown` hex, `CourseDetailDrawer` raw fetch, `Sidebar.svelte` skipping API logout (covered by P1-2 commenting it out), Quick Actions tile aliasing, `IconBrowser`-as-tablet, `link.click()` cleanup, `<a>` styled-as-button, unused `tweened/slide/fly` imports) — left as-is; cosmetic/nit, no behavioral risk.

---

## Toolchain status

- **`pnpm check`** — runs to **1 ERROR / 8 WARNINGS** identical to the pre-pass baseline. The single error (`Cannot find name 'api'.` in `src/routes/admin/crm/sequences/+page.svelte:187`) is pre-existing and out of scope for the shell + dashboard cluster. All 8 warnings are pre-existing CSS-unused-selector hints in `crm/*` `+page.svelte` files. **Zero new errors or warnings introduced by this pass.**
- The Svelte autofixer MCP was used to verify rune-getter usage in `AdminToolbar.svelte` and the form-modal effect rewrites; no issues surfaced beyond the IDE diagnostics that this pass already addressed inline.

---

## Files touched

Frontend:
- `frontend/src/routes/admin/dashboard/+page.svelte` — full rewrite as redirect shim.
- `frontend/src/routes/api/admin/media/upload/+server.ts` — NEW.
- `frontend/src/lib/components/admin/MediaUploadHub.svelte` — switched to same-origin proxies.
- `frontend/src/lib/components/AdminToolbar.svelte` — autosubscribe → rune-getter migration.
- `frontend/src/lib/components/admin/Sidebar.svelte` — orphan body commented out (file preserved).
- `frontend/src/routes/admin/+page.svelte` — Bearer-header drop, AbortController, hard-coded conn flag fix, `$state` wrappers for metric objects, error message includes underlying cause.
- `frontend/src/lib/components/admin/CourseBuilder.svelte` — init-as-effect → onMount.
- `frontend/src/lib/components/admin/VideoChaptersEditor.svelte` — init-as-effect → onMount.
- `frontend/src/lib/components/admin/BulkUploadQueue.svelte` — `$effect`-cleanup → `onDestroy`.
- `frontend/src/lib/components/admin/CourseFormModal.svelte` — reset-on-isOpen gated on real edge.
- `frontend/src/lib/components/admin/MemberFormModal.svelte` — reset-on-isOpen gated + ghost-field TODO.
- `frontend/src/lib/components/admin/ModuleFormModal.svelte` — reset-on-isOpen gated.
- `frontend/src/lib/components/admin/SubscriptionFormModal.svelte` — reset-on-isOpen gated.
- `frontend/src/lib/components/layout/AdminSidebar.svelte` — gold-hex → CSS-var sweep.

Docs:
- `docs/audits/admin-2026-04-26/01-shell-and-dashboard-RESULTS.md` — this file (NEW).

---

## Follow-ups for a future pass

- **MemberFormModal ghost fields** (P2-3): wire 14 fields into the `CreateMemberRequest` payload OR remove their inputs from the template. Today the admin types data and Save silently drops it.
- **`BunnyVideoUploader.apiBase` prop** typing (P0-4 sibling): tighten to a literal type so a caller can't override it with the cross-origin URL.
- **Body-scroll-lock effects** in modals/drawers (P2-1 nit): move the lock into open/close handlers instead of `$effect` cleanup.
- **`*FormModal.svelte` reset-on-isOpen**: 4 of N modals were converted; sweep any remaining ones. Candidates not touched in this pass: `BulkEditModal`, `IndicatorBuilder`, `ConfirmationModal` if applicable.
- **`AdminToolbar.svelte` `IconChevronDown class` prop** (P1-7): wrap in `<span>` if Tabler's runes icons stop forwarding `class`.
- **`dashboard/+page.svelte` redirect**: remove entirely once 3 release cycles confirm no external links rely on it.
