# 06 — CRM Cluster — Deferred / Backend-Pending

**Date:** 2026-04-26
**Companion to:** [`06-crm.md`](./06-crm.md), [`06-crm-RESULTS.md`](./06-crm-RESULTS.md)

This document enumerates frontend stubs that were created to fix the
"orphan-link epidemic" (audit P0 #1) but whose underlying backend
endpoints either do not exist or were not verified to exist during this
remediation pass.

Each route below renders a "Backend integration pending" placeholder
with a working back-link, so navigation no longer 404s. Wiring the form
to a real backend is left to the team owning that endpoint.

---

## Stubs awaiting backend wiring

| Route | Backend endpoint expected | Status |
|---|---|---|
| `/admin/crm/templates/new` | `POST /api/admin/crm/email-templates` | Endpoint may exist (see `crmAPI.createEmailTemplate`) but no UI form has been built |
| `/admin/crm/templates/[id]/edit` | `PATCH /api/admin/crm/email-templates/:id` | Endpoint may exist (`crmAPI.updateEmailTemplate`) but no UI form has been built |
| `/admin/crm/companies/new` | `POST /api/admin/crm/companies` | Backend pending verification |
| `/admin/crm/companies/[id]/edit` | `PATCH /api/admin/crm/companies/:id` | Backend pending verification |
| `/admin/crm/sequences/new` | `POST /api/admin/crm/sequences` | Endpoint exists (`crmAPI.createSequence`); UI not built |
| `/admin/crm/sequences/[id]/edit` | `PATCH /api/admin/crm/sequences/:id` | Endpoint exists (`crmAPI.updateSequence`); UI not built |
| `/admin/crm/lists/new` | `POST /api/admin/crm/contact-lists` | Endpoint exists; UI not built |
| `/admin/crm/lists/[id]/edit` | `PATCH /api/admin/crm/contact-lists/:id` | Endpoint exists; UI not built |
| `/admin/crm/segments/new` | `POST /api/admin/crm/segments` | Endpoint exists; UI not built |
| `/admin/crm/segments/[id]/edit` | `PATCH /api/admin/crm/segments/:id` | Endpoint exists; UI not built |
| `/admin/crm/campaigns/new` | `POST /api/admin/crm/campaigns` | Endpoint exists; UI not built |
| `/admin/crm/campaigns/[id]/edit` | `PATCH /api/admin/crm/campaigns/:id` | Endpoint exists; UI not built |
| `/admin/crm/recurring-campaigns/new` | `POST /api/admin/crm/recurring-campaigns` | Backend pending verification |
| `/admin/crm/recurring-campaigns/[id]/edit` | `PATCH /api/admin/crm/recurring-campaigns/:id` | Backend pending verification |
| `/admin/crm/deals/new` | `POST /api/admin/crm/deals` | Endpoint exists (`crmAPI.createDeal`); UI not built |
| `/admin/crm/deals/[id]/edit` | `PATCH /api/admin/crm/deals/:id` | Endpoint exists (`crmAPI.updateDeal`); dedicated edit form not built (deals/[id] is detail view) |
| `/admin/crm/managers/new` | `POST /api/admin/crm/managers` | Backend pending verification |
| `/admin/crm/managers/[id]/edit` | `PATCH /api/admin/crm/managers/:id` | Backend pending verification |
| `/admin/crm/import-export/new` | n/a — multi-step wizard | Wizard UI not built |
| `/admin/crm/contacts/[id]/edit` | `PATCH /api/admin/crm/contacts/:id` | Endpoint exists; dedicated edit form not built (contacts/[id] is detail view) |
| `/admin/crm/leads/[id]/edit` | `PATCH /api/admin/crm/leads/:id` | Endpoint exists; dedicated edit form not built (leads/[id] is detail view) |

## Backend endpoints to add or verify

The following backend endpoints are referenced by the frontend code but
were not verified to be implemented in `api/src/routes/`:

- `POST /api/admin/crm/sequences/:id/test` — used by `sequences/+page.svelte`
  `sendTestEmail()` after the audit P1 #4 fix replaced the fake
  `setTimeout` success. If this endpoint is missing the frontend now
  surfaces an honest error rather than faking success.

## Note on "edit" routes that overlap detail pages

For `contacts/[id]`, `leads/[id]`, and `deals/[id]`, the existing detail
pages already include inline editing in some sections. The audit asked
for separate `/edit` routes because the detail pages link to them in
"Edit X" buttons. The stubs were built per the audit. A future cleanup
could either (a) build a dedicated edit form per resource, or (b) repoint
those "Edit X" buttons back to the detail page if inline edit is the
intended flow.
