# 07-marketing — Deferred Items

Date: 2026-04-26  
Author: principal engineer (07-marketing implementation pass)

These items were explicitly deferred from the implementation pass per the task
instructions. Each entry notes the recommendation and the reasons for deferral.

---

## D1 — Duplicate route: `/admin/email/settings` vs `/admin/email/smtp`

**Files:**
- `frontend/src/routes/admin/email/settings/+page.svelte`
- `frontend/src/routes/admin/email/smtp/+page.svelte`

**Situation:** Both pages render functionally identical UIs hitting the same
`/api/admin/email/settings` endpoint. They differ only in their navigation
entry point. The audit (P2-2) confirmed they are byte-for-byte the same UI.

**Recommendation:** Keep `/admin/email/settings` as the canonical URL.
Redirect `/admin/email/smtp` → `/admin/email/settings` via a SvelteKit
`+page.ts` redirect (`throw redirect(301, '/admin/email/settings')`).
Do NOT delete the smtp page file per `feedback_create_not_delete.md` until
the redirect is confirmed live in production.

**Blocker:** Requires confirming which URL is linked from the sidebar nav and
any external docs before redirecting, to avoid breaking bookmarks.

---

## D2 — Duplicate route: `/admin/seo/404s` vs `/admin/seo/404-monitor`

**Files:**
- `frontend/src/routes/admin/seo/404s/+page.svelte`
- `frontend/src/routes/admin/seo/404-monitor/+page.svelte`

**Situation:** Both pages display 404 error logs with bulk-delete. They use
*different* API clients: `seoApi.list404s()` vs raw `fetch('/api/seo/404-logs')`.
This means two backend routes serve the same data with divergent error handling.

**Recommendation:** Keep `/admin/seo/404-monitor` (it uses the newer raw-fetch
pattern consistent with other SEO pages, and has `loadStats()` + `sortBy`
filtering). Update `/admin/seo/404s` to redirect to `/admin/seo/404-monitor`.
Before redirecting, verify that `seoApi.list404s()` and `/api/seo/404-logs`
resolve to the same backend endpoint in Rust.

**Blocker:** Backend route audit required before consolidating the API client.

---

## D3 — SMTP credential storage redesign

**File:** `frontend/src/routes/api/admin/email/settings/+server.ts`

**Situation:** The current proxy forwards SMTP credentials to the Rust backend.
The Rust backend stores them (encrypted or plaintext — unknown without reading
`api/src/routes/admin.rs`). The correct long-term storage is:

1. Credentials encrypted at rest using Fly secrets / AWS KMS / libsodium.
2. Backend returns `has_password: boolean` only on GET — never the ciphertext.
3. Frontend treats an empty password field as "keep stored value" and sends no
   `password` key on save (the proxy already strips the `••••••••` sentinel as
   of the 2026-04-26 fix, but the client-side form still binds `password` to
   the field value).

**Blocker:** Requires Rust backend work (schema migration + KMS integration)
and a coordinated frontend form change. Tracked separately.

---

## D4 — SEO stub pages that need full implementation

The following SEO admin pages are placeholders with no real data loading.
They are deferred because implementation requires backend endpoints that do
not yet exist:

| Page | Issue |
|------|-------|
| `admin/seo/meta/+page.svelte` | `loadEntities()` always returns `[]` (P2-4) |
| `admin/seo/+page.svelte` | `loadSeoData()` is a no-op comment (P2-5) |
| `admin/seo/keywords/+page.svelte` | "Add Keyword" buttons have no `onclick` (P2-6) |
| `admin/seo/schema/+page.svelte` | "Add Schema" links to `schema/create` route that doesn't exist (P2-13) |
| `admin/seo/store-locator/+page.svelte` | Hardcodes Cloudflare Pages preview domain (P2-7) |

**Recommendation:** Hide unimplemented features behind a `FEATURE_FLAG_SEO`
env var until backend APIs are ready, rather than shipping no-op UIs that
mislead admins.

---

## D5 — Popup edit page feature parity with new page (P1-7)

`admin/popups/[id]/edit/+page.svelte` is missing the following fields that
exist in `admin/popups/new/+page.svelte`:

- Include / exclude page targeting
- Secondary CTA
- Video embed
- Form integration
- Overlay settings

**Impact:** Saving a popup via the edit page silently drops these fields.

**Recommendation:** Extract a shared `<PopupFormFields>` component used by
both `/new` and `/[id]/edit`. This is a medium-effort refactor; deferred
because the field list may grow and a shared component is the right foundation.

---

## D6 — `_showAbTestModal` in popup edit page (mentioned in audit P0-4)

`admin/popups/[id]/edit/+page.svelte:46` sets `_showAbTestModal = true` at
line 783 but no `<AbTestModal>` is rendered. Clicking "Create A/B Test" does
nothing.

**Recommendation:** Either implement the A/B test modal (requires backend
support) or remove the button. Per `feedback_create_not_delete.md`, build the
missing side rather than disabling the button.

**Blocker:** A/B test backend endpoint needs to be confirmed or built first.
