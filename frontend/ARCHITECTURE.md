# Revolution Trading Pros - Frontend Architecture

## Overview

This document describes the frontend architecture for Revolution Trading Pros, designed for Netflix-level reliability, scalability, and developer experience.

**Version:** 2.0.0  
**Stack:** SvelteKit 5, Svelte 5, Tailwind CSS 4, TypeScript 5

---

## Directory Structure

```
src/
├── lib/
│   ├── api/                    # API clients and configuration
│   │   ├── config.ts           # Base API configuration
│   │   ├── client.ts           # Enhanced HTTP client
│   │   └── [domain].ts         # Domain-specific API modules
│   │
│   ├── components/
│   │   ├── ui/                 # Primitive UI components
│   │   │   ├── Button.svelte
│   │   │   ├── Input.svelte
│   │   │   ├── Modal.svelte
│   │   │   └── index.ts
│   │   │
│   │   ├── patterns/           # Composed pattern components
│   │   │   ├── DataTable.svelte
│   │   │   ├── StatCard.svelte
│   │   │   ├── EmptyState.svelte
│   │   │   ├── ErrorBoundary.svelte
│   │   │   ├── PageHeader.svelte
│   │   │   └── index.ts
│   │   │
│   │   ├── layout/             # Layout shell components
│   │   │   ├── MarketingNav.svelte
│   │   │   ├── MarketingFooter.svelte
│   │   │   ├── AppSidebar.svelte
│   │   │   ├── AdminSidebar.svelte
│   │   │   ├── TradingRoomShell.svelte
│   │   │   └── index.ts
│   │   │
│   │   ├── charts/             # Chart components and theming
│   │   │   ├── EnterpriseChart.svelte
│   │   │   ├── ChartTheme.ts
│   │   │   └── index.ts
│   │   │
│   │   └── index.ts            # Main barrel export
│   │
│   ├── stores/                 # Svelte stores
│   │   ├── auth.ts
│   │   ├── cart.ts
│   │   └── ...
│   │
│   ├── observability/          # Metrics, experiments, telemetry
│   │   ├── metrics.ts          # Event tracking
│   │   ├── experiments.ts      # Feature flags & A/B tests
│   │   ├── telemetry.ts        # OpenTelemetry integration
│   │   └── index.ts
│   │
│   ├── utils/                  # Utility functions
│   └── types/                  # TypeScript type definitions
│
├── routes/
│   ├── +layout.svelte          # Root layout (global chrome)
│   ├── +page.svelte            # Homepage
│   │
│   ├── (marketing)/            # Marketing pages (future route group)
│   │   ├── about/
│   │   ├── courses/
│   │   └── ...
│   │
│   ├── dashboard/              # Member dashboard
│   ├── account/                # Account settings
│   │
│   ├── live-trading-rooms/     # Trading room pages
│   │   ├── day-trading/
│   │   ├── swing-trading/
│   │   └── small-accounts/
│   │
│   └── admin/                  # Admin dashboard
│       ├── +layout.svelte      # Admin layout shell
│       ├── members/
│       ├── crm/
│       └── ...
│
└── app.css                     # Global styles and design tokens
```

---

## Design System

### Color Tokens

All colors are defined in `app.css` using Tailwind CSS 4's `@theme` directive:

```css
@theme {
  /* Primary Brand Colors */
  --color-rtp-primary: oklch(0.60 0.20 250);
  --color-rtp-primary-light: oklch(0.70 0.18 250);
  --color-rtp-primary-dark: oklch(0.50 0.22 250);
  
  /* Surface Colors */
  --color-rtp-bg: oklch(0.12 0.02 250);
  --color-rtp-surface: oklch(0.18 0.02 250);
  --color-rtp-surface-elevated: oklch(0.22 0.02 250);
  
  /* Text Colors */
  --color-rtp-text: oklch(0.96 0.01 250);
  --color-rtp-muted: oklch(0.60 0.02 250);
  
  /* Semantic Colors */
  --color-rtp-success: oklch(0.65 0.20 145);
  --color-rtp-error: oklch(0.60 0.22 25);
  --color-rtp-warning: oklch(0.75 0.18 85);
}
```

### Usage in Components

```svelte
<div class="bg-rtp-surface text-rtp-text border-rtp-border">
  <!-- Content -->
</div>

<!-- Or with CSS variables -->
<style>
  .card {
    background: var(--color-rtp-surface);
    color: var(--color-rtp-text);
  }
</style>
```

---

## Component Architecture

### Importing Components

```typescript
// Import from main barrel
import { Button, DataTable, StatCard } from '$lib/components';

// Or from specific category
import { Button, Input } from '$lib/components/ui';
import { DataTable, EmptyState } from '$lib/components/patterns';
import { MarketingNav } from '$lib/components/layout';
```

### Component Categories

| Category | Purpose | Examples |
|----------|---------|----------|
| **ui/** | Primitive, atomic components | Button, Input, Modal, Badge |
| **patterns/** | Composed, reusable patterns | DataTable, StatCard, EmptyState |
| **layout/** | Page shells and navigation | MarketingNav, AdminSidebar |
| **charts/** | Data visualization | EnterpriseChart, ChartTheme |

---

## Layout System

### Route-Based Layouts

The root layout (`+layout.svelte`) automatically detects the route and applies appropriate chrome:

```svelte
{#if isAdminArea || isTradingRoom || isEmbedArea}
  <!-- No marketing chrome -->
  <slot />
{:else}
  <!-- Marketing chrome -->
  <MarketingNav />
  <slot />
  <MarketingFooter />
{/if}
```

### Layout Components

- **MarketingNav** - Navigation for public/marketing pages
- **MarketingFooter** - Footer for public pages
- **AdminSidebar** - Sidebar navigation for admin dashboard
- **AppSidebar** - Sidebar for authenticated member area
- **TradingRoomShell** - Full-screen layout for live trading rooms

---

## Observability

### Event Tracking

```typescript
import { track, Events, trackPageView } from '$lib/observability';

// Track page view
trackPageView({ title: 'Dashboard' });

// Track custom event
track(Events.BUTTON_CLICKED, {
  button_name: 'signup',
  location: 'hero'
});

// Track with timer
const endTimer = startTimer('checkout_flow');
// ... do work ...
endTimer(); // Automatically tracks duration
```

### Feature Flags

```typescript
import { useFeatureFlag, useExperiment } from '$lib/observability';

// Check feature flag
if (useFeatureFlag('new_dashboard')) {
  // Show new dashboard
}

// Get experiment variant
const variant = useExperiment('pricing_page');
if (variant === 'treatment_a') {
  // Show treatment A
}
```

---

## Performance Guidelines

### Code Splitting

Heavy libraries are automatically code-split via `vite.config.js`:

```javascript
manualChunks: {
  'vendor-svelte': ['svelte', '@sveltejs/kit'],
  'vendor-charts': ['lightweight-charts'],
  'vendor-animation': ['gsap'],
  'vendor-icons': ['@tabler/icons-svelte'],
}
```

### Lazy Loading

Use dynamic imports for heavy components:

```svelte
<script>
  import { onMount } from 'svelte';
  
  let ChartComponent;
  
  onMount(async () => {
    const module = await import('$lib/components/charts');
    ChartComponent = module.EnterpriseChart;
  });
</script>

{#if ChartComponent}
  <svelte:component this={ChartComponent} data={chartData} />
{/if}
```

### Image Optimization

- Use WebP format for images
- Implement lazy loading with `loading="lazy"`
- Use appropriate `srcset` for responsive images

---

## Adding New Features

### 1. Create a New Page

```bash
# Create route
mkdir -p src/routes/my-feature
touch src/routes/my-feature/+page.svelte
touch src/routes/my-feature/+page.server.ts  # For SSR data
```

### 2. Use Standard Components

```svelte
<script lang="ts">
  import { PageHeader, DataTable, StatCard } from '$lib/components';
  import { track, Events } from '$lib/observability';
  import { onMount } from 'svelte';
  
  export let data;
  
  onMount(() => {
    track(Events.PAGE_VIEW, { page: 'my_feature' });
  });
</script>

<PageHeader 
  title="My Feature" 
  description="Description of the feature"
>
  <svelte:fragment slot="actions">
    <button class="btn-primary">Action</button>
  </svelte:fragment>
</PageHeader>

<DataTable 
  columns={columns} 
  data={data.items}
  on:rowClick={handleRowClick}
/>
```

### 3. Add API Integration

```typescript
// src/lib/api/my-feature.ts
import { api } from './config';

export async function getItems() {
  return api.get('/api/my-feature/items');
}

export async function createItem(data: CreateItemDto) {
  return api.post('/api/my-feature/items', data);
}
```

---

## Testing

### Unit Tests

```bash
npm run test:unit
```

### E2E Tests

```bash
npm run test        # Headless
npm run test:ui     # With UI
npm run test:headed # Headed mode
```

---

## Build & Deploy

```bash
# Development
npm run dev

# Production build
npm run build

# Preview production build
npm run preview

# Type checking
npm run check
```

---

## Migration Notes

### From Previous Architecture

1. **Layout Components** - Extracted from monolithic layouts into `$lib/components/layout/`
2. **Design Tokens** - Consolidated in `app.css` with `@theme` directive
3. **Pattern Components** - New reusable patterns in `$lib/components/patterns/`
4. **Observability** - New metrics and experiments infrastructure in `$lib/observability/`

### Breaking Changes

- Import paths changed: Use `$lib/components` instead of direct file imports
- Admin layout now uses `AdminSidebar` component
- Root layout detects route type automatically

---

## Support

For questions about the architecture, contact the frontend team or refer to the component documentation in Storybook (coming soon).
