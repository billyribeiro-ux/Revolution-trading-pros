# 10 — Workflow Audit (Boards / Schedules / Watchlist / CMS / Consent)

**Audit date:** 2026-04-26
**Auditor:** Principal-engineer pass, read-only
**Scope:** Productivity / workflow admin surfaces — Boards, Schedules, Watchlist, CMS Datasources, Consent.

---

## Files reviewed

### Boards
- [`frontend/src/routes/admin/boards/+page.svelte`](../../../frontend/src/routes/admin/boards/+page.svelte) — board listing dashboard
- [`frontend/src/routes/admin/boards/[id]/+page.svelte`](../../../frontend/src/routes/admin/boards/[id]/+page.svelte) — kanban detail view (drag/drop, modals, timer)
- [`frontend/src/routes/admin/boards/settings/+page.svelte`](../../../frontend/src/routes/admin/boards/settings/+page.svelte) — boards settings + cloud-storage credentials
- [`frontend/src/routes/admin/boards/templates/+page.svelte`](../../../frontend/src/routes/admin/boards/templates/+page.svelte) — template gallery + provisioning
- [`frontend/src/routes/admin/boards/time-tracking/+page.svelte`](../../../frontend/src/routes/admin/boards/time-tracking/+page.svelte) — time-entry reports
- [`frontend/src/routes/admin/boards/import/+page.svelte`](../../../frontend/src/routes/admin/boards/import/+page.svelte) — Trello/Asana/CSV import wizard
- [`frontend/src/routes/admin/boards/reports/+page.svelte`](../../../frontend/src/routes/admin/boards/reports/+page.svelte) — board analytics

### Schedules
- [`frontend/src/routes/admin/schedules/+page.svelte`](../../../frontend/src/routes/admin/schedules/+page.svelte) — weekly schedule editor
- [`frontend/src/routes/api/admin/schedules/+server.ts`](../../../frontend/src/routes/api/admin/schedules/+server.ts) — list + create
- [`frontend/src/routes/api/admin/schedules/[id]/+server.ts`](../../../frontend/src/routes/api/admin/schedules/[id]/+server.ts) — get/put/delete
- [`frontend/src/routes/api/admin/schedules/bulk-delete/+server.ts`](../../../frontend/src/routes/api/admin/schedules/bulk-delete/+server.ts)
- [`frontend/src/routes/api/admin/schedules/bulk-update/+server.ts`](../../../frontend/src/routes/api/admin/schedules/bulk-update/+server.ts)

### Watchlist
- [`frontend/src/routes/admin/watchlist/+page.svelte`](../../../frontend/src/routes/admin/watchlist/+page.svelte)
- [`frontend/src/routes/admin/watchlist/create/+page.svelte`](../../../frontend/src/routes/admin/watchlist/create/+page.svelte)
- [`frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte`](../../../frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte)

### CMS
- [`frontend/src/routes/admin/cms/datasources/+page.svelte`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte) — datasource + entry CRUD with drag-reorder, CSV import/export

### Consent
- [`frontend/src/routes/admin/consent/+page.svelte`](../../../frontend/src/routes/admin/consent/+page.svelte) — analytics dashboard
- [`frontend/src/routes/admin/consent/+page.ts`](../../../frontend/src/routes/admin/consent/+page.ts)
- [`frontend/src/routes/admin/consent/settings/+page.svelte`](../../../frontend/src/routes/admin/consent/settings/+page.svelte) — GDPR/CCPA settings
- [`frontend/src/routes/admin/consent/templates/+page.svelte`](../../../frontend/src/routes/admin/consent/templates/+page.svelte)
- [`frontend/src/routes/admin/consent/templates/+page.ts`](../../../frontend/src/routes/admin/consent/templates/+page.ts)
- [`frontend/src/routes/api/admin/consent/settings/+server.ts`](../../../frontend/src/routes/api/admin/consent/settings/+server.ts)
- [`frontend/src/routes/api/admin/consent/settings/bulk/+server.ts`](../../../frontend/src/routes/api/admin/consent/settings/bulk/+server.ts)
- [`frontend/src/routes/api/admin/consent/settings/reset/+server.ts`](../../../frontend/src/routes/api/admin/consent/settings/reset/+server.ts)

---

## Critical bugs (P0)

### P0-1 — Mock-only schedule proxies will silently corrupt prod once backend is wired
**File:** [`frontend/src/routes/api/admin/schedules/+server.ts:362-490`](../../../frontend/src/routes/api/admin/schedules/+server.ts), all four schedule proxies.

The schedule proxies follow a "try backend, fall back to mock" pattern. The fallback writes succeed and return `success: true` even when the backend is reachable but returned a non-`success` response (e.g. `400`, `409`, `500`):

```ts
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	...
	if (!response.ok) {
		console.warn(`Backend returned ${response.status} for ${endpoint}`);
		return null;
	}
	...
}

// in POST:
if (backendData?.success) {
	return json(backendData);
}

// Fallback to mock data
...
mockSchedules.push(newSchedule);
return json({ success: true, data: newSchedule, ... });
```

Any backend that is reachable but unhealthy (auth fails, validation rejects, race-condition transient 500) is **silently swallowed**, and the proxy lies to the client by returning `success: true` from the in-process mock array. The mock array is also process-scoped — completely lost on every redeploy / dyno cycle — so admins will see schedules "save" then disappear. The same anti-pattern is in [`bulk-delete/+server.ts:38-80`](../../../frontend/src/routes/api/admin/schedules/bulk-delete/+server.ts), [`bulk-update/+server.ts:38-91`](../../../frontend/src/routes/api/admin/schedules/bulk-update/+server.ts), and [`[id]/+server.ts:129-228`](../../../frontend/src/routes/api/admin/schedules/[id]/+server.ts).

Compounding it, [`[id]/+server.ts:24`](../../../frontend/src/routes/api/admin/schedules/[id]/+server.ts) declares `const mockSchedules: any[] = []` **per file** — so the GET-by-id mock array is a different instance from the list/create mock array in [`+server.ts:46`](../../../frontend/src/routes/api/admin/schedules/+server.ts). Creating a schedule in mock mode then fetching it by id returns 404.

**Recommendation:** Either (a) delete the mock fallback and fail loud when the backend is misconfigured, or (b) gate the fallback strictly on `if (!BACKEND_URL || network error)` and propagate non-2xx upstream responses verbatim.

### P0-2 — Bulk endpoints have no transaction wrapper and report fictitious success
**Files:** [`frontend/src/routes/api/admin/schedules/bulk-delete/+server.ts:71-79`](../../../frontend/src/routes/api/admin/schedules/bulk-delete/+server.ts), [`bulk-update/+server.ts:81-90`](../../../frontend/src/routes/api/admin/schedules/bulk-update/+server.ts).

Mock fallback returns `deleted_count: ids.length` regardless of what actually happened:

```ts
return json({
	success: true,
	data: {
		deleted_count: ids.length,
		deleted_ids: ids
	},
	message: `${ids.length} schedules deleted successfully`,
	_mock: true
});
```

There is no per-id loop, no validation, no `Pool::begin()` wrapper at the proxy layer (proxy can't do this — but the **frontend caller** also can't tell partial-failure from total-success). Combined with P0-1, an admin clicking "Delete 50 selected" will get a green toast even if zero schedules were actually deleted upstream. CLAUDE.md §"Bulk-action safety" explicitly calls this pattern out.

**Recommendation:** Forward upstream's structured `{deleted_ids, failed_ids}` response. Don't fabricate counts in the proxy.

### P0-3 — Cloud storage secret keys are read back to the browser in plaintext
**File:** [`frontend/src/routes/admin/boards/settings/+page.svelte:610-635`](../../../frontend/src/routes/admin/boards/settings/+page.svelte).

The settings load path issues `boardsAPI.getStorageConfig()` then binds the response into a `<input type="password">` two-way bound to `storageConfig.secret_key` and `storageConfig.access_key`:

```svelte
<input
    type="password"
    id="secret-key"
    autocomplete="current-password"
    bind:value={storageConfig.secret_key}
    ...
/>
```

`type="password"` only obscures rendering — DevTools, view-source, and the network tab will all show the secret. For S3/R2/B2 credentials this is a credential-leakage class bug. Per CLAUDE.md §"CMS datasource credentials": secrets must never be returned to the client.

**Recommendation:** Backend should return masked placeholder (`"***"`) when secret is set, and accept partial updates (only update if user typed a new value). The bound state should be a separate "newSecret" buffer that is empty by default and only sent on save.

### P0-4 — Drag-drop in `[id]/+page.svelte` desyncs when reorder fails
**File:** [`frontend/src/routes/admin/boards/[id]/+page.svelte:351-373`](../../../frontend/src/routes/admin/boards/[id]/+page.svelte).

```ts
async function handleDrop(event: DragEvent, stageId: string, position: number) {
    event.preventDefault();
    if (!draggedTask) return;

    try {
        const updated = await boardsAPI.moveTask(boardId, draggedTask.id, stageId, position);
        tasks = tasks.map((t) => (t.id === draggedTask!.id ? updated : t));

        // Reorder other tasks in the stage
        const stageTasks = tasks.filter((t) => t.stage_id === stageId && t.id !== draggedTask!.id);
        stageTasks.forEach((t, i) => {
            if (i >= position) {
                t.position = i + 1;
            }
        });
    } catch (error) {
        console.error('Failed to move task:', error);
    } ...
}
```

Three problems:

1. **Mutation of derived array — silent no-op.** `stageTasks` is a fresh filtered array; mutating `t.position` on `t` mutates objects already in `tasks`, but Svelte 5's deep proxy will not notice unless `tasks` is reassigned. The visible reorder is therefore inconsistent.
2. **No optimistic-rollback.** On failure, `console.error` swallows the exception; the UI is left showing the dragged task in the new column even though the server rejected the move. A page refresh restores reality.
3. **Race condition with concurrent drops.** `draggedTask` is module-level state; if a user fires two quick drops, the second `await` resolves against the wrong `draggedTask` (the `!` non-null assertions on lines 357 and 360 will throw).

**Recommendation:** Capture `draggedTask` into a local const before the await, do an optimistic local update with rollback in catch, and use `tasks = [...tasks]` to force reactivity.

---

## High-severity issues (P1)

### P1-1 — Schedule cron/recurrence has zero validation
**File:** [`frontend/src/routes/admin/schedules/+page.svelte:357-386`](../../../frontend/src/routes/admin/schedules/+page.svelte) (createSchedule) and [`api/admin/schedules/+server.ts:446-490`](../../../frontend/src/routes/api/admin/schedules/+server.ts) (POST validation).

The form accepts `recurrence: 'weekly' | 'biweekly' | 'monthly' | null` but:

- **No check that `start_time < end_time`.** A user can save `start_time: 17:00, end_time: 09:00` and the conflict-detector at line 239 will silently produce nonsense (`a.start_time < b.end_time && b.start_time < a.end_time` becomes a string-lex comparison on `"17:00" < "09:00"` which evaluates `false`, hiding the conflict).
- **`day_of_week` is sent as whatever the `<select>` returns**, including `null` for "One-time" recurrence form layout, but the type says `recurrence: 'weekly' | ... | null` not `day_of_week`. The server route at line 449 only checks `body.day_of_week === undefined`, so `null` slips through.
- **`timezone` is unvalidated** — accepts any string. There is no IANA-DB check; sending `"America/Atlantis"` bypasses both client and server.
- **Cron-expression parsing was in scope for the audit** but no cron expression is exposed in the UI; the only recurrence model is the enum. If the backend uses cron internally to render this schedule, the frontend gives no visibility into how `biweekly` / `monthly` are anchored (which week? which Monday-of-month?). Edge cases like "5th Monday of March" are unrepresentable.

### P1-2 — Conflict detector uses string compare, doesn't normalize timezone
**File:** [`frontend/src/routes/admin/schedules/+page.svelte:228-247`](../../../frontend/src/routes/admin/schedules/+page.svelte).

```ts
// Check for overlap
if (a.start_time < b.end_time && b.start_time < a.end_time) {
    conflictPairs.push([a, b]);
}
```

Lex-compare on `"HH:MM"` happens to work for same-day same-tz comparisons, but the model has a `timezone` per event — two events at `09:30 ET` and `09:30 PT` are the same day_of_week and start_time but three hours apart in real time, and this code marks them as conflicting. Conversely `23:30 ET` and `02:30 ET (next day)` aren't seen as adjacent.

**Recommendation:** Convert to UTC minutes (`hh * 60 + mm + tzOffset`) before comparison; or do the conflict check server-side where Postgres `tstzrange` can do it correctly.

### P1-3 — `useTemplate` fallback creates orphaned partial boards on failure
**File:** [`frontend/src/routes/admin/boards/templates/+page.svelte:213-245`](../../../frontend/src/routes/admin/boards/templates/+page.svelte).

```ts
async function useTemplate(template: BoardTemplate) {
    creatingBoard = template.id;
    try {
        const board = await boardsAPI.createFromTemplate(template.id, template.title);
        goto(`/admin/boards/${board.id}`);
    } catch (error) {
        // Fallback: Create board manually with template config
        try {
            const board = await boardsAPI.createBoard({...});
            for (const stage of template.stages) {
                await boardsAPI.createStage(board.id, stage);   // <-- if this fails halfway,
            }                                                    // board is half-built
            for (const label of template.labels) {
                await boardsAPI.createLabel(board.id, label);
            }
            goto(`/admin/boards/${board.id}`);
        } catch (err) {
            console.error('Failed to create board:', err);  // <-- and we leave the orphan
        }
    }
}
```

If `createStage` fails on stage 4 of 7, the board exists with 3 stages, no labels, and zero rollback. No transaction wrapper, no compensating delete. CLAUDE.md §"Rust API > new mutations that touch >1 table need a `Pool::begin()` ... transaction wrapper" applies here — it should be a single backend RPC, not a sequence of frontend awaits.

### P1-4 — Watchlist delete uses `alert()` and has no confirmation modal in `[slug]/edit`
**File:** [`frontend/src/routes/admin/watchlist/+page.svelte:208-211`](../../../frontend/src/routes/admin/watchlist/+page.svelte).

```ts
if (newItem.rooms.length === 0) {
    alert('Please select at least one room');
    return;
}
```

`alert()` was supposed to have been replaced with `toastStore` (see boards/settings/+page.svelte for the FIX-2026-04-26 comments). One straggler.

Also [`watchlist/[slug]/edit/+page.svelte:139-148`](../../../frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte):

```ts
async function handleDelete() {
    try {
        await watchlistApi.delete(slug);
        await goto('/admin/watchlist');
    } catch (err) {
        ...
        showDeleteModal = false;
    }
}
```

Successfully deleting on the edit page does **not close the modal first** — instead it `await goto`'s while the modal is still open. If the goto fails (network blip, browser-back), the modal stays mounted and the user has lost trace of what happened.

### P1-5 — Polling import status leaks when component unmounts
**File:** [`frontend/src/routes/admin/boards/import/+page.svelte:103-120`](../../../frontend/src/routes/admin/boards/import/+page.svelte).

```ts
async function pollImportStatus() {
    if (!importJob) return;
    const poll = async () => {
        try {
            const job = await boardsAPI.getImportJob(importJob!.id);
            importJob = job;
            if (job.status === 'processing' || job.status === 'pending') {
                setTimeout(poll, 2000);  // <-- never cleared on unmount
            }
        } catch (err) {
            console.error('Failed to poll import status:', err);
        }
    };
    poll();
}
```

If user navigates away mid-import the poll continues forever (well, until status changes — but the network call still runs, and `importJob = job` writes to a defunct component's state, throwing `state_unsafe_mutation` warnings in production builds). No `onDestroy` / cleanup.

### P1-6 — `+page.ts` for boards [id] is missing — kanban detail page renders without server config
The boards listing has no load function, but [`/admin/boards/[id]/+page.svelte`](../../../frontend/src/routes/admin/boards/[id]/+page.svelte) does heavy DOM work (drag handlers, intervals, modals) and uses `page.url.searchParams.get('task')` at mount time **before** `tasks` has loaded ([line 118-124](../../../frontend/src/routes/admin/boards/[id]/+page.svelte)):

```ts
onMount(() => {
    loadBoard();     // async, doesn't await
    const taskId = page.url.searchParams.get('task');
    if (taskId) {
        const task = tasks.find((t) => t.id === taskId);  // tasks is still []
        if (task) openTaskModal(task);
    }
    ...
});
```

Deep-linking to a task (`/admin/boards/abc?task=xyz`) silently does nothing because `tasks` is `[]` at this point — `loadBoard()` was called but not awaited. A new arrival from the dashboard sidebar that uses these `?task=` deep links will appear broken with no error.

### P1-7 — CMS datasources page reads token from `auth.svelte` but other routes use `rtp_access_token` cookie
**File:** [`frontend/src/routes/admin/cms/datasources/+page.svelte:35`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte) imports `getAuthToken` from `$lib/stores/auth.svelte` and bears it as `Authorization: Bearer ${token}` directly to `${API_BASE_URL}/cms/datasources` — **bypassing the SvelteKit proxy entirely**. Every other admin surface in this audit calls `/api/admin/...` and the proxy attaches the canonical `rtp_access_token` cookie token (per CLAUDE.md auth convention).

Three concrete consequences:
1. CORS — direct calls to `<your-api-host>` from the browser require the backend to allow the admin origin, while same-origin proxy calls don't. Production is currently coupling the CMS surface to a CORS contract no other surface needs.
2. Token lifecycle drift — refresh-token rotation handled by the proxy doesn't apply.
3. Inconsistent failure modes — when the rest of admin starts redirecting on 401, this page won't.

### P1-8 — Watchlist edit page `$effect` + `untrack` pattern is fighting Svelte 5
**File:** [`frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte:60-70`](../../../frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte).

```ts
$effect(() => {
    if (!browser) return;
    const currentSlug = slug;     // tracks slug
    untrack(() => {
        loadWatchlistItem(currentSlug);  // doesn't track loadWatchlistItem internals
    });
});
```

This works but is fragile: if `loadWatchlistItem` ever begins reading a `$state` outside `untrack`, you'll get an infinite loop. The simpler pattern — `onMount(() => loadWatchlistItem(slug))` plus a manual reload guard for slug-changes — was used everywhere else in this audit (boards, schedules, time-tracking) and is recommended in the recent commit `34a0bd070` ("replace $effect with onMount to resolve write-while-reading cascade issues"). This file is the same anti-pattern that was just fixed in CRM/campaigns.

---

## Medium issues (P2)

### P2-1 — Boards listing initialization uses `$effect` instead of `onMount`
[`frontend/src/routes/admin/boards/+page.svelte:101-103`](../../../frontend/src/routes/admin/boards/+page.svelte):

```ts
$effect(() => {
    if (browser) loadData();
});
```

Per recent commit `34a0bd070`, this is the exact pattern that caused write-while-reading cascades on CRM/campaigns and was systematically replaced with `onMount`. Same fix needed here.

### P2-2 — `loadData()` on time-tracking re-fetches everything on every filter change
[`frontend/src/routes/admin/boards/time-tracking/+page.svelte:265-318`](../../../frontend/src/routes/admin/boards/time-tracking/+page.svelte) — every `<select>` and date input invokes `onchange={() => loadData()}`. `loadData` also re-fetches `getBoards()` even though the boards list never changes. Trivial network waste; user sees flashes between filters.

### P2-3 — `dragOverPosition` flicker
[`frontend/src/routes/admin/boards/[id]/+page.svelte:340-349`](../../../frontend/src/routes/admin/boards/[id]/+page.svelte) — `handleDragOver` fires per-pixel, mutating `dragOverStage`/`dragOverPosition` and re-rendering the entire kanban for each event. Combined with the broken drop logic (P0-4) the board feels jittery during drag. Throttle to `requestAnimationFrame` or only update when position actually changes.

### P2-4 — Mock data and route don't agree on shape
[`api/admin/schedules/+server.ts:46`](../../../frontend/src/routes/api/admin/schedules/+server.ts) declares `interface ScheduleEvent` with `room_type: 'live' | 'recorded' | 'hybrid'` and the page declares the same — but the `[id]/+server.ts` mock array at line 24 is `any[]`. Drift waiting to happen.

### P2-5 — Consent dashboard exposes `getAuditLog()` from localStorage with no audit-log retention/PII filter
[`frontend/src/routes/admin/consent/+page.svelte:296-321`](../../../frontend/src/routes/admin/consent/+page.svelte) renders the full audit log directly from `getAuditLog()` (which reads localStorage). No pagination, no IP-redaction, no PII filter on `entry.method` (which can carry email-derived strings). For a system that claims "Proof of Consent" and "GDPR/CCPA-compliant" status, the admin viewer shouldn't ship the full record set into the DOM.

### P2-6 — Consent settings has no visibility for "test mode" footgun
[`frontend/src/routes/admin/consent/settings/+page.svelte:355-368`](../../../frontend/src/routes/admin/consent/settings/+page.svelte) — `test_mode` is a single toggle with description "Show the consent banner to logged-in admins only." This means **production users see no banner** when test mode is on. There's no banner-status indicator on admin pages so an admin who flips the toggle and walks away has effectively turned off consent collection site-wide. Should be a header-bar warning (or a hard expiry).

### P2-7 — Bulk schedule selection state isn't cleared on filter change
[`frontend/src/routes/admin/schedules/+page.svelte:113-114`](../../../frontend/src/routes/admin/schedules/+page.svelte): when `filterDay` or `filterActive` change, `selectedIds` retains schedule IDs that are no longer rendered. Bulk-delete then operates on hidden IDs the admin can't see.

### P2-8 — Drag-reorder in CMS datasources writes optimistically but rollback re-fetches
[`frontend/src/routes/admin/cms/datasources/+page.svelte:386-414`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte):

```ts
async function reorderEntries(newOrder: string[]) {
    ...
    try {
        const response = await fetch(...);
        if (!response.ok) throw new Error('Failed to reorder entries');
        // Update local state
        const orderedEntries = newOrder.map(...).filter(...);
        entries = orderedEntries;
    } catch (_err) {
        showToastMessage('Failed to reorder entries', 'error');
        await fetchEntries(); // Refresh to get correct order
    }
}
```

This is **pessimistic** (UI updates only on success). For consistency you usually want optimistic UI with on-failure rollback. Right now the user drags an item and sees no visual feedback until the server round-trip finishes; on slow links the drop appears to "fail" repeatedly until it hops to the new position. The real bug is that `handleDrop` line 598-615 doesn't update `entries` locally before the fetch, so the dragged row snaps back during the network call.

### P2-9 — Bulk-delete confirmation only references count, not which schedules
[`frontend/src/routes/admin/schedules/+page.svelte:1100-1108`](../../../frontend/src/routes/admin/schedules/+page.svelte): `Delete ${selectedIds.size} selected schedules?` — no titles, no destructive-action visual differentiation between deleting 1 vs 50. Watchlist delete on the listing page has the same shape ([`watchlist/+page.svelte:614`](../../../frontend/src/routes/admin/watchlist/+page.svelte) — uses `itemToDelete?.title`, which is better; schedule should match).

### P2-10 — `mockSchedules` is mutated across requests (memory leak)
[`api/admin/schedules/+server.ts:326`](../../../frontend/src/routes/api/admin/schedules/+server.ts): `let scheduleIdCounter = mockSchedules.length + 1;` — server-side state lives across requests in the dyno. Two admins using fallback at the same time will interleave their writes on a shared array.

---

## Low / nits (P3)

### P3-1 — `id="page-checkbox"` etc are not unique
Throughout the schedules page (and others), `id` attributes are re-used inside `{#each}` blocks (e.g. [`schedules/+page.svelte:797-803`](../../../frontend/src/routes/admin/schedules/+page.svelte) uses `id="page-checkbox"` for every event). This breaks accessibility (screen readers, label association) and DOM-test selectors.

### P3-2 — Watchlist new-item form `alert('Please select at least one room')` while edit form uses inline error
[`watchlist/+page.svelte:209`](../../../frontend/src/routes/admin/watchlist/+page.svelte) vs [`watchlist/[slug]/edit/+page.svelte:111-114`](../../../frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte). Inconsistent UX.

### P3-3 — Boards modal `<select bind:value={newBoard.folder_id}>` line 899 with `<option value={null}>` — `null` isn't a valid `<option value>` and silently becomes the empty string, breaking the type-narrowing of `folder_id?: string`.

### P3-4 — `cms/datasources` has dead-code `if (entrySearchQuery)` branch
[`frontend/src/routes/admin/cms/datasources/+page.svelte:189-191`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte): `if (entrySearchQuery) { /* Add search parameter */ }` — comment, no code. Either implement server-side search or remove.

### P3-5 — Templates page falls back to `defaultTemplates` (in-memory list of 6 templates) when API returns 0
[`boards/templates/+page.svelte:185-192`](../../../frontend/src/routes/admin/boards/templates/+page.svelte). Useful in dev, surprising in prod — admins will see hardcoded templates with no indication they aren't backend-provisioned (so editing them via the API won't persist).

### P3-6 — `consent/settings` `incrementVersion()` is one-way
[`consent/settings/+page.svelte:269-271`](../../../frontend/src/routes/admin/consent/settings/+page.svelte): version only increments. No way to undo a misclick, even before save. Add a "revert to current version" button.

### P3-7 — `+page.svelte` `style="border-left: 4px solid {board.background_color || '#E6B800'}"` (boards listing line 578) — `background_color` is user-controlled. If the backend doesn't validate, an admin could put `red; }/* css injection */` in their board color. Defense-in-depth: validate hex on save.

### P3-8 — Admin schedules navigates to `/login` on 401 but session cookie may already be expired silently
[`schedules/+page.svelte:339-342`](../../../frontend/src/routes/admin/schedules/+page.svelte): `if (response.status === 401) goto('/login'); return;` — works, but lacks the unsaved-form warning that the new-board / edit-watchlist pages have. Admin loses what they were typing.

### P3-9 — `consent/templates` is opt-out for prerender via `+page.ts` but the analytics dashboard relies on `localStorage` on every render
That's fine — but it means `loadData()` runs in `onMount` only; the page will look empty for a fraction of a second after mount. Add a skeleton loader.

### P3-10 — `boards/import/+page.svelte` `onDrop` and `onDragOver` use `role="button"` on a div that isn't a button
Accessibility annoyance only.

---

## Cross-cutting concerns

### Schedule mock/proxy architecture is the highest-risk item in this audit
Three of the four critical bugs (P0-1 / P0-2 plus the partial mock-data drift in P2-4 / P2-10) all reduce to the same architectural choice: **proxies that lie about success when the backend is unreachable.** This pattern was OK during local development but is hostile in production. The fix is small (delete the fallbacks; let SvelteKit return upstream errors) and high-impact.

### Two auth approaches in one audit (P1-7 / P3-8)
- Schedules + Boards use `/api/admin/...` proxies that read `rtp_access_token` cookie and forward as Bearer. This matches the recent fixes in commit `e2356fa46` and is the right pattern.
- CMS datasources hits the backend directly with a token from a Svelte store. Wrong pattern; needs to migrate to a proxy.

### Svelte 5 anti-pattern remediation is incomplete
Recent commits (`34a0bd070`, `05acf3231`) standardized on `onMount` for one-shot data loads. This audit found the legacy `$effect(() => { if (browser) loadData(); })` pattern still living in:
- [`boards/+page.svelte:101-103`](../../../frontend/src/routes/admin/boards/+page.svelte)
- [`watchlist/+page.svelte:76-81`](../../../frontend/src/routes/admin/watchlist/+page.svelte) (`if (!browser) return; loadWatchlistItems()`)
- [`schedules/+page.svelte:258-262`](../../../frontend/src/routes/admin/schedules/+page.svelte) (`$effect(() => { if (selectedRoomId) loadSchedules() })`)
- [`watchlist/[slug]/edit/+page.svelte:60-70`](../../../frontend/src/routes/admin/watchlist/[slug]/edit/+page.svelte)
- [`boards/reports/+page.svelte:53-57`](../../../frontend/src/routes/admin/boards/reports/+page.svelte)
- [`cms/datasources/+page.svelte:618-630`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte)

The schedules and reports cases are arguably valid (they re-run on filter change), but should use a tracked `$derived` + manual loader rather than a free-form `$effect`.

### Bulk action UX is universally weak across this audit
- Schedule bulk-delete has no per-item visibility (P2-9).
- CMS datasource delete only shows the first N entries' count (line 1198-1201 — "all 247 entries").
- Watchlist has no bulk action at all (delete is always one-at-a-time).
- Boards has no bulk action at all on the listing page.

For an admin tool of this size, every list view should surface "select all on page" + bulk archive / delete with explicit per-item failure reporting.

### No audit log for sensitive operations
- Boards storage credential changes — no audit entry.
- Consent settings changes (P2-6 — flipping `test_mode`) — no audit entry.
- Schedule bulk-delete — no audit entry.

The consent dashboard ironically has an audit log viewer but nothing else does.

### CSV import has no preview / mapping step
- [`boards/import/+page.svelte`](../../../frontend/src/routes/admin/boards/import/+page.svelte): users upload Trello/Asana/CSV blindly; first preview happens on the import-results page when it's already been written.
- [`cms/datasources/+page.svelte:448-483`](../../../frontend/src/routes/admin/cms/datasources/+page.svelte): same — file → server → result with no preview.

A standard "upload → show first 20 rows → confirm column mapping → import" flow would prevent a lot of imports-of-the-wrong-spreadsheet incidents.

### Type safety is weak in the schedule API surface
`exceptions: any[]` at [`api/admin/schedules/+server.ts:37`](../../../frontend/src/routes/api/admin/schedules/+server.ts) and `mockSchedules: any[]` at [`[id]/+server.ts:24`](../../../frontend/src/routes/api/admin/schedules/[id]/+server.ts). Tighten to the same `ScheduleEvent` type used in [`+server.ts:24-40`](../../../frontend/src/routes/api/admin/schedules/+server.ts).

---

## Summary

This audit covers ~15 admin pages and 7 SvelteKit `+server.ts` proxies across boards, schedules, watchlist, CMS datasources, and consent.

**Severity counts:**
- P0 (Critical): **4** — all reachable in production today
- P1 (High): **8** — degraded UX, security-adjacent, missing rollback
- P2 (Medium): **10** — reactivity hygiene, bulk-action visibility, mock drift
- P3 (Low/nit): **10** — accessibility, dead code, defense-in-depth

The single highest-leverage fix is deleting the mock-fallback layer from the four schedule proxies (P0-1 / P0-2). Second priority is **stop returning storage `secret_key` to the browser** (P0-3) — this is a credential leak. Third is the boards drag-drop reorder (P0-4) which has been quietly desyncing. After those three, the recurring pattern across P1/P2 is **incomplete migration off `$effect`-for-data-loading**, which the team is already mid-fixing in CRM/campaigns; the remaining instances are listed in the cross-cutting section.

Two areas had no critical or high-severity bugs and are in good shape:
- **Consent templates** — clean store-driven design, proper preview/apply pattern.
- **Boards reports** — read-only, no mutation, no bulk action surface.

Everything else needs at least one P1 fix before it's safe to call "shipped."
