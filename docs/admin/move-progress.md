# Admin/backend source completion and extraction

Task started 2026-11-03 from `move-prompt.md`. Preserve the source, exclude the blog editor, finish and verify source functionality before copying.

Destination resolved from the user's clarification (the empty folder): `/Users/billyribeiro/dev/admin-dashboard`. Inspected empty. Recheck immediately before copying; never overwrite unrelated files.

## Phase status

- Phase 1: in progress; source inventory generated, workflow tracing incomplete.
- Phase 2–4: not started; gated on Phase 1. No copy has been made.
- Initial working tree: only untracked `move-prompt.md` (user supplied).
- Blog editor: excluded. Pre-edit hashes in `blog-editor-boundary.json`.

## Confirmed findings from current source

- CRM create/edit pages still render `ComingSoonStub` (see completion matrix).
- CRM settings handler acknowledges saves without persistence; page-specific settings endpoints are absent.
- Media scanning returns `is_clean: true` with provider `not_implemented`, scans empty bytes and stores no result.
- API startup logs migration errors and continues, allowing a misleading healthy process.
- Fresh databases require manually loading schema + migration ledger; structural defaults are forward migration 073, outside the baseline ledger ending at 072.
- Role aliases differ between Rust extractors and SvelteKit guards; developer role is rejected by the server page gate despite Rust granting admin access.
- Behavior events endpoint acknowledges events without persistence.
- Deal/pipeline handler header says unrouted, but actual router mounts CRUD. Treat earlier reports and comments as investigation leads only.

## Verification and environment

- Svelte MCP sections discovered and relevant docs fetched before development.
- Existing source API on port 8080 and frontend on 5173 must not be interrupted.
- Existing local PostgreSQL listens on 55432; Docker daemon access requires sandbox escalation. Use a new disposable local database/cluster, never existing app/customer rows.
- All requested gates and authenticated browser workflow checks remain pending.

## Resume order

1. Implement/test fail-fast migrations and reproducible empty-database initialization.
2. Persist CRM settings and mount endpoints actually called by the settings UI.
3. Implement real malware scanning with truthful unavailable/failed states and durable results.
4. Complete role alignment, behavior, CRM, boards, analytics/SEO, segments and remaining inventory workflows.
5. Run source gates, disposable-database integration and authenticated browser checks; update matrix with evidence.
6. Only after Phase 1 passes, copy into verified-empty destination and prove independent startup.
