# Frontend Guide - Revolution Trading Pros
**Apple Principal Engineer ICT 7 Grade - January 2026**

Complete frontend implementation guide for SvelteKit + Tailwind CSS v4.

---

## Table of Contents
1. [Architecture](#architecture)
2. [Design System](#design-system)
3. [Component Library](#component-library)
4. [Implementation Guides](#implementation-guides)
5. [Data Structures](#data-structures)
6. [Development Tools](#development-tools)

---

## Architecture

### Tech Stack
- **Framework:** SvelteKit 2.x
- **Styling:** Tailwind CSS v4 (Vite plugin)
- **Icons:** Lucide
- **Deployment:** Cloudflare Pages
- **State:** Svelte stores + $state runes

### Project Structure
```
frontend/src/
├── routes/              # File-based routing
├── lib/
│   ├── components/      # Reusable components
│   ├── stores/          # Global state
│   ├── utils/           # Helper functions
│   └── config.ts        # API configuration
└── app.css              # Global styles + animations
```

### Animation System
**Location:** `src/app.css` lines 1561-1642

**Naming Convention:**
- `global-*` = Subtle (12px) for dashboards/admin
- `marketing-*` = Dramatic (30px) for landing pages
- `admin-*` = Admin-specific animations

**Marketing Animations:**
- `marketing-fadeIn`
- `marketing-fadeInUp` (30px)
- `marketing-fadeInDown` (30px)
- `marketing-fadeInLeft` (30px)
- `marketing-fadeInRight` (30px)
- `marketing-scaleIn`

**Used By:**
- `/alerts/*` (explosive-swings, spx-profit-pulse)
- `/live-trading-rooms/*` (all 5 rooms)
- Frontpage sections (Hero, TradingRooms, AlertServices)

---

## Design System

### Color Palette

**Primary Colors:**
- `#143E59` - Dark teal/navy (dashboard primary)
- `#0984ae` - Light blue (deprecated, replaced by #143E59)

**Dashboard Colors:**
- Secondary nav background: `#143E59`
- Active link indicators: `#143E59`
- Border highlights: `#143E59`
- Link hover states: `#143E59`

### Typography
- **Headings:** Inter (font-weight: 600-700)
- **Body:** Inter (font-weight: 400)
- **Code:** JetBrains Mono

### Spacing Scale
```css
/* Tailwind spacing */
0.5 = 2px
1 = 4px
2 = 8px
4 = 16px
6 = 24px
8 = 32px
12 = 48px
16 = 64px
```

---

## Component Library

### Classes Components
**Location:** `src/lib/components/classes/`

**Usage:**
```svelte
<script>
  import ClassCard from '$lib/components/classes/ClassCard.svelte';
  import ClassHero from '$lib/components/classes/ClassHero.svelte';
</script>

<ClassHero 
  title="Course Title"
  description="Course description"
  thumbnail="/images/course.jpg"
/>

<ClassCard
  slug="course-slug"
  title="Course Title"
  instructor="John Doe"
  duration="2 hours"
/>
```

### Dashboard Components
**Location:** `src/lib/components/dashboard/`

**Key Components:**
- `DashboardNav.svelte` - Main navigation
- `DashboardBreadcrumbs.svelte` - Breadcrumb navigation
- `StatsCard.svelte` - Metric display cards
- `VideoPlayer.svelte` - Bunny.net video player

### Admin Components
**Location:** `src/lib/components/admin/`

**Key Components:**
- `BunnyVideoUploader.svelte` - Video upload interface
- `AdminTable.svelte` - Data tables
- `AdminForm.svelte` - Form builder

---

## Implementation Guides

### Classes/Courses Implementation

**Reference:** `/indicators/[id]/+page.svelte` is the SSOT for all indicator pages.

**Requirements:**
- Exact WordPress structure match
- Mobile-first responsive design
- Svelte 5 best practices ($derived, $props)
- Breadcrumbs using DashboardBreadcrumbs component
- HaveQuestionsSection as reusable component

**Example:**
```svelte
<script>
  let { data } = $props();
  let course = $derived(data.course);
</script>

<DashboardBreadcrumbs items={[
  { label: 'Classes', href: '/classes' },
  { label: course.title }
]} />

<ClassHero {...course} />
<ClassContent lessons={course.lessons} />
```

### Indicators Implementation

**Data Structure:**
```typescript
interface Indicator {
  id: number;
  slug: string;
  title: string;
  description: string;
  thumbnail_url: string;
  video_url?: string;
  files: IndicatorFile[];
}

interface IndicatorFile {
  id: number;
  filename: string;
  platform: string; // 'thinkorswim' | 'ninjatrader' | 'tradingview'
  download_url: string;
}
```

**Implementation:**
```svelte
<script>
  import { apiFetch } from '$lib/utils/api';
  
  let indicators = $state([]);
  
  async function loadIndicators() {
    const response = await apiFetch('/api/indicators');
    indicators = response.data;
  }
</script>

{#each indicators as indicator}
  <IndicatorCard {...indicator} />
{/each}
```

### Cart System Migration

**Old System:** Laravel-based cart
**New System:** Stripe Checkout

**Migration Steps:**
1. Remove old cart components
2. Implement Stripe Checkout Sessions
3. Use `/api/checkout/create-session` endpoint
4. Redirect to Stripe-hosted checkout

**Example:**
```typescript
async function checkout(planId: string) {
  const response = await apiFetch('/api/checkout/create-session', {
    method: 'POST',
    body: JSON.stringify({ plan_id: planId })
  });
  
  window.location.href = response.checkout_url;
}
```

---

## Data Structures

### Watchlist Data Structure

**API Endpoint:** `/api/watchlist/:slug`

**Response:**
```typescript
interface WatchlistEntry {
  id: number;
  room_slug: string;
  week_of: string; // ISO date
  ticker: string;
  entry_price: number;
  target_price: number;
  stop_loss: number;
  analysis: string;
  chart_url?: string;
  status: 'active' | 'hit' | 'stopped';
}
```

**Usage:**
```svelte
<script>
  let watchlist = $state([]);
  
  async function loadWatchlist(slug: string) {
    const response = await apiFetch(`/api/watchlist/${slug}`);
    watchlist = response.data;
  }
</script>

<WatchlistTable entries={watchlist} />
```

### Trade Plan Data Structure

**API Endpoint:** `/api/rooms/:slug/trade-plan`

**Response:**
```typescript
interface TradePlan {
  id: number;
  room_slug: string;
  week_of: string;
  ticker: string;
  direction: 'LONG' | 'SHORT';
  entry_min: number;
  entry_max: number;
  target_1: number;
  target_2?: number;
  stop_loss: number;
  risk_reward: number;
  notes: string;
}
```

---

## Development Tools

### Storybook Setup

**Installation:**
```bash
pnpm add -D @storybook/sveltekit @storybook/addon-essentials
npx storybook@latest init
```

**Configuration:** `.storybook/main.ts`
```typescript
export default {
  stories: ['../src/**/*.stories.svelte'],
  addons: ['@storybook/addon-essentials'],
  framework: '@storybook/sveltekit'
};
```

**Run:**
```bash
pnpm run storybook
```

### DevTools

**Browser Extensions:**
- Svelte DevTools (Chrome/Firefox)
- Redux DevTools (for store debugging)

**VS Code Extensions:**
- Svelte for VS Code
- Tailwind CSS IntelliSense
- ESLint
- Prettier

**Local Development:**
```bash
# Install dependencies
pnpm install

# Run dev server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview
```

---

## API Integration

### Configuration
**File:** `src/lib/config.ts`

```typescript
export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';
export const CDN_URL = import.meta.env.VITE_CDN_URL;
```

### API Utilities
**File:** `src/lib/utils/api.ts`

```typescript
export async function apiFetch(endpoint: string, options = {}) {
  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    credentials: 'include', // Include cookies
    headers: {
      'Content-Type': 'application/json',
      ...options.headers
    }
  });
  
  if (!response.ok) {
    throw new Error(`API error: ${response.status}`);
  }
  
  return response.json();
}
```

---

## Testing

### Unit Tests (Vitest)
```bash
pnpm test
```

### E2E Tests (Playwright)
```bash
pnpm test:e2e
```

### Component Tests
```svelte
<script>
  import { render } from '@testing-library/svelte';
  import ClassCard from './ClassCard.svelte';
  
  test('renders class card', () => {
    const { getByText } = render(ClassCard, {
      props: { title: 'Test Course' }
    });
    expect(getByText('Test Course')).toBeInTheDocument();
  });
</script>
```

---

## Performance Optimization

### Image Optimization
- Use WebP format
- Lazy load images: `loading="lazy"`
- Serve from CDN (R2)

### Code Splitting
- SvelteKit automatically code-splits routes
- Use dynamic imports for heavy components:
  ```typescript
  const HeavyComponent = await import('./HeavyComponent.svelte');
  ```

### Caching Strategy
- Static assets: 1 year cache
- API responses: 5 minutes cache
- User data: No cache

---

## Deployment

### Build Command
```bash
pnpm run build
```

### Output Directory
`.svelte-kit/cloudflare`

### Environment Variables
Set in Cloudflare Pages dashboard:
- `VITE_API_URL`
- `VITE_CDN_URL`
- `VITE_GTM_ID`
- `PUBLIC_GA4_MEASUREMENT_ID`

---

**Last Updated:** January 24, 2026
