# 04 — Long-form Content Audit (Blog / Courses / Resources / Categories)

**Date:** 2026-04-26
**Scope:** Admin surfaces for long-form content — blog posts, courses, resources, and content categories/tags.
**Repo:** `Revolution-trading-pros` @ `main`
**Mode:** Read-only forensic review.

---

## Files reviewed

### Admin pages (Svelte)
- [`frontend/src/routes/admin/blog/+page.svelte`](../../../frontend/src/routes/admin/blog/+page.svelte) — 3,092 LOC
- [`frontend/src/routes/admin/blog/create/+page.svelte`](../../../frontend/src/routes/admin/blog/create/+page.svelte) — 1,255 LOC
- [`frontend/src/routes/admin/blog/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte) — 1,398 LOC
- [`frontend/src/routes/admin/blog/categories/+page.svelte`](../../../frontend/src/routes/admin/blog/categories/+page.svelte) — 1,473 LOC
- `frontend/src/routes/admin/blog/categories/+page.svelte.backup` — 598 LOC (orphan backup)
- [`frontend/src/routes/admin/courses/+page.svelte`](../../../frontend/src/routes/admin/courses/+page.svelte) — 1,189 LOC
- [`frontend/src/routes/admin/courses/[id]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte) — 1,134 LOC
- [`frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte) — 864 LOC
- [`frontend/src/routes/admin/courses/create/+page.svelte`](../../../frontend/src/routes/admin/courses/create/+page.svelte) — 4,857 LOC
- [`frontend/src/routes/admin/resources/+page.svelte`](../../../frontend/src/routes/admin/resources/+page.svelte) — 2,534 LOC
- [`frontend/src/routes/admin/categories/+page.svelte`](../../../frontend/src/routes/admin/categories/+page.svelte) — 1,768 LOC

### Admin proxy endpoints (SvelteKit)
- [`frontend/src/routes/api/admin/posts/+server.ts`](../../../frontend/src/routes/api/admin/posts/+server.ts)
- [`frontend/src/routes/api/admin/posts/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/posts/[...rest]/+server.ts)
- [`frontend/src/routes/api/admin/posts/stats/+server.ts`](../../../frontend/src/routes/api/admin/posts/stats/+server.ts)
- [`frontend/src/routes/api/admin/courses/+server.ts`](../../../frontend/src/routes/api/admin/courses/+server.ts)
- [`frontend/src/routes/api/admin/courses/[id]/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/+server.ts)
- [`frontend/src/routes/api/admin/courses/[id]/publish/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/publish/+server.ts)
- [`frontend/src/routes/api/admin/courses/[id]/unpublish/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/unpublish/+server.ts)
- [`frontend/src/routes/api/admin/courses/[id]/analytics/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/analytics/+server.ts)
- [`frontend/src/routes/api/admin/categories/+server.ts`](../../../frontend/src/routes/api/admin/categories/+server.ts)
- [`frontend/src/routes/api/admin/categories/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/categories/[...rest]/+server.ts)
- [`frontend/src/routes/api/admin/tags/+server.ts`](../../../frontend/src/routes/api/admin/tags/+server.ts)
- [`frontend/src/routes/api/admin/tags/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/tags/[...rest]/+server.ts)

### Supporting libs (read for context, not audited as primary scope)
- [`frontend/src/lib/utils/createProxyShim.ts`](../../../frontend/src/lib/utils/createProxyShim.ts)
- [`frontend/src/lib/api/admin.ts`](../../../frontend/src/lib/api/admin.ts) (categoriesApi, tagsApi)

---

## Critical bugs (P0)

### P0-1 — POST/PUT proxy collapses every backend error into "Invalid request body" 400

**File:** [`frontend/src/routes/api/admin/posts/+server.ts:81-99`](../../../frontend/src/routes/api/admin/posts/+server.ts), and the parallel block at lines `116-133` (PUT). Identical pattern in [`frontend/src/routes/api/admin/categories/+server.ts:78-95`](../../../frontend/src/routes/api/admin/categories/+server.ts).

```ts
try {
    const body = await request.json();
    const { data, status } = await fetchFromBackend('/admin/posts', { ... });
    if (status >= 400) {
        error(status, 'Failed to create post');     // throws HttpError
    }
    return json(data);
} catch (err) {
    console.error('POST /api/admin/posts error:', err);
    error(400, 'Invalid request body');             // catches the HttpError above
}
```

In SvelteKit 2 `error()` **throws** an `HttpError`. The surrounding `try/catch` catches that throw and re-throws as `error(400, 'Invalid request body')`. The result: every non-2xx response from the Rust backend (validation 422, conflict 409, server 500) is rewritten to a 400 with the misleading body `Invalid request body`. The browser code in `blog/create/+page.svelte:243-251` reads `result.message` to surface — so users see "Invalid request body" for slug collisions, missing fields, transient 500s, and the like.

**Impact:** every actionable error from the backend is lost; admins cannot self-diagnose (slug conflicts in particular look identical to client-side validation failures).

**Suggested fix:** rethrow `HttpError` instances from the catch (`if (err instanceof HttpError) throw err;`) or restructure to avoid wrapping the upstream-error path.

---

### P0-2 — Course creation modal ignores backend success flag, navigates on undefined `data.success`

**File:** [`frontend/src/routes/admin/courses/+page.svelte:115-138`](../../../frontend/src/routes/admin/courses/+page.svelte)

The proxy at [`api/admin/courses/+server.ts:65-88`](../../../frontend/src/routes/api/admin/courses/+server.ts) returns whatever the Rust backend returns verbatim (`return json(data, { status: response.status })`); when the backend returns `{ data: { id: ... } }` (no top-level `success`), `data.success` is undefined.

```ts
const data = await adminFetch('/api/admin/courses', { method: 'POST', body: ... });
if (data.success && data.data?.id) {
    goto(`/admin/page-builder?course=${data.data.id}`);
} else {
    quickCreateError = data.error || 'Failed to create course';
}
```

If the Rust API returns `{ id: ... }` (without the `success: true` wrapper, which it usually does — see other endpoints throughout the codebase), the modal will silently report "Failed to create course" even though the course was created in the DB. **The user retries, creating duplicate courses with `(2)`-style slug suffixes.**

The same `data.success` shape assumption is hard-coded across [`courses/[id]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte) (`fetchCourse`, `saveCourse`, `publishCourse`, `addModule`, `addLesson`, `handleFileUpload`, every single mutation) and [`courses/[id]/lessons/[lessonId]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte).

**Impact:** wherever the backend returns a bare entity vs. a wrapped `{ success, data }`, every save silently fails the UI assertion and the user re-saves, causing duplicate inserts. Combined with the absence of slug uniqueness checks on the frontend (see P1-3), this is a recipe for orphaned/duplicated courses.

---

### P0-3 — Lesson editor uses native `prompt()` for Add Module / Add Lesson / Add Download (data-loss + no validation)

**File:** [`frontend/src/routes/admin/courses/[id]/+page.svelte:169`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte), `:207`, and [`frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte:166-169`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte).

```ts
const title = prompt('Enter module title:');
if (!title) return;
// ...immediately POSTs to /api/admin/courses/${courseId}/modules
```

```ts
const title = prompt('Enter download title:');
if (!title) return;
const fileUrl = prompt('Enter file URL (or leave empty to use course downloads):');
```

Issues:
1. `prompt()` is **blocked entirely in some browsers** (Chrome blocks it on cross-origin iframes; many embedded webviews and progressive enhancement contexts strip it) — clicking "Add Module" silently does nothing.
2. There is no validation, no length limit, no slug suggestion, no description, no module assignment — the moment the prompt closes, a row is written to the DB.
3. Hitting "Cancel" returns `null` → the function early-returns silently — but **hitting OK with an empty string** triggers `title.trim()` only via the prompt's default — and an empty-string title is happily POSTed.
4. The `fileUrl` `prompt` accepts arbitrary text — no URL validation, no allow-listing, no XSS scrub. That value is then bound to the lesson and may be rendered as a download link.

**Impact:** broken UX, silent failures in modern browsers, garbage data in the DB. The rest of the codebase has a `ConfirmationModal` component already in use here for delete flows — there's no excuse for `prompt()`.

---

### P0-4 — Dual unguarded slug-mutation `$effect`s on `blog/create` race against user input

**File:** [`frontend/src/routes/admin/blog/create/+page.svelte:307-311`](../../../frontend/src/routes/admin/blog/create/+page.svelte)

```svelte
$effect(() => {
    if (post.title) {
        generateSlug();
    }
});
```

`generateSlug()` (line 135-142) checks `if (!post.slug && post.title)` — but the effect re-runs on every keystroke of `post.title`. If the user has manually edited the slug to something deliberate and then *adjusts the title* (a very common edit flow), the effect fires, sees `post.slug` is non-empty, and no-ops correctly. **But** if the user clears the slug field for any reason — for instance to retype it — the very next title keystroke clobbers it. There is no debounce, no "user-edited slug" flag, no original-vs-derived comparison.

Worse, in [`admin/blog/categories/+page.svelte:379-390`](../../../frontend/src/routes/admin/blog/categories/+page.svelte) and [`admin/categories/+page.svelte:122-126`](../../../frontend/src/routes/admin/categories/+page.svelte), the equivalent effects are *unconditional* on the slug field:

```svelte
$effect(() => {
    if (categoryForm.name && !editingCategory) {
        categoryForm.slug = generateSlug(categoryForm.name);
    }
});
```

Once the modal is open in **create** mode, every name-keystroke overwrites the slug. The user cannot type a custom slug — every character they type into Slug is wiped on the next Name re-render. (This was the exact "shadow-state" pattern the CLAUDE.md flagged in commit `05acf3231`; the fix didn't reach these files.)

**Impact:** users cannot author custom slugs while typing names; slug collisions become more frequent because every category gets the auto-generated default.

---

## High-severity issues (P1)

### P1-1 — Course-related proxies omit unauthorized error path; cookie-only auth means token expiry → empty-list grid silently

**Files:**
- [`api/admin/courses/+server.ts:30-46`](../../../frontend/src/routes/api/admin/courses/+server.ts)
- [`api/admin/courses/[id]/+server.ts:14-41`](../../../frontend/src/routes/api/admin/courses/[id]/+server.ts), `:43-67`, `:69-91`
- [`api/admin/courses/[id]/publish/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/publish/+server.ts)
- [`api/admin/courses/[id]/unpublish/+server.ts`](../../../frontend/src/routes/api/admin/courses/[id]/unpublish/+server.ts)

Unlike the posts/categories/tags shims which `error(401, 'Unauthorized')` when the cookie is missing, the courses proxies **ignore a missing token entirely**:

```ts
const token = cookies.get('rtp_access_token');
// ...
const response = await fetch(`${BACKEND_URL}/api/admin/courses${queryString}`, {
    headers: {
        ...(token ? { Authorization: `Bearer ${token}` } : {})
    }
});
```

The backend returns 401, then the `if (response.status === 401 || response.status === 404)` branch returns an **empty success-shaped payload**:

```ts
return json({ success: true, data: { courses: [], total: 0, ... } });
```

So when the admin's session expires, the courses page renders "0 courses" with no error and the only signal is that the new-course modal works (because it returns a different error). This actively hides session expiry.

**Impact:** indistinguishable from "you have no courses." Admins get no nudge to re-login.

---

### P1-2 — `selectedPosts.size === posts.length` evaluates `true` when both are zero → "select all" toggles silently

**File:** [`frontend/src/routes/admin/blog/+page.svelte:287`](../../../frontend/src/routes/admin/blog/+page.svelte)

```ts
selectAll = selectedPosts.size === posts.length;
```

When `posts.length === 0` (empty list, mid-load, or filtered to nothing), `selectAll` flips to `true`, and the master checkbox becomes checked even though there's nothing to select. Subsequent `togglePostSelection` calls in this state can mismatch the UI from the underlying Set.

Same pattern in [`admin/categories/+page.svelte:111-113`](../../../frontend/src/routes/admin/categories/+page.svelte) (`allSelected`) — guarded by `filteredCategories.length > 0`, which is the correct fix and should be ported back.

---

### P1-3 — Bulk-status notification reads `selectedPosts.size` after `clear()`, always shows `Updated 0 posts`

**File:** [`frontend/src/routes/admin/blog/+page.svelte:317-337`](../../../frontend/src/routes/admin/blog/+page.svelte)

```ts
loadPosts();
loadStats();
selectedPosts.clear();
selectAll = false;
showNotification('success', `Updated ${selectedPosts.size} posts to ${newStatus}`);
```

`selectedPosts.size` is `0` after `.clear()`. Every successful bulk-status change tells the user "Updated 0 posts to published." The sibling `confirmBulkDelete` (line 298-315) gets this right by capturing `count = selectedPosts.size` before clearing — port the pattern.

---

### P1-4 — No frontend slug-uniqueness check on blog create/edit; collisions surface as P0-1 generic 400

**Files:** [`admin/blog/create/+page.svelte`](../../../frontend/src/routes/admin/blog/create/+page.svelte), [`admin/blog/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte), [`admin/courses/+page.svelte`](../../../frontend/src/routes/admin/courses/+page.svelte) QuickCreate.

The slug input is free-form; there is no `/api/admin/posts/check-slug` style probe before submission, no debounced validation, no warning. When the backend rejects with a unique-constraint error, the proxy bug (P0-1) collapses it to "Invalid request body."

**Impact:** an admin types a fresh slug, hits Publish, sees "Invalid request body," cannot tell what's wrong.

---

### P1-5 — `[id]/lessons/[lessonId]` derives `courseId`/`lessonId` from `window.location.pathname` (skips Svelte router)

**File:** [`frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte:55-64`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte)

```ts
let courseId = $state('');
let lessonId = $state('');

onMount(() => {
    const pathParts = window.location.pathname.split('/');
    lessonId = pathParts[pathParts.length - 1];
    courseId = pathParts[pathParts.length - 3];
    fetchLesson();
});
```

Same pattern in [`admin/courses/[id]/+page.svelte:84-88`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte). These pages should use SvelteKit's `page.params.id` / `page.params.lessonId` (as `admin/blog/edit/[id]/+page.svelte:60` correctly does). The path-string approach:
- Breaks if any trailing slash is added (split returns an empty final element).
- Breaks if the URL gains a query string or hash (it doesn't here, but it would silently bake the query into `lessonId`).
- Doesn't update on client-side navigation between two lessons (the data fetches once on mount, never re-runs when the route changes — sibling navigation requires a hard reload).

**Impact:** navigating between lessons within a course displays stale data until the user hits F5.

---

### P1-6 — Course detail / lesson editor lacks unsaved-changes guard; closing the tab discards work

**Files:** [`admin/courses/[id]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte), [`admin/courses/[id]/lessons/[lessonId]/+page.svelte`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte), [`admin/blog/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte)

The course-create page tracks `hasUnsavedChanges` and runs a 30s autosave; the **course-edit and blog-edit pages do not**. There's no `beforeunload` handler. The user types an entire lesson description, switches tabs, and loses everything if the laptop sleeps.

The lesson-editor `saveLesson()` doesn't even surface its `_errorMessage` state — note the leading underscore (line 70) and the lack of any UI binding in the template; failed saves are silently swallowed.

---

### P1-7 — Resources page double-loads on mount (effect + selectRoom both call `loadResources`)

**File:** [`admin/resources/+page.svelte:744-774`](../../../frontend/src/routes/admin/resources/+page.svelte)

```svelte
$effect(() => {
    if (!browser) return;
    const init = async () => {
        await loadRoomsAndTraders();
        if (selectedRoom) {
            await loadResources();          // ← load #1
        }
    };
    init();
});

let previousRoomId = $state<number | null>(null);
$effect(() => {
    if (selectedRoom && !isLoadingRooms && previousRoomId !== null && previousRoomId !== selectedRoom.id) {
        loadResources();
    }
    if (selectedRoom) {
        previousRoomId = selectedRoom.id;
    }
});
```

The mount effect awaits `loadRoomsAndTraders`, which sets `selectedRoom = rooms[0]`. The second effect tracks `selectedRoom`; on the first run, `previousRoomId` is `null`, so `loadResources()` is correctly skipped — but the assignment `previousRoomId = selectedRoom.id` happens *inside the effect that read `selectedRoom`*, classic write-while-reading.

`selectRoom()` (line 476-480) also calls `loadResources()` directly, racing with the trailing effect and producing two in-flight requests for the same room when the user clicks a tab.

The whole double-effect dance is the legacy pattern CLAUDE.md flagged. It should collapse into a single `onMount` plus an explicit click handler, no derived effect.

---

### P1-8 — Blog admin polling refresh fires every 30s with no auth retry; runs even when the tab is hidden

**File:** [`admin/blog/+page.svelte:136-156`](../../../frontend/src/routes/admin/blog/+page.svelte)

```ts
$effect(() => {
    if (!browser) return;
    loadPosts();
    loadStats();
    setupWebSocket();
    setupKeyboardShortcuts();
    refreshInterval = setInterval(() => {
        loadStats();
        if (viewMode === 'list') loadPosts();
    }, 30000);
    return () => { ... };
});
```

The interval keeps firing while the tab is in the background. There's no `document.visibilityState === 'visible'` gate. Combined with the cookie-token expiry behavior (P1-1 in courses, similar here), an idle tab keeps refreshing into an expired session and silently rendering empty stats forever.

---

### P1-9 — Lesson `bunny_video_guid` is bound directly into iframe `src` without sanitization

**File:** [`admin/courses/[id]/lessons/[lessonId]/+page.svelte:302-309`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte)

```svelte
<iframe
    src="https://iframe.mediadelivery.net/embed/{lesson.bunny_video_guid}"
    title={lesson.title}
    allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
    allowfullscreen
></iframe>
```

The user can paste arbitrary text into the GUID field at line 357 (`bind:value={lesson.bunny_video_guid}`), and that value is interpolated unescaped into the iframe `src`. The Bunny CDN itself will likely 404 garbage, but the input is also susceptible to:
- `https://iframe.mediadelivery.net/embed/abc?..` injection — a `?` or `&` in the GUID gets through and could craft the embed URL.
- Spaces / newlines pasted in are not validated.

It's an admin-only surface, so the XSS surface is low — but the GUID format (`UUID-style`) should be regex-validated. Same for `thumbnail_url` (line 421) which is freely-set and rendered into `<img src={lesson.thumbnail_url}>` without `javascript:`-scheme protection.

The blog admin preview iframe ([`admin/blog/+page.svelte:1285`](../../../frontend/src/routes/admin/blog/+page.svelte)) interpolates `previewPost.slug` straight into the URL — backend-controlled, but if a malicious slug like `foo?preview=true&inject=<script>` ever made it into the DB, it would run in the same origin. URL-encode before interpolating.

---

## Medium issues (P2)

### P2-1 — `[id]/+page.svelte` `prompt()` flow lacks slug auto-generation; new modules/lessons land with title-derived slugs only on the backend

This is the same flow as P0-3 but viewed from the data side: created modules/lessons have no client-side slug; they depend entirely on the backend slugifier, which the frontend then can't preview. Combined with P0-1, an admin who tries to give two modules the same title gets "Invalid request body" with no hint that it's a slug clash.

### P2-2 — `admin/blog/categories/+page.svelte` writes `selectedCategories = selectedCategories;` self-assignment hack to force reactivity (Svelte 5 anti-pattern)

**File:** [`admin/blog/categories/+page.svelte:499-500`](../../../frontend/src/routes/admin/blog/categories/+page.svelte), `:600`, etc.

```ts
selectedCategories.add(category.id);
// ...
selectedCategories = selectedCategories;
```

In Svelte 5, `Set`/`Map` mutations *are* reactive — the self-assignment is unnecessary noise and a dead giveaway that the developer tested in legacy mode. [`admin/categories/+page.svelte:393`](../../../frontend/src/routes/admin/categories/+page.svelte) does it the right way: `selectedIds = new Set(selectedIds);`. Pick one and standardize.

### P2-3 — `categoryForm.slug` rune validation regex `/^[a-z0-9-]+$/` rejects valid empty-after-trim cases inconsistently

**File:** [`admin/categories/+page.svelte:359-367`](../../../frontend/src/routes/admin/categories/+page.svelte) and `admin/blog/categories/+page.svelte:186-194`.

```ts
if (!categoryForm.slug.trim()) formErrors.push('Slug is required');
else if (!/^[a-z0-9-]+$/.test(categoryForm.slug)) {
    formErrors.push('Slug can only contain lowercase letters, numbers, and hyphens');
}
```

But the auto-generator (`generateSlug`) also strips underscores, accents, etc. — produces a slug that always passes the regex. So the regex is dead code on the create path. On edit, however, an existing slug containing a stray uppercase character or underscore (legacy data) cannot be saved without manual fixing — and the error message doesn't tell you the offending character.

### P2-4 — `admin/blog/create/+page.svelte` & `edit/[id]` keep an always-visible HTML compatibility layer (`blocksToHtml`) that is never sanitized

**File:** [`admin/blog/edit/[id]/+page.svelte:227-256`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte)

```ts
function blocksToHtml(blocks: Block[]): string {
    return blocks.map(block => {
        // ...
        case 'paragraph':
            return `<p>${content.text || ''}</p>`;
        case 'heading':
            return `<h${level}>${content.text || ''}</h${level}>`;
        case 'html':
            return content.html || '';
        // ...
    }).join('\n');
}
```

The `html` block-type passes user-provided HTML through verbatim and the resulting string is sent back to the server as `content`. There is no sanitizer (no DOMPurify, no allow-list). On the server side this is presumably trusted (admin-only). When this content is rendered on the public blog with `{@html}` (very likely), an admin account compromise is XSS-everywhere. The mitigation is server-side sanitization on the Rust side — but the frontend should at least DOMPurify-on-write so the editor preview matches reality.

Same applies to `lesson.content_html` ([`courses/[id]/lessons/[lessonId]/+page.svelte:369`](../../../frontend/src/routes/admin/courses/[id]/lessons/[lessonId]/+page.svelte)).

### P2-5 — Course-create page ignores localStorage corruption on draft load

**File:** [`admin/courses/create/+page.svelte:1275-1290`](../../../frontend/src/routes/admin/courses/create/+page.svelte)

```ts
function loadDraft() {
    const draftStr = localStorage.getItem('course-draft');
    if (draftStr && draftDate) {
        try {
            const draft = JSON.parse(draftStr);
            pendingDraft = { course: draft.course || draft, date };
            showLoadDraftModal = true;
        } catch (e) {
            console.error('Failed to load draft:', e);
        }
    }
}
```

The fallback `draft.course || draft` will accept a top-level draft that is missing the `course` key — meaning an old draft from a different schema version overwrites the entire `course` $state with arbitrary fields. There's no schema check; whatever shape the user had last is reapplied. If two course-create tabs are open, draft 1 silently overwrites draft 2.

### P2-6 — `confirmBulkDelete` and friends call `loadCategories()` after a Set mutation but without awaiting; two consecutive bulk operations race

**File:** [`admin/categories/+page.svelte:247-260`](../../../frontend/src/routes/admin/categories/+page.svelte)

`bulkDelete` → `await categoriesApi.bulkDelete(...)` → `selectedIds = new Set()` → `await loadCategories()`. If a user clicks bulk-delete twice in quick succession (or the network is slow and the success modal stacks), the second invocation operates on the cleared `selectedIds` — silently a no-op — but neither flow cleans up the in-flight loading state. Add an "operation in progress" gate.

### P2-7 — `admin/blog/+page.svelte` registers a `keydown` listener inside the mount effect's setup but the cleanup uses an unbound reference

**File:** [`admin/blog/+page.svelte:151-156`](../../../frontend/src/routes/admin/blog/+page.svelte)

```ts
return () => {
    if (ws) ws.close();
    if (refreshInterval) clearInterval(refreshInterval);
    document.removeEventListener('keydown', handleKeyboard);
};
```

`handleKeyboard` is a closure that captures `posts`, `selectedPosts`, etc. via `bulkDelete()`. When the page hot-reloads in dev (or when the admin layout remounts after auth-flow changes), the second mount adds a new listener but `removeEventListener` on the *old* effect uses a different closure each time. Result: leaked listeners that pile up across navigation. Use a stable named function or an `AbortController`-backed listener.

### P2-8 — Resources page swallows backend failures without surfacing them

**File:** [`admin/resources/+page.svelte:457-465`](../../../frontend/src/routes/admin/resources/+page.svelte)

```ts
} catch {
    // ICT 7: CORB or API failure - show empty state, don't crash
    resources = [];
    // Don't set error - just show empty state
}
```

The user can't tell whether a room genuinely has no resources or whether the request 500'd. Every error masquerades as "no content found." Particularly damaging for the alert-service rooms where a missing weekly-alert resource is a content-team mistake — but the dashboard makes it look intentional.

### P2-9 — Course-create payload sends empty `slug: undefined`; relies on backend slugifier with no client-side preview

**File:** [`admin/courses/create/+page.svelte:1330`](../../../frontend/src/routes/admin/courses/create/+page.svelte)

```ts
slug: course.slug || undefined,
```

Because `generateSlug()` only fires on `onblur` of the title (line 1879) and only if `course.slug` is empty, a user who types a title and clicks "Publish" before the title input loses focus may submit with `slug: undefined`. Couple with `productsApi.create` returning a server-generated slug that the user has no visibility into until they reload the courses list.

### P2-10 — `admin/courses/[id]/+page.svelte` does not refetch course state after `publishCourse`

**File:** [`admin/courses/[id]/+page.svelte:150-166`](../../../frontend/src/routes/admin/courses/[id]/+page.svelte)

```ts
if (data.success) {
    course = data.data;
}
```

If the publish endpoint returns *just* the course header (no modules, no downloads, no unassigned lessons), this assignment overwrites the rich tree built up by `fetchCourse`, blanking the Content / Downloads tabs until the user navigates away and back. Same pattern in `saveCourse` line 134.

### P2-11 — Resources page eagerly hard-codes a 6-room `FALLBACK_ROOMS` list; the actual rooms are configurable in the backend

**File:** [`admin/resources/+page.svelte:283-402`](../../../frontend/src/routes/admin/resources/+page.svelte)

When the rooms API fails, the admin UI shows the six legacy room names + slugs even if the production DB has different names. New rooms added in the backend won't appear without a frontend rebuild; renamed rooms display under the old name. Move to a strict empty-state when the API fails.

---

## Low / nits (P3)

### P3-1 — `admin/blog/categories/+page.svelte.backup` is an orphan checked-in file

**File:** `frontend/src/routes/admin/blog/categories/+page.svelte.backup` (598 LOC).

Per CLAUDE.md's hard rule "CREATE, never DELETE — never `git mv` to `.disabled`/`.bak`," this file should not be checked in. It risks SvelteKit's file-router picking it up if anyone renames it back accidentally. Either delete or move out of the tree.

### P3-2 — Many `<input>` elements on the same page share `id="page-checkbox"`

**Files:** [`admin/blog/categories/+page.svelte:489`](../../../frontend/src/routes/admin/blog/categories/+page.svelte), `:590`, [`admin/blog/edit/[id]/+page.svelte:706-710`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte). Inside `{#each ...}` blocks, `id="page-checkbox"` is rendered N times — invalid HTML, breaks `<label for="...">` association and screen-reader navigation.

### P3-3 — `admin/blog/+page.svelte` notification IDs use `Date.now()` — collisions if two notifications fire in the same millisecond

**File:** [`admin/blog/+page.svelte:608-615`](../../../frontend/src/routes/admin/blog/+page.svelte)

```ts
const id = Date.now();
notifications = [...notifications, { id, type, message }];
```

Use `crypto.randomUUID()` (already imported elsewhere) or a monotonic counter. Otherwise the keyed-each block will tear when two `showNotification`s land in the same tick.

### P3-4 — `admin/blog/edit/[id]/+page.svelte` `loadTags()` & `loadPost()` race; `availableTags` may be empty when the post's tags are rendered

**File:** [`admin/blog/edit/[id]/+page.svelte:117-119`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte)

```ts
onMount(async () => {
    await Promise.all([loadPost(), loadTags()]);
});
```

Conceptually fine since `Promise.all` awaits both — but `availableTags.find((t) => t.id === tagId)` on line 673 silently no-ops for tags whose API call lost the race in dev (e.g., dev server warm-up timing). Render a loading state for the tags list separately from the post.

### P3-5 — `admin/courses/create/+page.svelte` `generateWithAI('curriculum')` writes a hard-coded fake curriculum to `course.modules`

**File:** [`admin/courses/create/+page.svelte:470-650`](../../../frontend/src/routes/admin/courses/create/+page.svelte)

The "AI Assistant" sidebar button overwrites whatever the user has typed with a 200-line hard-coded technical-analysis curriculum. There's no confirmation, no merge, just `course.modules = [ ... 50 hard-coded objects ... ]`. Add a confirmation modal or remove the dead-code marketing feature.

### P3-6 — `admin/blog/+page.svelte` logs `import.meta.env['VITE_WS_URL']` lookup in dot-bracket form

**File:** [`admin/blog/+page.svelte:206`](../../../frontend/src/routes/admin/blog/+page.svelte)

`import.meta.env['VITE_WS_URL']` works but the conventional spelling is `import.meta.env.VITE_WS_URL`. TS-side it's the same; the bracket form bypasses Vite's static replacement on some configurations and ships the literal string to production. Verify the build output.

### P3-7 — `admin/courses/+page.svelte` toast for handlePublish failure but not for `data.success === false` case

**File:** [`admin/courses/+page.svelte:217-234`](../../../frontend/src/routes/admin/courses/+page.svelte)

```ts
if (data.success) {
    course.is_published = !course.is_published;
    course.status = course.is_published ? 'published' : 'draft';
    courses = [...courses];
}
```

If `data.success` is false, no error is shown — the button silently does nothing. Match the pattern in `handleDelete`.

### P3-8 — `admin/courses/[id]/lessons/[lessonId]/+page.svelte:70` declares `_errorMessage` (leading underscore) but UI never displays it

```ts
let _errorMessage = $state('');
```

Variable starts with underscore by convention "unused" — but the function bodies write to it (`_errorMessage = 'Failed to delete lesson'`). The user never sees the error. Either render it or stop writing to it. Currently every lesson save/delete failure is silent.

### P3-9 — `admin/resources/+page.svelte` `selectedResources.clear(); selectedResources = new Set(selectedResources);`

**File:** [`admin/resources/+page.svelte:567-569`](../../../frontend/src/routes/admin/resources/+page.svelte)

Same dead self-assignment as P2-2.

### P3-10 — `admin/blog/+page.svelte` `transition:scale` on every post card causes janky re-renders on filter changes

**File:** [`admin/blog/+page.svelte:923`](../../../frontend/src/routes/admin/blog/+page.svelte)

Animating in/out 50+ post cards on every filter change is a perf hit. Use `animate:flip` for reordering or no transition for filters.

### P3-11 — `admin/blog/edit/[id]/+page.svelte:151` indexes raw API tag arrays via `(typeof t === 'object' ? t.id : t)`

This polymorphism papers over a backend-shape mismatch. Pin the type at the API boundary and remove the runtime branch.

### P3-12 — `admin/categories/+page.svelte:300-310` builds a JSON blob inline instead of calling the actual `/admin/categories/export` proxy

```ts
const response = await categoriesApi.export();
const blob = new Blob([JSON.stringify(response.data, null, 2)], { type: 'application/json' });
```

This is a re-serialize-then-download dance. If the backend export endpoint already returns a content-disposition CSV/JSON, the blob roundtrip strips file metadata. Confirm what the backend produces.

### P3-13 — Many pages register `$effect` for derived computations that should be `$derived`

Examples:
- [`admin/categories/+page.svelte:128-136`](../../../frontend/src/routes/admin/categories/+page.svelte) — stats computed in an effect, write-while-reading the `categories` rune.
- [`admin/courses/create/+page.svelte:1566-1571`](../../../frontend/src/routes/admin/courses/create/+page.svelte) — `validateAll()` and `updateCourseDuration()` invoked from effect chains.

Convert to `$derived` to remove implicit-update cascades (the recurring "shadow-state" problem).

---

## Cross-cutting concerns

### CC-1 — Auth-cookie inconsistency between proxy generations
Posts, categories, and tags proxies use `cookies.get('rtp_access_token')` AND fall back to `Authorization: Bearer …` from the request header. Courses proxies use cookie-only with no header fallback (and fail open with empty data on missing token, see P1-1). Standardize on the helper in [`createProxyShim.ts`](../../../frontend/src/lib/utils/createProxyShim.ts) or a single auth utility and have every long-form proxy share it.

### CC-2 — `success`/`data` envelope assumption is inconsistent
Some endpoints return `{ success: true, data: ... }`; others (especially course mutations) return the bare entity, prompting the `data.data || data` defensive coding seen in [`admin/blog/edit/[id]/+page.svelte:125`](../../../frontend/src/routes/admin/blog/edit/[id]/+page.svelte). Pin one envelope shape across the Rust API contract and update the TypeScript type definitions; remove the runtime branches.

### CC-3 — Three different "categories" surfaces with overlapping but-not-identical schemas
- `admin/blog/categories` operates on `categoriesApi` (full Category type w/ `parent_id`, `meta_*`).
- `admin/categories` operates on the same `categoriesApi` but with a totally different UI/UX (drag/drop, merge).
- `admin/blog/+page.svelte` and `admin/blog/create` use a HARD-CODED `predefinedCategories` array of 18 entries with `id: string` (not `number`!) — entirely disconnected from the database categories and the categoriesApi.

This means a post's `categories: string[]` field stores symbolic IDs (`'market-analysis'`) that don't exist in the categories DB table — they're just strings. The "Categories & Tags" admin page and the "Blog" admin page **manage two unrelated concepts** that look identical to the admin. Pick one model.

### CC-4 — `prompt()` and `alert()` were partially purged
The recent FIX-2026-04-26 sweep replaced `alert()` with `toastStore` in many places — visible in the comment markers throughout `admin/courses/[id]/+page.svelte`. But `prompt()` was missed. Finish the sweep with a `ConfirmationModal`-style input modal (already used elsewhere in the codebase).

### CC-5 — Long-form admin pages are SSR-disabled with no role check on either side of the wire
[`admin/+layout.ts`](../../../frontend/src/routes/admin/+layout.ts) sets `ssr=false; prerender=false;` and [`admin/+layout.svelte`](../../../frontend/src/routes/admin/+layout.svelte) checks `isAuthenticated.current` in `onMount` only — there's no role-gate. Anyone with a valid `rtp_access_token` cookie can render the admin UI. Authorization is presumed to be 100% on the Rust side. This is fine in principle but means the *UI* surface (delete buttons, publish buttons, etc.) shows up for non-admin users until they click and get a 403, which is then mishandled per P0-1.

### CC-6 — Image upload paths diverge across surfaces
- Blog edit/create uses `mediaApi.uploadFile` with `optimize: true, generate_webp: true`.
- Course create uses `adminFetch('/api/admin/media/upload', { ... })` with FormData and a 50MB cap.
- Course `[id]` edit uses a Bunny presigned-URL flow (`upload-url` then PUT to the storage host).
- Lesson editor uses `BunnyVideoUploader` component.
- Resources page uses `roomResourcesApi`.

Each path has its own error-handling, fallback (some use `URL.createObjectURL` on failure — leaking blob URLs), and state machine. Consolidate around one media uploader and one media-record schema.

---

## Summary

The long-form admin surface boots, lets you click the right buttons, and writes the right rows in the happy path. Outside the happy path it falls apart. **Two P0 bugs corrupt error semantics** end-to-end (the proxy double-throw and the `data.success` envelope assumption) — between them, every backend validation, conflict, and 5xx surfaces to the admin as either a misleading "Invalid request body" 400 or as silent success. **One P0 UX bug** keeps `prompt()` in the create-module / create-lesson / create-download flows, which is unsupported in modern browser contexts and unguarded against empty-string entries. **One P0 reactivity bug** in the categories modals overwrites user-typed slugs on every name keystroke.

The blog and resources lists tab around fine, but they hide stale-session and partial-auth states behind empty-state UIs (P1-1). Bulk-action notifications lie about counts (P1-3). The courses sub-routes derive params from `window.location.pathname` instead of the SvelteKit router and don't react to client-side navigation (P1-5). The single `predefinedCategories` array hard-coded in three places (CC-3) means a chunk of the categories admin manages content categories that the public blog never reads.

The proxy layer is otherwise consistently using `$env/dynamic/private` (no hardcoded backend URLs in the in-scope routes — confirmed across all twelve `+server.ts` files audited). XSS surface is mostly limited to admin-trusted HTML being passed through without client-side sanitization (`html` block-type + `lesson.content_html`); the Rust side is presumed to sanitize on render.

**Recommended order of fix:**
1. Strip the wrapping try/catch from posts/categories proxies (P0-1) — 10-line patch, unblocks every other issue.
2. Standardize the response envelope (CC-2) — kills P0-2 and the `data.data || data` rot.
3. Replace `prompt()` flows with the existing modal system (P0-3, CC-4).
4. Fix the slug auto-generation effects (P0-4).
5. Audit category proxy auth + courses proxy auth to a single shared helper (CC-1).
