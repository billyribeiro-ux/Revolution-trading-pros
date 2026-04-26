#!/usr/bin/env bash
# scripts/pe7_gate.sh — verify all PE7 invariants. Block if any fails.
#
# Run before every push to main. Wire into CI as a required check.
# Contract: docs/audits/PE7_CONTROL_PLAN_2026-04-25.md
#
# Exit codes:
#   0 — all 8 invariants pass.
#   1 — one or more invariants failed (see stderr for which).

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

red()    { printf '\033[31m%s\033[0m\n' "$*"; }
green()  { printf '\033[32m%s\033[0m\n' "$*"; }
yellow() { printf '\033[33m%s\033[0m\n' "$*"; }
bold()   { printf '\033[1m%s\033[0m\n' "$*"; }

bold "▸ Frontend typecheck (svelte-check) ..."
pnpm --filter revolution-svelte check

bold "▸ Rust build (locked, release) ..."
(cd api && cargo build --locked --release)

bold "▸ Rust clippy (warnings as errors) ..."
(cd api && cargo clippy --locked --all-targets -- -D warnings)

bold "▸ Rust fmt (check only) ..."
(cd api && cargo fmt --check)

bold "▸ Rust no-DB tests ..."
(cd api && cargo test --test utils_test --test stripe_test)

bold "▸ !important sweep (must be 0) ..."
# Note: grep returns 1 when no matches; `|| true` keeps the pipeline status 0
# under `set -euo pipefail` so a clean tree doesn't abort the gate.
COUNT=$( { grep -rn ': .*!important;' frontend/src --include='*.css' --include='*.svelte' 2>/dev/null || true; } | wc -l | tr -d ' ')
if [ "$COUNT" != "0" ]; then
	red "✗ Found $COUNT !important declarations."
	red "  See frontend/src/lib/styles/IMPORTANT_USAGE.md — repo contract is zero."
	exit 1
fi
green "  0 !important declarations — clean."

bold "▸ Result.unwrap_or_default sweep (informational; manual triage) ..."
RESULT_UNWRAPS=$( { grep -rn '\.unwrap_or_default()' api/src --include='*.rs' 2>/dev/null || true; } \
	| { grep -v -E '(_test\.rs|tests/)' || true; } \
	| wc -l | tr -d ' ')
yellow "  $RESULT_UNWRAPS occurrences in api/src (Option<T> ok; Result<T,E> not — see audit issue #1)"

green ""
green "✓ All PE7 invariants pass."
