# 05 — Video / Media / Trading Rooms / Indicators Audit

Audit date: 2026-04-26
Scope: admin video, media, trading-rooms, and indicators surfaces, plus their `/api/admin/bunny` and `/api/admin/trading-rooms` proxies.
Method: read every file in scope, cross-reference against repo conventions in `CLAUDE.md`, look for orphan endpoints in `frontend/src/routes/api/admin`.

## Files reviewed

Frontend pages
- [`frontend/src/routes/admin/videos/+page.svelte`](../../../frontend/src/routes/admin/videos/+page.svelte) (3626 lines)
- [`frontend/src/routes/admin/media/+page.svelte`](../../../frontend/src/routes/admin/media/+page.svelte) (2797 lines)
- [`frontend/src/routes/admin/media/analytics/+page.svelte`](../../../frontend/src/routes/admin/media/analytics/+page.svelte) (1398 lines)
- [`frontend/src/routes/admin/trading-rooms/+page.svelte`](../../../frontend/src/routes/admin/trading-rooms/+page.svelte) (357 lines)
- [`frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte`](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte) (3274 lines)
- [`frontend/src/routes/admin/trading-rooms/[slug]/+page.server.ts`](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.server.ts)
- [`frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte) (1209 lines)
- [`frontend/src/routes/admin/trading-rooms/[slug]/components/TradeEntryManager.svelte`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/TradeEntryManager.svelte) (1069 lines)
- [`frontend/src/routes/admin/indicators/+page.svelte`](../../../frontend/src/routes/admin/indicators/+page.svelte) (818 lines)
- [`frontend/src/routes/admin/indicators/[id]/+page.svelte`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte) (1348 lines)
- [`frontend/src/routes/admin/indicators/create/+page.svelte`](../../../frontend/src/routes/admin/indicators/create/+page.svelte) (2273 lines)

API proxies
- [`frontend/src/routes/api/admin/bunny/uploads/+server.ts`](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts)
- [`frontend/src/routes/api/admin/bunny/create-video/+server.ts`](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts)
- [`frontend/src/routes/api/admin/bunny/upload/+server.ts`](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts)
- [`frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts`](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts)
- [`frontend/src/routes/api/admin/trading-rooms/+server.ts`](../../../frontend/src/routes/api/admin/trading-rooms/+server.ts)
- [`frontend/src/routes/api/admin/trading-rooms/traders/+server.ts`](../../../frontend/src/routes/api/admin/trading-rooms/traders/+server.ts)
- [`frontend/src/routes/api/admin/trading-rooms/videos/+server.ts`](../../../frontend/src/routes/api/admin/trading-rooms/videos/+server.ts)
- [`frontend/src/routes/api/admin/trading-rooms/videos/[slug]/+server.ts`](../../../frontend/src/routes/api/admin/trading-rooms/videos/[slug]/+server.ts)

---

## Critical bugs (P0)

### P0-1. WeeklyVideoUploader writes a literal `VIDEO_LIBRARY_ID` placeholder into video URLs

[`WeeklyVideoUploader.svelte:201`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte#L201)

```ts
form.video_url = `https://iframe.mediadelivery.net/embed/VIDEO_LIBRARY_ID/${video_id}`;
```

The string `VIDEO_LIBRARY_ID` is not interpolated — it is the literal placeholder. Every Bunny upload performed through this component therefore produces a broken iframe URL such as `https://iframe.mediadelivery.net/embed/VIDEO_LIBRARY_ID/abc123-...`. When the room admin then publishes the weekly video, the resulting record persists this URL to the DB and members see a 404 iframe.

### P0-2. WeeklyVideoUploader expects `upload_url` from `/api/admin/bunny/create-video` but the proxy never returns one

[`WeeklyVideoUploader.svelte:179-198`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte#L179-L198)

```ts
const createRes = await fetch('/api/admin/bunny/create-video', { method: 'POST', ... });
const { video_id, upload_url } = await createRes.json();

const uploadRes = await fetch(upload_url, { method: 'PUT', ... body: uploadFile });
```

The proxy at [`api/admin/bunny/create-video/+server.ts:60-62`](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L60-L62) returns `backendData` directly. Backends typically only return Bunny's GUID, not a presigned upload URL — Bunny is a TUS / direct-PUT to the well-known Stream endpoint with the API key in headers (which is exactly why the sibling [`api/admin/bunny/upload/+server.ts`](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L48-L59) exists as a server-side proxy). The component uses neither shape: it treats `upload_url` as if Bunny returned a presigned URL, then `fetch`'s it from the browser. Because no `upload_url` field exists, the second `fetch(undefined)` either errors or hits the current document URL. End result: the file is never uploaded.

The correct path is the `PUT /api/admin/bunny/upload?video_guid=...&library_id=...` proxy that already exists; this component was wired to the wrong sequence.

### P0-3. WeeklyVideoUploader bypasses auth — no `Authorization` / cookie handling

Same block, [`WeeklyVideoUploader.svelte:179-186`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte#L179-L186): the request is plain `fetch` without `credentials: 'include'` and without `Authorization`. The proxy at [`api/admin/bunny/create-video/+server.ts:20-24`](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L20-L24) reads `cookies.get('rtp_access_token')` and 401s if missing. Same-origin fetches do send cookies, so this happens to work, but it is inconsistent with the rest of the admin which uses `adminFetch` (Bearer header). If session expires mid-modal it silently fails.

### P0-4. `/api/admin/indicators/*` is an orphan — frontend calls have no proxy

The indicators pages call `adminFetch('/api/admin/indicators...')` extensively:

- [`admin/indicators/+page.svelte:54`](../../../frontend/src/routes/admin/indicators/+page.svelte#L54) — `GET /api/admin/indicators`
- [`admin/indicators/+page.svelte:72`](../../../frontend/src/routes/admin/indicators/+page.svelte#L72) — `DELETE /api/admin/indicators/${id}`
- [`admin/indicators/[id]/+page.svelte:121, 140, 155, 173, 195, 225, 255, 277, 301, 328`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L121) — full CRUD against `/api/admin/indicators/{id}/files`, `.../videos`, `.../toggle`
- [`admin/indicators/create/+page.svelte:548, 560, 578`](../../../frontend/src/routes/admin/indicators/create/+page.svelte#L548) — `POST /api/admin/indicators` and `.../files`, `.../documentation`

`find frontend/src/routes/api/admin -type d` reveals there is no `api/admin/indicators/` proxy folder, and `hooks.server.ts` does not generically proxy `/api/admin/*` to backend. In production all these requests are 404s served by SvelteKit's catch-all. (It may "work" in dev because of Vite's `/api` proxy, but on Pages.dev as deployed there is nothing to handle them.)

### P0-5. `indicators/create` sends an array index as `platform_id` foreign key

[`admin/indicators/create/+page.svelte:560-571`](../../../frontend/src/routes/admin/indicators/create/+page.svelte#L560-L571)

```ts
await adminFetch(`/api/admin/indicators/${indicatorId}/files`, {
    method: 'POST',
    body: JSON.stringify({
        platform_id: PLATFORMS.findIndex((p) => p.id === pf.platform_id) + 1,
        ...
```

`PLATFORMS` is a hardcoded array of 3 entries (`thinkorswim`, `tradingview`, `trendspider`). `findIndex + 1` produces 1/2/3 with no relation to the real `platforms` table primary keys in the backend. Either every file insertion fails the FK, or worse, inserts get silently associated with the wrong platform if the table happens to have rows 1-3. Compare with the editor at [`admin/indicators/[id]/+page.svelte:218-222`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L218-L222) which (correctly) sends the platform string slug.

### P0-6. `videos` page has triple-fire video-load on initial render

[`admin/videos/+page.svelte:955-974`](../../../frontend/src/routes/admin/videos/+page.svelte#L955-L974)

```ts
$effect(() => {                  // (A) init
    if (!browser) return;
    const init = async () => {
        await loadRoomsAndTraders();
        if (selectedRoom) {
            await loadVideos();
        }
    };
    init();
});

$effect(() => {                  // (B) react to room change
    if (selectedRoom && !isLoadingRooms) {
        loadVideos();
    }
});
```

Plus `selectRoom()` at [`videos/+page.svelte:287-291`](../../../frontend/src/routes/admin/videos/+page.svelte#L287-L291) also calls `loadVideos()`. Sequence on first visit:
1. (A) loads rooms, sets `selectedRoom`, calls `loadVideos()` — fetch #1.
2. Selecting `selectedRoom` triggers (B) — fetch #2.
3. `isLoadingRooms` flipping `true→false` re-runs (B) — fetch #3.

When the user clicks a different tab, `selectRoom()` directly calls `loadVideos()` AND mutates `selectedRoom` which fires (B) again → another double-fetch. Symptoms: spinner flicker, last-write-wins races on `videos`/`totalPages`, wasted backend calls.

### P0-7. `trading-rooms/[slug]` reintroduces the shadow-state anti-pattern explicitly banned in `CLAUDE.md`

[`trading-rooms/[slug]/+page.svelte:300-322`](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte#L300-L322)

```ts
$effect(() => { tradePlanEntries = data.initialData?.tradePlan ?? []; });
$effect(() => { alerts = data.initialData?.alerts ?? []; });
$effect(() => { currentVideo = data.initialData?.weeklyVideo ?? null; });
$effect(() => { roomStats = data.initialData?.roomStats ?? null; });
$effect(() => { trades = data.initialData?.trades ?? []; });
$effect(() => { videoResources = data.initialData?.videoResources ?? []; });
```

This is exactly the pattern `CLAUDE.md` calls out:

> ❌ `let foo = $state(props.foo ?? null); $effect(...)` (the shadow-state pattern that emits `state_referenced_locally` warnings)
> ✅ `let { foo = $bindable(null) } = $props();`

These six effects shadow the `data` prop into local state. Any time `data` is reassigned (e.g. SvelteKit re-runs `load`), all six fire and clobber any locally edited state — for instance, an admin who has just optimistically toggled an alert pin will see the change disappear if the loader returns. The fix is to use `$derived` or the `data` prop directly in the template, or to remove the `$state` declarations entirely.

### P0-8. `+page.server.ts` SSR loader checks the wrong cookie name

[`trading-rooms/[slug]/+page.server.ts:25-28`](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.server.ts#L25-L28)

```ts
const sessionToken = cookies.get('session_token');
if (!sessionToken) {
    redirect(302, '/login?redirect=/admin/trading-rooms/' + slug);
}
```

The canonical cookie set by the login proxy is `rtp_access_token` (per recent commit `e2356fa46` and the convention in CLAUDE.md). `session_token` is never set anywhere I can find. Either every authenticated admin gets redirected to `/login` on every SSR pass (in which case the page is unusable), or this check is silently no-opping because the cookie *also* gets some legacy value somewhere. Either way it is wrong.

Also the parallel `fetch()` calls below it (lines 38-47) do not forward any auth cookie, so the SSR-only API requests likely all 401 silently and the user is rescued only by the second client-side load.

### P0-9. `bunny/uploads` proxy forwards `rtp_access_token` as a `Cookie` named `session=` instead of as Bearer

[`api/admin/bunny/uploads/+server.ts:28-37`](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts#L28-L37)

```ts
// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not session.
// const session = cookies.get('session');
const session = cookies.get('rtp_access_token');
if (session) {
    headers['Cookie'] = `session=${session}`;
}
```

The fix half-replaced the read but kept the write. The backend now receives `Cookie: session=<jwt>` — the backend extracts auth from a Bearer header or its own JWT cookie name, not a cookie named `session`. The neighbouring proxies ([`bunny/create-video/+server.ts:34-46`](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L34-L46), [`bunny/upload/+server.ts:51-59`](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L51-L59), [`bunny/video-status/[guid]/+server.ts:33-40`](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts#L33-L40)) all correctly send `Authorization: Bearer ${accessToken}`. This proxy alone is broken and silently 401s the backend, then masks the failure by returning a fake empty success (`{ success: true, data: [] }`) at line 60-63. The "Bunny Uploads" admin list will *always* show empty.

---

## High-severity issues (P1)

### P1-1. Bunny `video-status` swallows real errors and lies "processing" forever

[`api/admin/bunny/video-status/[guid]/+server.ts:42-84`](../../../frontend/src/routes/api/admin/bunny/video-status/[guid]/+server.ts#L42-L84)

When the backend returns 4xx/5xx OR throws, the proxy returns `{ success: true, status: 'processing', status_code: 3, ... }`. From the client's perspective every poll reports "still processing" — there is no way for the UI to surface a real error (auth failure, deleted video, backend down). Long-running uploads will appear hung indefinitely. Should propagate a real `success: false` with the upstream status so the admin client can stop polling.

### P1-2. `bunny/create-video` ignores cookies map and only checks Bearer Authorization

[`api/admin/bunny/create-video/+server.ts:20-24`](../../../frontend/src/routes/api/admin/bunny/create-video/+server.ts#L20-L24)

```ts
const accessToken = cookies.get('rtp_access_token');
if (!accessToken) error(401, 'Authentication required');
```

OK on its own. But it also never validates the user is admin — it forwards the JWT verbatim and trusts the backend. If the backend's `create-video` route is mistakenly opened to non-admins, this proxy would happily forward a regular member token. Defense in depth is recommended; minimally call a `requireAdmin()` helper at the proxy.

### P1-3. `videos/+page.svelte` polling for batch status has no abort path on unmount

[`admin/videos/+page.svelte:586-603`](../../../frontend/src/routes/admin/videos/+page.svelte#L586-L603)

```ts
async function pollBatchStatus(batchId: string): Promise<void> {
    let attempts = 0;
    const maxAttempts = 60; // 5 minutes with 5-second intervals
    while (attempts < maxAttempts) {
        const response = await bulkUploadApi.getBatchStatus(batchId);
        ...
        await new Promise((resolve) => setTimeout(resolve, 5000));
        attempts++;
    }
}
```

No `AbortController`, no cleanup if the user navigates away mid-upload. The component will continue to poll the backend every 5 s for up to 5 minutes after navigation — and `bunnyUploadStatus = response.data` will fire on a state-detached component, throwing a Svelte runtime warning. Multiply by 60 attempts × concurrent uploads.

### P1-4. `videos/+page.svelte` upload progress reporting fires N requests per file

[`admin/videos/+page.svelte:555-562`](../../../frontend/src/routes/admin/videos/+page.svelte#L555-L562)

```ts
xhr.upload.addEventListener('progress', async (e) => {
    if (e.lengthComputable) {
        const progress = Math.round((e.loaded / e.total) * 100);
        await bulkUploadApi.updateItemStatus(itemId, {
            status: 'uploading',
            progress_percent: progress
        });
    }
});
```

Every progress event (XHR fires these many times per second on slow uplinks) issues an HTTP call to update the bulk-upload item status. A single 1 GB upload over a slow connection can produce hundreds of progress events. This will hammer the backend and may itself slow the upload by saturating the backend's request queue. Throttle to 1 update per second or per-percent-change.

### P1-5. `indicators/+page.svelte` fires `fetchIndicators` twice on initial render

[`admin/indicators/+page.svelte:96-114`](../../../frontend/src/routes/admin/indicators/+page.svelte#L96-L114)

```ts
onMount(() => { fetchIndicators(); });

$effect(() => {
    clearTimeout(searchDebounceTimer);
    searchDebounceTimer = setTimeout(() => {
        untrack(() => { page = 1; fetchIndicators(); });
    }, 300);
});
```

The `$effect` body always reads `search` and `statusFilter` (those are the comments line 105 references), so it runs once on mount in addition to `onMount`. Net effect: 2 fetches 300 ms apart, the second resetting `page = 1` (harmless on initial load but wasteful). The author flagged this with comments but it still happens.

### P1-6. `indicators/[id]/+page.svelte` uses raw `fetch` for FormData upload, skipping `adminFetch`

[`admin/indicators/[id]/+page.svelte:225-229`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L225-L229)

```ts
const response = await fetch(`/api/admin/indicators/${indicatorId}/files`, {
    method: 'POST',
    body: formData,
    credentials: 'include'
});
```

This relies on the server reading the `rtp_access_token` cookie (which `adminFetch` would send as a Bearer header). If the backend route insists on `Authorization`, this 401s. Combined with P0-4 (no proxy at all for `/api/admin/indicators/*`), this upload is doubly broken.

### P1-7. `media/+page.svelte` upload talks directly to backend (cross-origin), bypassing the proxy convention

[`admin/media/+page.svelte:245-247`](../../../frontend/src/routes/admin/media/+page.svelte#L245-L247)

```ts
xhr.open('POST', `${API_BASE_URL}${API_ENDPOINTS.admin.media.upload}`);
xhr.withCredentials = true;
```

`API_BASE_URL` is the absolute Fly.io URL (per [`lib/api/config.ts:16`](../../../frontend/src/lib/api/config.ts#L16)). This is a direct browser→backend request. CLAUDE.md explicitly mandates the `+server.ts` proxy convention; this XHR opts out, requiring CORS to be configured for credentials. There is no `/api/admin/media/upload/+server.ts` proxy. Same browser-direct concern at [`media/+page.svelte:381, 417, 456`](../../../frontend/src/routes/admin/media/+page.svelte#L381) (`/api/media/ai/analyze/...`, `/api/media/ai/alt-text/...`, `/api/media/{id}/replace`) — those endpoints have no proxy folder either.

### P1-8. `media/analytics/+page.svelte` calls `/api/media/analytics/*` which has no proxy

[`admin/media/analytics/+page.svelte:115-117`](../../../frontend/src/routes/admin/media/analytics/+page.svelte#L115-L117)

```ts
fetch('/api/media/analytics/overview'),
fetch(`/api/media/analytics/bandwidth?range=${timeRange}`),
fetch('/api/media/analytics/formats')
```

There is no `frontend/src/routes/api/media/analytics/...` and no `/api/admin/media/analytics`. These all 404 in production. The page handles failure by showing a "Connection error" message ([line 156-163](../../../frontend/src/routes/admin/media/analytics/+page.svelte#L156-L163)), so this fails silently — the dashboard simply *never* shows real data.

### P1-9. `WeeklyVideoUploader.svelte` cleanup-less polling of `roomSlug` change

[`WeeklyVideoUploader.svelte:281-285`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte#L281-L285)

```ts
$effect(() => {
    if (roomSlug) {
        untrack(() => loadVideos());
    }
});
```

Two issues. (1) `untrack` here means `loadVideos()` will not pick up subsequent changes to `roomSlug`; the `if (roomSlug)` outside `untrack` is the only dependency, so any later mutation that changes truthiness *won't* (e.g. slug going from `"a"` to `"b"`). This is mostly fine because `roomSlug` is a prop, but mishandling — typical correct form is `untrack` only the function call, with the dep read **outside** `untrack` (that part is right) but you still need the read of the new value, which is happening. So the logic is OK, but the pattern is fragile. (2) No abort on unmount; `loadVideos` continues even after the parent destroys the component.

### P1-10. `videos/+page.svelte` `$effect` on `formData.video_url` mutates `formData.video_platform` on every render

[`admin/videos/+page.svelte:899-903`](../../../frontend/src/routes/admin/videos/+page.svelte#L899-L903)

```ts
$effect(() => {
    if (formData.video_url) {
        formData.video_platform = detectPlatform(formData.video_url);
    }
});
```

This effect reads `formData.video_url`, writes `formData.video_platform`. It shares the reactive container with `formData` itself, so any other write to `formData.*` in the component will retrigger this effect (since `$state(formData)` is a deep-tracked proxy, and the effect dependency is on the *URL property* — Svelte 5 fine-grained tracking should narrow this, but combined with the manual reset in `openUploadModal()` and `openEditModal()` you're guaranteed extra effect runs). Better: write the `video_platform` once when the user types/blurs the URL field instead of via reactive coupling.

### P1-11. Mock-data fallbacks in `traders` and `videos` proxies will leak in production

[`api/admin/trading-rooms/traders/+server.ts:19-84`](../../../frontend/src/routes/api/admin/trading-rooms/traders/+server.ts#L19-L84) and [`api/admin/trading-rooms/videos/+server.ts:19-120`](../../../frontend/src/routes/api/admin/trading-rooms/videos/+server.ts#L19-L120) ship 84 lines and 120 lines of *hardcoded mock data* that are returned as a fallback if the backend call doesn't return `success: true`. They flag the response with `_mock: true` but the admin UI does not check the flag — mock traders Mike/Sarah/James/Emily will silently appear in the dropdown if the backend hiccups. Same for the videos. This means a 5xx on the backend is invisible in the UI; the admin sees "all is well" with phantom data.

### P1-12. `trading-rooms/[slug]/+page.svelte` SSR-rehydration `$effect` overwrites client edits

[`trading-rooms/[slug]/+page.svelte:330-347`](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte#L330-L347)

```ts
$effect(() => {
    if (!browser) return;
    const currentSlug = slug;
    if (currentSlug && data.slug !== currentSlug) {
        untrack(() => {
            loadTradePlan(); loadAlerts(); loadWeeklyVideo();
            loadRoomStats(); loadTrades(); loadVideoResources();
        });
    }
});
```

The branch `data.slug !== currentSlug` is unreachable in normal navigation: `slug` is `$derived(data.slug ?? page.params.slug ?? '')`, so `slug === data.slug` whenever `data.slug` is set. This effect runs but never does anything — except feed effect-graph noise to the six shadow-state effects above (P0-7). Either the condition is wrong (probably should be `currentSlug !== prevSlug`) or this whole block is dead.

---

## Medium issues (P2)

### P2-1. `videos` page `$effect` reload on `analyticsPeriod` re-fetches even when the panel is closed

[`admin/videos/+page.svelte:650-654`](../../../frontend/src/routes/admin/videos/+page.svelte#L650-L654) — guarded by `showAnalyticsPanel` so OK in steady state, but the function `toggleAnalyticsPanel` already calls `loadAnalytics()` once on open ([line 630-635](../../../frontend/src/routes/admin/videos/+page.svelte#L630-L635)) and the effect re-fires, producing a duplicate call on first open.

### P2-2. `videos` `$effect` on `selectedVideoIds` runs on every video reload

[`admin/videos/+page.svelte:846-860`](../../../frontend/src/routes/admin/videos/+page.svelte#L846-L860) reads `videos.length` and `selectedVideoIds` and writes `selectedVideoIds` if pruning is needed. This is correct logically but allocates a fresh `Set` on every video change, even when no pruning is necessary; cheap but a hot path on reorder/refresh.

### P2-3. `delete` confirmations use `window.confirm` in indicators editor

[`admin/indicators/[id]/+page.svelte:252, 325`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L252) — uses native `confirm()` while the rest of the admin uses `ConfirmationModal.svelte`. Inconsistent UX, and the native dialog is blocked in some sandboxed contexts.

### P2-4. `videos/[slug]` proxy mock data drift

[`api/admin/trading-rooms/videos/[slug]/+server.ts:18-24`](../../../frontend/src/routes/api/admin/trading-rooms/videos/[slug]/+server.ts#L18-L24)

```ts
const roomSlugToId: Record<string, number> = {
    'day-trading-room': 1, 'swing-trading-room': 2,
    'small-account-mentorship': 3, 'spx-profit-pulse': 4,
    'explosive-swing': 5  // singular — but config has 'explosive-swings' (plural)
};
```

[`admin/trading-rooms/+page.svelte:57`](../../../frontend/src/routes/admin/trading-rooms/+page.svelte#L57) and rooms config use `'explosive-swings'` (plural). The mock 404 path in the proxy will be hit for the plural slug; nothing critical because real backend takes over, but a footgun if backend ever fails.

### P2-5. `formatDate` of `video.video_date` truncates only when string

[`admin/videos/+page.svelte:320-322`](../../../frontend/src/routes/admin/videos/+page.svelte#L320-L322)

```ts
video_date: typeof video.video_date === 'string' ? video.video_date.split('T')[0] : video.video_date
```

If the API returns a Date-like or a number, this passes it through to a date input, which expects `YYYY-MM-DD`. Easy to silently produce an invalid HTML date input value.

### P2-6. `videos/+page.svelte` `xhr.send(file)` direct-to-Bunny upload at line 580-583

[`admin/videos/+page.svelte:580-583`](../../../frontend/src/routes/admin/videos/+page.svelte#L580-L583) PUTs file straight to whatever URL the backend provided. If that URL is an absolute Bunny.net Stream upload URL with the API key embedded as a query parameter (Bunny's TUS variant), the API key briefly hits the browser's network panel and console — depending on Bunny config this may or may not be a leak, but the convention in this repo is "API keys never leave the server" (see [`bunny/upload/+server.ts:9-10`](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L9-L10)). Audit the response shape of `bulkUploadApi.init()` — if it returns full Bunny presigned URLs, this is a P1 leak; if it returns proxy URLs back to our own server, it is fine.

### P2-7. `WeeklyVideoUploader.svelte` upload progress is set to 0 → 100 with nothing in between

[`WeeklyVideoUploader.svelte:172-211`](../../../frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte#L172-L211) — the progress bar in the template (line 458) animates from 0% to 100% with no granularity because the `fetch(... PUT)` API has no progress callback. If P0-1/P0-2 are fixed, this is still an XHR-vs-fetch issue.

### P2-8. `indicators/create` reactive slug never lets you keep a custom slug

[`admin/indicators/create/+page.svelte:173-181`](../../../frontend/src/routes/admin/indicators/create/+page.svelte#L173-L181)

```ts
$effect(() => {
    if (indicator.name) {
        indicator.slug = indicator.name.toLowerCase()...;
    }
});
```

If the admin manually edits the slug field, the next character typed into the name field clobbers it. Should only auto-fill the slug when it's empty or untouched.

### P2-9. `bunny/uploads` proxy returns lying success on backend failure

[`api/admin/bunny/uploads/+server.ts:60-63`](../../../frontend/src/routes/api/admin/bunny/uploads/+server.ts#L60-L63) — if backend is unreachable, returns `{ success: true, data: [] }`. Same pattern as P1-1; admin sees no uploads, no error.

### P2-10. `videos` page `$effect` to clear selection on video change uses `videos.length` only

[`admin/videos/+page.svelte:847`](../../../frontend/src/routes/admin/videos/+page.svelte#L847) — guarded by `if (videos.length)`. If the user deletes the last selected video, `videos.length === 0` skips the prune and stale IDs remain in `selectedVideoIds`. Should run unconditionally.

---

## Low / nits (P3)

- [`admin/trading-rooms/+page.svelte:36`](../../../frontend/src/routes/admin/trading-rooms/+page.svelte#L36) — `const managedRooms = $derived(ROOMS)` is wrapping a constant in `$derived`; pointless, just `const managedRooms = ROOMS`.
- [`admin/videos/+page.svelte:22-39`](../../../frontend/src/routes/admin/videos/+page.svelte#L22-L39) — imports from `$lib/icons` but rest of repo uses direct `@tabler/icons-svelte-runes/icons/...` imports for code-splitting. Mixed convention.
- [`api/admin/trading-rooms/videos/[slug]/+server.ts`](../../../frontend/src/routes/api/admin/trading-rooms/videos/[slug]/+server.ts) — duplicates 100+ lines of mock data already present in the parent endpoint. Extract or delete one.
- [`admin/indicators/[id]/+page.svelte:584`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L584) — uses literal `X` characters as close button content; should use `IconX` like the rest of the codebase.
- [`admin/indicators/+page.svelte:38-39`](../../../frontend/src/routes/admin/indicators/+page.svelte#L38-L39) — `// @ts-ignore write-only state` on `statusFilter` indicates dead state; clean up.
- [`admin/videos/+page.svelte:217-219`](../../../frontend/src/routes/admin/videos/+page.svelte#L217-L219) — `Promise.all` wins instead of `Promise.allSettled` — if either rooms or traders fails, both errors are surfaced as a single "failed to load rooms and traders" without distinction.
- `[`admin/media/+page.svelte:217, 231, 255, 267`](../../../frontend/src/routes/admin/media/+page.svelte#L217)` — `uploadQueue = uploadQueue` reactive-trigger hack is the legacy Svelte 4 idiom; in Svelte 5 with `$state` proxies this is unnecessary noise.
- [`admin/indicators/[id]/+page.svelte:14`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L14) — uses `import { adminFetch }` but light/dark themed inline styles still use legacy `#143e59` and `#fff` hardcoded values; doesn't honor the no-global-theme rule.
- [`api/admin/bunny/upload/+server.ts:32`](../../../frontend/src/routes/api/admin/bunny/upload/+server.ts#L32) — default library ID `585929` hardcoded. Should come from env var.
- [`admin/indicators/create/+page.svelte:259-269`](../../../frontend/src/routes/admin/indicators/create/+page.svelte#L259-L269) — on upload failure, falls back to `URL.createObjectURL(file)` which is a *blob: URL valid only for the current tab*. If the indicator is then saved, the persisted `file_url` is a dead-on-reload local URL.
- The proxy at `bunny/uploads/+server.ts` (older style: missing the `API_BASE_URL || BACKEND_URL ||` chain) only uses `env.BACKEND_URL`; the rest of the proxies use `env.API_BASE_URL || env.BACKEND_URL || ...`. Inconsistent and a potential prod-only break.
- [`admin/indicators/[id]/+page.svelte:225`](../../../frontend/src/routes/admin/indicators/[id]/+page.svelte#L225) — console.error variants leak unhandled rejections; user sees nothing for `_e` ignored failures.

---

## Notes on recent video-fetch refactor (commit 33112d5a1)

The commit message "new video fetching approach across learning center and trading room archive pages" suggests the **archive/library** pages (member-facing) were the focus. Within the admin scope I audited:

1. **`trading-rooms/[slug]/+page.svelte`** is the closest companion. Its `loadVideoResources()` at [line 416-430](../../../frontend/src/routes/admin/trading-rooms/[slug]/+page.svelte#L416-L430) goes through `roomResourcesApi.adminList({ room_slug, resource_type: 'video', ... })`. The shape change appears to have introduced the six shadow-state effects (P0-7) — those `$effect` blocks are how the page rehydrates from the new SSR-loaded `data.initialData.videoResources`. The parallel client refetch at lines 416-430 is then redundant on first render, because SSR already populated the array.

2. **`WeeklyVideoUploader.svelte`** is a sibling component. P0-1 (`VIDEO_LIBRARY_ID` placeholder) and P0-2 (wrong upload sequence) are exactly the kind of half-completed work that often sneaks in during such a refactor — the new fetch path was added but the upload path was never wired up to a working backend response shape. **Strongly recommend running an end-to-end weekly-video upload manually before next deploy.**

3. **`videos/+page.svelte`** does not appear to use the new `roomResourcesApi` flow — it still calls `tradingRoomApi.videos.adminListByRoom()` ([line 250-254](../../../frontend/src/routes/admin/videos/+page.svelte#L250-L254)). This may be intentional (admin-specific shape) or it may be drift; `$lib/api/trading-rooms.ts` and `$lib/api/room-resources.ts` are now both serving "videos by room" with different shapes. Consider consolidating.

4. The Bunny proxies were untouched by the refactor and look stable except for P0-9 (the `bunny/uploads` cookie name regression), which has comments dated `FIX-2026-04-26` indicating the partial fix is recent and unfinished.

---

## Summary

| Severity | Count | Highlights |
|---|---|---|
| P0 | 9 | Weekly-video uploader writes `VIDEO_LIBRARY_ID` literal into URLs (P0-1); broken upload sequence calls non-existent `upload_url` (P0-2); 100% of `/api/admin/indicators/*` is orphaned (P0-4); `+page.server.ts` SSR redirects on wrong cookie (P0-8); shadow-state pattern reintroduced in 6 effects (P0-7); triple video-fetch on init (P0-6); `bunny/uploads` proxy ships forwarded JWT under wrong cookie name (P0-9); `indicators/create` posts integer indices as platform FKs (P0-5). |
| P1 | 12 | Status proxy hides errors as "processing" forever (P1-1); progress XHR fires hundreds of HTTP per upload (P1-4); media uploads bypass proxy convention (P1-7); analytics page hits non-existent endpoints (P1-8); 100+ lines of mock data leak silently into prod responses on backend failure (P1-11). |
| P2 | 10 | Slug-clobber on every name keystroke (P2-8); fake-success fallbacks throughout (P2-9); proxy slug mismatch `explosive-swing` vs `explosive-swings` (P2-4); hardcoded library ID (covered in P3). |
| P3 | 11 | Mixed icon-import conventions, redundant `$derived` on constants, dead state, blob-URL fallbacks for failed uploads, `uploadQueue = uploadQueue` Svelte-4 idioms. |

**Most urgent:** P0-1, P0-2, P0-3 (weekly-video upload is silently 100% broken right now); P0-4 (entire indicators surface 404s in production); P0-7 (CLAUDE.md-banned anti-pattern reintroduced — typecheck gate violation per the repo policy); P0-8 (per-room admin pages may be 100% redirecting to login).
