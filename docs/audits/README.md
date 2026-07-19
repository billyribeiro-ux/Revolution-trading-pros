# Audits

End-to-end audits of the Revolution Trading Pros codebase, indexed
newest-first. Dates not encoded in the filename are taken from the
document header (≈ marks dates inferred from branch/session context).

| Date | Doc | Summary |
|------|-----|---------|
| 2026-07-19 | [PRINCIPAL_ENGINEER_AUDIT_2026-07-19.md](PRINCIPAL_ENGINEER_AUDIT_2026-07-19.md) | Whole-repo principal-engineer audit (gates re-run, six parallel subsystem auditors, every P0/P1 hand-verified); verdict B− — excellent bones, two production-blocking defects. |
| 2026-06-07 | [RUST_DEEP_AUDIT_2026-06-07.md](RUST_DEEP_AUDIT_2026-06-07.md) | Rust backend deep audit (~110k LOC): money path, auth/secrets, SQL safety, async/concurrency, error handling; core judged well-engineered, `panic = "abort"` makes reachable handler panics high severity. |
| 2026-06-05 | [TAILWIND_TO_SCOPED_CSS_BASELINE_2026-06-05.md](TAILWIND_TO_SCOPED_CSS_BASELINE_2026-06-05.md) | Baseline for migrating the frontend from Tailwind to plain Svelte-scoped page/component CSS, grounded in official Svelte docs. |
| 2026-06-01 | [TODO-RESOLVED-2026-06-01.md](TODO-RESOLVED-2026-06-01.md) | Resolution log for every TODO/FIXME in the Rust backend, verified with cargo check/clippy/fmt. |
| 2026-05-29 | [SVELTE_AUDIT_2026-05-29.md](SVELTE_AUDIT_2026-05-29.md) | Svelte frontend sweep (1,127 components) via svelte-check + MCP autofixer; frontend in excellent Svelte 5 health; includes a correction note retracting a phantom-file finding from an earlier draft. |
| 2026-05-24 | [FORENSIC_AUDIT_2026-05-24.md](FORENSIC_AUDIT_2026-05-24.md) | Forensic snapshot at the May Google-SEO-updates branch: all quality gates green, repo-hygiene debt cleaned, two prior P0 blockers mechanically closed (live Playwright pass still owed). |
| 2026-05-20 | [ORPHAN_INVENTORY_2026-05-20.md](ORPHAN_INVENTORY_2026-05-20.md) | Read-only inventory of orphaned files (frontend API clients with no backend, etc.), each a candidate for reconnection under the "create, never delete" rule. |
| 2026-05-17 | [FORENSIC_FULL_REPO_AUDIT_2026-05-17.md](FORENSIC_FULL_REPO_AUDIT_2026-05-17.md) | Independent second full-repo audit at a newer HEAD (16b350b): 10 parallel deep-read agents plus forensic hand-verification of every load-bearing P0/P1. |
| 2026-05-17 | [FULL_REPO_AUDIT_2026-05-17.md](FULL_REPO_AUDIT_2026-05-17.md) | Full monorepo audit at HEAD c31c43d: all quality gates plus six parallel deep-read audits; hand-verified findings marked [V]. |
| 2026-05-17 | [REMEDIATION_PLAN_2026-05-17.md](REMEDIATION_PLAN_2026-05-17.md) | Staged root-cause remediation plan for every finding in the 2026-05-17 full-repo audit (principal-engineer discipline, regression test per fix; Bunny CDN outage worked around). |
| 2026-05-17 | [G0_3_SCHEMA_BASELINE_2026-05-17.md](G0_3_SCHEMA_BASELINE_2026-05-17.md) | Schema-reproducibility fix: reconstructed baseline proven with all 60 migrations replaying on a fresh DB; one owner-gated production cutover step remains. |
| 2026-05-17 | [MONEY_PATH_DIG_2026-05-17.md](MONEY_PATH_DIG_2026-05-17.md) | End-to-end money-contract trace (form → proxy → Rust → SQL) exposing a family of admin forms sending dollars where the API expects integer cents. |
| 2026-05-16 | [SVELTE_FRONTEND_AUDIT_2026-05-16_2146.md](SVELTE_FRONTEND_AUDIT_2026-05-16_2146.md) | Evidence-based frontend audit (898 .svelte files): typecheck genuinely clean; the real signal is in the 2,242-problem eslint surface. |
| 2026-05-10 | [REPO_AUDIT_2026-05-10.md](REPO_AUDIT_2026-05-10.md) | Monorepo investigation: every quality gate run end-to-end plus four parallel specialized audits (frontend, backend, security, hygiene). |
| 2026-04-29 | [FINAL_REPORT.md](FINAL_REPORT.md) | Wrap-up of the payments-fix arc: Batches 4/5a/5b/5c merged as PRs #562–#565, Batch 7 PR open, Batch 6 (Postmark) skipped. |
| 2026-04-29 | [SECURITY_GAPS_2026-04-29.md](SECURITY_GAPS_2026-04-29.md) | Security-gap audit + fix pass: OAuth token leak, unverified JWT decode, JWT blacklist fail-open and more fixed; R2 credential rotation left as user action. |
| ≈2026-04-29 | [BATCH7_RESULT.md](BATCH7_RESULT.md) | Pre-launch checks: `validate_production_secrets()` verifying live Stripe key/webhook-secret shapes; gates green, runtime re-run pending operator. |
| ≈2026-04-29 | [BATCH6_RESULT.md](BATCH6_RESULT.md) | Postmark transactional-email integration; gates green, verification scenarios pass or partial with documented blockers. |
| ≈2026-04-29 | [BATCH4_RESULT.md](BATCH4_RESULT.md) | Re-subscribe history + coupon admin gap fixes; gates green, runtime scenarios pending operator. |
| 2026-04-28 | [BATCH3_5_RESULT.md](BATCH3_5_RESULT.md) | Coupon billing fix: server-applied coupons previously recorded a discount in the DB while Stripe charged full price. |
| 2026-04-28 | [BATCH2_RESULT.md](BATCH2_RESULT.md) | Admin dashboard de-zeroing: ghost columns + swallowed SQLx errors produced silent $0 metrics; adds migration 061 (money-cents unification). |
| 2026-04-28 | [SECURITY_FIXES_RESULT.md](SECURITY_FIXES_RESULT.md) | Implementation record for the critical payment security fixes, starting with the free-subscription reactivate bypass. |
| 2026-04-28 | [SECURITY_FIXES_VERIFICATION.md](SECURITY_FIXES_VERIFICATION.md) | Independent verification of the security fixes by re-reading source plus compiler gates. |
| 2026-04-28 | [TASK2_RESULT.md](TASK2_RESULT.md) | Real Stripe checkout E2E evidence: seeded plans, live test-mode session, webhook-created membership, idempotency confirmed. |
| 2026-04-28 | [TASK7_RESULT.md](TASK7_RESULT.md) | End-to-end payment verification matrix: subscribe/cancel/resume/re-subscribe/failed-payment/refund/course-purchase scenarios all PASS. |
| ≈2026-04-28 | [TASK4PROMPT.MD](TASK4PROMPT.MD) | The operator prompt that kicked off Task 4 (Postmark email service + templates). |
| ≈2026-04-28 | [EXECUTION.MD](EXECUTION.MD) | The operator directive for the payments-fix arc: fix checkout/subscriptions/security/email in place, no rebuilds. |
| 2026-04-28 | [SUBSCRIPTION_AUDIT.md](SUBSCRIPTION_AUDIT.md) | Adversarial payments/billing security audit: NOT ready for paying customers — free-reactivation bypass, webhook body handling, base64 "encrypted" Stripe key. |
| 2026-04-28 | [SUBSCRIPTION_DISCOVERY.md](SUBSCRIPTION_DISCOVERY.md) | Live-vs-dead classification of the subscription surface: what is exploitable, broken, dangerous, or actually functional. |
| 2026-04-28 | [ADMIN_SYSTEM_DISCOVERY.md](ADMIN_SYSTEM_DISCOVERY.md) | Read-only discovery of the full `/admin` + `/api/admin` surface (~50 backend modules, 190+ pages, 163 DB tables) with live Postgres cross-checks. |
| 2026-04-28 | [MIGRATION_REPAIR_2026-04-28.md](MIGRATION_REPAIR_2026-04-28.md) | Migration-repair plan: backup + inventory complete, execution phases gated on user approval. |
| 2026-04-28 | [SESSION_CONTINUITY.md](SESSION_CONTINUITY.md) | Session hand-off notes: repo-wide svelte-autofixer sweep complete (keyed `{#each}` blocks), pointers for the next session. |
| 2026-04-27 | [AUTH_AUDIT.md](AUTH_AUDIT.md) | Adversarial auth/secrets/CSRF/CSP/session audit; critical finding: plaintext R2 credentials in `api/.env` needing rotation. |
| 2026-04-27 | [CLEANUP_RESULT.md](CLEANUP_RESULT.md) | Removal of ~2,565 lines of working-arc blog documentation after folding the substance into CHANGELOG.md. |
| 2026-04-26 | [admin-2026-04-26/](admin-2026-04-26/00-MASTER-SUMMARY.md) | Directory: 12-cluster admin audit + remediation series (shell/dashboard, members, commerce, content, video, CRM, marketing, analytics, system, workflow, cross-cutting, sidebar) — start at the master summary. |
| 2026-04-26 | [PRINCIPAL_FIX_PLAN_2026-04-26.md](PRINCIPAL_FIX_PLAN_2026-04-26.md) | Five surgical architectural changes that resolve the whole family of recurring `/admin` defects instead of 50 individual patches. |
| 2026-04-26 | [AUDIT_REPORT.md](AUDIT_REPORT.md) | Comprehensive full-stack audit via 14 parallel sub-agents: 24 blockers (wrong-cookie admin proxies, Svelte 5 cascade bombs, silent DB error swallows) + 88 majors. |
| 2026-04-26 | [AUDIT_FIX_PLAN.md](AUDIT_FIX_PLAN.md) | Phased fix plan for AUDIT_REPORT.md: restore production correctness first, then enforce standards. |
| 2026-04-26 | [AUDIT_FIX_SUMMARY.md](AUDIT_FIX_SUMMARY.md) | Implementation summary of the audit fix plan: 12 parallel sub-agents across 5 waves, all verification gates green. |
| 2026-04-26 | [CASCADE_ROOT_CAUSE_REPORT.md](CASCADE_ROOT_CAUSE_REPORT.md) | Root cause of the `effect_update_depth_exceeded` cascades: one defect in `connections.svelte.ts` inherited by 13 admin pages; one store fix converts them all to safe. |
| 2026-04-26 | [BACKEND_DEEP_DIVE_REPORT.md](BACKEND_DEEP_DIVE_REPORT.md) | Rust/Axum forensic deep-dive beyond the main audit: 4 critical findings led by string-interpolated SQL in `videos.rs`. |
| 2026-04-26 | [ADMIN_DASHBOARD_REPORT.md](ADMIN_DASHBOARD_REPORT.md) | Forensic pass over 129 admin routes: SSR disabled for `/admin/*`, broken quick-action tiles, duplicate dashboards, `alert()`/`prompt()` UX regressions. |
| 2026-04-26 | [ADMIN_FAILURE_DATA.md](ADMIN_FAILURE_DATA.md) | Live HTTP probe + source cross-reference of admin/dashboard/sidebar; top findings: logout deletes the wrong cookies, structural POST-405 cliff in the proxy tree. |
| 2026-04-26 | [SIDEBAR_REPORT.md](SIDEBAR_REPORT.md) | Forensic teardown of `AdminSidebar.svelte`: import/render sites, props, and per-link behavior. |
| 2026-04-25 | [REPO_STATE_2026-04-25.md](REPO_STATE_2026-04-25.md) | Snapshot of the post-reset baseline: why main was reset, and the minimum fixes to make it install and pass all gates from a clean checkout. |
| 2026-04-25 | [PRODUCT_AND_AUTH_AUDIT_2026-04-25.md](PRODUCT_AND_AUTH_AUDIT_2026-04-25.md) | Auth, RBAC/ABAC, CRUD coverage and the four product verticals (indicators, courses, trading rooms, pricing) + Svelte Remote Functions migration plan. |
| 2026-04-25 | [ADMIN_AND_CMS_AUDIT_2026-04-25.md](ADMIN_AND_CMS_AUDIT_2026-04-25.md) | Backend admin, headless CMS v2, frontend admin/dashboard surfaces, and the integration layer; every claim cited to file:line. |
| 2026-04-25 | [DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md](DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md) | Anti-patterns, test coverage, dead code, perf, security hygiene, a11y/SEO/observability/i18n; §9 is the consolidated prioritized backlog (39 items). |
| 2026-04-25 | [UIUX_FORENSIC_AUDIT_2026-04-25.md](UIUX_FORENSIC_AUDIT_2026-04-25.md) | UI/UX forensic of admin shell + CMS editor + CSS architecture + tables + member dashboard; criticals include a CMS editor with no autosave. |
| 2026-04-25 | [ADMIN_SIDEBAR_AUDIT_2026-04-25.md](ADMIN_SIDEBAR_AUDIT_2026-04-25.md) | Forensic audit of the admin sidebar: all 24 primary nav links verified against implemented routes; no dead links. |
| 2026-04-25 | [ANALYTICS_DASHBOARD_AUDIT_2026-04-25.md](ANALYTICS_DASHBOARD_AUDIT_2026-04-25.md) | Analytics dashboard gap analysis against a Linear/Vercel bar (hand-rolled SVG charts, stubbed Goals/Funnels/Cohorts APIs); grade C+. |
| 2026-04-25 | [CSS_CASCADE_AUDIT_2026-04-25.md](CSS_CASCADE_AUDIT_2026-04-25.md) | Confirms `app.css` global `@layer base` rules override admin/dashboard styles by design, forcing `!important` fights; smallest fix is scoping. |
| 2026-04-25 | [RUST_BACKEND_AUDIT_2026-04-25.md](RUST_BACKEND_AUDIT_2026-04-25.md) | Rust backend forensic: top systemic issue is >200 `unwrap_or_default()` call sites silently masking DB errors. |
| 2026-04-25 | [PE7_CONTROL_PLAN_2026-04-25.md](PE7_CONTROL_PLAN_2026-04-25.md) | The enforced quality contract: 8 non-regressable invariants every change (agent or human) is judged against before reaching main. |
| 2026-04-25 | [MASTER_UIUX_BACKLOG.md](MASTER_UIUX_BACKLOG.md) | Canonical done/pending tracker for every UI/UX finding from the four 2026-04-25 audits. |
| 2026-04-25 | [ADMIN_QUICK_ACTIONS_BACKLOG.md](ADMIN_QUICK_ACTIONS_BACKLOG.md) | Backlog for the six `/admin` quick-action tiles aliased to neighboring surfaces because their dedicated routes don't exist yet. |
| 2026-02-05 | [SCHEMA_AUDIT_REPORT.md](SCHEMA_AUDIT_REPORT.md) | DB schema → Rust models → TS interfaces alignment check; the 500 error was middleware/proxy bugs, not a schema mismatch. |
| 2026-01-28 | [DEPENDENCY_AUDIT_REPORT.md](DEPENDENCY_AUDIT_REPORT.md) | Pre-pnpm dependency and configuration audit; all packages updated and both stacks building clean at the time. |
| 2026-01-25 | [EXPLOSIVE_SWINGS_AUDIT_REPORT.md](EXPLOSIVE_SWINGS_AUDIT_REPORT.md) | The first feature audit (became the model for later ones): Explosive Swings dashboard post-refactor, 242 hardcoded colors left to tokenize. |
| 2026-01-24 | [WEEKLYHERO_COMPONENT_AUDIT.md](WEEKLYHERO_COMPONENT_AUDIT.md) | Component-level audit of the WeeklyHero feature in the Explosive Swings dashboard. |

## Reading order for the 2026-04-25 series

1. **[REPO_STATE_2026-04-25.md](REPO_STATE_2026-04-25.md)** — what changed in
   the pnpm-migration commit and the verification gates passing.
2. **[PRODUCT_AND_AUTH_AUDIT_2026-04-25.md](PRODUCT_AND_AUTH_AUDIT_2026-04-25.md)**
3. **[ADMIN_AND_CMS_AUDIT_2026-04-25.md](ADMIN_AND_CMS_AUDIT_2026-04-25.md)**
4. **[DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md](DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md)**
   — **§9 is the consolidated prioritized backlog (39 items).**
