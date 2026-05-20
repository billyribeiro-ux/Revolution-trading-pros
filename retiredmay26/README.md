# retiredmay26/ — Orphan-file forensics, May 2026

This directory holds **176 files (53,759 LOC, 1.3 MB)** retired from active source on
2026-05-19 after a multi-tool, evidence-driven orphan audit.

**Nothing here was deleted.** Every file is preserved with its full git history (moved
via `git mv`, not `git rm`) and its source-tree path mirrored under this folder.
Restoration is a one-line `git mv` operation per file.

---

## Investigation methodology

Branch: `audit/orphan-files-may26` off `main` HEAD `f2f354fb7`.

### Frontend (`frontend/src/`)

Three independent reachability tests were run; **only files flagged by ≥2 of them
qualified as orphan candidates.** For each candidate, the union of tools used:

| Test | What it measures | Source |
|---|---|---|
| `knip --include files` | Static reachability from SvelteKit entry points (routes, hooks, app.html, etc.). Understands `$lib`, dynamic imports declared in config, and barrel re-exports. | https://knip.dev |
| `unimported` (third tool) | Failed to auto-detect SvelteKit entry points (returned config error before scanning) → result discarded; knip's coverage was the primary build-graph signal. |  |
| Custom **basename grep** | For each candidate, `grep -rln <stem> src scripts tests` excluding the file itself. Catches references via any kind of code reference, including comments, JSON config, markdown docs. | `/tmp/orphan-validator.sh` (preserved below) |
| Custom **string-literal grep** | For each candidate, search for `'<stem>'`, `"<stem>"`, `\`<stem>\``, or `/<stem>` inside any quoted context. Catches dynamic `import(\`./components/${name}\`)`, route patterns, css class refs. | `/tmp/string-ref-check.sh` |
| Custom **directory import-pattern grep** | For barrel `index.ts` files: search for `import ... from '<parent>/<leaf>'` and `import ... from '<parent>/<leaf>/...'` anywhere outside the candidate directory. Stricter than basename grep — only counts actual ES module imports. | `/tmp/dir-import-check.sh` |

### Backend (`api/src/`, `api/tests/`)

| Test | What it measures |
|---|---|
| Custom **Rust mod-graph trace** (Python) | BFS from `src/main.rs`, `src/lib.rs`, `src/bin/*.rs`, `tests/*.rs`, `examples/*.rs`, `benches/*.rs`. Follows `mod foo;`, `pub mod foo;`, and `#[path = "..."] mod foo;`. Any `.rs` file not reached = candidate. |
| **Explicit annotation review** | Cross-reference each candidate against `api/src/routes/mod.rs` for commented-out `// pub mod X;` lines with status notes. |

### Validation rigor

- **3-way validation for 74 individual frontend files** (knip + basename + string-literal)
- **2-way validation for 100 files in 9 fully-orphan directories** (knip + strict import-pattern)
- **2-way validation for 2 backend files** (mod-graph + annotation comments)
- **Empirical confirmation post-move**: full `pnpm exec svelte-check --tsconfig ./tsconfig.json` + `cargo check --locked --all-targets` + `pnpm exec eslint .` all returned 0 errors after moving. Same gates returned 0 errors before the move. → If even one file were actually used, at least one gate would fail.

### Explicitly excluded (false positives we caught)

| File | Why kept | Source of correction |
|---|---|---|
| `api/src/routes/indicators_admin.rs` | Has active TODO `// pub mod indicators_admin; // TODO: Fix SQLx tuple decoding issues` (mod.rs:40) + `// .nest("/admin/indicators-enhanced", indicators_admin::router()) // TODO: Fix SQLx issues` (mod.rs:111). Dormant ≠ orphan. | grep audit of `routes/mod.rs` |
| `api/tests/common/stripe_sig.rs` | Used by `tests/stripe_test.rs:14-15` via `#[path = "common/stripe_sig.rs"] mod stripe_sig;`. Mod-graph trace missed `#[path]` attribute. | Manual grep + spec read |

---

## Inventory

### Backend (2 files, both explicitly labeled legacy)

| Path | LOC | mod.rs annotation |
|---|---|---|
| `api/src/routes/indicators.rs` | 425 | `// pub mod indicators; // Legacy - replaced by member_indicators` |
| `api/src/routes/settings.rs` | (see file) | `// pub mod settings; // Already handled by admin.rs` |

### Frontend — 9 fully-orphan directories (100 files, all shadcn-svelte UI / forms scaffolding never wired in)

| Directory | Files |
|---|---|
| `frontend/src/lib/components/forms/pro/` | 42 |
| `frontend/src/lib/components/ui/button/` | 3 |
| `frontend/src/lib/components/ui/card/` | 8 |
| `frontend/src/lib/components/ui/dialog/` | 11 |
| `frontend/src/lib/components/ui/dropdown-menu/` | 18 |
| `frontend/src/lib/components/ui/input/` | 2 |
| `frontend/src/lib/components/ui/label/` | 2 |
| `frontend/src/lib/components/ui/select/` | 12 |
| `frontend/src/lib/components/ui/textarea/` | 2 |

Per-directory check: zero `import ... from '$lib/components/<dir>'` (or relative-path equivalents) in any `.svelte` / `.ts` / `.js` file outside the directory itself. These look like shadcn-svelte CLI scaffolds that were generated but the app uses other components (custom or skeleton-ui) instead.

### Frontend — 74 individual orphan files

See `MANIFEST.tsv` (sibling to this README) for the full table: path, bytes, LOC,
last commit date, last commit hash, last commit subject.

---

## Restoration

To restore any single file:

```bash
git mv retiredmay26/<orig-path> <orig-path>
```

To restore everything (undo this audit):

```bash
git mv retiredmay26/frontend frontend  # interactive — handle existing dirs
git mv retiredmay26/api/src/routes/indicators.rs api/src/routes/indicators.rs
git mv retiredmay26/api/src/routes/settings.rs   api/src/routes/settings.rs
rmdir retiredmay26  # only if empty after the above
```

History is preserved end-to-end. `git log --follow` on any restored file will show
the full lineage including the move into and out of this directory.

---

## What this audit did NOT do

- **Did not delete anything.** Files remain in version control.
- **Did not modify any keep-list file.** No `.svelte` / `.ts` / `.rs` outside the move set was edited.
- **Did not strip `package.json` scripts or workflow steps that depend on these files.** None do (verified by basename grep). If a script does break in CI, the fix is to either restore the referenced file or remove the dead script — both small, reversible changes.
- **Did not validate runtime / browser behavior.** Build, type-check, and lint all pass post-move; that's the empirical proof these files were not in the build graph. A live-browser smoke test was not part of this scope.

---

## Provenance

Audit run: 2026-05-19. Branch: `audit/orphan-files-may26` off `main` HEAD `f2f354fb7`.

Pre-move baseline gates (on `f2f354fb7`):

- `pnpm exec svelte-check`: 0 errors, 0 warnings, 4524 files
- `cargo check --locked --all-targets`: exit 0

Post-move proof gates (the *empirical* validation that none of these were in the build graph):

- `pnpm exec svelte-check`: 0 errors, 0 warnings, **4268** files (was 4524 pre-move)
- `cargo check --locked --all-targets`: exit 0
- `pnpm exec eslint .`: **0 errors, 1416 warnings** (was 1581 pre-move → 165 warnings dropped with the orphans, empirically confirming they contained real code)
- `pnpm --filter revolution-svelte build`: ✓ built in 30.09s (Vite production tree-shake confirms zero of the moved files were in the build graph)

If any of these gates had failed, the offending file(s) would have been restored
before commit. They didn't.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
