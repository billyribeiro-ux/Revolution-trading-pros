# GitHub Actions Workflows

This directory contains exactly two workflows. There is no production
auto-deploy from Actions: the Cloudflare Pages dashboard Git integration
builds and deploys every push to `main` and is the source of truth for
production.

## 1. `ci.yml` — CI gates (blocking)

**Triggers:** push and pull_request to `main` / `develop`, plus manual
`workflow_dispatch`. Concurrency cancels in-progress runs per ref.

**Job `frontend`** (Node 24.18.0, pnpm activated by Corepack from
`package.json#packageManager`):

1. `pnpm install --frozen-lockfile`
2. Typecheck — `pnpm --filter revolution-svelte check`
3. Lint — eslint, blocking, 0 errors required
4. Unit tests — vitest
5. Production build (with `API_BASE_URL` / `BACKEND_URL` set to
   `http://localhost:8080` for build-time SSR/prerender fetches)

**Job `backend`** (Rust 1.96.0, pinned via `dtolnay/rust-toolchain@master`
and `rust-toolchain.toml`; runs in `api/` with `SQLX_OFFLINE=true`):

1. `cargo fmt --all --check`
2. `cargo clippy --locked --all-targets -- -Dwarnings`
3. `cargo check --locked --all-targets`
4. No-DB tests — `cargo test --locked --test router_smoke_test
   --test utils_test --test stripe_test`

ci.yml references no secrets.

## 2. `deploy-cloudflare.yml` — secondary frontend deploy

**Triggers:**

- `workflow_dispatch` with a `branch` input (default `preview`) — manual
  deploy of a specific branch/commit
- `pull_request` to `main` / `develop`, path-filtered to `frontend/**`,
  this workflow file, and `pnpm-lock.yaml` — deploys a `pr-N` preview

**Job `build-and-deploy`:** same Node/pnpm setup and production build as
CI, then `cloudflare/wrangler-action@v4.0.0` runs
`pages deploy .svelte-kit/cloudflare` against project
`revolution-trading-pros` with the resolved branch (`pr-N` for PRs,
otherwise the dispatch input or `preview`).

**Secrets:** `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` — see
[`../SECRETS.md`](../SECRETS.md). If either is missing, a gate step skips
the deploy with a notice instead of failing.

## What is intentionally NOT here

- **No production auto-deploy** — handled by the Cloudflare Pages
  dashboard Git integration.
- **No backend deploy workflow** — backend deploy target is TBD.
- **No E2E in CI** — the previous `tests/e2e/` Playwright suite was
  removed 2026-05-19 after surfacing real backend issues that need triage
  against the running app, not via a CI black box. A new suite will be
  authored once those are fixed. Playwright can still be run locally (see
  CLAUDE.md gates).
- **No Lighthouse, cache purge, or Slack notifications.**

## History

Fly.io deployment (backend) was removed 2026-04-28; the deploy target has
been TBD since. The e2e CI job was removed 2026-05-19 along with the old
suite. Any older docs mentioning `deploy-fly.yml`, staging/production
auto-deploys, Lighthouse CI, cache purging, or Slack webhooks describe
workflows that no longer exist.

## Troubleshooting

- **"Skipping deploy" notice** — add the two Cloudflare secrets.
- **Frontend job OOM** — the job sets
  `NODE_OPTIONS=--max-old-space-size=6144`; see TECHNICAL_NOTES.md.
