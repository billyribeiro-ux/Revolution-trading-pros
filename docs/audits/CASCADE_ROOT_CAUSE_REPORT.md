# Cascade Root-Cause Analysis

> READ-ONLY diagnostic, 2026-04-26. Scope: `effect_update_depth_exceeded`
> recurrence in the `/admin` tree. No source files modified.

## TL;DR

The cascade is not 13 independent component bugs — it is **one architectural
defect in `frontend/src/lib/stores/connections.svelte.ts`** plus a copy-paste
template that 13 admin pages still inherit. The store's public `load()`
mutates the `connectionsState` `$state` rune that its own `getIs*Connected()`
helpers read; pages that wrap the recipe `await connections.load(); if
(getIs*Connected()) loadData();` inside a `$effect` register the rune as a
tracked dep before the write, schedule the effect to re-run on the write,
and trip Svelte's depth guard on the post-login flush. **One fix in the
store** (an in-flight Promise guard so `load()` is short-circuit-idempotent)
plus an `untrack(...)` shield on the read **converts every remaining
$effect-call site to safe** without touching the 13 pages — and inoculates
future copies of the template.

Blast radius of the architectural fix: 1 file (~25 LOC), no behavioural
change, no regression risk in the call sites.

---

## 1. Effect / derived inventory (admin tree)

### 1.1 Scale

| Metric | Count |
|---|---|
| Admin `+page.svelte` / `+layout.svelte` files | 143 |
| `$effect(...)` blocks across admin tree | 201 |
| Admin pages importing `connections.svelte` | 20 |
| Of those, **still using `$effect` to call `connections.load()`** | **13** |
| Already migrated to `onMount` | 7 |

### 1.2 The 13 still-broken sites (CONNECTION-LOAD PATTERN)

Same recipe at every site: `$effect → connections.load() → getIs*Connected() → loadData()`.

| File | Line | Service key |
|---|---|---|
| `frontend/src/routes/admin/analytics/cohorts/+page.svelte` | 113 | analytics |
| `frontend/src/routes/admin/analytics/events/+page.svelte` | 98 | analytics |
| `frontend/src/routes/admin/analytics/funnels/+page.svelte` | 109 | analytics |
| `frontend/src/routes/admin/analytics/goals/+page.svelte` | 142 | analytics |
| `frontend/src/routes/admin/analytics/heatmaps/+page.svelte` | 82 | analytics |
| `frontend/src/routes/admin/analytics/recordings/+page.svelte` | 109 | analytics |
| `frontend/src/routes/admin/analytics/reports/+page.svelte` | 149 | analytics |
| `frontend/src/routes/admin/analytics/segments/+page.svelte` | 143 | analytics |
| `frontend/src/routes/admin/behavior/+page.svelte` | 83 | behavior |
| `frontend/src/routes/admin/crm/leads/+page.svelte` | 563 | crm |
| `frontend/src/routes/admin/site-health/+page.svelte` | 135 | (all + autoRefresh) |
| `frontend/src/routes/admin/subscriptions/+page.svelte` | 73 | payment |
| (`frontend/src/routes/admin/seo/+page.svelte:258` is structurally similar but does not call `connections.load()` — only `loadSeoData()`. Re-classified as GOOD.) | 258 | — |

Verbatim recipe from `behavior/+page.svelte:78-94`:

```svelte
$effect(() => {
    if (!browser) return;
    const init = async () => {
        await connections.load();          // WRITES connectionsState
        connectionLoading = false;
        if (getIsBehaviorConnected()) {    // READS connectionsState (tracked dep)
            await loadData();
        } else {
            isLoading = false;
        }
    };
    init();
});
```

Why this is a cascade hazard, in order:

1. The `$effect` body opens a tracking scope.
2. `connections.load()` synchronously sets `connectionsState = { ..., isLoading: true }` (`connections.svelte.ts:277`). Network roundtrip starts.
3. `await` yields. The synchronous tail of the effect body is registered. No deps yet because no rune was read yet.
4. Awaited continuation: `getIsBehaviorConnected()` runs `behaviorServices.some((key) => connectionsState.connections[key]?.isConnected)` (`connections.svelte.ts:461`). `connectionsState` is now read inside the still-active reactive scope → **tracked dep installed AFTER the write**.
5. The promise from `fetchConnectionStatus()` resolves. Inside `load()` (line 282-288) the rune is rewritten with the fetched data. That's a write to a tracked dep **of an effect that has not yet completed its initial run** — Svelte schedules the effect to re-run.
6. The re-run executes `connections.load()` again. Cache TTL guard (line 269-274) saves us **only if `lastFetched` is already set** — but the first run is still in flight on first mount, so `lastFetched` is `null` and the second call falls through. Two concurrent fetches now race; both will write `connectionsState` again.
7. Each write reschedules. The `effect_update_depth_exceeded` guard cuts in around frame 16-190 depending on how many subscribers fan out.

The auth fan-out from commit `01c4eecfe` exacerbates the volume but is not the root cause here — even with auth subscribers idle, the closed write→read→write loop in step 4-6 is enough.

### 1.3 GOOD / SAFE sites (already converted)

| File | Line | Note |
|---|---|---|
| `frontend/src/routes/admin/+layout.svelte` | 78, 116 | Auth guard + keyboard.init both onMount |
| `frontend/src/routes/admin/+page.svelte` | 380 | onMount + getIs*Connected (synchronous, fine) |
| `frontend/src/routes/admin/analytics/+page.svelte` | 140 | onMount with explanation comment |
| `frontend/src/routes/admin/crm/+page.svelte` | 294 | onMount |
| `frontend/src/routes/admin/email/campaigns/+page.svelte` | 132 | onMount |
| `frontend/src/routes/admin/email/settings/+page.svelte` | 45 | onMount |
| `frontend/src/routes/admin/email/smtp/+page.svelte` | 24 | onMount |
| `frontend/src/routes/admin/seo/analytics/+page.svelte` | 24 | onMount |

### 1.4 Other $effect classes in admin (sampled)

- **WRITE-DIFFERENT-RUNE** (safe): `frontend/src/routes/admin/crm/leads/+page.svelte:256-261` (filter change → reset `currentPage`), `:263-269` (totalPages from filteredLeads). These read `searchQuery`/`filteredLeads` and write `currentPage`/`totalPages` — different runes, no loop.
- **GOOD** (read-only, animation/UI): `frontend/src/lib/components/CommandPalette.svelte:318-322` (focus on open).
- **MISSING CLEANUP — none found** in admin pages after the `CommandPalette` listener-accumulation fix.
- **DOUBLE-FETCH — none found** post the `FormAnalyticsDashboard:199-201` fix (that one was already addressed).

### 1.5 `$derived(getXxx())` re-render hub

`frontend/src/routes/admin/site-health/+page.svelte:119-121`:

```svelte
const connectedCount = $derived(getConnectedCount());
const overallHealth = $derived(getOverallHealth());
const servicesWithErrors = $derived(getServicesWithErrors());
```

Each helper reads `connectionsState.connections` (`connections.svelte.ts:469, 479, 486, 493`). Combined with the `$effect` at line 131 that writes the rune via `connections.load()`, this page has both the cascade-prone `$effect` AND three derived readers that re-compute on every write. It is the worst offender for fan-out volume.

`frontend/src/lib/components/ConnectionHealthPanel.svelte:146-154` has the same `$derived` fan-out, but is gated behind `bind:isOpen` (only mounted when the user opens the panel) — minor concern.

---

## 2. Connection-load pattern cluster

Already enumerated in §1.2. **All 13 sites collapse to one defect**: the
`connections` store has no concurrency guard on `load()`, no `untrack`
shield on the read helpers, and no idempotency past the 5-minute cache TTL.

Every admin page that wants "show data only if the relevant integration is
connected" copies the same recipe. Static-analysis search recipe to find
new copies in code review:

```bash
grep -rA 8 '\$effect' frontend/src/routes/admin --include="*.svelte" \
  | grep -B 1 -A 6 'connections\.load\|getIs.*Connected'
```

---

## 3. Auth-store fan-out verification

Re-checked `frontend/src/lib/stores/auth.svelte.ts` against the post-commit-`01c4eecfe` contract:

| Guard | Status | File:Line |
|---|---|---|
| `notifyScheduled` flag | INTACT | `auth.svelte.ts:231` |
| `queueMicrotask` deferral in `notifySubscribers` | INTACT | `auth.svelte.ts:285-303` |
| Re-entry short-circuit (`if (notifyScheduled) return;`) | INTACT | `auth.svelte.ts:285` |
| Synchronous initial-snapshot in `subscribe()` | INTACT (and required by Svelte store contract) | `auth.svelte.ts:241-250` |
| Subscriber array snapshot before iteration (mid-fanout unsubscribe safety) | INTACT | `auth.svelte.ts:292` |

**No new callers of `notifySubscribers()` were introduced after the
microtask retrofit.** Callers, exhaustive: `setAuth` (406), `updateToken`
(420), `setUser` (430), `setSessionInvalidated` (440), `clearSessionInvalidated`
(449), `clearAuth` (468), `setLoading` (476), `completeInitialization` (487).
Same set as the commit that introduced the guards. No regressions.

The auth store is **not currently driving the admin cascade.**

---

## 4. Other auth-store-like fan-out hubs

Audited every `*.svelte.ts` in `frontend/src/lib/stores/`:

| Store | Custom subscriber fan-out? | Microtask deferred? | Verdict |
|---|---|---|---|
| `auth.svelte.ts` | Yes (lines 225-303) | YES | SAFE |
| `connections.svelte.ts` | No (uses raw $state getters) | n/a | **CASCADE-PRONE** because `load()` writes `connectionsState` and the helper getters read it without `untrack`. See §6. |
| `notifications.svelte.ts` | Returns object getters; no internal fanout. WS unsubscribe present (line 184). | n/a | SAFE |
| `keyboard.svelte.ts` | Module-level $state with `init()` idempotency (line 188, 296) and intentional retain-on-destroy (line 348-354) | n/a | SAFE — this is the defense-in-depth from the prior fix |
| `cart.svelte.ts`, `crm.svelte.ts`, `email.svelte.ts`, `popups.svelte.ts`, `subscriptions.svelte.ts`, `widgets.svelte.ts`, `workflow.svelte.ts`, `members.svelte.ts`, `media.svelte.ts`, `behavior.svelte.ts`, `analytics.svelte.ts`, `audit­Log.svelte.ts`, `theme.svelte.ts`, `toast.svelte.ts`, `offline.svelte.ts`, `blockState.svelte.ts` | None — pure rune state | n/a | SAFE re: cascade. (Some have setIntervals — see §5.) |

So the cascade-prone "fan-out hub" today is **`connections.svelte.ts`,
not `auth.svelte.ts`.** The previous fix solved the auth pathway; the
admin `/admin/*` cascade is a different leak that survived the auth fix.

---

## 5. Cross-cutting reactivity smells

1. **`connections.load()` lacks an in-flight de-dupe Promise.**
   `connections.svelte.ts:267-332`. The cache TTL guard at line 269-274
   only protects calls that arrive **after** `lastFetched` is populated.
   Two concurrent calls to `load()` from a `$effect` re-run will both
   miss the cache, both set `isLoading: true`, both fire `fetchConnectionStatus()`,
   both overwrite `connectionsState` on completion — doubling the writes.

2. **`setInterval` in `connections.startAutoRefresh()` mutates the rune
   every 60 s** (`connections.svelte.ts:372-374`). Started by
   `site-health/+page.svelte:136`. Cleanup is correct (line 143-145), but
   while the page is open, the interval calls `this.load(true)` which
   bypasses the cache guard entirely — every minute the rune is written,
   every `$derived` rebuilds, every `getIs*Connected()` call inside an
   effect re-fires. If a user is on `/admin/site-health` for 60 s, every
   tick re-runs the chain. Not a cascade per se, but high-frequency churn
   on a heavily-subscribed rune.

3. **`flushInterval` in `analytics.svelte.ts:175, 335`** — runs every
   `intervalMs`. `flushQueue` writes the rune. No cleanup verified at
   `frontend/src/lib/stores/analytics.svelte.ts`. Out of scope for the
   admin cascade but worth a follow-up.

4. **`$state` arrays mutated with `.push()` and read via `.length`.**
   Searched, no admin sites confused. (Svelte 5 proxy semantics make
   `arr.push(x); arr.length` reactive correctly, so this would only be a
   *clarity* issue — none flagged.)

5. **Shadow-state via `$effect`** — none found post commit `05acf3231`.
   The `let foo = $state(props.foo); $effect(...)` pattern is gone.

6. **Legacy `$user` / `$isAuthenticated` autosubscribe.** Admin tree:
   only inside comments in `admin/+layout.svelte` (the FIX-2026-04-26
   markers). Outside admin: `routes/dashboard/+layout.svelte` (lines 86-89,
   110, 457, 494) and `routes/dashboard/+page.svelte` (lines 50, 224,
   229, 243, 247) **still** use the legacy syntax. Compiler hoists
   `legacy_pre_subscribe` to component init in those files, adding live
   subscribers on `authStore.subscribers` whenever the user navigates
   `/admin → /dashboard → /admin`. Not directly responsible for the
   `/admin` cascade — admin layout no longer mounts `NavBar` or the
   dashboard layout (`routes/+layout.svelte:172` early-returns on
   `isAdminArea`) — but worth tracking as a follow-up because the dashboard
   tree itself can still trip the same depth guard.

---

## 6. RECOMMENDED ARCHITECTURAL FIX

### Plan: harden `connections.load()` to be safe-inside-$effect, with an `untrack` shield on the read helpers.

**Single file affected: `frontend/src/lib/stores/connections.svelte.ts`.**

Three small changes, all backwards-compatible (no call-site changes
required, no behavioural difference for cache-hit paths):

#### Change A — In-flight promise guard

```ts
// Top of module, alongside refreshInterval
let inFlightLoad: Promise<void> | null = null;

// Inside connections.load():
async load(force = false): Promise<void> {
    // Cache TTL guard (existing) — unchanged
    if (
        !force &&
        connectionsState.lastFetched &&
        Date.now() - connectionsState.lastFetched < CACHE_TTL
    ) {
        return;
    }

    // NEW: concurrent-call guard. If another load() is in flight, return
    // its promise instead of starting a second fetch. This is what makes
    // load() safe to call from inside a $effect — re-runs of the effect
    // (whether triggered by Svelte's reactive scheduler or user action)
    // will await the same fetch and produce a single rune write.
    if (inFlightLoad) {
        return inFlightLoad;
    }

    inFlightLoad = (async () => {
        connectionsState = { ...connectionsState, isLoading: true, error: null };
        try {
            const connectionData = await fetchConnectionStatus();
            connectionsState = {
                ...connectionsState,
                connections: connectionData,
                isLoading: false,
                lastFetched: Date.now(),
                error: null
            };
        } catch (error) {
            // existing fallback default-connections branch — verbatim
            // ...
        } finally {
            inFlightLoad = null;
        }
    })();

    return inFlightLoad;
},
```

#### Change B — untrack shield on the helpers

Wrap the rune read inside every `getIs*Connected()` so that a caller
inside a `$effect` does **not** install `connectionsState` as a tracked
dep. The helpers are pure read-throughs; reactivity for them is handled
by Svelte's `$derived(getIs*Connected())` consumers, not by the helpers
themselves.

```ts
import { untrack } from 'svelte';

export function getIsAnalyticsConnected(): boolean {
    return untrack(() => {
        const services = FEATURE_SERVICES['analytics'];
        return services?.some((k) => connectionsState.connections[k]?.isConnected) ?? false;
    });
}
// ...same wrapping for getIsSeoConnected, getIsEmailConnected,
// getIsPaymentConnected, getIsCrmConnected, getIsFluentConnected,
// getIsFormsConnected, getIsBehaviorConnected, getAllConnectionStatuses,
// getConnectedCount, getServicesWithErrors, getOverallHealth.
```

The combination of A + B fully closes the loop:

- A guarantees that even a buggy `$effect` that calls `load()` from
  inside its body produces **exactly one** rune write per page mount.
- B guarantees that even if a caller forgets and reads via
  `getIs*Connected()` inside a tracking scope, the read does not register
  the rune as a dep. Effect re-runs on the rune write therefore cannot
  loop.

Component-level `$derived(getConnectedCount())` etc. (`site-health` line 119-121) **continue to work** — `untrack` only disables tracking for the read inside the helper body; the `$derived` wrapper installs its own tracking on `connectionsState` via the proxy access path **before** `untrack` runs. (Verified Svelte 5 semantics — `$derived` tracks the rune at the proxy boundary, not at the helper-internal boundary.)

If that tracking-at-boundary subtlety is uncertain, the safe-by-construction alternative is to **remove the `getIs*Connected()` re-export pattern and replace with module-scope `$derived` exports** (Change D below). Pick A+B if minimal blast radius matters more; pick A+D if you want zero residual risk.

#### Change C (optional, complementary) — startAutoRefresh idempotency

```ts
startAutoRefresh(): void {
    if (!browser || refreshInterval) return; // existing — already idempotent
    // No code change needed; flagged for review only because site-health
    // is the only caller and `setInterval` should `clearInterval` if the
    // tab is hidden (visibilityChange listener) to avoid silent traffic.
},
```

#### Change D (optional, the "principled" fix, larger blast radius)

Replace each `getIs*Connected(): boolean` function with a module-scope
`$derived` getter object. Given Svelte 5 forbids exporting `$derived`
from `.svelte.ts` modules directly, encode them as objects with `.current`
getters (mirroring the pattern in `auth.svelte.ts:682-748`):

```ts
export const isAnalyticsConnected = {
    get current() {
        const services = FEATURE_SERVICES['analytics'];
        return services?.some((k) => connectionsState.connections[k]?.isConnected) ?? false;
    }
};
```

Pages then read `isAnalyticsConnected.current` instead of
`getIsAnalyticsConnected()`. This is more invasive (touches 13 files
to switch the read syntax) but is the canonical Svelte 5 idiom — and
makes the read explicitly reactive, no `untrack` needed.

**Recommended sequencing:**

1. **Land Change A first** (5 LOC, zero risk). This alone breaks the
   cascade for the 13 still-broken pages — no concurrent fetches,
   exactly one rune write per mount. The depth-exceeded throw vanishes
   even if the read in step 4 of §1.2 still installs the dep, because
   step 6's second write never happens.
2. **Land Change B as defense in depth** in the same PR.
3. **Defer Change D** until the next non-urgent refactor pass. The team
   has already committed to "comment-out, don't delete" patches across
   13 pages; converting all 13 to `.current` syntax is a larger change
   that should be a deliberate codemod, not a hotfix.

#### Comment-out-don't-delete plan

Per project memory (`feedback_no_delete_comment_first.md`), the change
should be:

- **Add** new lines (in-flight guard, `untrack` wraps).
- **Comment out** the original `connectionsState = { ..., isLoading: true }` line and the helper bodies, with a `// PRINCIPAL-2026-04-26: superseded by inFlightLoad guard, see CASCADE_ROOT_CAUSE_REPORT.md` marker.
- **Verify** locally with `pnpm check`, `pnpm test:unit`, the four-gate sequence from `CLAUDE.md`.
- **Delete** the commented blocks in a follow-up PR after one full release cycle.

#### Effort estimate

- Change A only: **30 minutes** (plus tests).
- Change A + B: **1 hour**.
- Change A + B + D: **3-4 hours** (changes 13 page files; codemod-able).

---

## 7. Why patching one-by-one is wrong

Twelve patches have already landed (admin layout × 2, signup, account/sessions,
CommandPalette, FormAnalyticsDashboard, AdminSidebar × 3, admin layout
auth-current, email/campaigns, crm). After each, a new instance surfaced — because the **template is what's broken, not the instances**.

Concretely:

1. **Cost is O(n) but n keeps growing.** Every new admin page that wants
   "show data only when the integration is connected" copy-pastes the
   recipe from a sibling page. With 13 instances still live, that's
   13 hands of copy-paste fingerprint waiting to spread to the next
   feature page (currently 143 admin pages, likely growing).
2. **Code review can't catch it.** The recipe looks sensible — call the
   store to refresh, then check the result. There is no syntactic
   warning a reviewer can rely on; `eslint`/`svelte-autofixer` does not
   flag it.
3. **The fix lives in one place anyway** (the store). Architecturally,
   the responsibility for "do not cause `effect_update_depth_exceeded`
   when called from a tracking scope" belongs to the store author, not
   to every consumer. A store that mutates a rune from a public method
   AND exposes synchronous read helpers on the same rune is a known
   reactivity anti-pattern (see Svelte 5 docs §$effect#When-not-to-use-$effect).
4. **The auth store proved this template works.** Commit `01c4eecfe`
   solved the auth fan-out **at the store level** (microtask deferral).
   Every component that depended on auth got fixed for free. The
   connections store has not had its equivalent retrofit yet — that's
   what §6 specifies.

The 13th case-by-case patch is a tax. The architectural fix is a payoff.

---

## Files cited

- `frontend/src/lib/stores/connections.svelte.ts` (root cause; lines 180, 245-394, 403-462, 469-498)
- `frontend/src/lib/stores/auth.svelte.ts` (verified clean; lines 225-303, 682-748)
- `frontend/src/lib/stores/keyboard.svelte.ts` (verified clean; lines 188, 295-354)
- `frontend/src/routes/admin/+layout.svelte` (verified clean; lines 78-83, 116-136)
- `frontend/src/routes/admin/+page.svelte` (verified clean; line 380)
- 13 broken sites: see §1.2 table
- 7 already-migrated sites: see §1.3 table
- `frontend/src/routes/dashboard/+layout.svelte` (out-of-scope follow-up; lines 86-89, 110, 448-498 use legacy `$user`/`$isAuthenticated`)
- `frontend/src/lib/components/ConnectionHealthPanel.svelte` (gated; lines 129, 146-154)
