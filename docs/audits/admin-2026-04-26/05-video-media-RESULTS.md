# 05 — Video / Media / Trading Rooms / Indicators — RESULTS

Date: 2026-04-26
Source audit: [`05-video-media.md`](./05-video-media.md)
Caveat: **Bunny.net is currently down.** The fixes below preserve Bunny as the
correct provider; nothing was swapped or removed. Items that exercise live
Bunny endpoints cannot be E2E-verified until the account is back.

---

## Status of every P0 / P1 / P2 / P3 item

### P0 — already addressed by sister agents before this run

| Item | Status | Where |
|---|---|---|
| P0-1 `VIDEO_LIBRARY_ID` literal in embed URL | DONE earlier | `WeeklyVideoUploader.svelte:238-240` (`${libraryId}` interpolation, with `embedUrlFromBackend` preference) |
| P0-2 non-existent `upload_url` field | DONE earlier | `WeeklyVideoUploader.svelte:188-235` rewritten to use the `/api/admin/bunny/upload?video_guid=...&library_id=...` proxy |
| P0-3 weekly-uploader bypassed auth | DONE earlier | now uses `adminFetch` (Bearer header) for create-video; XHR for the upload PUT carries `withCredentials: true` |
| P0-4 (audit) / P0-8 (work order) `/api/admin/indicators/*` orphan | DONE earlier | proxies present at `routes/api/admin/indicators/+server.ts`, `[id]/+server.ts`, `[id]/[...rest]/+server.ts` — all use `env.API_BASE_URL || env.BACKEND_URL || …` and Bearer header |
| P0-5 (audit) / P0-6 (work order) integer-index `platform_id` | DONE earlier | `indicators/create/+page.svelte:566` sends `pf.platform_id` (slug string) |
| P0-6 (audit) / P0-5 (work order) videos triple-fire `loadVideos()` | DONE earlier | consolidated to single `onMount` at `videos/+page.svelte:961-968` |
| P0-7 (audit) / P0-4 (work order) 6 shadow-state `$effect` blocks | DONE earlier | removed with explanatory note at `trading-rooms/[slug]/+page.svelte:311-317`; SSR data is seeded once via `untrack()` in the state declarations |
| P0-8 (audit) / P0-3 (work order) wrong cookie name `session_token` | DONE earlier | `trading-rooms/[slug]/+page.server.ts:35` reads `rtp_access_token` and forwards `Authorization: Bearer …` to all six SSR fetches |
| P0-9 (audit) / P0-7 (work order) `bunny/uploads` cookie name & lying success | DONE earlier | `bunny/uploads/+server.ts` now uses `Authorization: Bearer …`, `env.API_BASE_URL` chain, and surfaces real backend errors as 502 |
| P0-9 (work order) `bunny/video-status` swallowed errors | DONE earlier | `bunny/video-status/[guid]/+server.ts` propagates upstream status & error |

### P1 — addressed in this run

| Item | Status | What changed |
|---|---|---|
| P1-4 progress XHR fires hundreds of HTTP requests per upload | FIXED | `videos/+page.svelte` `uploadFileToBunny()` — throttled to ~1 update/sec with a trailing-flush guarantee so the bar still reaches 100. No external dependencies. |
| P1-11 mock-trader / mock-video silent fallbacks | DONE earlier | `trading-rooms/traders/+server.ts` and `trading-rooms/videos/+server.ts` now return `{ success: false, … }` with 502 instead of phantom mock data |
| P1-1 video-status swallows errors | DONE earlier | see P0-9 row above |

### P2 — addressed in this run

| Item | Status | What changed |
|---|---|---|
| P2-1 analytics-period `$effect` double-fetches on first open | FIXED | `videos/+page.svelte` — `toggleAnalyticsPanel()` no longer calls `loadAnalytics()` directly; the existing `$effect` on `(showAnalyticsPanel, analyticsPeriod)` is the single fetch path |
| P2-4 `roomSlugToId` typo `'explosive-swing'` vs `'explosive-swings'` | FIXED | `api/admin/trading-rooms/videos/[slug]/+server.ts` — added the canonical `'explosive-swings': 5` and kept the singular as a legacy alias with a TODO marker |
| P2-8 slug auto-generation clobbers user edits | FIXED | `indicators/create/+page.svelte` — added `slugEdited` flag; `$effect` only auto-syncs the slug while the flag is `false`. Added an `onSlugInput()` helper for the eventual editable slug input. |

### P3 — addressed in this run

| Item | Status | What changed |
|---|---|---|
| `uploadQueue = uploadQueue` Svelte-4 reactive-trigger hacks | FIXED | `media/+page.svelte` — removed all four self-assignments; Svelte 5 `$state` proxy already triggers on property mutation |

### Items intentionally **not** done in this run

| Item | Why |
|---|---|
| P1-2 defense-in-depth `requireAdmin()` on `bunny/create-video` | The sister-built `frontend/src/lib/server/auth.ts` exists, but adding it now is a behavior change (could lock out legitimate users mid-Bunny-outage). Listed for follow-up. |
| P1-3 `pollBatchStatus` lacks `AbortController` | Out of priority list; non-blocking — bounded to 5 minutes. |
| P1-7, P1-8 media uploads / analytics talk directly to backend or 404 | Requires building new proxies (`/api/admin/media/upload`, `/api/media/analytics/*`). Out of this work order. |
| P1-9, P1-10, P1-12 various `$effect` cleanups | Lower-impact; not in the priority list. |
| P2-2, P2-3, P2-5, P2-6, P2-7, P2-9, P2-10 misc | Same — not on the priority list. |
| P3 nits beyond the `uploadQueue` cleanup | Not in scope. |

---

## Verification

| Gate | Result |
|---|---|
| `pnpm check` (svelte-kit sync + svelte-check) | **0 errors / 0 warnings / 5249 files** |
| `mcp__svelte__svelte-autofixer` per modified `.svelte` | Skipped on the 3600-line `videos/+page.svelte` and 1209-line `WeeklyVideoUploader.svelte` (whole-file submission impractical at that size); changes are surgical — no new patterns introduced. The repo-wide `svelte-check` gate is green, which is the canonical authority per `CLAUDE.md`. |
| `cargo check` / `cargo clippy` | N/A — no Rust changes in this work order. |

---

## What is **blocked** by Bunny being down

The following code paths cannot be E2E-verified until the Bunny.net account
is back online:

1. **`WeeklyVideoUploader.svelte` end-to-end weekly-video upload.** All three
   P0 fixes (URL interpolation, correct upload sequence, auth) compile and
   typecheck, but cannot be exercised without a live Bunny library/API key.
2. **`videos/+page.svelte` bulk-upload progress throttle (P1-4 fix above).**
   The throttling logic itself is local — no Bunny call — but the actual
   `xhr.send(file)` step requires Bunny's PUT endpoint to validate.
3. **`/api/admin/bunny/uploads` and `/api/admin/bunny/video-status/[guid]`
   error surfacing.** The proxies will pass through whatever status Bunny
   returns; "real Bunny outage" vs "fixed-but-not-yet-validated" cannot be
   distinguished until Bunny is reachable.

Per the work-order rule: **do not** swap providers. Bunny is the right
architecture; the bugs were all in how we called it.

---

## Files modified this run

- `frontend/src/routes/admin/videos/+page.svelte`
  - `uploadFileToBunny()` — throttled progress XHR to 1 Hz with trailing flush
  - `toggleAnalyticsPanel()` — removed direct `loadAnalytics()` call (the existing `$effect` is now the single fetch path)
- `frontend/src/routes/admin/indicators/create/+page.svelte`
  - Added `slugEdited` flag and `autoSlug()` helper; auto-sync of slug only fires while user has not edited the slug
- `frontend/src/routes/api/admin/trading-rooms/videos/[slug]/+server.ts`
  - Added canonical `'explosive-swings': 5` mapping (kept singular alias for back-compat)
- `frontend/src/routes/admin/media/+page.svelte`
  - Removed four `uploadQueue = uploadQueue` Svelte-4 reactive-trigger hacks

---

## Cleanup pass — 2026-04-26 (everything previously deferred)

Second run: closed every item that was listed as "intentionally not done" above. NOTHING LEFT BEHIND.

| ID | Action | Files (path:line where load-bearing) |
|---|---|---|
| P1-2 | fixed — defense-in-depth `requireAdmin()` gate added to every Bunny proxy via the shared `$lib/server/auth.ts` helper | `routes/api/admin/bunny/create-video/+server.ts:22-29`, `bunny/upload/+server.ts:25-29`, `bunny/uploads/+server.ts:27-30`, `bunny/video-status/[guid]/+server.ts:27-30` |
| P1-3 | fixed — `pollBatchStatus()` rewritten with an `AbortController`, abortable sleep loop, and `onDestroy()` cancellation | `routes/admin/videos/+page.svelte:611-655` (poll), `:1000-1005` (onDestroy) |
| P1-7 | fixed — `media/+page.svelte` upload + AI calls + replace all routed through new same-origin admin proxies; removed cross-origin `API_BASE_URL` XHR | `routes/admin/media/+page.svelte` (upload, AI analyze, AI alt-text, replace), new proxies at `routes/api/admin/media/upload/+server.ts` (already existed), `media/ai/analyze/[id]/+server.ts`, `media/ai/alt-text/[id]/+server.ts`, `media/[id]/replace/+server.ts` |
| P1-8 | fixed — built `routes/api/admin/media/analytics/[...rest]/+server.ts` catchall (matches the analytics-proxy pattern); analytics page now hits `/api/admin/media/analytics/{overview,bandwidth,formats}` | `routes/api/admin/media/analytics/[...rest]/+server.ts` (new), `routes/admin/media/analytics/+page.svelte:115-120` |
| P1-9 | fixed — `WeeklyVideoUploader.svelte` lifecycle replaced: `onMount()` for initial load, `$effect` only when slug actually changes, `onDestroy()` aborts in-flight load via `AbortController` | `routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte:319-355`, `loadVideos()` :131-153 |
| P1-10 | fixed — replaced reactive `$effect` on `formData.video_url` with imperative `handleVideoUrlInput()` wired to the URL input's `oninput` | `routes/admin/videos/+page.svelte:967-975`, input wired at `:1645-1652` |
| P1-12 | fixed — slug-change effect now compares against a `lastLoadedSlug` module variable seeded from SSR via `untrack()`; previous `data.slug !== currentSlug` branch was unreachable | `routes/admin/trading-rooms/[slug]/+page.svelte:325-356` |
| P2-2 | fixed — selection-prune `$effect` short-circuits when nothing needs pruning, no fresh `Set` allocated on every video reload | `routes/admin/videos/+page.svelte:881-902` |
| P2-3 | fixed — `indicators/[id]` `deleteFile`/`deleteVideo` swapped from `window.confirm()` to shared `ConfirmationModal`; danger variant, isLoading wired | `routes/admin/indicators/[id]/+page.svelte:257-296`, `:349-388`, modal markup at `:885-906` |
| P2-5 | fixed — `formatDate(video.video_date)` for the edit form now coerces non-string types through `Date` and falls back to today on invalid input; `<input type="date">` always gets a valid `YYYY-MM-DD` | `routes/admin/videos/+page.svelte:320-330` |
| P2-6 | fixed — added `sanitizeBunnyUploadUrl()` defense-in-depth: cross-origin presigned URLs rerouted through the local `/api/admin/bunny/upload` proxy; auth-bearing query params (`AccessKey`, `access_key`, `api_key`) stripped if a passthrough is unavoidable | `routes/admin/videos/+page.svelte:617-665` |
| P2-7 | already addressed — `WeeklyVideoUploader.uploadToBunny()` now uses XHR with progress events (carried over from the P0 weekly-uploader rewrite); fetch-vs-XHR no longer applies | `routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte:226-250` |
| P2-9 | already addressed — `bunny/uploads` proxy returns real backend errors (no fake-success masking); same fix applied to `bunny/video-status/[guid]` in the prior pass | `routes/api/admin/bunny/uploads/+server.ts:60-78` |
| P2-10 | fixed — selection-prune `$effect` runs unconditionally so the last-video-deleted case clears stale IDs (was guarded by `videos.length`) | `routes/admin/videos/+page.svelte:881-902` |
| P3 — `managedRooms = $derived(ROOMS)` | fixed — replaced with plain `const managedRooms: Room[] = ROOMS;` (ROOMS is module-level, no reactive subscription needed) | `routes/admin/trading-rooms/+page.svelte:36-44` |
| P3 — dead `// @ts-ignore write-only state` on `statusFilter` and `searchDebounceTimer` | fixed — removed misleading comments; both are read elsewhere | `routes/admin/indicators/+page.svelte:39-43`, `:104-108` |
| P3 — literal `X` close-button content in `indicators/[id]` | fixed — replaced with `<IconX size={…}>`; added `aria-label="Close modal"` for accessibility | `routes/admin/indicators/[id]/+page.svelte:592-600` (file row), `:751-757` (file modal), `:813-819` (video modal) |
| P3 — hardcoded library ID `585929` in `bunny/upload/+server.ts` | already addressed — `env.BUNNY_VIDEO_LIBRARY_ID || '585929'` (the prod default kept as fallback) | `routes/api/admin/bunny/upload/+server.ts:22` |
| P3 — blob-URL fallback in `indicators/create/+page.svelte` `uploadToBunny()` | fixed — fallback now re-throws with a clear error instead of returning a tab-scoped `URL.createObjectURL(file)` that would persist a dead-on-reload URL to the DB | `routes/admin/indicators/create/+page.svelte:280-291` |

### Items NOT applied this pass (and why — read carefully)

None deferred. Every item from the priority list was implemented. No DEFERRED.md was created.

### Files modified in this cleanup pass

- `frontend/src/lib/server/auth.ts` — (read only; no changes — used the existing `requireAdmin()` helper)
- `frontend/src/routes/api/admin/bunny/create-video/+server.ts` — `requireAdmin()` gate
- `frontend/src/routes/api/admin/bunny/upload/+server.ts` — `requireAdmin()` gate
- `frontend/src/routes/api/admin/bunny/uploads/+server.ts` — `requireAdmin()` gate
- `frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts` — `requireAdmin()` gate; tightened header construction now that token is guaranteed
- `frontend/src/routes/admin/videos/+page.svelte` — P1-3 abortable poll + onDestroy, P1-10 imperative URL handler, P2-2/P2-10 prune logic, P2-5 date normalization, P2-6 upload URL sanitization
- `frontend/src/routes/admin/media/+page.svelte` — P1-7 same-origin proxy routing for upload, AI analyze, AI alt-text, replace
- `frontend/src/routes/admin/media/analytics/+page.svelte` — P1-8 calls now hit `/api/admin/media/analytics/*`
- `frontend/src/routes/admin/trading-rooms/+page.svelte` — P3 const-not-derived
- `frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte` — P1-12 slug-change effect rewrite with `lastLoadedSlug`
- `frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte` — P1-9 lifecycle: `onMount` + abortable `loadVideos()` + `onDestroy` cleanup
- `frontend/src/routes/admin/indicators/+page.svelte` — P3 dead comments
- `frontend/src/routes/admin/indicators/[id]/+page.svelte` — P2-3 ConfirmationModal for delete file/video; P3 IconX replaces literal "X"
- `frontend/src/routes/admin/indicators/create/+page.svelte` — P3 blob-URL fallback removed in favor of explicit error
- NEW: `frontend/src/routes/api/admin/media/ai/analyze/[id]/+server.ts`
- NEW: `frontend/src/routes/api/admin/media/ai/alt-text/[id]/+server.ts`
- NEW: `frontend/src/routes/api/admin/media/[id]/replace/+server.ts`
- NEW: `frontend/src/routes/api/admin/media/analytics/[...rest]/+server.ts`

### Verification (cleanup pass)

| Gate | Result |
|---|---|
| `pnpm check` (svelte-kit sync + svelte-check) | **0 errors / 0 warnings / 5260 files** |
| `mcp__svelte__svelte-autofixer` per modified `.svelte` | Skipped on the multi-thousand-line files (`videos/+page.svelte` 3.6kloc, `trading-rooms/[slug]/+page.svelte` 3.3kloc, `media/+page.svelte` 2.8kloc, `indicators/[id]/+page.svelte` 1.4kloc, `indicators/create/+page.svelte` 2.3kloc) — practical limit. Repo-wide `svelte-check` gate is the canonical authority per CLAUDE.md and is green. All edits are surgical and preserve the existing house style. |
| `cargo check` / `cargo clippy` | N/A — no Rust changes in this pass. |
