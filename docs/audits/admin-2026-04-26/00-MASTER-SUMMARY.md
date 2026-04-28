# Admin Audit + Remediation — Master Summary

> **Note (2026-04-28):** Fly.io references in this document are historical. The Fly.io deployment was removed; deploy target is TBD. See `backups/fly-io-removed-2026-04-28.md` for original Fly configuration.
**Date:** 2026-04-26
**Scope:** Entire `/admin` surface — 155 admin source files, 53 admin API proxies, 37 admin route folders
**Method:** 12 parallel principal-engineer audit agents → 12+ implementation agents → cross-cutting cleanup passes

---

## Final Status: ✅ ALL 12 REPORTS COMPLETE

`pnpm check` from `frontend/`: **5261 FILES • 0 ERRORS • 0 WARNINGS • 0 FILES_WITH_PROBLEMS**

| # | Cluster | Audit | RESULTS | DEFERRED |
|---|---------|-------|---------|----------|
| 01 | Shell + Dashboard | [link](01-shell-and-dashboard.md) | [link](01-shell-and-dashboard-RESULTS.md) | — |
| 02 | Members / Subs | [link](02-members-subscriptions.md) | [link](02-members-subscriptions-RESULTS.md) | [Stripe redesign](02-members-subscriptions-DEFERRED.md) |
| 03 | Commerce | [link](03-commerce.md) | [link](03-commerce-RESULTS.md) | [Two-router conflict](03-commerce-DEFERRED.md) |
| 04 | Long-form Content | [link](04-content-longform.md) | [link](04-content-longform-RESULTS.md) | none |
| 05 | Video / Media | [link](05-video-media.md) | [link](05-video-media-RESULTS.md) | none |
| 06 | CRM | [link](06-crm.md) | [link](06-crm-RESULTS.md) | [Backend-pending stubs](06-crm-DEFERRED.md) |
| 07 | Marketing | [link](07-marketing.md) | [link](07-marketing-RESULTS.md) | [Duplicate route decisions](07-marketing-DEFERRED.md) |
| 08 | Analytics | [link](08-analytics.md) | [link](08-analytics-RESULTS.md) | none |
| 09 | System / Auth | [link](09-system.md) | [link](09-system-RESULTS.md) | [CSRF strategy](09-system-DEFERRED.md) |
| 10 | Workflow | [link](10-workflow.md) | [link](10-workflow-RESULTS.md) | none |
| 11 | Cross-cutting | [link](11-cross-cutting.md) | [link](11-cross-cutting-RESULTS.md) | none |
| 12 | Sidebar Coverage | [link](12-sidebar-coverage.md) | [link](12-sidebar-coverage-RESULTS.md) | — |

---

## Highest-Impact Wins

### Security (closed)
- **Privilege escalation closed** — Phantom super-admin in `users/[id]` mock data removed; `requireSuperadmin` guard added on all destructive admin endpoints; `connections/status` proxy now auth-gated.
- **SMTP credential leak closed** — `email/settings` proxy now auth-gated and redacts secrets in GET responses; PUT only updates non-empty values.
- **XSS sinks closed** — DOMPurify wrapper at `lib/sanitize.ts`; routed through every `{@html}` in popup preview, blog `html` blocks, lesson content, invoice settings, CRM template preview.
- **SSRF closed** — Webhook URL validator rejects `http://`, `localhost`, all RFC1918 ranges, `169.254.0.0/16`.
- **Cryptographic randomness fix** — Webhook secrets now from `crypto.getRandomValues(32)`, not `Math.random()`.
- **Cloud-storage secret_key leak closed** — Server-side redaction; PUT only updates if non-empty new value provided.
- **CSV injection closed** — Form-entries export prefixes `=`/`+`/`-`/`@`/`\t`/`\r` cells with `'`.

### Architecture (now consistent)
- **Auth-token canonicalization** — Created `frontend/src/lib/server/auth.ts` (`requireAdmin` + `requireSuperadmin`); rolled out across system, members, marketing, video, courses proxies.
- **Backend URL pattern** — All 53 admin proxies now use `env.API_BASE_URL || env.BACKEND_URL || '<your-api-host>'`. 18 redundant `BACKEND_URL = PROD_BACKEND` rebindings collapsed.
- **Same-origin policy** — `apiFetch` patched to short-circuit `/admin/*` to same-origin; 5 email pages no longer bypass the SK proxy.
- **Method-coverage** — `[...rest]` shim built for `courses/[id]/`; missing PUT on `tags/`; missing PUT on `coupons/[id]`.
- **Build proxies, don't fake responses** — Removed silent mock-data fallbacks across schedule, products/stats, coupons, analytics, members. Proxies now surface real backend errors (502 on backend down, 401 on auth fail) instead of fabricating success.

### Reactivity (Svelte 5 cleanup)
- **`$effect → onMount` migration** — Continued the sweep started in commit 34a0bd070. ~30 init-`$effect` blocks converted to `onMount` across CRM, analytics, workflow, video, members, content clusters.
- **Shadow-state eliminated** — 0 occurrences of `let foo = $state(props.foo); $effect(() => { foo = props.foo })` remain in admin.
- **`as any` reduction** — 41 → 1 (the lone remaining cast is documented as irreducible).
- **Filter-effect bug class fixed** — 7+ list pages where filter `$effect` had no reactive reads (so search/status changes never refetched) — all fixed.

### Missing Functionality (built, not removed)
- **CRM orphan-link epidemic** — 21 stub `+page.svelte` files created for "New X / Edit X" buttons across templates, companies, sequences, lists, segments, campaigns, recurring-campaigns, deals, managers, import-export, contacts/leads.
- **Indicators API** — Built `+server.ts`, `[id]/+server.ts`, `[id]/[...rest]/+server.ts` proxies (the entire surface was orphan).
- **Analytics catchall** — Single `[...rest]/+server.ts` covers ~25 previously-orphan endpoints (kpis, funnels, cohorts, segments, events, realtime, attribution, forecast, reports, recordings, heatmaps, goals, behavior).
- **Forms proxy** — Built (page was calling non-existent endpoint).
- **Membership-plans CRUD** — Built PUT/POST/DELETE handlers.
- **User-memberships grant/extend/revoke** — Proxies built; backend gaps documented.
- **Media upload + analytics proxies** — Built with same-origin; AI alt-text and replace endpoints proxied.
- **CMS datasources proxy** — Built; page no longer talks directly to Rust API with a stored token.
- **Site-health proxy** — Built; dashboard no longer renders hardcoded "Operational" lies.

### UX Bugs Fixed
- **Coupon edit cursor-jump** on uppercase coercion → CSS + submit-time conversion.
- **Coupon field-name drift** (`type`/`value` vs `discount_type`/`discount_value`) → canonical-with-legacy-aliases mapping.
- **Product type filter** `?type=` → `?product_type=`; product create now sends `slug`/`sale_price`/`currency`/`features`.
- **Products filter dead** (`untrack` swallowing reactivity) → fixed.
- **Schedule conflict detector** string-compare ignoring per-event timezone → `Intl.DateTimeFormat` UTC instants.
- **Kanban drag-drop** swallowing failures, racing on concurrent drops → optimistic with refetch-on-error rollback + per-board lock.
- **Slug auto-generation clobbering user edits** — `slugEdited` flag stops auto-sync once user manually edits.
- **Weekly video uploader 100% broken** — Literal `VIDEO_LIBRARY_ID` placeholder + non-existent `upload_url` field references → fixed.
- **Trading-room admin redirect on wrong cookie** (`session_token` vs `rtp_access_token`) → fixed.
- **`!getIsAnalyticsConnected` testing function reference** in 9 pages → invocation fixed.
- **`!getIsPaymentConnected` testing function reference** on Subscription Management → fixed.
- **System health reporting hardcoded "Operational"** → real probe via site-health proxy.
- **Bulk-action count fabrication** — Real backend counts now propagated.
- **Off-by-one pagination** ("Showing 1 to 0 of 0") → fixed.
- **Stale state cascades** — `funnelId` capture as `const` (not `$derived`) on automation pages → fixed.

### Sidebar
- **0 dead sidebar links** (verified across all 27 hrefs).
- **Sign-out** triple-defense already correct (verified).
- **Mobile drawer** wiring verified (hamburger, X-close, overlay-tap, link-tap).

---

## Deferred Items (with documentation)

These items genuinely need human design decisions before implementation:

### `02-members-subscriptions-DEFERRED.md`
- D1 — `delete_member` Stripe-cancel + transaction wrapper (needs Stripe webhook reconciliation design)
- D2 — Real invoice-settings backend
- D3 — Real CSV import backend
- D4 — Past-members dashboard backend
- D5 — Cluster-wide `apiClient` → SK-proxy refactor
- D6 — Cancel/revoke Stripe state-machine wrappers (`update_member`, `cancel_subscription`, `revoke_membership`)

### `03-commerce-DEFERRED.md`
- Two coupon Rust routers contradict the DB schema (needs a winner picked)
- "Membership" product type missing from backend enum
- Money as `f64` end-to-end (system-wide refactor to integer cents)
- `/admin/cart/abandoned` backend not built

### `06-crm-DEFERRED.md`
- 21 stub pages need their backend endpoints implemented (templates/companies/sequences/lists/segments/campaigns/recurring-campaigns/deals/managers/import-export/contacts/leads CRUD)

### `07-marketing-DEFERRED.md`
- `/email/settings` ↔ `/email/smtp` duplicate (pick one)
- `/seo/404s` ↔ `/seo/404-monitor` duplicate (pick one)
- SEO stub pages (schema, meta, keywords) need full implementation
- SMTP credential storage redesign (currently module-global; needs a real KV/secret store)

### `09-system-DEFERRED.md`
- CSRF strategy (project-wide decision: double-submit cookie vs SameSite-only)
- Rust ACL mirroring (frontend role check + backend role check both needed; coordinate)
- Org-hierarchy proxies
- Photo upload endpoint
- HIBP integration for breach checks

---

## Stats

| Metric | Value |
|---|---|
| Audit reports written | 12 |
| Implementation passes | 12 + 2 cleanup |
| Issues identified | 200+ across all severities |
| Issues fixed | ~190 |
| Issues deferred (with documentation) | ~15 |
| New `+server.ts` proxies built | ~30+ |
| New stub pages built | 23 (21 CRM + 2 analytics empty-state CTAs) |
| Files modified (across all commits) | ~400+ |
| Final `pnpm check` | 0 errors / 0 warnings |
| Backend `cargo check` / `clippy` | 0 errors / 0 warnings |

---

## Verification Checklist

- [x] All 12 audit reports written
- [x] All 12 RESULTS docs written
- [x] All deferrals documented
- [x] `pnpm check` clean (0 errors, 0 warnings, 5261 files)
- [x] `cargo check` clean (Rust side)
- [x] `cargo clippy` clean (Rust side)
- [x] No files deleted
- [x] No commits/pushes performed (per user rule)
- [x] Privilege-escalation P0s closed
- [x] SMTP credential leak closed
- [x] XSS sinks closed
- [x] SSRF in webhooks closed
- [x] Bunny credentials never reach client
- [x] Sidebar coverage complete (0 dead links, sign-out verified)

---

## Notable Patterns Worth Keeping (from audits)

These came up as positive patterns the audits explicitly called out — preserve them:

- The `bestActiveHref` longest-prefix algorithm in `AdminSidebar.svelte:117-126` is correct.
- Three-layer sign-out defense (`api/logout/+server.ts:48-49` + `auth.ts:725-738` finally block + sidebar try/catch) — keep all three layers.
- The canonical proxy pattern (`env.API_BASE_URL || env.BACKEND_URL || 'https://...fly.dev'` + `requireAdmin(cookies)` + Bearer to backend).
- The CRM reactivity refactor (commit 34a0bd070's `$effect → onMount` migration) is the right pattern — propagate to remaining clusters.

---

*Generated 2026-04-26 by parallel principal-engineer agent run. Each cluster has its own RESULTS doc with per-issue file:line citations.*
