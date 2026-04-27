# 02 — Members / Subscriptions / Memberships — Implementation Results

Date: 2026-04-26
Pass executed by the principal-engineer agent against
`02-members-subscriptions.md`. Working alongside sister agents who landed
many of these fixes earlier in the same window — this doc reflects the
cumulative state, not just my edits.

---

## Status by audit item

| Audit | Title | Status | Notes |
|---|---|---|---|
| P0-1 | Subscription Mgmt page tests function reference | DONE (sister) | `!getIsPaymentConnected()` invocation at `admin/subscriptions/+page.svelte:347` |
| P0-2 | `delete_member` non-transactional + no Stripe cancel | DEFERRED → D1 | Stripe state machine; design proposal in DEFERRED §D1 |
| P0-3 | Memberships CRUD hits GET-only proxy | DONE (sister) | `api/admin/membership-plans/+server.ts` (POST) and `[id]/+server.ts` (GET/PUT/PATCH/DELETE) forward to `/api/admin/subscriptions/plans/...` |
| P0-4 | Subscription Plans page CRUD proxies missing | DONE (sister) | `api/admin/subscriptions/plans/{+server.ts,[id]/+server.ts,[id]/price/+server.ts,[id]/price-history/+server.ts}` |
| P0-5 | Member detail Grant/Extend/Revoke 404s | DONE (sister) | `api/admin/user-memberships/{+server.ts,[id]/+server.ts}` — uses `requireSuperadmin` for grant/PUT/DELETE |
| P1-1 | Member detail notes/emails calls 404 | DONE (sister) | `api/admin/members/[id]/{notes,emails}/+server.ts` exist; `addNote()` else-branch surfaces real error (no fake `id: Date.now()`) |
| P1-2 | Six dead analytics endpoints | DONE (sister) | All six proxies present under `api/admin/members/analytics/` |
| P1-3 | Past-Members dead `/api/admin/past-members-dashboard/*` | THIS PASS — partial | Catch-all proxy created; backend still missing → DEFERRED §D4 |
| P1-4 | Segments page `Math.random()` for member counts | DONE (sister) | Segments page renders `'—'` when count is null |
| P1-5 | Invoice-settings dead | DONE (sister) | `api/admin/invoice-settings/+server.ts` proxy + page wired; backend MISSING → DEFERRED §D2 |
| P1-6 | `$effect` write-while-read on memberships pages | DONE (sister) | `stats` is now `$derived.by(...)`; slug uses `oninput` handler |
| P1-7 | Divide-by-zero on stat ring | DONE (sister) | `total_members > 0 ? ... : 0` guard at lines 568-580 of `admin/members/+page.svelte` |
| P1-8 | Fake CSV import w/ `Math.random()` | DONE (sister) | Toast warns admin import is not wired; tracked in DEFERRED §D3 |
| P1-9 | Plaintext temp password in toast | DONE (sister) | Modal-based reveal with copy-to-clipboard; `temporaryPasswordToReveal` state |
| P1-10 | Broad `require_admin` covers destructive ops | THIS PASS | Added `require_superadmin()` to `admin_members.rs`; `delete_segment`, `delete_member_tag`, `delete_member_filter`, `bulk_assign_tags` now require super-admin |
| P1-11 | `unwrap_or((0,))` swallowing DB errors | THIS PASS | `subscriptions_admin.rs::plan_stats` (3 queries), `list_subscriptions` total count, `list_plans` total count — all now use `?` propagation w/ `tracing::error!` |
| P2-1 | Mixed apiClient vs proxy | DEFERRED → D5 | Cross-cutting refactor |
| P2-4 | Optimistic delete swallows error | DONE (sister) | `handleConfirmDelete` re-throws and calls `loadData()` on failure |
| P2-7 | Pagination off-by-one on empty | DONE (sister) | `Showing {total === 0 ? 0 : start} to ...` in members + churned |
| P2-11 | Duplicate `id="page-checkbox"` | DONE (sister) | Per-row `id="select-{table}-{member.id}"` |
| P3-4 | `formatCurrency(null) → $NaN` | DONE (sister) | Null/NaN guards across admin/members pages |
| P3-8 | Raw `{@html previewHtml}` in invoice-settings | DONE (sister) | Routed through `sanitizeHtml(previewHtml, 'rich')` from `$lib/utils/sanitize` |

---

## Files modified in this pass

### Rust

- `api/src/routes/admin_members.rs`
  - Added `require_superadmin(user: &User) -> Result<...>` helper
  - Tightened `delete_segment`, `delete_member_tag`, `delete_member_filter`,
    `bulk_assign_tags` from `require_admin` → `require_superadmin`
- `api/src/routes/subscriptions_admin.rs`
  - `plan_stats`: replaced 3× `unwrap_or((0,))` with `?` + `tracing::error!`
  - `list_subscriptions`: total-count query now propagates errors
  - `list_plans`: total-count query now propagates errors

### Frontend

- `frontend/src/routes/api/admin/past-members-dashboard/[...path]/+server.ts`
  (NEW)
  - Catch-all GET/POST proxy that forwards every path with the canonical
    `env.API_BASE_URL || env.BACKEND_URL || ...` pattern and
    `requireAdminToken` from `$lib/server/auth`. Returns upstream status
    verbatim (no fake-success masking).

### Docs

- `docs/audits/admin-2026-04-26/02-members-subscriptions-DEFERRED.md` (NEW)
- `docs/audits/admin-2026-04-26/02-members-subscriptions-RESULTS.md` (this file)

---

## Gates

- `cargo check` (api) — PASS
- `cargo clippy` (api) — PASS (no new warnings)
- `pnpm check` (frontend) — PASS, **0 errors / 0 warnings** across 5252 files

No `.svelte` files were edited in this pass (sister agents already
landed those). The Svelte autofixer was therefore not invoked.

---

## Stripe-related items deferred (CALL OUT)

Per the principal hard rule "HIGH BLAST RADIUS — Stripe state machines:
Any change that could double-charge / double-cancel / orphan a Stripe
subscription → DEFER", the following items were intentionally NOT
implemented and are documented in `02-members-subscriptions-DEFERRED.md`:

- **D1 / P0-2** — `delete_member` transaction wrapper + Stripe-cancel
  call. Has a full design proposal that needs human approval before any
  code touches the file.
- **D6 / CC-4** — Same risk class as D1: `update_member`,
  `cancel_subscription`, `revoke_membership` are also non-transactional
  and should ride the same wrapper pattern when D1 lands.

The Subscription Plans page proxies (P0-4) DO forward to backend handlers
that touch Stripe (`change_plan_price` re-prices Stripe products); per
the spec the proxy is a thin pass-through that does NO Stripe calls of
its own — backend continues to own the Stripe write path. That's a stub,
not a stripe write, and is in scope.
