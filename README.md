# Revolution Trading Pros

A SvelteKit + Rust monorepo for the Revolution Trading Pros platform — live trading
rooms, alert services, indicators, courses, mentorship, and a headless CMS.

> **Production status (2026-04-25):** the production database (Fly.io Postgres) is
> currently unreachable; live login fails. Local development runs against a Docker
> stack. See [`docs/development/LOCAL_DEV.md`](docs/development/LOCAL_DEV.md).

---

## Quick start (local development)

```bash
# 1. Bring up the backend stack (Postgres, Redis, Rust API)
docker compose up -d db redis api

# 2. Seed an admin user (only first time, or after a fresh DB reset)
./api/scripts/seed-local-admin.sh "you@example.com" "YourStrongPass1!" "Your Name"

# 3. Start the frontend natively (faster than Docker on macOS)
pnpm dev
```

Then open [http://localhost:5173](http://localhost:5173) and log in at
[/auth/login](http://localhost:5173/auth/login). Full runbook in
[`docs/development/LOCAL_DEV.md`](docs/development/LOCAL_DEV.md).

---

## Repository layout

```
.
├── api/                    Rust + Axum backend (PostgreSQL, Redis, R2)
│   ├── src/
│   ├── migrations/         sqlx migrations (run automatically on boot)
│   ├── scripts/
│   │   └── seed-local-admin.sh
│   └── Dockerfile
├── frontend/               SvelteKit 3 + Svelte 5 (runes) + Vite 8
│   └── src/
├── docs/                   Documentation (see below)
├── scripts/                Repo-level utility scripts
├── svelte-click-to-source/ Local workspace package (dev-only ergonomics)
├── docker-compose.yml      Local Postgres + Redis + API
├── pnpm-workspace.yaml     pnpm workspace declaration
└── CLAUDE.md               Repo-specific Svelte-MCP usage rules
```

## Docs

| Where | What |
|-------|------|
| [`docs/development/LOCAL_DEV.md`](docs/development/LOCAL_DEV.md) | Boot the stack from a fresh clone in 10 min |
| [`docs/architecture/SYSTEM_ARCHITECTURE_AND_AUTH.md`](docs/architecture/SYSTEM_ARCHITECTURE_AND_AUTH.md) | Stack reference, public URLs, full auth flow |
| [`docs/audits/`](docs/audits/) | All audits, newest first — start with `PRINCIPAL_ENGINEER_AUDIT_2026-07-19.md` |
| [`docs/setup/`](docs/setup/) | Production deployment + Stripe + R2 setup |
| [`docs/frontend/`](docs/frontend/) | Frontend-specific guides (admin responsive, footer audit, remote-functions migration) |
| [`docs/audits/`](docs/audits/) (incident/forensic reports) | Older incident and forensic reports also live in `docs/audits/` — there is no separate `docs/forensics/` directory |

## Stack

| Layer | Technology | Where it runs |
|-------|-----------|---------------|
| Frontend | SvelteKit 3 / Svelte 5 / Vite 8 | Cloudflare Pages (prod), `pnpm dev` (local) |
| Backend | Rust 1.98 + Axum 0.8 | prod target TBD (Fly.io stripped 2026-04-28), Docker (local) |
| Database | PostgreSQL 18 + sqlx 0.9 | prod target TBD, Docker (local) |
| Cache / sessions / rate-limit / JWT blacklist | Redis 8 | Upstash (prod), Docker (local) |
| Object storage | Cloudflare R2 (S3-compatible) | Cloudflare |
| Search | Meilisearch | Meilisearch Cloud |
| Email | Postmark | Postmark |
| Payments | Stripe | external |
| Video / large file CDN | Bunny.net | Bunny |
| Realtime | WebSockets + SSE | same API process (deploy target TBD) |
| Error tracking | Sentry | Sentry SaaS |

## Development commands

From the repo root:

```bash
pnpm dev              # vite dev server on :5173
pnpm build            # production build
pnpm check            # svelte-kit sync && svelte-check
pnpm test             # vitest
pnpm format           # prettier --write
pnpm lint             # eslint
```

End-to-end tests:

```bash
cd frontend && pnpm test:a11y
```

Backend:

```bash
cd api && cargo check                    # compile
cd api && cargo test --no-run            # compile tests
cd api && cargo test --test utils_test \
                     --test stripe_test  # run no-DB tests
```

## Status of the system (2026-08-14, post SvelteKit 3 migration)

| Gate | Status |
|------|--------|
| Frontend typecheck (svelte-check) | ✅ 4,847 files / 0 errors / 0 warnings |
| Frontend strict typecheck (`check:strict`, `--fail-on-warnings`) | ✅ 0 / 0 |
| Frontend a11y compiler gate (`check:a11y`) | ✅ 0 / 0 |
| Frontend lint (eslint) | ✅ 0 errors (4 pre-existing `no-useless-escape` warnings) |
| Frontend format (prettier) | ✅ clean |
| Frontend unit (vitest) | ✅ 2,272 passed / 32 skipped |
| Frontend production build + prerender + service worker | ✅ |
| Frontend a11y suite (playwright) | ⚠️ 1 pre-existing failure — see BACKLOG.md |
| Backend `cargo fmt` + `clippy -D warnings` | ✅ clean |
| Backend `cargo deny check` + `cargo machete` | ✅ clean / no unused deps |
| Backend no-DB tests (router smoke, utils, stripe, crypto) | ✅ 38 pass |
| Production deploy | ⚠️ target TBD (Fly.io stripped 2026-04-28) |

> SvelteKit 3 is pinned to the `3.0.0-next` prerelease line — there is no stable
> 3.0.0 release yet. See [BACKLOG.md](BACKLOG.md) "SvelteKit 3 follow-ups".

Current priorities live in
[`docs/audits/PRINCIPAL_ENGINEER_AUDIT_2026-07-19.md`](docs/audits/PRINCIPAL_ENGINEER_AUDIT_2026-07-19.md)
(the earlier 2026-04-25 series is historical; its top blockers have since been
fixed or superseded).

## Contributing

Code style is enforced via `prettier` and `eslint` for the frontend, `rustfmt` and
`clippy` for the backend. Before opening a PR:

```bash
pnpm check && pnpm lint && pnpm --filter revolution-svelte test:unit
cd frontend && pnpm build && pnpm test:a11y
cd api && cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings
cd api && cargo deny check && cargo test --test utils_test --test stripe_test
```

`cargo deny check` and `cargo machete` (wrapped together as `pnpm api:lint`)
need a one-time `cargo install cargo-machete cargo-deny`.

Commits should land cleanly through both gates. CI replicates these checks via
`.github/workflows/`.
