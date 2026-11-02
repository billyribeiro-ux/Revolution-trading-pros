---
description: Adversarial verification of a completed /seo-audit run — assume it is wrong, prove or disprove every claim, fix confirmed defects, repeat until two consecutive clean passes
---

# MISSION

A previous Claude session ran `/seo-audit` on this repository and reported it finished. **Your job is to prove that report wrong.**

Treat every line of `docs/seo/audit/08-report.md`, every commit on the audit branch, and every PASS row as an **unverified claim made by someone else**. You confirm a claim only by reproducing it yourself in this session. Anything you can't reproduce is a defect.

You are not here to be agreeable. A run that finds zero defects is acceptable **only** if every check below was actually executed and its output recorded.

---

# RUN CONDITIONS

- **Start in a fresh session.** The previous session's reasoning must not be in your context; only files on disk count. If this session also performed the audit, STOP and tell the owner to run `/clear` or open a new session first.
- Read `docs/seo/SEO_STANDARD.md` and `.claude/commands/seo-audit.md` in full. **All rules in seo-audit.md's "NON-NEGOTIABLE EVIDENCE RULES" apply here unchanged**: evidence tags, no memory-as-evidence, installed version wins, no fabricated content, no cheating the gates, and owner decisions belong to the owner.
- Set inputs:
  - `BASE_REF` = the commit before the audit branch began (find it with `git merge-base`)
  - `AUDIT_REF` = the HEAD of the audit branch
  - `VERIFY_PASS` = 1 (incremented each loop)
- All output goes to `docs/seo/verify/pass-<N>/`.

---

# CHECK 1 — CLEAN-ROOM REPRODUCTION

1. `git worktree add ../seo-verify-<N> <AUDIT_REF>` and work **only** in that worktree, so nothing from local caches or uncommitted state leaks in.
2. Run `pnpm install --frozen-lockfile`, `pnpm check`, `pnpm lint`, `pnpm test`, `pnpm seo:test`, and `pnpm build`. Record the exit code, **warning count**, and test counts (passed/failed/skipped) for each.
   - Any warning, failure, or skipped test introduced since `BASE_REF` is a DEFECT.
3. Re-run the full audit tooling (`pnpm seo:audit` or the Phase 3 scripts) from scratch into `docs/seo/verify/pass-<N>/artifacts/`. Don't reuse any artifact the previous run produced.
4. **Recompute every number in `08-report.md` yourself** from your fresh artifacts: URLs audited, rows per status, FAIL counts. Any mismatch is a DEFECT, even a small one.

# CHECK 2 — CLAIM-BY-CLAIM FALSIFICATION

For **every** row in the final matrix that is PASS, PASS-STRUCTURE, WARN, or N-A:

- Open the cited evidence:
  - for `[CODE]`, open the file at that line and confirm it says what's claimed
  - for `[CMD]`, re-run the command
  - for `[HTTP]`, re-request the URL
  - for `[DOC]`, re-fetch the page
- Mark the row:
  - `CONFIRMED` — you reproduced it
  - `DEFECT` — the evidence is missing, stale, misquoted, or contradicted
  - `UNVERIFIABLE` — it can't be reproduced in principle; this is a DEFECT if the row was PASS
- **N-A rows:** prove the template or feature really doesn't exist (e.g. no forum routes in the inventory) before confirming N-A.
- **REQUIRES-HUMAN / REQUIRES-PRODUCTION rows:** confirm that each one genuinely can't be verified from the repo, and that the owner action list gives an exact, actionable step.

Write `docs/seo/verify/pass-<N>/claims.md`: row → verdict → your evidence.

# CHECK 3 — SOURCE RE-VERIFICATION

1. Re-fetch **every** URL in `docs/seo/audit/01-sources.md` and the live Google Search Gallery. For each one, confirm the recorded paraphrase still matches the live page and that nothing changed since the audit.
2. Pick 10 requirement IDs at random (list the seed you used) and verify each against its primary source **independently of the standard**: Svelte MCP `get-documentation` for `SK:*`, WebFetch for `G:*`. If the standard itself is wrong, record `STANDARD-DEFECT` with a citation.
3. Check the installed versions: for every SvelteKit/Svelte API the new code uses, confirm it exists in the **installed** package's `.d.ts` in `node_modules` (`[CODE node_modules/...:line]`). A docs page alone is not enough.

# CHECK 4 — ANTI-GAMING DIFF REVIEW

Run `git diff <BASE_REF>..<AUDIT_REF>` and `git log --stat <BASE_REF>..<AUDIT_REF>`. Every occurrence of the following, unless justified in `DECISIONS.md` with evidence, is a DEFECT:

- `test.skip`, `.only`, `.fixme`, `test.fail`, deleted or weakened `expect(...)` calls, or reduced assertion counts in existing tests
- lowered numbers in unlighthouse, LHCI, or Playwright budgets, timeouts, or thresholds
- prerender `handleHttpError` / `handleMissingId` / `handleUnseenRoutes` set to `'warn'` or `'ignore'`
- `any`, `as unknown as`, `@ts-ignore`, `@ts-expect-error`, `eslint-disable`, `// TODO`, `FIXME`
- **literal values in JSON-LD builders** (ratingValue, reviewCount, price, author name, credentials, dates) that don't come from a real data source
- a sitemap built from a hand-written URL list instead of the route inventory and data sources
- origins hardcoded anywhere except `src/lib/seo/config.ts` (and its env var)
- more than one place emitting `<title>`, `<meta>`, canonical, or JSON-LD (grep `<svelte:head>` across `src/`)
- `ssr = false` or `csr = false` on indexable routes without a matching requirement
- changes to unrelated files (visual design, business logic, auth, payments) not tied to a requirement ID
- dependencies added without a stated, demonstrated benefit; upgrades that weren't requested

Write `docs/seo/verify/pass-<N>/diff-review.md`.

# CHECK 5 — MUTATION TESTING (prove the tests actually test)

A test that can't fail proves nothing. In a scratch branch inside the worktree, apply **each** mutation below **one at a time**. Run `pnpm seo:test` after each and confirm **the named test fails**, then revert. A mutation that doesn't cause a failure is a DEFECT: the gate is fake.

| #   | Mutation                                                          | Must fail       |
| --- | ----------------------------------------------------------------- | --------------- |
| M1  | Remove the canonical tag from `Seo.svelte`                        | URL-02          |
| M2  | Give two pages the same title                                     | META-01         |
| M3  | Make a missing-slug `load` return data instead of `error(404)`    | IDX-05          |
| M4  | Put `</script><script>alert(1)</script>` in a JSON-LD field       | SD-02 unit test |
| M5  | Remove one breadcrumb item from the DOM only                      | SD-05           |
| M6  | Add `<priority>0.8</priority>` to sitemap output                  | MAP-03          |
| M7  | Add a sitemap URL that returns a 404                              | MAP-01          |
| M8  | Remove `X-Robots-Tag` for non-production hosts                    | IDX-09          |
| M9  | Add `loading="lazy"` to the LCP image                             | PERF-03         |
| M10 | Emit a second `<meta name="description">`                         | META-05         |
| M11 | Remove `rel="ugc"` from forum user links (if forum exists)        | SPAM-06         |
| M12 | Set `ssr = false` on an indexable layout                          | IDX-01          |
| M13 | Remove `isAccessibleForFree` from a gated page (if SD-13 applies) | SD-13           |
| M14 | Remove `hreflang` `x-default` (if multilingual)                   | I18N-03         |
| M15 | Disallow `/_app/` in robots.txt                                   | IDX-07          |

Then add **at least 5 mutations of your own** targeting requirements this repo actually implements. Write `docs/seo/verify/pass-<N>/mutations.md` with each mutation, the test that caught it, and the failure output excerpt. Reset the scratch branch afterwards and confirm with `git status`.

# CHECK 6 — NO REGRESSIONS (function and visuals)

1. **Visual:** with Playwright, screenshot one URL per template at 390 px and 1440 px on `BASE_REF` and on `AUDIT_REF`. Any pixel diff above 0.1% outside the intended changes (e.g. a newly visible breadcrumb UI) is a DEFECT until explained by a requirement ID. Store the images and diff images.
2. **Functional:** run every pre-existing test plus a smoke test of the critical user flows. Navigation, auth/login, checkout or subscription, search, and forum posting, wherever each exists. Do it on both refs. Any behavior difference not tied to a requirement ID is a DEFECT.
3. **Performance:** compare the Lighthouse lab metrics (LCP, CLS, TBT) per template on `BASE_REF` and `AUDIT_REF`. Any template that got worse by more than run-to-run noise (median of 3 runs) is a DEFECT.

# CHECK 7 — FUTURE-PAGE STANDARDS

1. `CLAUDE.md` begins with `@AGENTS.md`, and `AGENTS.md` exists.
2. Every file path in `AGENTS.md` exists. Every command runs and exits 0. Every requirement ID exists in the standard. Every version matches the lockfile.
3. **Dogfood test.** In the scratch branch, create one new page of the most common template, **following only `AGENTS.md`**: a new article, say, or whatever the repo's main content type is. Don't consult the existing implementation. Run `pnpm seo:test` and the gate commands. If the new page fails any requirement, `AGENTS.md` is incomplete: DEFECT. Delete the scratch page afterwards.

# CHECK 8 — PRODUCTION (only if the owner says the branch is deployed)

Against `PRODUCTION_ORIGIN`:

- re-run the crawler
- check the redirect chains (HTTP→HTTPS, host) and HSTS
- confirm robots.txt and the sitemap are live
- confirm `X-Robots-Tag` is **absent** on production indexable URLs and present on preview
- fetch with a Googlebot user agent vs a browser user agent and diff the responses
- query the PSI/CrUX API if a key exists

Field Core Web Vitals need about 28 days of data after deploy; until then they stay REQUIRES-PRODUCTION.

---

# FIX LOOP

1. Write `docs/seo/verify/pass-<N>/verdict.md`:
   - totals per verdict
   - every DEFECT, with evidence, severity (blocker/major/minor), and the requirement ID
2. If there are **zero** DEFECTs and all 8 checks ran, the pass is CLEAN.
3. If there are DEFECTs:
   - fix each one in the main working tree (not the worktree), following seo-audit.md Phase 5's rules: autofixer, gate commands, one commit per defect group, message `fix(seo-verify): <IDs>`
   - for STANDARD-DEFECTs, propose the exact edit to `SEO_STANDARD.md` and apply it only after the owner approves
4. Increment `VERIFY_PASS` and **run all checks again from CHECK 1** in a new worktree.
5. **Stop condition:** **two consecutive CLEAN passes.** If pass 3 still has DEFECTs, STOP and escalate to the owner with the remaining defects. Don't loop forever, and don't relax a check to get a clean pass.

# FINAL OUTPUT

Write `docs/seo/verify/SUMMARY.md` containing:

1. Each pass's verdict table.
2. Every defect found, how it was fixed, and the commit.
3. The mutation results table, showing every mutation caught.
4. Visual, functional, and performance regression results.
5. The remaining owner actions, carried over and re-validated.
6. The final gate-command outputs.

Don't write "everything is perfect." Write what was proven and how.
