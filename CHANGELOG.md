# Changelog

All notable changes to this project. Format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); we don't strictly adhere to SemVer because the product isn't a published library.

## [Unreleased] — 2026-04-25

### Local development

- **Added** Docker Compose stack (`db` + `redis` + `api`) so the platform runs locally without Fly.io.
- **Added** `api/scripts/seed-local-admin.sh` — Argon2id-hashed admin upsert via parameterized psql; safe against email/name containing quotes.
- **Added** `frontend/.env.local` template + auto-loading by Vite so `pnpm dev` points at the local API out of the box.
- **Added** `docs/development/LOCAL_DEV.md` runbook (boot from a fresh clone in 10 minutes, with troubleshooting).
- **Added** `docs/development/ENV_VARS.md` — canonical matrix of all 28 backend + 20+ frontend env vars.

### Tooling / quality

- **Changed** every SvelteKit `+server.ts` proxy under `frontend/src/routes/api/` (76 files) to read `env.API_BASE_URL || env.BACKEND_URL` from `$env/dynamic/private`, with the Fly URL only as a fallback. The previous hardcoded constants made local dev impossible.
- **Changed** `frontend/src/routes/+page.server.ts` (homepage SSR posts loader) to the same env-driven pattern.
- **Migrated** to pnpm 10.33 from npm; added `pnpm-workspace.yaml`.
- **Bumped** all dependencies to current latest (Vite 7, TypeScript 6, ESLint 10, Vitest 4, bits-ui 2, isomorphic-dompurify 3, zod 4, etc.).
- **Migrated** 41 shadcn-svelte wrapper components from the legacy `let props = $props(); let foo = $state(props.foo)` shadowing pattern to idiomatic Svelte 5 destructured `$bindable()` props (eliminates 50 `state_referenced_locally` warnings).

### Repo organization

- **Added** canonical `README.md` at the root.
- **Added** `CONTRIBUTING.md`, `.editorconfig`, `CHANGELOG.md`.
- **Moved** 20+ accumulated audit/setup/forensic markdowns into a structured `docs/` taxonomy (`docs/audits/`, `docs/architecture/`, `docs/setup/`, `docs/forensics/`, `docs/frontend/`, `docs/development/`).
- **Moved** stray shell scripts from the root into `scripts/`.
- **Removed** 9 one-shot frontend migration scripts that had completed their job (recoverable from git history).
- **Removed** stale `.build-trigger` placeholder.

### Documentation

- **Added** `SYSTEM_ARCHITECTURE_AND_AUTH.md` reference doc.
- **Added** `REPO_STATE_2026-04-25.md` snapshot of the post-pnpm-migration state.
- **Added** four end-to-end audits (in `docs/audits/`):
  - `PRODUCT_AND_AUTH_AUDIT_2026-04-25.md` — auth, RBAC/ABAC, CRUD coverage, products, Remote Functions migration plan
  - `ADMIN_AND_CMS_AUDIT_2026-04-25.md` — backend admin, CMS v2, frontend admin/dashboard, integration layer
  - `DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` — anti-patterns, test gaps, dead code, perf, security, a11y/SEO/observability/i18n
  - 39-item prioritized remediation backlog

### Test gates

All passing as of 2026-04-25:

| Gate | Result |
|------|--------|
| `pnpm check` | 8799 files / 0 errors / 0 warnings |
| `pnpm test:unit` | 1442 passed / 32 skipped / 0 errors |
| `playwright (chromium)` | 85 passed / 8 skipped (3 fixme + 5 API-gated) / 0 failed |
| `cargo check` | clean |
| `cargo test --no-run` | all 5 binaries compile |
| `cargo test utils_test stripe_test` | 17 / 17 |

### Known broken (not regressed; documented)

- Production Fly Postgres is unreachable while the subscription is renewed.
- CMS editor toolbar add-block click is a no-op (bits-ui v2 regression at `frontend/src/routes/cms/editor/+page.svelte:113`).
- Stripe Checkout-Session creation is a `// TODO` stub (`api/src/routes/subscriptions.rs:446`); funnel cannot take money via self-serve.
- Frontend `/admin` has no role gate (any logged-in user can mount the UI; backend rejects data calls).
- Favorites proxy reads the wrong cookie (`frontend/src/routes/api/favorites/+server.ts:32`).

See `docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` §9 for the prioritized backlog.
