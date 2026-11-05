# Sequence verification

The canonical CRM sequence backend has locally verified authoring, enrollment and shared-worker delivery subsets. The module and broader source project remain incomplete. No extraction or copy has begun.

## Implemented and locally verified

Migration104 preserves legacy definitions and counters while adding immutable revisions, durable enrollments, engine mappings and unique per-enrollment email receipts. Explicit definition replacement retains the original revision. Unknown legacy definitions require review; their email counts remain unknown. Revenue and currency remain unmeasured. Migration105 distinguishes an operator's sequence pause from provider unavailability. Both migrations have been applied in disposable proofs and are immutable.

New authoring supports real search/status/sort/pagination, atomic audit records, version checks, ordered email CRUD and duplication with new email identities and remapped condition references. Enrollment requires matching filters and current newsletter confirmation. Repeated enrollment cannot restart an existing cohort. Frozen content remains independent of later edits. New delivery uses the existing outbox, actual provider receipts, tracking, retained opt-out links and bounded retry/uncertainty rules. Internal delivery engines cannot be edited as public CRM broadcasts or counted as broadcast totals.

The scheduler advances an enrollment after its actual receipt and schedules its next email relative to recorded acceptance. Pause blocks further staging and claims. A provider call already in flight retains its actual acknowledgment even after cancellation. Unknown outcomes require reconciliation and cannot be automatically resubmitted. Contact deletion, changed addresses and withdrawn confirmation prevent new submissions.

## Evidence

- Fresh `admin_move_verify_sequence_authoring_r66`:131.85s,96migration versions/latest104. Includes all earlier regressions plus CRUD/ordering/nullable text/version checks, role rejection, confirmation enforcement, enrollment replay, subscriber pagination, removal, duplication, audit rollback, legacy review/repair retaining original content, and archived enrollment history. Authoring and enrollment made no provider requests; this proof does not establish execution.
- Fresh `admin_move_verify_sequence_delivery_r67`:137.06s. Includes earlier regressions plus actual local provider acknowledgments, custom sender/reply-to/plain alternatives, escaped contact variables, frozen second-step content after edits, exactly60seconds between recorded acceptance and next due time, no early staging, concurrent scheduler/claim exclusion, terminal rejection, uncertainty without resend, changed/deleted/withdrawn contacts, actual held provider submission with pause/resume rejection/removal and retained acknowledgment, and a one-connection pool. Only the disposable fixture clock was advanced after verifying its due boundary. This calls the actual scheduler and provider-worker functions; verification of the running timer and browser workflow is still pending.
- Rust check and Clippy passed without warnings for the r67 artifact. Later statistics, position, recovery and client work requires subsequent proof. An initial tuple complexity warning was corrected and is excluded from passing evidence.
- The new authenticated sequence proxies enforce route/byte bounds, preserve failures and never retry writes. Their3tests passed332ms. Initial Svelte validation caught an array-buffer type mismatch; the corrected check passed0errors/0warnings. All changed proxy modules passed the Svelte autofixer with0issues/0suggestions.
- Secondary catalog/proxy tests passed7tests823ms: identity/scope, pagination, preserved legacy review state, position mapping, read-only fields, unknown rate denominators, receipt relationships and no automatic write retry. Later client changes need their own current checks. No sequence editor/browser result is claimed.

November 5 continuation evidence supersedes the pending checks in the older bullets above:

- Fresh `admin_move_verify_resume_r70` passed in 137.19 seconds, with 97 successful migration versions through 105. It covers the additional observation callbacks, conditional steps, scheduler audit rollback, provider-pause recovery, HTTP 429 backoff, expired leases and authenticated reconciliation, plus the new contact-identity regression and independent newsletter/member source IDs. No migration was changed.
- The committed `api/scripts/verify-email-backend.sh` independently passed the same complete proof in 137.20 seconds on its own new PostgreSQL cluster. It recorded 97 migration versions through 105, exited successfully and stopped only its owned cluster. The printed fixture directory retains the proof, migration summary and shutdown logs. Rust all-target check, Clippy with warnings denied, build and formatting passed. Library tests passed 167 cases with one existing ClamAV-dependent ignore.
- The current canonical client tests passed 6 cases, including multi-record pagination, exact reorder acknowledgment and UUID normalization. Full root unit tests passed 2,941 cases across 109 files, with 32 existing skips. Current Svelte validation and scoped lint/autofixer checks passed.
- Three authenticated browser cases passed for two-email ordering, saved content and reload, activation/read-only/pause/edit, independent duplication/deletion, failed and malformed read recovery, stale-version rejection, confirmed-save preservation during a statistics outage, no write on reads, and anonymous/member rejection in both proxy namespaces. A separate final SMTP case passed in 18.0 seconds: the actual running timers submitted one message to a recipient-restricted local relay, recorded acceptance and completion, retained metrics after reload and did not resend after repeated enrollment. No fixture clock or direct scheduler call was used in this browser case. Positive workflows reported no unexpected console or API failures.

Initial browser attempts with an omitted Redis dependency, an inherited development WebSocket URL and incorrect fixture selectors failed; those runs are excluded from passing evidence. Redis-backed login rate limiting remained enabled. The disposable Redis state was reset between independent browser suites after their fixture logins exhausted the per-IP allowance.

This is controlled local-adapter verification. No live Postmark service, external inbox, human engagement or real customer campaign has been verified or submitted.

## Source triggers, test messages and secondary actions (November 5)

Migrations106/107 add durable test-message requests and actual source-event enrollment. Both were applied and are immutable. Test messages bind a UUID to the saved sequence version, selected email, recipient and creative. Replaying that UUID returns its recorded provider outcome after edits or archival, without a second submission. An ambiguous same-address CRM identity is rejected. QA receipts use `sequence_test` scope and do not inflate production sequence metrics.

Triggers capture actual signup, newsletter confirmation, added contact tags, completed-order transitions and authenticated custom events in their source transactions. Only sequences active at capture receive a frozen revision match. The worker rechecks current source identity, audience, consent and order state before enrollment. Paused work remains held; archived work is cancelled. An unsupported historical trigger remains visible with an explicit repair state, and its original revision survives repair.

Current evidence:

- Fresh runner `/private/tmp/revolution-completion-fresh109b.log`: full guarded suite passed **151.51 seconds**, with101 successful migrations through109. It includes every prior regression plus actual QA provider acceptance/rejection/uncertainty and replay, source-event capture and enrollment, an overlapping source-write/worker deadlock regression, legacy trigger repair and atomic rollback. The runner stopped its owned PostgreSQL cluster and retained its logs.
- `sequences.spec.ts` and `sequence-actions.spec.ts`: **all9 browser cases passed in1.1 minutes**, log `/private/tmp/revolution-sequences109-browser2.log`. Cases cover both admin interfaces, actual selected-email SMTP QA and receipt history, rejection and replay without reconnecting, actual custom-event worker enrollment/history, legacy repair,21-record pagination and last-page deletion, malformed-response recovery, ordered authoring/reload/version barriers, role rejection and actual timer delivery without repeated-enrollment resend. Relay19027 accepted only explicit `.test` recipients.
- The first browser attempt failed because an in-flight shared contact import was duplicated. Its log is retained; the owner corrected the source before the passing rerun. No sequence assertions were weakened.

The verified browser database had zero active sequences and zero processing deliveries after cleanup. A controlled process-restart proof and the broader CRM permission work remain separate outstanding checks; no live-provider delivery or human engagement is claimed.

## Remaining work

The canonical and secondary clients, source triggers, test messages, enrollment/activity views and actual running timers have the local verification above. A read-only reconciliation found no remaining visible sequence action stub; sequence import/export controls belong to the separate Automations module. Some advanced field interactions and subscriber cancellation/filter paging have backend proof rather than individual browser-click evidence. A controlled process restart and finer CRM manager scopes remain open. The source-wide completion gate remains open.

One enrollment per sequence/contact and normalized address prevents accidental resubmission. Existing cohorts retain their revision. A deliberately new sequence identity can be created through duplication; it does not inherit enrollments or submit messages. Live-provider configuration and account verification remain external requirements, distinct from the adapter proofs above.

## Resumed session

The November 5 continuation recovered session `01a1a3fb-7e6f-7ed2-8016-88e7714aa29c` ("Execute move-prompt.md"). Its final messages record successful additional callback, rollback, provider-pause, HTTP 429 and reconciliation proofs and an implemented form. Those changes are present in the committed source, ahead of the older progress notes above. The previous temporary scripts and logs are unavailable, so this continuation is recreating independent evidence rather than treating their absence as a passing check.

The current repairs have the local proof described above: CRM audience eligibility checks the selected contact ID, so a same-address contact cannot supply matching tags, lists or fields. Newsletter and member sources retain their separate address mapping. Reorder acknowledgments must confirm the requested email identities and order. List mutation and pagination guards and editor navigation guards prevent stale responses from being presented as current saved state.

## Reproduce the backend proof

From the repository root, run `api/scripts/verify-email-backend.sh`. It requires the installed Rust dependencies and PostgreSQL with `pg_stat_statements`; it creates a new isolated database, uses only explicit fixture credentials and retains its logs after stopping its own server. See [the runner instructions](../../api/scripts/README.md) for port and PostgreSQL installation options. This command verifies the backend suite; it does not start the browser stack or establish live-provider delivery.
