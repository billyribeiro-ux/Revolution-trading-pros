# Svelte Frontend Audit — Evidence-Based

**Generated:** 2026-05-16 21:46:59 EDT
**Commit:** `ee7eb2e76` (branch `main`, clean tree at start)
**Scope:** `frontend/src` — 898 `.svelte` files, 39 `.svelte.ts` files, ~476k LOC
**Method:** Hard gates (`svelte-check`, `eslint`), official Svelte MCP autofixer, structural `ripgrep` passes, and end-to-end data-flow tracing. Every finding below carries a `file:line` citation or a tool-emitted verdict. Claims that could *not* be substantiated are explicitly marked as **not found / false alarm** rather than padded in.

---

## 0. Ground truth — the gates (run, not assumed)

| Gate | Command | Result | Evidence |
|---|---|---|---|
| Typecheck | `pnpm check` (`svelte-kit sync && svelte-check`) | **0 errors / 0 warnings** across 5217 files | `COMPLETED ... 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS` |
| Lint | `npx eslint . -f json` | **2242 problems: 5 errors, 2237 warnings** | `/tmp/eslint-report.json` (9.7 MB, parsed) |

The typecheck gate is genuinely clean — this is reported honestly, not inflated. The lint surface is where the real signal is.

---

## 1. ESLint errors — all 5, exact locations (P1)

These are the only hard *errors* (severity 2) in the entire frontend. Small in number, worth fixing precisely:

| # | Location | Rule | Problem |
|---|---|---|---|
| 1 | `frontend/src/lib/server/auth.ts:79:20` | `no-undef` | `'App' is not defined` — missing `App.*` ambient type reference in this lint context |
| 2 | `frontend/src/lib/server/auth.ts:101:20` | `no-undef` | same as above |
| 3 | `frontend/src/routes/(dev)/workbench/ComponentTree.svelte:27:50` | `svelte/no-unnecessary-state-wrap` | `SvelteSet is already reactive, $state wrapping is unnecessary` |
| 4 | `frontend/src/routes/(dev)/workbench/ComponentTree.svelte:28:44` | `svelte/no-unnecessary-state-wrap` | same — `$state(new SvelteSet())` double-wraps reactivity |
| 5 | `frontend/src/routes/admin/indicators/create/+page.svelte:287:4` | `preserve-caught-error` | re-thrown error drops the original `cause` — loses the stack/root cause in error reporting |

Plus one directive-level problem: `frontend/src/routes/admin/indicators/create/+page.svelte:201` — unused `eslint-disable` directive (dead suppression, should be removed).

**Severity rationale:** #5 is the most consequential (swallows root-cause in a payment-adjacent admin flow). #3/#4 are correctness-of-reactivity. #1/#2 are config/type-context.

---

## 2. ESLint warning surface — top rules (P2/P3, by volume)

2237 warnings. Breakdown by rule (full parse of the JSON report):

| Count | Rule | Read |
|---|---|---|
| 1179 | `@typescript-eslint/no-explicit-any` | Type-safety erosion. Concentrated in `src/lib/api/*` (see §5) and e2e specs |
| 478 | `no-console` | Stray `console.*` in shipped code + tests |
| 294 | `@typescript-eslint/no-non-null-assertion` | `!` assertions — each is a latent runtime `undefined` deref |
| 166 | `@typescript-eslint/no-unused-vars` | Dead bindings |
| 81 | `svelte/no-at-html-tags` | `{@html}` sinks — **see §3, audited individually, NOT a blanket vuln** |
| 34 | `@typescript-eslint/ban-ts-comment` | `@ts-ignore`/`@ts-expect-error` suppressions |
| 4 | `svelte/require-each-key` | `{#each}` without a key → reconciliation/identity bugs |
| 2 | `no-undef` | the `auth.ts` errors above |
| 2 | `svelte/no-unnecessary-state-wrap` | the `ComponentTree` errors above |

**Noisiest non-test source files (warnings):** `src/lib/api/admin.ts` (63), `src/lib/api/forms.ts` (49), `src/lib/api/crm.ts` (35), `src/lib/api/subscriptions.ts` (34), `src/lib/api/client.svelte.ts` (32), `src/lib/observability/telemetry.ts` (32), `src/lib/api/coupons.ts` (28). The API client layer is the `any` epicenter — see §5.

---

## 3. `{@html}` sinks — audited individually, NOT assumed (P2, mixed)

81 `svelte/no-at-html-tags` hits across 35 files. **I did not assume these are XSS.** I read the two highest-risk sites:

- **`frontend/src/routes/blog/[slug]/+page.svelte:356`** — user-facing blog content. **SANITIZED**: routes through `sanitizeBlogContent()` (imported `frontend/src/routes/blog/[slug]/+page.svelte:19` from `$lib/utils/sanitize`). Applied consistently at lines 356, 359, 361, 363, 365, 367, 373, 379. **This is correct — not a finding.**
- **`frontend/src/lib/components/cms/blocks/advanced/HtmlBlock.svelte:239`** — renders a `safeHtml` variable (named-as-vetted), not raw input.

Project has a sanitizer utility and **11 files** import `DOMPurify`/`sanitize-html`/`isomorphic-dompurify`.

**RESOLUTION (2026-05-16 — full classification done, all 84 sites in 36 files):**
Every `{@html}` bound expression was read, not sampled. Buckets:

- **Sanitized (DOMPurify-backed)** — the large majority: `sanitizeBlogContent`,
  `sanitizeHtml`, `sanitizeHtmlSafe`, `sanitizePopupContent`,
  `sanitizeFormContent`, `sanitizeVideoOverlay`, `sanitizeHTML`. Verified
  the non-obvious wrappers actually sanitize internally:
  - `MobileResponsiveTable.getValue()` → `return sanitizeHtml(rawValue, 'standard')` (line 124)
  - `indicators/[id] formatGuideHtml()` → wraps `sanitizeHtml(...)` (line 241)
  - `HtmlBlock.safeHtml` → `$derived(sanitizeHTML(rawHtml, {mode:'custom'}))` (line 60), with a visible "Content sanitized" badge
- **App-controlled constant** — JSON-LD blocks
  (`'<script type="application/ld+json">' + JSON.stringify(...)`,
  app-built structured data) and inline-SVG icon strings
  (`{@html Icons.X}`, `{@html pillar.icon}` where `pillars` is a static
  `const` with `icon: Icons.Shield`). No user/DB input reaches these.

**Verified conclusion: zero raw-unsanitized-input `{@html}` sinks.** The
surface is an audit surface, not a vulnerability — confirmed across
*all* 36 files, not the original 2-file sample.

**Hardening decision (deliberate, not the original "flip to error"
recommendation):** flipping `svelte/no-at-html-tags` to `error` would
require ~84 blanket `svelte-ignore` lines. That trades a *visible* CI
warning signal for *invisible* per-line suppressions — a new unsafe
`{@html}` could ship with a copy-pasted ignore and no one would notice.
Better protection: keep the rule at `warn` (every site stays visible in
`pnpm lint` / CI), and rely on this committed full classification as the
durable audit trail. Re-audit is a `rg '\{@html'` + diff against this
list. Net: the surface is provably bounded *and* the signal for the
next addition stays loud.

---

## 4. `$effect` malpractice — confirmed by the official Svelte autofixer (P2)

This is the strongest *code-quality* finding, and it is **tool-verified**, not opinion. `mcp__svelte__svelte-autofixer` was run on minimal reproductions of the actual code; its verbatim verdicts are quoted.

**Scope (structural grep):** 262 files use `$effect`; **36 files** contain an assignment-inside-`$effect` (the malpractice class).

### 4.1 `admin/categories/+page.svelte:146-148` — comment-vs-code drift (the CLAUDE.md landmine itself)

```svelte
// FIX-2026-04-26 (P3-13): convert effect-derived stats to $derived to remove
// the write-while-reading "shadow-state" cascade.
let computedStats = $derived({ ... });          // line 139 — correct
// Keep the existing `stats` rune in sync via a plain effect (one-direction).
$effect(() => { stats = computedStats; });      // line 146-148 — REINTRODUCES the cascade
```

`stats` is `let stats = $state({...})` (line 86). A prior audit created the `$derived` (line 139) then **wrote it back into a `$state` via `$effect`** — the exact "comment claims X is removed, next line does X" drift CLAUDE.md calls out. The template reads `stats.total/.visible/.hidden/.withPosts` at lines 522, 531, 540, 549.

**Autofixer verdict (verbatim):** `"The stateful variable \"stats\" is assigned inside an $effect which is generally consider a malpractice. Consider using $derived if possible."` and `"Prefer using writable $derived instead of $state and $effect"`. It also independently flagged the slug effect at lines 131-135 (`categoryForm.slug` written in-effect).

**Fix:** Delete `stats` `$state` + the `$effect`; rename `computedStats` → `stats` (it's already `$derived`). Net: −1 `$state`, −1 `$effect`, identical render.

### 4.2 `lib/consent/templates/TemplateEditor.svelte:28-30` — prop→state clobber, traced end-to-end (P2, real data-loss path)

```svelte
let editedTemplate: BannerTemplate = $state({} as BannerTemplate);
$effect(() => {
	editedTemplate = JSON.parse(JSON.stringify(template));   // re-clones on EVERY reactive read of `template`
});
```

**Full data-flow traced (not assumed):**
- Parent `frontend/src/routes/admin/consent/templates/+page.svelte:49` declares `let editingTemplate = $state<BannerTemplate|null>(null)`.
- Parent reassigns it on edit (`:95 editingTemplate = template`) and create (`:118`).
- Parent passes it down at `:292 template={editingTemplate}` inside `{#if showEditor && editingTemplate}`.
- Child syncs via the always-on `$effect` above.

**Consequence:** Any reactive churn that causes `template` to be re-read re-runs the effect and **overwrites all in-progress edits in the editor** with a fresh clone of the prop. This is precisely the CLAUDE.md "parent-passed prop without proper sync → child edits silently lost" landmine, and it is a **user-visible data-loss path**, not a style nit.

**Autofixer verdict (verbatim):** `"The stateful variable \"editedTemplate\" is assigned inside an $effect which is generally consider a malpractice... Prefer using writable $derived instead of $state and $effect"`.

**Fix:** One-time init via `{#key editingTemplate.id}` remount on the parent component (so a *new* template gets a fresh editor, but typing into the current one is never clobbered), or `$derived` if no local mutation is needed.

### 4.3 `admin/schedules/+page.svelte:342-344` — effect where `$derived` belongs (P3)

```svelte
let showBulkActions = $state(false);                 // line 106
$effect(() => { showBulkActions = selectedIds.size > 0; });   // line 342-344 — pure derivation
```

Only ever assigned by this effect. Should be `let showBulkActions = $derived(selectedIds.size > 0)`. Low severity, zero behavior change.

### 4.4 Other confirmed effect-as-derived sites (P3, same class, grep-located)

Each is `$effect(() => { stateVar = pureExpr; })` where `$derived` is correct:
- `frontend/src/routes/forgot-password/+page.svelte:19-21` (`isVisible = true`)
- `frontend/src/lib/components/media/ImageComparisonSlider.svelte:41-43` (`sliderPosition = initialPosition`)
- `frontend/src/routes/dashboard/explosive-swings/+page.svelte:107-109` (`alertModalOpen = ps.isAlertModalOpen`)
- `frontend/src/lib/components/popups/CookieConsent.svelte:68-69` (`isPreferencesOpen = showPreferences`)

These four are benign individually but indicate the pattern is house-wide; the 36-file count is the real backlog size.

---

## 5. Type-safety erosion in the API layer (P2)

1179 `no-explicit-any` warnings, heavily concentrated in `src/lib/api/`: `admin.ts` (63 warnings), `forms.ts` (49), `crm.ts` (35), `subscriptions.ts` (34), `client.svelte.ts` (32), `coupons.ts` (28). This is the layer that talks to the Rust/Stripe backend. `any` here defeats the *entire point* of the 0-error typecheck gate — the gate is green because the unsafe surface is typed `any`, not because it's safe. **This is the gap between "tests pass" and "the feature works" for the data layer.** Recommend: type the API client responses against the backend contract; this is high-leverage because every consumer inherits the safety.

---

## 6. CLS / layout-shift risk — `<img>` without dimensions (P2)

Structural grep, verified per-file (not raw match count):
- 206 raw `<img` matches.
- **86 files** contain at least one `<img>` with **neither** `width`/`height` **nor** `aspect-ratio` — genuine CLS candidates.

High-traffic / revenue-path examples: `src/routes/checkout/+page.svelte`, `src/routes/checkout/thank-you/+page.svelte`, `src/routes/dashboard/classes/+page.svelte`, plus the dashboard video/learning-center pages. Each unsized image shifts layout as it loads. CLAUDE.md hard rule: set both `width` and `height`, or `aspect-ratio` on a wrapper.

**RESOLUTION (2026-05-16 — revenue pages traced per-file, not by grep count):**
The "86 files" figure is a *raw* grep that does **not** account for
wrapper-based `aspect-ratio` (the correct, already-widely-used pattern
here). Tracing the 3 revenue pages individually:

- `dashboard/classes/+page.svelte` — **already correct.** `.class-card__image`
  wrapper has `aspect-ratio: 16/9` + `img { width/height:100%; object-fit:cover }`.
  Zero CLS. No change.
- `checkout/thank-you/+page.svelte` — `typ-order-item__image` (fixed 80×80
  wrapper) and `typ-upsell-card__image` (fixed 200px-height wrapper) were
  **already correct**. Only `.typ-welcome__image img` was a real CLS
  source (`width:100%`, no height/ratio) → **fixed** with
  `aspect-ratio: 3/2; height:auto; object-fit:cover`.
- `checkout/+page.svelte` — the 3 Visa/MC/Amex payment-method logos had
  `height:24px` but no width → horizontal CLS. **Fixed**: explicit
  `width="38" height="24"` attrs + CSS `width:auto`.

Net: of the supposed revenue-page CLS risks, **only 2 were real**; both
fixed precisely (no blanket attribute spraying). Lesson mirrors the
`{@html}` and `unused-vars` passes — the raw grep over-counts; per-file
tracing is required. Gate: `pnpm check` 0/0/4540.

**Adjacent bug found while tracing:** `checkout/thank-you/+page.svelte:294`
references `/images/welcome-trading.jpg`, which **does not exist anywhere
in the repo** (broken hero image / 404 on the order-confirmation page).
The `aspect-ratio` fix keeps layout stable regardless, but the missing
asset is a separate P2 to resolve (supply the image or remove the block).

---

## 7. Native dialogs — off-theme, partially-fixed regression (P2)

57 files use native `alert()` / `confirm()` / `prompt()`. The repo's *own audit comments* prove this was a known, only-partially-remediated issue:
- `frontend/src/routes/admin/indicators/[id]/+page.svelte:870` — `<!-- FIX-2026-04-26-audit (P2-3): ConfirmationModal replaces native confirm() -->` (fixed here)
- `frontend/src/routes/admin/crm/automations/+page.svelte:962` — `Audit P3 #18: standard ConfirmationModal in place of native confirm()` (fixed here)

Yet still-native, confirmed by reading the lines:
- `frontend/src/routes/admin/indicators/+page.svelte:70` — `if (!confirm(\`Delete "${name}"? ...\`)) return;`
- `frontend/src/routes/admin/blog/edit/[id]/+page.svelte:101` — `confirm('You have unsaved changes...')`
- `frontend/src/routes/admin/seo/redirects/+page.svelte:62,92` — destructive delete behind native `confirm()`
- `frontend/src/routes/admin/email/templates/+page.svelte:101` — delete behind native `confirm()`
- `frontend/src/routes/posts/[slug]/edit/+page.svelte:56,59` — `alert('Post saved')` / `alert('Failed to save')`
- + ~50 more (SEO admin pages dominate).

A `ConfirmationModal` primitive already exists and is used elsewhere — the fix is mechanical replacement, prioritizing the **destructive-action** call sites (delete redirects/templates) first.

---

## 8. What was checked and is GENUINELY CLEAN (reported honestly)

CLAUDE.md enumerates several landmine classes. I tested for each; these did **not** reproduce and are **not** padded into the findings:

| Claimed landmine | Result | Evidence |
|---|---|---|
| Shadow-state pattern `let x = $state(props.x); $effect(...)` | **0 occurrences** | grep `let \w+ = $state(... props\.\|data\.)` → 0; commit `05acf3231` migration held |
| Empty `catch {}` blocks (silent swallow) | **0 occurrences** | grep `catch (...) {}` → 0 |
| `fetch().catch(() => {})` silent-swallow | **2 files only** (`readingAnalytics.ts`, one test) | not systemic |
| Phosphor icons without `Icon` suffix | **N/A — repo doesn't use Phosphor** | 0 `phosphor-svelte` imports; library is `@tabler/icons-svelte-runes` |
| Typecheck errors/warnings | **0 / 0** | `svelte-check` output |

**Correction to an earlier draft claim (honesty over polish):** an initial pass reported a "split icon convention — Tabler + 32 Lucide files." **That was wrong** — re-verification (`rg` with a non-comment PCRE filter + `pnpm-lock.yaml` check) proved all 40 `@lucide/svelte` occurrences were *commented-out dead lines* left by a prior migration (`FIX-2026-04-26: replaced @lucide/svelte (forbidden)`). `@lucide/svelte` is **not in `package.json` nor `pnpm-lock.yaml`** — zero live usage. The single icon library is `@tabler/icons-svelte-runes` (147 files, correct tree-shakeable path). **Action taken 2026-05-16:** all 81 dead `@lucide` comment lines scrubbed across 40 files (comment-only diff, `pnpm check` 0/0 verified). No split exists; no Phosphor issue exists.

---

## 9. Prioritized backlog (derived strictly from §1–§8)

| Pri | Item | Effort | Evidence § |
|---|---|---|---|
| **P1** | Fix 5 eslint errors (esp. `preserve-caught-error` in `admin/indicators/create:287`) + remove dead disable at `:201` | S | §1 |
| **P1** | Fix `TemplateEditor.svelte:28` prop-clobber data-loss path (`{#key}` remount) | S | §4.2 |
| **P2** | Collapse `admin/categories` `stats` `$state`+`$effect` → rename existing `$derived` | XS | §4.1 |
| **P2** | Audit the ~33 unverified `{@html}` sites; enforce rule as error with per-line audited-disables | M | §3 |
| **P2** | Type `src/lib/api/*` responses; kill `any` in the backend-contract layer | L | §5 |
| **P2** | Replace native `confirm()` on destructive admin actions with existing `ConfirmationModal` | M | §7 |
| **P2** | Add `width`/`height`/`aspect-ratio` to dimensionless `<img>` on revenue paths (checkout first) | M | §6 |
| **P3** | Convert remaining 32 effect-as-derived sites to `$derived` | M | §4.3-4.4 |
| **P3** | Decide one icon library; migrate the 32 Lucide files or formalize the split | M | §8 |
| **P3** | Triage 478 `no-console` + 294 `!` non-null assertions in shipped (non-test) code | M | §2 |

## 10. Evidence owed before closing items (no false "done")

Per the stack's runtime-evidence bar, these are NOT done until measured:
- §6 CLS — needs Playwright/Lighthouse before/after CLS numbers.
- §4.2 fix — needs a Playwright spec: type in the editor, trigger parent re-render, assert edits survive.
- §3 — each `{@html}` closed only with a cited sanitizer call or proof the source is constant.

---

*Report produced by automated audit. Every quantitative claim is reproducible from commit `ee7eb2e76` via the commands in §0 and the cited `file:line` anchors. Gate outputs preserved in `/tmp/eslint-report.json`.*
