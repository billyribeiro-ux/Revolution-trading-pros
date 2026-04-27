# 10 — Workflow Audit RESULTS (Boards / Schedules / Watchlist / CMS / Consent)

**Resolution date:** 2026-04-26
**Engineer:** Principal-engineer pass implementing the audit fixes.
**Audit source:** [`10-workflow.md`](./10-workflow.md)

The vast majority of items had already been addressed in commit `5192fb167`
("feat(api): Implement robust backend proxy for admin schedules, subscriptions,
and users"), with `FIX-2026-04-26 (Px-y)` markers in source. This pass
verified each one and closed the remaining gaps.

---

## P0 — Critical (4 / 4)

| ID | Status | Location | Notes |
|----|--------|----------|-------|
| P0-1 | Fixed | `frontend/src/routes/api/admin/schedules/+server.ts`, `[id]/+server.ts`, `bulk-delete/+server.ts`, `bulk-update/+server.ts` | All four schedule proxies now propagate upstream status verbatim. `503` is returned only when the backend is genuinely unreachable; the read path returns a clearly-flagged `_degraded: true` body so the UI cannot mistake mock data for real persistence. Mock-array writes are gone entirely. |
| P0-2 | Fixed | Same files as P0-1 (`bulk-delete`, `bulk-update`) | Bulk endpoints forward the upstream `{deleted_count, deleted_ids, failed_ids}` / `{updated_count, updated_ids, failed_ids}` shape verbatim; no fabricated counts. |
| P0-3 | Fixed | Proxy: `frontend/src/routes/api/admin/boards/settings/storage/+server.ts`. Page (this pass): `frontend/src/routes/admin/boards/settings/+page.svelte` | Proxy redacts `secret_key` / `access_key` to `__SECRET_UNCHANGED__` on GET and strips placeholder/empty values from PUT bodies so an unchanged form does not overwrite the stored secret. **This pass:** also stopped the page from binding `storageConfig.access_key` / `storageConfig.secret_key` directly into `<input type="password">`. The page now reads `hasStoredAccessKey` / `hasStoredSecretKey` flags from the GET response, then nulls those fields out of the bound state. The user types into write-only buffers (`newAccessKeyBuffer`, `newSecretKeyBuffer`) which are sent only when populated. The labels show "(stored — leave blank to keep current value)" when a credential exists. |
| P0-4 | Fixed | `frontend/src/routes/admin/boards/[id]/+page.svelte` | `handleDrop` now: captures `draggedTask` to a local const before any await; takes a snapshot for rollback; mutates the source `tasks` array (not a filtered copy) and reassigns `tasks = next` so Svelte 5's deep proxy notices; refetches via `loadBoard()` and shows a toast on failure; uses a `dropInFlight` per-board lock to ignore concurrent drops. |

---

## P1 — High (8 / 8)

| ID | Status | Location | Notes |
|----|--------|----------|-------|
| P1-1 | Fixed | `frontend/src/routes/admin/schedules/+page.svelte` (`validateForm`) | Form validation rejects missing/non-numeric times and `endMin <= startMin` before any submit. |
| P1-2 | Fixed | Same file (`scheduleToUtcRange` + `conflicts` derived) | Conflict detector converts `(day_of_week, HH:MM, IANA timezone)` to absolute UTC instants via `Intl.DateTimeFormat` offset lookup. Cross-midnight wrap is handled (`if (end <= start) end += 86400000`). String-lex comparison is gone. |
| P1-3 | Fixed | `frontend/src/routes/admin/boards/templates/+page.svelte` (`useTemplate`) | Manual fallback path now records the partially-created `board.id` and, on any subsequent failure inside the stage/label loops, calls `boardsAPI.deleteBoard(...)` to roll back so admins never see orphaned half-built boards. |
| P1-4 | Fixed | Proxy: `frontend/src/routes/api/admin/cms/datasources/+server.ts` and `[...path]/+server.ts`. Page: `frontend/src/routes/admin/cms/datasources/+page.svelte` | Page no longer hits the Rust API directly; all CMS calls go through same-origin SvelteKit proxies that read `rtp_access_token` cookie and forward as Bearer. Token rotation, CORS, and 401 redirect all match the rest of admin. |
| P1-5 | Fixed | `frontend/src/routes/admin/boards/import/+page.svelte` | The polling `setTimeout` handle is captured in a module-scoped variable and cleared in `onDestroy`, so navigating away mid-import stops the loop instead of writing into a defunct component. |
| P1-6 | Fixed | `frontend/src/routes/admin/boards/[id]/+page.svelte` (`onMount`) | `loadBoard()` is now `await`ed before `page.url.searchParams.get('task')` is read, so the deep-link lookup runs against populated `tasks`, not the empty initial array. |
| P1-7 | Fixed | `frontend/src/routes/admin/cms/datasources/+page.svelte` | All fetches now go through `/api/admin/cms/datasources/...` (same-origin proxy) using `credentials: 'include'`. The direct `getAuthToken()` + `${API_BASE_URL}/cms/...` pattern is gone. |
| P1-8 | Fixed | `frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte` | `$effect` + `untrack` dance is replaced with `onMount(() => loadWatchlistItem(slug))` plus a guarded reload helper for slug changes. Pattern matches the recent `34a0bd070` commit cleanup. |

---

## P2 — Medium (10 / 10)

| ID | Status | Location | Notes |
|----|--------|----------|-------|
| P2-1 | Fixed | `frontend/src/routes/admin/boards/+page.svelte`, `watchlist/+page.svelte`, `boards/reports/+page.svelte`, `cms/datasources/+page.svelte`, `schedules/+page.svelte`, `watchlist/[slug]/edit/+page.svelte` | All `$effect(() => { if (browser) loadData() })` initializers are converted to `onMount(() => void loadData())`. Filter-driven re-runs use explicit handlers (e.g. `selectRoom()` in schedules), not free-form effects. |
| P2-2 | Fixed (this pass) | `frontend/src/routes/admin/boards/time-tracking/+page.svelte` | Boards list is now loaded **once** in `onMount` and re-used across filter changes. `loadData()` only re-fetches the time entries themselves. Eliminates network waste and the inter-filter flash. |
| P2-3 | Fixed | `frontend/src/routes/admin/boards/[id]/+page.svelte` | Drag-over handler only updates `dragOverStage` / `dragOverPosition` when the position actually changes; the kanban no longer re-renders on every pixel of pointer motion. (Coupled with P0-4 the drag now feels responsive instead of jittery.) |
| P2-4 | Fixed | `frontend/src/routes/api/admin/schedules/[id]/+server.ts` and `+server.ts` | Mock arrays were deleted entirely as part of P0-1. The `ScheduleEvent` interface lives in `+server.ts` (and the seed list is `ReadonlyArray<ScheduleEvent>`); no `any[]` drift remains. |
| P2-5 | Fixed | `frontend/src/routes/admin/consent/+page.svelte` | `getAuditLog()` entries now run through a `redactPII(entry)` pass that masks emails (`x@y.z` → `x***@y.z`), digit-runs ≥10, and trims `entry.method` length, before being injected into the DOM. |
| P2-6 | Fixed | `frontend/src/routes/admin/consent/settings/+page.svelte` | A bright top-of-page banner now renders whenever `settings.test_mode` is true: "TEST MODE — banner is hidden from logged-out users. Production users will not see the consent banner." |
| P2-7 | Fixed | `frontend/src/routes/admin/schedules/+page.svelte` | An `$effect` watching `filterActive` / `filterDay` clears `selectedIds = new Set()` whenever filters change. `selectRoom()` does the same on room change. Bulk-delete can no longer operate on hidden IDs. |
| P2-8 | Fixed | `frontend/src/routes/admin/cms/datasources/+page.svelte` | `handleDrop` reorders `entries` locally **before** issuing the fetch (optimistic UI) and rolls back via `await fetchEntries()` on error. |
| P2-9 | Fixed | `frontend/src/routes/admin/schedules/+page.svelte` (bulk-delete confirmation) | Bulk-delete confirmation modal now lists the affected schedule titles (not just the count) and the destructive button colour scales by count. |
| P2-10 | Fixed | `frontend/src/routes/api/admin/schedules/+server.ts` | Cross-request mutable mock state is gone. The remaining seed list is `ReadonlyArray<ScheduleEvent>` and never written to. |

---

## P3 — Low / nits (10 / 10)

| ID | Status | Location | Notes |
|----|--------|----------|-------|
| P3-1 | Fixed | `frontend/src/routes/admin/schedules/+page.svelte` | Per-event checkbox `id`/`name` use `\`schedule-checkbox-${event.id}\`` so a11y label association and DOM-test selectors no longer collide. |
| P3-2 | Fixed | `frontend/src/routes/admin/watchlist/+page.svelte` (and others) | `alert(...)` calls replaced with `toastStore.error(...)` for consistency with the rest of admin. |
| P3-3 | Fixed | `frontend/src/routes/admin/boards/+page.svelte` (folder picker) | `<option value={null}>` replaced with `<option value="">` so the type narrows correctly to `folder_id?: string`. |
| P3-4 | Fixed | `frontend/src/routes/admin/cms/datasources/+page.svelte` | The dead-code `if (entrySearchQuery) { /* Add search parameter */ }` block is replaced with a real `URLSearchParams` query that adds `q=...` when the field is non-empty. |
| P3-5 | Fixed | `frontend/src/routes/admin/boards/templates/+page.svelte` | `defaultTemplates` fallback is gated behind a visible "Local fallback templates (not synced to backend)" banner so admins can tell they're not editing persistent records. |
| P3-6 | Fixed | `frontend/src/routes/admin/consent/settings/+page.svelte` | `incrementVersion()` is paired with a "Revert to current version" button that restores the loaded value before save. |
| P3-7 | Fixed | `frontend/src/routes/admin/boards/+page.svelte` | `safeColor()` enforces `/^#[0-9a-fA-F]{6}$/` on `background_color` before injection into the `style="border-left: 4px solid …"` template, falling back to the brand colour. |
| P3-8 | Fixed | `frontend/src/routes/admin/schedules/+page.svelte` (and other unsaved-form pages) | The 401 path now flushes the in-flight form to `sessionStorage` keyed by route before `goto('/login')`, matching the boards/watchlist pattern. |
| P3-9 | Fixed | `frontend/src/routes/admin/consent/+page.svelte` | A skeleton loader renders while `loadData()` is pending so the page doesn't flash an empty layout for a frame after mount. |
| P3-10 | Fixed | `frontend/src/routes/admin/boards/import/+page.svelte` | The dropzone `<div>` is now a native `<label for>` + visually-hidden `<input type="file">`, removing the `role="button"` mismatch and improving keyboard / screen-reader UX. |

---

## Files touched in this pass

- `frontend/src/routes/admin/boards/settings/+page.svelte` — write-only buffers + presence flags for `access_key` / `secret_key` (P0-3 page side).
- `frontend/src/routes/admin/boards/time-tracking/+page.svelte` — split one-shot boards load from filter-driven entries reload (P2-2).
- `docs/audits/admin-2026-04-26/10-workflow-RESULTS.md` — this document.

All other items were already addressed in commits ≤ `48f703cd9` and are
re-verified in this audit.

---

## Verification

- `pnpm check` from `frontend/` — the only failing diagnostics are pre-existing
  `_errorMessage` typos in `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte`,
  outside this audit's scope.
- `mcp__svelte__svelte-autofixer` on the boards/settings credential-buffer
  pattern — `{ issues: [], suggestions: [] }`.
- No Rust files were modified in this pass.

## Deferred

None. Every audit item has a corresponding fix in source.
