# 07 — Marketing / Conversion Audit (Email / Popups / Forms / SEO)

Scope: every file under
- [`frontend/src/routes/admin/email/`](../../../frontend/src/routes/admin/email)
- [`frontend/src/routes/admin/popups/`](../../../frontend/src/routes/admin/popups)
- [`frontend/src/routes/admin/forms/`](../../../frontend/src/routes/admin/forms)
- [`frontend/src/routes/admin/seo/`](../../../frontend/src/routes/admin/seo)
- [`frontend/src/routes/api/admin/email/`](../../../frontend/src/routes/api/admin/email)

Reviewed read-only. Date: 2026-04-26. Author: principal engineer.

---

## Files reviewed

### Email (admin pages)
- [`admin/email/campaigns/+page.svelte`](../../../frontend/src/routes/admin/email/campaigns/+page.svelte)
- [`admin/email/campaigns/[id]/report/+page.svelte`](../../../frontend/src/routes/admin/email/campaigns/[id]/report/+page.svelte)
- [`admin/email/campaigns/[id]/report/+page.ts`](../../../frontend/src/routes/admin/email/campaigns/[id]/report/+page.ts)
- [`admin/email/settings/+page.svelte`](../../../frontend/src/routes/admin/email/settings/+page.svelte)
- [`admin/email/smtp/+page.svelte`](../../../frontend/src/routes/admin/email/smtp/+page.svelte)
- [`admin/email/subscribers/+page.svelte`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte)
- [`admin/email/templates/+page.svelte`](../../../frontend/src/routes/admin/email/templates/+page.svelte)
- [`admin/email/templates/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/email/templates/edit/[id]/+page.svelte)
- [`admin/email/templates/new/+page.svelte`](../../../frontend/src/routes/admin/email/templates/new/+page.svelte)
- [`admin/email/templates/preview/[id]/+page.svelte`](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte)
- [`lib/components/admin/TemplateForm.svelte`](../../../frontend/src/lib/components/admin/TemplateForm.svelte) (sibling — used by edit/new)

### Email (proxy endpoints)
- [`api/admin/email/settings/+server.ts`](../../../frontend/src/routes/api/admin/email/settings/+server.ts)
- [`api/admin/email/templates/+server.ts`](../../../frontend/src/routes/api/admin/email/templates/+server.ts)
- [`api/admin/email/templates/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/email/templates/[...rest]/+server.ts)

### Popups
- [`admin/popups/+page.svelte`](../../../frontend/src/routes/admin/popups/+page.svelte)
- [`admin/popups/new/+page.svelte`](../../../frontend/src/routes/admin/popups/new/+page.svelte)
- [`admin/popups/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte)
- [`admin/popups/[id]/analytics/+page.svelte`](../../../frontend/src/routes/admin/popups/[id]/analytics/+page.svelte)

### Forms
- [`admin/forms/+page.svelte`](../../../frontend/src/routes/admin/forms/+page.svelte)
- [`admin/forms/create/+page.svelte`](../../../frontend/src/routes/admin/forms/create/+page.svelte)
- [`admin/forms/entries/+page.svelte`](../../../frontend/src/routes/admin/forms/entries/+page.svelte)
- [`admin/forms/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/forms/[id]/edit/+page.svelte)
- [`admin/forms/[id]/edit/+page.ts`](../../../frontend/src/routes/admin/forms/[id]/edit/+page.ts)
- [`admin/forms/[id]/submissions/+page.svelte`](../../../frontend/src/routes/admin/forms/[id]/submissions/+page.svelte)
- [`admin/forms/[id]/submissions/+page.ts`](../../../frontend/src/routes/admin/forms/[id]/submissions/+page.ts)
- [`admin/forms/[id]/analytics/+page.svelte`](../../../frontend/src/routes/admin/forms/[id]/analytics/+page.svelte)
- [`admin/forms/[id]/analytics/+page.ts`](../../../frontend/src/routes/admin/forms/[id]/analytics/+page.ts)

### SEO
- [`admin/seo/+page.svelte`](../../../frontend/src/routes/admin/seo/+page.svelte)
- [`admin/seo/meta/+page.svelte`](../../../frontend/src/routes/admin/seo/meta/+page.svelte)
- [`admin/seo/redirects/+page.svelte`](../../../frontend/src/routes/admin/seo/redirects/+page.svelte)
- [`admin/seo/schema/+page.svelte`](../../../frontend/src/routes/admin/seo/schema/+page.svelte)
- [`admin/seo/sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/sitemap/+page.svelte)
- [`admin/seo/keywords/+page.svelte`](../../../frontend/src/routes/admin/seo/keywords/+page.svelte)
- [`admin/seo/analytics/+page.svelte`](../../../frontend/src/routes/admin/seo/analytics/+page.svelte)
- [`admin/seo/analysis/+page.svelte`](../../../frontend/src/routes/admin/seo/analysis/+page.svelte)
- [`admin/seo/404s/+page.svelte`](../../../frontend/src/routes/admin/seo/404s/+page.svelte)
- [`admin/seo/404-monitor/+page.svelte`](../../../frontend/src/routes/admin/seo/404-monitor/+page.svelte)
- [`admin/seo/settings/+page.svelte`](../../../frontend/src/routes/admin/seo/settings/+page.svelte)
- [`admin/seo/news-sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/news-sitemap/+page.svelte)
- [`admin/seo/video-sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/video-sitemap/+page.svelte)
- [`admin/seo/image-seo/+page.svelte`](../../../frontend/src/routes/admin/seo/image-seo/+page.svelte)
- [`admin/seo/store-locator/+page.svelte`](../../../frontend/src/routes/admin/seo/store-locator/+page.svelte)
- [`admin/seo/reports/+page.svelte`](../../../frontend/src/routes/admin/seo/reports/+page.svelte)
- [`admin/seo/search-console/+page.svelte`](../../../frontend/src/routes/admin/seo/search-console/+page.svelte)
- [`admin/seo/bing/+page.svelte`](../../../frontend/src/routes/admin/seo/bing/+page.svelte)

---

## Critical bugs (P0)

### P0-1 — Email settings endpoint stores SMTP credentials in a process-global variable, plaintext, **shared across all admins/instances**, with **no auth check**

[`api/admin/email/settings/+server.ts:11-21`](../../../frontend/src/routes/api/admin/email/settings/+server.ts)

```ts
// Default SMTP settings (in production, these would come from database)
let emailSettings = {
    provider: 'smtp',
    host: '',
    port: 587,
    username: '',
    password: '',
    encryption: 'tls',
    from_address: '',
    from_name: 'Revolution Trading Pros'
};
```

This is a module-scoped `let` variable. Five separate failures stack here:

1. **No auth.** [`GET`](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L23) and [`POST`](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L36) check no cookie, no role, nothing. Anyone can hit `/api/admin/email/settings` and read `host`, `username`, encryption mode, sender domain — that is a credential leak even without the password.
2. **Plaintext password.** No KMS, no hashing, not even base64.
3. **Process-global state.** On Cloudflare Pages / Fly the process is multi-tenant and ephemeral. Every cold start wipes settings; every concurrent request reads the same `let`.
4. **No backend persistence.** Comment at L11 says "in production, these would come from database" — that's an incomplete feature shipped to production. The corresponding [admin/email/settings/+page.svelte:74-77](../../../frontend/src/routes/admin/email/settings/+page.svelte#L74) calls `apiFetch('/admin/email/settings')` which goes through `[...path]` to the **Rust API**, but the SvelteKit `+server.ts` directly above it short-circuits that — so the page sometimes hits the local in-memory shim and sometimes hits Rust depending on `apiFetch` semantics. Inconsistent state.
5. **Duplicate page.** `/admin/email/settings/+page.svelte` and `/admin/email/smtp/+page.svelte` are byte-for-byte the same UI hitting the same endpoint. Two URLs writing to the same in-memory blob with no coordination.

**Fix:** delete this `+server.ts` file entirely. Let `[...path]` proxy `/admin/email/settings` to Rust where credentials must be stored encrypted (libsodium, AWS KMS, or Fly secrets) and read with auth + role check.

---

### P0-2 — SMTP password sent **back to the client** in plaintext on every reload

[`api/admin/email/settings/+server.ts:25-29`](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L25)

```ts
return json({
    ...emailSettings,
    password: emailSettings.password ? '••••••••' : ''
});
```

The masking pattern is correct in spirit but the `loadSettings()` page logic at [`admin/email/settings/+page.svelte:56-63`](../../../frontend/src/routes/admin/email/settings/+page.svelte#L56) does `settings = data` and then submits the entire `settings` object back via `bind:value` on the password field. So on save without retyping the password the bullets `••••••••` get sent to the server, where [`L52`](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L52) checks `body.password !== '••••••••'` to skip the overwrite — **but the `testConnection` path at [L101-105](../../../frontend/src/routes/admin/email/settings/+page.svelte#L101) sends `settings` (with bullets as the password) to `/admin/email/settings/test`.** That endpoint isn't in this scope, but if it forwards `body.password` as-is to the SMTP test, every "Test Connection" click after page reload tests with the literal string `••••••••` — silently misleading users and leaking that the real password is unknown to the form.

Two fixes: (a) never include the password field, even masked, in `GET` responses — return `has_password: boolean`; (b) on the client, treat `password === ''` as "use stored value" and send no `password` key at all.

---

### P0-3 — Email-templates SQL/SQLSTATE error swallowing presents stale UX, masks misconfig

[`admin/email/templates/+page.svelte:69-89`](../../../frontend/src/routes/admin/email/templates/+page.svelte#L69)

```ts
if (
    e.message.includes('SQLSTATE') ||
    e.message.includes('Column not found') ||
    e.message.includes('deleted_at')
) {
    error =
        'Database configuration issue. Email templates feature is being set up. Please try again later or contact support.';
}
```

The page silently catches `SQLSTATE`/`Column not found`/`deleted_at` SQL errors from Rust and displays a generic "feature is being set up" message. This means:

- A real outage looks like a UX bug to admins.
- The template-list page goes blank without telling ops there's a schema drift.
- This was clearly added as a workaround for a missing migration. Either run the migration or fix the query — the workaround makes the bug invisible.

Cross-reference: [`admin/email/templates/+page.svelte:49-51`](../../../frontend/src/routes/admin/email/templates/+page.svelte#L49) loads via `$effect(() => { loadTemplates(); })` which is a Svelte 5 antipattern — should be `onMount`. With no deps, the effect re-runs on any reactive change in scope.

---

### P0-4 — Popups list page: delete button sets state but no `ConfirmationModal` is rendered

[`admin/popups/+page.svelte:19-77`](../../../frontend/src/routes/admin/popups/+page.svelte#L19)

```ts
let _showDeleteModal = $state(false);
let pendingDeleteId = $state<string | null>(null);
…
function handleDelete(popupId: string) {
    pendingDeleteId = popupId;
    _showDeleteModal = true;
}

async function _confirmDeletePopup() {
    …
    await deletePopup(popupId);
    …
}
```

The leading underscore on `_showDeleteModal` and `_confirmDeletePopup` means they're flagged as unused. Searching the template body — there is no `<ConfirmationModal isOpen={_showDeleteModal} … />` anywhere on the page. The delete button at [L245](../../../frontend/src/routes/admin/popups/+page.svelte#L245) calls `handleDelete()` which just sets dead state. **Clicking "Delete" in the popups admin does nothing visible.** Users will think the click was missed and click again — multiple times — until something reloads. Worst case: someone opens devtools and pokes `_confirmDeletePopup()` directly, deleting a popup with no confirmation at all.

**Fix:** wire up `<ConfirmationModal>` like every other admin list page (cf. [`admin/email/subscribers/+page.svelte:589-600`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L589)) and rename to drop the underscores.

Same pattern in [`admin/popups/[id]/edit/+page.svelte:46`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte#L46): `_showAbTestModal` is set true at [L783](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte#L783) but never rendered. Click "Create A/B Test" → nothing happens.

---

### P0-5 — `/admin/popups/new` `{@html formData.content}` in live preview without sanitization

[`admin/popups/new/+page.svelte:578-583`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L578)

```svelte
<div
    class="preview-popup-content"
    style="color: {formData.design?.textColor || '#4b5563'}"
>
    {@html formData.content}
</div>
```

The textarea at [L674](../../../frontend/src/routes/admin/popups/new/+page.svelte#L674) accepts raw HTML, the placeholder even brags "HTML allowed". The preview executes that HTML in the admin's authenticated session. This is a self-XSS only as long as the admin is the sole author — but:

- Multi-admin: any admin who can edit popups can plant a `<script>` in `content` and harvest the next admin's session via `/api/auth/me`, escalating to account takeover.
- The same `formData.content` is persisted and rendered to every site visitor by the runtime popup component (out of this scope, but the sink is upstream).
- [`admin/popups/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte) edits the same field with no sanitization and no preview escape.

**Fix:** import `sanitizeHtml` from `$lib/utils/sanitize` (already used by [`templates/preview/[id]/+page.svelte:39`](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte#L39)) and wrap: `{@html sanitizeHtml(formData.content, 'rich')}`. Sanitize the persisted value server-side too.

---

### P0-6 — `admin/popups/new` calls `/api/admin/forms` which does not exist as a proxy

[`admin/popups/new/+page.svelte:83-93`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L83)

```ts
async function loadForms() {
    try {
        const response = await fetch('/api/admin/forms');
        if (response.ok) {
            const data = await response.json();
            availableForms = data.forms || data || [];
        }
    } catch (e) {
        console.error('Failed to load forms:', e);
    }
}
```

There is no `frontend/src/routes/api/admin/forms/` directory. The request hits the catch-all [`api/[...path]/+server.ts`](../../../frontend/src/routes/api/[...path]/+server.ts) which proxies to Rust at `/api/admin/forms` — that backend route may or may not exist (out of scope to verify). The result either way: silent failure, "Form Integration" select stays empty, "Include form in popup" feature never works. No toast. The popup can be saved with `has_form: true, form_id: undefined` which will likely break the runtime.

Same shape applies to [`admin/email/campaigns/+page.svelte:174`](../../../frontend/src/routes/admin/email/campaigns/+page.svelte#L174) calling `/api/admin/email/templates` (this one **does** exist) and [L191](../../../frontend/src/routes/admin/email/campaigns/+page.svelte#L191) calling `/api/admin/members/segments` (not in scope, presumably exists).

---

## High-severity issues (P1)

### P1-1 — `TemplateForm` shadow-state pattern violates `CLAUDE.md` Svelte 5 rule

[`lib/components/admin/TemplateForm.svelte:14-41`](../../../frontend/src/lib/components/admin/TemplateForm.svelte#L14)

```ts
const template = $derived(props.template ?? null);
…
let name = $state('');
let slug = $state('');
…
$effect(() => {
    if (template) {
        name = template.name ?? '';
        slug = template.slug ?? '';
        …
    }
});
```

`CLAUDE.md` explicitly forbids `let foo = $state(props.foo ?? null); $effect(...)` (the "shadow-state" pattern that emits `state_referenced_locally` warnings). This file is exactly that. The `$effect` writes to local state derived from a prop — exactly the pattern fixed in commit `05acf3231`.

Replacement: drive the form fields directly off `props.template` using `$bindable` or `$derived`, or accept the prop as a one-shot initial value via `untrack(() => …)`.

---

### P1-2 — `TemplateForm` `JSON.parse(variables)` with no try/catch — admin can crash the save

[`lib/components/admin/TemplateForm.svelte:46-58`](../../../frontend/src/lib/components/admin/TemplateForm.svelte#L46)

```ts
const payload = {
    …
    variables: variables ? JSON.parse(variables) : [],
    …
};
```

The `variables` textarea is free-form JSON (placeholder reads "Enter JSON variables"). Any typo throws `SyntaxError` synchronously inside `handleSubmit` — caught by the outer `catch (e)` at [L68](../../../frontend/src/lib/components/admin/TemplateForm.svelte#L68) which sets `error = 'Failed to save template'`. The admin sees a generic error with no indication their JSON is invalid. Worse, the loading spinner gets stuck if the throw bypasses `loading = false` (it doesn't here because of `finally`, but the UX is opaque).

**Fix:** validate with a try/catch around `JSON.parse` and surface `error = 'Variables must be valid JSON: ' + e.message` before submission.

---

### P1-3 — Email-template `delete` uses `confirm()` browser dialog, inconsistent with rest of admin

[`admin/email/templates/+page.svelte:100-101`](../../../frontend/src/routes/admin/email/templates/+page.svelte#L100)

```ts
async function deleteTemplate(id: number) {
    if (!confirm('Are you sure you want to delete this template?')) return;
```

Every other delete in this audit uses `<ConfirmationModal>` from `$lib/components/admin/ConfirmationModal.svelte`. Native `confirm()` blocks the JS thread and is unstyled. Trivial fix; flagging because of inconsistency cited in `feedback_no_delete_comment_first.md`.

Same for [`admin/seo/redirects/+page.svelte:62`](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L62), [`admin/seo/redirects/+page.svelte:92`](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L92) (bulkDelete), [`admin/seo/schema/+page.svelte:46`](../../../frontend/src/routes/admin/seo/schema/+page.svelte#L46), and [`admin/seo/settings/+page.svelte:73`](../../../frontend/src/routes/admin/seo/settings/+page.svelte#L73) (`alert('Settings saved successfully!')`).

---

### P1-4 — Subscriber bulk-delete does N synchronous round-trips, no atomicity, no progress

[`admin/email/subscribers/+page.svelte:160-173`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L160)

```ts
async function confirmBulkDelete() {
    showBulkDeleteModal = false;
    try {
        for (const id of selectedSubscribers) {
            await emailApi.deleteSubscriber(id);
        }
        toastStore.success(`Deleted ${selectedSubscribers.size} subscribers`);
```

Sequential `await` in a `for…of` over potentially hundreds of subscribers. If the 47th DELETE 500s, 46 are deleted, no rollback, the catch swallows the failure and **toasts success for the deletes that didn't happen**. Selected set is cleared regardless.

**Fix:** add a `bulkDelete([…])` endpoint (Rust transaction) — same pattern as [`admin/seo/redirects/+page.svelte:91`](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L91) which already does it for redirects.

---

### P1-5 — Auth in email templates proxy is correct but missing `cache: 'no-store'`, no body forwarding for non-JSON

[`api/admin/email/templates/+server.ts:24-34`](../../../frontend/src/routes/api/admin/email/templates/+server.ts#L24)

```ts
const response = await fetch(`${backendUrl}/api${endpoint}`, {
    ...options,
    headers: {
        'Content-Type': 'application/json',
        Accept: 'application/json',
        ...(options?.headers || {})
    }
});

const data = await response.json();
```

- `await response.json()` will throw on a 404 HTML body or empty 204. The outer `catch` returns `status: 500` losing the real status.
- Hardcoded `Content-Type: application/json` overrides any incoming multipart (not relevant for templates today, but if attachments are added it's a footgun).
- No timeout. The catch-all proxy has the 30s timeout + retry; this purpose-built proxy doesn't.

**Fix:** prefer letting `[...path]` proxy `/admin/email/templates` and delete this file. The shim at [`templates/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/email/templates/[...rest]/+server.ts) is a workaround for the SvelteKit POST=405 cliff on the parent — but `[...path]` already handles all methods.

---

### P1-6 — SEO redirects page: regex redirects are saved with **no client-side validation**

[`admin/seo/redirects/+page.svelte:225-249`](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L225)

The page lists `is_regex` redirects (rendering a "REGEX" badge at L240) but neither the list page nor the editor (out of scope, in `RedirectEditor.svelte`) is shown to validate the source pattern with `new RegExp()`. A malformed regex committed to the redirect table will:
- Crash whatever Rust route handles redirect dispatch.
- Or worse, slip through and ReDoS the request handler with a pathological pattern (e.g. `(a+)+$` against a long URL).

The Rust side **must** compile-and-bound-check regexes before persisting. Frontend should pre-validate so the user sees errors before round-trip. Currently, the Add Redirect button at [L137](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L137) opens an editor whose validation is opaque from this audit.

---

### P1-7 — Popups edit page lost the include/exclude page-targeting fields that exist in `new`

Compare [`admin/popups/new/+page.svelte:1100-1140`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L1100) (renders include/exclude page targeting textareas) with [`admin/popups/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte) (no such fields anywhere).

Editing a popup loses the ability to change page-targeting rules. If you need to change include/exclude pages you must duplicate and delete. The display/frequency rules read from `formData.display_rules` are partially preserved but include/exclude pages never bind, never save. Same regression for: Secondary CTA, Video Embed, Form Integration, Overlay Settings — all features that exist on `/new` but not `/[id]/edit`.

This is a feature-parity bug, not a bug per se. P1 because users will unknowingly destroy popup config every time they hit Save.

---

### P1-8 — `admin/seo/404-monitor` double-loads on first render

[`admin/seo/404-monitor/+page.svelte:32-35,114-118`](../../../frontend/src/routes/admin/seo/404-monitor/+page.svelte#L32)

```ts
onMount(() => {
    loadLogs();
    loadStats();
});
…
$effect(() => {
    if (sortBy) {
        loadLogs();
    }
});
```

Both fire on first render: `onMount` calls `loadLogs()`, and `$effect` reads `sortBy` (initial value `'hits'`) → calls `loadLogs()` again. Two HTTP calls per page load. The fix is to drop `onMount` and let `$effect` cover the initial load (since `sortBy` always has a value), or use `$effect.pre` with a "hasMounted" guard.

---

## Medium issues (P2)

### P2-1 — Email-templates `+server.ts` and the `[...rest]` shim are redundant with `[...path]`

[`api/admin/email/templates/+server.ts`](../../../frontend/src/routes/api/admin/email/templates/+server.ts) and [`api/admin/email/templates/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/email/templates/[...rest]/+server.ts) both forward to the same `/admin/email/templates` Rust route the catch-all `[...path]` already proxies. The duplication adds two failure modes (the JSON-only proxy in P1-5 and the createProxyShim — fine but unnecessary).

Either delete them, or migrate every `/api/admin/<thing>` route off `[...path]` for consistency. Pick one.

### P2-2 — Duplicate routes: `/admin/email/settings` and `/admin/email/smtp`

[`admin/email/settings/+page.svelte`](../../../frontend/src/routes/admin/email/settings/+page.svelte) and [`admin/email/smtp/+page.svelte`](../../../frontend/src/routes/admin/email/smtp/+page.svelte) are functionally identical UIs hitting the same endpoint. Per `feedback_create_not_delete.md`, comment out the lesser-linked one with a `TODO` rather than `git rm`.

### P2-3 — Duplicate routes: `/admin/seo/404s` and `/admin/seo/404-monitor`

[`admin/seo/404s/+page.svelte`](../../../frontend/src/routes/admin/seo/404s/+page.svelte) and [`admin/seo/404-monitor/+page.svelte`](../../../frontend/src/routes/admin/seo/404-monitor/+page.svelte) both display 404 errors with bulk-delete actions. They use **different** API clients (`seoApi.list404s()` vs raw `fetch('/api/seo/404-logs')`) — meaning two backend routes serve the same data. Pick one frontend, route both URLs to it.

### P2-4 — `admin/seo/meta` is a stub that always shows the empty state

[`admin/seo/meta/+page.svelte:15-19`](../../../frontend/src/routes/admin/seo/meta/+page.svelte#L15)

```ts
async function loadEntities() {
    // This would load entities (pages, posts, products, etc.)
    // For now, we'll just create a placeholder
    entities = [];
}
```

Hardcoded empty list. The page renders a "no content found" empty state forever. Either implement the loader or hide this nav entry.

### P2-5 — `admin/seo/+page.svelte` `loadSeoData()` is a no-op

[`admin/seo/+page.svelte:277-288`](../../../frontend/src/routes/admin/seo/+page.svelte#L277)

```ts
async function loadSeoData() {
    // This would fetch from connected SEO services
    // For now, show placeholder until APIs are connected
    try {
        // API call would go here
        // const response = await fetch('/api/admin/seo/metrics');
        // seoData = await response.json();
        lastUpdated = new Date();
    }
}
```

`seoData` is never populated. Whatever metrics card relies on `seoData` will permanently show blank.

### P2-6 — `admin/seo/keywords` "Add Keyword" button has no `onclick`

[`admin/seo/keywords/+page.svelte:104-107,158-160`](../../../frontend/src/routes/admin/seo/keywords/+page.svelte#L104)

Both Add Keyword buttons lack handlers. Click does nothing. The page is read-only despite advertising add functionality.

### P2-7 — `admin/seo/store-locator` hardcodes preview-deploy URL

[`admin/seo/store-locator/+page.svelte:91`](../../../frontend/src/routes/admin/seo/store-locator/+page.svelte#L91)

```ts
website: 'https://revolution-trading-pros.pages.dev',
```

Cloudflare Pages preview domain hardcoded in what looks to be sample/seed data. If this leaks into LocalBusiness schema markup it ships the dev URL to Google. Replace with `https://revolutiontradingpros.com` or read from env.

### P2-8 — `admin/forms/entries/+page.svelte:155` JSON-stringifies submission payload to render preview

[`admin/forms/entries/+page.svelte:154-156`](../../../frontend/src/routes/admin/forms/entries/+page.svelte#L154)

```svelte
<td class="max-w-xs truncate text-gray-600">
    {JSON.stringify(entry.data).substring(0, 100)}...
</td>
```

If the form collects PII (`email`, `phone`), this puts it in plain view of anyone with admin access. Acceptable for an admin tool, but two hygiene fixes:
- Redact obvious fields (`password`, `ssn`, `card_number`) before render.
- Truncate by character count is dumb on multi-byte text. Use a CSS line-clamp instead.

Also, every form-entry rendering path in [`admin/forms/entries/+page.svelte`](../../../frontend/src/routes/admin/forms/entries/+page.svelte) lacks a CSV-injection guard for the `exportSubmissions(formId, 'csv')` call at [L72](../../../frontend/src/routes/admin/forms/entries/+page.svelte#L72). If a submission contains `=cmd|...` that's a known Excel/Sheets RCE vector. Server-side sanitization required.

### P2-9 — Templates page list `$effect(() => loadTemplates())` is a misuse

[`admin/email/templates/+page.svelte:49-51`](../../../frontend/src/routes/admin/email/templates/+page.svelte#L49)

`$effect` for a one-shot data load is what `onMount` is for. With no tracked deps the effect runs once on mount, but if any rune accessed inside `loadTemplates` becomes reactive in a refactor, it'll loop. `CLAUDE.md` cites this exact bug class (commit `dad9af2cf` for the dashboard fix; `33112d5a1` for CRM/campaigns).

Same antipattern in:
- [`admin/seo/+page.svelte:258`](../../../frontend/src/routes/admin/seo/+page.svelte#L258)
- [`admin/seo/keywords/+page.svelte:22`](../../../frontend/src/routes/admin/seo/keywords/+page.svelte#L22)
- [`admin/seo/404s/+page.svelte:24`](../../../frontend/src/routes/admin/seo/404s/+page.svelte#L24)
- [`admin/seo/analysis/+page.svelte:86`](../../../frontend/src/routes/admin/seo/analysis/+page.svelte#L86)
- [`admin/popups/+page.svelte:23`](../../../frontend/src/routes/admin/popups/+page.svelte#L23)
- [`admin/email/campaigns/[id]/report/+page.svelte:231`](../../../frontend/src/routes/admin/email/campaigns/[id]/report/+page.svelte#L231)
- [`admin/popups/new/+page.svelte:79`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L79)

The campaigns page got the right fix on 2026-04-26 ([campaigns/+page.svelte:132-134](../../../frontend/src/routes/admin/email/campaigns/+page.svelte#L132)) — replicate.

### P2-10 — Subscribers page Set mutation requires reassignment workaround

[`admin/email/subscribers/+page.svelte:192-199`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L192)

```ts
function toggleSubscriber(id: string) {
    if (selectedSubscribers.has(id)) {
        selectedSubscribers.delete(id);
    } else {
        selectedSubscribers.add(id);
    }
    selectedSubscribers = selectedSubscribers;  // self-assign to trigger reactivity
}
```

Svelte 5 makes `Set`/`Map` reactive natively when wrapped via `new SvelteSet()`. The self-assign trick works but is a code smell. Switch to `SvelteSet` from `svelte/reactivity` and drop the reassign.

### P2-11 — SEO settings `alert('Settings saved successfully!')` and `alert('Failed to save settings')`

[`admin/seo/settings/+page.svelte:73,76`](../../../frontend/src/routes/admin/seo/settings/+page.svelte#L73) — use `toastStore` like the rest of admin.

### P2-12 — Schema page uses `alert('JSON-LD copied to clipboard!')` after `clipboard.writeText`

[`admin/seo/schema/+page.svelte:66-69`](../../../frontend/src/routes/admin/seo/schema/+page.svelte#L66)

`navigator.clipboard.writeText` returns a Promise — not awaited. If clipboard permission is denied the alert fires anyway claiming success.

### P2-13 — Schema page navigates to `/admin/seo/schema/create` and `/admin/seo/schema/[id]/edit` which don't exist in the routes tree

[`admin/seo/schema/+page.svelte:82,133,146`](../../../frontend/src/routes/admin/seo/schema/+page.svelte#L82). The audit's directory listing under `/admin/seo/schema/` shows only `+page.svelte` — no `create/`, no `[id]/edit/`. Clicking "Add Schema" or "Edit Schema" routes to a 404. (Per `feedback_create_not_delete.md`: build the missing side, don't disable.)

### P2-14 — Email-templates list missing pagination

[`admin/email/templates/+page.svelte`](../../../frontend/src/routes/admin/email/templates/+page.svelte) has search filtering on the client. With more than ~100 templates the page is slow and `templates = response.data` loads everything. Compare to [`admin/email/subscribers/+page.svelte:478-498`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L478) which paginates correctly.

---

## Low / nits (P3)

- [`admin/popups/new/+page.svelte:1`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L1) and [`admin/popups/[id]/edit/+page.svelte:1`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte#L1): both files start with a Svelte 4→5 migration error comment. The migration was incomplete — TODO marker still present. Either finish the migration or remove the stale comment.
- [`admin/popups/[id]/edit/+page.svelte:529-554`](../../../frontend/src/routes/admin/popups/[id]/edit/+page.svelte#L529): three `<input id="page-checkbox" name="page-checkbox">` with **identical** ids. Duplicate IDs break label-for and assistive tech. Same in [`admin/email/subscribers/+page.svelte:379-405`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L379) (header + each row). Use `id="subscriber-{subscriber.id}-select"`.
- [`admin/email/campaigns/+page.svelte:201-219`](../../../frontend/src/routes/admin/email/campaigns/+page.svelte#L201): segment fallback hardcodes "12,847 members" when API fails. Misleading vanity numbers. Surface the error.
- [`admin/email/campaigns/[id]/report/+page.svelte:241-326`](../../../frontend/src/routes/admin/email/campaigns/[id]/report/+page.svelte#L241): `loadReport()` falls back to `generateDemoReport()` on **any** error including 401/404. Shows fake data to admins who lack permission. Should propagate the error.
- [`admin/email/templates/edit/[id]/+page.svelte:9`](../../../frontend/src/routes/admin/email/templates/edit/[id]/+page.svelte#L9): `template: Record<string, unknown> | null = $state(null)` then passed to `<TemplateForm {template} isEdit={true} />` — `TemplateForm` doesn't validate the shape. If Rust returns wrapped `{data: {…}}` vs `{…}`, `isEdit` triggers `template.id` access which is `undefined`.
- [`admin/email/templates/preview/[id]/+page.svelte:13-19`](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte#L13): preview hits `POST /admin/email/templates/{id}/preview` with `body: '{}'`. Nothing inserts sample variables. Templates with required `{{name}}` placeholders render literally as `{{name}}`, defeating the point of preview.
- [`admin/email/subscribers/+page.svelte:202-207`](../../../frontend/src/routes/admin/email/subscribers/+page.svelte#L202): `toggleAllSubscribers` "select all" is bound to **current page** only. Misleading checkbox semantics on paginated lists.
- [`admin/popups/+page.svelte:23-38`](../../../frontend/src/routes/admin/popups/+page.svelte#L23): GSAP imported and animation runs on every `$effect` re-trigger, including after `loadPopups` toggles. Stagger replays unexpectedly.
- [`admin/seo/keywords/+page.svelte:144`](../../../frontend/src/routes/admin/seo/keywords/+page.svelte#L144): `<button class="btn-secondary" onclick={loadKeywords}>` — `loadStats()` not called on refresh, stats go stale.
- [`admin/seo/sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/sitemap/+page.svelte), [`admin/seo/news-sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/news-sitemap/+page.svelte), [`admin/seo/video-sitemap/+page.svelte`](../../../frontend/src/routes/admin/seo/video-sitemap/+page.svelte): not deeply audited (large files), but sitemap content escaping is critical and should be re-reviewed.
- [`admin/email/templates/+page.svelte:101`](../../../frontend/src/routes/admin/email/templates/+page.svelte#L101): no role check on delete button. UI assumes the bearer cookie also implies admin role; defense in depth requires role check both client (hide button) and server (reject).
- Forms package routes [`admin/forms/[id]/edit`](../../../frontend/src/routes/admin/forms/[id]/edit), [`submissions`](../../../frontend/src/routes/admin/forms/[id]/submissions), [`analytics`](../../../frontend/src/routes/admin/forms/[id]/analytics) all duplicate the "back-btn → h1 → header-actions" header. Extract a shared `<FormPageHeader>` component.
- [`admin/forms/[id]/submissions/+page.svelte:17`](../../../frontend/src/routes/admin/forms/[id]/submissions/+page.svelte#L17) and similar: `parseInt(page.params['id']!)` with non-null assertion. If the URL is `/admin/forms/foo/submissions`, `parseInt('foo')` is `NaN` and nothing renders. Validate.
- [`admin/popups/new/+page.svelte:1155,1165`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L1155): `bind:value={(formData as any).start_date}` casts away the type. `Popup` type is missing the start/end-date fields. Either add them or accept the lint debt explicitly.

---

## Cross-cutting concerns

1. **Auth header sourcing.** The recent commit `e2356fa46` migrated proxies to read `rtp_access_token` cookie. The settings proxy at [`api/admin/email/settings/+server.ts`](../../../frontend/src/routes/api/admin/email/settings/+server.ts) was missed entirely (P0-1). Audit every `+server.ts` under `routes/api/admin/**` for the same pattern: `cookies.get('rtp_access_token') || authHeader.replace(/^Bearer\s+/i, '')`.

2. **Mixing direct `fetch('/api/seo/...')` with `apiFetch`/`adminFetch`.** SEO pages use raw `fetch` ([keywords:32](../../../frontend/src/routes/admin/seo/keywords/+page.svelte#L32), [redirects:32](../../../frontend/src/routes/admin/seo/redirects/+page.svelte#L32), [schema:35](../../../frontend/src/routes/admin/seo/schema/+page.svelte#L35), [settings:45](../../../frontend/src/routes/admin/seo/settings/+page.svelte#L45), [404-monitor:40](../../../frontend/src/routes/admin/seo/404-monitor/+page.svelte#L40)). Email pages use `apiFetch`/`adminFetch`. Form pages use `getForms`/etc clients. Three idioms for the same job. Raw `fetch` skips error normalization, retry, correlation IDs, the circuit breaker — every benefit of the `[...path]` proxy is forfeited inside the page.

3. **Modal-rendering discipline.** P0-4 (`admin/popups/+page.svelte` dead delete modal) and the sibling `_showAbTestModal` issue indicate a pattern where state was added without the corresponding component wiring. A linter rule (or a code-review checklist item: `if you grep "showXyzModal", you must grep "<.* isOpen={showXyzModal}"`) would catch all of these.

4. **No CSRF token on any of the proxies.** [`api/admin/email/settings/+server.ts:36`](../../../frontend/src/routes/api/admin/email/settings/+server.ts#L36) `POST` uses cookie-based auth without a CSRF token. Cookie-only auth + cross-site POST = classic CSRF setup. The catch-all proxy [`api/[...path]/+server.ts`](../../../frontend/src/routes/api/[...path]/+server.ts) has the same gap. SameSite=Lax mitigates but not for embedded admin actions in iframes/popups.

5. **HTML sinks need a single sanitizer.** [`templates/preview/[id]/+page.svelte:39`](../../../frontend/src/routes/admin/email/templates/preview/[id]/+page.svelte#L39) sanitizes via `sanitizeHtml(...,'rich')`. [`popups/new/+page.svelte:582`](../../../frontend/src/routes/admin/popups/new/+page.svelte#L582) doesn't. Both sinks render admin-supplied HTML in the admin's authenticated session. Standardize on `sanitizeHtml` everywhere `{@html}` is used; lint to ban raw `{@html foo}`.

6. **Form/popup builders have feature drift between `new` and `[id]/edit`.** P1-7 catalogues this for popups; the pattern likely applies to forms as well. Extract a shared form-config component used by both screens.

7. **`/admin/seo/*` is a sea of stubs.** Meta, keywords (no add), schema (links to nonexistent routes), 404s vs 404-monitor duplication, the SEO command center itself loading nothing. The whole subdirectory needs a "what is real, what is roadmap" pass before another P0 hides in the noise.

---

## Summary

| Severity | Count | Worst |
|---------|-------|-------|
| P0      | 6     | In-memory plaintext SMTP creds with no auth check ([`api/admin/email/settings/+server.ts`](../../../frontend/src/routes/api/admin/email/settings/+server.ts)) |
| P1      | 8     | Shadow-state `$effect` shadowing in `TemplateForm` violates Svelte 5 rule from `CLAUDE.md` |
| P2      | 14    | `_showDeleteModal` style dead-modal state, duplicate routes, stub SEO pages, `JSON.stringify` PII in form-entries preview |
| P3      | ~14   | Stale migration comments, duplicate input IDs, raw `confirm()`, GSAP replays |

Top three to fix this week:
1. **P0-1 / P0-2** — delete `api/admin/email/settings/+server.ts`, route `/admin/email/settings` through `[...path]` to a real Rust endpoint with encrypted credential storage and role auth.
2. **P0-4** — wire up `<ConfirmationModal>` for popup delete (and rename `_showAbTestModal` → `showAbTestModal` with an actual modal). Both are immediate UX bugs reachable from the public admin UI.
3. **P0-5 / P0-6** — sanitize `{@html formData.content}` in popups; verify or add `/api/admin/forms` proxy used by `loadForms()` in `admin/popups/new`.
