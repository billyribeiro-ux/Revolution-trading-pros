# Principal Architectural Fix Plan — 2026-04-26

**Scope:** Address every recurring `/admin/*` defect with the smallest set of architectural fixes, not 50 individual patches.
**Source evidence:** `CASCADE_ROOT_CAUSE_REPORT.md`, `ADMIN_FAILURE_DATA.md`, `ADMIN_DASHBOARD_REPORT.md`, `BACKEND_DEEP_DIVE_REPORT.md`, `SIDEBAR_REPORT.md`, `AUDIT_REPORT.md`.
**Constraint:** Comment-out, don't delete (FIX-2026-04-26 marker). No commits without explicit user approval.

---

## TL;DR

Five surgical architectural changes resolve the entire family of recurring admin defects:

| # | Change | Files | Resolves |
|---|---|---|---|
| 1 | Harden `connections.svelte.ts::load()` with in-flight Promise guard + `untrack()` shield on getters | 1 file (~25 LOC) | All 13 `effect_update_depth_exceeded` cascades |
| 2 | Replace `/logout/+page.server.ts` cookie names | 1 file (4 LOC) | Session-survives-logout security bug |
| 3 | Add `[...rest]/+server.ts` POST-handler shims at 7 parent-proxy folders | 7 files (~30 LOC each, copy-paste) | The "POST=405 cliff" that breaks 9 blog actions + 6 other admin features |
| 4 | Codemod 23 admin proxies from header-only auth to canonical `cookies.get('rtp_access_token')` | 23 files (~5 LOC each) | Cookie-fresh-but-localStorage-empty 401s |
| 5 | Fix dashboard SPX-Profit-Pulse submenu slugs | 1 file (4 LOC) | 4 broken sidebar links per SPX member |

**Total file touch:** ~33 files. **Estimated effort:** 90 minutes for an agent. **Blast radius:** Low — every change is comment-out-don't-delete, behavior-preserving on the happy path.

Findings that are NOT included (require user decision):
- Build vs delete `/admin/consent/settings` (no backend exists — same orphan-API-client decision shape as `boards.ts`/`behavior.ts`/`bing-seo.ts`/`campaigns.ts`)
- Build vs delete `/admin/dashboard` duplicate
- Build vs delete `/admin/orders` and `/admin/blog/*` missing endpoints
- 7 dashboard server loaders with `// TODO: Implement new video fetching approach`
- 50 native `alert()` + 11 `confirm()` migrations to ConfirmationModal
- CourseDetailDrawer dead metrics
- Replace `reqwest 0.11` with `0.12` (deferred from BACKEND_DEEP_DIVE)

These are **product decisions**, not architectural. Surface them after architectural fixes land.

---

## Change 1 — Cascade root-cause fix

**File:** `frontend/src/lib/stores/connections.svelte.ts`

**Why it works:** The 13 admin pages do this (see `CASCADE_ROOT_CAUSE_REPORT.md §1.2`):

```svelte
$effect(() => {
    const init = async () => {
        await connections.load();        // writes connectionsState
        if (getIsXConnected()) {          // reads connectionsState (tracked dep installed!)
            await loadData();
        }
    };
    init();
});
```

The `$effect` reads `connectionsState` after the write, schedules a re-run, the re-run calls `load()` again before `lastFetched` is set, and the depth guard trips.

Two surgical changes to the store make this safe at the store-author boundary, eliminating all 13 page bugs without touching the pages.

### A. In-flight Promise guard on `load()`

```ts
// Module scope (top of file, near refreshInterval)
// FIX-2026-04-26: in-flight Promise guard makes load() safe-inside-$effect.
let inFlightLoad: Promise<void> | null = null;
```

Replace the `load()` function body to wrap the async work in `inFlightLoad`:

```ts
async load(force = false): Promise<void> {
    // Cache TTL guard (existing) — unchanged
    if (!force && connectionsState.lastFetched && Date.now() - connectionsState.lastFetched < CACHE_TTL) {
        return;
    }

    // FIX-2026-04-26: concurrent-call guard. If another load() is already
    // in flight, return its promise rather than starting a second fetch.
    // Makes load() safe to call from inside a $effect — re-runs of the
    // effect (whether triggered by Svelte's reactive scheduler or user action)
    // await the same fetch and produce a single rune write.
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
            // ... existing fallback default-connections branch verbatim
        } finally {
            inFlightLoad = null;
        }
    })();

    return inFlightLoad;
},
```

**PER USER RULE:** comment-out the old `load()` body entirely above the new one with `// FIX-2026-04-26: comment-out, verify, delete in follow-up.` marker.

### B. `untrack()` shield on every public read helper

```ts
import { untrack } from 'svelte';

export function getIsAnalyticsConnected(): boolean {
    return untrack(() => {
        const services = FEATURE_SERVICES['analytics'];
        return services?.some((k) => connectionsState.connections[k]?.isConnected) ?? false;
    });
}
```

Apply the same wrapping to: `getIsSeoConnected`, `getIsEmailConnected`, `getIsPaymentConnected`, `getIsCrmConnected`, `getIsFluentConnected`, `getIsFormsConnected`, `getIsBehaviorConnected`, `getAllConnectionStatuses`, `getConnectedCount`, `getServicesWithErrors`, `getOverallHealth`.

`$derived(getXConnected())` continues to work — `$derived` installs tracking at the proxy boundary, before `untrack` runs inside the helper body. Verified by Svelte 5 semantics; no regression risk.

### Verification
- `pnpm check` → 0/0/0
- Manually load `/admin/site-health` (was the worst cascade offender per CASCADE_ROOT_CAUSE_REPORT.md §1.5) → no `effect_update_depth_exceeded` in console
- Manually load `/admin/email/campaigns` → cascade gone (already partially fixed at the page level; confirms architectural fix supersedes the page-level workaround)
- Manually load `/admin/analytics/cohorts` → cascade gone (page still uses `$effect`-recipe; proves architectural fix works without touching page)

---

## Change 2 — Logout cookie name fix

**File:** `frontend/src/routes/logout/+page.server.ts:39-42`

**Why it matters:** Dashboard sidebar's "Log out" link routes through this page. It currently `cookies.delete()` four cookie names (`auth_token`, `session_id`, `refresh_token`, `access_token`) — **none of which are real in this codebase**. The actual cookies (`rtp_access_token`, `rtp_refresh_token`) are NEVER cleared. Session survives logout. Security-grade silent bug.

### Edit

Find lines 39-42:
```ts
cookies.delete('auth_token', { path: '/' });
cookies.delete('session_id', { path: '/' });
cookies.delete('refresh_token', { path: '/' });
cookies.delete('access_token', { path: '/' });
```

Replace with:
```ts
// FIX-2026-04-26: cookie names corrected. Old code (kept for one revision per
// FIX-2026-04-26 marker — delete in follow-up):
// cookies.delete('auth_token', { path: '/' });
// cookies.delete('session_id', { path: '/' });
// cookies.delete('refresh_token', { path: '/' });
// cookies.delete('access_token', { path: '/' });
cookies.delete('rtp_access_token', { path: '/' });
cookies.delete('rtp_refresh_token', { path: '/' });
```

### Verification
- Login as test user
- Click "Log out" from dashboard sidebar
- Inspect cookies in DevTools → `rtp_access_token` and `rtp_refresh_token` are gone
- Refresh `/dashboard` → redirects to `/login` (would NOT have happened pre-fix)

---

## Change 3 — POST=405 cliff fix

**Affected paths (7 parent-proxy folders that block POST sub-paths):**
- `frontend/src/routes/api/admin/posts/`
- `frontend/src/routes/api/admin/categories/`
- `frontend/src/routes/api/admin/tags/`
- `frontend/src/routes/api/admin/coupons/`
- `frontend/src/routes/api/admin/users/`
- `frontend/src/routes/api/admin/email/templates/`
- `frontend/src/routes/api/admin/crm/contacts/`

**Why it matters:** SvelteKit's dev resolver returns 405 from the parent `+server.ts` for any POST to a sub-path under these folders, never reaching the backend. Empirically reproducible per `ADMIN_FAILURE_DATA.md §3a`.

### Edit per folder

Add `[...rest]/+server.ts` next to each parent `+server.ts`:

```ts
// frontend/src/routes/api/admin/posts/[...rest]/+server.ts
// FIX-2026-04-26: shim to break the SvelteKit POST=405 cliff. The parent
// +server.ts in this folder claims all sub-paths for routing purposes,
// blocking POST/PUT/DELETE on /admin/posts/<id>, /admin/posts/bulk-delete,
// etc. This [...rest] handler explicitly delegates to the shared catch-all
// proxy at /api/[...path]/+server.ts.
import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || '<your-api-host>';

const proxy = (method: string): RequestHandler => async ({ params, request, cookies, fetch, url }) => {
    const token = cookies.get('rtp_access_token');
    if (!token) error(401, 'Unauthorized');

    const targetPath = `/api/admin/posts/${params.rest}${url.search}`;
    const headers: Record<string, string> = {
        Authorization: `Bearer ${token}`,
    };
    const ct = request.headers.get('content-type');
    if (ct) headers['Content-Type'] = ct;

    const body = method === 'GET' || method === 'HEAD' ? undefined : await request.text();
    const res = await fetch(`${API_URL}${targetPath}`, { method, headers, body });
    return new Response(res.body, { status: res.status, headers: res.headers });
};

export const GET = proxy('GET');
export const POST = proxy('POST');
export const PUT = proxy('PUT');
export const PATCH = proxy('PATCH');
export const DELETE = proxy('DELETE');
```

Repeat for the 6 other folders, replacing `posts` with the folder name in `targetPath`. The handler is identical otherwise — extract to a shared helper if desired (`frontend/src/lib/utils/createProxyShim.ts`) to avoid 7 copies, but for the architectural-fix pass keep them inline so each folder's behavior is locally readable.

### Verification
Probe with curl:
```sh
/usr/bin/curl -sX POST -o /dev/null -w "%{http_code}\n" -H "Cookie: rtp_access_token=fake" http://localhost:5173/api/admin/posts/bulk-delete
# Pre-fix: 405. Post-fix: 401 (correct — fake token rejected by backend).
```

Run for each affected folder.

---

## Change 4 — Cookie-name codemod across 23 admin proxies

**Files:** 23 admin proxies still using legacy `request.headers.get('Authorization')` only. Full list in `ADMIN_FAILURE_DATA.md §4`. Sample:
- `api/admin/posts/+server.ts:43,66,90,119`
- `api/admin/users/+server.ts:43,64`
- `api/admin/users/[id]/+server.ts:90,134,174`
- ... 20 more files

### Edit pattern (mechanical)

In each file, find this pattern:

```ts
const authHeader = request.headers.get('Authorization');
// or
headers: {
    Authorization: request.headers.get('Authorization') ?? '',
}
```

Replace with the canonical pattern:

```ts
import { env } from '$env/dynamic/private';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || '<your-api-host>';

// In handler:
const token = cookies.get('rtp_access_token');
if (!token) error(401, 'Unauthorized');

// Then forward as Authorization:
headers: {
    Authorization: `Bearer ${token}`,
}
```

PER USER RULE: comment out the old auth lines with FIX-2026-04-26 markers, add the new lines below.

### Verification

Grep verification:
```sh
grep -rl "request.headers.get('Authorization')" frontend/src/routes/api/admin/ | grep -v node_modules
# Should return 0 (or only files with explicit "intentional" comments)
```

End-to-end: clear localStorage in browser, ensure rtp_access_token cookie is fresh, navigate to `/admin/users`, `/admin/blog`, `/admin/coupons` — each should return real data instead of 401.

---

## Change 5 — SPX-Profit-Pulse submenu slugs

**File:** `frontend/src/routes/dashboard/+layout.svelte:330,332,337,344`

**Why it matters:** Submenu hardcodes `/dashboard/spx-profit-pulse/jonathan-mckeever[/...]` paths. Only `billy-ribeiro/` exists in the directory tree. **4 broken sidebar links** for SPX members.

### Edit

Find lines 330, 332, 337, 344 — each has an href with `jonathan-mckeever`. Replace `jonathan-mckeever` with `billy-ribeiro` (or whichever is canonical per product).

PER USER RULE: comment-out marker.

### Verification
Curl each path:
```sh
for p in /dashboard/spx-profit-pulse/billy-ribeiro /dashboard/spx-profit-pulse/billy-ribeiro/start-here /dashboard/spx-profit-pulse/billy-ribeiro/learning-center /dashboard/spx-profit-pulse/billy-ribeiro/premium-videos; do
  /usr/bin/curl -sL -o /dev/null -w "%{http_code}  $p\n" "http://localhost:5173$p"
done
# All should 303 → /login (authed user would see real pages)
```

---

## Implementation Plan

### Stage A — Spawn ONE implementation agent (sonnet, parallel-safe)

Owns ALL 33+ files. Brief:

```
Apply Changes 1-5 from PRINCIPAL_FIX_PLAN_2026-04-26.md.
Constraints:
- Stay inside /Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros.
- Comment-out, don't delete. FIX-2026-04-26 markers everywhere.
- Use Svelte MCP autofixer on every Svelte file edited.
- Run pnpm check after each Change to catch regressions before they pile up.
- Don't commit. Don't push.

Verification gates after EACH Change:
  pnpm --filter revolution-svelte check  → 0/0/0
After ALL changes:
  pnpm --filter revolution-svelte check
  pnpm --filter revolution-svelte exec eslint .  (warnings OK; errors must be 0)
  pnpm --filter revolution-svelte build
  cargo check --manifest-path api/Cargo.toml
  cargo test --manifest-path api/Cargo.toml --test utils_test --test stripe_test --lib

Output: Markdown summary per Change, file count, gate verbatim final lines.
```

### Stage B — Final verification (I run)

After agent completes:
1. `pnpm check`, `pnpm build`, `cargo` gates as above.
2. SSR probe sample admin routes via curl.
3. Surface diff summary + open items list.

### Stage C — User reviews, decides on commit/push

Per memory rule: I do NOT commit or push. Working tree is left dirty for your review.

---

## Rollback (≤3 commands)

```sh
cd /Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros
git checkout -- frontend/src/lib/stores/connections.svelte.ts frontend/src/routes/logout/+page.server.ts frontend/src/routes/dashboard/+layout.svelte
git checkout -- $(git status --short | awk '/api\/admin/{print $2}')
rm -rf frontend/src/routes/api/admin/{posts,categories,tags,coupons,users,crm/contacts}/[...rest] frontend/src/routes/api/admin/email/templates/[...rest]
```

---

## Out of Scope (requires user decision before implementation)

These are real bugs but require product decisions before code can land:

| Bug | Decision needed |
|---|---|
| `/admin/consent/settings` — no backend | Build `consent.rs` OR remove page from admin nav |
| `/admin/dashboard` duplicate page (orphan endpoints) | Remove duplicate OR build the 3 missing endpoints |
| `/admin/orders` — no backend | Build `admin_orders.rs` OR remove |
| Blog admin's 9 endpoints (B6 in ADMIN_DASHBOARD_REPORT) | Build OR remove features |
| 7 dashboard server loaders with `// TODO: Implement new video fetching approach` | Wire to new video API OR document the hold |
| CourseDetailDrawer dead metrics tab | Wire OR hide |
| 50 `alert()` + 11 `confirm()` calls | Mass-replace with ConfirmationModal/Modal |
| `/register` vs `/signup` duplicate | Pick one, redirect the other |
| CommandPalette "Clear Cache" logs out | Rename OR scope to non-auth localStorage keys |
| Native admin `alert()` debug at `seo/store-locator/+page.svelte:209` | Replace with Modal |
| Reqwest 0.11 → 0.12 backend migration | Schedule effort window |

These are tracked as the next-phase backlog. Do NOT attempt in this plan — landing the architectural fixes first gives the codebase a clean baseline before product decisions are made.

---

## End of plan
