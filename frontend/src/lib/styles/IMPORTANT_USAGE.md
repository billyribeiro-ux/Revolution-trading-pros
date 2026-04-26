# `!important` Usage Registry

**Effective:** 2026-04-25 · **Owner:** the team that approved
[CSS_ISOLATION_PLAN_2026-04-25.md](../../../docs/audits/CSS_ISOLATION_PLAN_2026-04-25.md).

This file is the contract. **Any new `!important` not listed here fails review.**
The contract defines four legitimate uses; everything else must rely on
specificity, cascade order, or `@layer`.

---

## The four legitimate categories

| Category | Why allowed |
|---|---|
| **Print media** (`@media print`) | Print output legitimately needs to override authored colors, backgrounds, and visibility regardless of theme. |
| **Accessibility** (`prefers-reduced-motion`) | WCAG 2.1 AAA. Must override component-level animations globally for users with vestibular disorders. |
| **Fullscreen overlay** | Modal / fullscreen / portal components legitimately need to ignore parent layout constraints. |
| **Inline-style override** | When markup sets a property via inline `style="..."` or `style:prop=`, only `!important` can win against inline-style specificity. |

If your case isn't one of the above, fix the specificity / cascade order
instead. Don't add to this list.

---

## Registry — every remaining `!important` declaration

### Category 1 — Print media (Category C)

| File | Line | Reason |
|------|------|--------|
| `lib/styles/print.css` | 1–end | Global `@media print` rules: force black-on-white, hide chrome, expand visibility. |
| `routes/admin/+layout.svelte` | 642 | `@media print` — hide admin chrome (header, mobile menu). |
| `routes/admin/+page.svelte` | 2166 | `@media print` — hide non-content header chrome. |
| `routes/admin/analytics/+page.svelte` | 1236 | `@media print` — hide header actions. |
| `routes/admin/blog/+page.svelte` | 3066 | `@media print` — hide controls / action buttons / shortcuts. |
| `routes/admin/members/+page.svelte` | 2192 | `@media print` — hide pagination / toolbar. |
| `routes/classes/quickstart-precision-trading-c/+page.svelte` | 449, 453, 458 | `@media print` — page bg, video wrapper, title color. |
| `routes/indicators/+page.svelte` | 1562 | `@media print` — hide decorative elements. |
| `lib/components/AdminToolbar.svelte` | 1341 | `@media print` — hide toolbar entirely. |
| `lib/components/nav/NavBar.svelte` | 2274 | `@media print` — hide nav controls. |
| `lib/styles/admin-responsive.css` | 581 | `@media print` — hide buttons/header-actions. |

### Category 2 — Reduced motion (Category D)

| File | Line | Reason |
|------|------|--------|
| `lib/styles/performance.css` | 217–228 (8) | Global `prefers-reduced-motion` — kill all animations/transitions. |
| `routes/dashboard/+layout.svelte` | 765–767 (3) | Local `prefers-reduced-motion` — kill animations on dashboard page. |
| `routes/classes/quickstart-precision-trading-c/+page.svelte` | 203–206 (4) | Local `prefers-reduced-motion`. |
| `routes/indicators/+page.svelte` | 1502, 1509 (2) | Local `prefers-reduced-motion` — kill decorative animations + transitions. |
| `routes/indicators/[id]/+page.svelte` | 337, 338 (2) | Local `prefers-reduced-motion`. |
| `lib/components/sections/CoursesSection.svelte` | 641 | Local `prefers-reduced-motion`. |
| `lib/components/AdminToolbar.svelte` | 1321–1323 (3) | Local `prefers-reduced-motion`. |
| `lib/components/nav/NavBar.svelte` | 1103–1104 (2), 2282–2284 (3) | Local `prefers-reduced-motion`. |
| `lib/components/PopupModal.svelte` | 1676–1677 (2) | Local `prefers-reduced-motion`. |
| `lib/components/popups/PopupRenderer.svelte` | 952 | Local `prefers-reduced-motion`. |
| `lib/styles/admin-responsive.css` | 536–538 (3) | Local `prefers-reduced-motion`. |

### Category 3 — Fullscreen overlay (Category D)

| File | Line | Reason |
|------|------|--------|
| `lib/components/VideoEmbed.svelte` | 1863–1876 (12) | `.fullscreen-video` — fullscreen overlay must reset position/size/transform regardless of parent. |

### Category 4 — Inline-style override (Category E)

The markup binds the same CSS property via inline `style="..."` or
`style:prop={value}`. Inline styles have specificity (1, 0, 0, 0) — higher
than any selector — so the only way for a stylesheet rule to override is
`!important`.

| File | Line | Inline-style source line | Reason |
|------|------|--------------------------|--------|
| `lib/components/media/MediaGrid.svelte` | 357, 363, 369 (3) | line 128 `style="--columns: {columns}"` | Responsive overrides need to beat the inline `--columns`. |
| `lib/components/auth/TradingScene3D.svelte` | 52, 53 (2) | Three.js `WebGLRenderer.setSize()` writes to `canvas.style` | Override library-set canvas dimensions. |
| `lib/components/cms/blocks/layout/ColumnsBlock.svelte` | 301, 302 (2) | line 160 `style:grid-template-columns / style:gap` | Mobile stacking must beat user-configured grid. |
| `lib/components/cms/blocks/trading/ChartBlock.svelte` | 594 (1) | line 451 `style="height: {heightValue}px"` | Fullscreen mode must beat author-configured height. |
| `lib/consent/templates/BannerRenderer.svelte` | 616, 624 (2) | lines 84/86/88 push `max-width` into `style=` array | Mobile max-width must beat template-configured `maxWidth`. |

---

## How to use this list

When you ship a new `!important`:

1. Verify it falls into one of the four categories above.
2. Add it to the table here in the same PR.
3. Add an inline comment in the source file referencing this document.

When you remove an `!important`:

1. Remove its row from this file in the same commit.

---

## Stylelint enforcement (recommended next step)

```js
// stylelint.config.js
{
    "rules": {
        "declaration-no-important": [true, {
            "severity": "error"
        }]
    }
}
```

Per-file exceptions via:

```css
/* stylelint-disable-next-line declaration-no-important
   reason: print media — see IMPORTANT_USAGE.md */
display: none !important;
```

---

## History

- **2026-04-25** — initial registry. Baseline: 321 `!important` declarations
  across the repo. After Phase 1–4 of CSS_ISOLATION_PLAN: 94 declarations
  remain, all in one of the four legitimate categories above. **Reduction:
  71% (227 removed).**
