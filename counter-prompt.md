# COUNTER-PROMPT — Independently verify `report.md` (the remediation report). Change nothing unless told.

You are a verifier, not a fixer. `report.md` claims all 16 audit items were fixed and gated. The
commit is `7f6a0f9df` ("fix(admin): resolve backend & frontend audit findings across all 4 tiers",
137 files, +4644/−2700). Rules: `CLAUDE.md`, `AGENTS.md`, `~/.claude/CLAUDE.md`. Every claim you make
needs a locator or command output. If you cannot reproduce a claim, say `NOT-REPRODUCED`.
**Do not edit, commit, push, revert or force anything.** Read-only plus throwaway DB/Redis only.

## 1. Problems already seen in the report (confirm each, with evidence)

1. **Process breach.** `HEAD == origin/main == 7f6a0f9df`: the 137-file change is committed AND pushed
   directly to `main`. The report says it came from branch `fix/audit-followup-2026-11` "squashed &
   merged"; no such branch exists locally. The instructions were: branch first, no push until asked.
   Establish what actually happened (`git reflog`, `git branch -a`, `git log --all`, `gh pr list
   --state all --limit 5`). Report only; the owner decides about any revert.
2. **Fabricated locators.** These cited paths do not exist: `api/src/utils/pagination.rs`,
   `api/src/utils/jwt.rs`, `api/src/routes/room_content.rs`, `api/src/routes/auth.rs`,
   `api/src/routes/admin_members.rs`, `api/src/routes/subscriptions.rs`. The helper
   `normalize_pagination` appears nowhere in the repo (pagination was fixed inline with
   `clamp`/`saturating_*`). `ON CONFLICT (token_hash) DO NOTHING` was not found. The report also says
   "SvelteKit 2" (repo is SvelteKit 3). For EVERY row in report §1 and §2, replace the cited locator
   with the real `file:line` of the change (use `git show 7f6a0f9df -- <file>`), or mark it
   `NO-SUCH-CHANGE`.
3. **"Eliminated silent error masking" is overstated.** At HEAD: `grep -rn "unwrap_or_default()"
   api/src/routes | wc -l` = 76 and 59 route files still mention `NaiveDateTime`. Classify each
   remaining `unwrap_or_default()` on a sqlx `Result` as harmless (e.g. string/JSON default) or a
   masked-DB-error bug, and list the bugs. Same for `NaiveDateTime` on `TIMESTAMPTZ` columns
   (check the column type in `api/migrations/schema.sql`, not by memory).
4. **Findings from the audits that the report never mentions.** Mark each FIXED / NOT-FIXED / NOT-A-BUG
   with evidence: global SvelteKit circuit breaker (`routes/api/[...path]/+server.ts`); `config.ts`
   sending `/api/admin/*` straight to the cross-origin API; `/admin/products` `formatPrice(product.price)`;
   `/admin/memberships` null `features`; money still `f64`/`Decimal` on abandoned-carts, `/api/coupons`,
   crm contacts/deals; repeat-`DELETE` returning 200 on campaigns/coupons/products; plan creation under
   6-way concurrency (`subscriptions_admin/plans.rs`, note the real handler is there, not
   `subscriptions.rs`); 60 CMS/page-layout routes extracting `Json<T>` before the role check; the
   `/setup-db` removal-or-gate decision (owner never answered it).
5. **Claim vs code on specific fixes** (read the diff, don't trust the prose):
   - Trade close (`room_content/trades.rs` ~333-392): is the guard one atomic
     `UPDATE … WHERE status='open' RETURNING` with `rows_affected()==0 → 409`, or a read-then-write?
   - Refresh `jti`: `Claims.jti` is a non-optional `String` at `api/src/utils/mod.rs:70`, not the
     `Option<String>` the report describes. Does `refresh_token` rotation still blacklist by token hash
     such that a same-second refresh no longer self-blacklists? Prove with the test at
     `tier1_followup_test.rs:174` actually exercising two refreshes in one second (not just checking
     `jti` is non-empty).
   - `tier1_5` (pagination, `tier1_followup_test.rs:260`): it asserts `200` for four URLs. Does it
     cover `page=-1&per_page=-5` on courses and indicators, and the "search=%00" case?
   - Migration `072_audit_followup_schema_alignment.sql`: forward-only and idempotent? Is every
     `ALTER … ADD COLUMN IF NOT EXISTS`, `CREATE … IF NOT EXISTS`? Does it run cleanly twice, on top
     of `schema.sql`? Does `schema.sql` now contain the same changes without duplicating them
     (it contains `ADD COLUMN IF NOT EXISTS target_audience` at ~line 22003 AND the column at 21885 –
     explain)? Any backfill (`ROUND(discount_value*100)::BIGINT`) safe on rows with NULL or huge values?
   - Were shadow-router tests really rewired to `common::setup_test_app()`, and do they still assert
     something meaningful (compare assertions before/after per file with `git diff`)? Count tests
     removed vs kept; the diff shows large deletions (cms_v2_test −340 lines).
   - `MockConnectInfo` added in `api/tests/common/mod.rs`: does this mask the real client-IP code path
     in prod (`SmartIpKeyExtractor`)? Say what the test does and does not prove.
6. **Gate claims.** The report says check/lint/vitest/build/fmt/clippy/cargo test all exited 0. Re-run
   them yourself ONCE each and paste exit codes + the last 5 lines of output:
   - frontend: `pnpm --filter revolution-svelte check`, `lint`, `test:unit`, and `build`
   - api: `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`
   - api tests: the full suite against a FRESH DB built from `api/migrations/schema.sql` +
     `sqlx-seed.sql` (Postgres `rtp-db` container, Redis **db 9**, `--test-threads=4`); report
     passed/failed/ignored per the earlier baseline of 847 passed / 0 failed / 7 ignored (state if that
     number changed and why).
   - Also run `cargo deny check` only if `Cargo.lock`/`Cargo.toml` changed in the commit.

## 2. Spot-check the live behaviour (fresh stack, API on a spare port, tokens via real login)

For these, show the raw status code before/after as far as you can (`git stash` is forbidden – use a
second worktree at `60288659d` for the "before"):

- no token / member token / admin token against: `GET /api/admin/cms-v2/{settings,menus,menus/1,workflows}`,
  `GET /api/admin/room-content/rooms/<slug>/{alerts,trades,stats}`.
- `POST /api/admin/members` (and `/api/admin/member-management`): row exists in `users` afterwards.
- `POST /api/admin/subscriptions/plans` with `price_cents:-5000`, blank name, duplicate name, and 6
  concurrent identical creates.
- `GET /api/admin/{courses,indicators,popups}?page=999999999&per_page=100000` and `?page=-1&per_page=-5`.
- login → refresh → refresh inside one second.
- 10 random routes the first audit called `LIVE-FAIL` and 10 it called `LIVE-OK` (seeded random, seed
  in report) — did the fix change the result? Any new regressions?

## 3. Browser pass (Playwright, headless Chromium, `page.on('dialog', d => d.accept())`)

Open each page the report says it fixed: `/admin/popups/new`, `/admin/popups/[id]/edit`,
`/admin/site-health`, `/admin/settings`, `/admin/media`, `/admin/orders`, `/admin/memberships`,
`/admin/subscriptions/plans`, `/admin/products/[id]/edit`, `/admin/coupons/edit/[id]`,
`/admin/users/edit/[id]`, `/admin/courses/create`, `/admin/members/service/[id]`. With a populated DB.
Record console errors, the text `NaN`, and a screenshot into `verify-evidence/`. Do NOT test blog
pages (out of scope; a replacement editor is being built elsewhere).

## 4. Deliverable: `verify-report.md` at the repo root

Sections: (1) verdict per report row — `CONFIRMED` / `PARTIAL` / `NOT-REPRODUCED` / `MISLOCATED`;
(2) corrected locator table; (3) the unmentioned-findings table from §1.4; (4) gate results with exit
codes; (5) live spot-check table; (6) browser table; (7) regressions found; (8) the git-history
finding from §1.1, stated neutrally with the commands you ran; (9) honest gaps; (10) teardown
commands (print, don't run). Final chat message: path + ≤ 15-line summary.
