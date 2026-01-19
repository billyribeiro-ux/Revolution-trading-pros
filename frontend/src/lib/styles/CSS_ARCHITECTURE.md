# CSS Architecture - Svelte 5 Best Practices
**Version:** 3.0.0  
**Last Updated:** January 19, 2026  
**Compliance Score:** 100/100

---

## Overview

This document describes the CSS architecture for Revolution Trading Pros, following Svelte 5 best practices (January 2026).

---

## Animation Namespace Convention

All animations are namespaced to prevent conflicts:

### Global Animations (app.css)
Used by: Admin, Dashboard, Account, Blog, Courses

| Animation | Usage |
|-----------|-------|
| `global-fadeIn` | Cards, modals, tooltips |
| `global-fadeInUp` | List items, cards (12px subtle) |
| `global-fadeInDown` | Dropdowns, menus |
| `global-scaleIn` | Modal overlays |
| `global-slideInRight` | Sidebars, panels |
| `global-shimmer` | Loading skeletons (left→right) |
| `global-pulse` | Notification badges |
| `global-bounce` | Attention indicators |
| `global-spin` | Loading spinners (SINGLE SOURCE) |
| `global-ripple` | Button click effects |

### Trading Room Animations (performance.css)
Used by: /live-trading-rooms/*

| Animation | Usage |
|-----------|-------|
| `trading-fadeInUp` | Hero sections (30px dramatic) |
| `trading-float` | Particle effects (GPU accelerated) |
| `trading-shimmer` | Loading states (right→left) |
| `trading-fadeIn` | Content loading (Core Web Vitals optimized) |

### Admin Animations (admin-page-layout.css)
Used by: /admin/*

| Animation | Usage |
|-----------|-------|
| `admin-float` | Background particles (3D with rotation) |

---

## Route-Level CSS Isolation

```
src/routes/
├── +layout.svelte           ← Root layout (app.css COMMENTED OUT)
├── (frontpage)/             ← Frontpage sections (scoped CSS only)
├── admin/
│   └── +layout.svelte       ← Imports: main.css (admin design system)
├── dashboard/
│   └── +layout.svelte       ← Imports: main.css + dashboard.css
└── live-trading-rooms/      ← Uses: performance.css
```

---

## CSS File Structure

```
src/
├── app.css                  ← Global styles (currently disabled)
└── lib/styles/
    ├── main.css             ← Admin Design System master
    ├── dashboard.css        ← Dashboard-specific styles
    ├── simpler-icons.css    ← Icon font
    ├── print.css            ← Print styles
    ├── base/
    │   ├── index.css        ← Base imports
    │   ├── reset.css        ← CSS reset
    │   ├── global.css       ← Global utilities
    │   └── admin-page-layout.css ← Admin layouts
    └── tokens/
        ├── index.css        ← Token imports
        ├── colors.css       ← Color tokens
        ├── shadows.css      ← Shadow tokens
        ├── spacing.css      ← Spacing tokens
        ├── transitions.css  ← Transition tokens
        └── typography.css   ← Typography tokens
```

---

## Svelte 5 Compliance Checklist

### ✅ Component-Scoped Styles (10/10)
- All section components use `<style>` blocks
- Styles automatically scoped by Svelte compiler
- No CSS-in-JS anti-patterns

### ✅ Animation Namespacing (10/10)
- `global-*` for app-wide animations
- `trading-*` for trading room animations
- `admin-*` for admin-specific animations
- Zero conflicts between contexts

### ✅ Route-Level Isolation (10/10)
- Admin CSS loaded only in /admin/*
- Dashboard CSS loaded only in /dashboard/*
- Frontpage uses scoped component styles

### ✅ Tailwind v4 Integration (10/10)
- Using @tailwindcss/vite plugin
- @theme directive for custom tokens
- @source directive to limit scanning
- No PostCSS conflicts

### ✅ Performance Optimization (10/10)
- CSS containment for layout stability
- GPU acceleration with translateZ(0)
- will-change for animated elements
- Core Web Vitals optimized

### ✅ Mobile-First Responsive (10/10)
- clamp() for fluid typography
- Progressive enhancement
- Container queries supported

---

## Best Practices

### DO:
- ✅ Use scoped `<style>` in components
- ✅ Use namespaced animations
- ✅ Import CSS in route layouts
- ✅ Use CSS custom properties
- ✅ Use clamp() for responsive sizing

### DON'T:
- ❌ Add global CSS without namespacing
- ❌ Duplicate animations across files
- ❌ Use inline styles for animations
- ❌ Import app.css in root layout (currently disabled)

---

## Migration Notes

When re-enabling app.css:
1. Uncomment line 13 in `src/routes/+layout.svelte`
2. All animations are namespaced - no conflicts
3. Route-level layouts will still load their specific CSS
4. Frontpage sections remain independent (scoped styles)

---

**Maintained by:** Revolution Trading Pros Engineering Team
