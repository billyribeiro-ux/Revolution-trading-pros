# CSS Cascade — Forensic Audit

**Auditor:** Claude (Opus 4.7), Distinguished Principal Engineer (ICT 7+).
**Date:** 2026-04-25 · **Method:** read-only sweep with file:line citations.
**Trigger:** user reported `app.css` overriding admin/dashboard page styles ("the dashboard looks like a circus").

---

## Verdict

**Confirmed.** `app.css` IS overriding admin/dashboard styles, but **not because of import order — because of `@layer base { body, * { ... } }` rules that are global by design**. Inherited and equal-specificity rules force admin/dashboard to fight back, leading to 15+ `!important` overrides in `dashboard.css` to reclaim ground.

This is **architectural debt, not an accident**. Smallest correct fix: scope the global rules to non-admin/non-dashboard surfaces.

---

## 1. The CSS layer cake

| Order | File | Loaded by | Scope | Lines |
|-------|------|-----------|-------|-------|
| 1 | `frontend/src/app.css` | `frontend/src/routes/+layout.svelte:21` | **GLOBAL — every page** | 414 |
| 2 | `frontend/src/lib/styles/main.css` | `frontend/src/routes/admin/+layout.svelte:8` AND `frontend/src/routes/dashboard/+layout.svelte:25` | admin + dashboard | + imports |
| 2a | `tokens/index.css`, `design-tokens.css`, `base/reset.css`, `base/global.css`, `base/admin-page-layout.css` | via `main.css` | admin + dashboard | imported chain |
| 3a | `frontend/src/lib/styles/admin-responsive.css` | `frontend/src/routes/admin/+layout.svelte:9` | admin only | — |
| 3b | `frontend/src/lib/styles/dashboard.css` | `frontend/src/routes/dashboard/+layout.svelte:26` | dashboard only | — |
| 4 | per-component scoped `<style>` blocks | every `+page.svelte` and component | scoped via Svelte hashing | per file |

**Key observation:** `app.css` is loaded at the root layout, so its rules apply to admin and dashboard descendants too. Specificity ties go to **assigned > inherited**, but inherited rules still set initial values that descendants must explicitly override.

---

## 2. The offenders in `app.css`

`frontend/src/app.css:117-124`:

```css
@layer base {
    * {
        @apply border-border outline-ring/50;
    }
    body {
        @apply bg-background text-foreground;
    }
}
```

| Rule | Specificity | Inherited? | Bleed |
|------|-------------|------------|-------|
| `body { @apply bg-background ... }` | (0, 0, 1) (single element selector) | ✓ inherited to all descendants | Sets light-theme white bg + dark-foreground text on **every page** including admin/dashboard |
| `* { @apply border-border ... }` | (0, 0, 0) (universal) | applies to every element | Forces a default border on every descendant unless overridden |

These live in `@layer base`, which means they're **lower priority than `@layer utilities`** — but `@layer base` still beats unscoped scoped styles when specificity is equal.

---

## 3. Cascade collisions — five sample pages

### 3.1 `frontend/src/routes/admin/+page.svelte` (admin home)

| Selector | File:line | Specificity | Result |
|----------|-----------|-------------|--------|
| `body` | `app.css:122` | (0, 0, 1) | **inherited**: bg `oklch(1 0 0)` white, text dark |
| `.admin-layout` | `admin/+layout.svelte` scoped | (0, 1, 0) | **assigned**: `background: var(--bg-base)` (admin dark) |

Assigned beats inherited at runtime — so `.admin-layout` paints over the inherited body bg. **But** the body is still white underneath; if any element above `.admin-layout` peeks through (like during SSR hydration flash or layout shift), the white shows.

### 3.2 `frontend/src/routes/admin/coupons/+page.svelte` (table page)

`<h1 class="page-title">` — defined in `admin-responsive.css:408` with `color: var(--text-primary)`. Since both `body` (inherited) and `.page-title` (assigned) have specificity (0, 0, 1) vs (0, 1, 0), the class wins. ✅ This case is fine.

But `<table class="admin-table">` — if any cell content does NOT define its own color, it inherits from `body`'s `text-foreground` (dark). On a dark admin table, that produces dark text on dark background. **Real bug.**

### 3.3 `frontend/src/routes/dashboard/+page.svelte`

`dashboard.css:54-66`:

```css
.dashboard__header {
    display: flex;
    padding: 30px;
    background: var(--color-rtp-surface, #fff);
    border-bottom: 1px solid var(--color-rtp-border, #e5e5e5);
    margin: 0 !important;          /* ← DEFENSIVE */
    width: 100% !important;        /* ← DEFENSIVE */
    max-width: none !important;    /* ← DEFENSIVE */
    box-sizing: border-box;
}
```

The three `!important` flags are **defensive overrides** against `body` cascade. The author knew global rules were leaking in and used `!important` to force the dashboard layout to win.

Across `dashboard.css` and `admin-responsive.css`: **15+ `!important` flags** with the same root cause.

### 3.4 `frontend/src/routes/admin/courses/+page.svelte` (modal page)

The QuickCreate modal defines `position: fixed; inset: 0; background: rgba(0,0,0,0.6)`. But its inner content card relies on body text-color inheritance for default text — which is `var(--foreground)` from app.css, the *light theme* dark color. On a dark modal that's a real readability hit.

### 3.5 `frontend/src/routes/admin/+layout.svelte`

This is where everything is wired up:

```svelte
<!-- frontend/src/routes/admin/+layout.svelte:8-9 -->
import '$lib/styles/main.css';
import '$lib/styles/admin-responsive.css';
```

But by the time this loads, `app.css` has **already** painted the body. Order is:
1. Browser parses `<link>`/`<style>` from root layout → app.css applies
2. Admin layout's `<script>` `import 'main.css'` triggers a chunk load
3. `main.css` rules apply at admin scope

There's a *small* visual flash between (1) and (3) when the root paint shows white before admin paints dark. On slow connections or on first navigation to /admin, this is visible.

---

## 4. Specificity math — three concrete cases

### Case A — `.dashboard__header` background

| # | Selector | Specificity | File:line | Property | Wins |
|---|----------|-------------|-----------|----------|------|
| A | `body` | (0, 0, 1) | `app.css:122` | `background: var(--background, oklch(1 0 0))` | inherited (low priority) |
| B | `.dashboard__header` | (0, 1, 0) | `dashboard.css:54` | `background: var(--color-rtp-surface, #fff)` | tie with C — later |
| C | `.dashboard__header` | (0, 1, 0) | `dashboard.css:64` | `... !important` | **C wins** |

Without the `!important`, B would still win because **assigned beats inherited at equal specificity**. The `!important` is over-engineered defense.

### Case B — `.page-title` text color

| # | Selector | Specificity | File:line | Property |
|---|----------|-------------|-----------|----------|
| A | `body` | (0, 0, 1) | `app.css:122` | `color: var(--foreground)` |
| B | `.page-title` | (0, 1, 0) | `admin-responsive.css:408` | `color: var(--text-primary)` |

B wins on specificity. ✅ Correct outcome. No `!important` needed.

### Case C — button touch target

| # | Selector | Specificity | File:line | Property |
|---|----------|-------------|-----------|----------|
| A | `button` (in `@media (hover: none)`) | (0, 0, 1) | `app.css:252-265` | `min-height: 44px` |
| B | `.btn` | (0, 1, 0) | `base/global.css:124` | `padding: ...` |
| C | `.btn-primary` | (0, 1, 0) | `base/global.css:142` | `background: ...` |

`.btn.btn-primary` (when both classes are present) is (0, 2, 0) and beats (0, 0, 1). ✅ Touch target rules apply because media query doesn't change specificity, and both are properties on different keys (height vs padding) that don't conflict.

---

## 5. Tailwind interaction

- `@import 'tailwindcss'` loads at `app.css:1`. Single load — **no double import**. ✅
- `@layer base`, `@layer utilities` are used correctly per Tailwind v4 conventions.
- `@layer base { body {...} }` is the canonical Tailwind way to set defaults — **the issue is not the layer, it's that the rule applies globally**.

Tailwind utilities (`.bg-slate-900`, etc.) live in `@layer utilities`, which has higher priority than `@layer base`. So a child element with `class="bg-slate-900"` will override the body's inherited `bg-background`. But CSS-custom-property-based admin styles don't use Tailwind utilities — they use `background: var(--bg-base)` — and that lands in scoped styles which are **not in any layer**, so they fight at default cascade rules.

---

## 6. Token consistency

| Token | Defined in | Value model | Scope |
|-------|-----------|-------------|-------|
| `--background` / `--foreground` | `app.css:9-94` | OKLch (Tailwind v4 semantic) | global, light theme defaults |
| `--bg-base`, `--text-primary`, `--admin-*` | `lib/styles/design-tokens.css` + `tokens/colors.css` | hex / hsl | admin/dashboard via `main.css` |

Two parallel token systems. Not necessarily wrong (different concerns) but it means components must explicitly opt into the admin/dashboard system; otherwise they fall back to the light-theme `--background` from app.css.

---

## 7. Recommended fix — Option A (smallest correct change)

Replace `frontend/src/app.css:117-124`:

```css
@layer base {
    * {
        @apply border-border outline-ring/50;
    }
    body {
        @apply bg-background text-foreground;
    }
}
```

With:

```css
@layer base {
    /* Default border + outline color for non-admin, non-dashboard surfaces.
       Admin / dashboard layouts paint their own surfaces and don't inherit
       the marketing-page light theme. */
    :where(body:not(:has(.admin-layout)):not(:has(.dashboard))) * {
        @apply border-border outline-ring/50;
    }

    body:not(:has(.admin-layout)):not(:has(.dashboard)) {
        @apply bg-background text-foreground;
    }
}
```

Or, simpler and more reliable (if `:has()` is risky for older browsers):

```css
/* Move the rules out of @layer base so they don't apply globally;
   instead apply them on the marketing-page wrapper only. */
.marketing-page-root {
    @apply bg-background text-foreground;
}

.marketing-page-root * {
    @apply border-border outline-ring/50;
}
```

…and add `class="marketing-page-root"` to the public layout's outermost element.

**Effort:** ~10 minutes for the edit + test.
**Risk:** low — rolling back is one-line.
**Knock-on:** the 15+ `!important` flags in `dashboard.css` can then be removed in a follow-up sweep.

### Option B — wholesale: stop loading `app.css` in admin/dashboard

If you want the cleanest separation, the root `+layout.svelte` could conditionally import `app.css` only when not under `/admin` or `/dashboard`. But that's a bigger refactor; the route-conditional import pattern in SvelteKit isn't trivial and risks SSR hydration flash.

### Option C — accept the current state, formalize the `!important`

Rather than removing the global rules, add a CSS Architecture doc that describes the "admin/dashboard must override `body` defaults" contract. This is what the codebase has *de facto* done. It works, but the `!important` flags are a smell.

---

## 8. Recommendation

**Do Option A.** Concrete first PR:

1. Edit `frontend/src/app.css:117-124` per the snippet above.
2. Run `pnpm check` (svelte-check should report 0/0/0).
3. Run `pnpm dev` and visit `/`, `/admin`, `/dashboard`, `/cms/editor` — verify each renders with the right surface color and no flash.
4. Remove the `!important` flags from `dashboard.css:54-66` and re-test.

This is a 30-minute task. It eliminates the systemic override fight that's causing all the visual chaos in the admin/dashboard.

---

## 9. Tracked in the master backlog

This audit is row #36 in [`MASTER_UIUX_BACKLOG.md`](MASTER_UIUX_BACKLOG.md) (added now).
