# 11 — Cross-cutting Audit (whole admin surface)

Scope: `frontend/src/routes/admin/**` and `frontend/src/routes/api/admin/**`
Method: read-only horizontal grep sweep, exhaustive enumeration.

Counts at-a-glance:

- Admin page files (`+page.svelte` / `+layout.svelte` / `+page.ts` / `+layout.ts`): **151**
- Admin API proxies (`+server.ts` under `routes/api/admin/`): **53**

---

## A. Hardcoded backend URLs

CLAUDE.md asserts that the canonical shape is:

```ts
import { env } from '$env/dynamic/private';
const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
```

**Result**: of the 53 admin proxies, **49 follow the canonical pattern**, **4 deviate**, and **0 use a hardcoded URL with no `env` lookup at all**.

### A.1 — Deviations: read only `env.BACKEND_URL` (skip `env.API_BASE_URL`)

These 4 files miss the `env.API_BASE_URL` first-fallback half of the canonical pattern. They still go through `env`, so they aren't strictly "hardcoded", but they are not the agreed shape:

| # | File | Line |
|---|------|------|
| 1 | [api/admin/bunny/uploads/+server.ts:16](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts#L16) | `const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';` |
| 2 | [api/admin/bunny/create-video/+server.ts:16](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L16) | same |
| 3 | [api/admin/bunny/upload/+server.ts:18](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L18) | same |
| 4 | [api/admin/bunny/video-status/[guid]/+server.ts:16](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts#L16) | same |

All four sit under `bunny/`. Trivially fixable (add the `env.API_BASE_URL ||` clause). The 14-of-14 audit fix from 2026-04-25 is otherwise holding.

### A.2 — Cosmetic redundancy across 25+ files

Many proxies declare both `PROD_BACKEND` (the canonical fallback chain) and then immediately rebind it to a local `BACKEND_URL`:

```ts
const PROD_BACKEND = env.API_BASE_URL || env.BACKEND_URL || 'https://...';
const BACKEND_URL = PROD_BACKEND;            // redundant
// ...inside handler...
const BACKEND_URL = PROD_BACKEND;            // redundant function-scoped redeclare
```

This is consistent across `courses/*`, `schedules/*`, `subscriptions/plans/stats`, `crm/*`, `trading-rooms/*`, `users/[id]`, `analytics/dashboard`, `products/stats`. Examples:

- [api/admin/courses/+server.ts:10-12](../../../frontend/src/routes/api/admin/courses/+server.ts#L10)
- [api/admin/schedules/+server.ts:17,333](../../../frontend/src/routes/api/admin/schedules/+server.ts#L17)
- [api/admin/crm/contacts/+server.ts:15-17](../../../frontend/src/routes/api/admin/crm/contacts/+server.ts#L15)
- [api/admin/users/[id]/+server.ts:14,59](../../../frontend/src/routes/api/admin/users/[id]/+server.ts#L14)
- [api/admin/analytics/dashboard/+server.ts:16,20](../../../frontend/src/routes/api/admin/analytics/dashboard/+server.ts#L16)

Not a correctness defect, but a sweeping inconsistency that violates the CLAUDE.md "match house style" rule.

---

## B. Svelte 5 shadow-state anti-pattern (`$state(props.x)` / `$state(... ?? null)` + `$effect`)

**Result**: **zero occurrences** anywhere under `frontend/src/routes/admin/`.

Greps:

```bash
grep -rEn --include="*.svelte" '\$state\(props\.'           # 0
grep -rEn --include="*.svelte" '\$state\([^)]*\?\?'         # 0
```

The 2026-04-25 migration of 41 components is intact for the admin surface. Section B is clean.

---

## C. Auth pattern inconsistency

CLAUDE.md / commit `e2356fa46` mandate `cookies.get('rtp_access_token')`.

**Result**: of the 53 proxies, **43 use `cookies.get('rtp_access_token')` directly** and **10 have no inline cookie check** because they delegate auth to a helper. Breakdown:

### C.1 — Proxies that delegate auth (no inline `cookies.get`)

| File | How auth is enforced |
|------|----------------------|
| [api/admin/posts/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/posts/[...rest]/+server.ts) | `createProxyShim` (reads `rtp_access_token` internally — see [createProxyShim.ts:51](../../../frontend/src/lib/utils/createProxyShim.ts#L51)) |
| [api/admin/categories/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/categories/[...rest]/+server.ts) | createProxyShim |
| [api/admin/tags/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/tags/[...rest]/+server.ts) | createProxyShim |
| [api/admin/coupons/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/coupons/[...rest]/+server.ts) | createProxyShim |
| [api/admin/users/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/users/[...rest]/+server.ts) | createProxyShim |
| [api/admin/orders/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/orders/[...rest]/+server.ts) | createProxyShim |
| [api/admin/email/templates/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/email/templates/[...rest]/+server.ts) | createProxyShim |
| [api/admin/crm/contacts/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/crm/contacts/[...rest]/+server.ts) | createProxyShim |
| [api/admin/email/settings/+server.ts](../../../frontend/src/routes/api/admin/email/settings/+server.ts) | **NO auth check at all** — see C.2 |
| [api/admin/connections/status/+server.ts](../../../frontend/src/routes/api/admin/connections/status/+server.ts) | **NO auth check at all** — see C.2 |

### C.2 — Proxies with NO auth check anywhere (HIGH SEVERITY)

Two proxies expose backend data with no token gate:

1. **[api/admin/email/settings/+server.ts](../../../frontend/src/routes/api/admin/email/settings/+server.ts)** — GET/PUT a module-scoped `emailSettings` object containing SMTP host / username / password (line 12-21). The PUT handler updates state in memory with no auth check. The route returns SMTP password (masked) on GET. Anyone reaching this URL can read or rewrite the SMTP config object on the frontend node.

2. **[api/admin/connections/status/+server.ts](../../../frontend/src/routes/api/admin/connections/status/+server.ts)** — returns built-in connection status; lower severity (mostly static).

Both are also orphan-ish: see Section H.

### C.3 — Naming variance (cosmetic)

Three different local-variable names are used for the same cookie:

- `cookieToken` — most files (e.g. [posts/+server.ts:46](../../../frontend/src/routes/api/admin/posts/+server.ts#L46))
- `token` — [connections/+server.ts:17](../../../frontend/src/routes/api/admin/connections/+server.ts#L17), [courses/*/+server.ts](../../../frontend/src/routes/api/admin/courses/+server.ts#L18), [consent/settings/*](../../../frontend/src/routes/api/admin/consent/settings/+server.ts#L22), [orders/[id]/+server.ts:29](../../../frontend/src/routes/api/admin/orders/[id]/+server.ts#L29)
- `accessToken` — [bunny/create-video/+server.ts:20](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L20), [bunny/upload/+server.ts:22](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L22), [bunny/video-status/[guid]/+server.ts:20](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts#L20)
- `session` — [bunny/uploads/+server.ts:31](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts#L31)

### C.4 — Dead commented-out alternative cookie reads

Several files still carry a `// const token = cookies.get('auth_token');` line above the live `rtp_access_token` read — leftover from the e2356fa46 migration. Examples (12 occurrences):

- [api/admin/membership-plans/+server.ts:16](../../../frontend/src/routes/api/admin/membership-plans/+server.ts#L16)
- [api/admin/products/stats/+server.ts:31](../../../frontend/src/routes/api/admin/products/stats/+server.ts#L31)
- [api/admin/subscriptions/plans/stats/+server.ts:18](../../../frontend/src/routes/api/admin/subscriptions/plans/stats/+server.ts#L18)
- [api/admin/courses/+server.ts:17,68](../../../frontend/src/routes/api/admin/courses/+server.ts#L17)
- [api/admin/courses/[id]/+server.ts:17,46,72](../../../frontend/src/routes/api/admin/courses/[id]/+server.ts#L17)
- [api/admin/courses/[id]/publish/+server.ts:17](../../../frontend/src/routes/api/admin/courses/[id]/publish/+server.ts#L17)
- [api/admin/courses/[id]/unpublish/+server.ts:17](../../../frontend/src/routes/api/admin/courses/[id]/unpublish/+server.ts#L17)
- [api/admin/bunny/uploads/+server.ts:30](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts#L30)

Per the user's `comment-out, don't delete` rule these are intentional. Per CLAUDE.md "match house style" they should at least carry a `TODO:` marker with a removal date. Currently neither.

---

## D. Direct external `fetch('http...')` from admin pages

**Result**: **zero** direct external fetches in `frontend/src/routes/admin/**.svelte` or `*.ts`.

```bash
grep -rEn --include="*.svelte" --include="*.ts" "fetch\(['\"\`]https?:" frontend/src/routes/admin/   # 0
```

However, there is an **indirect violation**: the helper `apiFetch` from [lib/api/config.ts:273](../../../frontend/src/lib/api/config.ts#L273) routes calls through `API_BASE_URL` (i.e. `https://revolution-trading-pros-api.fly.dev` in prod) directly, bypassing every SvelteKit `+server.ts` proxy in this repo. Five admin call sites do this:

| Page | Endpoint | Line |
|------|----------|------|
| [admin/email/settings/+page.svelte:58](../../../frontend/src/routes/admin/email/settings/+page.svelte#L58) | `apiFetch('/admin/email/settings')` | 58 |
| [admin/email/settings/+page.svelte:74](../../../frontend/src/routes/admin/email/settings/+page.svelte#L74) | `apiFetch('/admin/email/settings', PUT)` | 74 |
| [admin/email/settings/+page.svelte:102](../../../frontend/src/routes/admin/email/settings/+page.svelte#L102) | `apiFetch('/admin/email/settings/test', POST)` | 102 |
| [admin/email/smtp/+page.svelte:33,45,73](../../../frontend/src/routes/admin/email/smtp/+page.svelte#L33) | mirror copies of the above | |
| [admin/email/templates/edit/[id]/+page.svelte:15](../../../frontend/src/routes/admin/email/templates/edit/[id]/+page.svelte#L15) | `apiFetch('/admin/email/templates/${id}')` | 15 |
| [admin/email/templates/preview/[id]/+page.svelte:16](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte#L16) | `apiFetch('/admin/email/templates/${id}/preview')` | 16 |

`apiFetch` resolves these to `https://revolution-trading-pros-api.fly.dev/api/admin/...` — the SK admin proxies at the same path are bypassed.

Mitigations the team should consider:
- Replace these with `fetch('/api/admin/...')` (same-origin, hits the proxy).
- Or have `apiFetch` short-circuit `/admin/*` to same-origin like `lib/api/admin.ts` already does ([admin.ts:540](../../../frontend/src/lib/api/admin.ts#L540) — `apiEndpoint = endpoint.startsWith('/api/') ? endpoint : '/api${endpoint}'`).

---

## E. `console.log` noise

**Result**: 7 occurrences. All `console.error` / `console.warn` left in are not flagged here per spec.

| File | Line | Context |
|------|------|---------|
| [admin/courses/create/+page.svelte:1520](../../../frontend/src/routes/admin/courses/create/+page.svelte#L1520) | 1520 | `console.log('Notification:', message);` |
| [admin/users/create/+page.svelte:1004](../../../frontend/src/routes/admin/users/create/+page.svelte#L1004) | 1004 | `console.log('Sending notifications for user:', user);` |
| [admin/users/create/+page.svelte:1009](../../../frontend/src/routes/admin/users/create/+page.svelte#L1009) | 1009 | `console.log('Tracking user creation:', user);` |
| [admin/users/create/+page.svelte:1052](../../../frontend/src/routes/admin/users/create/+page.svelte#L1052) | 1052 | `console.log('Success:', message);` |
| [api/admin/bunny/video-status/[guid]/+server.ts:37](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts#L37) | 37 | `console.log(\`[Bunny API] Fetching: ${BACKEND_URL}/...\`);` |
| [api/admin/bunny/upload/+server.ts:46](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L46) | 46 | `console.log(\`[Bunny Upload] Uploading ${fileBuffer.byteLength} bytes...\`);` |
| [api/admin/email/settings/+server.ts:59](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L59) | 59 | `console.log('Email settings updated successfully');` |

Per-cluster:
- `admin/users` — 3
- `admin/courses` — 1
- `api/admin` — 3
- All other 22 admin clusters — 0

Bunny logs leak the backend URL into logs — low severity but they belong behind a `[bunny]` debug flag, not unconditional `console.log`.

---

## F. `as any` and `as unknown as` casts

### F.1 — `as any` (41 occurrences)

| File | Line | Notes |
|------|------|-------|
| [admin/forms/create/+page.svelte:37](../../../frontend/src/routes/admin/forms/create/+page.svelte#L37) | 37 | `})) as any,` |
| [admin/trading-rooms/[slug]/+page.svelte:557-567](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte#L557) | 557–567 | 11× `(alert as any).field` — clearly an `Alert` shape mismatch. Should be a proper interface. |
| [admin/crm/deals/+page.svelte:239](../../../frontend/src/routes/admin/crm/deals/+page.svelte#L239) | 239 | `{ status: 'abandoned' } as any` |
| [admin/crm/deals/[id]/+page.svelte:233](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte#L233) | 233 | same |
| [admin/crm/automations/new/+page.svelte:52](../../../frontend/src/routes/admin/crm/automations/new/+page.svelte#L52) | 52 | `conditions: [] as any[]` |
| [admin/blog/edit/[id]/+page.svelte:67](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte#L67) | 67 | `content_blocks: [] as any[]` |
| [admin/blog/edit/[id]/+page.svelte:230](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte#L230) | 230 | `block.content as any` |
| [admin/blog/create/+page.svelte:60](../../../frontend/src/routes/admin/blog/create/+page.svelte#L60) | 60 | `content_blocks: [] as any[]` |
| [admin/blog/create/+page.svelte:166](../../../frontend/src/routes/admin/blog/create/+page.svelte#L166) | 166 | `block.content as any` |
| [admin/dashboard/+page.svelte:83](../../../frontend/src/routes/admin/dashboard/+page.svelte#L83) | 83 | `(connections as any)[conn.key] = true;` |
| [admin/courses/create/+page.svelte:166](../../../frontend/src/routes/admin/courses/create/+page.svelte#L166) | 166 | `resources: [] as any[]` |
| [admin/popups/new/+page.svelte:1155,1165](../../../frontend/src/routes/admin/popups/new/+page.svelte#L1155) | 1155, 1165 | `bind:value={(formData as any).start_date}` |
| [admin/popups/[id]/edit/+page.svelte:712-741](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte#L712) | 712, 722, 728, 732, 734, 736, 737, 738, 740, 741 | 10× `(formData as any).start_date / .end_date`. Form type missing two fields. |
| [admin/users/create/+page.svelte:520](../../../frontend/src/routes/admin/users/create/+page.svelte#L520) | 520 | `(submitData as any).profile_photo = photoUrl;` |
| [admin/boards/[id]/+page.svelte:1129](../../../frontend/src/routes/admin/boards/[id]/+page.svelte#L1129) | 1129 | `(e.currentTarget as HTMLSelectElement).value as any` |
| [admin/seo/404s/+page.svelte:49](../../../frontend/src/routes/admin/seo/404s/+page.svelte#L49) | 49 | `stats = response as any;` |
| [admin/email/smtp/+page.svelte:33](../../../frontend/src/routes/admin/email/smtp/+page.svelte#L33) | 33 | `(await apiFetch(...)) as any` |
| [admin/email/settings/+page.svelte:58](../../../frontend/src/routes/admin/email/settings/+page.svelte#L58) | 58 | `(await apiFetch(...)) as any` |
| [admin/analytics/cohorts/+page.svelte:65](../../../frontend/src/routes/admin/analytics/cohorts/+page.svelte#L65) | 65 | `cohorts = (response.cohorts \|\| []) as any;` |
| [admin/analytics/attribution/+page.svelte:240](../../../frontend/src/routes/admin/analytics/attribution/+page.svelte#L240) | 240 | `(report.conversion_paths as any).length` |
| [admin/media/+page.svelte:1509](../../../frontend/src/routes/admin/media/+page.svelte#L1509) | 1509 | `(detailItem.variants as any) \|\| []` |

### F.2 — `as unknown as` (2 occurrences)

| File | Line | Code |
|------|------|------|
| [admin/courses/+page.svelte:239](../../../frontend/src/routes/admin/courses/+page.svelte#L239) | 239 | `selectedCourse = course as unknown as APICourse;` |
| [admin/email/templates/+page.svelte:62](../../../frontend/src/routes/admin/email/templates/+page.svelte#L62) | 62 | `templates = (response.data as unknown as EmailTemplate[]) \|\| [];` |

Pattern observation: the heaviest concentrations (11 in `trading-rooms/[slug]`, 10 in `popups/[id]/edit`) point at a missing field on a single shared type — fixing the type definition kills the whole cluster in one edit.

---

## G. Missing `await parent()` in load functions

`grep -rn "await parent" frontend/src/routes/admin/` returns **0**. There are 8 load files:

| File | Behaviour |
|------|-----------|
| [admin/+layout.ts](../../../frontend/src/routes/admin/+layout.ts) | only sets `ssr=false; prerender=false;` — no data |
| [admin/consent/+page.ts](../../../frontend/src/routes/admin/consent/+page.ts) | only flags |
| [admin/forms/[id]/edit/+page.ts](../../../frontend/src/routes/admin/forms/[id]/edit/+page.ts) | only flags |
| [admin/forms/[id]/submissions/+page.ts](../../../frontend/src/routes/admin/forms/[id]/submissions/+page.ts) | only flags |
| [admin/forms/[id]/analytics/+page.ts](../../../frontend/src/routes/admin/forms/[id]/analytics/+page.ts) | only flags |
| [admin/consent/templates/+page.ts](../../../frontend/src/routes/admin/consent/templates/+page.ts) | only flags |
| [admin/email/campaigns/[id]/report/+page.ts](../../../frontend/src/routes/admin/email/campaigns/[id]/report/+page.ts) | reads `params.id`, returns it |
| [admin/members/past/+page.ts](../../../frontend/src/routes/admin/members/past/+page.ts) | full `Load` that hits 4 API endpoints in parallel; depends on `authStore.getToken()` from a Svelte store, **not** parent data |

Since `+layout.ts` exposes no data into `$page.data` and pages don't merge with parent state, the absence of `await parent()` is technically correct. **No defects**, but the auth check in `members/past` should be moved into the layout to avoid every admin sub-page reinventing it. That's a backlog item, not a Section G violation.

---

## H. Orphan API proxies (no caller)

A "true orphan" is a `+server.ts` whose path is not referenced anywhere in `frontend/src/` outside the proxy itself. Comprehensive caller-string scan against all 53 proxies yields:

| # | Proxy | Status |
|---|-------|--------|
| 1 | [api/admin/bunny/upload/+server.ts](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts) | **ORPHAN** — only mentioned in its own JSDoc. (Distinct from `bunny/uploads`.) |
| 2 | [api/admin/connections/status/+server.ts](../../../frontend/src/routes/api/admin/connections/status/+server.ts) | **ORPHAN** — and unauthenticated (see C.2). |
| 3 | [api/admin/email/settings/+server.ts](../../../frontend/src/routes/api/admin/email/settings/+server.ts) | **EFFECTIVELY ORPHAN** — pages call this path through `apiFetch` which goes to the backend directly, never via the SK proxy (see Section D). The proxy is reachable but no caller actually addresses `/api/admin/email/settings`. Also unauthenticated (see C.2). |
| 4 | [api/admin/member-management/+server.ts](../../../frontend/src/routes/api/admin/member-management/+server.ts) | **EFFECTIVELY ORPHAN** — only `lib/api/members.ts:435` calls `/api/admin/member-management/export` which falls through to the global catch-all `frontend/src/routes/api/[...path]/+server.ts` since `export` isn't covered by this proxy and there's no `[...rest]` shim. |
| 5 | [api/admin/member-management/[id]/+server.ts](../../../frontend/src/routes/api/admin/member-management/[id]/+server.ts) | **ORPHAN** — no caller anywhere uses `/api/admin/member-management/<id>`. |
| 6 | [api/admin/email/templates/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/email/templates/[...rest]/+server.ts) | **NOT ORPHAN, BUT BYPASSED** — exists to cover `/api/admin/email/templates/<id>` but the live call sites use `apiFetch` which goes direct to backend (Section D). |

The remaining 47 proxies all have at least one direct or `api.method()`-style caller.

---

## I. Pages calling non-existent proxies (dead links)

A "dead link" is a frontend `fetch('/api/admin/<path>')` (or equivalent) for which **no** `+server.ts` claims that path AND the global catch-all [`api/[...path]/+server.ts`](../../../frontend/src/routes/api/[...path]/+server.ts) won't be reached. Per the documented SvelteKit POST=405 cliff (see [createProxyShim.ts:1-30](../../../frontend/src/lib/utils/createProxyShim.ts#L1)), **a sub-path is blocked from the catch-all if its parent folder owns its own `+server.ts` AND no `[...rest]` sibling exists**.

Cross-referencing every fetched admin URL against the proxy tree:

### I.1 — Confirmed dead (parent owns `+server.ts`, no `[...rest]` sibling exists)

These will return 405 for POST/PUT/PATCH/DELETE in dev and likely also in prod:

| Caller | URL pattern | Why dead |
|--------|-------------|----------|
| [lib/api/courses.ts:450,457,472,483,491](../../../frontend/src/lib/api/courses.ts#L450) | `/api/admin/courses/${id}/modules*` | parent `courses/[id]/+server.ts` exists, no `[...rest]` |
| [lib/api/courses.ts:502,512,519,534,545,553](../../../frontend/src/lib/api/courses.ts#L502) | `/api/admin/courses/${id}/lessons*` | same |
| [lib/api/courses.ts:564,571,586,597](../../../frontend/src/lib/api/courses.ts#L564) | `/api/admin/courses/${id}/downloads*` | same |
| `lib/api/courses.ts` (search for `upload-url`, `video-upload`) | `/api/admin/courses/${id}/upload-url`, `/video-upload` | same |
| [lib/api/admin.ts:1098](../../../frontend/src/lib/api/admin.ts#L1098) | `/api/admin/forms/${id}/submissions/export` | no `forms/+server.ts` at all in proxy tree — falls through to catch-all (NOT dead, but no dedicated handler exists) |

The course sub-routes are the most visible failure mode in this category — they exactly mirror the pattern that motivated the 7 existing `[...rest]` shims, but `courses/` was never given one. Fix: drop a `frontend/src/routes/api/admin/courses/[id]/[...rest]/+server.ts` calling `createProxyShim('/api/admin/courses')`.

### I.2 — Falls through the global catch-all (NOT dead)

The vast majority of "missing proxy" hits (e.g. `/api/admin/members/analytics/*`, `/api/admin/organization/*`, `/api/admin/subscriptions/plans*`, `/api/admin/user-memberships*`, `/api/admin/seo/metrics`, `/api/admin/abandoned-carts`, `/api/admin/dashboard`, `/api/admin/site-health`, `/api/admin/sharp/upload`, `/api/admin/watchlist`, `/api/admin/workflows/*`, etc.) successfully reach the backend through [api/[...path]/+server.ts](../../../frontend/src/routes/api/[...path]/+server.ts). They are not dead.

### I.3 — Mismatched method coverage

[api/admin/orders/+server.ts](../../../frontend/src/routes/api/admin/orders/+server.ts) only handles GET; POST/PUT/PATCH/DELETE are needed for create/update operations. The newly-added [orders/[...rest]/+server.ts](../../../frontend/src/routes/api/admin/orders/[...rest]/+server.ts) covers child paths but NOT the bare `/api/admin/orders` POST. This is a method gap, not a path gap.

Same shape: [api/admin/posts/+server.ts](../../../frontend/src/routes/api/admin/posts/+server.ts), [tags/+server.ts](../../../frontend/src/routes/api/admin/tags/+server.ts), [categories/+server.ts](../../../frontend/src/routes/api/admin/categories/+server.ts), [coupons/+server.ts](../../../frontend/src/routes/api/admin/coupons/+server.ts), [users/+server.ts](../../../frontend/src/routes/api/admin/users/+server.ts) — verify each handles the full CRUD surface its callers expect.

---

## Top 10 highest-priority cross-cutting fixes (ranked)

1. **[CRITICAL] Auth bypass on `email/settings` proxy.** [api/admin/email/settings/+server.ts](../../../frontend/src/routes/api/admin/email/settings/+server.ts) GET/PUT have no `cookies.get('rtp_access_token')` check and update an in-memory SMTP config object. Add the canonical token gate, or delete the proxy entirely if it's superseded by the backend (see #2 below).

2. **[HIGH] Five admin pages bypass the SK proxy via `apiFetch`.** [admin/email/settings/+page.svelte](../../../frontend/src/routes/admin/email/settings/+page.svelte), [admin/email/smtp/+page.svelte](../../../frontend/src/routes/admin/email/smtp/+page.svelte), [admin/email/templates/edit/[id]/+page.svelte](../../../frontend/src/routes/admin/email/templates/edit/[id]/+page.svelte), [admin/email/templates/preview/[id]/+page.svelte](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte) call `apiFetch('/admin/...')` which resolves to the prod fly.dev URL directly. Either patch `apiFetch` to short-circuit `/admin/*` to same-origin (mirroring `lib/api/admin.ts:540`) or replace these call sites with `fetch('/api/admin/...')`.

3. **[HIGH] Add `[...rest]` shim for `courses/[id]/`.** Every callsite in [lib/api/courses.ts:450-597](../../../frontend/src/lib/api/courses.ts#L450) for modules / lessons / downloads / upload-url / video-upload will hit the SK 405 cliff for non-GET methods. Mirror the pattern used for `posts`, `users`, `email/templates`, etc.

4. **[HIGH] Auth bypass on `connections/status` proxy.** Add the standard token gate to [api/admin/connections/status/+server.ts](../../../frontend/src/routes/api/admin/connections/status/+server.ts).

5. **[MEDIUM] Bunny proxies miss `env.API_BASE_URL` fallback.** Four files in [api/admin/bunny/](../../../frontend/src/routes/api/admin/bunny/) use only `env.BACKEND_URL`. Align with the canonical pattern.

6. **[MEDIUM] Method-coverage gaps.** Audit [orders/+server.ts](../../../frontend/src/routes/api/admin/orders/+server.ts), [posts/+server.ts](../../../frontend/src/routes/api/admin/posts/+server.ts), [tags/+server.ts](../../../frontend/src/routes/api/admin/tags/+server.ts), [categories/+server.ts](../../../frontend/src/routes/api/admin/categories/+server.ts), [coupons/+server.ts](../../../frontend/src/routes/api/admin/coupons/+server.ts), [users/+server.ts](../../../frontend/src/routes/api/admin/users/+server.ts) — each has a `[...rest]` shim for child paths, but the bare-path proxies need the full HTTP-verb set their callers use.

7. **[MEDIUM] Fix the `popups` and `trading-rooms/[slug]` form types.** 11+10 = 21 of the 41 `as any` casts in admin code stem from one missing field on `Alert` and two missing fields on `Popup` form-data. One type fix erases ~half the unsafe casts.

8. **[MEDIUM] Decide the `member-management` proxy's fate.** Both [api/admin/member-management/+server.ts](../../../frontend/src/routes/api/admin/member-management/+server.ts) and [api/admin/member-management/[id]/+server.ts](../../../frontend/src/routes/api/admin/member-management/[id]/+server.ts) are orphan/effectively-orphan. Either wire them to a real caller or comment them out with a TODO removal date (per the user's "comment-out, don't delete" rule).

9. **[LOW] Unify auth-token variable naming.** Standardize on `cookieToken` throughout — the existing 43-of-43 majority — and migrate the `token` / `accessToken` / `session` outliers in `connections/*`, `courses/*`, `consent/settings/*`, `orders/[id]`, `bunny/*`. Pure consistency win.

10. **[LOW] Drop dead `console.log` calls and the redundant `BACKEND_URL = PROD_BACKEND` rebindings.** 7 console.logs (Section E) plus ~25 redundant `const BACKEND_URL = PROD_BACKEND;` rebindings (Section A.2). Either purge or hide behind a single shared `const BACKEND = ...` re-export.

---

## Summary

The admin surface has held the line on the two "famous" 2026-04-25 fixes:

- **Hardcoded backend URLs**: 49/53 canonical, 4 cosmetic deviations under `bunny/`. Zero hardcoded URLs reintroduced. ✓
- **Svelte 5 shadow-state**: zero occurrences in 151 admin .svelte / .ts files. ✓

The most important new findings are *not* about hardcoded URLs:

1. Two unauthenticated proxies (`email/settings`, `connections/status`) — the first one is genuinely high-severity because it stores SMTP credentials in module scope.
2. Five admin pages bypass the SK proxy entirely via `apiFetch` and hit the prod backend directly (`email/settings`, `email/smtp`, `email/templates/{edit,preview}`).
3. A whole sub-tree (`courses/[id]/{modules,lessons,downloads,upload-url,video-upload}`) lacks a `[...rest]` shim and will 405 on POST/PUT/PATCH — the same documented cliff that the 7 existing shims were created to fix.
4. The `bunny/` cluster uses only `env.BACKEND_URL` instead of the documented `env.API_BASE_URL || env.BACKEND_URL` fallback chain.

Type-safety hygiene is uneven: 41 `as any` casts cluster in `popups/` (12), `trading-rooms/[slug]` (11), and `blog/` (4), and roughly half could be eliminated with two type-definition fixes.

The auth pattern is cosmetically inconsistent (3 different variable names, 12 commented-out legacy reads) but functionally correct in 51 of 53 proxies.

`await parent()`: not relevant on this surface — the admin layout exposes no data.

Orphan proxies: 5 truly orphan (`bunny/upload`, `connections/status`, `email/settings`, `member-management`, `member-management/[id]`) plus 1 bypassed (`email/templates/[...rest]`). All are candidates for either rewiring or comment-out-with-TODO.
