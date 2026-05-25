# Retired PE7 migration scripts (2026-05-23)

One-shot Python regex patchers used by the PE7 (Tailwind v4 / GSAP)
migration. They are retained for forensic reference per repo policy
("never delete retired files"); they are NOT runnable on this machine.

## Why they were retired

Every script in this folder hard-codes the absolute path
`/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/...`,
i.e. a single contributor's macOS desktop. They will refuse to run on
any other workstation, on CI, or in the cloud agent environment. They
fired once during the PE7 migration sweep and were never meant to be
re-run.

## Forensic provenance

| File | Original location | Recovered from | sha256 verified |
|---|---|---|---|
| `fix_spx.py` | repo root | commit `eb91d4c^` (= `ff7f510`) | ✓ |
| `fix_spx_all.py` | repo root | commit `eb91d4c^` | ✓ |
| `fix_spx_complete.py` | repo root | commit `eb91d4c^` | ✓ |
| `fix_spx_now.py` | repo root | commit `eb91d4c^` | ✓ |
| `fix_gsap.py` | `frontend/src/routes/alerts/spx-profit-pulse/` | commit `eb91d4c^` | ✓ |
| `fix_plans.py` | `frontend/src/routes/alerts/spx-profit-pulse/` | commit `eb91d4c^` | ✓ |

Added: commit `ff7f510` — _"docs(gsap+tailwind): add 4 audit/plan docs + fix_spx.py script for PE7 migration tracking"_

Originally deleted: commit `eb91d4c` — _"chore(repo): delete one-shot PE7 migration scripts + relocate root audit MDs"_. That deletion was reversed in the very next session per the maintainer's standing rule "we never delete retired files; move them into the retired folder."

## Caller / reference audit

`git grep -nE "fix_spx|fix_gsap|fix_plans"` across the repo (excluding
`docs/audits/` and `CHANGELOG.md`) returns **zero references** — these
scripts were one-shot, not called by any other code path.
