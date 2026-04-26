# PE7 Total Control Plan — 2026-04-25

**Effective immediately. Owner: Billy Ribeiro + Claude (Opus 4.7).**
**Status: enforced.**

This is the contract that defines what "Distinguished Principal Engineer
Level 7+ enterprise grade" means in this repository. Every change — agent
or human — is judged against it. **Anything failing this contract is
blocked from `main`.**

---

## 1. The 8 invariants (cannot regress)

| # | Invariant | How verified |
|---|-----------|--------------|
| 1 | `pnpm check` reports `0 errors / 0 warnings / 0 files-with-problems`. | `pnpm --filter revolution-svelte check` |
| 2 | `cargo build --locked --release` succeeds with 0 warnings. | `cd api && cargo build --locked --release` |
| 3 | `cargo clippy --locked -- -D warnings` succeeds. | `cd api && cargo clippy --locked -- -D warnings` |
| 4 | `cargo fmt --check` is clean. | `cd api && cargo fmt --check` |
| 5 | `cargo test --test utils_test --test stripe_test` is 32/32 green. | `cd api && cargo test --test utils_test --test stripe_test` |
| 6 | Zero `!important` declarations in any CSS or `<style>`. | `grep -rn ': .*!important;' frontend/src --include='*.css' --include='*.svelte' \| wc -l` → must be **0** |
| 7 | `mcp__svelte__svelte-autofixer` returns `{"issues":[],"suggestions":[]}` on every Svelte file edited. | per-file inspection during edit |
| 8 | No new `unwrap_or_default()` on any `Result<T, E>` in `api/src/`. | `grep -rn '\.unwrap_or_default()' api/src \| <inspect each — false-alarm-tolerant for Option<T>>` |

If any of the 8 fails, the change does not merge. No exceptions, no
"we'll fix it later." Fix it before merge or revert.

---

## 2. The "total control" architecture

The user must be able to control every operational behaviour from the
admin UI without a code deploy. That means:

### 2A. Credentials live in the database, not in `.env`

Where credentials currently live in env vars (`STRIPE_SECRET`,
`OPENAI_API_KEY`, etc.), the runtime must:

1. Look up the credential in the `connections` table first.
2. Fall back to env vars only if no DB row exists (preserves local dev).
3. Cache lookups with a short TTL (≤ 60s) so admin changes propagate
   within a minute without a restart.

The pattern is implemented via a `CredentialResolver` service. Every API
client instantiation flows through it. **No raw `env::var("...")` calls
in route handlers.**

Currently in scope:
- ✅ Connections table exists (`api/src/routes/connections.rs`).
- ✅ Admin Settings UI exists (`frontend/src/routes/admin/settings/+page.svelte:265`).
- ⏳ Stripe runtime client to read from DB (in flight, agent
  `a81d843fc45664e75`).
- ❌ Other services (email, OpenAI, etc.) — backlog.

### 2B. Admin actions emit audit events

Every privileged admin action (price change, member subscription
modification, role grant, connection key rotation, etc.) writes a row to
an `admin_audit_log` table with:

- `actor_id` (admin user)
- `action` (machine-readable enum)
- `target_type` + `target_id`
- `metadata` (JSONB — old/new values, request IP, user agent)
- `created_at`

Currently in scope:
- ❌ `admin_audit_log` table does not exist. Backlog item, not blocking.

### 2C. Every multi-table mutation is transactional

Per `CLAUDE.md`'s Rust API rules. See `docs/audits/RUST_BACKEND_AUDIT_2026-04-25.md`
issue #3 — only 2 sites in the repo wrap mutations in `pool.begin()`. Week
2 agent (in flight) closes most of this gap.

### 2D. Every dropped DB error is logged with context

Per the same audit, issue #1. Week 1 agent shipped the highest-impact
fixes (Stripe webhook silent failure). Backlog: sweep remaining 270+ call
sites in waves.

### 2E. Every admin price/inventory mutation has a 3-way grandfathering choice

"New only / Next renewal / Immediate with proration" — applies to
subscription prices, but the *pattern* (admin chooses scope of effect)
applies to every mutation that affects existing customers. Document each
choice in the UI with plain-English tooltips. (Stripe agent in flight
ships this.)

---

## 3. Verification harness — run before every merge

A single script runs all 8 invariants:

```bash
#!/usr/bin/env bash
# scripts/pe7_gate.sh — block if any invariant fails

set -euo pipefail

echo "▸ Frontend typecheck..."
pnpm --filter revolution-svelte check

echo "▸ Rust build (release, locked)..."
(cd api && cargo build --locked --release)

echo "▸ Rust clippy (warnings as errors)..."
(cd api && cargo clippy --locked --all-targets -- -D warnings)

echo "▸ Rust fmt..."
(cd api && cargo fmt --check)

echo "▸ Rust no-DB tests..."
(cd api && cargo test --test utils_test --test stripe_test)

echo "▸ !important sweep (must be 0)..."
COUNT=$(grep -rn ': .*!important;' frontend/src --include='*.css' --include='*.svelte' | wc -l | tr -d ' ')
if [ "$COUNT" != "0" ]; then
  echo "✗ Found $COUNT !important declarations — see frontend/src/lib/styles/IMPORTANT_USAGE.md"
  exit 1
fi

echo "▸ unwrap_or_default on Result sweep..."
RESULT_UNWRAPS=$(grep -rn '\.unwrap_or_default()' api/src --include='*.rs' | grep -v -E '(_test\.rs|tests/)' | wc -l | tr -d ' ')
echo "  ($RESULT_UNWRAPS occurrences total — manual classification of Result vs Option ongoing)"

echo "✓ All PE7 invariants pass."
```

This file is checked into `scripts/pe7_gate.sh`. Run it locally before
any push. Wire into CI as a required check on `main`.

---

## 4. Branch-merge protocol

1. **Agent works in an isolated worktree on a feature branch.** No agent
   commits directly to `main`.
2. **Before merge**, verify:
   - Agent's own report claims clean gates.
   - I run `scripts/pe7_gate.sh` against the branch.
   - Diff is reviewable (≤ 1000 lines preferably; if larger, justified by
     the brief — e.g., the CSS isolation pass).
3. **Merge with `--no-ff`** so the merge commit is visible in `git log`.
4. **Push to origin immediately** so Cloudflare deploys.
5. **Stash any leftover dirty files in my worktree before checkout** —
   parallel-agent territory often overlaps; the agents will deliver
   complete versions that supersede partial state.

---

## 5. Things that are NOT acceptable (zero tolerance)

- **Skipping git hooks** (`--no-verify`, `--no-gpg-sign`).
- **Force-pushing to `main`.**
- **Committing secrets.** Even into `.env.local` — that file is
  gitignored, but if a key ever lands in tracked code, it's a security
  incident.
- **`// TODO`, `// FIXME`, or commented-out code in the diff.** If the
  feature isn't done, scope it down. If the code isn't needed, delete it.
- **Adding back `!important`.** The contract is zero. See
  `frontend/src/lib/styles/IMPORTANT_USAGE.md`.
- **`unwrap()` / `expect()` outside test code or one-shot setup.** Every
  panic vector is a production-incident vector.
- **Mocking out tests to "make them pass"** without the mocks reflecting
  real behaviour.

---

## 6. The role of agents

Agents are the workforce; I am the gatekeeper. Agents go fast and
parallel; I serialize merges to `main` so the integration is clean.

When an agent reports "done", I:

1. Read its report.
2. Switch to its branch.
3. Run `scripts/pe7_gate.sh`.
4. Eyeball the diff for non-obvious deviations from the brief.
5. Merge with `--no-ff` if clean. Send a follow-up message if not.
6. Delete the worktree after successful merge.

I do not blindly trust agent self-reports. The gate script is the source
of truth.

---

## 7. Updating this contract

This file is the canonical PE7 contract. To loosen any invariant requires
a written justification appended below. To tighten one, just do it.

### History

- **2026-04-25** — initial. 8 invariants, total-control architecture
  defined, verification harness scripted, merge protocol established.
