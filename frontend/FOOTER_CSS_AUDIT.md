# MarketingFooter CSS Audit Report

**Date:** January 19, 2026  
**Objective:** End-to-end investigation of all CSS controlling the MarketingFooter component

---

## 🎯 Executive Summary

The MarketingFooter is controlled by **3 primary CSS sources**:

1. **Component-scoped styles** in `MarketingFooter.svelte`
2. **Tailwind utility classes** on the layout wrapper in `+layout.svelte`
3. **Print-only styles** in `print.css` (hidden on screen)

**No global CSS in `app.css` affects the footer.**

---

## 📍 1. Component-Scoped Styles

**Location:** `@/frontend/src/lib/components/layout/MarketingFooter.svelte:140-380`

### Critical Layout Properties

```css
.marketing-footer {
	background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
	border-top: 1px solid rgba(99, 102, 241, 0.1);
	padding: 4rem 0 2rem;
	width: 100%;
	min-width: 0;
	flex-shrink: 0;
	margin-top: auto; /* ⚠️ CRITICAL: Pushes footer to bottom */
}
```

### Key CSS Features

| Property                 | Value       | Purpose                                   |
| ------------------------ | ----------- | ----------------------------------------- |
| `margin-top: auto`       | auto        | Pushes footer to bottom in flexbox layout |
| `flex-shrink: 0`         | 0           | Prevents footer from shrinking            |
| `width: 100%`            | 100%        | Full width                                |
| `min-width: 0`           | 0           | Prevents overflow issues                  |
| ~~`overflow-x: hidden`~~ | **REMOVED** | Was causing layout break                  |

### Responsive Breakpoints

```css
/* Tablet: max-width: 1024px */
- padding: 3rem 0 1.5rem
- grid-template-columns: 1fr 1fr

/* Mobile: max-width: 640px */
- padding: 2.5rem 0 1.5rem
- grid-template-columns: 1fr
```

### Accessibility Features

```css
/* Focus states */
.social-link:focus-visible {
	outline: 2px solid rgba(129, 140, 248, 0.9);
	outline-offset: 2px;
}

.footer-list a:focus-visible {
	outline: 2px solid rgba(129, 140, 248, 0.9);
	outline-offset: 3px;
	border-radius: 0.25rem;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
	.social-link,
	.footer-list a {
		transition: none;
	}
	.social-link:hover {
		transform: none;
	}
}
```

---

## 📍 2. Layout Wrapper (Tailwind Utilities)

**Location:** `@/frontend/src/routes/+layout.svelte:107`

### Parent Container Classes

```html
<div class="min-h-screen flex flex-col" class:has-admin-toolbar="{isAdmin}">
	<NavBar />

	<main id="main-content" class="flex-grow">{@render children()}</main>

	<MarketingFooter />
</div>
```

### Tailwind Class Breakdown

| Class                 | CSS Output               | Purpose                      |
| --------------------- | ------------------------ | ---------------------------- |
| `min-h-screen`        | `min-height: 100vh`      | Ensures full viewport height |
| `flex`                | `display: flex`          | Enables flexbox layout       |
| `flex-col`            | `flex-direction: column` | Vertical stacking            |
| `flex-grow` (on main) | `flex-grow: 1`           | Main expands to fill space   |

### How Sticky Footer Works

```
┌─────────────────────────────────┐
│ <div class="min-h-screen        │ ← min-height: 100vh
│      flex flex-col">             │ ← flex-direction: column
│                                  │
│  ┌────────────────────────────┐ │
│  │ <NavBar />                 │ │ ← Fixed height
│  └────────────────────────────┘ │
│                                  │
│  ┌────────────────────────────┐ │
│  │ <main class="flex-grow">   │ │ ← Expands to fill space
│  │   Page content             │ │
│  │                            │ │
│  └────────────────────────────┘ │
│                                  │
│  ┌────────────────────────────┐ │
│  │ <MarketingFooter />        │ │ ← margin-top: auto
│  │ (margin-top: auto)         │ │   pushes to bottom
│  └────────────────────────────┘ │
└─────────────────────────────────┘
```

**Formula:**

1. Parent: `min-h-screen flex flex-col` (100vh minimum, column layout)
2. Main: `flex-grow` (takes available space)
3. Footer: `margin-top: auto` (pushes to bottom)

---

## 📍 3. Print Styles

**Location:** `@/frontend/src/lib/styles/print.css:40-45`

```css
@media print {
	nav,
    footer,
    .navbar,
    .nav-bar,
    .header-nav,
    .site-footer,
    .marketing-footer,  /* ← Footer hidden when printing */
    .admin-toolbar,
    /* ... */ {
		display: none !important;
	}
}
```

**Impact:** Footer is hidden when printing pages. No effect on screen rendering.

---

## 📍 4. Global CSS (app.css)

**Location:** `@/frontend/src/app.css`

**Finding:** ✅ **No footer-specific styles in app.css**

The file contains:

- Tailwind imports
- `@theme` tokens for RTP design system
- `.reveal` animation classes
- `.animate-bounce-slow` keyframes

**None of these affect the footer.**

---

## 🔍 Historical Issues & Fixes

### Issue #1: `overflow-x: hidden` Causing Layout Break

**Date:** January 19, 2026  
**Location:** `MarketingFooter.svelte:154`  
**Problem:** `overflow-x: hidden` on `.marketing-footer` was causing layout collapse  
**Fix:** Removed the property

**Before:**

```css
.marketing-footer {
	overflow-x: hidden; /* ❌ Caused layout break */
	margin-top: auto;
}
```

**After:**

```css
.marketing-footer {
	/* overflow-x: hidden removed */
	margin-top: auto;
}
```

### Issue #2: Page Wrappers with `overflow-x-hidden`

**Date:** Previous session  
**Location:** Various page files  
**Problem:** Individual pages had `overflow-x-hidden` on their wrapper divs  
**Fix:** Removed from all marketing pages

---

## 🎨 CSS Specificity & Cascade

### Specificity Hierarchy

1. **Inline styles** (none on footer)
2. **Component scoped styles** (`.marketing-footer` - specificity: 0,0,1,0)
3. **Tailwind utilities** (on parent wrapper - specificity: 0,0,1,0)
4. **Print media query** (only applies when printing)

### Cascade Order

```
1. Tailwind base (@import 'tailwindcss')
2. Component scoped styles (<style> in .svelte)
3. Tailwind utilities (class="...")
4. Print styles (@media print)
```

**No conflicts detected.** Each layer serves a distinct purpose.

---

## 📊 CSS Dependencies Graph

```
MarketingFooter Rendering
│
├─ Component Styles (MarketingFooter.svelte)
│  ├─ .marketing-footer (main container)
│  ├─ .footer-container (max-width wrapper)
│  ├─ .footer-grid (CSS Grid layout)
│  ├─ .footer-brand, .footer-column (grid items)
│  ├─ .social-links, .social-link (social icons)
│  ├─ .footer-list, .footer-list a (navigation links)
│  ├─ .risk-disclaimer (warning box)
│  └─ .footer-bottom (copyright)
│
├─ Layout Wrapper (+layout.svelte)
│  ├─ min-h-screen (100vh minimum)
│  ├─ flex flex-col (column flexbox)
│  └─ flex-grow on <main> (expands main content)
│
└─ Print Styles (print.css)
   └─ display: none @media print
```

---

## ✅ Recommendations

### Current State: ✅ HEALTHY

1. **Layout structure is correct** - Flexbox sticky footer pattern properly implemented
2. **No CSS conflicts** - Clean separation of concerns
3. **Accessibility added** - Focus states and reduced motion support
4. **Performance optimized** - `overflow-x: hidden` removed

### Best Practices Applied

- ✅ Component-scoped styles (no global pollution)
- ✅ Semantic HTML (`<nav>` elements with ARIA labels)
- ✅ Responsive design (mobile-first breakpoints)
- ✅ Accessibility (focus-visible, reduced motion)
- ✅ Print optimization (hidden on print)

### No Action Required

The footer CSS is production-ready. All previous layout break issues have been resolved.

---

## 🔗 Related Files

| File                                                          | Purpose                                   |
| ------------------------------------------------------------- | ----------------------------------------- |
| `@/frontend/src/lib/components/layout/MarketingFooter.svelte` | Footer component with scoped styles       |
| `@/frontend/src/routes/+layout.svelte`                        | Root layout with flexbox wrapper          |
| `@/frontend/src/lib/styles/print.css`                         | Print-only styles                         |
| `@/frontend/src/app.css`                                      | Global Tailwind config (no footer styles) |

---

**End of Audit Report**
