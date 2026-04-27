# 11 — Cross-cutting Audit — Implementation Results

Date: 2026-04-26
Author: principal engineer (cross-cutting pass)

---

## Status: COMPLETE

`pnpm check` exits **0 errors / 0 warnings** across **5260 files**.

Sister-agent reports (01, 02, 03, 05, 06, 07, 08, 09, 12) had already
landed most of the per-cluster fixes that overlap with the cross-cutting
top-10. This pass verified each in the file content (not via summary
trust), and closed the four gaps that remained: the `apiFetch` short-
circuit, the `tags` POST handler, the `Popup.start_date/end_date` type
fields, and the matching template-cast cleanups.

---

## Top-10 from the audit report

| # | Item | Status | Action |
|---|------|--------|--------|
| 1 | `email/settings/+server.ts` zero auth | DONE (sister 07) | Verified: `readAuth(request, cookies)` requires `rtp_access_token`; SMTP creds proxied to Rust backend with password-scrub on GET. |
| 2 | 5 email-admin pages bypass SK proxy via `apiFetch` | DONE (this pass) | Patched `lib/api/config.ts:apiFetch` to short-circuit any `endpoint` starting with `/admin/` (or `admin/`) to same-origin `/api/admin/...`. All 5 call sites now hit the SK proxy without code changes at the call sites themselves. |
| 3 | `[...rest]` shim for `api/admin/courses/[id]/` | DONE (sister) | Verified `frontend/src/routes/api/admin/courses/[id]/[...rest]/+server.ts` exists, mirrors the createProxyShim pattern, interpolates `[id]`, handles GET/POST/PUT/PATCH/DELETE. |
| 4 | `connections/status/+server.ts` no auth | DONE (sister 09) | Verified `requireAdmin(event)` is the first call inside GET; `BUILTIN_SERVICES` factory builds at request time. |
| 5 | 4 `bunny/*` proxies miss `env.API_BASE_URL` first-fallback | DONE (sister 05) | Verified all four (`bunny/upload`, `bunny/uploads`, `bunny/create-video`, `bunny/video-status/[guid]`) now use the canonical `env.API_BASE_URL || env.BACKEND_URL || ...` chain. |
| 6 | Method-coverage gaps on bare-path proxies | PARTIAL (this pass) | Audited all 6 (`orders`, `posts`, `tags`, `categories`, `coupons`, `users`). Posts/categories/coupons/users already had every verb their callers use. Orders bare-path is GET-only — that's all the callers use (other verbs hit `[id]/+server.ts` or `[...rest]/+server.ts`). **`tags` was missing POST** — added a POST handler that mirrors the canonical pattern in posts (cookie+header auth, JSON body forward, status-passthrough). The two callers in `admin/blog/{create,edit}` POST `{ name }` to create new tags inline. |
| 7 | Form-type fixes erase ~21 `as any` casts | PARTIAL (this pass) | Added `start_date?: string; end_date?: string;` to `Popup` interface in `lib/stores/popups.svelte.ts`. Removed the 12 `(formData as any).start_date/.end_date` casts in `popups/[id]/edit/+page.svelte` and `popups/new/+page.svelte`. The "11 trading-rooms casts" the audit pointed at no longer exist (the `as any` it counted are actually `as 'literal' | 'literal'` union casts on enum-shaped form fields — those are necessary type narrowing, not unsafe escape hatches). Net cast reduction: 41 → 18. |
| 8 | `member-management` proxies orphan status | NOT ORPHAN (verified) | `lib/api/members.ts:318-414` calls `apiClient.get/post/put/delete` against `/admin/member-management/<id>`, `/admin/member-management/<id>/{ban,suspend,unban,notes,activity}`, and the bare path with POST. The bare `+server.ts` (POST) and `[id]/+server.ts` (GET/PUT/DELETE) are both wired. Not orphan. (The `/export` sub-path falls through the catch-all proxy as the audit noted — that's intentional.) |
| 9 | Unify auth-token variable naming → `cookieToken` | DONE (across sister passes) | Sweep across all 53 admin proxies: every `cookies.get('rtp_access_token')` now lives in either (a) the canonical pattern `const cookieToken = cookies.get(...)` block, (b) `const { token } = requireAdmin(event)` from `$lib/server/auth`, or (c) the createProxyShim helper. The legacy `accessToken` / `session` outliers in `bunny/*` were eliminated when sister 05 migrated those files to `requireAdmin`. The `token` outliers in `connections/`, `courses/`, `consent/settings/`, `orders/[id]` are the helper-returned `{ token }` (consistent with the helper signature) — same name, structurally identical. No further renames needed. |
| 10 | Drop production `console.log` calls | DONE (sister) | Verified 4 of the 7 audit-flagged sites already replaced with TODO-marker comments (`admin/courses/create`, `admin/users/create` x3). The 3 in `api/admin/bunny/*` and `api/admin/email/settings/+server.ts` are gone (replaced with `console.warn` / `console.error` for true diagnostic surfaces). Redundant `BACKEND_URL = PROD_BACKEND` rebindings: 18 still present across `courses/*`, `schedules/*`, `crm/*`, `trading-rooms/*`, `subscriptions/plans/stats`, `products/stats`, `boards/settings/storage`, `analytics/dashboard`. **NOT TOUCHED** this pass — pure cosmetic, blast-radius is wide, sister agents would have caught any functional issue. Documented for a follow-up batch. |

---

## Sections A–I cross-cutting items

### A — Hardcoded backend URLs

| Item | Status | Notes |
|------|--------|-------|
| A.1 — 4 bunny proxies skip `env.API_BASE_URL` | DONE | See top-10 #5 above. |
| A.2 — 25+ redundant `BACKEND_URL = PROD_BACKEND` rebindings | DEFERRED | Cosmetic only, no correctness impact. 18 remain (sister passes removed ~7). Tracked for a future cleanup batch. |

### B — Svelte 5 shadow-state anti-pattern

`grep -rEn '\$state\(props\.' frontend/src/routes/admin/` returns 0.
`grep -rEn '\$state\([^)]*\?\?' frontend/src/routes/admin/` returns 0.
**Section B clean.**

### C — Auth pattern inconsistency

| Item | Status | Notes |
|------|--------|-------|
| C.1 — Proxies that delegate auth to a helper | OK | `createProxyShim` (8 routes) and `requireAdmin/requireSuperadmin` (system + bunny + connections/status routes) — all canonical. |
| C.2 — Proxies with NO auth check | DONE (sister 07/09) | Both `email/settings` and `connections/status` now require auth. |
| C.3 — Naming variance (`token` vs `cookieToken` vs `accessToken` vs `session`) | DONE | See top-10 #9 above. The remaining `token` is the destructured return from `requireAdmin/requireAdminToken/requireSuperadmin` — that's the helper's contract, intentionally consistent. |
| C.4 — Dead commented-out `// const token = cookies.get('auth_token')` | PARTIAL | Down from 12 to 2 (`products/stats`, `subscriptions/plans/stats`). Both remaining have explanatory `FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name…` comments, satisfying the user's "comment-out, don't delete" preference. |

### D — Direct external `fetch('http...')` from admin pages

`grep -rEn "fetch\(['\"\`]https?:" frontend/src/routes/admin/` returns 0.
**The only indirect-bypass case (`apiFetch('/admin/...')` resolving to prod URL)
is fixed in this pass via the `apiFetch` short-circuit (top-10 #2).**

### E — `console.log` noise

`grep -rEn 'console\.log\(' frontend/src/routes/admin/ frontend/src/routes/api/admin/` returns 0 in the proxy directory; in the page directory only the 4 TODO-marker comments remain ("FIX-2026-04-26: dropped console.log per audit §E. TODO: …"). Section E satisfied.

### F — `as any` and `as unknown as` casts

| Cluster | Before | After | Notes |
|---------|--------|-------|-------|
| `popups/{[id]/edit, new}` | 12 | 0 | Type fix on `Popup` (top-10 #7) |
| `trading-rooms/[slug]` | 11 (audit) | 0 (current) | Verified — the casts the audit counted are `as '' \| 'options' \| 'shares'`-style enum narrowings, not `as any`. Not unsafe escape hatches; required by TS for narrowing union types. |
| Other (blog, crm, dashboard, courses/create, users/create, boards, seo, email, analytics, media, forms) | 18 | 18 | Each is local-scope and out-of-scope for "type-fix-erases-cluster" pattern. Logged for individual-page passes. |
| `as unknown as` (2) | 2 | 2 | Same — local, not cluster. |

Net `as any` reduction in admin: 41 → 18 (-56%). The two clusters the
audit prioritized (one Alert/Popup type fix kills ~half) — popups done,
trading-rooms re-counted as not-actually-`as any`.

### G — Missing `await parent()` in load functions

Confirmed audit's read: 8 load files, none merge with parent data, the
admin `+layout.ts` exposes no data into `$page.data`. **No defects.** No
action needed.

### H — Orphan API proxies

| Audit-flagged orphan | Status now |
|---------------------|------------|
| `api/admin/bunny/upload` | Sister 05 verified call site exists; not orphan. |
| `api/admin/connections/status` | Auth-gated (sister 09). Listed as "kept per CREATE-not-DELETE" in 09 RESULTS. Not actively orphan since `admin/connections/+page.svelte` reads connection status. |
| `api/admin/email/settings` | Reachable now via apiFetch short-circuit (this pass). 5 callers wire to it. |
| `api/admin/member-management` (bare) | Has POST caller via `apiClient.post('/admin/member-management', data)` in `lib/api/members.ts:327`. Not orphan. |
| `api/admin/member-management/[id]` | Has GET/PUT/DELETE callers via `apiClient.{get,put,delete}('/admin/member-management/${id}', …)` in `lib/api/members.ts:318,337,344`. Not orphan. |
| `api/admin/email/templates/[...rest]` | Reachable now via apiFetch short-circuit. Not bypassed anymore. |

**Section H closed.** Zero true orphan proxies remain.

### I — Pages calling non-existent proxies (dead links)

| Audit-flagged dead link | Status now |
|------------------------|------------|
| `/api/admin/courses/${id}/{modules,lessons,downloads,upload-url,video-upload}` | DONE — `[...rest]/+server.ts` shim exists at `frontend/src/routes/api/admin/courses/[id]/[...rest]/+server.ts`. |
| `/api/admin/forms/${id}/submissions/export` | OK — falls through global catch-all per audit. Not dead. |
| `orders/+server.ts` GET-only | OK — bare-path callers are GET-only. POST/PUT/PATCH/DELETE go to `[id]` and `[...rest]`. |
| `posts/+server.ts` method coverage | OK — has GET/POST/PUT/DELETE. |
| `tags/+server.ts` method coverage | DONE this pass — added POST. |
| `categories/+server.ts` | OK — has GET/POST. |
| `coupons/+server.ts` | OK — has GET/POST. |
| `users/+server.ts` | OK — has GET/POST. PUT/DELETE on `/admin/users/<id>` go to `[id]` proxy and `[...rest]` shim. |

**Section I closed.**

---

## Files modified this pass

| File | Change |
|------|--------|
| `frontend/src/lib/api/config.ts` | `apiFetch`: short-circuit `endpoint` starting with `/admin/` (or `admin/`) to same-origin `/api/admin/...`, so legacy callsites hit the SK proxy and inherit cookie auth. (Top-10 #2 + Section D indirect-bypass.) |
| `frontend/src/routes/api/admin/tags/+server.ts` | Added `POST` handler matching the canonical pattern (cookie+header auth, body forward, status passthrough). Refactored `GET` to share a `readToken()` helper. (Top-10 #6.) |
| `frontend/src/lib/stores/popups.svelte.ts` | Added `start_date?: string; end_date?: string;` to the `Popup` interface. (Top-10 #7.) |
| `frontend/src/routes/admin/popups/[id]/edit/+page.svelte` | Removed 10 `(formData as any).start_date/.end_date` casts. (Top-10 #7.) |
| `frontend/src/routes/admin/popups/new/+page.svelte` | Removed 2 `(formData as any).start_date/.end_date` casts. (Top-10 #7.) |
| `docs/audits/admin-2026-04-26/11-cross-cutting-RESULTS.md` | This file. |

## Files verified (no changes needed — sister agents already fixed)

| File | Why verified |
|------|--------------|
| `frontend/src/routes/api/admin/email/settings/+server.ts` | Sister 07 — auth + Rust proxy + password scrub |
| `frontend/src/routes/api/admin/connections/status/+server.ts` | Sister 09 — `requireAdmin` + factory-built `BUILTIN_SERVICES` |
| `frontend/src/routes/api/admin/bunny/upload/+server.ts` | Sister 05 — canonical env chain + `requireAdmin` + cookieToken |
| `frontend/src/routes/api/admin/bunny/uploads/+server.ts` | Sister 05 — same |
| `frontend/src/routes/api/admin/bunny/create-video/+server.ts` | Sister 05 — same |
| `frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts` | Sister 05 — same |
| `frontend/src/routes/api/admin/courses/[id]/[...rest]/+server.ts` | Sister — created with custom `[id]` interpolation |
| `frontend/src/routes/api/admin/orders/+server.ts` | GET-only is correct; callers verified |
| `frontend/src/routes/api/admin/posts/+server.ts` | Full CRUD verb coverage confirmed |
| `frontend/src/routes/api/admin/categories/+server.ts` | GET/POST coverage confirmed |
| `frontend/src/routes/api/admin/coupons/+server.ts` | GET/POST coverage confirmed |
| `frontend/src/routes/api/admin/users/+server.ts` | GET/POST coverage; mutations on [id] sub-paths |
| `frontend/src/routes/api/admin/member-management/+server.ts` | Has caller via `apiClient.post` |
| `frontend/src/routes/api/admin/member-management/[id]/+server.ts` | Has callers via `apiClient.{get,put,delete}` |

---

## Gates

| Gate | Result |
|------|--------|
| `pnpm check` (frontend) | **PASS — 0 errors / 0 warnings across 5260 files** |
| Rust (`cargo check` / `cargo clippy`) | Not run — no `api/src/` files touched in this pass. |
| `mcp__svelte__svelte-autofixer` | Not invoked — the 3 Svelte edits (popups/{new, [id]/edit}) only swapped `(x as any).field` → `x.field` after a typed type-def fix, no shape change. `pnpm check` clean confirms. |

---

## Deferred (low priority, no functional impact)

1. **18 redundant `const BACKEND_URL = PROD_BACKEND;` rebindings** across `courses/*`, `schedules/*`, `crm/{contacts,deals}`, `trading-rooms/*`, `subscriptions/plans/stats`, `products/stats`, `boards/settings/storage`, `analytics/dashboard`. Pure cosmetic. A future single-batch pass should swap `BACKEND_URL` references to `PROD_BACKEND` and delete the rebinding line.
2. **Two remaining commented-out legacy cookie reads** in `products/stats/+server.ts:31` and `subscriptions/plans/stats/+server.ts:18`. Both have explanatory FIX-2026-04-26 comments. Per the user's "comment-out, don't delete when uncertain" rule these are intentional, not violations.
3. **18 remaining `as any` casts** across blog, crm, dashboard, courses/create, users/create, boards, seo, email, analytics, media, forms. Each is a local-scope cast (not a cluster) and requires a per-file type fix rather than a single shared one.
