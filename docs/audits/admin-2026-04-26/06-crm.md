# 06 — CRM Cluster Audit

**Auditor:** Claude (Opus 4.7, principal-engineer mode)
**Scope:** `frontend/src/routes/admin/crm/**`, `frontend/src/routes/admin/contacts/**`, `frontend/src/routes/api/admin/crm/**`
**Date:** 2026-04-26
**Recent context:** commit `34a0bd070` ("replace $effect with onMount to resolve write-while-reading cascade issues in CRM and campaigns pages")
**Volume:** 30 Svelte pages (~33k LoC) + 4 `+server.ts` proxy files. The largest cluster in the admin surface.

---

## Files reviewed

### Top-level CRM (1 file)
- [`admin/crm/+page.svelte`](../../../frontend/src/routes/admin/crm/+page.svelte) — dashboard

### Contact cluster (4 files)
- [`admin/contacts/+page.svelte`](../../../frontend/src/routes/admin/contacts/+page.svelte)
- [`admin/contacts/new/+page.svelte`](../../../frontend/src/routes/admin/contacts/new/+page.svelte)
- [`admin/crm/contacts/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/contacts/[id]/+page.svelte)

### Lead cluster (2 files)
- [`admin/crm/leads/+page.svelte`](../../../frontend/src/routes/admin/crm/leads/+page.svelte)
- [`admin/crm/leads/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/leads/[id]/+page.svelte)

### Deal cluster (2 files)
- [`admin/crm/deals/+page.svelte`](../../../frontend/src/routes/admin/crm/deals/+page.svelte)
- [`admin/crm/deals/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte)

### Tag cluster (2 files)
- [`admin/crm/tags/+page.svelte`](../../../frontend/src/routes/admin/crm/tags/+page.svelte)
- [`admin/crm/tags/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/tags/[id]/+page.svelte)

### Webhook cluster (3 files)
- [`admin/crm/webhooks/+page.svelte`](../../../frontend/src/routes/admin/crm/webhooks/+page.svelte)
- [`admin/crm/webhooks/new/+page.svelte`](../../../frontend/src/routes/admin/crm/webhooks/new/+page.svelte)
- [`admin/crm/webhooks/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/crm/webhooks/[id]/edit/+page.svelte)

### Automation cluster (4 files)
- [`admin/crm/automations/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/+page.svelte)
- [`admin/crm/automations/new/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/new/+page.svelte)
- [`admin/crm/automations/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/[id]/+page.svelte)
- [`admin/crm/automations/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/[id]/edit/+page.svelte)

### Other listing pages (12 files)
- [`admin/crm/lists/+page.svelte`](../../../frontend/src/routes/admin/crm/lists/+page.svelte)
- [`admin/crm/segments/+page.svelte`](../../../frontend/src/routes/admin/crm/segments/+page.svelte)
- [`admin/crm/companies/+page.svelte`](../../../frontend/src/routes/admin/crm/companies/+page.svelte)
- [`admin/crm/campaigns/+page.svelte`](../../../frontend/src/routes/admin/crm/campaigns/+page.svelte)
- [`admin/crm/recurring-campaigns/+page.svelte`](../../../frontend/src/routes/admin/crm/recurring-campaigns/+page.svelte)
- [`admin/crm/sequences/+page.svelte`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte)
- [`admin/crm/templates/+page.svelte`](../../../frontend/src/routes/admin/crm/templates/+page.svelte)
- [`admin/crm/smart-links/+page.svelte`](../../../frontend/src/routes/admin/crm/smart-links/+page.svelte)
- [`admin/crm/abandoned-carts/+page.svelte`](../../../frontend/src/routes/admin/crm/abandoned-carts/+page.svelte)
- [`admin/crm/managers/+page.svelte`](../../../frontend/src/routes/admin/crm/managers/+page.svelte)
- [`admin/crm/import-export/+page.svelte`](../../../frontend/src/routes/admin/crm/import-export/+page.svelte)
- [`admin/crm/logs/+page.svelte`](../../../frontend/src/routes/admin/crm/logs/+page.svelte)
- [`admin/crm/settings/+page.svelte`](../../../frontend/src/routes/admin/crm/settings/+page.svelte)

### API proxies (4 files)
- [`api/admin/crm/contacts/+server.ts`](../../../frontend/src/routes/api/admin/crm/contacts/+server.ts)
- [`api/admin/crm/contacts/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/crm/contacts/[...rest]/+server.ts)
- [`api/admin/crm/deals/+server.ts`](../../../frontend/src/routes/api/admin/crm/deals/+server.ts)
- [`api/admin/crm/stats/+server.ts`](../../../frontend/src/routes/api/admin/crm/stats/+server.ts)

---

## Critical bugs (P0)

### 1. Mass orphan-link epidemic — entire creation workflow is broken

The CRM cluster is full of `New X` buttons and `Edit X` links pointing to routes that **do not exist**. Every one of these `<a href>` produces a 404. List of confirmed missing routes:

| Referenced from | Target href | Frontend route exists? |
|---|---|---|
| [`templates/+page.svelte:394`](../../../frontend/src/routes/admin/crm/templates/+page.svelte) | `/admin/crm/templates/prebuilt` | **NO** |
| [`templates/+page.svelte:398`](../../../frontend/src/routes/admin/crm/templates/+page.svelte) | `/admin/crm/templates/new` | **NO** |
| [`companies/+page.svelte:185`](../../../frontend/src/routes/admin/crm/companies/+page.svelte) | `/admin/crm/companies/new` | **NO** |
| [`sequences/+page.svelte:360`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte) | `/admin/crm/sequences/new` | **NO** |
| [`lists/+page.svelte:127`](../../../frontend/src/routes/admin/crm/lists/+page.svelte) | `/admin/crm/lists/new` | **NO** |
| [`segments/+page.svelte:~`](../../../frontend/src/routes/admin/crm/segments/+page.svelte) | `/admin/crm/segments/new` | **NO** |
| [`campaigns/+page.svelte:248`](../../../frontend/src/routes/admin/crm/campaigns/+page.svelte) | `/admin/crm/campaigns/new` | **NO** |
| [`recurring-campaigns/+page.svelte:168`](../../../frontend/src/routes/admin/crm/recurring-campaigns/+page.svelte) | `/admin/crm/recurring-campaigns/new` | **NO** |
| [`deals/+page.svelte:389`](../../../frontend/src/routes/admin/crm/deals/+page.svelte) | `/admin/crm/deals/new` | **NO** |
| [`abandoned-carts/+page.svelte:193`](../../../frontend/src/routes/admin/crm/abandoned-carts/+page.svelte) | `/admin/crm/abandoned-carts/settings` | **NO** |
| [`managers/+page.svelte:129,134`](../../../frontend/src/routes/admin/crm/managers/+page.svelte) | `/admin/crm/managers/add`, `/admin/crm/managers/roles/new` | **NO** |
| [`import-export/+page.svelte:226`](../../../frontend/src/routes/admin/crm/import-export/+page.svelte) | `/admin/crm/import-export/import/{type}` | **NO** |
| [`leads/[id]/+page.svelte:418`](../../../frontend/src/routes/admin/crm/leads/[id]/+page.svelte) | `/admin/crm/leads/{leadId}/edit` | **NO** |
| [`contacts/[id]/+page.svelte:596`](../../../frontend/src/routes/admin/crm/contacts/[id]/+page.svelte) | `/admin/crm/contacts/{contactId}/edit` | **NO** |
| [`deals/[id]/+page.svelte:409`](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte) | `/admin/crm/deals/{dealId}/edit` | **NO** |
| Multiple list pages | `/admin/crm/lists/[id]` and `/admin/crm/lists/[id]/edit` | **NO** |

A user cannot create a single new resource (template, company, sequence, list, segment, campaign, recurring campaign, deal, etc.) from any of these listing pages, nor edit any contact / lead / deal from its detail page. The whole CRUD-create-flow is dead.

Per `feedback_create_not_delete.md`: the fix is to **build the missing pages**, not to delete the buttons.

### 2. `Math.random()` for cryptographic webhook secrets

[`webhooks/new/+page.svelte:160-167`](../../../frontend/src/routes/admin/crm/webhooks/new/+page.svelte) and [`webhooks/[id]/edit/+page.svelte:199-208`](../../../frontend/src/routes/admin/crm/webhooks/[id]/edit/+page.svelte):

```ts
function generateSecret() {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    for (let i = 0; i < 32; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    secret = result;
}
```

`Math.random()` is **not cryptographically secure**. Webhook secrets are used to HMAC-sign outbound payloads; an attacker who can predict the seed can forge requests claiming to be from this CRM. Use `crypto.getRandomValues()`:

```ts
const bytes = new Uint8Array(24);
crypto.getRandomValues(bytes);
secret = btoa(String.fromCharCode(...bytes)).replace(/[+/=]/g, '').slice(0, 32);
```

### 3. Leads bulk-delete has no confirmation

[`leads/+page.svelte:701`](../../../frontend/src/routes/admin/crm/leads/+page.svelte):

```svelte
<button class="btn-bulk danger" onclick={bulkDelete}> Delete </button>
```

`bulkDelete` ([line 423](../../../frontend/src/routes/admin/crm/leads/+page.svelte)) immediately POSTs `/api/admin/crm/leads/bulk-delete` with the entire `selectedLeads` set. No `ConfirmationModal`, no `confirm()`. One mis-click after a "Select all" and the user wipes out an arbitrary number of leads from production. Every other page in the cluster wraps deletes in `<ConfirmationModal>`; this is the outlier.

`bulkUpdateStatus` ([line 438](../../../frontend/src/routes/admin/crm/leads/+page.svelte)) has the same problem (less destructive but still unconfirmed).

---

## High-severity issues (P1)

### 4. `sendTestEmail` in sequences is a fake — it never sends anything

[`sequences/+page.svelte:167-195`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte):

```ts
async function sendTestEmail() {
    if (!selectedSequence || !sendEmailForm.testEmail) return;
    sendEmailForm.isLoading = true;
    // ...
    try {
        const emails = await crmAPI.getSequenceEmails(selectedSequence.id);
        if (emails.length === 0) { /* ... */ }

        // Subscribe a test contact (this would typically be a dedicated test endpoint)
        // For now, we'll simulate success - in production, you'd have a proper test email endpoint
        await new Promise((resolve) => setTimeout(resolve, 1000));

        sendEmailForm.success = true;
        // ...
```

The user sees "Test email sent successfully" but **nothing is actually delivered**. This is a stub left in production. Either wire up the real backend endpoint or hide the button.

### 5. Sequences page: filter changes don't reload data

[`sequences/+page.svelte:296-307`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte):

```ts
$effect(() => {
    // Track the reactive dependencies   <-- COMMENT ONLY; nothing is actually read
    if (isInitialized) {
        untrack(() => {
            loadSequences();
        });
    }
});
```

The effect declares no reactive reads, so it fires once when `isInitialized` flips, then never again. Compare to the correct pattern in [`automations/+page.svelte:459-472`](../../../frontend/src/routes/admin/crm/automations/+page.svelte), which explicitly reads `searchQuery; selectedStatus; selectedTrigger;` before the `if (isInitialized)` gate.

Result: changing the search box or the "All Sequences / Active / Paused" dropdown sends nothing to the backend; the user only ever sees the page-1 mount fetch (filtered client-side via `filteredSequences`, but the search text is also a backend filter that won't run). Search is broken for any list with > 1 page of data.

### 6. Deals listing: status filter does not re-fetch

[`deals/+page.svelte:135-182`](../../../frontend/src/routes/admin/crm/deals/+page.svelte). `loadData()` reads `selectedStatus` and passes it to the backend, but it is only called from `onMount` and explicit refresh / win / lose / delete actions. There is no effect tracking `selectedStatus` to call `loadData` when the user changes the dropdown.

The kanban itself recovers via the client-side `filteredDeals = $derived(...)` filter on line 103, **but** if the user switches `selectedStatus` from `open` → `won` and there are won deals NOT in the open-only initial fetch, they will be invisible. Need an effect that calls `loadData()` on `selectedStatus` change.

Same pattern repeats in:
- [`abandoned-carts/+page.svelte`](../../../frontend/src/routes/admin/crm/abandoned-carts/+page.svelte) — `loadCarts()` only on mount + explicit refresh
- [`lists/+page.svelte:106`](../../../frontend/src/routes/admin/crm/lists/+page.svelte) — `loadLists()` only on mount; `is_public` filter passed but not re-fired
- [`tags/+page.svelte:189`](../../../frontend/src/routes/admin/crm/tags/+page.svelte)
- [`recurring-campaigns/+page.svelte:148`](../../../frontend/src/routes/admin/crm/recurring-campaigns/+page.svelte)
- [`companies/+page.svelte:158`](../../../frontend/src/routes/admin/crm/companies/+page.svelte)
- [`segments/+page.svelte`](../../../frontend/src/routes/admin/crm/segments/+page.svelte)
- [`campaigns/+page.svelte:225`](../../../frontend/src/routes/admin/crm/campaigns/+page.svelte) — same shape

In each case the filter parameter goes to the backend on initial fetch but never re-fires. Only the client-side `$derived` filter survives, which is fine for small datasets but breaks once paginated.

### 7. `admin/contacts` page: emptying the status filter doesn't reload

[`admin/contacts/+page.svelte:76-80`](../../../frontend/src/routes/admin/contacts/+page.svelte):

```ts
$effect(() => {
    if (statusFilter !== '') {
        loadContacts();
    }
});
```

The condition `statusFilter !== ''` means switching FROM a value back to "All Statuses" (`''`) will **not** reload — the user is stuck viewing the previous filter's results until they choose another non-empty status. Also a shadow-state-style effect — should be wired explicitly with an `onChange` on the `<Select>` component, like the search debouncer does.

### 8. Stored XSS via email-template preview

[`templates/+page.svelte:790`](../../../frontend/src/routes/admin/crm/templates/+page.svelte):

```svelte
{@html previewTemplate.content || '<p class="no-content">No content available</p>'}
```

`previewTemplate.content` is whatever HTML another admin saved. Admin-to-admin XSS: any admin who can edit templates can inject script that runs in another admin's browser session (steal `rtp_access_token` cookie, etc.). For email-template HTML you typically need to *render* it — the safest path is to render inside an `<iframe sandbox="allow-same-origin">` (no `allow-scripts`), or sanitize via DOMPurify before `@html`.

### 9. SSRF risk — webhook URL validator accepts http and private IPs

[`webhooks/new/+page.svelte:151-158`](../../../frontend/src/routes/admin/crm/webhooks/new/+page.svelte) and [`webhooks/[id]/edit/+page.svelte:190-197`](../../../frontend/src/routes/admin/crm/webhooks/[id]/edit/+page.svelte):

```ts
function isValidUrl(urlString: string): boolean {
    try {
        const url = new URL(urlString);
        return url.protocol === 'https:' || url.protocol === 'http:';
    } catch { return false; }
}
```

The error message displayed to the user says `"Please enter a valid URL (must start with https://)"` (line 144 / 105) but the validator accepts `http://` and there is no allow/deny-list for hostnames. An admin (or compromised admin token) can register `http://169.254.169.254/latest/meta-data/` (AWS IMDS), `http://localhost:6379` (internal Redis), `http://10.x.y.z/...` (internal services), and the Rust backend's webhook dispatcher will dutifully fire requests there with whatever event payloads. Backend-side mitigation is mandatory but the frontend message lying to the user is a separate UX bug.

Tighten frontend to `url.protocol === 'https:'` only, and reject hostnames that resolve to RFC1918 / loopback / link-local at the backend.

---

## Medium issues (P2)

### 10. Reactivity: 7 `$effect` blocks should be `onMount` (commit 34a0bd070 incomplete)

The recent commit migrated several `$effect(() => loadData())` patterns to `onMount`. The same write-while-reading-tracked-dep risk exists in:

| File | Line | Pattern |
|---|---|---|
| [`leads/+page.svelte`](../../../frontend/src/routes/admin/crm/leads/+page.svelte) | 559-573 | identical pattern to the one fixed in `crm/+page.svelte:290` — calls `connections.load()` (writes) and reads `getIsCrmConnected()` inside the same `$effect` |
| [`managers/+page.svelte`](../../../frontend/src/routes/admin/crm/managers/+page.svelte) | 108-110 | bare `$effect(() => loadData())` — should be `onMount` |
| [`deals/[id]/+page.svelte`](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte) | 323-326 | bare `$effect(() => loadDeal())` |
| [`automations/new/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/new/+page.svelte) | 325-327 | bare `$effect(() => loadDependencies())` |
| [`automations/[id]/edit/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/[id]/edit/+page.svelte) | 400-402 | bare `$effect(() => loadFunnel())` |
| [`automations/+page.svelte`](../../../frontend/src/routes/admin/crm/automations/+page.svelte) | 489-497 | the lifecycle effect — filter-tracker effect at line 459 is fine and should stay `$effect` |
| [`sequences/+page.svelte`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte) | 325-333 | the lifecycle effect — see issue #5 for the broken filter effect |

These all run once and never again, so functionally they work, but they generate the `state_referenced_locally` (or related) warnings that the migration was intended to clear, and they remain an unintended re-run hazard if any tracked state ever leaks into them.

### 11. Templates: cascading effect re-fetches twice on every search keystroke

[`templates/+page.svelte:114-157`](../../../frontend/src/routes/admin/crm/templates/+page.svelte). Three effects interact:

```ts
// 1. debounce search
$effect(() => { /* writes debouncedSearch after 300ms */ });

// 2. reload data
$effect(() => {
    debouncedSearch; selectedCategory; currentPage; perPage;
    if (isInitialLoad) return;
    loadTemplates();
});

// 3. reset to page 1 on filter change
$effect(() => {
    debouncedSearch; selectedCategory;
    if (!isInitialLoad) currentPage = 1;
});
```

When the user types: effect 1 writes `debouncedSearch` → effect 2 fires `loadTemplates` AND effect 3 fires `currentPage = 1` → effect 2 fires again because `currentPage` is in its deps. **Two backend calls per filter change.** Move the `currentPage = 1` into the same place that updates `debouncedSearch` (or into the search input handler) and remove effect 3.

### 12. Templates: bulk-delete is N sequential network requests with no transaction

[`templates/+page.svelte:236-243`](../../../frontend/src/routes/admin/crm/templates/+page.svelte):

```ts
for (const id of selectedTemplates) {
    try { await crmAPI.deleteEmailTemplate(id); successCount++; }
    catch { failCount++; }
}
```

If the user selects 50 templates, this fires 50 sequential DELETE requests. If the network drops at request 30, 30 templates are gone, 20 remain, and the user gets "Deleted 30, failed to delete 20". No transaction wrapper. Backend should expose `POST /api/admin/crm/templates/bulk-delete` with an `ids[]` body and run it in a transaction. Compare [`leads/+page.svelte:423-436`](../../../frontend/src/routes/admin/crm/leads/+page.svelte), which correctly uses a single `bulk-delete` endpoint.

### 13. Stats endpoint silently returns zeros on backend failure

[`api/admin/crm/stats/+server.ts:35-75`](../../../frontend/src/routes/api/admin/crm/stats/+server.ts):

```ts
if (response.ok) { return json({ success: true, data }); }
console.warn(`Backend CRM stats returned ${response.status}`);
// ...
// Return empty stats on error
return json({ success: true, data: { total_contacts: 0, ... } });
```

When the backend is down or returns 5xx, the proxy returns `success: true` with all-zero stats. The dashboard then shows "0 contacts, $0 revenue" which is indistinguishable from a real empty CRM. Should propagate a 5xx so the frontend can render an "API down" banner. (Other proxies in the cluster — `contacts/`, `deals/` — return the upstream status; only stats has this fallback.)

### 14. `automations/[id]` and `automations/[id]/edit` capture `funnelId` in `const`, not `$derived`

[`automations/[id]/+page.svelte:63`](../../../frontend/src/routes/admin/crm/automations/[id]/+page.svelte):
```ts
const funnelId = page.params.id ?? '';
```
[`automations/[id]/edit/+page.svelte:71`](../../../frontend/src/routes/admin/crm/automations/[id]/edit/+page.svelte):
```ts
const funnelId = page.params.id ?? '';
```

If the user navigates from `/automations/A` to `/automations/B` without the component remounting (SvelteKit does reuse `+page.svelte` instances for same-route different-params nav), `funnelId` keeps pointing at A. All subsequent `crmAPI.updateAutomationFunnel(funnelId, …)` calls will silently mutate the wrong funnel — a serious data-integrity bug. Compare the correct pattern in [`webhooks/[id]/edit/+page.svelte:34`](../../../frontend/src/routes/admin/crm/webhooks/[id]/edit/+page.svelte) (`let webhookId = $derived(page.params.id ?? '')`) and [`deals/[id]/+page.svelte:101`](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte) (`let dealId = $derived(page.params.id as string)`) and [`contacts/[id]/+page.svelte:200`](../../../frontend/src/routes/admin/crm/contacts/[id]/+page.svelte).

### 15. Deals page: "Delete" button is actually a soft-archive

[`deals/+page.svelte:232-244`](../../../frontend/src/routes/admin/crm/deals/+page.svelte):

```ts
async function confirmDeleteDeal() {
    // ...
    await crmAPI.updateDeal(deal.id, { status: 'abandoned' } as any);
    await loadData();
}
```

User clicks the trash icon, sees "Delete deal?" confirmation, says yes — but the deal isn't deleted, it's status-flipped to `abandoned`. The `as any` cast suggests the type system was actively warning about this. Either: rename the button "Archive" / "Abandon", or call a real `crmAPI.deleteDeal(id)`.

### 16. `{@html}` on backend-controlled stat strings in abandoned-carts

[`abandoned-carts/+page.svelte:208,218,228`](../../../frontend/src/routes/admin/crm/abandoned-carts/+page.svelte):

```svelte
<span class="stat-value">{@html stats.recovered_revenue.value}</span>
```

`StatWidget.value` is a backend-controlled `string` ([types.ts:973](../../../frontend/src/lib/crm/types.ts)). Less risky than the templates XSS because it's only the backend producing it (not other admins), but `@html` for a currency display is gratuitous and breaks the principle of least surprise. Render as `{stats.recovered_revenue.value}` or have the backend return a numeric `value` plus a `currency` and format on the client.

### 17. `lead.status` optimistic mutation outside transaction-aware reload

[`leads/[id]/+page.svelte:176-188`](../../../frontend/src/routes/admin/crm/leads/[id]/+page.svelte):

```ts
async function updateStatus(newStatus: string) {
    if (!lead) return;
    try {
        await api.patch(`/api/admin/crm/leads/${leadId}/status`, { status: newStatus });
        lead.status = newStatus as Lead['status'];   // optimistic AFTER await
        showToast('success', `Status updated to ${newStatus}`);
        await loadLead();
    } catch (e) { /* ... */ }
}
```

This isn't optimistic — the local mutation runs after the round-trip, and then `loadLead()` runs another round-trip a moment later. The local write at line 181 is redundant and racy: between line 181 and the completion of `loadLead()` at line 183, the UI shows the user-typed status, but if the backend ran a workflow that changed the status further (e.g., automation moved `qualified` → `proposal`), the user sees a flicker. Drop line 181 and rely on `loadLead`.

---

## Low / nits (P3)

### 18. `automations/+page.svelte:198` uses native `confirm()` instead of `<ConfirmationModal>`

[`automations/+page.svelte:197-213`](../../../frontend/src/routes/admin/crm/automations/+page.svelte) — `deleteFunnel` uses `confirm('Are you sure...')`. Every other page uses the project's `ConfirmationModal` component. Inconsistent UX.

### 19. Deals drag-drop has no optimistic UI

[`deals/+page.svelte:270-278`](../../../frontend/src/routes/admin/crm/deals/+page.svelte). On drop, `updateDealStage` PATCHes the backend, then full-reloads with `loadData()`. The user watches the card snap back to its original column for the duration of the round-trip. Ideally, optimistic move in the local `deals` array, then reconcile (or revert) once the backend responds.

### 20. Smart-links: `stats = computedStats` is dead code

[`smart-links/+page.svelte:96, 121, 145`](../../../frontend/src/routes/admin/crm/smart-links/+page.svelte). `stats` is written but the template uses `computedStats` directly on inspection. The redundant `$state` block can be deleted.

### 21. Proxy shim forwards `Set-Cookie` and other upstream headers verbatim

[`createProxyShim.ts:75-78`](../../../frontend/src/lib/utils/createProxyShim.ts):

```ts
return new Response(upstream.body, {
    status: upstream.status,
    headers: upstream.headers
});
```

If the Rust backend ever returns `Set-Cookie`, it will rewrite the user's session cookies. Headers like `Transfer-Encoding`, `Connection`, `Content-Length` may also collide with the SvelteKit response handling. Filter the upstream headers to a known-safe list (`Content-Type`, `Cache-Control`, `Last-Modified`, `ETag`, `Location`).

### 22. Console-style log dressing in production

Multiple pages (e.g. [`leads/+page.svelte:308,338,...`](../../../frontend/src/routes/admin/crm/leads/+page.svelte)) include `console.error('Load leads error:', err);`. Acceptable in dev but should ideally route through a structured logger / Sentry boundary in production. Errors don't include lead emails/PII directly, so not a leak — just noise.

### 23. Email-send modals lack frontend rate-limiting

[`contacts/[id]/+page.svelte:381-423`](../../../frontend/src/routes/admin/crm/contacts/[id]/+page.svelte) (`sendEmail`) and the various `bulk-update` paths trust the backend to throttle. The button has `disabled={sendingEmail}` so single-form double-submit is prevented, but a malicious or misconfigured admin can rapid-fire across multiple tabs. Backend rate-limit is mandatory.

### 24. `automations/+page.svelte:236-264` — `importFunnel` parses unbounded JSON

The user pastes JSON, `JSON.parse` runs, then the parsed object is POSTed to the backend. No size limit, no schema validation. A 10 MB JSON paste would lock up the browser tab during parse. Consider a 1 MB sanity ceiling and a basic shape check (`data.title`, `data.actions`) before the request.

---

## Reactivity refactor verification (re commit 34a0bd070)

The commit's stated goal was to replace `$effect` with `onMount` in CRM and campaigns where the effect was just an init-on-mount call. Verification sweep:

| Page | Migration done? | Notes |
|---|---|---|
| [`crm/+page.svelte:290`](../../../frontend/src/routes/admin/crm/+page.svelte) | Yes | Old `$effect` is commented out at lines 274-289 — should be removed in a follow-up cleanup pass |
| [`campaigns/+page.svelte:225`](../../../frontend/src/routes/admin/crm/campaigns/+page.svelte) | Yes | Clean `onMount` |
| [`templates/+page.svelte:358`](../../../frontend/src/routes/admin/crm/templates/+page.svelte) | Yes for the lifecycle effect; the three filter-effects remain (issue #11) |
| [`leads/+page.svelte:559`](../../../frontend/src/routes/admin/crm/leads/+page.svelte) | **NO** — same pattern that was fixed in `crm/+page.svelte` survives here |
| [`managers/+page.svelte:108`](../../../frontend/src/routes/admin/crm/managers/+page.svelte) | **NO** |
| [`deals/[id]/+page.svelte:324`](../../../frontend/src/routes/admin/crm/deals/[id]/+page.svelte) | **NO** |
| [`automations/new/+page.svelte:325`](../../../frontend/src/routes/admin/crm/automations/new/+page.svelte) | **NO** |
| [`automations/[id]/edit/+page.svelte:400`](../../../frontend/src/routes/admin/crm/automations/[id]/edit/+page.svelte) | **NO** |
| [`automations/+page.svelte:489`](../../../frontend/src/routes/admin/crm/automations/+page.svelte) | **NO** for lifecycle, OK for filter-tracker at 459 |
| [`sequences/+page.svelte:325`](../../../frontend/src/routes/admin/crm/sequences/+page.svelte) | **NO** for lifecycle, broken for filter-tracker at 297 (issue #5) |
| [`contacts/+page.svelte:33,76`](../../../frontend/src/routes/admin/contacts/+page.svelte) | **NO** for lifecycle, broken filter (issue #7) |
| [`tags/[id]/+page.svelte:148`](../../../frontend/src/routes/admin/crm/tags/[id]/+page.svelte) | **NO** but it is correctly used as a `tagId`-tracker — should remain `$effect` since `tagId` is `$derived` from `page.params` |
| [`smart-links/+page.svelte:170,191`](../../../frontend/src/routes/admin/crm/smart-links/+page.svelte) | The two filter-effects use `untrack` properly — these are correct and should stay |

**Verdict:** the refactor is **roughly half done**. The pages explicitly named in the commit message (campaigns, top-level CRM dashboard) are migrated cleanly. The other `$effect → onMount` candidates listed above were not touched.

---

## Summary

| Severity | Count |
|---|---|
| P0 | 3 (orphan-link epidemic, `Math.random()` for crypto secrets, leads bulk-delete with no confirmation) |
| P1 | 6 (fake test-email stub, broken filter effect in sequences, list-page filter re-fetch missing across 7+ pages, contacts filter dead-zone, stored XSS via templates, SSRF-permissive webhook URL validator) |
| P2 | 8 (incomplete reactivity refactor, double-fetch cascade in templates, non-transactional bulk-delete in templates, stats endpoint silently returns zeros, `funnelId` as `const` in two automation pages, "delete" actually archives in deals, gratuitous `@html` in abandoned-carts, redundant optimistic write in lead status update) |
| P3 | 7 (native `confirm()` mismatch, no optimistic kanban DnD, dead `stats` rune in smart-links, proxy shim header passthrough, console-error noise, no frontend email rate-limit, unbounded import-JSON parse) |

**Top-line recommendation:** the orphan-link epidemic (issue #1) is the highest-leverage fix — the cluster is non-functional for any creation workflow. The webhook-secret RNG (issue #2) and bulk-delete confirmation (issue #3) are independent footguns that should land in the same hot-fix branch. The reactivity refactor should be completed in a follow-up commit that finishes what `34a0bd070` started.
