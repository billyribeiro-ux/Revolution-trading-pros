# RULES-TAILWIND-RESPONSIVE.md
## Tailwind CSS v4 & Breakpoints | January 2026

## 🎨 TAILWIND V4 CONFIG
```css
/* app.css */
@import "tailwindcss";

@theme {
  --breakpoint-xs: 20rem;      /* 320px */
  --breakpoint-sm: 24.5rem;    /* 392px - iPhone 16/17 */
  --breakpoint-md: 48rem;      /* 768px - iPad portrait */
  --breakpoint-lg: 64rem;      /* 1024px - iPad landscape */
  --breakpoint-xl: 80rem;      /* 1280px - Laptops */
  --breakpoint-2xl: 96rem;     /* 1536px - Large desktops */
  --breakpoint-3xl: 120rem;    /* 1920px - 4K */
  
  --color-surface-dark: #0f0f0f;
  --color-surface-card: #1a1a1a;
  --color-surface-elevated: #262626;
  --color-brand-primary: #3b82f6;
  
  --font-sans: 'Inter', system-ui, sans-serif;
  --ease-smooth: cubic-bezier(0.4, 0, 0.2, 1);
}
```

## 📱 DEVICE VIEWPORTS (2025)
| Device | Width | Height |
|--------|-------|--------|
| iPhone 17 Pro Max | 440px | 956px |
| iPhone 17/Pro | 402px | 874px |
| iPhone 16 | 393px | 852px |
| iPad Pro 13" | 1032px | 1376px |
| iPad Air 11" | 820px | 1180px |
| iPad Mini | 744px | 1133px |

## 📐 BREAKPOINTS
| Breakpoint | Width | Target |
|------------|-------|--------|
| default | 0px | Mobile base |
| sm | 392px | iPhone 16/17 |
| md | 768px | iPad portrait |
| lg | 1024px | iPad landscape |
| xl | 1280px | Desktops |
| 2xl | 1536px | Large desktops |
| 3xl | 1920px | 4K |

## 🧩 RESPONSIVE PATTERNS

### Mobile-First Component
```svelte
<div class="
  flex flex-col gap-4 p-4 text-sm
  sm:gap-5 sm:text-base
  md:flex-row md:gap-6 md:p-6
  lg:gap-8 lg:p-8 lg:max-w-5xl lg:mx-auto
  xl:max-w-6xl
">
```

### Navbar
```svelte
<nav class="fixed top-0 inset-x-0 h-14 px-4 md:h-16 md:px-6 lg:px-8">
  <a href="/"><Logo class="h-8" /></a>
  <div class="hidden lg:flex gap-6"><!-- Desktop nav --></div>
  <button class="lg:hidden min-h-[44px]"><!-- Mobile menu --></button>
</nav>
```

### Sidebar
```svelte
<aside class="
  fixed inset-y-0 left-0 w-64 -translate-x-full
  data-[open=true]:translate-x-0
  md:relative md:translate-x-0 md:w-16
  lg:w-64
">
```

### Grid
```svelte
<div class="grid gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
```

### Table → Cards on Mobile
```svelte
<table class="hidden md:table"><!-- Desktop --></table>
<div class="md:hidden flex flex-col gap-4"><!-- Mobile cards --></div>
```

## 👆 TOUCH TARGETS
```svelte
<button class="min-h-[44px] min-w-[44px]"><!-- 44px minimum --></button>
```

## 📋 CHECKLIST
```
□ Mobile-first (base = mobile)
□ Tested at 375px, 768px, 1024px, 1440px
□ Touch targets 44x44px minimum
□ Navbar collapses on mobile
□ Tables become cards on mobile
```
