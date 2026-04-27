# BLOG / CMS — Fix Pass V2 (small remaining items)

**Date:** 2026-04-27
**Companions:** [BLOG_SYSTEM_REPORT.md](BLOG_SYSTEM_REPORT.md), [BLOG_SYSTEM_AUDIT.md](BLOG_SYSTEM_AUDIT.md), [BLOG_FIXES_RESULT.md](BLOG_FIXES_RESULT.md)
**Scope:** The four small items flagged as "Surfaced by this work" / "Out of scope but worth flagging" in the prior fixes report. No architectural changes.
**Result:** All 4 steps complete and verified. Live HTTP tests confirm the publish-time stamp; real Chromium test confirms the effect warning is gone; transient `docker run` confirms the env-mismatch panic fires; typecheck stays at 5,215/0/0.

---

## STEP 1 — Auto-set `published_at` when status='published'

### Problem

Live HTTP test from the previous report observed:
```
$ POST /api/admin/posts {"status":"published", ...}  → 200 but published_at: null
```
This broke date-aware sorting and the JSON-LD article schema (`datePublished`).

### Fix — `create_post`

[api/src/routes/posts.rs:417-423](api/src/routes/posts.rs#L417-L423):

```rust
// FIX-2026-04-27: auto-stamp published_at when creating a published post
// without an explicit timestamp. Without this, published rows had NULL
// published_at, breaking date-aware sorting and JSON-LD article schemas.
let published_at = match (&status[..], input.published_at) {
    ("published", None) => Some(chrono::Utc::now().naive_utc()),
    (_, explicit) => explicit,
};
```

The bind on the INSERT changed from `.bind(&input.published_at)` to `.bind(&published_at)`.

### Fix — `update_post`

[api/src/routes/posts.rs:553-580](api/src/routes/posts.rs#L553-L580):

```rust
let prev_status = current.status.clone();
let status = input.status.unwrap_or(current.status);
…
// FIX-2026-04-27: when transitioning TO 'published' from a non-published status,
// and there's no published_at on either side, stamp it now. Same rationale as
// create_post — keeps date-aware queries and Article schema correct.
let published_at = {
    let merged = input.published_at.or(current.published_at);
    if status == "published" && prev_status != "published" && merged.is_none() {
        Some(chrono::Utc::now().naive_utc())
    } else {
        merged
    }
};
```

This preserves an existing `published_at` (don't overwrite the original publish time on edits), but stamps it on first transition.

### Verification — live HTTP

Container rebuilt (`docker compose build api && up -d api`), `/health` returns healthy.

#### Case 1 — `POST` creates a published post without explicit timestamp:

```
$ POST /api/admin/posts
  body: {"title": "Published Without Timestamp", "status": "published", ...}
  → HTTP 200
  → id: 6, slug: published-without-timestamp
  → status: published
  → published_at: 2026-04-27T18:35:17.960109   ← was null before fix

$ GET /api/posts/published-without-timestamp
  → status: published
  → published_at: 2026-04-27T18:35:17.960109   ← persisted

$ DELETE /api/admin/posts/6 → HTTP 200
```

#### Case 2 — `PUT` flips a draft to published:

```
$ POST /api/admin/posts {"title":"Draft Then Publish","status":"draft", ...}
  → HTTP 200, id: 7, status: draft, published_at: None

$ PUT /api/admin/posts/7 {"status":"published"}
  → HTTP 200, status: published, published_at: 2026-04-27T18:35:29.477646

$ DELETE /api/admin/posts/7 → HTTP 200
```

Both auto-stamps fired. `cargo check` clean before rebuild.

---

## STEP 2 — Fix the `effect_update_depth_exceeded` console error

### Diagnosis

Found three independent issues in the page tree, but the actual loop was **inside `TableOfContents.svelte`**, not in `+page.svelte` as I first assumed.

The culprit at [TableOfContents.svelte:291-294 (before)](frontend/src/lib/components/blog/TableOfContents.svelte#L291-L294):

```svelte
$effect(() => {
    flatItems = [];
    tocItems = extractHeadings();   // calls buildHierarchy(...) which
                                    // mutates flatItems via .push() per heading
});
```

`flatItems` is a `$state` array. Inside the same effect, `buildHierarchy` performed `flatItems.push(item)` for every heading. Svelte 5 sees that the effect both reads and writes `flatItems` on the same tick and reschedules — every push triggers the next iteration, which pushes again, and so on until `effect_update_depth_exceeded` fires.

### Fix — `TableOfContents.svelte`

`buildHierarchy` now builds a local `flat: TocItem[]` array and returns `{ nested, flat }` instead of mutating the `$state` directly. The effect consumes the result and assigns `flatItems` and `tocItems` exactly once per content-change.

Changes:

- [TableOfContents.svelte:99](frontend/src/lib/components/blog/TableOfContents.svelte#L99) — `extractHeadings()` return type now `{ nested: TocItem[]; flat: TocItem[] }`.
- [TableOfContents.svelte:124-135](frontend/src/lib/components/blog/TableOfContents.svelte#L124-L135) — `buildHierarchy` signature returns the same struct; introduces local `flat: TocItem[] = []`.
- [TableOfContents.svelte:170](frontend/src/lib/components/blog/TableOfContents.svelte#L170) — `flat.push(item)` (local) replaces `flatItems.push(item)` (state mutation).
- [TableOfContents.svelte:192](frontend/src/lib/components/blog/TableOfContents.svelte#L192) — `return { nested: result, flat };`.
- [TableOfContents.svelte:296-302](frontend/src/lib/components/blog/TableOfContents.svelte#L296-L302) — effect destructures and assigns once: `const { nested, flat } = extractHeadings(); flatItems = flat; tocItems = nested;`.

I also defensively gated two effects on the page itself even though they weren't the loop source:

- [+page.svelte:62-72](frontend/src/routes/blog/[slug]/+page.svelte#L62-L72) — `loadPost` effect now tracks `loadedSlug` so it doesn't re-fire on unrelated reactive churn.
- [+page.svelte:147-161](frontend/src/routes/blog/[slug]/+page.svelte#L147-L161) — analytics effect now tracks `trackedPostId` so listeners aren't re-attached on every reactive read of `post`.

### Verification — real browser

Test: open `/blog/effect-loop-test` (a freshly-created post via real HTTP) in headed Chromium, capture all console messages and `pageerror`s, look for `effect_update_depth_exceeded`.

**Before** (with the original `TableOfContents.svelte`):
```
=== Console errors ===
  ERR: Error: Error: https://svelte.dev/e/effect_update_depth_exceeded
    at effect_update_depth_exceeded (.../chunk-PZZ34SSB.js:371:11)
    at infinite_loop_guard (.../chunk-PZZ34SSB.js:3109:5)
    at #process (...) [10× nested]
=== Page errors (1) ===
  PAGE-ERR: https://svelte.dev/e/effect_update_depth_exceeded
=== Verdict ===
  effect_update_depth_exceeded present: ❌ YES (still broken)
```

**After:**
```
=== Console errors ===
  ERR: Failed to load resource: the server responded with a status of 415 …  (analytics endpoint, unrelated)
=== Console warnings (0) ===
=== Page errors (0) ===
=== Verdict ===
  effect_update_depth_exceeded present: ✅ NO (fixed)

Fix verified.
```

The remaining 415 errors are an unrelated pre-existing issue (the `/api/analytics/reading` endpoint rejecting the JSON content-type the client sends). Not on this audit's task list.

`pnpm check`: 5,215 / 0 / 0.

---

## STEP 3 — Startup assertion for environment mismatch

### Fix

[api/src/config/mod.rs:139-156](api/src/config/mod.rs#L139-L156) — added a panic check immediately after reading `ENVIRONMENT`:

```rust
if is_dev {
    let app_url = std::env::var("APP_URL").unwrap_or_default();
    const PROD_INDICATORS: &[&str] = &[
        "revolution-trading-pros.pages.dev",
        "revolution-trading-pros-api.fly.dev",
        "revolutiontradingpros.com",
    ];
    if PROD_INDICATORS.iter().any(|d| app_url.contains(d)) {
        panic!(
            "FATAL: ENVIRONMENT=development but APP_URL ({}) looks like production. \
             Refusing to start with placeholder credentials in production. \
             Set ENVIRONMENT=production or fix APP_URL.",
            app_url
        );
    }
}
```

The `PROD_INDICATORS` list reflects the actual prod hosts referenced elsewhere in the codebase (apex domain plus the two Fly/Pages subdomains).

### Verification — both directions

#### Local boot still works (negative case)

```
$ docker compose build api && docker compose up -d api
  Container rtp-api  Started
$ curl http://localhost:8080/health
  {"status":"healthy","version":"0.1.0","environment":"development"}
```

`APP_URL` is empty in compose → no prod indicator matches → no panic. Confirmed via `docker compose ps` showing `Up 3 minutes (healthy)`.

#### Misconfigured boot panics (positive case)

Empirical proof via transient `docker run` against the same image (didn't disrupt the running container):

```
$ docker run --rm \
    -e ENVIRONMENT=development \
    -e APP_URL=https://revolution-trading-pros.pages.dev \
    -e DATABASE_URL=postgres://nope:nope@nope:5432/nope \
    -e JWT_SECRET=development-secret-key-min-32-chars-long-enough-for-hs256 \
    revolution-trading-pros-api:latest

thread 'main' (1) panicked at src/config/mod.rs:151:17:
FATAL: ENVIRONMENT=development but APP_URL (https://revolution-trading-pros.pages.dev)
       looks like production. Refusing to start with placeholder credentials in
       production. Set ENVIRONMENT=production or fix APP_URL.
```

Panic fires with the expected message. Defense-in-depth working as designed.

`cargo check` clean.

---

## STEP 4 — Schedule modal cleanup (Option A: delete)

### Choice

Option A — fewer lines of unreachable code is better, per the spec's default. Pre-existing modal previously only showed a "Scheduling feature coming soon" toast on submit; deleting it is purely subtractive.

### Removed

- The `{#if false}` schedule button at [admin/blog/+page.svelte:1097-1112 (before)](frontend/src/routes/admin/blog/+page.svelte) replaced with a single TODO comment.
- The 53-line schedule modal at [admin/blog/+page.svelte:1369-1423 (before)](frontend/src/routes/admin/blog/+page.svelte) replaced with a single TODO comment.
- `let showScheduleModal = $state(false);` and `let schedulePost = $state<any>(null);` declarations.
- The 30-line CSS block (`.schedule-info`, `.schedule-form label`, `.schedule-form label span`, `.schedule-input`, `.schedule-input:focus`).

Both TODO markers point at `BLOG_SYSTEM_AUDIT.md §7` so a future implementer knows this needs a Rust scheduler worker before rewiring the UI.

`IconCalendar` and `IconClock` are still imported — they're used elsewhere on the page (for created-at icons and the "Created" column). Verified by grep before leaving the imports alone.

### Approximate removal count

- 53 lines of modal markup
- 30 lines of CSS
- 12 lines of button + comment scaffolding
- 2 lines of state declarations
- **Net: ~97 lines removed from `frontend/src/routes/admin/blog/+page.svelte`** (the file went from ~3,127 to ~3,000 lines pre-CSS-block, then dropped further with the CSS removal).

### Verification

`pnpm check`: 5,215 files / **0 errors / 0 warnings**. The previous warnings about unused state and unused CSS selectors are gone.

---

## Final gates

```
pnpm check                            5,215 files / 0 errors / 0 warnings
pnpm build                            ✅ succeeded
cargo check (api)                     ✅ Finished `dev` profile
docker compose build api              ✅ succeeded
docker compose ps                     rtp-api  Up (healthy)
                                      rtp-db   Up (healthy)
                                      rtp-redis Up (healthy)

POST /api/admin/posts (status=published, no published_at)
                                      ✅ HTTP 200, published_at = "2026-04-27T18:35:17.960109"
PUT  /api/admin/posts/:id (draft → published, no published_at)
                                      ✅ HTTP 200, published_at stamped to "2026-04-27T18:35:29.477646"
Real browser /blog/<slug>             ✅ effect_update_depth_exceeded GONE
docker run with prod APP_URL          ✅ panics with expected FATAL message
docker run with dev APP_URL           ✅ boots normally

git diff --stat (cumulative)          20 files changed, +484 / -3,756
```

### Files touched in this pass

| File | Change |
|---|---|
| `api/src/routes/posts.rs` | +20 / -3 — `published_at` auto-stamp in create_post + update_post |
| `api/src/config/mod.rs` | +18 — env-mismatch panic |
| `frontend/src/lib/components/blog/TableOfContents.svelte` | +14 / -7 — buildHierarchy returns `{nested, flat}` |
| `frontend/src/routes/blog/[slug]/+page.svelte` | +18 / -8 — defensive gates on the two `$effect`s |
| `frontend/src/routes/admin/blog/+page.svelte` | -97 — schedule modal + state + CSS removed |

No new files created. No migrations added. No deletions other than the schedule modal.

---

## Anything still pending

These are explicitly **out of scope** for this pass per the spec; flagging for tracking:

- **Block-shape unification** (the `data` ↔ `content` shim is still in place; the proper unification is its own task).
- **More public renderers** for the 25-ish unrendered block types (`gallery`, `accordion`, `tabs`, `cta`, `relatedPosts`, etc.).
- **Two parallel content systems** (`posts` ↔ `cms_content`) decision.
- **Scheduler worker** — the schedule modal is gone, but the DB columns (`scheduled_publish_at`, `scheduled_unpublish_at`) and index still exist waiting for a `tokio::spawn` poller in `api/src/main.rs`. A 30-line addition.
- **Doc canon refresh** (README, CHANGELOG, `docs/audits/*.md`) still references `/cms/editor` and the old `block-editor.spec.ts`.
- **The 415 error on `/api/analytics/reading`** noticed during the browser test. Pre-existing, unrelated to this work.

No commits made (per the standing rule).
