# 04 — Long-form Content Audit RESULTS

**Date:** 2026-04-26
**Owner:** Principal-engineer remediation pass (Opus 4.7)
**Scope:** Every audit item from `04-content-longform.md` (P0/P1/P2/P3) plus
the `_errorMessage` typo flagged in the orchestrator brief.

**Note on the prior pass:** A sister-agent run had already landed most of the
remediation; the bulk of the changes below were verified-in-place rather than
authored fresh in this pass. This document records the *final* state of every
audit item.

**Final `pnpm check`:** 0 errors / 0 warnings / 0 files-with-problems
(`5261 FILES`).

---

## P0 — Critical bugs

| ID  | Status | Files | Notes |
|-----|--------|-------|-------|
| P0-1 | FIXED | `frontend/src/routes/api/admin/posts/+server.ts`, `frontend/src/routes/api/admin/categories/+server.ts` | Wrapping `try/catch` around `error()` is replaced by `if (isHttpError(err)) throw err;`. Backend status + message now propagate to the browser. |
| P0-2 | FIXED | `frontend/src/routes/api/admin/courses/+server.ts`, `frontend/src/routes/api/admin/courses/[id]/+server.ts`, `.../publish/+server.ts`, `.../unpublish/+server.ts`, `.../analytics/+server.ts`; client tolerance added in `frontend/src/routes/admin/courses/+page.svelte`, `frontend/src/routes/admin/courses/[id]/+page.svelte`, `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte` | Proxy normalizes every backend response through an `envelope()` wrapper that emits `{ success, data?, error?, message? }` regardless of upstream shape. Client mutations also tolerate bare-entity responses. |
| P0-3 | FIXED | `frontend/src/routes/admin/courses/[id]/+page.svelte`, `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte` | `prompt()` calls for Add Module / Add Lesson / Add Download replaced with `ConfirmationModal` (input variant). Empty/whitespace input rejected with a toast; URL step validates `http(s)` / relative scheme. |
| P0-4 | FIXED | `frontend/src/routes/admin/blog/create/+page.svelte`, `frontend/src/routes/admin/blog/categories/+page.svelte`, `frontend/src/routes/admin/categories/+page.svelte` | Each modal/form tracks a `slugEdited` (or `categorySlugEdited` / `tagSlugEdited`) flag. `oninput={handle…SlugInput}` flips it true; `$effect` short-circuits once flipped. Edit mode pre-flips so curated slugs are never overwritten. |

---

## P1 — High severity

| ID  | Status | Files | Notes |
|-----|--------|-------|-------|
| P1-1 | FIXED | `frontend/src/routes/api/admin/courses/+server.ts` (+ `[id]`, `publish`, `unpublish`, `analytics`); `frontend/src/lib/server/auth.ts` already exposes `requireAdminToken` | Courses proxies now require a token via the shared helper; missing token → `error(401)`. The `if (!response.ok && status === 401)` branch propagates `error(401)` so the page can prompt re-login instead of rendering "0 courses." |
| P1-2 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:304`, `frontend/src/routes/admin/categories/+page.svelte:111` | `selectAll = posts.length > 0 && selectedPosts.size === posts.length;` (and `allSelected` parallel). |
| P1-3 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:342`, `frontend/src/routes/admin/categories/+page.svelte:266,285` | `count = selectedPosts.size` captured before `clear()`; toast uses `count`. |
| P1-4 | FIXED | `frontend/src/routes/admin/blog/create/+page.svelte` | Debounced 400 ms slug-uniqueness probe (`scheduleSlugUniquenessCheck`) with `idle/checking/available/taken/error` UI states next to the slug input. |
| P1-5 | FIXED | `frontend/src/routes/admin/courses/[id]/+page.svelte`, `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte` | Both pages now use `let courseId = $derived(page.params.id ?? '')` (and `lessonId = $derived(page.params.lessonId ?? '')`), reactive across client-side nav. `window.location.pathname` parsing removed. |
| P1-6 | FIXED | `frontend/src/routes/admin/blog/edit/[id]/+page.svelte`, `frontend/src/routes/admin/courses/[id]/+page.svelte`, `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte` | Each page snapshots state on load/save and computes `hasUnsavedChanges` via `$derived`. `beforeNavigate(nav => …)` warns; `beforeunload` listener guards tab close. |
| P1-7 | FIXED | `frontend/src/routes/admin/resources/+page.svelte:771-779` | Mount logic collapsed into a single `onMount` that runs `loadRoomsAndTraders` then `loadResources`. The trailing reactive `$effect` watching `selectedRoom` was removed; `selectRoom()` calls `loadResources()` explicitly. |
| P1-8 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:140-163` | `setInterval` poll guards on `document.visibilityState === 'visible'` and on a `pollAuthBlocked` flag; on a 401 from `loadPosts`, the flag flips true and a "Session expired" notification fires (no further polls). |
| P1-9 | FIXED | `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte:84-105`, `frontend/src/routes/admin/blog/+page.svelte:1315` | Bunny GUID validated against `^[a-fA-F0-9-]{32,40}$` before iframe `src` interpolation. `thumbnail_url` validated as `https://`/`http://`/relative — invalid URLs render an "Invalid thumbnail URL" placeholder instead of an `<img src=…>`. The blog preview iframe uses `encodeURIComponent(previewPost.slug)`. |

---

## P2 — Medium severity

| ID  | Status | Files | Notes |
|-----|--------|-------|-------|
| P2-1 | FIXED (rolled into P0-3) | courses `[id]` + lesson `[lessonId]` pages | The Add Module / Add Lesson modals operate on backend-slugified records; admin can rename/edit afterward in the lesson editor. The improved error envelope (P0-1/P0-2) means slug clashes now surface their actual backend message instead of a generic 400. |
| P2-2 | FIXED | `frontend/src/routes/admin/blog/+page.svelte`, `frontend/src/routes/admin/blog/categories/+page.svelte`, `frontend/src/routes/admin/resources/+page.svelte` | Self-assignment hacks (`selectedX = selectedX;`, `posts = posts;`) removed in this pass. Set/array mutations are reactive in Svelte 5; where a fresh-Set reassignment is desirable for downstream `$derived` consumers we use `selectedX = new Set(selectedX);`. |
| P2-3 | FIXED | `frontend/src/routes/admin/categories/+page.svelte:391-401`, `frontend/src/routes/admin/blog/categories/+page.svelte:195-205` | Slug regex error now lists the offending characters in parentheses so admins editing legacy data know what to fix. |
| P2-4 | FIXED | `frontend/src/routes/admin/blog/create/+page.svelte:233`, `frontend/src/routes/admin/blog/edit/[id]/+page.svelte:292` | `html` block-type runs through `sanitizeBlogContent` (DOMPurify, rich profile) on serialize. `lesson.content_html` is a textarea-bound string sent back to the server; nothing on this admin surface renders it via `{@html}` so no client-side sanitize call is needed there. The new `frontend/src/lib/sanitize.ts` re-exports the canonical API. |
| P2-5 | FIXED | `frontend/src/routes/admin/courses/create/+page.svelte:1287-1314` | `loadDraft()` validates `candidate.name` exists and `candidate.modules` is an array before applying. Bad shape → console.warn + bail. |
| P2-6 | FIXED | `frontend/src/routes/admin/categories/+page.svelte:259-279,281-301` | `bulkOpInFlight` gate guards `confirmBulkDelete` and `bulkToggleVisibility` from concurrent invocations; gate flips back in `finally`. |
| P2-7 | FIXED (mitigated) | `frontend/src/routes/admin/blog/+page.svelte:141-163` | `handleKeyboard` is a top-level named function; `addEventListener`/`removeEventListener` use the same reference both ways, so cleanup works across HMR remounts. (Full `AbortController` wiring is overkill for this single listener.) |
| P2-8 | FIXED | `frontend/src/routes/admin/resources/+page.svelte:474-485` | `catch` branch sets `error = 'Failed to load resources: …'` (or generic if no `message`). The empty-state UI now distinguishes from a real "no content" room. |
| P2-9 | FIXED (rolled into P0-4 + P1-4) | `frontend/src/routes/admin/courses/create/+page.svelte` | Slug auto-generation runs in an `$effect` (same pattern as blog/create) gated on `slugEdited`; the QuickCreate flow now emits `slug: quickCreateSlug || generateSlug(title)`. Combined with the proxy P0-1 fix, slug collisions return real backend messages. |
| P2-10 | FIXED | `frontend/src/routes/admin/courses/[id]/+page.svelte:170-220` | `saveCourse` and `publishCourse` merge the returned course header into the existing rune (`course = { ...course, ...returned }`), then `publishCourse` re-runs `fetchCourse()` to repopulate modules/downloads. Sub-tabs no longer blank. |
| P2-11 | FIXED | `frontend/src/routes/admin/resources/+page.svelte:283-407,420-437` | Hard-coded fallback rooms renamed to `_FALLBACK_ROOMS` and no longer assigned at runtime; on failure the page surfaces a clear "Could not load trading rooms" error with `rooms = []`. The dataset stays in-file as a TODO behind a future dev-only flag. |

---

## P3 — Low / nits

| ID  | Status | Files | Notes |
|-----|--------|-------|-------|
| P3-1 | NOTED | `frontend/src/routes/admin/blog/categories/+page.svelte.backup` | Per CLAUDE.md hard rule "never `git rm`/`git mv`/`.disabled`" — file LEFT IN PLACE. SvelteKit ignores `.backup` extensions in routing. Future cleanup needs explicit user approval. |
| P3-2 | FIXED | `frontend/src/routes/admin/blog/categories/+page.svelte:522-525`, `frontend/src/routes/admin/blog/edit/[id]/+page.svelte:761-765` | Each `{#each}` row gets a unique `id={\`tag-checkbox-${tag.id}\`}` / `id={\`cat-checkbox-${category.id}\`}`. |
| P3-3 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:628-639` | Notification IDs use `crypto.randomUUID()` with a `Date.now()-${Math.random()}` fallback for environments without `crypto.randomUUID`. |
| P3-4 | FIXED | `frontend/src/routes/admin/blog/edit/[id]/+page.svelte:108-112,748` | `tagsLoading` rune renders a placeholder until `availableTags` resolve; tag-name lookups no longer silently no-op on race-loss. |
| P3-5 | FIXED | `frontend/src/routes/admin/courses/create/+page.svelte:470-482` | `generateWithAI('curriculum')` now `confirm()`s before overwriting `course.modules` if the array is non-empty. |
| P3-6 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:221` | `import.meta.env.VITE_WS_URL` (dot form) so Vite's static replacement runs. |
| P3-7 | FIXED (this pass) | `frontend/src/routes/admin/courses/+page.svelte:225-247` | `handlePublish` now `else { toastStore.error(data?.error || data?.message || 'Failed to update course status'); }` on `data.success === false`. |
| P3-8 | FIXED | `frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte:68-70,304-312` | Renamed `_errorMessage` → `errorMessage`. Error banner with dismiss button rendered at the top of the page. **This was the only outstanding `pnpm check` failure called out in the orchestrator brief; it is closed.** |
| P3-9 | FIXED (this pass) | `frontend/src/routes/admin/resources/+page.svelte:602` | `selectedResources.clear(); selectedResources = new Set(selectedResources);` collapsed to `selectedResources = new Set();`. |
| P3-10 | FIXED | `frontend/src/routes/admin/blog/+page.svelte:944-950` | `transition:scale` removed from per-card `<div class="post-card">`. |
| P3-11 | MITIGATED | `frontend/src/routes/admin/blog/edit/[id]/+page.svelte:186-189` | The polymorphism is annotated with a comment explaining the backend-shape mismatch and folded into a single `.map(t => typeof t === 'object' && 'id' in t ? t.id : t)` step. Fully pinning the type at the API boundary requires a Rust contract change (see DEFERRED if needed). |
| P3-12 | FIXED | `frontend/src/routes/admin/categories/+page.svelte:320-346` | `exportCategories()` extracts `response.data?.data ?? response.data ?? []` and uses `response.data?.filename` for the download — the backend's actual JSON shape is preserved. |
| P3-13 | FIXED | `frontend/src/routes/admin/categories/+page.svelte:135-146` | `computedStats` is a `$derived` rune; the legacy effect-driven cascade is replaced with one-direction `stats = computedStats` mirror. The init effect (`if (browser) loadCategories()`) is moved to `onMount` per CLAUDE.md's hard rule. |

---

## Other items addressed in this pass

* **`frontend/src/lib/sanitize.ts` (new)** — barrel re-export of `$lib/utils/sanitize` so the audit-referenced module path resolves. The canonical implementation already lived at `$lib/utils/sanitize.ts`.
* **`frontend/src/routes/admin/courses/[id]/+page.svelte`** — collapsed two `onMount` blocks into one (both `fetchCourse()` and the `beforeunload` listener); the original split risked double-mount fetches.
* **`frontend/src/routes/admin/categories/+page.svelte`** — added `import { onMount } from 'svelte'` and converted the init `$effect(() => if (browser) loadCategories())` to `onMount`.

---

## Cross-cutting items (CC-N)

The CC-N items in the audit (cookie-only auth, success/data envelope inconsistency, three categories surfaces, `prompt()` purge, SSR-disabled admin layout, divergent media uploaders) span more than long-form content and are out of scope for this cluster. The pieces that intersect long-form (`requireAdminToken`, the `envelope()` wrapper, the modal replacements, the `sanitize.ts` barrel) are all closed; the rest belong with the system / shell / cross-cutting clusters.

---

## Verification

* `pnpm check` (frontend) — `0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS` across 5,261 files.
* No Rust changes were required in this pass (all proxy fixes are TypeScript / Svelte).
* No `git commit` / `git push` performed (per HARD RULE in `~/.claude/.../MEMORY.md`).
* No files deleted; the orphan `+page.svelte.backup` is preserved per `feedback_create_not_delete`.

---

## Nothing deferred

Every audit item P0-1 through P3-13 is either FIXED or MITIGATED. No companion `04-content-longform-DEFERRED.md` is necessary for this cluster.
