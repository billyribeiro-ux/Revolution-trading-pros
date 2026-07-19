# Contributing

Quick reference for working on this repo. Read [`docs/development/LOCAL_DEV.md`](docs/development/LOCAL_DEV.md) first if you haven't booted the stack yet.

## Branch + commit conventions

- Branch from `main`. Use a topic prefix matching the change type:
  - `feat/<short-name>` — new functionality
  - `fix/<short-name>` — bug fix
  - `chore/<short-name>` — repo plumbing / deps / tooling
  - `docs/<short-name>` — docs only
  - `refactor/<short-name>` — code change without behavior change
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  ```
  feat(auth): add MFA verification step at login
  fix(cms): toolbar add-block click handler regressed by bits-ui v2
  chore(deps): bump @sveltejs/kit to ^2.58
  ```
  The scope (`auth`, `cms`, `deps`) is the area touched.
- Body should explain **why**, not what. The diff already shows what.
- Co-author lines from pair programming or AI assistants are appreciated.

## Required local checks before opening a PR

```bash
# Frontend
pnpm check                                                # typecheck
pnpm --filter revolution-svelte test:unit                 # vitest
cd frontend && pnpm test:a11y                             # playwright a11y suite

# Backend
cd api
cargo check                                               # compile
cargo test --test utils_test --test stripe_test           # no-DB unit tests
```

A PR that breaks any of these will not pass CI. The complete backend
integration tests (`cargo test`) need a running Postgres; CI runs them
against a service container.

## Code style

| Language | Tool | Config |
|----------|------|--------|
| TypeScript / Svelte | Prettier 3 | `frontend/.prettierrc` (or root) |
| TypeScript / Svelte | ESLint 10 | `frontend/eslint.config.js` |
| Rust | rustfmt | defaults (no `rustfmt.toml` in the repo) |
| Rust | clippy | `cargo clippy --locked --all-targets -- -D warnings` |
| All files | EditorConfig | `.editorconfig` (this repo's tabs/spaces rules) |

Don't manually format; let the tools do it. Most editors auto-format on save when EditorConfig is detected.

## Svelte rules

This repo uses **Svelte 5 runes** (`$state`, `$derived`, `$props`, `$bindable`). Don't introduce legacy patterns:

- ❌ `let foo = $state(props.foo ?? null); $effect(() => …)`
- ✅ `let { foo = $bindable(null) } = $props();`

CLAUDE.md contains the project-specific Svelte MCP tooling rules (use the
`svelte-autofixer` MCP tool when writing components).

## When you add a new SvelteKit `+server.ts` proxy

It's tempting to copy a hardcoded production URL into `const API_URL`. **Don't.** The PE7 pattern is:

```ts
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';
```

This way the proxy works against:
- local Docker (`API_BASE_URL=http://localhost:8080` from `frontend/.env.local`)
- the Cloudflare Pages env in production (`API_BASE_URL` set in dashboard)
- the Fly fallback for the rare case env vars are unset

See [`docs/development/ENV_VARS.md`](docs/development/ENV_VARS.md) for the full env-var matrix.

## When you change the database schema

1. Add a numbered migration in `api/migrations/NNN_description.sql`. Idempotent (`CREATE TABLE IF NOT EXISTS`, `ADD COLUMN IF NOT EXISTS`) is preferred so re-runs are safe.
2. Run the API container — migrations apply automatically on boot via sqlx.
3. Re-generate the offline `sqlx-data` so CI builds without a DB:
   ```bash
   cd api && cargo sqlx prepare
   ```
4. Commit `api/.sqlx/` along with the migration.

## What's NOT in the active build

`frontend/Do's/` and `frontend/blogsample` are reference / design-mockup material kept in tree (`retiredmay26/` and `frontend/Implementation/` were removed 2026-07-19 — recover from git history if needed)
for historical context. They are:

- **Tracked** in git (so they're not lost)
- **Excluded** from `frontend/tsconfig.json` (so svelte-check / TypeScript ignore them)
- **Excluded** from the active import graph (so the production bundle never picks them up)

Don't grep these for code patterns when auditing — they will produce false
positives. Real code lives under `frontend/src/**`.

## When you touch `seed-local-admin.sh` or any shell that talks to Postgres

Use `psql -v key=value <<'SQL' … :'key' …` parameter substitution. Do **not** inline shell variables into SQL strings — that's an SQL injection vector even in scripts that only run locally.

## Documentation

- New audit reports go in `docs/audits/` with the date in the filename.
- New runbooks go in `docs/development/`.
- Setup guides go in `docs/setup/`.
- Architecture docs go in `docs/architecture/`.
- Postmortems go in `docs/audits/` with the date in the filename (there is no separate `docs/forensics/` directory).

The top-level `README.md` only links to canonical entry points; resist the
urge to dump audit content there.

## Pull request checklist

- [ ] Branch is up-to-date with `main`
- [ ] Commits follow Conventional Commits
- [ ] All four gates pass locally (typecheck, vitest, playwright, cargo)
- [ ] No new hardcoded production URLs (use `env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080'` pattern)
- [ ] No new `as any` casts in TypeScript without justification in a comment
- [ ] No new `unwrap_or_default()` on `Result<T, E>` in Rust without justification
- [ ] No secrets committed (run `git diff` against `frontend/.env.production` and `api/.env*`)
- [ ] If you added a new env var, it's documented in [`docs/development/ENV_VARS.md`](docs/development/ENV_VARS.md) and added to both the example file and (if needed) the production file
- [ ] If you touched the auth flow, you tested it against the local Docker stack
