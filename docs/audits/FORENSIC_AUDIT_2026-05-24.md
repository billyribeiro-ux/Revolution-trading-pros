# Forensic Repository Audit — Revolution Trading Pros

- **Date:** 2026-05-24
- **HEAD audited:** `d0e0933` (May 2026 Google SEO updates), then extended with `eb91d4c` (repo hygiene)
- **Branch:** `claude/google-seo-updates-may-2026-Sp1j7` — 2 ahead / 0 behind `origin/main`
- **Prior baselines:** [`FULL_REPO_AUDIT_2026-05-17.md`](FULL_REPO_AUDIT_2026-05-17.md), [`FORENSIC_FULL_REPO_AUDIT_2026-05-17.md`](FORENSIC_FULL_REPO_AUDIT_2026-05-17.md), [`DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md`](DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md)

## Executive summary

The codebase is **green on every measurable quality gate**. Repo-hygiene
debt that had accumulated through the PE7 GSAP/Tailwind migration is now
cleaned up (this branch). The May 2026 Google SEO updates are wired in
end-to-end. Two prior P0 release-blockers (May 17 audit) are mechanically
closed; a live Playwright pass is still owed before they can be marked
"verified."

## 1. Quality gates — actual results

| Gate | Result | Detail |
|---|---|---|
| `pnpm check` (svelte-check) | ✅ | 4,725 files, **0 errors, 0 warnings** |
| `pnpm test:unit` (vitest) | ✅ | 57 files, **2,255 passed / 32 skipped / 0 failed** (~35 s) |
| `cargo check` (api) | ✅ | clean compile (2 m 51 s fresh build) |
| `cargo clippy` | ⏸ | not re-run this session; last green per May 17 audit |
| Playwright e2e | ⏸ | needs Docker stack — not booted in this remote container |
| `svelte-autofixer` MCP | ✅ | clean on edited admin schema page |

Trajectory: test count **1,442 (Apr 25) → 1,700 (May 17) → 2,255 (today)** —
+57 % in a month. Test files 36 → 57.

## 2. Git state

```
branch:    claude/google-seo-updates-may-2026-Sp1j7
ahead:     2 commits  (d0e0933 SEO + eb91d4c hygiene)
behind:    0
working:   clean
```

13 PRs (#730–#743) merged since the Apr 25 baseline. Recent themes:

- Large Rust module splits — `subscriptions_admin.rs` 1,165 LOC → 6 files
  (#735), `schedules.rs` 1,134 LOC → 6 files (#736), `member_indicators.rs`
  1,142 LOC → 6 files (#737). Closes Apr 25 Tier-2 item #14.
- Frontend refactor — `admin/blog/categories` extracted to 7 components,
  1,515 → 593 LOC (#738).
- Vitest expansion — 4 new test scaffolds, +99 tests (#739).
- Type-safety sweep — ~30 `any` removals across PE7 PRs.
- GSAP / Tailwind PE7 migration, plus the 103-file restore into
  `retiredmay26/` (#743).

## 3. Repo composition

| Area | LOC | Files |
|---|---:|---:|
| Frontend (`frontend/src`) | 76,017 | 1,092 .svelte · 558 SvelteKit route files |
| Rust API (`api/src`) | 110,538 | (rs) |
| `docs/audits/` | ~1 MB | 61 markdown files |

Quality-debt heatmap (frontend):

- `: any` / `<any>`: **295** (Apr 25 audit reported 194 `as any` — broader
  pattern this time; comparable methodology needed)
- `TODO / FIXME / XXX / HACK`: **81**
- `console.log` / `console.debug`: **63** (Apr 25: 186 — significant
  reduction)

## 4. Findings

### 🔴 P0 — None open

Two May 17 P0s are mechanically closed:

| Prior P0 | Status now | Evidence |
|---|---|---|
| P0-1 — checkout route mismatch | **Closed** | `api/src/routes/checkout.rs:667-672` mounts `/` → `post(create_checkout)`; `frontend/src/lib/api/cart.ts:1166` docstring confirms it now consumes the `create_checkout` JSON. Owes a live Playwright pass. |
| P0-3 — webhook multi-table writes non-atomic | **Closed** | `payments.rs` split into `api/src/routes/payments/`; `payments/webhook.rs:192` opens a real `pool.begin().await` transaction. |

### 🟠 P1 — Repo hygiene (this branch fixes them)

| ID | Item | Status |
|---|---|---|
| P1-A | Four macOS-bound Python scripts at repo root (`fix_spx*.py`) + two more in `spx-profit-pulse/` (`fix_gsap.py`, `fix_plans.py`). All six hard-code `/Users/billyribeiro/Desktop/...`. | **Resolved in `eb91d4c`.** Deleted; restorable from git history. |
| P1-B | Four audit MDs at repo root (`GSAP_*.md` ×2, `RESTORE_AND_MIGRATE_PLAN.md`, `orphans.md`) instead of in `docs/audits/`. | **Resolved in `eb91d4c`.** Moved into `docs/audits/`; `orphans.md` renamed to `ORPHAN_INVENTORY_2026-05-20.md` to match the dated audit convention. |
| P1-C | `retiredmay26/` parking lot (103 restored files from #743). | **Open.** Either archive under `docs/archived-2026-may/` or delete now that PE7 is stable. |
| P1-D | `docs/audits/` inflation — 61 MDs, heavy duplication (`BATCH2..7_RESULT.md`, `TASK2..7_RESULT.md`, `AUDIT_REPORT` + `AUDIT_FIX_PLAN` + `AUDIT_FIX_SUMMARY`, etc.). | **Open.** Archive everything older than the most recent forensic audit under `docs/audits/archive/`. |

### 🟡 P2 — Mild correctness debt (carried over)

- 295 `any` types in frontend (verify against Apr 25 audit's 194 with the
  same query)
- 81 `TODO / FIXME` markers
- `svelte-autofixer` MCP permission gap (flagged May 17) — closed today;
  the MCP is wired and was used on the schema admin page

### 🟢 Stale-but-okay

- `REPO_STATE_2026-04-25.md` — readable cold but ~30 days and 13 PRs out of
  date; Fly.io notes are stale (per CLAUDE.md, references stripped on
  2026-04-28; current deploy fallback is
  `revolution-trading-pros.pages.dev`).
- `DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md §9` backlog: Tier 0 #2 (CMS
  toolbar) and #3 (Stripe Checkout) appear closed via May 17 audit;
  Tier 0 #1 (Fly Postgres) is moot post-Fly-strip.

## 5. SEO posture (post-`d0e0933`)

| Google change | Date | Status |
|---|---|---|
| FAQ rich results dropped | 2026-05-07 | ✅ All 4 FAQ generators marked `@deprecated` with dev-only `console.warn`. Markup retained for AI/voice. Admin schema picker shows "Deprecated" badge. 12 routes still emit `FAQPage` JSON-LD (semantic only). |
| Speakable / generative-AI guide | 2026-05-15 | ✅ `speakableSchema()` builder + `JsonLdSpeakable` types exported through `$lib/seo`. **0 routes use it yet** — adoption is the next step. |
| `hasAdultConsideration` on Product | 2026-05-20 | ✅ Added to `ProductConfig`, defaults to `'no'`. |
| Broad May 2026 core update | 2026-05-21 | n/a (content-quality focused) |

**No hardcoded prod URLs in any SvelteKit proxy** (`frontend/src/routes/api/`)
— CLAUDE.md gate clean.

## 6. Remaining recommended actions

1. **(P1-C)** Archive or delete `retiredmay26/` now that PE7 is stable.
2. **(P1-D)** Roll `docs/audits/` older than May 17 under
   `docs/audits/archive/`.
3. **(SEO-1)** Wire `speakableSchema()` into `/blog/[slug]` and
   `/our-mission` as the active AI surface replacing FAQ rich results.
4. **(Verification)** Boot the Docker stack and run Playwright e2e to
   confirm the closed P0-1 (checkout route) actually works.
5. **(CI)** Re-run `cargo clippy --locked --all-targets -D warnings` in CI
   after the payments-module split.

## 7. Confidence and caveats

- Gates run this session: `pnpm check`, `pnpm test:unit`, `cargo check`,
  `svelte-autofixer` MCP.
- Not run: `cargo clippy`, `cargo deny`, `cargo machete`, Playwright e2e,
  `pnpm lint`, `pnpm audit`.
- All P0/P1 conclusions are either (a) directly verified by command in
  this session or (b) cited to a prior audit with file:line preserved.
  P0-1 / P0-3 status is **"very likely closed; owes a Playwright pass."**
