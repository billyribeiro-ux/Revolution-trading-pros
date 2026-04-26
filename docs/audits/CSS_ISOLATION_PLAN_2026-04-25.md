# CSS Isolation Plan — Eliminate `!important` at the Root

**Auditor:** Distinguished Principal Engineer (ICT 7+) agent + Claude (Opus 4.7) consolidation.
**Date:** 2026-04-25 · **Method:** read-only forensic sweep, file:line citations on every claim.
**Trigger:** "as of April 25, 2026, `!important` is unethical, unprofessional, and a cheap patch — fix the root."

This plan is the canonical, deterministic execution path. Each step has its own
file:line target, before/after snippet, verification check, and risk rating.
Execute in order. Don't skip steps. Don't merge steps. Verify gates between each
phase.

---

## 1. Inventory — `!important` count by file (top 10 offenders)

`grep -rn '!important' frontend/src --include='*.css' --include='*.svelte' | wc -l` → **321**.

| File | Count | Primary category |
|------|-------|-----------------|
| `lib/styles/dashboard.css` | 25 | **B** — defensive width/margin/max-width |
| `lib/styles/print.css` | 22 | **C** — print-media color/background (legitimate) |
| `routes/admin/popups/new/+page.svelte` | 20 | **A** — responsive `display:` toggles |
| `lib/components/sections/CoursesSection.svelte` | 19 | **B** — responsive padding fights |
| `routes/dashboard/+layout.svelte` | 15 | **B** — width/margin defenses |
| `routes/courses/+page.svelte` | 13 | **A** — responsive show/hide |
| `routes/dashboard/+page.svelte` | 12 | **B** |
| `lib/components/VideoEmbed.svelte` | 12 | **D** — fullscreen overlay reset (legitimate) |
| `routes/admin/indicators/create/+page.svelte` | 8 | **A** |
| `lib/styles/performance.css` | 8 | **A** — `prefers-reduced-motion` (legitimate) |

### Categories

| Category | Count | Verdict |
|----------|-------|---------|
| **A** — `display: none/block`, `animation: none` for responsive show/hide and `prefers-reduced-motion` | 85 | Mostly fine — convert to `:where()` for zero specificity |
| **B** — `margin/padding/width/max-width/position` defending against cascade leakage | 189 | **Structural defects.** Root cause is global rules in `app.css @layer base { body, * }`. |
| **C** — `color/background/border` for theme/print | 35 | Print = legitimate. Component = defect. |
| **D** — `transform/animation/transition` for overlays/fullscreen | 12 | Legitimate. Document and keep. |

---

## 2. CSS load order

| Order | File | Loaded by | Scope |
|-------|------|-----------|-------|
| 1 | `frontend/src/app.css` | `routes/+layout.svelte:21` | **GLOBAL** — every page |
| 2 | `lib/styles/main.css` (imports `tokens/`, `design-tokens.css`, `base/reset.css`, `base/admin-page-layout.css`) | `admin/+layout.svelte:8` AND `dashboard/+layout.svelte:25` | admin + dashboard |
| 3a | `lib/styles/admin-responsive.css` | `admin/+layout.svelte:9` | admin only |
| 3b | `lib/styles/dashboard.css` | `dashboard/+layout.svelte:26` | dashboard only |
| 4 | per-component scoped `<style>` | Svelte hash-scoped | per file |
| 5 | `print.css` | imported in app.css | global, print media |
| 5b | `performance.css` | imported in app.css | global, accessibility |

**The defect lives at step 1** — `app.css:117-124`'s `@layer base { body, * { ... } }` rules apply to admin/dashboard descendants by inheritance, and admin/dashboard fight back with `!important` (step 2/3).

---

## 3. Five concrete override-fight cases

### Case 1 — `dashboard.css:18-21` (defensive, REMOVE)

```css
.dashboard__main .dashboard__header,
…
header[class*='dashboard__header'] {
    margin: 0 !important;
    width: 100% !important;
    max-width: none !important;
    box-sizing: border-box !important;
}
```

**Defending against:** inherited body cascade. But `.dashboard__main .dashboard__header` is specificity (0, 2, 0); inherited body rules are (0, 0, 1). The class chain wins without `!important`. The flags are cargo-culted defense, not necessary override.

**Action:** Remove all four `!important` flags after Phase 1.

---

### Case 2 — `dashboard.css:251-289` (responsive utilities, CONVERT)

```css
.hidden-xs { display: none !important; }
.hidden-md { display: block !important; }
@media (max-width: 768px) {
    .hidden-xs { display: block !important; }
    .hidden-md { display: none !important; }
}
```

**Defending against:** any component-level `display:` rule.

**Action:** Either drop `!important` (cascade order wins for utilities loaded after components) OR wrap in `:where()` for explicit zero-specificity. **Recommended: `:where()`.**

---

### Case 3 — `print.css:22-27` (print media, KEEP)

```css
@media print {
    * {
        background: white !important;
        color: black !important;
        box-shadow: none !important;
        text-shadow: none !important;
    }
}
```

**Why kept:** print output legitimately needs to override authored colors regardless of theme.

---

### Case 4 — `performance.css:217-228` (accessibility, KEEP)

```css
@media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
        scroll-behavior: auto !important;
    }
}
```

**Why kept:** WCAG 2.1 AAA. Must override component-level animations globally for users with vestibular disorders.

---

### Case 5 — `VideoEmbed.svelte:1863-1876` (fullscreen overlay, KEEP)

```css
.fullscreen-video {
    position: fixed !important;
    top: 0 !important;
    left: 0 !important;
    /* … */
}
```

**Why kept:** fullscreen overlays legitimately need to ignore parent constraints. Same applies to other modal/overlay components.

---

## 4. Root-cause grouping

| Root cause | `!important` count | Fix |
|------------|---------------------|-----|
| Global cascade bleed from `app.css:117-124` `@layer base { body, * }` | ~189 (Category B) | Phase 1: scope to `.marketing-page-root` wrapper, **inside `@layer base`** |
| Responsive display utilities expecting universal override | ~85 (Category A, including responsive subset) | Phase 2: convert to `:where()` (zero specificity) |
| Print media context | 22 | Keep, document as exception |
| Accessibility (`prefers-reduced-motion`) | 8 | Keep, document as exception |
| Fullscreen overlays | 12 | Keep, document as exception |
| **Total kept (justified):** | 42 |
| **Total removed/converted:** | 274 |
| **Reduction:** | **85%** |

---

## 5. Why the previous Option A failed (and how to avoid)

The earlier attempt (commit on f95d2d79d's branch, reverted in `a87d29362`) put the rule **outside** `@layer base`:

```css
/* WRONG: outside @layer base. Specificity (0, 1, 0) for `.marketing-page-root *`
   vs Tailwind utilities like `.border-0` (0, 1, 0) inside @layer utilities.
   Equal specificity → later wins. But layer order trumps specificity:
   `@layer utilities` (Tailwind's) > unlayered (this rule) is FALSE — unlayered
   actually wins over any @layer. So `.border-0` LOST and the login form
   showed unwanted default borders on every box. */
.marketing-page-root * {
    @apply border-border outline-ring/50;
}
```

The fix is to put the rule **inside `@layer base`**:

```css
@layer base {
    /* Inside base layer — Tailwind utilities (in @layer utilities) win at
       cascade-layer level regardless of specificity. */
    .marketing-page-root * {
        @apply border-border outline-ring/50;
    }
    .marketing-page-root {
        @apply bg-background text-foreground;
    }
}
```

Three options were evaluated:

| Option | Approach | Verdict |
|--------|----------|---------|
| **A1** | Wrapper class **inside `@layer base`** | **RECOMMENDED.** Tailwind utilities still win; cleanly scoped. |
| **A2** | `:where(*)` to drop specificity to 0 | Works but applies globally; admin/dashboard rules still need to outspecify. |
| **A3** | Conditional `app.css` import per route | Fragile, SSR hydration risk. Not recommended. |

**Recommendation: A1.**

---

## 6. Implementation plan — deterministic, ordered

### Phase 0 — pre-flight

- [ ] Verify clean working tree: `git status -sb`
- [ ] Run `pnpm check` baseline. Note count.
- [ ] Run `grep -rn '!important' frontend/src | wc -l` baseline. Note 321.

### Phase 1 — scope global rules (file: `app.css` + `+layout.svelte`)

**Step 1.1**: Edit `frontend/src/app.css:117-124`.

Before:
```css
@layer base {
    * { @apply border-border outline-ring/50; }
    body { @apply bg-background text-foreground; }
}
```

After:
```css
@layer base {
    /* Marketing-only defaults. Wrapped in .marketing-page-root so admin /
       dashboard / cms layouts (which don't apply the wrapper) are isolated.
       Inside @layer base so Tailwind utilities in @layer utilities still win
       on layer order. */
    .marketing-page-root * {
        @apply border-border outline-ring/50;
    }
    .marketing-page-root {
        @apply bg-background text-foreground;
    }
}
```

**Step 1.2**: Edit `frontend/src/routes/+layout.svelte` to add the wrapper class on the marketing-only branch (the existing `{:else}` branch in the `if isAdminArea || isEmbedArea` block).

**Verification**:
- `pnpm check` → 0/0/0
- `pnpm dev`, visit `/` → marketing theme intact
- visit `/admin` → no light-theme bleed
- visit `/auth/login` → form boxes have correct borders (no over-bordering regression)
- visit `/dashboard/day-trading-room` → admin-style layout, no light-theme bleed

**Risk**: LOW (single-rule scope change, easy to revert).

### Phase 2 — remove defensive `!important` from `dashboard.css`

**Step 2.1**: `dashboard.css:13-22` — `.dashboard__main .dashboard__header` block. Remove 4 `!important`.

**Step 2.2**: `dashboard.css:25-33` — `.dashboard__content`. Remove 4 `!important`.

**Step 2.3**: `dashboard.css:35-42` — `.dashboard__content-main`. Remove 4 `!important`.

**Step 2.4**: `dashboard.css:54-66` — `.dashboard__header`. Remove 3 `!important`.

**Verification per step**: `pnpm dev`, visit any `/dashboard/<room>` page. Check:
- Header full width with no gaps
- No horizontal scrollbar
- Border/divider visible at bottom of header

**Risk**: MEDIUM. If Phase 1 was wrong, headers may shrink. Test each step before moving to the next.

### Phase 3 — convert responsive display utilities to `:where()`

**Step 3.1**: `dashboard.css:251-289` — wrap selectors in `:where()`.

Before:
```css
.hidden-xs { display: none !important; }
@media (max-width: 768px) {
    .hidden-xs { display: block !important; }
}
```

After:
```css
:where(.hidden-xs) { display: none; }
@media (max-width: 768px) {
    :where(.hidden-xs) { display: block; }
}
```

**Verification**: dashboard pages still hide/show correctly at 768px breakpoint.

**Risk**: LOW.

### Phase 4 — repeat Phase 2/3 patterns across other files

Files to process (priority order):
1. `routes/dashboard/+layout.svelte` (15 `!important`)
2. `routes/dashboard/+page.svelte` (12)
3. `routes/admin/popups/new/+page.svelte` (20 — most are Category A)
4. `lib/components/sections/CoursesSection.svelte` (19)
5. `routes/courses/+page.svelte` (13)
6. `routes/admin/indicators/create/+page.svelte` (8)

For each: classify each `!important`, then either remove (Category B), convert to `:where()` (Category A), or document and keep (Category C/D).

**Risk**: HIGH if rushed. **Process one file at a time, verify between each.**

### Phase 5 — document the legitimate exceptions

Create `frontend/src/lib/styles/IMPORTANT_USAGE.md` with a list of every remaining `!important`:
- File:line
- Reason
- Date approved

The document is the contract — any future `!important` not listed in it should fail review.

### Phase 6 — final verification

```bash
# Should be ~42 (down from 321)
grep -rn '!important' frontend/src --include='*.css' --include='*.svelte' | wc -l

# Should be 0/0/0
pnpm --filter revolution-svelte run check

# Visual regression test on critical routes
pnpm dev
# Open: /, /auth/login, /admin, /admin/dashboard, /admin/coupons,
#       /dashboard/day-trading-room, /cms/editor
```

---

## 7. The final invariant

After execution, the repo MUST satisfy:

1. **`!important` count: ≤ 50.** Each remaining instance documented in `IMPORTANT_USAGE.md` with one of three valid reasons:
   - Print media (`@media print`)
   - Accessibility (`prefers-reduced-motion`)
   - Fullscreen overlay (legitimately resetting parent constraints)
2. **`pnpm check`: 0 errors / 0 warnings.**
3. **All admin / dashboard / CMS / marketing routes render visually correctly** (manual smoke test required — no automated visual regression in this repo yet).
4. **Login form boxes render without unwanted default borders** (the regression that killed the previous attempt).
5. **No `effect_update_depth_exceeded` on any page load.**

---

## 8. Estimated effort

| Phase | Effort | Risk |
|-------|--------|------|
| Phase 0 (pre-flight) | 5 min | None |
| Phase 1 (scope global) | 10 min | LOW |
| Phase 2 (dashboard.css `!important` removal) | 20 min | MEDIUM |
| Phase 3 (`:where()` conversion) | 15 min | LOW |
| Phase 4 (remaining files) | 4-6 hours | HIGH if rushed |
| Phase 5 (documentation) | 30 min | None |
| Phase 6 (final verification) | 30 min | None |
| **Total** | **6-8 hours** | — |

Each phase is independently committable and rollback-able.

---

## 9. Out of scope (separate work)

- The 47 admin pages still rendering empty `<div class="bg-effects">…` markup. Already neutralized by removing the CSS rules in commit `a87d29362`. Markup cleanup needs an AST-aware Svelte transform (regex demonstrably failed) — track separately.
- Dashboard component CSS rewrites (e.g., `KpiCard3D` per `ANALYTICS_DASHBOARD_AUDIT_2026-04-25.md`).
- Inline-SVG → `<Icon>` migration in `admin/dashboard/+page.svelte`.

---

## 10. The contract

Every PR after this merges should fail review if it adds an `!important` not listed in `IMPORTANT_USAGE.md`. ESLint or stylelint can enforce this:

```json
// stylelint.config.js
{
    "rules": {
        "declaration-no-important": true
    }
}
```

Configure exceptions per-file via `// stylelint-disable declaration-no-important` with a comment explaining why.

---

## 11. Tracking

This plan is row #37 in [`MASTER_UIUX_BACKLOG.md`](MASTER_UIUX_BACKLOG.md) (added now).

---

## 12. 2026-04-26 amendment — wrapper background regression

**Reported by:** end user. **Fixed by:** PE7 forensic agent.

### Symptom

Every marketing page rendered with a white background. The Hero, sections,
and footer dark-canvas designs appeared "stripped" — marketing components
expect to paint dark gradients on top of `body { background-color: #0a0a0a }`
(set unlayered in `frontend/src/app.html:95-102`). Backend surfaces
(admin/dashboard/cms) were unaffected.

### Root cause

Phase 1 (Step 1.1) of this plan moved BOTH

```css
* { @apply border-border outline-ring/50; }
body { @apply bg-background text-foreground; }
```

into a `.marketing-page-root`-scoped block:

```css
.marketing-page-root * { @apply border-border outline-ring/50; }
.marketing-page-root   { @apply bg-background text-foreground; }
```

Scoping the universal `*` rule was correct (admin/dashboard isolation).
Scoping `bg-background` to the wrapper was the defect: the wrapper div is
a child of `<body>`, and Tailwind's `bg-background` resolves to
`oklch(1 0 0)` (white) in light mode (the default). The wrapper therefore
painted a white rectangle directly on top of body's dark canvas, hiding
it. The original layered `body { @apply bg-background }` was *also*
"white" by token, but it lost on cascade-layer order to the unlayered
inline `body { background-color: #0a0a0a }` in `app.html` — so marketing
stayed dark. The wrapper rule had no such unlayered competitor on the div
itself, so white won.

### Fix

`frontend/src/app.css` — restore the `body` target for `bg-background`
while keeping the universal-border rule wrapper-scoped. Diff:

```diff
 @layer base {
-    .marketing-page-root * {
-        @apply border-border outline-ring/50;
-    }
-    .marketing-page-root {
-        @apply bg-background text-foreground;
-    }
+    .marketing-page-root * {
+        @apply border-border outline-ring/50;
+    }
+    body {
+        @apply bg-background text-foreground;
+    }
 }
```

Cascade reasoning (post-fix):

1. `body { @apply bg-background }` is in `@layer base` (layered).
2. `app.html`'s inline `body { background-color: #0a0a0a }` is unlayered.
3. Unlayered beats layered → marketing body stays dark.
4. Admin / dashboard / cms layouts paint their own root container
   backgrounds (`.admin-layout`, `.dashboard__main`, etc.); compound-class
   specificity (≥ 0,1,0) beats the inherited body style without
   `!important`.
5. The universal `*` border rule remains scoped to `.marketing-page-root`,
   so it does not bleed into admin/dashboard descendants.

### Verification

- `grep -rn ': .*!important;' frontend/src --include='*.css' --include='*.svelte' | wc -l` → 0.
- `pnpm --filter revolution-svelte check` → 0 errors / 0 warnings.
- `bash scripts/pe7_gate.sh`: `!important` invariant passes (full gate
  also runs the Rust toolchain).
- Manual smoke (when dev server is up): `/`, `/courses`, `/indicators`,
  `/auth/login` paint dark; `/admin`, `/dashboard/*`, `/cms/editor`
  paint their own backend themes.

### Risk

LOW. Single-line scope change in `app.css`. Trivially revertable.
