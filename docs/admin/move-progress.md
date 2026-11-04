# Admin/backend source completion and extraction

Task: execute `move-prompt.md`, finish source before copying, exclude the blog editor. Started 2026-11-03.

**Phase 1 is incomplete. No extraction/copy has been made.** The user's destination clarification resolved to the empty `/Users/billyribeiro/dev/admin-dashboard` directory (discovered and inspected). Recheck emptiness immediately before copying. Leave the completed source intact.

## Current state

- Inventory: 168 admin pages and source client/proxy/route/service/job declarations, SQL references and page actions/navigation in `admin-inventory.json`. Manual cross-layer tracing is incomplete.
- Completion and remaining gaps: `completion-matrix.md`.
- Actual verification evidence and limitations: `verification-results.md`.
- Blog editor: excluded; all 55 file hashes in `blog-editor-boundary.json` match the pre-edit source. Do not change editor-specific integrations outside those paths either.
- Initial working tree contained only the user's untracked `move-prompt.md`. HEAD advanced externally during this session to `92532a21e`, incorporating the first repair batch; the agent did not make that commit. Preserve subsequent user changes.

## Latest continuation (companies, lists, templates, scheduling)

- Company create/edit/detail, update/duplicate, validation, filtering and pagination are implemented. Association/statistics workflows remain open.
- Contact-list create/edit/detail, durable memberships (076), membership add/removal, derived counts, and contacts list_id filtering are implemented. Browser tests prove removal preserves contacts.
- Templates now persist HTML/design/configuration/default/owner fields (077), with update/duplicate/categories/prebuilt and real pagination/filtering. Create/edit stubs replaced and missing prebuilt page added. Core browser workflow passed; broader library actions still need coverage.
- Shared client preserves pagination envelopes for 29 CRM calls, does not deduplicate writes, and sends DELETE bodies. Three regression tests passed. Existing store subscriptions remain for compatibility; the autofixer's conditional store migration suggestions do not apply to this established store interface.
- CMS schedules/releases now run through an actual recurring worker started after migrations. Content changes, statuses and history commit together; concurrent claims and retry backoff are tested. Generic queue UUID IDs and atomic leases repaired. Missing provider/unknown or unimplemented jobs now fail truthfully. Unsupported job implementations, legacy scheduled-content/webhook correctness and release notifications remain open.
- Fresh empty r8 DB regression passed (12.21s), including actual recurring worker execution, release effects, concurrent claims, retries and terminal queue failures. Controlled Postmark HTTP adapter test passed; this is not live-provider verification.
- Five browser tests passed (21.7s): login/settings/behavior/logout, access rejection, company CRUD, list memberships, template CRUD/preview. Frontend check 0/0; 65 unit files, 2,796 passed / 32 skips. Latest lint/build/SEO still require completion or reruns after new changes.
- Test isolation correction: a production build overwrote the verification server's shared generated environment and later fixture login attempts reached source API port 8080. Those runs failed and are not proof. No real credentials were used. Frontend now uses `.svelte-kit-admin-verify` plus a separate Vite cache, generated configuration confirmed 8281, and all five browser tests subsequently passed. Do not run the verification frontend with the production `.svelte-kit` output.

## Latest contact/lead and worker verification

- Contact create/edit and lead edit forms are connected. Missing contact-new page and lead GET endpoint added. Company-name payload alias preserved; server-side email/status/length/tag/custom-field validation added; duplicate-email conflicts are useful errors.
- Local contact/lead listing pages no longer require an external CRM provider. Backend errors surface; CRM contact/deal/campaign totals query actual rows instead of fabricated deal zeros. Full list pagination/filter/bulk and detail/history/notes/scoring/value work remains open.
- Six browser workflows passed (26.3s), now with shared application-console, page-error and failed-API assertions in every test. Third-party browser traffic is deliberately blocked and distinguished from application failures.
- Fresh r11 regression passed (22.45s): contact/lead edits and validation, preserved tags/custom fields, company alias and actual deal totals, including statistics error propagation, plus previous source repairs.
- r10 passed legacy unpublish/update execution via the actual queue worker, unknown-type failures, webhook SSRF rejection and expired final-attempt leases. Webhook delivered_at/attempt counters/atomic claims/backoff were repaired; successful external webhook delivery still needs controlled/provider proof.
- Independent baseline proof passed after 076/077: 69 applied migration versions, 215 tables. Latest root unit gate: 65 files / 2,796 passed / 32 skips. Latest completed SEO: 112 passed. Root check exits 0 with 301 warnings, still below the required zero-warning gate.
- Latest Rust build/check/clippy passed before the small recursive-null-validation and email sender-alias adjustments; recursive-null regression passed. Rerun final checks after new work. Latest verification API includes the recurring CMS worker and latest contact/lead/stats endpoints on 8281; original source 8080/5173 stays running.
- HEAD remains 92532a21e. Files were staged externally during this continuation; do not reset or unstage the user's index.
- Current open implementation focus: CRM segments (new/edit stubs, no PUT/contact retrieval, sync only changes timestamp). Investigated only; **no segment changes yet**. Other major gaps listed below remain.

## Implemented in this session

1. Atomic empty-database baseline + actual forward migrations/defaults; fail-fast API startup; close migration connection on error to release SQLx session advisory locks.
2. Durable CRM root/double-opt-in/email-preference settings endpoints, with partial-update locking and validation.
3. Real ClamAV adapter, truthful persisted media states, actual object retrieval, infected direct-upload rejection and infected stored-object removal paths. Scanner tested separately from storage integration; S3 paths and media UI remain unverified/incomplete.
4. Durable first-party behavior ingestion, atomic validation, retry deduplication, sanitized event persistence and real dashboard aggregates, available without external analytics credentials.
5. Shared base admin-role policy across Rust/UserResponse, frontend helpers and SvelteKit guards; developer role and established aliases covered. Configured-email/fine-grained policy audit remains open.
6. Fixed logout proxy body, access/refresh revocation, cookie clearance and truthful failures; auth-prefixed logout alias added.
7. Corrected the baseline test so it executes forward migrations rather than merely marking their checksums applied.

## Verified

- Frontend check: 0 errors / 0 warnings; eslint exit 0; unit 2,793 passed / 32 pre-existing skips.
- Production build and library pack exit 0. Build still warns about 76 adapter route exclusions being dropped.
- SEO browser suite: initial 112 passed; final rerun in `/tmp/admin-move-seo.log`.
- Root `vp check`: exit 0 but 304 warnings, including excluded BlockEditor files. Does not meet zero-warning completion gate. Do not suppress warnings or weaken checks.
- Rust check/clippy, 53 no-DB integration tests, 9 media contract tests, 2 scanner protocol tests passed; real ClamAV clean/EICAR check passed.
- New regression explicitly ran on an empty disposable r4 DB and passed. Independent baseline proof passed with 67 migration versions / 214 tables.
- Two authenticated browser checks passed against isolated API/frontend/Redis: login, CRM save/reload, behavior fixture display, logout, access/refresh replay rejection, anonymous/non-admin denial, no page errors or unexpected failed API responses in tested flows.

## Local test resources

Existing source services on 8080/5173 were not stopped. Verification used:

- API `8281`, frontend `5274`, no live provider credentials; process output `/tmp/admin-move-api.log`, `/tmp/admin-move-web.log`.
- Disposable databases `admin_move_verify_20261103`, `_r2`, `_r3`, `_r4`, `_r5`, `_r6`, `_r7`, `_r8`, `_r9`, `_r10`, `_r11`, `_baseline` on local PostgreSQL `127.0.0.1:55432`.
- Created Docker containers `admin-move-redis-20261103` (loopback `16380`) and `admin-move-clamav-20261103` (loopback `13310`, amd64 image on ARM).
- Fixture accounts only: `admin@admin-completion.test`, `user@admin-completion.test`; password `OnlyForDisposableTest123!` (test data, never use outside disposable environments).
- Tests use an empty environment and explicit local configuration. See the generated-environment isolation correction above; failed fixture auth attempts touched source API rate/audit state. No live campaigns, charges, production DB changes or deployments were made.

Stop/remove only these task-owned test containers/processes after verification. Keep the source services untouched. Disposable DBs contain only fixtures and may be dropped explicitly by name when no longer useful.

## Resume work

1. Re-read `move-prompt.md`, this progress file and the completion matrix. Verify the editor hashes and current working tree.
2. Finish the remaining 13 CRM stubs and their actual backend contracts. Settings configuration persistence alone is not complete CRM behavior.
3. Fix media UI `files/pagination` versus backend `data/meta` and incorrect CRUD/bulk/optimization paths. Finish actual processing and verify uploads/scanning/removal with an isolated S3-compatible service.
4. Implement mounted boards routes and workflows; heatmaps/replay and other analytics endpoints; SEO loaders/contracts; member-segment edit/export/analytics actions.
5. Finish background job implementations: several branches currently only log and return `Ok(())`; the email worker also claims success when credentials are absent. Prove scheduling/retries/worker effects against isolated providers.
6. Finish the role/email/fine-grained permission audit and every remaining route/action trace. Run authenticated navigation and actual fixture workflows across every module.
7. Resolve the zero-warning gate conflict with the editor exclusion explicitly; do not edit excluded files or change lint thresholds to force a pass.
8. Update all evidence, run every required source gate to completion and satisfy Phase 1 before extracting anything.
9. Only then copy to the verified-empty destination, omit secrets/customer data/dependencies/builds/Git history, prove independent startup and deliver Phase 3/4 setup/customization documentation.
