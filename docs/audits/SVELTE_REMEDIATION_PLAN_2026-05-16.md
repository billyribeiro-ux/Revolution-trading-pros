# Svelte Frontend Remediation Plan — Phased

**Created:** 2026-05-16 · **Source of truth:** [SVELTE_FRONTEND_AUDIT_2026-05-16_2146.md](SVELTE_FRONTEND_AUDIT_2026-05-16_2146.md) · **Base commit:** `ee7eb2e76`

Each phase = one feature branch + one PR. No commit/push without explicit per-change approval (HARD RULE). Every phase ends with the 4 gates green and runtime evidence where the change type demands it.

---

## Phase 0 — Safety net (do first, ~30 min)

Goal: make the rest measurable and reversible.

- Branch `audit/phase-0-safety-net` off `main`.
- Capture baseline runtime evidence so every later phase can prove improvement:
  - `pnpm check` → archive `0/0` baseline.
  - `npx eslint . -f json -o docs/audits/eslint-baseline-2026-05-16.json` (committed as the diff target).
  - Lighthouse/Playwright CLS run on 3 revenue pages (`/checkout`, `/checkout/thank-you`, `/dashboard/classes`) → baseline CLS numbers saved.
- **DoD:** baseline files committed; CLS numbers recorded. No source code touched.

---

## Phase 1 — P1 correctness (small, high-value, ~1–2 hrs)

Goal: zero eslint errors + close the one traced data-loss path.

Branch `audit/phase-1-p1-correctness`.

1. `frontend/src/lib/server/auth.ts:79,101` — resolve `no-undef 'App'` (add the ambient `App.*` reference / lint env, not an `eslint-disable`).
2. `frontend/src/routes/(dev)/workbench/ComponentTree.svelte:27,28` — drop `$state()` wrapper around `SvelteSet` (already reactive).
3. `frontend/src/routes/admin/indicators/create/+page.svelte:287` — re-throw with `{ cause }` preserved; **remove** the dead `eslint-disable` at `:201`.
4. `frontend/src/lib/consent/templates/TemplateEditor.svelte:28-30` — replace the prop→state `$effect` clobber with a `{#key editingTemplate.id}` remount in parent `frontend/src/routes/admin/consent/templates/+page.svelte:290-292` (one-time init; typing never overwritten).

- **Gates:** `pnpm check` (0/0), eslint error count `5 → 0`.
- **Runtime evidence (required):** Playwright spec — open banner editor, type into a field, trigger a parent re-render, assert the typed value survives. CLAUDE.md cite: "parent-passed prop without proper sync" landmine.
- **DoD:** eslint errors = 0; data-loss Playwright spec green; PR cites the rule.

---

## Phase 2 — `$effect` → `$derived` collapse (mechanical, ~2–3 hrs)

Goal: eliminate the 36-file effect-malpractice class; each change verified by `mcp__svelte__svelte-autofixer`.

Branch `audit/phase-2-effect-to-derived`. Split into 2 PRs (≤18 files each) for reviewability.

- 2a — confirmed sites: `admin/categories/+page.svelte:86/139/146` (collapse `$state`+`$effect`, rename existing `$derived` → `stats`), `admin/schedules/+page.svelte:106/342`, `forgot-password/+page.svelte:19`, `ImageComparisonSlider.svelte:41`, `dashboard/explosive-swings/+page.svelte:107`, `CookieConsent.svelte:68`.
- 2b — the remaining ~30 from the 36-file grep pool, audited individually (skip any where the effect has a real side-effect, not pure derivation).
- Per file: run autofixer until `issues: []` AND no "assigned inside an $effect" suggestion.

- **Gates:** `pnpm check` 0/0; autofixer clean per touched file.
- **Out of scope:** any effect doing real side-effects (DOM, fetch, subscriptions) — leave untouched.
- **DoD:** assignment-in-effect grep pool 36 → ≤ (legit-side-effect remainder), each documented.

---

## Phase 3 — `{@html}` XSS surface (security, ~3–4 hrs)

Goal: prove the 81-hit / 35-file `{@html}` surface is bounded and safe (fintech-critical).

Branch `audit/phase-3-html-xss`.

- For each of the 35 files: classify the bound expression — (a) sanitized (`sanitizeBlogContent`/DOMPurify), (b) constant/trusted-config, (c) raw user/DB input (= **bug, fix**).
- Route any (c) through the existing `$lib/utils/sanitize` util.
- Add per-line `// audited 2026-05: <reason>` and flip `svelte/no-at-html-tags` to **error** in eslint config so the surface can never silently grow.

- **Evidence (required):** a table in the PR — file:line → classification → sanitizer call cited.
- **DoD:** rule is `error`; eslint passes; every `{@html}` has an audited-disable with cited reason.

---

## Phase 4 — API-layer type safety (largest, ~1–2 days, splittable)

Goal: kill `any` in the backend-contract layer (1179 total; epicenter `src/lib/api/*`).

Branch family `audit/phase-4-api-types-<file>`. One PR per file to keep diffs reviewable:
`admin.ts` (63) → `forms.ts` (49) → `crm.ts` (35) → `subscriptions.ts` (34) → `client.svelte.ts` (32) → `coupons.ts` (28) → remainder.

- Type responses against the Rust backend contract. **Money rule:** any `*_cents` field is `number` in TS (i64 end-to-end) — verify against backend structs, never widen to `any`.
- `subscriptions.ts`/`coupons.ts` touch Stripe money — highest care, do with backend struct open side-by-side.

- **Gates:** `pnpm check` 0/0 after each file; eslint `any` count strictly decreasing per PR.
- **DoD:** `src/lib/api/*` `any` count → 0; money fields verified `i64`-consistent.

---

## Phase 5 — UX hardening (CLS + native dialogs, ~1 day)

Branch `audit/phase-5-ux-hardening`. Two PRs.

- 5a — CLS: add `width`/`height` or wrapper `aspect-ratio` to dimensionless `<img>` (86 files). Order: revenue paths first (`/checkout`, `/checkout/thank-you`, dashboard). **Evidence:** Lighthouse/Playwright CLS before/after vs Phase-0 baseline — must show measurable drop.
- 5b — native dialogs: replace `confirm()`/`alert()` with existing `ConfirmationModal`, **destructive admin actions first** (`admin/seo/redirects:62,92`, `admin/email/templates:101`, `admin/indicators:70`). 57 files; batch by admin section.

- **DoD:** CLS numbers improved with evidence; destructive-action native confirms = 0.

---

## Phase 6 — Cleanup / consistency (P3, ~half day)

Branch `audit/phase-6-cleanup`.

- Triage 478 `no-console` (strip from shipped non-test code; keep in tests if intentional) + 294 `!` non-null assertions (replace with guards where they guard real nullables).
- Icon library decision: consolidate the 32 `@lucide/svelte` files onto `@tabler/icons-svelte-runes`, OR formally document the split with a lint rule. **Needs your decision before starting** (bundle vs. migration cost).

- **DoD:** `no-console` in `src/` (non-test) = 0; icon decision recorded.

---

## "onom" — BLOCKED, awaiting input

You confirmed "onom" is a typo for a specific package but haven't named it. **Not in any phase until you provide the real package name.** I will not guess a dependency to bump on a fintech codebase.

---

## Cross-cutting rules (every phase)

- One phase = one branch = one (or few) PR(s). Never push to `main` directly (fintech app, CLAUDE.md hard rule).
- 4 gates green before any commit: `pnpm check`, `pnpm test:unit`, Playwright e2e (chromium), `cargo check` (unaffected here but run for safety).
- Re-read `git diff --staged` like a reviewer before each commit; cite the governing CLAUDE.md rule in the commit message.
- No commit/push without explicit per-change approval from you.
- Runtime evidence (Playwright/Lighthouse/eslint-count-delta), not "tests pass", closes every phase.

## Suggested execution order

`0 → 1 → 2 → 3 → 5 → 4 → 6` (security Phase 3 before the long Phase 4; UX Phase 5 is independent and can run in parallel by a second branch).
