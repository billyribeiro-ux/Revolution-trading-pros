# CLAUDE.md — agent instructions for this repo

This file is read by Claude Code (and other MCP-aware agents) when working
in this repo. Keep it short, current, and pointed at canonical sources of
truth.

---

## Read first

| Doc | When |
|-----|------|
| [README.md](README.md) | First time in the repo |
| [docs/development/LOCAL_DEV.md](docs/development/LOCAL_DEV.md) | Booting the local stack |
| [docs/development/ENV_VARS.md](docs/development/ENV_VARS.md) | Setting any env var |
| [docs/audits/FORENSIC_AUDIT_2026-05-24.md](docs/audits/FORENSIC_AUDIT_2026-05-24.md) | Quick "what's the state of this codebase?" |
| [docs/audits/FULL_REPO_AUDIT_2026-05-17.md](docs/audits/FULL_REPO_AUDIT_2026-05-17.md) | Picking the next thing to fix (P0/P1 findings + the prior audit's backlog still apply for P2/P3 work) |
| [docs/audits/REPO_STATE_2026-04-25.md](docs/audits/REPO_STATE_2026-04-25.md) | Stale (~30 days, 13 PRs out of date) but useful for "why is this branch shaped this way?" history |

---

## MCP servers wired into this repo

Two project-scoped MCPs are configured in [`.mcp.json`](.mcp.json) and
auto-load when Claude Code starts in this directory.

### `svelte` — Svelte 5 / SvelteKit official docs + autofixer

Pre-configured globally on most installs. Tools:

- `list-sections` — discover all available Svelte / SvelteKit doc sections
- `get-documentation` — fetch full doc content for one or more sections
- `svelte-autofixer` — analyze Svelte code and return issues + suggestions
- `playground-link` — generate a Svelte Playground URL

**Discipline:** When writing any non-trivial Svelte component, run
`svelte-autofixer` on the result and iterate until it returns no issues.
Use `list-sections` → `get-documentation` to ground every Svelte 5 / Kit 2
question in the official docs before guessing.

### `rust` — cargo + clippy + rustup wrapper

Backed by [`rust-mcp-server`](https://github.com/Vaiz/rust-mcp-server) v0.3.7.
Install once on a new machine:

```bash
cargo install rust-mcp-server
```

Tools (selection):

- **Cargo:** `cargo-build`, `cargo-check`, `cargo-test`, `cargo-clippy`,
  `cargo-fmt`, `cargo-doc`
- **Deps:** `cargo-add`, `cargo-remove`, `cargo-update`, `cargo-search`,
  `cargo-info`
- **Quality:** `cargo-deny-check`, `cargo-machete`, `cargo-hack`
- **Toolchain:** `rustc-explain`, `rustup-show`, `rustup-toolchain-add`,
  `rustup-update`

**Discipline:** Run `cargo-check` and `cargo-clippy` after any edit to
`api/src/`. Do not bypass with `Bash` calls when the MCP tool exists for
the same operation — the MCP tool gives Claude structured output it can
reason about; raw `cargo` output is just text.

### Other (user-scoped, not project-scoped)

`Linear`, `Google Drive`, `MT Newswires`, etc. — these are personal to the
user, not part of this repo's contract.

---

## When you write or edit code

### Svelte 5

This repo uses Svelte 5 runes throughout. **Do not** introduce legacy
patterns:

- ❌ `let foo = $state(props.foo ?? null); $effect(...)` (the shadow-state
  pattern that emits `state_referenced_locally` warnings)
- ✅ `let { foo = $bindable(null) } = $props();`

41 components were migrated off the legacy pattern in commit `05acf3231`.
The repo's typecheck must stay at 0 errors / 0 warnings.

### SvelteKit `+server.ts` proxies

Every proxy under `frontend/src/routes/api/` reads its backend URL from
`$env/dynamic/private`. Match this shape:

```ts
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';
```

**Never** add a new proxy with a hardcoded production URL. The audit found 14 of
these on 2026-04-25 and they were all fixed; don't re-introduce them. The
fallback `'http://localhost:8080'` is for local dev — production deploy target
is TBD (Fly.io references stripped on 2026-04-28).

### Rust API

- All SQL is parameterized via `sqlx` macros. Never use `format!()` to
  build SQL with user-controlled values.
- New mutations that touch >1 table need a `Pool::begin()` → `tx.commit()`
  transaction wrapper.
- Don't swallow errors with `unwrap_or_default()` on `Result<T, E>` —
  propagate via `?`.

### When you finish a change

Run the four gates locally:

```bash
pnpm --filter revolution-svelte check                      # frontend typecheck
pnpm --filter revolution-svelte test:unit                 # vitest
cd frontend && pnpm test:a11y                             # playwright a11y suite
cd ../api && cargo check
cd api && cargo test --test router_smoke_test --test utils_test --test stripe_test  # no-DB tests
```

All four must pass before committing.

---

## When in doubt

1. Re-read [README.md](README.md) and [docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md](docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md) §9 (the backlog).
2. Don't invent new patterns. Match the existing house style — even when
   it's not perfect, consistency beats novelty.
3. Ask before destructive actions (deleting branches, force-pushing,
   running migrations against prod).
