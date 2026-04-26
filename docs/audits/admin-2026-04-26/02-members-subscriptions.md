# 02 — Members / Subscriptions / Memberships Audit

Date: 2026-04-26
Scope: `frontend/src/routes/admin/{members,memberships,subscriptions}/**` and matching `frontend/src/routes/api/admin/{members,member-management,membership-plans,subscriptions}/**`. Backend touch-points (`api/src/routes/admin_member_management.rs`, `admin_members.rs`, `subscriptions_admin.rs`) cited where they are the actual root cause of a frontend bug.

Read-only audit — no files modified.

---

## Files reviewed

Frontend pages (12):

- [admin/members/+page.svelte](frontend/src/routes/admin/members/+page.svelte) (2443 lines)
- [admin/members/[id]/+page.svelte](frontend/src/routes/admin/members/[id]/+page.svelte) (2248 lines)
- [admin/members/analytics/+page.svelte](frontend/src/routes/admin/members/analytics/+page.svelte) (1239 lines)
- [admin/members/churned/+page.svelte](frontend/src/routes/admin/members/churned/+page.svelte) (1420 lines)
- [admin/members/past/+page.svelte](frontend/src/routes/admin/members/past/+page.svelte) (1994 lines)
- [admin/members/past/+page.ts](frontend/src/routes/admin/members/past/+page.ts) (112 lines)
- [admin/members/segments/+page.svelte](frontend/src/routes/admin/members/segments/+page.svelte) (1874 lines)
- [admin/members/service/[id]/+page.svelte](frontend/src/routes/admin/members/service/[id]/+page.svelte) (1005 lines)
- [admin/members/subscriptions/+page.svelte](frontend/src/routes/admin/members/subscriptions/+page.svelte) (1072 lines)
- [admin/memberships/+page.svelte](frontend/src/routes/admin/memberships/+page.svelte) (1926 lines)
- [admin/memberships/create/+page.svelte](frontend/src/routes/admin/memberships/create/+page.svelte) (985 lines)
- [admin/subscriptions/+page.svelte](frontend/src/routes/admin/subscriptions/+page.svelte) (942 lines)
- [admin/subscriptions/invoice-settings/+page.svelte](frontend/src/routes/admin/subscriptions/invoice-settings/+page.svelte) (996 lines)
- [admin/subscriptions/plans/+page.svelte](frontend/src/routes/admin/subscriptions/plans/+page.svelte) (1708 lines)

Frontend proxies (5):

- [api/admin/member-management/+server.ts](frontend/src/routes/api/admin/member-management/+server.ts) (78 lines)
- [api/admin/member-management/[id]/+server.ts](frontend/src/routes/api/admin/member-management/[id]/+server.ts) (148 lines)
- [api/admin/members/stats/+server.ts](frontend/src/routes/api/admin/members/stats/+server.ts) (64 lines)
- [api/admin/membership-plans/+server.ts](frontend/src/routes/api/admin/membership-plans/+server.ts) (45 lines)
- [api/admin/subscriptions/plans/stats/+server.ts](frontend/src/routes/api/admin/subscriptions/plans/stats/+server.ts) (64 lines)

Spot-checks for cross-cutting issues: [api/src/routes/admin_member_management.rs](api/src/routes/admin_member_management.rs), [api/src/routes/admin_members.rs](api/src/routes/admin_members.rs), [api/src/routes/subscriptions_admin.rs](api/src/routes/subscriptions_admin.rs).

---

## Critical bugs (P0)

### P0-1 — `admin/subscriptions/+page.svelte` is structurally broken: connection check tests function reference, never the result

[admin/subscriptions/+page.svelte:347](frontend/src/routes/admin/subscriptions/+page.svelte#L347)

```svelte
{#if connectionLoading}
    <div class="loading">…</div>
{:else if !getIsPaymentConnected}
    <ServiceConnectionStatus feature="payment" variant="card" showFeatures={true} />
{:else}
    …all real content (stats, alerts, table, modals)…
{/if}
```

`getIsPaymentConnected` is a function imported from `$lib/stores/connections.svelte`. `!getIsPaymentConnected` evaluates the function reference, which is always truthy → `!fn` is always `false`, so the `{:else if}` branch is never taken… which means the actual page content (the entire `{:else}` block) is also never reached. **The Subscription Management page renders only the `ServiceConnectionStatus` placeholder, no matter what.** The same boolean *is* called correctly elsewhere at [line 77](frontend/src/routes/admin/subscriptions/+page.svelte#L77) and [line 330](frontend/src/routes/admin/subscriptions/+page.svelte#L330), so this is an isolated typo. Fix: `{:else if !getIsPaymentConnected()}`.

### P0-2 — Member soft-delete is non-transactional and never tells Stripe to stop charging

[api/src/routes/admin_member_management.rs:660-749](api/src/routes/admin_member_management.rs#L660) (the `delete_member` handler that backs the frontend `DELETE /api/admin/member-management/:id` call wired up in [admin/members/+page.svelte:260](frontend/src/routes/admin/members/+page.svelte#L260)).

Three independent writes, no `Pool::begin()`:

```rust
.execute(&state.db.pool)             // 1. UPDATE users (anonymize)
…
let _ = sqlx::query(
    "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW() …"
).execute(&state.db.pool).await;     // 2. result swallowed via let _

let _ = sqlx::query(
    "INSERT INTO user_activity_log …"
).execute(&state.db.pool).await;     // 3. result swallowed via let _
```

Failure modes:

- If step 1 succeeds and step 2 fails the user is anonymized but their subscriptions stay `active` — the row no longer has an identifiable owner but Stripe / the renewal job will keep billing it.
- The `let _ = … .await` pattern violates the CLAUDE.md rule "Don't swallow errors with `unwrap_or_default()` on `Result<T,E>` — propagate via `?`". Network blips become invisible data corruption.
- There is no Stripe call to cancel the subscription on Stripe's side. Even with a happy path, Stripe will continue to attempt charges against the (now dangling) `stripe_customer_id` until the next webhook delivery fails.

CLAUDE.md is explicit: "New mutations that touch >1 table need a `Pool::begin()` → `tx.commit()` transaction wrapper." This one touches three. The sibling `create_member` at [admin_member_management.rs:451-498](api/src/routes/admin_member_management.rs#L451) was already retrofitted with a transaction in 2026-04-26 — `delete_member` was missed.

### P0-3 — `/admin/memberships` CRUD calls a proxy that only implements GET

Frontend [admin/memberships/+page.svelte](frontend/src/routes/admin/memberships/+page.svelte) hits four mutation paths via `adminFetch`:

- `POST /api/admin/membership-plans` — duplicate plan, [line 324](frontend/src/routes/admin/memberships/+page.svelte#L324)
- `PUT /api/admin/membership-plans/${id}` — edit plan, [line 267](frontend/src/routes/admin/memberships/+page.svelte#L267)
- `DELETE /api/admin/membership-plans/${id}` — delete plan, [line 287](frontend/src/routes/admin/memberships/+page.svelte#L287)
- `PATCH /api/admin/membership-plans/${id}` — toggle active, [line 301](frontend/src/routes/admin/memberships/+page.svelte#L301)

…and `admin/memberships/create/+page.svelte` posts to the same path at [line 145](frontend/src/routes/admin/memberships/create/+page.svelte#L145):

```ts
await adminFetch('/api/admin/membership-plans', {
    method: 'POST',
    body: JSON.stringify({ ... })
});
```

The proxy file [api/admin/membership-plans/+server.ts](frontend/src/routes/api/admin/membership-plans/+server.ts) **only exports `GET`**. Every create/edit/duplicate/delete/toggle in the Membership Plans UI returns a 405 from SvelteKit before it ever reaches Rust. The CRUD that *does* exist in Rust lives at `/admin/subscriptions/plans` ([subscriptions_admin.rs:1101-1110](api/src/routes/subscriptions_admin.rs#L1101)) — wrong base path. Either the proxy needs `POST/PUT/PATCH/DELETE` handlers, or the page needs to be repointed at `/api/admin/subscriptions/plans`.

### P0-4 — `/admin/subscriptions/plans` page's CRUD/Stripe-price flows hit a proxy that doesn't exist

[admin/subscriptions/plans/+page.svelte](frontend/src/routes/admin/subscriptions/plans/+page.svelte) calls **five** different paths under `/api/admin/subscriptions/plans/...`:

- `GET /api/admin/subscriptions/plans?per_page=100` ([line 131](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L131))
- `PUT /api/admin/subscriptions/plans/${id}` ([line 285](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L285), also `togglePlanActive` [line 324](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L324))
- `GET /api/admin/subscriptions/plans/${id}/price-history` ([line 200](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L200))
- `POST /api/admin/subscriptions/plans/${id}/price` ([line 233](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L233))

The only file under `frontend/src/routes/api/admin/subscriptions/` is [plans/stats/+server.ts](frontend/src/routes/api/admin/subscriptions/plans/stats/+server.ts). The Rust router exists and is correct ([subscriptions_admin.rs:1102-1110](api/src/routes/subscriptions_admin.rs#L1102)) — what's missing is the SvelteKit proxy layer. Result: the entire Subscription Plans admin page (the Stripe-syncing one) is non-functional in production for every operation except the stats card. List view, edit, status toggle, price-change, price history — all 404.

### P0-5 — Member detail page's Grant/Extend/Revoke membership all hit a non-existent proxy

[admin/members/[id]/+page.svelte](frontend/src/routes/admin/members/[id]/+page.svelte) drives the per-member subscription actions through `/api/admin/user-memberships`:

- `PUT /api/admin/user-memberships/${selectedSubscription.id}` — extend by N days, [line 365](frontend/src/routes/admin/members/[id]/+page.svelte#L365)
- `POST /api/admin/user-memberships` — grant new membership, [line 397](frontend/src/routes/admin/members/[id]/+page.svelte#L397)
- `DELETE /api/admin/user-memberships/${subId}` — revoke, [line 437](frontend/src/routes/admin/members/[id]/+page.svelte#L437)

There is **no** `frontend/src/routes/api/admin/user-memberships/` directory at all — these are 404s, swallowed by the page's `catch` block which then shows a generic "Failed to extend membership" toast even though the request never landed.

The frontend also calls `GET /api/admin/membership-plans` for the plan picker at [line 328](frontend/src/routes/admin/members/[id]/+page.svelte#L328) — that one *does* exist (the GET-only proxy from P0-3), so the modal opens with plans, but the user can't grant.

---

## High-severity issues (P1)

### P1-1 — Member detail page's notes & email-history calls have no proxy and silently fall through to "empty state"

[admin/members/[id]/+page.svelte](frontend/src/routes/admin/members/[id]/+page.svelte):

- `loadMemberNotes()` at [line 133](frontend/src/routes/admin/members/[id]/+page.svelte#L133): `fetch('/api/admin/members/${memberId}/notes')`
- POST in `addNote()` at [line 189](frontend/src/routes/admin/members/[id]/+page.svelte#L189): `fetch('/api/admin/members/${memberId}/notes', { method: 'POST', … })`
- `loadEmailHistory()` at [line 149](frontend/src/routes/admin/members/[id]/+page.svelte#L149): `fetch('/api/admin/members/${memberId}/emails')`

No proxy file exists for any of these (only [api/admin/members/stats/+server.ts](frontend/src/routes/api/admin/members/stats/+server.ts) is present). The calls 404, the `if (response.ok) { … } else { notes = []; }` path silently sets the empty state, and the "Notes" / "Emails" tab appears to legitimately have no data — masking the missing wiring.

Backend routes *do* exist at [admin_members.rs:1306-1307](api/src/routes/admin_members.rs#L1306) (`/admin/members/:id/notes` and `/admin/members/:id/emails`) and at [admin_member_management.rs:1472-1473](api/src/routes/admin_member_management.rs#L1472) (a duplicate notes implementation under `/admin/member-management/:id/notes`). Two backend routes for "notes" doubles the surface but only adds confusion.

`addNote()` has a worse failure mode: the `else` branch at [line 202-215](frontend/src/routes/admin/members/[id]/+page.svelte#L202) treats a non-2xx response as success-with-warning ("Note added locally - sync pending") and inserts a fake note with `id: Date.now()` into `notes`. That note will never be on the server and will vanish on refresh, leaving the admin to believe a note was saved when it wasn't.

### P1-2 — `/admin/members/analytics` calls six endpoints that don't exist; entire page renders the "not connected" empty state by design

[admin/members/analytics/+page.svelte:71-79](frontend/src/routes/admin/members/analytics/+page.svelte#L71)

```ts
const [metricsRes, growthRes, cohortRes, revenueRes, churnRes, segmentRes] =
    await Promise.allSettled([
        fetch(`/api/admin/members/analytics/metrics?range=${dateRange}`),
        fetch(`/api/admin/members/analytics/growth?range=${dateRange}`),
        fetch(`/api/admin/members/analytics/cohorts?range=${dateRange}`),
        fetch(`/api/admin/members/analytics/revenue?range=${dateRange}`),
        fetch(`/api/admin/members/analytics/churn-reasons?range=${dateRange}`),
        fetch(`/api/admin/members/analytics/segments?range=${dateRange}`)
    ]);
```

None of `/api/admin/members/analytics/*` exist as proxy files, and `grep` finds no `members/analytics` routes in the Rust router either. All six fail, `dataReceived` stays false, and the page surfaces `"Member analytics data is not available. Ensure your analytics service is connected and configured."` as if this were a config issue. This is dead UI shipping in production.

### P1-3 — Past Members dashboard calls a phantom `/api/admin/past-members-dashboard/*` namespace

[lib/api/past-members-dashboard.ts:22](frontend/src/lib/api/past-members-dashboard.ts#L22) sets `const API_BASE = '/api/admin/past-members-dashboard';` and the file then `fetch`es `/overview`, `/services`, `/campaigns`, `/realtime`, `/bulk-winback`, `/bulk-survey`, `/invalidate-cache` (lines 243-443). None of these proxy directories exist. The *page* ([admin/members/past/+page.svelte](frontend/src/routes/admin/members/past/+page.svelte)) is 1994 lines of UI code that depends entirely on this dead client. The `+page.ts` load function at [past/+page.ts:85-89](frontend/src/routes/admin/members/past/+page.ts#L85) calls four of these and returns an `authError: true` payload on any failure, so the page renders the "session expired" branch in production.

### P1-4 — Segments page's CRUD calls fall back silently to local-only state

[admin/members/segments/+page.svelte:186-189](frontend/src/routes/admin/members/segments/+page.svelte#L186)

```ts
const [segmentsRes, tagsRes, filtersRes] = await Promise.all([
    adminFetch('/api/admin/members/segments').catch(() => null),
    adminFetch('/api/admin/members/tags').catch(() => null),
    adminFetch('/api/admin/members/filters').catch(() => null)
]);
```

No proxies exist for any of those three paths. The `.catch(() => null)` is the giveaway — segment/tag/filter data is currently always served from the hardcoded fallbacks in the same function (lines 197-296), then mutations like `handleCreateSegment` ([line 330](frontend/src/routes/admin/members/segments/+page.svelte#L330)) insert into local-only state with `id: segments.length + 1` and `memberCount: Math.floor(Math.random() * 1000) + 100`. Random subscriber counts being shown to admins as if they were real is an enterprise-credibility liability.

The deletion paths are also uniquely sketchy — [line 426-433](frontend/src/routes/admin/members/segments/+page.svelte#L426):

```ts
try {
    await adminFetch(`/api/admin/members/segments/${segment.id}`, { method: 'DELETE' });
} catch {
    // Fallback to local state
}
segments = segments.filter((s) => s.id !== segment.id);
```

Whether the request succeeds, fails, or 404s, the row disappears from the UI and a "Segment deleted" toast fires. The admin can never tell what actually happened.

### P1-5 — `/admin/subscriptions/invoice-settings` whole page targets a missing proxy namespace

[admin/subscriptions/invoice-settings/+page.svelte:15](frontend/src/routes/admin/subscriptions/invoice-settings/+page.svelte#L15)

```ts
const API_BASE = '/api/admin/invoice-settings';
```

Used by `loadSettings`, `saveSettings`, `uploadLogo`, `removeLogo`, `loadPreview`, `downloadPreview`, `confirmResetToDefaults` (lines 73-180). No proxy and no Rust route — `grep -nE "invoice.settings|invoice_settings"` against `api/src/routes/*.rs` returns nothing. The page is dead. Listed P1 not P0 only because the surface is non-billing-critical.

### P1-6 — Memberships page rebuilds `stats` inside `$effect` (write-while-read cascade)

[admin/memberships/+page.svelte:208-229](frontend/src/routes/admin/memberships/+page.svelte#L208)

```ts
$effect(() => {
    // Recalculate stats when plans change
    if (plans.length > 0) {
        stats = {
            total_plans: plans.length,
            active_plans: plans.filter((p) => p.is_active).length,
            …
            total_mrr: plans.reduce(…)
        };
    }
});
```

This reads `plans` (reactive) and writes `stats` (reactive). It is the same write-while-read cascade pattern that commit `34a0bd070` migrated CRM and campaigns away from per CLAUDE.md ("the shadow-state pattern that emits `state_referenced_locally` warnings"). The fix is `let stats = $derived.by(() => { … })` — no `$effect` needed.

A second instance lives in [admin/memberships/create/+page.svelte:102-110](frontend/src/routes/admin/memberships/create/+page.svelte#L102):

```ts
$effect(() => {
    // Auto-generate slug from name if not manually edited
    if (!slugManuallyEdited && membership.name) {
        membership.slug = membership.name.toLowerCase()…
    }
});
```

Reads `membership.name` + `slugManuallyEdited`, writes `membership.slug`. Same anti-pattern. Should be a `$derived` or wired to the input's `oninput`.

### P1-7 — `admin/members/+page.svelte` divides by zero in the active-subscriber stat ring

[admin/members/+page.svelte:526-535](frontend/src/routes/admin/members/+page.svelte#L526)

```svelte
stroke-dasharray="{(stats.subscriptions.active / stats.overview.total_members) *
    100} 100"
…
{Math.round((stats.subscriptions.active / stats.overview.total_members) * 100)}%
```

When `total_members === 0` (fresh install or stats endpoint returning the zero-fallback at [api/admin/members/stats/+server.ts:45-63](frontend/src/routes/api/admin/members/stats/+server.ts#L45)) both branches divide-by-zero. `stroke-dasharray="NaN 100"` is invalid SVG and renders inconsistently across browsers; the percentage label renders as `NaN%`. Same risk applies if `total_members < active` (impossible in practice but unchecked). Guard with a denominator check.

### P1-8 — Bulk-imported member CSV is fake (`Math.random()` count), no actual upload

[admin/members/+page.svelte:215-233](frontend/src/routes/admin/members/+page.svelte#L215)

```ts
async function handleImport() {
    …
    importing = true;
    try {
        // Simulate import - in production this would call the API
        await new Promise((r) => setTimeout(r, 1500));
        toastStore.success(`Imported ${Math.floor(Math.random() * 50) + 10} members successfully`);
        …
```

The toast lies. The import button is purely cosmetic, but it's prominent enough in the toolbar that an admin will assume it works. Either gate it behind a "feature in development" badge or remove it. P1 because users believe data was imported.

### P1-9 — Plaintext temporary password surfaced via toast

[admin/members/+page.svelte:272-279](frontend/src/routes/admin/members/+page.svelte#L272)

```ts
function handleMemberSaved(savedMember: Member, temporaryPassword?: string) {
    if (temporaryPassword) {
        toastStore.success(
            `Member ${savedMember.name} created! Temporary password: ${temporaryPassword}`
        );
    }
…
```

The Rust handler returns `temporary_password` once on creation ([admin_member_management.rs:503](api/src/routes/admin_member_management.rs#L503)) — that part is fine. The frontend then renders the password into a transient toast, where it can be auto-dismissed before the admin reads it, screenshotted by support tooling, or copied into the browser's accessibility tree (most screen readers announce toasts). Best practice is a modal with explicit Copy-to-Clipboard + "I've recorded this" acknowledge before close.

### P1-10 — Admin role gate is too broad for destructive operations

[api/src/routes/admin_members.rs:25-38](api/src/routes/admin_members.rs#L25)

```rust
fn require_admin(user: &User) -> Result<(), …> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" || role == "developer" {
        Ok(())
    } else { Err(…) }
}
```

`require_admin` is applied uniformly across read-only endpoints (e.g. `GET /admin/members`) and destructive ones (delete member, cancel subscription, refund flows). CLAUDE.md asks for `isSuperadmin` checks before "destructive ops (cancel sub, refund, delete member)"; this codebase never distinguishes. Anyone with role `admin` (or `developer`!) can soft-delete an account that anonymizes their email and cancels their subs. Recommend splitting into `require_admin` (read) and `require_superadmin` (write) and gating `delete_member`, `change_plan_price`, `cancel_subscription`, `delete_plan` behind the latter.

### P1-11 — Plan-stats handler swallows DB errors with `unwrap_or((0,))`

[api/src/routes/subscriptions_admin.rs:1055-1078](api/src/routes/subscriptions_admin.rs#L1055)

```rust
let total_plans: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM membership_plans")
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));
```

Three identical queries each `.unwrap_or((0,))` — if the DB connection drops or the table doesn't exist, the dashboard cheerfully shows `0/0/0` instead of erroring. Direct violation of CLAUDE.md "Don't swallow errors with `unwrap_or_default()` on `Result<T, E>` — propagate via `?`." This is also visible in the frontend which rounds it into a "Membership plans operating normally" headline.

---

## Medium issues (P2)

### P2-1 — Two parallel front-end call patterns for the same admin domain

`apiClient.get('/admin/members…')` (e.g. [members.ts:165](frontend/src/lib/api/members.ts#L165)) hits the Rust API directly cross-origin (auth via `Authorization: Bearer` from `getAuthToken`, no SvelteKit proxy). `fetch('/api/admin/members…')` (e.g. [members.ts:300](frontend/src/lib/api/members.ts#L300)) hits the SvelteKit proxy, which then forwards. Same file, same domain, two cookie/auth assumptions. Recent commit `e2356fa46` standardized proxies on the `rtp_access_token` cookie; the `apiClient` flow doesn't go through that proxy at all so it never benefits. Recommend picking one (proxy is the right answer for prod CORS + cookie-based auth) and removing `apiClient.*` for admin domains.

### P2-2 — `member-management/+server.ts` has confusing "split-personality" routing

[api/admin/member-management/+server.ts](frontend/src/routes/api/admin/member-management/+server.ts) is POST-only; [api/admin/member-management/[id]/+server.ts](frontend/src/routes/api/admin/member-management/[id]/+server.ts) is GET/PUT/DELETE. Listing members lives at `/api/admin/members` (which has no proxy file at all — only a `stats` subdir). The header banner of the +server.ts at [line 6-8](frontend/src/routes/api/admin/member-management/+server.ts#L6) acknowledges this. Confusing for any new contributor; bears risk of someone introducing a list endpoint here that competes with the elsewhere-undefined `/api/admin/members`.

### P2-3 — `subscriptions/+page.svelte` row click + nested action buttons missing `stopPropagation`

[admin/subscriptions/+page.svelte:580-660](frontend/src/routes/admin/subscriptions/+page.svelte#L580)

```svelte
<tr
    class="hover:bg-slate-700/30 transition-colors cursor-pointer"
    onclick={() => openSubscriptionDetail(subscription)}
>
    …
    <button onclick={() => handlePause(subscription)} …>Pause</button>
    <button onclick={() => handleCancel(subscription)} …>Cancel</button>
```

Every action button bubbles up to the row, so clicking "Cancel" both opens the cancel modal *and* opens the detail drawer. The drawer is then immediately covered by the modal but takes focus from it. Add `e.stopPropagation()` to the inner buttons.

Same pattern in [admin/members/segments/+page.svelte:622-697](frontend/src/routes/admin/members/segments/+page.svelte#L622) — outer card has `onclick`, action buttons inside don't stop the bubble.

### P2-4 — Optimistic UI can desync after failed mutations on segments page

[admin/members/segments/+page.svelte:417-465](frontend/src/routes/admin/members/segments/+page.svelte#L417) — every delete branch swallows the API error and applies the local mutation regardless. Since the backend doesn't actually exist yet (P1-4) this is currently masked, but if/when wired up, an HTTP 500 will leave the UI showing the row gone while the DB still has it. Pattern needs a real `if (apiOk) update local; else show error` flow.

### P2-5 — `subscriptions/+page.svelte` `loadData` re-runs from `oninput` filter changes (`onchange={loadData}`) but other filters use `bind:value` only

[admin/subscriptions/+page.svelte:478](frontend/src/routes/admin/subscriptions/+page.svelte#L478) — the Status select fires `onchange={loadData}`, the Sort select [line 510](frontend/src/routes/admin/subscriptions/+page.svelte#L510) does not (it relies on the `$derived` `getFilteredSubscriptions` doing in-memory sort). Inconsistent: status and interval filters round-trip the server, sort doesn't. Doc the contract or unify.

### P2-6 — Subscriptions search filter is duplicate-implemented

[admin/subscriptions/+page.svelte](frontend/src/routes/admin/subscriptions/+page.svelte) sends `searchQuery` to `getSubscriptions()` at [line 114](frontend/src/routes/admin/subscriptions/+page.svelte#L114) **and** filters in-memory in the `getFilteredSubscriptions` `$derived` at [line 165-172](frontend/src/routes/admin/subscriptions/+page.svelte#L165). When the server-side search returns subset A and the client filter applies to subset A, two text-search algorithms compete. Pick one.

### P2-7 — Pagination boundary off-by-one when `total === 0`

[admin/members/+page.svelte:802-805](frontend/src/routes/admin/members/+page.svelte#L802)

```svelte
Showing {(pagination.current_page - 1) * pagination.per_page + 1} to
        {Math.min(pagination.current_page * pagination.per_page, pagination.total)} of
        {pagination.total} members
```

When `pagination.total === 0`, this renders "Showing 1 to 0 of 0 members". Same string in [admin/members/churned/+page.svelte:476-479](frontend/src/routes/admin/members/churned/+page.svelte#L476). Either gate the display behind `{#if pagination.total > 0}` or compute the start index as `pagination.total === 0 ? 0 : …`.

Also the "next" disabled check is `pagination.current_page === pagination.last_page` — when `last_page === 0` and `current_page === 1` this is `false`, so the "Next" button stays clickable on an empty list.

### P2-8 — Search debounce timer in `/admin/members` not cleared on unmount

[admin/members/+page.svelte:82-92](frontend/src/routes/admin/members/+page.svelte#L82) creates `searchDebounceTimer` but the `$effect` block at [line 96-116](frontend/src/routes/admin/members/+page.svelte#L96) returns nothing (no cleanup). Navigate away mid-typing → timer fires after destroy → store update on a destroyed component. The sibling page [admin/subscriptions/+page.svelte:86-90](frontend/src/routes/admin/subscriptions/+page.svelte#L86) does this correctly with a `return () => clearTimeout(…)`. Apply same pattern.

### P2-9 — `/admin/members/segments` `$effect(() => loadData())` is a fire-and-forget one-shot

[admin/members/segments/+page.svelte:174-176](frontend/src/routes/admin/members/segments/+page.svelte#L174)

```ts
$effect(() => {
    loadData();
});
```

Reads no reactive state but runs in an effect; intent is `onMount`. CLAUDE.md commit `34a0bd070` already migrated CRM and campaigns away from this pattern for the same reason — it can re-run during effect-tear-down in dev mode and produce a duplicate fetch. Use `onMount(() => { loadData(); })`.

### P2-10 — `members/+page.svelte` Set mutation pattern leaks reactive grace

[admin/members/+page.svelte:142-149](frontend/src/routes/admin/members/+page.svelte#L142)

```ts
function toggleMemberSelection(id: number) {
    if (selectedMembers.has(id)) {
        selectedMembers.delete(id);
    } else {
        selectedMembers.add(id);
    }
    selectedMembers = selectedMembers;        // self-assign to trigger reactivity
}
```

Same code in `selectAllMembers` ([line 151-158](frontend/src/routes/admin/members/+page.svelte#L151)) and replicated across the `churned`, `service/[id]`, `subscriptions` pages. Self-assignment is a Svelte 4 pattern; in Svelte 5 with `$state` the cleaner way is `selectedMembers = new Set(selectedMembers); selectedMembers.add(id)`. The current pattern works but mixes idioms and is easy to forget (the past page does it correctly with `new Set(selectedMembers)` at [line 226-232](frontend/src/routes/admin/members/past/+page.svelte#L226)).

### P2-11 — Duplicate `id="page-checkbox"` on every row breaks accessibility / tests

[admin/members/churned/+page.svelte:367-373](frontend/src/routes/admin/members/churned/+page.svelte#L367) and [389-395](frontend/src/routes/admin/members/churned/+page.svelte#L389): each rendered row has

```svelte
<input id="page-checkbox" name="page-checkbox" type="checkbox" …>
```

— so for a 20-row table there are 21 elements with `id="page-checkbox"`. Same issue in [admin/memberships/+page.svelte:795](frontend/src/routes/admin/memberships/+page.svelte#L795) (`page-feature-feature-name` repeats per feature), [admin/memberships/+page.svelte:773](frontend/src/routes/admin/memberships/+page.svelte#L773) (`page-formdata-is-active` global), and [admin/members/segments/+page.svelte](frontend/src/routes/admin/members/segments/+page.svelte) (less audited but same lint pattern — generic `page-…` ids). Breaks `<label for=…>` association and Playwright selectors. Use `id="checkbox-{member.id}"`.

### P2-12 — `members/[id]/+page.svelte` orders count uses a property not in the `Member` type

[admin/members/[id]/+page.svelte:681](frontend/src/routes/admin/members/[id]/+page.svelte#L681)

```svelte
<span class="detail-value">{member.orders?.length || 0}</span>
```

`Member.orders` exists in the type at [members.ts:26](frontend/src/lib/api/members.ts#L26), but it's only populated by `GET /admin/members/:id`. The detail page actually fetches via [member-management/[id]/+server.ts](frontend/src/routes/api/admin/member-management/[id]/+server.ts) → `/admin/member-management/:id`, which returns a different shape. Quick read of [admin_member_management.rs:160-200](api/src/routes/admin_member_management.rs#L160) shows the orders field is built from `stripe_orders` / `user_memberships` and not aligned with the `Order` interface in `members.ts`. Worth a typecheck pass to confirm field names match (`number` vs `order_number`, `created_at` etc.).

---

## Low / nits (P3)

### P3-1 — Throwaway `_` prefix variables left in shipped code

- `_isConnected` at [admin/members/analytics/+page.svelte:23](frontend/src/routes/admin/members/analytics/+page.svelte#L23) (read-only, never displayed).
- `_formatDate` at [admin/memberships/+page.svelte:402](frontend/src/routes/admin/memberships/+page.svelte#L402) and [admin/subscriptions/plans/+page.svelte:349](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L349) (defined, never called).
- `_viewSegmentAnalytics` at [admin/members/segments/+page.svelte:519](frontend/src/routes/admin/members/segments/+page.svelte#L519).

Dead code; either wire them up or delete.

### P3-2 — `Math.random()` placeholder values reach stat cards in segments page

[admin/members/segments/+page.svelte:346](frontend/src/routes/admin/members/segments/+page.svelte#L346) — `memberCount: Math.floor(Math.random() * 1000) + 100` is set when an admin creates a segment, then displayed in the segment card. Not a security issue but pollutes the dashboard.

### P3-3 — `parseInt` / `parseFloat` not used; `Number()` and unary-`+` mixed

[admin/memberships/create/+page.svelte:149](frontend/src/routes/admin/memberships/create/+page.svelte#L149): `price: parseFloat(membership.price) || 0`. `parseFloat('5abc')` → 5; bind:value already constrains because input type=number, but this differs from `Number(input.price)` used elsewhere. Pick one for consistency.

### P3-4 — `formatCurrency(amount: number)` doesn't guard against `null/undefined/NaN` consistently

[admin/members/+page.svelte:366-376](frontend/src/routes/admin/members/+page.svelte#L366) does guard. Most other pages don't:

- [admin/members/[id]/+page.svelte:267](frontend/src/routes/admin/members/[id]/+page.svelte#L267)
- [admin/members/churned/+page.svelte:139](frontend/src/routes/admin/members/churned/+page.svelte#L139)
- [admin/members/service/[id]/+page.svelte:121](frontend/src/routes/admin/members/service/[id]/+page.svelte#L121)
- [admin/memberships/+page.svelte:394](frontend/src/routes/admin/memberships/+page.svelte#L394)

If the API returns `null` for `total_spent` (and it does for new members per the type at [members.ts:20](frontend/src/lib/api/members.ts#L20)), `Intl.NumberFormat.format(null)` returns `"$NaN"`. Consolidate into a shared `lib/utils/format.ts`.

### P3-5 — Email body templates use raw `{{ name }}` literal — no replacement client-side

[admin/members/+page.svelte:887](frontend/src/routes/admin/members/+page.svelte#L887) placeholder reads `Email body... Use {{ name }} for personalization`. The send path passes `personalize: true` flag at [line 168](frontend/src/routes/admin/members/+page.svelte#L168), so the backend handles substitution. Confirmed in [admin_members.rs](api/src/routes/admin_members.rs) but worth doc-commenting in the page itself; an admin debugging "why didn't `{{ first_name }}` substitute?" has no in-line hint.

### P3-6 — `admin/subscriptions/plans/+page.svelte` re-encodes the `Authorization: Bearer` header even though the proxy will pull from cookies

[admin/subscriptions/plans/+page.svelte:130-137](frontend/src/routes/admin/subscriptions/plans/+page.svelte#L130) reads `getAuthToken()` and sets `Authorization: Bearer …` for the SvelteKit proxy. Since [the existing proxies prefer cookie tokens](frontend/src/routes/api/admin/member-management/+server.ts#L42), this is redundant with `credentials: 'include'`. Harmless but inconsistent with the rest of the codebase.

### P3-7 — Stat ring divides by zero **and** uses a missing fallback

In addition to the P1-7 NaN issue, the gradient class on the `bg-slate-` stat-ring background (e.g. `getRetentionColor` in [admin/members/analytics/+page.svelte:169-176](frontend/src/routes/admin/members/analytics/+page.svelte#L169)) returns `'bg-slate-800/50'` for `value === 0` but applies it to a percentage label, not a heatmap cell — the function is repurposed for cohort retention and the empty case is fine there. Worth annotating to avoid copy-paste accidents.

### P3-8 — `previewHtml` fed into HTML directly (potential XSS)

[admin/subscriptions/invoice-settings/+page.svelte:160](frontend/src/routes/admin/subscriptions/invoice-settings/+page.svelte#L160) — `previewHtml = data.html;` with intent to render via `{@html previewHtml}` later in the template (file truncated at 200 lines but `previewHtml` is a state variable typed as a string with no sanitization). Trust boundary is "our own backend" so risk is bounded but not zero — Stripe webhook payloads or admin-uploaded company info flow into the same template. Sanitize via DOMPurify or render in a sandboxed iframe.

---

## Cross-cutting concerns

### CC-1 — The `/api/admin/...` proxy layer has large gaps

Counting only this audit's scope, the front-end calls these proxy paths that don't exist:

| Page | Path called | Backend route | Verdict |
|---|---|---|---|
| `/admin/subscriptions/plans` | `/api/admin/subscriptions/plans` (GET/PUT) | `subscriptions_admin.rs::plans_router()` | proxy missing |
| `/admin/subscriptions/plans` | `/api/admin/subscriptions/plans/:id/price` | `subscriptions_admin.rs::change_plan_price` | proxy missing |
| `/admin/subscriptions/plans` | `/api/admin/subscriptions/plans/:id/price-history` | `subscriptions_admin.rs::list_plan_price_history` | proxy missing |
| `/admin/memberships` | `POST/PUT/PATCH/DELETE /api/admin/membership-plans/:id?` | `subscriptions_admin.rs::plans_router()` (via `/admin/subscriptions/plans`!) | proxy partial + path mismatch |
| `/admin/members/[id]` | `/api/admin/user-memberships*` | needs verification | proxy missing |
| `/admin/members/[id]` | `/api/admin/members/:id/notes` | `admin_members.rs::get_member_notes` | proxy missing |
| `/admin/members/[id]` | `/api/admin/members/:id/emails` | `admin_members.rs::get_member_emails` | proxy missing |
| `/admin/members/analytics` | `/api/admin/members/analytics/*` | none | proxy + backend missing |
| `/admin/members/segments` | `/api/admin/members/{segments,tags,filters}` | none | proxy + backend missing |
| `/admin/members/past` | `/api/admin/past-members-dashboard/*` | likely a separate `past_members.rs` backend; unverified | proxy missing |
| `/admin/subscriptions/invoice-settings` | `/api/admin/invoice-settings/*` | none | proxy + backend missing |
| `/admin/members` (export) | `/api/admin/members/export`, `/api/admin/member-management/export` | possibly `admin_members.rs::export_members` | proxy missing |

P0/P1 above call out the worst of these. CC-1's value is recognizing this is a *systemic* gap, not a per-page bug — the SvelteKit proxy layer for `/admin/{members,member-management,membership-plans,subscriptions}` is roughly 30-40 % implemented relative to what the pages assume.

### CC-2 — Two distinct admin client patterns coexist in the same file

`apiClient.get('/admin/members')` (cross-origin, header auth, [members.ts:165](frontend/src/lib/api/members.ts#L165)) versus `fetch('/api/admin/members/export')` (proxied, cookie auth, [members.ts:300](frontend/src/lib/api/members.ts#L300)). Pick one and migrate. The recent commit `e2356fa46` ("update authorization handling across admin API routes to prefer rtp_access_token cookie") only fixed the proxy side; the direct `apiClient` path still needs to be reconciled.

### CC-3 — Proxy fallbacks return mock-shaped success bodies on 401/404/500

[api/admin/subscriptions/plans/stats/+server.ts:32-43](frontend/src/routes/api/admin/subscriptions/plans/stats/+server.ts#L32):

```ts
if (response.status === 401 || response.status === 404 || response.status === 500) {
    return json({
        data: {
            active_subscriptions: 0,
            total_active: 0,
            …
        }
    });
}
```

A 401 is returned to the client as a 200 with `{ data: { active_subscriptions: 0 } }`. The page then renders "0 active subscriptions" instead of redirecting to login — masking auth issues as data issues. The frontend has no way to tell the difference. Either propagate the actual status, or only fall back on `404`. Same pattern in [api/admin/members/stats/+server.ts:39-43](frontend/src/routes/api/admin/members/stats/+server.ts#L39) but at least there it logs `console.warn`.

### CC-4 — No transaction wrappers on multi-table member mutations

The CLAUDE.md rule is unambiguous; the audit found `delete_member` as the most prominent violation (P0-2). Other multi-table writes worth grepping for in subsequent audits: `update_member` (also writes activity log), `cancel_subscription` (writes user_memberships + activity_log + potentially Stripe), `revoke_membership`. Recommend a single PR that retrofits all of `admin_member_management.rs` and `subscriptions_admin.rs` mutations to use `Pool::begin()` / `tx.commit()` and tracks the change in `RUST_BACKEND_AUDIT_2026-04-25.md`.

### CC-5 — Role gate insufficiently granular for billing/PII writes

P1-10 covers the specific issue. A second-pass audit should map every backend route in scope to:
- `requires_auth`
- `requires_admin`
- `requires_superadmin`

…and gate (a) `delete_member`, (b) any subscription cancel, (c) plan price change ([change_plan_price](api/src/routes/subscriptions_admin.rs#L717) is currently `AdminUser`, not `SuperadminUser`), (d) plan delete, (e) refund flows. Today every `admin` role can do these, including the `developer` role which is meant for engineers debugging in prod.

---

## Summary

The Members / Subscriptions / Memberships surface is in worse shape than the recently-clean proxy snapshots imply. The proxy file directory itself looks fine — every existing `+server.ts` already follows the `env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev'` rule and the `rtp_access_token` cookie convention. **The problem is that 70-80 % of the admin pages in this scope call proxy paths that don't have a `+server.ts` at all.** The pages compile, render, and seem to work in dev because Promise.allSettled / `.catch()` quietly swallows the 404s and shows empty states.

The two genuine code-correctness bugs are:
1. The Subscription Management page's connection guard tests a function reference (P0-1) — the page literally never renders its own data block.
2. `delete_member` lacks a transaction and never tells Stripe the customer is gone (P0-2) — silent data + billing desync.

Everything else falls into three buckets: missing proxy wiring (P0-3 through P1-5), `$effect`-as-onMount / write-while-read cascades (P1-6, P2-9), and accumulated UI polish (duplicate IDs, click-bubbling, divide-by-zero, plaintext password in toast).

Counts:
- **P0** (critical bugs): 5
- **P1** (high-severity issues): 11
- **P2** (medium issues): 12
- **P3** (low / nits): 8
- **Cross-cutting concerns**: 5
