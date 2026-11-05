# Members workflow implementation checklist

Working source trace, November 5, 2026. These are confirmed contract and behavior gaps, followed by implementation and proof tasks. This document does not establish completion of Members, subscriptions, exports, or the broader admin move.

The statistics/service and operational UTC checkpoints passed on the verified 115 runtime, including four browser workflows and a separate nonzero recorded-money browser case. The 116 canonical profile, histories, notes and all-order export passed the fresh 116e backend proof and four page/drawer browser cases. Its downloaded 27-order CSV also passed independent comparison with the retained pre-cleanup source snapshot. Remaining account, tag-control, membership lifecycle and member-mail workflows are still open. Other tasks below remain open unless a specific proof is identified. Preserve the locally verified segment evidence in [the completion matrix](completion-matrix.md); this inventory does not supersede it.

## Verification boundary

- The existing member list uses `admin_member_facts` and the shared segment compiler. The completed segment/tag/filter evidence remains scoped to those previously tested workflows.
- Current export changes add real CSV, OOXML and PDF generation, a shared list/export filter selection, authenticated binary proxies, and strict download checks. The 112c endpoint proof passed; independent file checks found a PDF source-character defect. The repaired generator subsequently passed the fresh112d endpoint proof, independent file QA and three actual export browser cases in their stated scopes below.
- `frontend/tests/admin/member-exports.spec.ts` passed all three cases on the verified112d runtime in17.9seconds with no retries. Its fixtures use only disposable `.test` members and disable welcome mail; the repaired menu was exercised by pointer and keyboard activation.
- Local unit/lint/type checks cover source contracts, including export filter serialization. They do not establish provider acceptance, correct rendered files, database persistence, current money totals, or complete member workflows.
- Export proof establishes the actual downloaded fields and list/filter parity. It does not establish correctness or completeness of the outstanding billing/finance subsystem.
- No live customer account, external inbox, real payment, production membership or external email provider has been exercised by this trace.

### Export verification checkpoint

- **Actual 112c endpoints:** the fresh backend suite passed in 167.87 seconds with 104 migration versions (latest 112) (`/private/tmp/revolution-completion-fresh112c.log`). The export proof completed actual CSV/XLSX/PDF and legacy downloads, 1,105 selected rows, date/status selection, role rejection, shared list/filter/sort parity, download hashes and stored audit, explicit 10,000-row boundary failure, oversized-XLSX rejection and PDF continuation. The audit uses the existing `system` category with `context.operation = member_export`; its deliberately failed audit correctly prevented a download. This establishes those endpoint behaviors, not complete Members or billing correctness.
- **Independent 112c CSV/XLSX checks:** all 1,105 selected rows were compared with the retained source snapshot. CSV fields matched with the documented formula-escaping prefix; XLSX retained literal text/IDs without formulas, typed timestamps within 1 ms reader precision, full filter bounds, frozen header and repeated print header. All 10 XLSX previews were inspected, including first/middle/last, Unicode and the 255-character name. One artifact-tool timestamp preview differs at the microsecond day boundary despite the correct independently read cell value; exact microsecond visual display is not established.
- **Original PDF failure retained:** both pypdf and pdfplumber lost U+0649 and U+0627 from the Arabic source name. Generic CMap extraction also exposed Devanagari control/CID artifacts; ActualText-aware Poppler and PDFium retained that Indic text. All1105 IDs/order/email/role/date cells, the long-name text,62 page bounds/footers, explicit empty result and five-page wrapped record were separately checked. The original PDF remains failed for source-character loss. Evidence, source hashes and all 19 inspected previews are retained in `/private/tmp/revolution-member-export-qa-112c.lrtex15x/`, including `parser.log`, `unicode-diagnostic.json`, `per-row-diagnostic.json` and `scoped-qa-report.json`. The seven source files were unchanged by inspection.
- **Minimal repaired generator probe:** Parley ligature continuation ranges now merge in logical order before complete atoms reverse for RTL display. No glyphs, dependencies or fonts were replaced. Two isolated tests passed, including source-byte coverage for standalone Arabic ligatures, Arabic/Hebrew/Indic names and mixed scripts/digits; scoped all-target Clippy passed with warnings denied. The actual generator was included in an isolated fixture constructor using the retained 1,105-row snapshot. Its independent report verifies exact IDs/order/email/role/date, 1,103 names after documented whitespace normalization, exact source-character counts for each remaining RTL name, the 255-character long row and five-page continuation. Evidence: `/private/tmp/revolution-pdf-probe.jxq6ln9y/{unit2.log,clippy.log,qa/report.json}`. This is generator-probe evidence, not a replacement for the new API proof.
- **Reader and rendering boundary:** Poppler's ActualText-aware output is the primary semantic check; pypdf diagnostics remain visible. The two RTL names have complete source characters but different reader output ordering, so their extracted strings are explicitly not labelled exact logical names. Source-range coverage and source-aware rendered review corroborate the intended text. `qa/rtl-source-comparison.png` in the probe directory compares the actual Arabic/Hebrew PDF ink with those exact source names shaped independently by Skia using the same font files; visible forms/order match. These facts do not establish identical copying/ordering in every PDF reader or exact original whitespace bytes.
- **Actual repaired112d checkpoint:** full fresh backend proof passed in166.93seconds with104 migrations/latest112 (`/private/tmp/revolution-completion-fresh112d.log`). Independent parser/render QA of the new actual API artifacts passed its scoped checks; all20 selected PDF/XLSX/source-comparison images were inspected and all seven source-file hashes stayed unchanged. Evidence: `/private/tmp/revolution-member-export-qa-112d._jku0091/{report.json,visual-and-integrity-report.json,parser.log,xlsx-renderer.log}`. Exact CSV/XLSX row values remain separate from the two RTL PDF names' complete characters with reader-dependent output ordering and the generic-reader Indic diagnostics described above.
- **Actual export browser checkpoint:** all three `frontend/tests/admin/member-exports.spec.ts` cases passed in17.9seconds without retries (`/private/tmp/revolution-member-exports112d-browser4.log`). Actual nonempty CSV/XLSX/PDF and legacy downloads matched the server's SHA256 receipt; exact captured-body equality was checked whenever Chromium supplied that attachment body. Chromium returned an empty captured body for some real attachments, so those cases additionally asserted attachment disposition and exact downloaded row/filter/content evidence rather than treating an empty download as a pass. Artifacts and transport diagnostics remain in `/private/tmp/revolution-member-exports112d-browser4-artifacts/`. These facts do not verify subscription exports, processor revenue, actual recurring billing, LTV or historical churn; all broader Members checkboxes remain open.

## 1. Dashboard statistics

Established UI: total members, new members, active/trial subscribers, monthly revenue, average LTV, churn rate, growth bars and top-service navigation on `/admin/members`.

Current path:

`/admin/members` → `membersStore.loadStats()` → `membersApi.getStats()` → `/api/admin/members/stats` → `members.rs::stats`.

Boundary repairs established by the verified 115 checkpoint:

- `members.rs::stats` now delegates to `member_statistics.rs`; one statement reads current nondeleted `admin_member_facts` cohorts, separate current active membership-record counts, actual top-plan membership counts and12 months of recorded registration data. Storage failures propagate. The shared view's current entitlement semantics remain distinct from the open cross-surface access/grace task.
- `schema_version:1` carries evaluation time, UTC comparison boundaries and the explicit legacy timestamp-without-timezone basis. Registration growth excludes missing/future recorded dates and reports their counts. Month-to-date registrations are explicitly compared with the previous full month; zero denominator yields null with a reason.
- `revenue.recorded_net_usd_order_value` contains integer `net_cents`, `currency:'USD'`, qualifying order count, excluded non-USD/unknown-currency counts, `scope:'recorded_orders_less_recorded_refunds'` and limitations. It excludes unmaterialized renewal invoices, processor fees and settlement; recorded order currency has not been independently reconciled with a processor.
- Actual MRR, average LTV and historical churn rate remain null with source reasons. A separate monthly catalogue estimate uses nominal `catalog_price_unit`, currency null, mutable prices and explicit included/excluded active membership-record counts. Quarterly/annual interval encodings are normalized without double division; unsupported/ambiguous cycles are excluded. Catalogue units never receive a dollar label.
- Forward migration114 admits the actual handler's `partial_refund` spelling alongside legacy `partially_refunded`, preserving historical orders and immutable099. It advances affected spending/date segment definition versions. Migrations 114 and 115 are applied and immutable in the retained proof databases. The 115 forward correction uses explicit operational UTC for naive membership comparisons/defaults/new writes without rewriting ambiguous historical timestamps.
- `member-statistics.ts` checks identity/units/nulls/cohort totals; reads are network-only and bounded. Dedicated authenticated proxies preserve real error statuses. The store supplies independent statistics/catalog loading and error states, explicit recovery and stale-response guards. The main cards and secondary `/admin` consumer distinguish members from membership records.

Local source evidence:23 scoped tests across seven contract/state/proxy and preserved export/filter/loading suites passed in643ms (`/private/tmp/revolution-members114-units3.log`); scoped lint has0 errors/0 warnings (`/private/tmp/revolution-members114-lint2.log`); all six changed Svelte modules/components returned 0 MCP issues/suggestions. These checks were followed by the verified115 fresh database and browser evidence below.
Checklist:

- [x] Read consistent current, non-deleted member/membership facts and propagate database failures through the proxy and a dedicated UI error state.
- [x] Define one typed money contract. Convert cents once at a checked adapter, or render typed cents explicitly; reject missing or malformed fields instead of inventing zero.
- [x] Use recorded order/refund facts with explicit currency/status coverage for scoped net order value. Disclose incomplete processor coverage and reuse the corrected net USD spending definition where appropriate.
- [x] Correct both member-facts refund-status filters in a new migration and prove actual `'partial_refund'` records, legacy `'partially_refunded'` records, full refunds and over-refund clamping without changing historical orders.
- [x] Label plan-price-derived recurring totals as estimates, with their billing-cycle/current-membership scope. Do not present them as received revenue.
- [x] Define the period and cohort for churn. Return unavailable/null when the denominator or required evidence is absent. Distinguish recorded average spending from unmeasured lifetime value.
- [x] Supply actual top-service/growth data with clear scope, or show the specific unavailable state while that feature remains open.
- [x] Make Refresh fetch current records and recover from a statistics outage without retaining a successful zero or silently stale totals.

Proof: active/trial/expired/future/deleted/overlapping memberships; deleted users; paid and refunded USD orders alongside non-USD orders; zero and missing denominators; controlled database/proxy failure and recovery; exact UI units and labels.

## 2. Service identity, catalog and member drill-down

Established UI: Service filter on the main member directory; top-service cards; `/admin/members/service/[id]` search, membership-status filter, member selection, pagination, statistics and bulk email.

Current paths:

- Catalog: `membersApi.getServices()` → dedicated authenticated no-store proxy → `/api/admin/members/services` → `members.rs::services`.
- Main filter: the catalog's `membership_plans.id` → `handleServiceFilter()` → `plan_id` → `member_selection::MemberSelection` → `user_memberships.plan_id`; independent `product_id` still selects purchased products.
- Drill-down: `getMembersByService(id)` → `/api/admin/members/service/:id` → `members.rs::members_by_service`, where the ID actually means `user_memberships.plan_id`.

Boundary repairs established by the verified 115 checkpoint:

- The Service picker now sends `plan_id`; independently supplied `product_id` retains purchased-product semantics through list/export selection. Catalog/search paging does not change the member audience.
- `/members/services` returns all actual membership plans, including inactive/unused plans, with validated search/page/per_page, actual nullable activation state, nominal price units/currency null and complete pagination. `selected_id` resolves the selected plan independently of the visible catalog page; nonexistent selected plans return404.
- `/members/service/:id` returns404 for a missing plan. Its baseline audience is members with any nondeleted membership record for that plan, matching main-directory/export `plan_id`. Each member receives one plan-local status: current active, then current trial, otherwise the latest effective record; cancelled/expired/pending/past_due/paused/scheduled/other remain explicit. Global `Member.status` stays separate. A member can be globally active and expired in this selected plan.
- A repeatable-read transaction uses the same validated search/status audience for rows, counts, cohort statistics, whole-member recorded net USD order value and pagination. The order value spans those members' orders across all products; it is explicitly not allocated service revenue.
- The store retains service filters across page calls and guards reused-route races. The page persists search/status/page/page-size in the URL, handles parameter reuse and reload, and exposes storage/malformed-success errors with an explicit retry instead of an empty successful list. Existing selection/detail/mail controls remain present; the separate member-mail workflow below remains open.

Verified runtime evidence: the complete fresh 115c backend suite passed in 175.19 seconds with 107 migration versions, latest 115 (`/private/tmp/revolution-completion-fresh115c.log`). `member_statistics::proof` covers current/deleted/future/overlap fixtures, recorded/null money, refunds/currencies, catalog/local/global cohort identity, search/paging, missing-plan/role rejection and a real view outage/recovery. `member_operational_utc::proof` uses the actual API grant without a supplied start under an America/New_York pool and checks timezone-independent current facts/catalog/cohort agreement, default writes, future/expired/trial/deleted/overlap boundaries and unchanged historical timestamp bytes.

Actual browser evidence: all four `frontend/tests/admin/member-statistics.spec.ts` cases passed in 13.7 seconds, workers 1/retries 0 (`/private/tmp/revolution-member-statistics115-browser3.log`). This includes a 62-plan catalog, off-page inactive selection, filtered CSV with actual hash receipt, current Refresh 3→4→3 network reads, URL paging/reload, local-expired/global-active distinction, malformed200/503/404 recovery and 401/403. `member-statistics-money.spec.ts` separately passed in 4.1 seconds with actual $123.45 DOM output from 10000+2345 recorded USD cents and excluded EUR/unknown-currency orders (`/private/tmp/revolution-member-money115-browser3.log`). This is recorded-order evidence, not processor revenue. The guarded synthetic order rows were physically removed and the money baseline restored; existing account DELETE only anonymized/disabled its fixture user and retained the account row. Account-deletion semantics remain open.

Checklist:

- [x] Make the existing Service control explicitly select `plan_id`. Preserve the separate purchased-product meaning of `product_id`; do not remap those IDs by coincidence.
- [x] Return a searched/paged catalog with explicit identity, currency/price units and complete selectable scope. Preserve selections outside the current catalog page.
- [x] Reuse current member facts and one validated selection for drill-down rows, counts and pagination. Define whether status is global member status or membership status within the selected plan, and label it accordingly.
- [x] Return404 for a missing/deleted plan and explicit errors for invalid filters. Propagate storage failures.
- [x] Preserve search, status and page size across page navigation, URL reloads and refresh; expose per-panel failures.

Proof: deliberately colliding plan/product IDs; more than50 plans and multiple result pages; active and historical memberships in the same plan; filtered totals and overlapping statuses; missing-plan404; malformed-success/error recovery; main filter, export and drill-down audience agreement.

The `/admin/members/churned` branch also calls `membersApi.getChurnedMembers()` → generic proxy → `members.rs::churned_members`. Its displayed win-back-potential and days-since-churn controls send `winback_potential` and `churned_within_days`, which `MemberQuery` does not deserialize/use. The backend returns basic user rows and `lost_revenue_cents`/hardcoded `avg_lifetime_days`, while the client expects rich member fields, `potential_recovery_revenue` and `top_churn_reasons`. Its membership/count queries use the older status-only rules and failed aggregates become zero.

- [ ] Give the churned branch checked current/former-member facts, actual filter semantics, consistent money/history scope and visible errors. Do not invent a recovery probability or reasons to populate the existing controls.
- [ ] Prove both churned filters actually change the selected audience and survive paging/reload; cover reactivated members, expiry/date boundaries and failed reads.

## 3. Full member profile and detail drawer

Established UI: `/admin/members/[id]` and `MemberDetailDrawer.svelte`, including overview, spending, subscriptions, orders, account state, timeline, notes and moderation actions.

Current paths:

- Full page: `MemberProfileView.svelte` → strict `memberProfileApi.get()` → exact authenticated no-store profile proxy → canonical `member_profile` v1. The legacy `membersApi.getMember(id)` delegates to the same checked adapter through the `/members/:id` alias.
- Drawer: the same shared controller and checked DTO inside a native dialog; `membersApi.getMemberFull(id)` delegates to the canonical `/member-management/:id` API. Both route families use the same profile/history/note implementation.

Original defects addressed in 116 source; verified scopes are recorded below:

- `members::show` returns a basic user, top-level subscriptions and a small stats object. The full page assumes rich `Member`, `engagement_score` and `timeline` fields, then substitutes `0`/`[]` for absent facts. It does not attach the actual top-level subscriptions to `member.subscriptions`.
- The richer endpoint returns `stats.total_spent_cents`, subscription `price_cents` and order `total_cents`. Drawer types/renderers instead read `total_spent`, `price` and `total` as dollars, so real amounts can display as zero.
- The richer spending query masks failure and sums gross completed orders without the net/refund/USD scope of the existing member-facts view. Active-subscription counts only inspect the status string.
- Rich `member.status` is taken from account moderation state or defaults to `active`, conflating account restrictions with subscription status. Its member query does not exclude soft-deleted users.
- The backend engagement formula is a hardcoded mix of subscription counts, recent activity and order counts. The full page displays it as an engagement percentage, and the basic endpoint does not even supply that value.
- Orders are capped at50, activity at20, and the constructed timeline at20 with no coverage or pagination contract. `stats.total_orders` is only the capped returned order count. The full page initializes only on mount, so reused route parameters and stale replies need explicit handling.

Current 116 source checkpoint: `member-profile.ts` validates member/row identity, exact paged row counts, source units/currency, account versus membership status, nullable unmeasured facts and raw legacy/qualified timestamp bases. Each history has independent loading/error/recovery and stale-ID cancellation. Notes use saved physical IDs/content/creator and explicit idempotent deletion receipts; an uncertain mutation retains its draft and requires a successful saved-history read before another explicit attempt. No automatic mutation retry is added. The fresh 116a backend suite passed in 176.98 seconds (`/private/tmp/revolution-completion-fresh116a.log`). Final parent check 4 passed 0 errors/0 warnings; full frontend units passed 3070 with 32 pre-existing skips (`/private/tmp/revolution-member116-{parent-check4,frontend-units1}.log`); all eight parent Svelte components/modules returned 0 MCP issues/suggestions. Actual page/drawer evidence is now recorded below. Account edits/moderation and membership lifecycle completion remain separate work.

Actual 116e checkpoint: the complete fresh backend suite passed in 192.46 seconds (`/private/tmp/revolution-completion-fresh116e.log`), with candidate SHA-256 `ccb6519f788e91ca3393b670e376e442cb39b46e0cd4634bc3559db83675dea0`. Four actual browser cases passed in 18.7 seconds, workers 1/retries 0 (`/private/tmp/revolution-member116e-browser3.log`). They establish factual cohorts/account separation, unknown money/engagement/MFA facts, every independently paged history, actual note creator and physical-ID delete/reload, a persisted write with a deliberately malformed acknowledgment and no automatic resubmit, narrow dialog reachability, 503/malformed-read recovery, a held real old-member response that cannot overwrite the newly opened drawer, and both API families' 401/403 boundary. The passing browser CLI log includes a `NO_COLOR`/`FORCE_COLOR` environment warning; browser console diagnostics and launcher warnings remain distinct.

The new all-member-order CSV is exported while only 10 order rows are visible. An independent Python `csv`/`Decimal` reader checked all 27 retained SQL source rows, actual member/order IDs, source sort order, statuses, exact recorded amounts/refunds/currencies, NULL and year-270000 wall-clock dates, formula prefix, twelve columns, BOM/CRLF, private receipt scope and SHA-256. The 4,632-byte download hash is `b30b8f73394fdda4a35d06335d3bdb8521214020b77b152f9d214237e2dca1f3`. The CSV, source snapshot and receipt were unchanged by QA; evidence is retained at `/private/tmp/revolution-member116e-qa-profile-38d8325d12cd4490aa88/{member-orders.csv,orders-source.json,receipt.json,independent-report.json}` and `/private/tmp/revolution-member116e-order-independent-qa.log`. These are recorded-order facts, without a processor settlement claim. The browser fixture cleanup asserted its captured orders, memberships, email logs and custom plan removed; account cleanup retained anonymized/disabled user records under the existing open deletion workflow.

The browser also exposed a real route collision: a broad new `/api/admin/members/[id]` GET consumed named `email-templates` and `churned` APIs. The repaired Kit 3 `src/params.ts` limits only the new profile GET to canonical positive i64 identities. Existing unmatched history children and named APIs retain their routes. Default check 5 passed with 0 errors/0 warnings; thirteen checks against the installed Kit manifest scanner/sorter/matcher passed (`/private/tmp/revolution-member116-route-manifest-probe.log`). The main Members page's established template-loading behavior was preserved.

Checklist:

- [ ] Reuse one canonical rich member DTO/adapter for page and drawer, with checked identity and nullable unknown facts. Keep account moderation and membership status separate.
- [ ] Preserve actual money units, currency/refund scope, current-membership rules, recorded timestamps and missing values.
- [ ] Expose the source/meaning of any calculated indicator; do not imply measured human engagement or invent a percentage for absent evidence.
- [ ] Add explicit history coverage and pagination, or accurately label a bounded recent-history summary with access to the full history.
- [ ] Render independent loading/error/empty states and prevent stale navigation responses from replacing the current member.
- [ ] Preserve existing edit, ban, suspend, unban and delete controls, requiring exact acknowledged identity and refreshed persisted state.
- [ ] Wire the full-page order Export and View icons, which currently have no handlers, to actual existing order workflows.

Proof: two independently linked members, soft deletion, known cents values, refunds/currencies, expired memberships, missing facts, more than50 orders/20 activity records, rapid member navigation, malformed200, authorization, moderation persistence and reload.

## 4. Notes, activity and email history

These aliases are mounted; they are not missing routes:

- `/api/admin/members/:id/notes` proxy → `admin_members/notes_emails.rs::{get_member_notes,create_member_note}`.
- `/api/admin/members/:id/emails` proxy → `admin_members/notes_emails.rs::get_member_emails`.
- `/api/admin/member-management/:id/{notes,activity}` → the existing management note/activity handlers used by the drawer.

Original defects addressed in 116 source; verified scopes are recorded below:

- Legacy POST notes returns only `{message}`. Full-page `addNote()` inserts that response as a Note and shows success without a saved note ID/content. The page expects an `author` field unlike the returned author fields, and its note-delete button has no handler.
- Failed/malformed full-page notes and email reads become `[]`, indistinguishable from a successful empty history.
- Management notes already return a real `{note}` on create and provide a delete endpoint. Drawer mutations nevertheless trust successful status/shape without checking the selected member/note identity.
- `admin_member_management/activity.rs::get_activity` filters rows by `action` but counts all actions for pagination. Empty metadata can report zero last pages. Notes/email history is unpaged or silently bounded.
- Email log status/open/click observations need their actual provider/receipt scope; they do not establish human engagement.

Checklist:

- [ ] Reuse the existing management note storage/handlers and make both aliases return/parse a consistent saved-note contract.
- [ ] Validate bounded content, existing non-deleted member identity, creator identity and atomic audit/persistence for mutations.
- [ ] Wire delete to the actual member/note pair, accept explicit idempotent outcomes, and reload confirmed state.
- [ ] Match paged counts to the exact activity/history filters; distinguish unsuccessful reads from empty results.
- [ ] Preserve actual timestamps and provider outcome/observation semantics, with coverage for truncated history.

Proof: note create/delete/reload through both entry points; cross-member IDs and roles; controlled audit/storage failure; malformed successful mutation; activity action/count parity across pages; empty/error/recovery and more than50 email records.

## 5. Member tags

Established UI: profile Tag modal, add/remove/custom tag controls and displayed member tags.

Original gap: the full page hardcoded six suggested labels and mutated local strings. The 116 controller now reads the searched/paged physical member-tag catalog, creates actual custom records, applies/removes audited assignments, validates receipts and checks refreshed profile state. Browser persistence, idempotency and failure-recovery proof for these newly wired profile controls is still pending; the earlier separate segment/tag evidence is preserved.

Existing infrastructure to reuse:

- `admin_members/tags.rs` and the mounted `/api/admin/members/tags` catalog/proxies.
- POST `tags/assign` with `{user_id,tag_id}` returns actual `assigned`, `already_assigned`, `requested` and tag rows.
- DELETE `tags/unassign` with the same identities returns `{removed,user_id,tag_id}`.
- These assignment operations already validate records and commit their audit with persistence. They use `member_tags` / `user_member_tags`, a separate namespace from CRM contact tags.

Checklist:

- [ ] Replace hardcoded choices with actual searched/paged member-tag catalog identities.
- [ ] Reuse the audited assignment/unassignment endpoints and validate their identities/counts before reporting success.
- [ ] Resolve displayed names through actual catalog records; do not guess CRM tag IDs or invent a new catalog entry merely to make a local label persist.
- [ ] Verify repeat assignments/removals, failure recovery and reload while preserving existing validated tag/segment behavior.

## 6. Membership grant, extend and revoke

Established UI: profile subscription history, Grant Membership, Extend and Revoke dialogs; available-plan picker.

Existing path: `/api/admin/membership-plans` and `/api/admin/user-memberships[/:id]` dedicated proxies → `api/src/routes/admin/memberships.rs` mounted handlers. These are existing writes, not absent infrastructure.

Confirmed gaps:

- The basic profile DTO is not mapped into the subscription shape consumed by `SubscriptionsTab.svelte`, so saved subscriptions can appear absent.
- The tab expects product/plan/price/interval/start/next-payment/total-paid fields unlike the backend rows. It substitutes `Standard`, `$0` or unavailable dates. A membership expiry must not be presented as an unrecorded next payment.
- Extend derives its starting date from `next_payment`, otherwise the current client time, rather than the mapped actual expiry. Successful grant/extend/revoke handling only checks HTTP status.
- Proxy/API roles for these privilege-bearing writes are not identical: relevant proxies require a global super-admin while the Rust handlers use `AdminUser`. This needs an approved policy and direct-API tests.

Checklist:

- [ ] Map actual membership and plan identities, recorded start/expiry/cancellation and explicitly typed price/currency fields. Leave unrecorded paid totals/next-charge data unknown.
- [ ] Preserve the existing plan picker, grant, extension and revoke features with validated bounds and exact member/plan/membership acknowledgments.
- [ ] Base extensions on the actual reviewed expiry and prevent stale/concurrent edits from silently producing a different result.
- [ ] Align proxy and direct-API authorization and make persistence/audit/rollback behavior explicit.
- [ ] Reconcile actual member access across the facts view, public room content and videos. The existing `room_content::ensure_room_access` accepts any status-only active/trial row without checking its room, deletion, start, expiry or trial end. The newer video gate checks room identity and dates but also imposes `current_period_end`; manual extensions and recorded payment-failure grace therefore need one documented, shared entitlement rule and real cross-surface proof.
- [ ] Prove the recorded `past_due` grace period has the access behavior promised by the actual payment-failure flow, including expiry, successful recovery and explicit administrative revocation. A subscription UI's grace flag alone is not access evidence.
- [ ] Replace the separate four-name course-admin helper with the canonical role policy, including the existing `administrator` and `superadmin` aliases. Keep ordinary enrolled/free/preview course access independent of staff roles.

Proof: grant→reload→extend→reload→revoke→reload; expired/open-ended memberships; invalid dates/status/IDs; concurrent edit; approved staff-role matrix; selected plan/member identity; failure and malformed-success recovery.

## 7. Member mail and welcome-mail controls

Established UI: single-member Send Email; selected-member email from the main, service and churned lists; template/preset selection; create-member welcome-mail checkbox.

Current mismatches:

- `membersApi.sendEmail()` calls POST `/api/admin/members/:id/send-email`; `sendBulkEmail()` calls POST `/api/admin/members/bulk-email`. Neither member route is mounted in the Rust router. The existing newsletter bulk-send and CRM contact personal-mail endpoints are different workflows.
- The member template endpoint returns preset metadata separately from an empty saved-template list. Native saved templates already have their own canonical catalog.
- `CreateMemberRequest` accepts `send_welcome_email` and the form exposes the checkbox, but `create_member` has no welcome-mail submission using it.
- Existing success handling trusts response status/message. It lacks a frozen intent, exact reviewed audience, actual provider receipts and an explicit uncertain outcome.

Checklist:

- [ ] Define single-member transactional mail separately from selected/audience marketing mail, including consent and existing member identity. Never create a guessed contact just to borrow a contact-mail endpoint.
- [ ] Reuse the existing transactional renderer, native template catalog, email logs and durable submission/receipt machinery; preserve exact actor/recipient/source intent and prevent automatic resend of an uncertain submission.
- [ ] Bind selected member IDs or the explicitly reviewed audience to any bulk intent, including current eligibility/consent checks. Do not silently broaden a selected audience.
- [ ] Implement the actual welcome-mail control or surface its unavailable outcome without claiming delivery.
- [ ] Show accepted/rejected/processing/unknown/unavailable provider outcomes, persist history, and distinguish observed opens/clicks from human engagement.

Proof: recipient-restricted local SMTP acceptance/rejection; chosen native template/creative; identical-intent replay without another send; changed-intent conflict; exact selected audience and consent withdrawal; actor roles; outage/unknown reconciliation; reload and historical receipt retention. No live-provider or customer-mail result is implied.

## Additional branch: past-members dashboard

`frontend/src/routes/admin/members/past/+page.svelte` → `past-members-dashboard.ts` → `/api/admin/past-members-dashboard/[...path]` proxy → the same Rust namespace. The proxy exists and propagates failures, but no corresponding Rust router/namespace was found in this trace.

Established controls include period/search/page selection, overview/churn reasons/services/campaign history/realtime reads, refresh/invalidate, win-back templates/custom subject/body/offer/discount, and surveys/incentives.

- [ ] Trace and implement each advertised endpoint before treating the shell as a functioning dashboard.
- [ ] Preserve selected IDs: current `handleSendBulkWinBack()` clears the selected set but sends only period/template/options, not selected IDs. Any future submission must use an explicit reviewed audience.
- [ ] Replace member-fetch failures that only log with a visible panel error and preserve filter/page state.
- [ ] Reuse current member facts, membership history, audience/consent compiler and durable mail receipts. Do not fabricate recovery revenue, churn reasons or successful surveys.
- [ ] Prove period boundaries, actual former-member eligibility, pagination, failed reads/recovery, exact selected audience, real local SMTP outcomes, reload, roles and no duplicate submissions.

## Additional branch: member subscriptions

`/admin/members/subscriptions` → `subscriptions.ts::getSubscriptions(...,true)` → GET `/api/admin/subscriptions`, which is mounted. The client expects `EnhancedSubscription` and embedded customer/billing/health fields; the backend returns the native paged user-membership DTO. This shape needs a dedicated checked mapping, not assertions or synthesized metrics.

Pause/resume/cancel currently call `updateSubscription()` → PATCH `/api/subscriptions/:id`. The existing admin lifecycle routes are POST `/api/admin/subscriptions/:id/{pause,resume,cancel}`. The public subscription router does not provide that PATCH endpoint.

- [ ] Align the existing list/search/status/interval/count/pagination controls with the actual admin subscription DTO and preserve numeric/string identity safely.
- [ ] Wire lifecycle actions to the real admin handlers with exact acknowledgment and current-state reload; preserve the actual provider/billing implications and role policy.
- [ ] Trace export at `/api/subscriptions/export` separately. The new member export proof does not verify subscription export or payment lifecycle behavior.
- [ ] Prove filtered/paged list, pause/resume/cancel persistence, invalid/malformed responses, roles, fresh state and actual exported subscription bytes before changing its matrix status.

## Infrastructure and evidence to reuse

| Existing source                                                                       | Reuse / boundary                                                                                                                                                                                                                                                                                                |
| ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api/migrations/099_member_current_memberships.sql`                                   | Current non-deleted membership/status, tags and net USD spending facts. Preserve immutable applied migrations; new changes need a forward migration if required.                                                                                                                                                |
| `api/src/services/member_segmentation.rs`                                             | Validated member/segment/audience predicates and existing configuration evidence.                                                                                                                                                                                                                               |
| `api/src/routes/member_selection.rs`                                                  | Shared list/export selection, including segment/tag/search/status/product/plan/date/spending/sort. Preserve list/export parity.                                                                                                                                                                                 |
| `api/src/routes/admin_members/{segments,tags,filters}.rs`                             | Existing persisted segment/tag/filter workflows and atomic audits; do not replace them with a second namespace.                                                                                                                                                                                                 |
| `api/src/routes/admin_member_management/{crud,notes,activity,moderation}.rs`          | Existing full-detail, CRUD, notes, activity and account-action infrastructure requiring contract/metric alignment.                                                                                                                                                                                              |
| `api/src/routes/admin/memberships.rs`, `subscriptions_admin/`                         | Existing membership/admin subscription handlers; preserve distinctions between manual membership state and provider billing operations.                                                                                                                                                                         |
| Native email templates, transactional renderer, `email_logs`, outbox/receipt services | Existing saved creative, delivery/provider evidence and idempotency patterns; do not equate HTTP success with delivery.                                                                                                                                                                                         |
| `api/tests/support/member_exports.rs`                                                 | Actual112d endpoint proof passed after the PDF range repair; preserve the scoped artifact/reader limits and retained112c failure above.                                                                                                                                                                         |
| `frontend/tests/admin/member-exports.spec.ts`                                         | Three prepared export browser cases: real CSV/XLSX/PDF and legacy downloads/filter/reload; malformed200/422/503 recovery without false success; anonymous/ordinary-user rejection. All three passed on verified112d without retries; see the exact transport/attachment scope above.                            |
| `api/scripts/verify-member-exports.py`                                                | Independent CSV/openpyxl and ActualText-aware Poppler row checks, retained generic-reader diagnostics and PDF rendering. The 112c run exposed a PDF defect; the repaired probe passed its stated scopes. Actual112d API-artifact QA passed its stated scopes; preserve the reader and timestamp-preview limits. |
| `api/scripts/render-member-export.mjs`                                                | Read-only artifact-tool XLSX previews; all 10 actual 112c previews inspected without changing the source workbook. Preserve the timestamp preview precision limitation and actual112d previews were also inspected with source hashes unchanged.                                                                |

## Proof recording checklist

- [ ] Record the compiled artifact/source checkpoint, fresh schema/migration summary, isolated database/Redis/local-provider identities and actual commands/log paths.
- [ ] Run backend proofs before browser suites; stagger fixture logins and local SMTP relay ownership. Reset only an identified owned fixture if rate limits require it.
- [ ] Independently read downloaded files, compare all selected rows with the retained database snapshot, verify CSV escaping/OOXML literal and date types/actual PDF pages, then inspect first/middle/last, Unicode, empty and wrapped-record renders.
- [ ] Run each existing action through UI→client→proxy→authorization→storage/provider→UI, including reload, real errors, malformed200, role rejection and recovery.
- [ ] Add scoped regression tests for confirmed defects and run the required zero-warning gates at the final source checkpoint. Exclude failed attempts from passing evidence.
- [ ] Update only the matrix rows supported by new evidence. Leave Members analytics and the other membership/subscription administration pages unverified until their separate traces/proofs run. Preserve completed segment evidence and keep every remaining workflow visible.
