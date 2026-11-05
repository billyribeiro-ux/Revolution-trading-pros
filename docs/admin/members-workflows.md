# Members workflow implementation checklist

Working source trace, November 5, 2026. These are confirmed contract and behavior gaps, followed by implementation and proof tasks. This document does not establish completion of Members, subscriptions, exports, or the broader admin move.

Application source is frozen while the new Rust artifact and disposable runtime are prepared. The tasks below are planned work unless a specific existing proof is identified. Preserve the locally verified segment evidence in [the completion matrix](completion-matrix.md); this inventory does not supersede it.

## Verification boundary

- The existing member list uses `admin_member_facts` and the shared segment compiler. The completed segment/tag/filter evidence remains scoped to those previously tested workflows.
- Current export changes add real CSV, OOXML and PDF generation, a shared list/export filter selection, authenticated binary proxies, and strict download checks. The 112c endpoint proof passed; independent file checks found a PDF source-character defect. The repaired generator has scoped probe evidence below, while a new full fresh proof, its artifact QA and the three export browser cases remain pending.
- `frontend/tests/admin/member-exports.spec.ts` contains three prepared cases and has been collected with Playwright `--list`. Collection is not execution. Its fixtures use only disposable `.test` members and disable welcome mail.
- Local unit/lint/type checks cover source contracts, including export filter serialization. They do not establish provider acceptance, correct rendered files, database persistence, current money totals, or complete member workflows.
- Export proof establishes the actual downloaded fields and list/filter parity. It does not establish correctness or completeness of the outstanding billing/finance subsystem.
- No live customer account, external inbox, real payment, production membership or external email provider has been exercised by this trace.

### Export verification checkpoint

- **Actual 112c endpoints:** the fresh backend suite passed in 167.87 seconds with 104 migration versions (latest 112) (`/private/tmp/revolution-completion-fresh112c.log`). The export proof completed actual CSV/XLSX/PDF and legacy downloads, 1,105 selected rows, date/status selection, role rejection, shared list/filter/sort parity, download hashes and stored audit, explicit 10,000-row boundary failure, oversized-XLSX rejection and PDF continuation. The audit uses the existing `system` category with `context.operation = member_export`; its deliberately failed audit correctly prevented a download. This establishes those endpoint behaviors, not complete Members or billing correctness.
- **Independent 112c CSV/XLSX checks:** all 1,105 selected rows were compared with the retained source snapshot. CSV fields matched with the documented formula-escaping prefix; XLSX retained literal text/IDs without formulas, typed timestamps within 1 ms reader precision, full filter bounds, frozen header and repeated print header. All 10 XLSX previews were inspected, including first/middle/last, Unicode and the 255-character name. One artifact-tool timestamp preview differs at the microsecond day boundary despite the correct independently read cell value; exact microsecond visual display is not established.
- **Original PDF failure retained:** both pypdf and pdfplumber lost U+0649 and U+0627 from the Arabic source name. Generic CMap extraction also exposed Devanagari control/CID artifacts; ActualText-aware Poppler and PDFium retained that Indic text. All1105 IDs/order/email/role/date cells, the long-name text,62 page bounds/footers, explicit empty result and five-page wrapped record were separately checked. The original PDF remains failed for source-character loss. Evidence, source hashes and all 19 inspected previews are retained in `/private/tmp/revolution-member-export-qa-112c.lrtex15x/`, including `parser.log`, `unicode-diagnostic.json`, `per-row-diagnostic.json` and `scoped-qa-report.json`. The seven source files were unchanged by inspection.
- **Minimal repaired generator probe:** Parley ligature continuation ranges now merge in logical order before complete atoms reverse for RTL display. No glyphs, dependencies or fonts were replaced. Two isolated tests passed, including source-byte coverage for standalone Arabic ligatures, Arabic/Hebrew/Indic names and mixed scripts/digits; scoped all-target Clippy passed with warnings denied. The actual generator was included in an isolated fixture constructor using the retained 1,105-row snapshot. Its independent report verifies exact IDs/order/email/role/date, 1,103 names after documented whitespace normalization, exact source-character counts for each remaining RTL name, the 255-character long row and five-page continuation. Evidence: `/private/tmp/revolution-pdf-probe.jxq6ln9y/{unit2.log,clippy.log,qa/report.json}`. This is generator-probe evidence, not a replacement for the new API proof.
- **Reader and rendering boundary:** Poppler's ActualText-aware output is the primary semantic check; pypdf diagnostics remain visible. The two RTL names have complete source characters but different reader output ordering, so their extracted strings are explicitly not labelled exact logical names. Source-range coverage and source-aware rendered review corroborate the intended text. `qa/rtl-source-comparison.png` in the probe directory compares the actual Arabic/Hebrew PDF ink with those exact source names shaped independently by Skia using the same font files; visible forms/order match. These facts do not establish identical copying/ordering in every PDF reader or exact original whitespace bytes.
- **Still pending after the repair:** full Rust gates and the new fresh API proof, independent checks/render review of that new run's actual artifacts, and execution of all three cases in `frontend/tests/admin/member-exports.spec.ts`. Keep every other Members workflow checkbox open until its own complete evidence exists. Exports do not verify subscription exports, processor revenue, actual recurring billing, LTV or historical churn.

## 1. Dashboard statistics

Established UI: total members, new members, active/trial subscribers, monthly revenue, average LTV, churn rate, growth bars and top-service navigation on `/admin/members`.

Current path:

`/admin/members` → `membersStore.loadStats()` → `membersApi.getStats()` → `/api/admin/members/stats` → `members.rs::stats`.

Confirmed gaps:

- `api/src/routes/members.rs::stats` converts failed database reads into `0` with `unwrap_or(0)`. It counts deleted users and membership statuses without the current-start/expiry/deletion rules already used by `admin_member_facts`.
- Its `mrr_cents` is derived from plan prices; `total_cents` sums plan prices across membership rows. That sum is not recorded payment revenue. `churn_rate` and `avg_ltv_cents` are hardcoded zero.
- The actual Stripe refund handler, `payments/webhook_refund.rs::handle_refund`, stores partial refunds as `orders.status = 'partial_refund'`. The applied `099_member_current_memberships.sql` spending and last-order filters admit `'partially_refunded'` instead, omitting those genuine partial-refund records. Correct the facts in a new forward migration; preserve immutable099 and existing order history rather than silently rewriting statuses.
- Rust returns `revenue.{mrr_cents,total_cents,avg_ltv_cents}`. `frontend/src/lib/api/members.ts::MemberStats` and `_components/StatsGrid.svelte` read `revenue.{mrr,total,avg_ltv}` in dollars. Missing properties become visible `$0` through fallbacks.
- `frontend/src/routes/api/admin/members/stats/+server.ts` returns HTTP200 with all-zero dollar-shaped statistics after any upstream/network failure. This creates a second false-success boundary.
- `membersApi.getStats()` requests a five-minute cache. `membersStore.loadStats()` only logs failures; there is no independent statistics error state. Rust does not provide the `top_services` and `growth_trend` arrays expected by the UI.

Checklist:

- [ ] Read consistent current, non-deleted member/membership facts and propagate database failures through the proxy and a dedicated UI error state.
- [ ] Define one typed money contract. Convert cents once at a checked adapter, or render typed cents explicitly; reject missing or malformed fields instead of inventing zero.
- [ ] Use recorded order/payment/refund facts with an explicit currency and status scope for received revenue. Reuse the existing net USD spending definition where appropriate.
- [ ] Correct both member-facts refund-status filters in a new migration and prove actual `'partial_refund'` records, legacy `'partially_refunded'` records, full refunds and over-refund clamping without changing historical orders.
- [ ] Label plan-price-derived recurring totals as estimates, with their billing-cycle/current-membership scope. Do not present them as received revenue.
- [ ] Define the period and cohort for churn. Return unavailable/null when the denominator or required evidence is absent. Distinguish recorded average spending from unmeasured lifetime value.
- [ ] Supply actual top-service/growth data with clear scope, or show the specific unavailable state while that feature remains open.
- [ ] Make Refresh fetch current records and recover from a statistics outage without retaining a successful zero or silently stale totals.

Proof: active/trial/expired/future/deleted/overlapping memberships; deleted users; paid and refunded USD orders alongside non-USD orders; zero and missing denominators; controlled database/proxy failure and recovery; exact UI units and labels.

## 2. Service identity, catalog and member drill-down

Established UI: Service filter on the main member directory; top-service cards; `/admin/members/service/[id]` search, membership-status filter, member selection, pagination, statistics and bulk email.

Current paths:

- Catalog: `membersApi.getServices()` → generic authenticated proxy → `/api/admin/members/services` → `members.rs::services`.
- Main filter: the catalog's `membership_plans.id` → `handleServiceFilter()` → `product_id` → `member_selection::MemberSelection` → `order_items.product_id`.
- Drill-down: `getMembersByService(id)` → `/api/admin/members/service/:id` → `members.rs::members_by_service`, where the ID actually means `user_memberships.plan_id`.

Confirmed gaps:

- The same displayed service ID crosses different namespaces. A membership-plan ID sent as a purchased-product ID can select a different audience, especially when numeric IDs collide.
- The catalog only includes plans with `active`/`trialing` memberships, omits other relevant plans and `trial` rows, and truncates at50 without pagination metadata. It emits `price_cents` while the client `Service` type expects `price`.
- The drill-down returns only basic user fields, but its page reads rich `Member` status, plan, spending and joined-date fields. An unknown plan gets a fabricated `Service {id}` response instead of a real not-found result.
- Its total is filtered while active/trial counts are not. `churned_members = total - active - trial` can be inconsistent or negative for filtered/overlapping membership rows. Revenue again sums plan prices, and failed count/revenue reads become zero.
- The service page renders empty results rather than `serviceMembersStore.state.error`. Its Next/Previous handlers send only `page`, dropping the active search/status filters. The store does not retain those service filters.

Checklist:

- [ ] Make the existing Service control explicitly select `plan_id`. Preserve the separate purchased-product meaning of `product_id`; do not remap those IDs by coincidence.
- [ ] Return a searched/paged catalog with explicit identity, currency/price units and complete selectable scope. Preserve selections outside the current catalog page.
- [ ] Reuse current member facts and one validated selection for drill-down rows, counts and pagination. Define whether status is global member status or membership status within the selected plan, and label it accordingly.
- [ ] Return404 for a missing/deleted plan and explicit errors for invalid filters. Propagate storage failures.
- [ ] Preserve search, status and page size across page navigation, URL reloads and refresh; expose per-panel failures.

Proof: deliberately colliding plan/product IDs; more than50 plans and multiple result pages; active and historical memberships in the same plan; filtered totals and overlapping statuses; missing-plan404; malformed-success/error recovery; main filter, export and drill-down audience agreement.

The `/admin/members/churned` branch also calls `membersApi.getChurnedMembers()` → generic proxy → `members.rs::churned_members`. Its displayed win-back-potential and days-since-churn controls send `winback_potential` and `churned_within_days`, which `MemberQuery` does not deserialize/use. The backend returns basic user rows and `lost_revenue_cents`/hardcoded `avg_lifetime_days`, while the client expects rich member fields, `potential_recovery_revenue` and `top_churn_reasons`. Its membership/count queries use the older status-only rules and failed aggregates become zero.

- [ ] Give the churned branch checked current/former-member facts, actual filter semantics, consistent money/history scope and visible errors. Do not invent a recovery probability or reasons to populate the existing controls.
- [ ] Prove both churned filters actually change the selected audience and survive paging/reload; cover reactivated members, expiry/date boundaries and failed reads.

## 3. Full member profile and detail drawer

Established UI: `/admin/members/[id]` and `MemberDetailDrawer.svelte`, including overview, spending, subscriptions, orders, account state, timeline, notes and moderation actions.

Current paths:

- Full page: `membersApi.getMember(id)` → generic authenticated proxy → `members.rs::show` at `/api/admin/members/:id`.
- Drawer: `membersApi.getMemberFull(id)` → dedicated `/api/admin/member-management/:id` proxy → `admin_member_management/crud.rs::get_member_full`.

Confirmed gaps:

- `members::show` returns a basic user, top-level subscriptions and a small stats object. The full page assumes rich `Member`, `engagement_score` and `timeline` fields, then substitutes `0`/`[]` for absent facts. It does not attach the actual top-level subscriptions to `member.subscriptions`.
- The richer endpoint returns `stats.total_spent_cents`, subscription `price_cents` and order `total_cents`. Drawer types/renderers instead read `total_spent`, `price` and `total` as dollars, so real amounts can display as zero.
- The richer spending query masks failure and sums gross completed orders without the net/refund/USD scope of the existing member-facts view. Active-subscription counts only inspect the status string.
- Rich `member.status` is taken from account moderation state or defaults to `active`, conflating account restrictions with subscription status. Its member query does not exclude soft-deleted users.
- The backend engagement formula is a hardcoded mix of subscription counts, recent activity and order counts. The full page displays it as an engagement percentage, and the basic endpoint does not even supply that value.
- Orders are capped at50, activity at20, and the constructed timeline at20 with no coverage or pagination contract. `stats.total_orders` is only the capped returned order count. The full page initializes only on mount, so reused route parameters and stale replies need explicit handling.

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

Confirmed gaps:

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

Confirmed gap: the full page hardcodes six suggested labels and `addTag()`, `removeTag()` and custom-tag actions only mutate local strings. Some actions show a success toast without any request. A reload loses the change.

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

| Existing source                                                                       | Reuse / boundary                                                                                                                                                                                                                              |
| ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api/migrations/099_member_current_memberships.sql`                                   | Current non-deleted membership/status, tags and net USD spending facts. Preserve immutable applied migrations; new changes need a forward migration if required.                                                                              |
| `api/src/services/member_segmentation.rs`                                             | Validated member/segment/audience predicates and existing configuration evidence.                                                                                                                                                             |
| `api/src/routes/member_selection.rs`                                                  | Shared list/export selection, including segment/tag/search/status/product/plan/date/spending/sort. Preserve list/export parity.                                                                                                               |
| `api/src/routes/admin_members/{segments,tags,filters}.rs`                             | Existing persisted segment/tag/filter workflows and atomic audits; do not replace them with a second namespace.                                                                                                                               |
| `api/src/routes/admin_member_management/{crud,notes,activity,moderation}.rs`          | Existing full-detail, CRUD, notes, activity and account-action infrastructure requiring contract/metric alignment.                                                                                                                            |
| `api/src/routes/admin/memberships.rs`, `subscriptions_admin/`                         | Existing membership/admin subscription handlers; preserve distinctions between manual membership state and provider billing operations.                                                                                                       |
| Native email templates, transactional renderer, `email_logs`, outbox/receipt services | Existing saved creative, delivery/provider evidence and idempotency patterns; do not equate HTTP success with delivery.                                                                                                                       |
| `api/tests/support/member_exports.rs`                                                 | Actual 112c endpoint proof passed. Repeat the full fresh API proof after the PDF range repair; see the checkpoint above for the exact scope and retained failure.                                                                             |
| `frontend/tests/admin/member-exports.spec.ts`                                         | Three prepared export browser cases: real CSV/XLSX/PDF and legacy downloads/filter/reload; malformed200/422/503 recovery without false success; anonymous/ordinary-user rejection. Pending execution on the new verified runtime.             |
| `api/scripts/verify-member-exports.py`                                                | Independent CSV/openpyxl and ActualText-aware Poppler row checks, retained generic-reader diagnostics and PDF rendering. The 112c run exposed a PDF defect; the repaired probe passed its stated scopes. New API-artifact QA remains pending. |
| `api/scripts/render-member-export.mjs`                                                | Read-only artifact-tool XLSX previews; all 10 actual 112c previews inspected without changing the source workbook. Preserve the timestamp preview precision limitation and repeat against the new actual artifacts.                           |

## Proof recording checklist

- [ ] Record the compiled artifact/source checkpoint, fresh schema/migration summary, isolated database/Redis/local-provider identities and actual commands/log paths.
- [ ] Run backend proofs before browser suites; stagger fixture logins and local SMTP relay ownership. Reset only an identified owned fixture if rate limits require it.
- [ ] Independently read downloaded files, compare all selected rows with the retained database snapshot, verify CSV escaping/OOXML literal and date types/actual PDF pages, then inspect first/middle/last, Unicode, empty and wrapped-record renders.
- [ ] Run each existing action through UI→client→proxy→authorization→storage/provider→UI, including reload, real errors, malformed200, role rejection and recovery.
- [ ] Add scoped regression tests for confirmed defects and run the required zero-warning gates at the final source checkpoint. Exclude failed attempts from passing evidence.
- [ ] Update only the matrix rows supported by new evidence. Leave Members analytics and the other membership/subscription administration pages unverified until their separate traces/proofs run. Preserve completed segment evidence and keep every remaining workflow visible.
