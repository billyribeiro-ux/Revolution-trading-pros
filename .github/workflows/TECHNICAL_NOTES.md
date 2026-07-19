# Workflow Technical Notes

Design decisions behind `ci.yml` and `deploy-cloudflare.yml` — the only
two workflows in this repo. Read README.md first for the overview.

## Toolchain pinning

- **Node 24.18.0** via `actions/setup-node`, in both workflows.
- **pnpm** is never installed by version number in the workflows.
  Corepack activates it from `package.json#packageManager` (pnpm@11.5.2),
  so CI, the deploy workflow, and local dev use the exact same pnpm. One
  source of truth, no drift.
- **Rust 1.96.0**, pinned via `dtolnay/rust-toolchain@master` with an
  explicit `toolchain:` input. The `@stable` wrapper of that action
  ALWAYS installs stable and silently ignores `toolchain:` — that is why
  the backend gate previously floated on a moving toolchain and broke
  non-deterministically once clippy became `-D warnings`. The same pin
  lives in `rust-toolchain.toml` so contributors match CI.

## Frontend job memory

GitHub-hosted runners have ~7 GB RAM. This is a very large Vite app
(threlte/three.js, ~1.6k modules); `vite build` and vitest OOM at the
default V8 heap. The frontend job sets a job-level
`NODE_OPTIONS=--max-old-space-size=6144` so build AND vitest get the
raised heap (previously only the lint npm script raised it).

## Build-time backend URLs

Both production-build steps set `API_BASE_URL` / `BACKEND_URL` to
`http://localhost:8080`. A literal placeholder host is an invalid URL for
build-time SSR/prerender fetches; localhost mirrors the proxy default and
produces a clean ECONNREFUSED the pages tolerate, so the cold build stays
green without a backend.

## Backend job

- Runs with `working-directory: api` and `SQLX_OFFLINE=true` (no database
  in CI; sqlx macros compile against the checked-in offline data).
- Gates, all blocking: `cargo fmt --check`, `cargo clippy --locked
  --all-targets -- -Dwarnings`, `cargo check --locked --all-targets`.
- `cargo test --locked --test router_smoke_test --test utils_test
  --test stripe_test` exercises the real `revolution_api` crate.
  `router_smoke_test` constructs the full `api_router()` — catching axum
  route-syntax panics and route conflicts, a failure class `cargo check`
  cannot see.

## Concurrency

Both workflows use `group: workflow-ref` with `cancel-in-progress: true`,
so a new push cancels the superseded run for the same ref.

## deploy-cloudflare.yml specifics

- **Secondary path only.** The Cloudflare Pages dashboard Git integration
  is the primary deploy (every push to `main` → production). This
  workflow covers manual dispatch and backup PR previews.
- **Branch resolution:** PRs deploy to `pr-N`; manual dispatch uses the
  `branch` input; anything else lands on `preview`.
- **Secrets in job-level `env`:** the `secrets` context is only valid in
  `with:` / `env:` bindings, not directly in step `if:` expressions. The
  two Cloudflare secrets are hoisted into job env so a shell gate step
  can test them and emit `ready=true/false`; the wrangler deploy step
  runs only when `ready == 'true'`. Missing secrets therefore skip the
  deploy with a `::notice::` instead of failing the run.
- Deploy command: `wrangler pages deploy .svelte-kit/cloudflare
  --project-name=revolution-trading-pros --branch=<resolved>` via
  `cloudflare/wrangler-action@v4.0.0`, `workingDirectory: frontend`.

## History

Fly.io backend deployment was removed 2026-04-28 (deploy target TBD
since). The Playwright e2e CI job and `tests/e2e/` suite were removed
2026-05-19 pending backend fixes. Older revisions of these notes
described Lighthouse CI, cache purging, Slack notifications, and a
`deploy-fly.yml` — none of that exists in the current workflows.
