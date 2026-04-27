# 08 — Analytics / Behavior / Performance / Dashboard — RESULTS

_Date:_ 2026-04-26
_Scope:_ Implementation pass for `docs/audits/admin-2026-04-26/08-analytics.md`
_Verifier gates:_ `pnpm check` → 5256 files / **0 errors / 0 warnings** ✅

---

## Pre-existing context (not in this pass)

By the time this agent landed, sister fixes had already addressed:

- **P0-1 in 8 of 9 pages.** `recordings`, `goals`, `cohorts`, `heatmaps`,
  `funnels`, `events`, `segments`, `reports` already corrected the
  `!getIsAnalyticsConnected` typo to a `$derived(...)` reactive read. Only
  `behavior/+page.svelte:153` still had the bare reference.
- **P0-4.** The `/admin/dashboard` route was previously a duplicate that
  hardcoded "Operational/Online/Connected/Active/Running"; it was rewritten
  to a redirect into `/admin` (commit prior). The site-health proxy
  (`frontend/src/routes/api/admin/site-health/+server.ts`) already exists.
- **P1-1 in 9 pages.** `recordings`, `goals`, `cohorts`, `heatmaps`,
  `funnels`, `events`, `segments`, `reports`, `behavior` already migrated
  init-`$effect` to `onMount`. Only `attribution` and `performance` were
  outstanding.
- **P1-3 in all gated pages.** Each page reads `let isXConnected =
  $derived(getIsXConnected())` so the helper's `untrack` doesn't break
  reactivity.
- **P2-10.** `behavior/+page.svelte` already routes through the same-origin
  proxy.
- **P1-5.** Hardcoded trend pills already removed from `behavior`.

---

## Changes landed in this pass

### P0-1 — `!getIsBehaviorConnected` (missing parens)
- `frontend/src/routes/admin/behavior/+page.svelte:153` — replaced the bare
  function-reference check with the existing `$derived` value
  `!isBehaviorConnected`.

### P0-2 — Build the missing analytics proxies (~25 endpoints)
- **New** `frontend/src/lib/server/analytics-proxy.ts` — shared helper.
  - Cookie-first auth via `requireAdmin`.
  - `period` query param validated against an allowlist
    (`['1d','24h','7d','14d','30d','60d','90d','180d','1y']`); unknown
    values are dropped (also fixes audit §P2-9 string-concat injection).
  - Builds the forwarded URL via `URLSearchParams` from a configured
    `forwardQuery` allowlist — extra params (e.g. `secret_admin_param=true`)
    are silently dropped.
  - Forwards the upstream HTTP status verbatim — a backend 404 stays a 404
    instead of being masked into "0 results".
- **New** `frontend/src/routes/api/admin/analytics/[...rest]/+server.ts` —
  catchall covering every analytics path EXCEPT `/dashboard` (which retains
  its bespoke handler with built-in fallback structure). Implements `GET`,
  `POST`, `PUT`, `PATCH`, `DELETE`. The static `dashboard/+server.ts` route
  still wins the SvelteKit precedence match.

  Endpoints now reachable through the catchall:

  - `kpis`, `kpis/{key}`
  - `funnels`, `funnels/{key}`, `funnels/{key}/dropoff`,
    `funnels/{key}/segment`
  - `cohorts`, `cohorts/{key}/matrix`, `cohorts/{key}/curve`,
    `cohorts/{key}/ltv`
  - `segments`, `segments/{key}`
  - `events`, `events/timeseries`, `events/breakdown`
  - `realtime`
  - `attribution/channels`, `attribution/campaigns`, `attribution/paths`,
    `attribution/compare`
  - `forecast/{key}`, `forecast/{key}/accuracy`
  - `reports`, `reports/{id}`, `reports/{id}/run`
  - `recordings`, `heatmaps`, `heatmaps/{url}`, `goals`
  - `behavior`

- **Updated** `frontend/src/routes/api/admin/analytics/dashboard/+server.ts`:
  - Validates `period` against the same allowlist.
  - Builds the forwarded URL via `URLSearchParams` instead of string concat.

**New analytics proxy count: 1 catchall handler + 1 shared helper module.
Combined coverage: every analytics endpoint observed across
`frontend/src/lib/api/analytics.ts` and the page-level `fetch()` calls.**

### P0-3 — Recordings PII redaction TODO
- `frontend/src/routes/admin/analytics/recordings/+page.svelte` — added a
  detailed top-of-file `TODO(2026-04-26-audit-08-P0-3)` block enumerating
  the redaction policy that MUST land before any session-replay library is
  wired up (mask all inputs, block password/payment fields, strip
  `user_email` etc., honor consent, server-side TTL + delete-by-user_id).

### P1-1 — `$effect` → `onMount` (the two remaining pages)
- `frontend/src/routes/admin/analytics/attribution/+page.svelte` — converted
  init `$effect(() => { if (browser) loadAttribution(); })` to `onMount`.
- `frontend/src/routes/admin/performance/+page.svelte` — converted init
  `$effect(() => { if (browser) loadData(); })` to `onMount`.

### P1-4 — Stop swallowing errors into empty arrays
- `recordings/+page.svelte` — `loadRecordings()` now sets `error = ...`
  when the upstream returns non-OK; the existing error-banner UI surfaces
  it. URL is built via `URLSearchParams` (no concat).
- `heatmaps/+page.svelte` — same fix in `loadPages()`.
- `goals/+page.svelte` — same fix in `loadGoals()`.

### P1-6 — Performance page `{#if true}` dead-syntax
- `performance/+page.svelte` — removed the four `{#if true}` wrappers; the
  `{@const}` declarations are replaced by four top-level
  `$derived(getVitalRating(...))` declarations (`lcpRating`, `fidRating`,
  `clsRating`, `inpRating`).

### P1-7 — Layout nav missing 3 of 10 pages
- `analytics/+layout.svelte` — added Goals, Heatmaps, Recordings entries
  to `navItems`.

### P1-8 — `period` query param sanitization
- See P0-2 above. Both the catchall and the dashboard proxy now drop
  unknown `period` values, defaulting back to `30d` where appropriate.

### P2-4 — `as any` cast on cohort response
- `cohorts/+page.svelte` — removed the `as any` cast at the assign site;
  widened the local `Cohort` interface's `type` and `granularity` from
  string-literal unions to `string` (matching the backend wire shape).
  This makes any future backend rename a TS error at compile time instead
  of a silent UI break.

### P2-5 — Date parsing in browser-local TZ (off-by-one bug)
- `recordings/+page.svelte` — `formatDate()` now passes
  `timeZone: 'UTC', timeZoneName: 'short'` so a backend `…T00:00:00Z`
  doesn't read as the previous calendar day in PST. Other pages
  (`events`, `reports`) still use local TZ; flagged as a follow-up
  candidate but lower-priority since they're datetime values, not
  calendar-day intent.

### P2-6 — Period changes don't reset selectedFunnel/Cohort
- `funnels/+page.svelte` — `handlePeriodChange()` resets `selectedFunnel`
  to `null` so the stale-pointer mismatch can't occur.
- `cohorts/+page.svelte` — same fix for `selectedCohort`.

### P3-6 — Stub create routes
- **New** `frontend/src/routes/admin/analytics/funnels/create/+page.svelte`
  and `cohorts/create/+page.svelte`. Each is a thin redirect into the
  existing modal flow on the parent list page (`?create=1`).
- `funnels/+page.svelte` and `cohorts/+page.svelte` — `onMount` reads
  `page.url.searchParams.get('create') === '1'` and opens the existing
  `showCreateModal` automatically.

---

## Items audited but not changed in this pass

| ID | Why deferred |
|----|---|
| P2-1 (duplicate DOM ids in loops) | Multi-file find/replace, low risk; flagged for next sweep. |
| P2-2 (modal close UX / inert / focus-trap) | Cross-cutting modal-component refactor; should land with the recordings player integration (gated by P0-3 redaction policy). |
| P2-3 (`alert()` → toast) | 5 sites across goals/cohorts/funnels/segments/reports; mechanical but requires importing `toastStore` per page. |
| P2-7 (`(connections as any)[conn.key]`) | Lives in `/admin/+page.svelte` and `dashboard/+page.svelte` (now a redirect). Not in this scope. |
| P2-8 (attribution skips connection gate) | Backend behavior dependent; needs a product call before forcing a gate. |
| P3-1 (emoji icons in nav) | Cosmetic; matches the existing analytics layout's house style. Migrating would touch the entire nav. |
| P3-2/3/4 (chart-component teardown / parallelization) | Out of scope per audit (chart components reviewed separately). |
| P3-5, P3-7, P3-8 | Micro-optimizations; not blocking. |

---

## Verification

```
pnpm check
> svelte-kit sync && svelte-check --tsconfig ./tsconfig.json
1777253736507 START
1777253736515 COMPLETED 5256 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

`mcp__svelte__svelte-autofixer` returned `{"issues":[],"suggestions":[]}`
on the new stub create-page (representative sample of the new Svelte 5
files).

No Rust changes in this pass — the only `api/src/` surface in scope was
the `analytics/dashboard` proxy, and the proxy lives on the SvelteKit
side. Backend endpoint coverage for the new proxies is the Rust team's
follow-up; today the proxies cleanly forward 404 when the upstream is
absent rather than masking with empty arrays.
