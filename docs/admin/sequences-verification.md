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

This is controlled local-adapter verification. No live Postmark service, external inbox, human engagement or real customer campaign has been verified or submitted.

## Remaining work

Finish and verify actual observation conditions, scheduler audit rollback, provider-pause recovery, retry/lease and reconciliation edges. Complete canonical client contracts, new/edit pages, list actions/pagination, secondary navigation and its loading/failure states. Implement the established triggers and test-send workflow. Verify real timer execution with the recipient-restricted local SMTP relay, process restart/reload, permissions and failed integrations. Broader CRM manager scopes and the source-wide completion gate remain open.

One enrollment per sequence/contact and normalized address prevents accidental resubmission. Existing cohorts retain their revision. A deliberately new sequence identity can be created through duplication; it does not inherit enrollments or submit messages. Live-provider configuration and account verification remain external requirements, distinct from the adapter proofs above.
