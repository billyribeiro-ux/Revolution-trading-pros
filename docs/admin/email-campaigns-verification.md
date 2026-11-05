# Email campaign verification

Native draft authoring, the member audience adapter and native shared-worker delivery are locally verified. The complete campaign module remains unfinished; no extraction or copy has begun.

## Implemented and locally verified

- Migration100 preserves existing campaign identities, content and historical counters. It adds validated authoring configuration, creator/version metadata and an optional shared-outbox reference. Applying the migration does not submit or send campaigns.
- Native CRUD, duplication, template snapshots, nullable reference/configuration updates, search, filtering and pagination use actual persistence and atomic audit records. Changing a template later does not silently change a saved campaign. A duplicate has its own identity, draft status and no inherited requested time. Sent records reject editing. New timestamps are explicitly UTC despite the legacy table's naive timestamp columns.
- Historical counters remain separate. Displayed delivery metrics count recorded outbox receipts; a missing rate denominator remains NULL. Provider acceptance is distinct from inbox delivery and recorded open signals are not asserted to be human engagement.
- SvelteKit proxies bound request bytes, require administrator access, preserve backend failures and never retry send requests. The primary client rejects malformed success responses, mismatched mutation identities and unconfirmed delete acknowledgments.
- Draft management remains available when email delivery is disconnected. Active member options and template options come from the API; fabricated segment counts were removed. List filters/search/page20, stale-response protection and deleting the last page's final record use the backend.
- The member audience adapter uses the same rule compiler as member listings. Both dynamic groups and static assignments still require current newsletter confirmation. Submitted audiences store criteria/version and member identity, without treating a member ID as a contact foreign key. Changes to live rules or new confirmations do not rewrite the submitted queue. Withdrawals, member deletion and changed member email are checked before delivery.

## Evidence

- Fresh `admin_move_verify_member_campaign_r56`:122.65s, adapter plus previous regressions. The controlled local provider accepted exactly one eligible member; withdrawn/deleted members were skipped. This is adapter verification, not live-provider verification.
- Fresh `admin_move_verify_campaign_authoring_r58`:120.94s,92migration versions/latest100. Covers actual CRUD/snapshots/clearing/reload through an independent pool/UTC clock, roles, filters/pages, audit rollback, duplication, historical-counter separation and deletion. Earlier r57 expected400 for an unknown JSON field that correctly returns422; it is excluded from passing evidence.
- Browser4:4tests14.3s, actual template/member options, draft creation/reloads/duplication/deletion,21record search/pagination/last-page deletion, anonymous/member rejection/logout, malformed200 recovery and explicit503 statistics recovery. Positive authoring includes console and failed-network assertions; fixtures were cleaned up. Browser1 used an invalid empty segment rule; browser2 used the wrong button label. Browser3's positive workflow passed5.6s; neither earlier failure is counted as passing.
- Primary contracts/proxy/member-client regressions:14tests747ms. Full frontend:100files2901passed,32existing skips. Current scoped lint and Svelte check passed. Rootcheck3 passed with0warnings/2121files; frontend build3 passed with87Cloudflare routing rules. The production preview identity was checked against the disposable API before privacy1 passed4.6s and SEO1 passed114tests21.3s. The subsequent native delivery evidence is recorded below.

- Fresh `admin_move_verify_campaign_delivery_r59`:120.91s,93migration versions/latest101. Actual native API scheduling/pause/resume/A-B cohorts, frozen subjects, two controlled Postmark acknowledgments, physical-template usage, atomic staging rollback, internal-engine mutation guards, cancellation and opt-out after campaign deletion passed. This is controlled adapter verification, not live Postmark verification.
- Native timed SMTP browser4 passed33.4s: real opt-in link received/redeemed, primary UI Schedule, pause surviving reload and due time, no submission while paused, Resume, exactly one local SMTP acknowledgment after due time, sent receipt/reload and no unexpected console/API/network failures. Recipient-restricted relay accepts only this run's `@example.test` fixture. Completed disposable campaign/member/tag/segment/template fixtures remain available for inspection; no queued work remains. Browser1/2 fixture422 and browser3 raw MIME header assertion failures are excluded from passing evidence.

## Report repair in progress

The report demo fallback has been removed. New report reads count durable outbox receipts/events in one database snapshot, keep historical counters separate, return nullable rates and measure frozen A/B cohorts without inventing a winner. Bounded provider observations preserve supplied device/client/country/link fields without retaining raw IP addresses. The provider field mapping follows [Postmark click webhook](https://www.postmarkapp.com/developer/webhooks/click-webhook) and [open webhook](https://postmarkapp.com/developer/webhooks/open-tracking-webhook) documentation. Existing SMTP capability requests retain truthful unknown device/geography.

This report batch is not runtime verified yet. Report contracts:8tests674ms including existing campaign contracts. Svelte check2:0errors/0warnings. MCP all changed report components and loaders:0issues/0suggestions. Fresh r60 database created for proof; compilation is pending. Same-template-reference snapshot preservation regression is included in that proof. Shared report CSV export now uses the existing formula-safe exporter.

## Remaining work blocking module completion

Complete measured report/export browser proof, test-message receipts, analytics compatibility endpoints and truthful delivery failure presentation. Native send/schedule/pause/resume/cancel and frozen A/B delivery are locally verified; advanced retry/uncertainty/concurrency cases still need native-specific proof. Finish the secondary `/email` campaign contract, actions and dashboard statistics. Complete reference pickers beyond the first100records and preserve selected identities during search. The report repair and unchanged-template snapshot regression still require fresh runtime verification.

Do not begin copying while these or other documented source gaps remain open.
