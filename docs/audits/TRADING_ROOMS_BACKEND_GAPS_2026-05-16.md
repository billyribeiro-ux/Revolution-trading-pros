# Trading-Rooms Admin: backend gaps found by end-to-end tracing

**Date:** 2026-05-16
**How found:** During QW-2 ("unused-vars cleanup") the operator correctly
insisted: *"before assuming it's dead code, trace every single file
first-to-last end-to-end, because it could be something we should be
adding instead of deleting."* Doing that on the trading-rooms admin
proxies surfaced two real backend gaps the cleanup framing had hidden.
The `mockVideos` / `mockTraders` arrays were **restored 100%** (not
deleted) because they are the only working data path while these gaps
exist.

---

## Gap 1 — Admin "list traders" backend is an unimplemented stub (HIGH)

**Evidence (hard, not inferred):**

- Route binding: `api/src/routes/trading_rooms.rs:1107`
  `.route("/traders", get(admin_list_traders))`
- Handler: `api/src/routes/trading_rooms.rs:738-759`
  ```rust
  async fn admin_list_traders(
      State(_state): State<AppState>,
      _admin: AdminUser,
      Query(_query): Query<TradersQuery>,   // query IGNORED (underscore)
  ) -> ... {
      Ok(Json(json!({
          "success": true,
          "data": [],                       // hardcoded empty
          "meta": { "current_page": 1, "per_page": 20, "total": 0, "total_pages": 0 }
      })))
  }
  ```

**Impact, traced end-to-end:**
`frontend/src/routes/api/admin/trading-rooms/traders/+server.ts` proxies
to this route. `FIX-2026-04-26-audit P1-11` removed the proxy's
`mockTraders` fallback so backend failures surface instead of phantom
data. But the backend doesn't *fail* — it returns `success: true` with
`data: []`. Net effect: **the admin Traders list now shows zero traders,
silently, with no error**, because the only real data source
(`mockTraders`) was the interim implementation and the backend was never
built. Deleting `mockTraders` (QW-2's original move) removed the only
thing making this screen functional. **Reverted.**

**What to actually do (the "add, don't delete" the operator called for):**
implement `admin_list_traders` to query the traders table with
`room_slug` / `active_only` filtering + pagination (mirror the working
`admin_list_videos` at `:775`). Until then, `mockTraders` stays
(commented context added in the proxy, array preserved).

---

## Gap 2 — Admin video filter contract mismatch (MEDIUM)

**Evidence:**

| Frontend proxy sends (`videos/+server.ts:145-149`) | Rust `VideosQuery` (`trading_rooms.rs:765-772`) | Match |
|---|---|---|
| `trading_room_id` (numeric) | `room_slug` (string) | ❌ |
| `trader_id` | *(absent)* | ❌ |
| `published_only` | `is_published` | ❌ (renamed) |
| `page` | `page` | ✅ |
| `per_page` | `per_page` | ✅ |

`admin_list_videos` SQL (`trading_rooms.rs:790-840`) filters **only** by
`content_type` and `is_published` against `unified_videos` — there is
**no `trading_room_id` / `trader_id` filter in the query at all**.
Room-scoped listing exists only via a *different* path-based handler
`admin_list_videos_by_room` (`:860`, `/videos/:slug`).

**Impact:** Admin video filtering by room/trader/published-status is a
no-op end-to-end — the proxy forwards `url.searchParams.toString()`
verbatim, but Axum's `Query<VideosQuery>` discards the unknown/renamed
keys. Pagination works (`page`/`per_page` align). This is a latent bug,
not dead code; the parsed consts in the proxy were the *old local-filter*
implementation and are harmless but misleading.

**What to do:** align the contract — either (a) change the proxy/consumer
to send `room_slug` + `is_published` and route room-scoped requests to
`/videos/:slug`, or (b) extend `VideosQuery` + the SQL to accept
`trading_room_id`/`trader_id`. A product/architecture decision, not a
cleanup.

---

## QW-2 corrected outcome

| File | Original QW-2 (wrong) | Corrected (this pass) |
|---|---|---|
| `trading-rooms/videos/+server.ts` | deleted 103-line `mockVideos` + 5 consts | **restored to `main` verbatim** (0 diff) |
| `trading-rooms/traders/+server.ts` | deleted 66-line `mockTraders` + const | **restored to `main` verbatim** (0 diff) |
| `users/[id]/+server.ts` | deleted `json` import | **commented out** (traced: `forwardJson()` used everywhere, `json()` never) |
| `admin/blog/create/+page.svelte` | deleted `type BlogCategory` | **commented out** (traced: page uses string category IDs) |
| `scripts/preview-component.js` | deleted `readFileSync` | **commented out** (traced: only `existsSync` used) |
| `dashboard/Breadcrumbs.svelte` | suppressed false-positive | **kept** — confirmed correct (`breadcrumbSchema` IS consumed in `<svelte:head>` `ld+json`; deleting it = SEO regression) |

Principle reaffirmed: on this codebase, "unused" ≠ "dead." Trace
first → last (frontend proxy → Rust route binding → handler → SQL →
consumer) before removing anything; comment out, don't delete, when the
absence might mean an unfinished feature.
