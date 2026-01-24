# RTP CSS Audit Report

**Date:** January 18, 2026  
**Auditor:** Apple ICT Level 7+ Principal Engineer  
**Project:** Revolution Trading Pros (SvelteKit 5 / Svelte 5)  
**Tailwind Version:** v4.1.17 (Vite Plugin)

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Total CSS Files | 19 |
| Total CSS Lines | 4,855 |
| Svelte Components with `<style>` | 516 |
| Components using `:global()` | 30 |
| `!important` Usage | 220 instances |
| Duplicate Variable Definitions | 36+ variables defined 3x each |
| Light Theme References | 40+ (should be 0 for dark-only) |

**Critical Finding:** Massive duplication between `app.css` and `tokens/*.css` files. The same color variables are defined 3 times each with different values.

---

## 1. File Inventory

### CSS Files (19 total)

| File | Lines | Purpose | Imports | Imported By |
|------|-------|---------|---------|-------------|
| `src/app.css` | 2,481 | Main entry, Tailwind, theme overrides | tailwindcss, simpler-icons.css, main.css | +layout.svelte |
| `src/lib/styles/main.css` | 15 | Admin design system entry | tokens/index.css, base/index.css | app.css, admin/+layout.svelte, dashboard/+layout.svelte |
| `src/lib/styles/tokens/index.css` | 11 | Token barrel file | colors.css, typography.css, spacing.css, shadows.css, transitions.css | main.css |
| `src/lib/styles/tokens/colors.css` | 102 | Admin color tokens (dark only) | - | tokens/index.css |
| `src/lib/styles/tokens/typography.css` | 42 | Typography tokens | - | tokens/index.css |
| `src/lib/styles/tokens/spacing.css` | 33 | Spacing & radius tokens | - | tokens/index.css |
| `src/lib/styles/tokens/shadows.css` | 26 | Shadow tokens | - | tokens/index.css |
| `src/lib/styles/tokens/transitions.css` | 32 | Transition tokens | - | tokens/index.css |
| `src/lib/styles/base/index.css` | 9 | Base styles barrel | reset.css, global.css, admin-page-layout.css | main.css |
| `src/lib/styles/base/reset.css` | 72 | CSS reset | - | base/index.css |
| `src/lib/styles/base/global.css` | 308 | Admin global styles (buttons, inputs, alerts) | - | base/index.css |
| `src/lib/styles/base/admin-page-layout.css` | 524 | Admin layout system | - | base/index.css |
| `src/lib/styles/simpler-icons.css` | 86 | Icon font definitions | - | app.css |
| `src/lib/styles/dashboard.css` | 311 | Dashboard-specific styles | - | dashboard/+layout.svelte |
| `src/lib/styles/print.css` | 376 | Print stylesheet | - | blog/[slug]/+page.svelte |
| `src/routes/live-trading-rooms/performance.css` | 297 | Page-specific styles | - | (local import) |
| `src/stories/page.css` | 68 | Storybook | - | Page.svelte |
| `src/stories/header.css` | 32 | Storybook | - | Header.svelte |
| `src/stories/button.css` | 30 | Storybook | - | Button.svelte |

---

## 2. Dependency Graph

```
+layout.svelte (root)
└── src/app.css (2,481 lines)
    ├── @import 'tailwindcss'
    ├── @import './lib/styles/simpler-icons.css' (86 lines)
    └── @import './lib/styles/main.css' (15 lines)
        ├── @import './tokens/index.css' (11 lines)
        │   ├── @import './colors.css' (102 lines)
        │   ├── @import './typography.css' (42 lines)
        │   ├── @import './spacing.css' (33 lines)
        │   ├── @import './shadows.css' (26 lines)
        │   └── @import './transitions.css' (32 lines)
        └── @import './base/index.css' (9 lines)
            ├── @import './reset.css' (72 lines)
            ├── @import './global.css' (308 lines)
            └── @import './admin-page-layout.css' (524 lines)

admin/+layout.svelte
└── @import '$lib/styles/main.css' (DUPLICATE - already in app.css)

dashboard/+layout.svelte
├── @import '$lib/styles/main.css' (DUPLICATE - already in app.css)
└── @import '$lib/styles/dashboard.css' (311 lines)
```

---

## 3. Token Source Mapping

### Variable Definition Locations

| Token Category | `app.css` | `tokens/*.css` | `base/global.css` | Notes |
|---------------|-----------|----------------|-------------------|-------|
| Colors (`--color-rtp-*`) | ✅ Lines 35-180 | ✅ colors.css | ❌ | **DUPLICATE with different values** |
| Background (`--bg-*`) | ❌ | ✅ colors.css | ❌ | Clean |
| Primary (`--primary-*`) | ❌ | ✅ colors.css | ❌ | Clean |
| Secondary (`--secondary-*`) | ❌ | ✅ colors.css | ❌ | Clean |
| Text (`--text-*`) | ❌ | ✅ colors.css + typography.css | ❌ | Split across files |
| Border (`--border-*`) | ❌ | ✅ colors.css | ❌ | Clean |
| Semantic (`--success/warning/error/info-*`) | ❌ | ✅ colors.css | ❌ | Clean |
| Typography (`--font-*`) | ✅ Lines 119-129 | ✅ typography.css | ❌ | **DUPLICATE with different values** |
| Spacing (`--space-*`) | ❌ | ✅ spacing.css | ❌ | Clean |
| Radius (`--radius-*`) | ❌ | ✅ spacing.css | ❌ | Clean |
| Shadows (`--shadow-*`) | ❌ | ✅ shadows.css | ❌ | Clean |
| Transitions (`--transition-*`) | ❌ | ✅ transitions.css | ❌ | Clean |
| Breakpoints (`--bp-*`) | ✅ Lines 99-106 | ❌ | ❌ | Only in app.css |
| Layout (`--sidebar-*`) | ✅ Lines 109-111 | ✅ colors.css | ❌ | **DUPLICATE** |

### Duplicate Variables (Defined 3+ Times)

These variables are defined multiple times with **potentially conflicting values**:

```
3x --color-rtp-primary
3x --color-rtp-primary-light
3x --color-rtp-primary-dark
3x --color-rtp-primary-soft
3x --color-rtp-bg
3x --color-rtp-bg-alt
3x --color-rtp-surface
3x --color-rtp-surface-elevated
3x --color-rtp-surface-overlay
3x --color-rtp-text
3x --color-rtp-text-secondary
3x --color-rtp-muted
3x --color-rtp-placeholder
3x --color-rtp-border
3x --color-rtp-border-light
3x --color-rtp-border-focus
3x --color-rtp-success
3x --color-rtp-success-soft
3x --color-rtp-warning
3x --color-rtp-warning-soft
3x --color-rtp-error
3x --color-rtp-error-soft
3x --color-rtp-info
3x --color-rtp-info-soft
3x --color-rtp-indigo
3x --color-rtp-violet
3x --color-rtp-blue
3x --color-rtp-emerald
3x --color-rtp-amber
3x --color-rtp-rose
3x --color-rtp-cyan
3x --color-rtp-purple
3x --shadow-sm
3x --shadow-md
3x --shadow-lg
3x --shadow-xl
```

---

## 4. Issues Found

### 🔴 Critical Issues

1. **Massive Variable Duplication**
   - `app.css` defines `--color-rtp-*` variables (lines 35-180)
   - `tokens/colors.css` defines different color system (`--bg-*`, `--primary-*`, etc.)
   - Same conceptual tokens with different naming and values
   - **Impact:** Confusion, inconsistent styling, larger bundle size

2. **Conflicting Typography Systems**
   - `app.css` lines 119-129: `--font-heading`, `--font-body`, `--font-mono`
   - `tokens/typography.css`: `--font-sans`, `--font-mono` (different values)
   - **Impact:** Unpredictable font rendering

3. **Light Theme Code in Dark-Only System**
   - `app.css` lines 136-176: Full `:root.light` definitions
   - `app.css` lines 1469-1540: Light theme premium effects
   - 40+ Svelte components with `:global(html.light)` / `:global(body.light)` selectors
   - **Impact:** Dead code if dark-only, or unintended light mode bleeding

### 🟠 Major Issues

4. **Duplicate CSS Imports**
   - `main.css` is imported by:
     - `app.css` (root layout)
     - `admin/+layout.svelte`
     - `dashboard/+layout.svelte`
   - These layouts are nested, causing triple import
   - **Impact:** Potential cascade conflicts, larger bundle

5. **220 `!important` Declarations**
   - Spread across `app.css`, `dashboard.css`, `print.css`, Svelte components
   - **Impact:** Specificity wars, harder maintenance

6. **516 Svelte Components with `<style>` Blocks**
   - Many with 200-800+ lines of CSS
   - Top offenders: Hero.svelte (856 lines), FormAIAssistant.svelte (439 lines)
   - **Impact:** Large component files, potential code splitting issues

### 🟡 Minor Issues

7. **30 Components Using `:global()`**
   - Auth components heavily use `:global(html.light)` patterns
   - Some legitimate uses (tooltip portals, table styling)
   - **Impact:** Potential global style leakage

8. **Orphaned/Page-Specific CSS Files**
   - `src/routes/live-trading-rooms/performance.css` (297 lines)
   - Should be scoped to component or consolidated

---

## 5. Token Value Comparison

### `app.css` (Tailwind v4 @theme) vs `tokens/colors.css`

| Concept | app.css Variable | app.css Value | tokens/colors.css Variable | tokens/colors.css Value |
|---------|------------------|---------------|---------------------------|-------------------------|
| Background | `--color-rtp-bg` | `oklch(0.12 0.02 250)` | `--bg-base` | `#0d1117` |
| Elevated | `--color-rtp-surface-elevated` | `oklch(0.22 0.02 250)` | `--bg-elevated` | `#161b22` |
| Primary | `--color-rtp-primary` | `oklch(0.6 0.2 250)` | `--primary-500` | `#e6b800` |
| Text | `--color-rtp-text` | `oklch(0.96 0.01 250)` | `--text-primary` | `#f0f6fc` |
| Border | `--color-rtp-border` | `oklch(0.3 0.02 250)` | `--border-default` | `#30363d` |

**Analysis:** Two completely different color systems coexist:
- `app.css` uses OKLCH color space (modern, perceptually uniform)
- `tokens/colors.css` uses hex values (GitHub-inspired dark theme)

---

## 6. Svelte Component Style Analysis

### Largest Style Blocks (Top 10)

| Component | Style Lines | Uses :global | Notes |
|-----------|-------------|--------------|-------|
| Hero.svelte | 856 | ❌ | Homepage hero - could extract |
| FormAIAssistant.svelte | 439 | ❌ | Complex form component |
| FormImportExport.svelte | 391 | ❌ | Form utilities |
| FormCollaborators.svelte | 334 | ❌ | Form component |
| InventoryField.svelte | 316 | ❌ | Form field |
| FormEmbedGenerator.svelte | 316 | ❌ | Form utilities |
| FormReport.svelte | 314 | ❌ | Form component |
| ConfigPanel.svelte | 313 | ❌ | Page builder |
| AdminApprovalStatus.svelte | 310 | ❌ | Admin component |
| ThemeCustomizer.svelte | 303 | ❌ | Theme settings |

### Components with `:global()` (High Risk)

| Component | `:global()` Usage | Risk Level |
|-----------|-------------------|------------|
| LoginLayout.svelte | `:global(html.light)` | 🟠 Medium |
| LoginForm.svelte | `:global(html.light)`, `:global(.arrow-icon)` | 🟠 Medium |
| SocialLoginButtons.svelte | `:global(html.light)` | 🟠 Medium |
| TradingHeroBackground.svelte | `:global(html.light)` | 🟠 Medium |
| Tooltip.svelte | `:global(.tooltip-portal)` | 🟢 Low (legitimate) |
| Table.svelte | `:global(tr:hover)`, `:global(td)` | 🟢 Low (legitimate) |
| BlockEditor.svelte | Editor content styling | 🟢 Low (legitimate) |

---

## 7. Build Configuration

### Tailwind CSS v4 Setup

```json
// package.json dependencies
"@tailwindcss/vite": "^4.1.17",
"tailwindcss": "^4.1.17"
```

- **Approach:** Vite Plugin (recommended for v4)
- **No PostCSS config:** Correct - Vite plugin handles everything
- **No tailwind.config.js:** Correct - v4 uses CSS-based config

### CSS Entry Points

| Layout | CSS Imports |
|--------|-------------|
| Root `+layout.svelte` | `../app.css` |
| Admin `+layout.svelte` | `$lib/styles/main.css` |
| Dashboard `+layout.svelte` | `$lib/styles/main.css`, `$lib/styles/dashboard.css` |

---

## 8. Recommended Consolidation Plan

### Phase 1: Establish Single Source of Truth

1. **Choose ONE color system:**
   - Option A: Keep `tokens/colors.css` (hex, GitHub-inspired) ✅ Recommended
   - Option B: Keep `app.css` @theme block (OKLCH, modern)
   - **Do NOT keep both**

2. **Remove duplicates from `app.css`:**
   - Delete lines 35-180 (color definitions in @theme)
   - Delete lines 136-180 (light theme overrides)
   - Keep only Tailwind config and truly global utilities

3. **Consolidate typography:**
   - Use `tokens/typography.css` as SSOT
   - Remove `--font-heading`, `--font-body` from `app.css`

### Phase 2: Remove Light Theme Dead Code

1. **If dark-only (confirmed):**
   - Remove all `:root.light`, `html.light`, `body.light` from `app.css`
   - Remove `:global(html.light)` from auth components
   - Search and remove: `prefers-color-scheme: light`

### Phase 3: Fix Import Chain

1. **Remove duplicate imports:**
   - `admin/+layout.svelte`: Remove `main.css` import (inherited from root)
   - `dashboard/+layout.svelte`: Remove `main.css` import (inherited from root)

### Phase 4: Reduce `!important`

1. **Audit each usage:**
   - Print styles: Acceptable
   - Reduced motion: Acceptable
   - Icon fonts: Acceptable
   - Dashboard visibility toggles: Refactor to use proper specificity

### Phase 5: Component Style Extraction

1. **Consider extracting large style blocks:**
   - Hero.svelte (856 lines) → `hero.css` if shared
   - Form components → Consider form-specific stylesheet

---

## 9. Questions Answered

### 1. What is in `main.css`?
- Just imports: `tokens/index.css` + `base/index.css`
- **Does NOT define any tokens directly**
- Serves as barrel file for admin design system

### 2. Are there other token files?
- Yes: `tokens/colors.css`, `tokens/typography.css`, `tokens/spacing.css`, `tokens/shadows.css`, `tokens/transitions.css`
- **These conflict with `app.css` definitions**

### 3. Which Svelte components have large style blocks?
- Hero.svelte (856 lines) - largest
- 10+ form components with 300+ lines each
- See Section 6 for full list

### 4. Are there component libraries with their own styles?
- No external UI libraries
- All custom components
- Icon font: `simpler-icons.css`

### 5. What's the Tailwind setup?
- **Tailwind CSS v4.1.17**
- Using Vite plugin (not PostCSS)
- CSS-based configuration in `app.css` `@theme` block
- **Correct modern setup**

### 6. Are there CSS-in-JS solutions?
- **No**
- Pure CSS + Svelte scoped styles

---

## 10. Action Items Checklist

### Immediate (Before Refactoring)

- [ ] Decide: OKLCH colors (`app.css`) or Hex colors (`tokens/colors.css`)
- [ ] Confirm: Is this truly dark-only? Remove light theme if yes
- [ ] Audit: Which `--color-rtp-*` variables are actually used in components?

### Short-term (This Sprint)

- [ ] Remove duplicate variable definitions
- [ ] Fix import chain (remove nested `main.css` imports)
- [ ] Remove light theme dead code if dark-only

### Medium-term (Next Sprint)

- [ ] Reduce `!important` usage by 50%
- [ ] Extract Hero.svelte styles if shared elsewhere
- [ ] Document final token system in README

---

## Appendix: File Locations

```
frontend/src/
├── app.css                              # 2,481 lines - MAIN ENTRY
├── lib/styles/
│   ├── main.css                         # 15 lines - barrel file
│   ├── simpler-icons.css                # 86 lines - icon font
│   ├── dashboard.css                    # 311 lines - dashboard specific
│   ├── print.css                        # 376 lines - print styles
│   ├── ADMIN-COLOR-SYSTEM.md            # Documentation
│   ├── tokens/
│   │   ├── index.css                    # 11 lines - barrel
│   │   ├── colors.css                   # 102 lines - color tokens
│   │   ├── typography.css               # 42 lines - type tokens
│   │   ├── spacing.css                  # 33 lines - spacing tokens
│   │   ├── shadows.css                  # 26 lines - shadow tokens
│   │   └── transitions.css              # 32 lines - transition tokens
│   └── base/
│       ├── index.css                    # 9 lines - barrel
│       ├── reset.css                    # 72 lines - CSS reset
│       ├── global.css                   # 308 lines - global styles
│       └── admin-page-layout.css        # 524 lines - layout system
└── routes/
    └── live-trading-rooms/
        └── performance.css              # 297 lines - page specific
```

---

**Report Complete. Do not proceed with changes until decisions are made on the action items above.**
