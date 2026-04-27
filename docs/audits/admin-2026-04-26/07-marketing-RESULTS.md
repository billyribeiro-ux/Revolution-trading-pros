# 07-marketing — Implementation Results

Date: 2026-04-26  
Author: principal engineer (implementation pass)

---

## Status: COMPLETE (within scope)

`pnpm check` exits with **6 pre-existing errors** in files outside this
cluster (`crm/sequences/+page.svelte` and `trading-rooms/[slug]/+page.svelte`).
**Zero errors introduced by this pass.** Zero warnings on marketing files.

---

## Items implemented

### P0-1 — Email settings endpoint: no auth check (SMTP creds exposed)

**File:** `frontend/src/routes/api/admin/email/settings/+server.ts`

Already fixed prior to this pass (the file had been rewritten with full
`rtp_access_token` cookie auth, Bearer header fallback, forwarding to Rust
backend, and defense-in-depth password scrubbing on GET responses).
Confirmed correct — no further action needed.

### P0-2 — `{@html formData.content}` XSS sink in popup preview

**File:** `frontend/src/routes/admin/popups/new/+page.svelte`

Already fixed prior to this pass. The preview block uses
`{@html sanitizeHtml(formData.content, 'rich')}` via the existing
`$lib/utils/sanitize.ts` utility (DOMPurify-backed). Confirmed correct.

### P0-3 — `loadForms()` calls non-existent `/api/admin/forms` proxy

**File:** `frontend/src/routes/api/admin/forms/+server.ts`

Already created prior to this pass. The proxy handles GET (with query-param
forwarding) and POST, reads `rtp_access_token` cookie, forwards to Rust
`/api/forms`, and returns `{ forms: [], total: 0 }` on error so the popup
builder degrades gracefully. Confirmed correct.

### P0-4 — Popup list delete button wired to dead `_showDeleteModal`

**File:** `frontend/src/routes/admin/popups/+page.svelte`

Already fixed prior to this pass. `<ConfirmationModal>` is rendered at the
bottom of the template, `showDeleteModal` (underscore dropped) controls it,
and `confirmDeletePopup` / `cancelDelete` are wired to `onConfirm` /
`onCancel`. Confirmed correct.

### P1 — `TemplateForm.svelte` shadow-state `$effect` pattern

**File:** `frontend/src/lib/components/admin/TemplateForm.svelte`

**Fixed this pass.**

- Replaced 8-field shadow-state `$effect` block with `untrack()`-seeded
  `$state` initialisers. Each field reads the prop value exactly once at
  component creation time without creating a reactive dependency.
- Added `import { untrack } from 'svelte'` at the top of the script block.
- Svelte autofixer returned `{"issues":[],"suggestions":[]}` — clean.

### P1 — `JSON.parse(variables)` crash on save

**File:** `frontend/src/lib/components/admin/TemplateForm.svelte`

**Fixed this pass.**

- Wrapped `JSON.parse(variables)` in try/catch inside `handleSubmit`.
- Added pre-parse array-type check: if the JSON is valid but not an array,
  shows `'Variables must be a JSON array, e.g. ["name", "date"]'`.
- On SyntaxError, shows `'Variables must be valid JSON: <parser message>'`.
- Sets `loading = false` and returns early — spinner never gets stuck.
- Type is cast to `string[]` after validation to satisfy `EmailTemplate`.

### P2 — CSV injection in form-entries export (`exportToCSV`)

**File:** `frontend/src/lib/utils/export.ts`

**Fixed this pass.**

Added a formula-injection prefix guard before the double-quote wrapper in
`exportToCSV`. Any cell value whose first character is `=`, `+`, `-`, `@`,
tab (`\t`), or carriage-return (`\r`) is prefixed with a single quote `'`,
causing Excel / Google Sheets to treat it as plain text rather than a
formula.

```ts
const safe = /^[=+\-@\t\r]/.test(formatted) ? `'${formatted}` : formatted;
return `"${safe.replace(/"/g, '""')}"`;
```

Note: the `exportSubmissions` path in `forms.ts` is server-generated (Rust
returns a raw blob). That path requires a Rust-side fix and is tracked in
the deferred doc as it's outside the frontend scope of this pass.

### P2-9 — `$effect(() => loadForms())` in `popups/new` and `$effect` init in `popups/+page`

**Files:**
- `frontend/src/routes/admin/popups/new/+page.svelte`
- `frontend/src/routes/admin/popups/+page.svelte`

**Fixed this pass.**

Both files replaced the one-shot `$effect` (which calls state-assigning
functions) with `onMount`. The popups list page also drops the `browser`
guard that was only needed because `$effect` runs on SSR — `onMount` is
browser-only by definition. Svelte autofixer confirmed both are clean.

---

## Deferred

See `docs/audits/admin-2026-04-26/07-marketing-DEFERRED.md` for:

- D1: `/admin/email/settings` vs `/admin/email/smtp` duplicate route decision
- D2: `/admin/seo/404s` vs `/admin/seo/404-monitor` duplicate route decision
- D3: SMTP credential storage redesign (Rust backend + KMS)
- D4: SEO stub pages (meta, keywords, schema, store-locator) — needs backend APIs
- D5: Popup edit page feature parity with new page (P1-7)
- D6: `_showAbTestModal` in popup edit page — needs A/B test backend

---

## Files modified this pass

| File | Change |
|------|--------|
| `frontend/src/lib/components/admin/TemplateForm.svelte` | Shadow-state → untrack; JSON.parse try/catch |
| `frontend/src/lib/utils/export.ts` | CSV injection prefix guard in `exportToCSV` |
| `frontend/src/routes/admin/popups/+page.svelte` | `$effect` + browser guard → `onMount` |
| `frontend/src/routes/admin/popups/new/+page.svelte` | `$effect` → `onMount` for `loadForms` |
| `docs/audits/admin-2026-04-26/07-marketing-DEFERRED.md` | Created (deferred items) |
| `docs/audits/admin-2026-04-26/07-marketing-RESULTS.md` | Created (this file) |

## Files confirmed correct (fixed prior to this pass)

| File | Fix confirmed |
|------|--------------|
| `frontend/src/routes/api/admin/email/settings/+server.ts` | Auth, password scrub, backend proxy |
| `frontend/src/routes/api/admin/forms/+server.ts` | GET/POST proxy, auth, graceful empty |
| `frontend/src/routes/admin/popups/+page.svelte` | ConfirmationModal wired for delete |
| `frontend/src/routes/admin/popups/new/+page.svelte` | `{@html sanitizeHtml(...)}` XSS fix |
