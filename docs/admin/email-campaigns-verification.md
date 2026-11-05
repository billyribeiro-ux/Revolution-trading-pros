# Email campaign verification

Native draft authoring and the member audience adapter are locally verified. The complete campaign module remains unfinished; no extraction or copy has begun.

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
- Primary contracts/proxy/member-client regressions:14tests747ms. Full frontend:100files2901passed,32existing skips. Current scoped lint and Svelte check passed. Rootcheck3 passed with0warnings/2121files; frontend build3 passed with87Cloudflare routing rules. The production preview identity was checked against the disposable API before privacy1 passed4.6s and SEO1 passed114tests21.3s. The subsequent atomic-submission helper refactor is still under verification and is not part of these runtime claims.

## Remaining work blocking module completion

Connect native campaign identities to actual shared-worker send/schedule/pause/resume/cancel execution, A/B subject assignment and measured variant reports, test-message receipts, report/export endpoints and truthful error states. The current requested schedule time only saves a draft; the UI's Schedule action must be connected before completion. Finish the secondary `/email` campaign contract, actions and dashboard statistics. Complete reference pickers beyond the first100records and preserve selected identities during search. The existing report page still substitutes fabricated demo results on failure and must be repaired.

Do not begin copying while these or other documented source gaps remain open.
