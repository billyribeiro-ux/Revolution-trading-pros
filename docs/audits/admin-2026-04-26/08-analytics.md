# 08 — Analytics / Behavior / Performance / Dashboard Audit

_Date:_ 2026-04-26
_Scope:_ `frontend/src/routes/admin/analytics/`, `frontend/src/routes/admin/behavior/`, `frontend/src/routes/admin/performance/`, `frontend/src/routes/admin/dashboard/`, `frontend/src/routes/api/admin/analytics/`
_Method:_ Read-only review of every file in the four admin folders plus the single `+server.ts` proxy.

---

## Files reviewed

Frontend pages:
- [`frontend/src/routes/admin/analytics/+layout.svelte`](../../../frontend/src/routes/admin/analytics/+layout.svelte)
- [`frontend/src/routes/admin/analytics/+page.svelte`](../../../frontend/src/routes/admin/analytics/+page.svelte) — Dashboard tab
- [`frontend/src/routes/admin/analytics/recordings/+page.svelte`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte)
- [`frontend/src/routes/admin/analytics/goals/+page.svelte`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte)
- [`frontend/src/routes/admin/analytics/cohorts/+page.svelte`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte)
- [`frontend/src/routes/admin/analytics/attribution/+page.svelte`](../../../frontend/src/routes/admin/analytics/attribution/+page.svelte)
- [`frontend/src/routes/admin/analytics/heatmaps/+page.svelte`](../../../frontend/src/routes/admin/analytics/heatmaps/+page.svelte)
- [`frontend/src/routes/admin/analytics/funnels/+page.svelte`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte)
- [`frontend/src/routes/admin/analytics/events/+page.svelte`](../../../frontend/src/routes/admin/analytics/events/+page.svelte)
- [`frontend/src/routes/admin/analytics/segments/+page.svelte`](../../../frontend/src/routes/admin/analytics/segments/+page.svelte)
- [`frontend/src/routes/admin/analytics/reports/+page.svelte`](../../../frontend/src/routes/admin/analytics/reports/+page.svelte)
- [`frontend/src/routes/admin/behavior/+page.svelte`](../../../frontend/src/routes/admin/behavior/+page.svelte)
- [`frontend/src/routes/admin/performance/+page.svelte`](../../../frontend/src/routes/admin/performance/+page.svelte)
- [`frontend/src/routes/admin/dashboard/+page.svelte`](../../../frontend/src/routes/admin/dashboard/+page.svelte)

API proxies:
- [`frontend/src/routes/api/admin/analytics/dashboard/+server.ts`](../../../frontend/src/routes/api/admin/analytics/dashboard/+server.ts) (only proxy that exists in scope)

Supporting files referenced:
- [`frontend/src/lib/api/analytics.ts`](../../../frontend/src/lib/api/analytics.ts) — same-origin client used by every page
- [`frontend/src/lib/stores/connections.svelte.ts:548`](../../../frontend/src/lib/stores/connections.svelte.ts) — `getIsAnalyticsConnected` / `getIsBehaviorConnected`
- [`frontend/src/lib/api/config.ts:11`](../../../frontend/src/lib/api/config.ts) — hardcoded `PRODUCTION_API_URL`

---

## Critical bugs (P0)

### P0-1 — `{:else if !getIsAnalyticsConnected}` is checking a function reference, not its result. The "Not Connected" branch is **dead code** in 7 pages.

`getIsAnalyticsConnected` is a function; `!getIsAnalyticsConnected` evaluates `!function` which is always `false`, so the conditional never enters that branch. The not-connected `<ServiceConnectionStatus />` card never renders, and control falls through to the connected layout where API calls fail silently.

Affected files (cite `file:line`):
- [`recordings/+page.svelte:168`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`goals/+page.svelte:214`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`cohorts/+page.svelte:200`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`heatmaps/+page.svelte:156`](../../../frontend/src/routes/admin/analytics/heatmaps/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`funnels/+page.svelte:165`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`events/+page.svelte:143`](../../../frontend/src/routes/admin/analytics/events/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`segments/+page.svelte:228`](../../../frontend/src/routes/admin/analytics/segments/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`reports/+page.svelte:213`](../../../frontend/src/routes/admin/analytics/reports/+page.svelte) — `{:else if !getIsAnalyticsConnected}`
- [`behavior/+page.svelte:137`](../../../frontend/src/routes/admin/behavior/+page.svelte) — `{:else if !getIsBehaviorConnected}`

Fix: parenthesise — `{:else if !getIsAnalyticsConnected()}`. Note that the same files use `{#if getIsAnalyticsConnected()}` correctly elsewhere on the page (e.g. funnels:142, recordings:153) — the `!`-prefixed form was dropped by a pattern-replace that missed the parens.

### P0-2 — Massive orphan endpoint surface. ~25 client routes call backend proxies that do not exist.

[`frontend/src/lib/api/analytics.ts`](../../../frontend/src/lib/api/analytics.ts) (`request<T>` at line 344) prefixes every endpoint with `/api/`, so each call hits a same-origin SvelteKit `+server.ts` proxy. Of the 25+ endpoints called, only one proxy exists:

- ✅ `/api/admin/analytics/dashboard/+server.ts`
- ❌ `/api/admin/analytics/kpis` ([`analytics.ts:426`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/funnels` and `/funnels/{key}/...` ([`analytics.ts:445,449,459,627,639`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/cohorts` and `/cohorts/{key}/...` ([`analytics.ts:473,482,488,665,676`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/segments` (CRUD) ([`analytics.ts:544,548,695,706,710`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/events`, `/events/timeseries`, `/events/breakdown` ([`analytics.ts:560,571,605`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/realtime` ([`analytics.ts:582`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/attribution/{channels,campaigns,paths,compare}` ([`analytics.ts:499,509,513,721`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/forecast/{key}` ([`analytics.ts:526,534`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/reports` (CRUD + run) ([`analytics.ts:732,736,752,765,769,773`](../../../frontend/src/lib/api/analytics.ts))
- ❌ `/api/admin/analytics/recordings` ([`recordings/+page.svelte:50`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte))
- ❌ `/api/admin/analytics/heatmaps` and `/heatmaps/{url}` ([`heatmaps/+page.svelte:42,61`](../../../frontend/src/routes/admin/analytics/heatmaps/+page.svelte))
- ❌ `/api/admin/analytics/goals` (CRUD) ([`goals/+page.svelte:86,110`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte))

Every one of these returns SvelteKit's default 404 HTML, parsed by `request<T>` ([`analytics.ts:373-376`](../../../frontend/src/lib/api/analytics.ts)) into the catchall `error.message || \`HTTP ${response.status}\`` thrown to the page, where it appears as a generic "Failed to load X" toast. Pages with explicit try/catch (recordings, heatmaps, goals) silently swallow into empty arrays.

Fix: either build the missing `+server.ts` proxies (CLAUDE.md "CREATE, never DELETE" rule from `feedback_create_not_delete.md`) or, faster, change `analytics.ts:351` to call the Rust API directly through the existing `apiClient` infrastructure — but that introduces CORB risk that this layer was specifically built to avoid.

### P0-3 — Recordings page has no PII redaction, no consent check, no auth gate on the (orphan) endpoint, and no rrweb-style scrubbing.

[`recordings/+page.svelte`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte) is described in its own header as "Watch how users interact with your site through recorded session playback." but:

- The fetch at line 49-51 sends no auth header and no period validation.
- The `Recording` interface at line 17-31 includes `user_email` raw — leaking PII into the admin UI without role check or masking.
- The "Player Area" at line 437-448 shows a placeholder; there is no integration with rrweb / FullStory / Hotjar, so no input-scrubbing has been built. **Any future integration that drops a player here will replay raw form inputs (passwords, payment fields) unless redaction is added first.**
- Line 305-307 displays `user_email` directly in the recordings table — same PII leak.

This is a P0 because the moment a real session-replay library lands, it will instantly start exposing user PII to anyone with admin access. The audit recommendation is: build a redaction-by-default policy now, before the player exists.

### P0-4 — Hardcoded "Operational / Online / Connected / Active / Running" placeholder text in the System Status panel of the admin dashboard.

[`dashboard/+page.svelte:466-531`](../../../frontend/src/routes/admin/dashboard/+page.svelte) shows the System Status pill as `Operational` and four sub-rows (API Server / Database / Cache / Job Queue) all hardcoded to `Online / Connected / Active / Running`. There is no `/api/admin/system/health` call anywhere in the file; these are unconditional text. The page header [`dashboard/+page.svelte:8-9`](../../../frontend/src/routes/admin/dashboard/+page.svelte) explicitly promises "NO FAKE DATA - Everything reflects actual connection status!" — this is a direct contradiction.

When the database is down or the job queue is wedged, this dashboard will report green. That is dangerous in an admin-facing tool.

---

## High-severity issues (P1)

### P1-1 — `$effect`-based one-shot init is the project's deprecated cascade pattern.

Eight files re-introduce the `$effect(() => { … connections.load() … })` pattern that was explicitly migrated **away from** in commit `34a0bd070` ("replace $effect with onMount to resolve write-while-reading cascade issues"). The fixed pattern lives in [`analytics/+page.svelte:140-163`](../../../frontend/src/routes/admin/analytics/+page.svelte) (`onMount(() => { … })`) and the comment there explains why:

> The previous `$effect` read `getIsAnalyticsConnected()` (which reads the `connectionsState` rune) and then synchronously called `connections.load()` which mutates that same rune. That's the classic write-while-reading-tracked-dep pattern that produces an `effect_update_depth_exceeded` cascade on /admin login.

Files still using the dangerous pattern:
- [`recordings/+page.svelte:107-119`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte)
- [`goals/+page.svelte:140-152`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte)
- [`cohorts/+page.svelte:111-123`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte)
- [`heatmaps/+page.svelte:80-92`](../../../frontend/src/routes/admin/analytics/heatmaps/+page.svelte)
- [`funnels/+page.svelte:107-119`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte)
- [`events/+page.svelte:96-108`](../../../frontend/src/routes/admin/analytics/events/+page.svelte)
- [`segments/+page.svelte:141-153`](../../../frontend/src/routes/admin/analytics/segments/+page.svelte)
- [`reports/+page.svelte:144-160`](../../../frontend/src/routes/admin/analytics/reports/+page.svelte)
- [`behavior/+page.svelte:78-94`](../../../frontend/src/routes/admin/behavior/+page.svelte)
- [`attribution/+page.svelte:59-61`](../../../frontend/src/routes/admin/analytics/attribution/+page.svelte) (`$effect(() => { if (browser) loadAttribution(); });`)
- [`performance/+page.svelte:33-35`](../../../frontend/src/routes/admin/performance/+page.svelte) (`$effect(() => { if (browser) loadData(); });`)
- [`dashboard/+page.svelte:184-194`](../../../frontend/src/routes/admin/dashboard/+page.svelte) (sets up 60s polling — see P1-2)

Fix: switch to `onMount(() => { /* init */; return () => { /* cleanup */ }; })` per the analytics dashboard precedent.

### P1-2 — Dashboard polling: `setInterval(loadDashboard, 60000)` runs indefinitely if `$effect` re-runs before unmount.

[`dashboard/+page.svelte:184-194`](../../../frontend/src/routes/admin/dashboard/+page.svelte):

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

The cleanup correctly clears `refreshInterval` on `$effect` re-run/destroy, but `loadDashboard()` ([`:154-158`](../../../frontend/src/routes/admin/dashboard/+page.svelte)) writes to `connections`, `metrics`, `roomStats`, `lastUpdated` — none of which are read inside the `$effect` body, so today the effect doesn't re-trigger. This is a fragile invariant: any future `$state` read added to this effect block will turn the polling into a self-amplifying loop. Use `onMount` per the project pattern.

### P1-3 — `connectionsState` reactivity is broken for connection status checks because helpers wrap reads in `untrack()`.

[`connections.svelte.ts:548-555`](../../../frontend/src/lib/stores/connections.svelte.ts):

```ts
export function getIsAnalyticsConnected(): boolean {
    return untrack(() => {
        const analyticsServices = FEATURE_SERVICES['analytics'];
        return (
            analyticsServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false
        );
    });
}
```

The store's own block comment (`:540-543`) acknowledges:

> `$derived(getXConnected())` continues to work: $derived installs tracking at the proxy boundary, before `untrack` runs inside the helper body.

But every page in scope calls the helper **bare** in templates (`{#if getIsAnalyticsConnected()}` at recordings:153, goals:191, cohorts:168, heatmaps:126, funnels:142, events template gating, segments:184, reports:193, behavior:114). These calls happen during template rendering, but because they're wrapped in `untrack`, **changes to `connectionsState` after mount do not re-render these gates**.

Today, this rarely surfaces because `connections.load()` runs once before the gate is first read, and admins don't toggle connections from another tab. But if a user disconnects a service mid-session, the analytics page won't update.

Fix: introduce a `$derived(getIsAnalyticsConnected())` per page or remove `untrack` from the helpers. The store comment recommends the former.

### P1-4 — `recordings`, `heatmaps`, `goals` swallow errors into empty arrays, masking the orphan-endpoint bug.

E.g. [`recordings/+page.svelte:59-64`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte):

```ts
} catch (_e) {
    // For now, set empty array since API might not exist yet
    recordings = [];
}
```

Same pattern at [`heatmaps/+page.svelte:49-52`](../../../frontend/src/routes/admin/analytics/heatmaps/+page.svelte) and [`goals/+page.svelte:94-97`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte). The `_e` rename signals "I know this is broken", but the comment "API might not exist yet" has clearly aged into a permanent silent failure. End users see "No data yet" forever.

### P1-5 — Behavior page renders fake hardcoded trend percentages alongside (potentially) real metric values.

[`behavior/+page.svelte:166-241`](../../../frontend/src/routes/admin/behavior/+page.svelte) shows six metric cards. Each has a `<div class="metric-trend positive"><span>12.5%</span></div>` — those numbers (`12.5`, `8.3`, `5.2`, `3.1`, `1.2`, `15.7`) are **literal strings in the template**, never bound to any state. The metric values above them come from the API; the trend pills are decoration. This is the explicit "no fake numbers" violation called out in the analytics dashboard header.

### P1-6 — Performance page wraps every Core Web Vital in `{#if true}` blocks.

[`performance/+page.svelte:172, 188, 204, 220`](../../../frontend/src/routes/admin/performance/+page.svelte) all use `{#if true}` to scope the `{@const lcpRating = …}` declaration:

```svelte
{#if true}
    {@const lcpRating = getVitalRating(coreWebVitals?.lcp_rating)}
    <div class="vital-card">…</div>
{/if}
```

This is dead-syntax — Svelte 5 supports `{@const}` directly inside `{#snippet}` and inside loop iterations. The pattern works but smells of someone who copy-pasted from Stack Overflow. Refactor by lifting the `getVitalRating` calls into `$derived` declarations.

### P1-7 — `analytics/+layout.svelte` nav is missing 3 of the 10 sibling pages.

[`+layout.svelte:15-23`](../../../frontend/src/routes/admin/analytics/+layout.svelte) lists Dashboard, Events, Funnels, Cohorts, Segments, Attribution, Reports — but `goals`, `heatmaps`, `recordings` exist as pages and are not surfaced. Users land on those URLs only via the "Quick Actions" panel on the dashboard or via deep links from elsewhere.

### P1-8 — Period query param flows to backend without validation/whitelist.

The same pattern recurs everywhere: `period = url.searchParams.get('period') || '30d'` then forwarded to `/api/admin/analytics/...?period=${period}`. The Rust backend should validate, but if it doesn't, a malformed `period` will at minimum break SQL and at worst trigger an error 500. Cite [`api/admin/analytics/dashboard/+server.ts:78,88`](../../../frontend/src/routes/api/admin/analytics/dashboard/+server.ts).

Fix: whitelist `['1d', '7d', '30d', '90d', '1y']` at the proxy edge.

---

## Medium issues (P2)

### P2-1 — Duplicate DOM IDs inside loops.

These IDs are emitted once per loop iteration inside an `{#each}`:
- `id="page-step-name"` and `id="page-step-event-name"` — [`funnels/+page.svelte:422,430`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte) (one per funnel step)
- `id="page-rule-value"` — [`segments/+page.svelte:564`](../../../frontend/src/routes/admin/analytics/segments/+page.svelte) (one per rule)

HTML requires `id` to be unique per document. Browsers will only honor the first match for label-clicks and `getElementById`. Solution: drop the `id` (the wrapping `<label>` is not present anyway, so the `id` is unused) or template it with the loop index.

### P2-2 — `selectedRecording` modal is rendered but the player is a placeholder; no `onClose` cleanup.

[`recordings/+page.svelte:410-471`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte) — the modal is dismissable only via the X button, not Escape, not backdrop-click, no focus-trap, no `inert` on the page beneath. When this becomes a real player it must `pause()` on close — there is no `onDestroy` hook in the component. Add a TODO marker.

### P2-3 — Selected `confirm()`/`alert()` for destructive actions.

- [`goals/+page.svelte:128`](../../../frontend/src/routes/admin/analytics/goals/+page.svelte) — `alert(e instanceof Error ? e.message : 'Failed to create goal')`
- [`cohorts/+page.svelte:106`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte) — same
- [`funnels/+page.svelte:95`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte) — same
- [`segments/+page.svelte:114`](../../../frontend/src/routes/admin/analytics/segments/+page.svelte) — same
- [`reports/+page.svelte:115`](../../../frontend/src/routes/admin/analytics/reports/+page.svelte) — same

`segments/+page.svelte` already imports `ConfirmationModal` and uses `toastStore.error` is available elsewhere — the inconsistency is the issue. Replace each `alert()` with the toast system.

### P2-4 — `analyticsApi.getCohorts()` typed-as-`any` cast erases compile-time safety.

[`cohorts/+page.svelte:65`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte) — `cohorts = (response.cohorts || []) as any;`. The local `Cohort` interface has fields (`key`, `granularity`, `summary`) that the API may not return. The `as any` papers over the mismatch and any backend rename will silently break the UI.

### P2-5 — Date formatting hardcoded to en-US locale, browser-local timezone.

Every `formatDate`/`formatTime`/`new Date(dateStr).toLocaleString('en-US', …)` (e.g. [`recordings/+page.svelte:85-93`](../../../frontend/src/routes/admin/analytics/recordings/+page.svelte), [`events/+page.svelte:80-88`](../../../frontend/src/routes/admin/analytics/events/+page.svelte), [`reports/+page.svelte:135-141`](../../../frontend/src/routes/admin/analytics/reports/+page.svelte)) parses an ISO string in the browser's local TZ. If the backend emits `2026-04-26T00:00:00Z` and the admin is in PST, the display will read as `Apr 25` — classic off-by-one. There's no UTC normalization or label.

### P2-6 — Period selector doesn't reset `selectedFunnel`/`selectedCohort` when the period changes.

In [`funnels/+page.svelte:65-68`](../../../frontend/src/routes/admin/analytics/funnels/+page.svelte) and [`cohorts/+page.svelte:76-79`](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte), `handlePeriodChange` calls `loadX()` but leaves `selectedFunnel`/`selectedCohort` pointing at a stale funnel object that may no longer be present (or have different data) in the new period. The `selectedFunnel?.key === funnel.key` test at funnels:220 then matches the wrong object.

### P2-7 — Dashboard `/api/admin/connections/summary` mutation pattern is unsafe.

[`dashboard/+page.svelte:80-86`](../../../frontend/src/routes/admin/dashboard/+page.svelte):

```ts
if (data.connections) {
    for (const conn of data.connections) {
        if (conn.is_connected) {
            (connections as any)[conn.key] = true;
        }
    }
}
```

`(connections as any)[conn.key] = true` writes whatever string the backend sends as a key into the `connections` rune, growing the proxy with arbitrary fields. Any typo or new service in the backend will silently inflate the rune. Use a switch on known `SERVICE_KEYS` and warn on unknown.

### P2-8 — `attribution/+page.svelte` doesn't gate on `getIsAnalyticsConnected()`.

Unlike the other 8 pages, [`attribution/+page.svelte`](../../../frontend/src/routes/admin/analytics/attribution/+page.svelte) skips the connection-check entirely. It loads on any visit (line 59-61). This is correct behavior **only if** the backend at `/api/admin/analytics/attribution/channels` returns a sensible empty state — but per P0-2 that endpoint is an orphan, so the page lands directly in the error state.

### P2-9 — `dashboard/+server.ts` uses a string-concat fetch URL with no period validation.

[`api/admin/analytics/dashboard/+server.ts:88`](../../../frontend/src/routes/api/admin/analytics/dashboard/+server.ts):

```ts
const backendData = await fetchFromBackend(`/api/admin/analytics/dashboard?period=${period}`, …);
```

`period` flows from `url.searchParams.get('period')` unsanitized. `period = "30d&secret_admin_param=true"` would be appended verbatim. Use `URLSearchParams` per [config.ts:354-360 pattern](../../../frontend/src/lib/api/config.ts).

### P2-10 — Behavior page directly calls Rust API at `revolution-trading-pros-api.fly.dev` instead of going through a proxy.

[`behavior/+page.svelte:48`](../../../frontend/src/routes/admin/behavior/+page.svelte): `await api.get(\`/api/admin/analytics/behavior?period=${selectedPeriod}\`)`. The `api` object from [`config.ts:293`](../../../frontend/src/lib/api/config.ts) prepends `API_BASE_URL` which is hardcoded to `https://revolution-trading-pros-api.fly.dev`. This bypasses the SvelteKit proxy layer that every other page uses, exposing the admin panel to CORS / CORB risk and making local-dev backend swaps impossible.

---

## Low / nits (P3)

### P3-1 — Emoji-as-icon literals.

The analytics layout ([`+layout.svelte:16-22`](../../../frontend/src/routes/admin/analytics/+layout.svelte)) and several other places use emoji string literals (`'📊'`, `'🔻'`) inside the navigation. The repo migrated to Tabler icons in commit `3a59cd374`. These hold-outs should match.

### P3-2 — `dashboard-grid` uses `position: absolute` panels for tab switching with `inert={...}`.

[`analytics/+page.svelte:296-475`](../../../frontend/src/routes/admin/analytics/+page.svelte) — clever layout-shift-prevention but creates a full 4× cost: every tab's chart components mount at once, including `RealTimeWidget`, `FunnelChart`, `CohortMatrix`, `AttributionChart`. The CSS `.tab-panel { contain: content }` doesn't prevent JS work. If any chart sets up timers, all four run simultaneously.

### P3-3 — `RealTimeWidget` is rendered inside the Overview tab — verify it tears down its polling on tab change.

(Outside scope — but flag for whoever owns that component. The audit can't see its source from here.)

### P3-4 — Dashboard uses `adminFetch` which is a single network helper, but the four endpoints called there (`connections/summary`, `analytics/realtime`, `payments/summary`, `rooms/stats`) are loaded sequentially — three of them in parallel via `Promise.all`, but they could all four be parallelized.

[`dashboard/+page.svelte:154-158`](../../../frontend/src/routes/admin/dashboard/+page.svelte): `Promise.all([fetchConnectionStatus(), fetchMetrics(), fetchRoomStats()])`. `fetchMetrics` itself awaits two unrelated endpoints serially ([`:96-112`](../../../frontend/src/routes/admin/dashboard/+page.svelte)). Parallelize those two analytics+payments calls.

### P3-5 — `formatProperties()` calls `JSON.stringify(props)` twice for length check.

[`events/+page.svelte:90-93`](../../../frontend/src/routes/admin/analytics/events/+page.svelte): `JSON.stringify(props).slice(0, 100) + (JSON.stringify(props).length > 100 ? '...' : '');`. Stringify once, store in a const.

### P3-6 — Three of the empty-state CTAs link to `/admin/analytics/funnels/create`, `/admin/analytics/cohorts/create` — those routes don't exist.

[`analytics/+page.svelte:412, 462`](../../../frontend/src/routes/admin/analytics/+page.svelte) — links to `/admin/analytics/funnels/create` and `/admin/analytics/cohorts/create`. Neither folder has a `create/` subroute — clicking 404s. The actual create flow is the modal in `funnels/+page.svelte:351` and `cohorts/+page.svelte:352`. Either build the create routes or change the links to open the modal via a query param.

### P3-7 — `analytics.ts:351` `endpoint.startsWith('/api/')` check.

```ts
const apiEndpoint = endpoint.startsWith('/api/') ? endpoint : `/api${endpoint}`;
```

Every call site passes `/admin/analytics/...` (no leading `/api`), so the prefix is always added. The branch is dead. Minor, but a hint that the abstraction is unused.

### P3-8 — `dashboard/+page.svelte` polls every 60s but the user has no UI to pause it.

Could double-tap into a "Live" toggle. Not urgent.

---

## Cross-cutting concerns

### Chart libs / rendering

None of the audited pages directly import a chart library — every chart is delegated to `KpiGrid`, `FunnelChart`, `CohortMatrix`, `TimeSeriesChart`, `AttributionChart`, `RealTimeWidget`. Those components are out of scope for this audit but should be reviewed separately for:
1. `onDestroy` teardown of any timers / `requestAnimationFrame` / `IntersectionObserver`.
2. ResizeObserver leaks if chart canvases re-mount on tab switches.
3. The Overview-tab grid renders 4 panels at once (P3-2) — if any chart has a mount-time cost, that cost is paid 4× even though only one panel is visible.

### Polling discipline

Dashboard is the only audited page with active polling. It uses `setInterval(loadDashboard, 60000)` and clears in `$effect` cleanup. Per P1-2 this should move to `onMount`. No other page polls. `RealTimeWidget` is presumably real-time — flag for that component's own audit.

### Date / timezone handling

Every `Date` parse uses browser-local TZ implicitly. Backend timestamps need explicit `Z` or `+00:00`, and the UI formatter should specify `timeZone: 'UTC'` (or expose a TZ selector). Currently the admin sees dates skewed by their local offset. P2-5 covers this.

### Auth

The single proxy ([`api/admin/analytics/dashboard/+server.ts`](../../../frontend/src/routes/api/admin/analytics/dashboard/+server.ts)) follows the project pattern correctly: prefer `rtp_access_token` cookie, fall back to header, 401 if missing — exactly matches commit `e2356fa46`. Endpoint URL is read from `env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'` per CLAUDE.md. **No URL hardcoding here.** ✅

Pages talking directly to the Rust API (e.g. `behavior/+page.svelte`, P2-10) bypass this auth layer entirely and rely on the `api.get` helper attaching tokens — needs separate review.

### PII / privacy

P0-3 documents the recordings PII risk. Beyond that:
- `events/+page.svelte:307-313` displays `event.user_id` raw. If `user_id` is a PII field (email, phone) rather than an opaque hash, this leaks.
- `events/+page.svelte:317-319` displays raw `event.properties` as JSON — these objects often contain form values, query strings, etc. No allowlist / denylist on what's shown.

---

## Summary

The analytics admin surface looks polished but is largely non-functional: only 1 of ~25 client-facing endpoints has a backing proxy (P0-2), and the connection-status gate that's supposed to show "Not Connected" is broken in 9 pages because of a missing `()` (P0-1). The dashboard reports system health as universally green using hardcoded text (P0-4). The session-recording surface ships with no PII redaction in advance of integrating a player (P0-3).

Underneath, 12 pages re-introduced the `$effect` cascade pattern that the project explicitly migrated away from (P1-1), and connection-status reactivity is silently broken because helpers wrap reads in `untrack()` (P1-3). Performance, behavior, and dashboard pages use placeholder fake numbers and `{#if true}` artifacts.

**Recommended order:**
1. Fix P0-1 with a global find-replace (`!getIsAnalyticsConnected` → `!getIsAnalyticsConnected()`).
2. Triage P0-2: either delete the unimplemented pages (per `feedback_create_not_delete.md`, build the missing proxies instead) or migrate `analytics.ts` to the direct API client. The "build the missing side" rule applies — these are orphans on the proxy side, so build the proxies.
3. Address P0-3 with a pre-integration PII redaction policy doc before any session-replay library ships.
4. Replace P0-4's hardcoded health status with a real `/api/admin/system/health` call.
5. Run the P1-1 `$effect` → `onMount` migration; keep P1-3 (`untrack`) in scope so the analytics dashboard pattern is followed everywhere.
