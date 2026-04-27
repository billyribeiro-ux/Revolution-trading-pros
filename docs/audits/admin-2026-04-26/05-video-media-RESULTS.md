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
