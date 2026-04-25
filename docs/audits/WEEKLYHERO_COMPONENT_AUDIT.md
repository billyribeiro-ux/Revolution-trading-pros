# WeeklyHero Component System Audit
**Apple Principal Engineer ICT 7 Standards**  
**Date:** January 24, 2026  
**Component:** `WeeklyHero.svelte`

---

## 1. File Location & Structure

### Primary Location
```
/frontend/src/routes/dashboard/explosive-swings/components/WeeklyHero.svelte
```

### Component Folder Pattern
```
/frontend/src/routes/dashboard/explosive-swings/
├── +page.svelte                    # Main dashboard page
├── +page.server.ts                 # Server-side data loading
├── +page.ts                        # Client-side page config
├── types.ts                        # Local type definitions
├── CSS_TRACE_AUDIT.md             # CSS cleanup documentation
├── components/                     # Page-specific components
│   ├── WeeklyHero.svelte          # ⭐ TARGET COMPONENT
│   ├── PerformanceSummary.svelte  # Stats + ticker pills
│   ├── AlertsFeed.svelte          # Live alerts feed
│   ├── AlertCard.svelte           # Individual alert display
│   ├── Sidebar.svelte             # Right sidebar
│   ├── VideoGrid.svelte           # Recent videos grid
│   ├── VideoUploadModal.svelte    # Admin video upload
│   ├── TradeEntryModal.svelte     # Admin trade entry
│   ├── ClosePositionModal.svelte  # Admin close position
│   ├── AddTradeModal.svelte       # Admin add trade
│   ├── ActivePositionCard.svelte  # Position display
│   ├── TickerPill.svelte          # Closed trade pills
│   ├── WeeklyVideoCard.svelte     # Sidebar video card
│   ├── VideoCard.svelte           # Video grid item
│   ├── PerformanceCard.svelte     # Performance metrics
│   ├── LatestUpdatesCard.svelte   # Updates display
│   └── ResourceLinks.svelte       # Resource links
└── utils/
    └── formatters.ts              # Utility functions
```

### Repository-Wide Component Pattern
```
/frontend/src/lib/components/      # Shared global components
├── dashboard/                     # Dashboard-specific shared
│   ├── TradingRoomHeader.svelte  # Used by WeeklyHero's parent
│   ├── DashboardBreadcrumbs.svelte
│   ├── TradeAlertModal.svelte
│   ├── alerts/
│   ├── pagination/
│   ├── stats/
│   └── video/
├── video/
│   └── BunnyVideoPlayer.svelte   # Potential replacement for inline player
├── ui/                           # Generic UI components
├── forms/                        # Form components
└── Modal.svelte                  # Base modal component
```

---

## 2. Existing Patterns

### Component Architecture Pattern
**Evidence:** All components in `/explosive-swings/components/` follow this structure:

```typescript
<script lang="ts">
  // 1. Type definitions (local interfaces)
  interface Props { ... }
  
  // 2. Props destructuring with Svelte 5 $props() rune
  const { prop1, prop2 = defaultValue }: Props = $props();
  
  // 3. Local state with $state rune
  let localState = $state(initialValue);
  
  // 4. Derived values with $derived rune
  const computed = $derived(expression);
  
  // 5. Functions
  function handler() { ... }
</script>

<!-- 6. Template -->
<div class="component-name">
  <!-- markup -->
</div>

<!-- 7. Scoped styles -->
<style>
  .component-name { ... }
</style>
```

### CSS Strategy
**Current Implementation:** Scoped CSS in `<style>` blocks

**Evidence from WeeklyHero.svelte:**
- Lines 340-1165: ~825 lines of scoped CSS
- Uses BEM-like naming: `.hero`, `.hero-tabs-bar`, `.hero-tab`
- No CSS modules, no Tailwind utility classes in component
- Custom properties for theming: `var(--color-rtp-primary)`

**Global CSS System:**
```
/frontend/src/app.css
├── @import 'tailwindcss'
├── @theme { ... }                 # Design tokens
├── Reveal animations (.reveal)
├── Marketing animations (marketing-*)
└── Breakpoint variables (--breakpoint-*)
```

**Design Tokens (from app.css):**
```css
--color-rtp-bg: #050505
--color-rtp-surface: #0a0a0a
--color-rtp-border: rgba(255, 255, 255, 0.08)
--color-rtp-text: #f4f4f5
--color-rtp-muted: #71717a
--color-rtp-primary: #3b82f6
--breakpoint-sm: 640px
--breakpoint-md: 768px
--breakpoint-lg: 1024px
--breakpoint-xl: 1280px
--breakpoint-2xl: 1536px
```

**Dashboard Color Scheme:**
- Primary accent: `#143E59` (dark teal/navy)
- Replaces legacy `#0984ae` (light blue)

### Testing Setup
**Framework:** Playwright + Vitest

**Evidence from package.json:**
```json
{
  "scripts": {
    "test": "playwright test",
    "test:ui": "playwright test --ui",
    "test:e2e": "playwright test tests/e2e",
    "test:unit": "vitest",
    "test:unit:ui": "vitest --ui"
  },
  "devDependencies": {
    "@playwright/test": "^1.57.0",
    "vitest": "^4.0.15",
    "@vitest/browser-playwright": "^4.0.15"
  }
}
```

**Test Location Pattern:**
```
/frontend/tests/
├── e2e/                          # Playwright E2E tests
│   ├── smoke/
│   ├── auth/
│   ├── trading-room/
│   └── checkout/
└── unit/                         # Vitest unit tests
```

---

## 3. Dependencies & Imports

### WeeklyHero.svelte Current Imports
**None** - Component is self-contained with inline types

### Parent Component (+page.svelte) Imports
```typescript
// Svelte core
import { onMount } from 'svelte';

// Shared dashboard components
import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
import TradeAlertModal from '$lib/components/dashboard/TradeAlertModal.svelte';
import Pagination from '$lib/components/dashboard/pagination/Pagination.svelte';

// Legacy shared components (marked for replacement)
import AlertCard from '$lib/components/dashboard/alerts/AlertCard.svelte';
import AlertFilters from '$lib/components/dashboard/alerts/AlertFilters.svelte';
import StatsBar from '$lib/components/dashboard/stats/StatsBar.svelte';

// Local page-specific components
import PerformanceSummary from './components/PerformanceSummary.svelte';
import AlertsFeed from './components/AlertsFeed.svelte';
import SidebarComponent from './components/Sidebar.svelte';
import VideoGrid from './components/VideoGrid.svelte';
import WeeklyHero from './components/WeeklyHero.svelte';  // ⭐ TARGET
import TradeEntryModal from './components/TradeEntryModal.svelte';
import VideoUploadModal from './components/VideoUploadModal.svelte';
import ClosePositionModal from './components/ClosePositionModal.svelte';
import AddTradeModal from './components/AddTradeModal.svelte';

// Types
import type { Alert, AlertFilter, QuickStats, VideoUpdate } from '$lib/components/dashboard/alerts/types';
import type { TradePlanEntry, RoomAlert, RoomStats, AlertCreateInput, AlertUpdateInput } from '$lib/types/trading';
import type { WatchlistResponse } from '$lib/types/watchlist';
import type { RoomResource } from '$lib/api/room-resources';
```

### Components That Import WeeklyHero
**Only one:** `/routes/dashboard/explosive-swings/+page.svelte`

**Usage:**
```svelte
<WeeklyHero 
  {weeklyContent} 
  {tradePlan} 
  {isAdmin}
  roomSlug={ROOM_SLUG}
  onAddEntry={openTradeEntryModal}
  onEditEntry={openEditTradeEntryModal}
  onUploadVideo={openVideoUploadModal}
/>
```

---

## 4. Type Definitions

### Local Types (WeeklyHero.svelte)
```typescript
interface WeeklyContent {
  title: string;           // "Week of January 13, 2026"
  thumbnail: string;       // Thumbnail URL
  duration: string;        // "24:35"
  videoTitle: string;      // "Weekly Breakdown: Top Swing Setups"
  videoUrl: string;        // Bunny.net iframe URL
  publishedDate: string;   // "January 13, 2026 at 9:00 AM ET"
}

interface TradePlanEntry {
  ticker: string;          // "NVDA"
  bias: string;            // "BULLISH" | "BEARISH" | "NEUTRAL"
  entry: string;           // "$142.50"
  target1: string;         // "$148.00"
  target2: string;         // "$152.00"
  target3: string;         // "$158.00"
  runner: string;          // "$165.00+"
  stop: string;            // "$136.00"
  optionsStrike: string;   // "$145 Call"
  optionsExp: string;      // "Jan 24, 2026"
  notes: string;           // Trade notes
}

interface Props {
  weeklyContent: WeeklyContent;
  tradePlan: TradePlanEntry[];
  videoUrl?: string;       // Optional override (defaults to weeklyContent.videoUrl)
  sheetUrl?: string;       // Google Sheets URL
  isAdmin?: boolean;       // Show admin controls
  roomSlug?: string;       // "explosive-swings"
  onAddEntry?: () => void;
  onEditEntry?: (entry: TradePlanEntry) => void;
  onUploadVideo?: () => void;
}
```

### Shared Types (types.ts)
**Location:** `/routes/dashboard/explosive-swings/types.ts`

```typescript
// Performance metrics
export interface WeeklyPerformance { ... }
export interface ThirtyDayPerformance { ... }

// Trades
export interface ClosedTrade { ... }
export interface ActivePosition { ... }

// Alerts
export interface Alert { ... }
export type AlertType = 'ENTRY' | 'UPDATE' | 'EXIT';

// Videos
export interface WeeklyVideo { ... }
export interface Video { ... }

// Color mappings
export const alertColors = { ... }
export const positionStatusColors = { ... }
export const performanceColors = { ... }
```

### Global Types
**Location:** `/lib/types/`

```
/lib/types/
├── trading.ts           # TradePlanEntry, RoomAlert, RoomStats
├── watchlist.ts         # WatchlistResponse
└── ...
```

**No barrel pattern** - Direct imports from specific type files

---

## 5. State Management

### State Architecture
**Pattern:** Svelte 5 runes (no external stores for this component)

**Parent Component State (+page.svelte):**
```typescript
// API state
let apiWeeklyVideo = $state<ApiWeeklyVideo | null>(null);
let apiAlerts = $state<RoomAlert[]>([]);
let apiTradePlan = $state<ApiTradePlanEntry[]>([]);
let apiStats = $state<RoomStats | null>(null);

// Loading states
let isLoadingAlerts = $state(false);
let isLoadingTradePlan = $state(false);
let isLoadingStats = $state(false);
let isLoadingVideos = $state(false);

// Admin state
let isAdmin = $state(false);
let isVideoUploadModalOpen = $state(false);
```

### Data Fetching Pattern
**Location:** `+page.svelte` `onMount()` lifecycle

```typescript
onMount(async () => {
  await checkAdminStatus();
  fetchAlerts();
  fetchTradePlan();
  fetchStats();
  fetchAllTrades();
  fetchWeeklyVideo();  // ⭐ Populates apiWeeklyVideo
});

async function fetchWeeklyVideo() {
  isLoadingVideos = true;
  try {
    const response = await fetch(`/api/weekly-video/${ROOM_SLUG}`, {
      credentials: 'include'
    });
    const data = await response.json();
    if (data.success && data.data) {
      apiWeeklyVideo = data.data;
    }
  } catch (err) {
    console.error('Failed to fetch weekly video:', err);
  } finally {
    isLoadingVideos = false;
  }
}
```

### Data Transformation
**Pattern:** `$derived` runes transform API data to component props

```typescript
// Transform API data to WeeklyContent interface
const weeklyContent = $derived<WeeklyContent>(
  apiWeeklyVideo
    ? {
        title: apiWeeklyVideo.week_title || `Week of ${...}`,
        videoTitle: apiWeeklyVideo.video_title,
        videoUrl: apiWeeklyVideo.video_url,  // Bunny.net iframe URL
        thumbnail: apiWeeklyVideo.thumbnail_url || 'https://placehold.co/...',
        duration: apiWeeklyVideo.duration || '',
        publishedDate: new Date(apiWeeklyVideo.published_at).toLocaleDateString(...)
      }
    : fallbackWeeklyContent  // Fallback if API fails
);

// Transform API trade plan to component format
const tradePlan = $derived<TradePlanEntry[]>(
  apiTradePlan.length > 0
    ? apiTradePlan.map(entry => ({
        ticker: entry.ticker,
        bias: entry.bias,
        entry: entry.entry,
        // ... map all fields
      }))
    : fallbackTradePlan  // Fallback data
);
```

### Global Stores (Available but NOT used by WeeklyHero)
**Location:** `/lib/stores/`

```typescript
// Auth store (Svelte 5 runes)
import { authStore } from '$lib/stores/auth.svelte';

// Other available stores
import { cartStore } from '$lib/stores/cart.svelte';
import { themeStore } from '$lib/stores/theme.svelte';
import { toastStore } from '$lib/stores/toast.svelte';
```

**Pattern:** Svelte 5 class-based stores using `$state` internally

---

## 6. Integration Points (MUST NOT BREAK)

### Parent Component Contract
**File:** `+page.svelte`  
**Lines:** 1247-1258

```svelte
<WeeklyHero 
  {weeklyContent}        # REQUIRED: Derived from apiWeeklyVideo
  {tradePlan}            # REQUIRED: Derived from apiTradePlan
  {isAdmin}              # REQUIRED: Controls admin UI visibility
  roomSlug={ROOM_SLUG}   # REQUIRED: "explosive-swings"
  onAddEntry={openTradeEntryModal}      # REQUIRED: Opens TradeEntryModal
  onEditEntry={openEditTradeEntryModal} # REQUIRED: Opens TradeEntryModal with data
  onUploadVideo={openVideoUploadModal}  # REQUIRED: Opens VideoUploadModal
/>
```

### Data Flow Chain (CRITICAL)
```
1. Backend API
   ↓
2. Frontend API Proxy (/api/weekly-video/explosive-swings)
   ↓
3. fetchWeeklyVideo() in +page.svelte
   ↓
4. apiWeeklyVideo $state variable
   ↓
5. weeklyContent $derived transformation
   ↓
6. WeeklyHero component prop
   ↓
7. Video player display
```

**Breaking this chain = video player shows fallback data instead of uploaded videos**

### Modal Integration
**VideoUploadModal.svelte** calls `onSuccess={fetchWeeklyVideo}` after upload:
```svelte
<VideoUploadModal
  isOpen={isVideoUploadModalOpen}
  roomSlug={ROOM_SLUG}
  onClose={closeVideoUploadModal}
  onSuccess={fetchWeeklyVideo}  # ⭐ Refreshes video data
/>
```

### CSS Dependencies
**From CSS_TRACE_AUDIT.md:**
- WeeklyHero has 825 lines of scoped CSS (lines 340-1165)
- Uses custom properties from `app.css`
- No external CSS dependencies
- Self-contained styling

---

## 7. Recommended Refactoring Approach

### Phase 1: Extract Video Player Component
**Rationale:** Video player logic is complex and reusable

**New Component:** `VideoPlayer.svelte`
```typescript
interface Props {
  videoUrl: string;
  thumbnail: string;
  duration: string;
  title: string;
  autoplay?: boolean;
}
```

**Benefits:**
- Testable in isolation
- Reusable across dashboard
- Cleaner WeeklyHero code
- Easier to add features (captions, quality selector, etc.)

### Phase 2: Extract Trade Plan Table
**Rationale:** Trade plan is a distinct feature

**New Component:** `TradePlanTable.svelte`
```typescript
interface Props {
  entries: TradePlanEntry[];
  isAdmin?: boolean;
  onAddEntry?: () => void;
  onEditEntry?: (entry: TradePlanEntry) => void;
}
```

**Benefits:**
- Reusable for other trading rooms
- Easier to add sorting/filtering
- Cleaner separation of concerns

### Phase 3: Reduce CSS Complexity
**Current:** 825 lines of scoped CSS  
**Target:** ~300 lines using composition

**Strategy:**
1. Extract common patterns to utility classes in `app.css`
2. Use Tailwind utilities for spacing/colors where appropriate
3. Keep component-specific styles scoped
4. Document design tokens

### Phase 4: Add Unit Tests
**Framework:** Vitest with `@vitest/browser-playwright`

**Test Coverage:**
- Video URL validation (`isValidVideoUrl`)
- Embed URL generation (`getEmbedUrl`)
- Tab switching logic
- Collapse/expand behavior
- Trade notes expansion

**Example:**
```typescript
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import WeeklyHero from './WeeklyHero.svelte';

describe('WeeklyHero', () => {
  it('validates Bunny.net video URLs', () => {
    const url = 'https://iframe.mediadelivery.net/embed/585929/...';
    expect(isValidVideoUrl(url)).toBe(true);
  });
});
```

---

## 8. File Tree (Relevant Directories)

```
/frontend/
├── src/
│   ├── app.css                                    # Global styles + design tokens
│   ├── lib/
│   │   ├── components/
│   │   │   ├── dashboard/                        # Shared dashboard components
│   │   │   │   ├── TradingRoomHeader.svelte
│   │   │   │   ├── DashboardBreadcrumbs.svelte
│   │   │   │   ├── TradeAlertModal.svelte
│   │   │   │   └── video/
│   │   │   │       └── BunnyVideoPlayer.svelte   # Potential video player base
│   │   │   ├── Modal.svelte                      # Base modal component
│   │   │   └── VideoEmbed.svelte                 # Multi-platform video embed
│   │   ├── stores/                               # Global Svelte 5 stores
│   │   │   ├── auth.svelte.ts
│   │   │   ├── cart.svelte.ts
│   │   │   └── theme.svelte.ts
│   │   └── types/                                # Global type definitions
│   │       ├── trading.ts
│   │       └── watchlist.ts
│   └── routes/
│       ├── api/
│       │   └── weekly-video/
│       │       └── [slug]/
│       │           └── +server.ts                # API proxy to backend
│       └── dashboard/
│           └── explosive-swings/
│               ├── +page.svelte                  # ⭐ Parent component
│               ├── +page.server.ts               # Server-side data
│               ├── types.ts                      # Local types
│               ├── CSS_TRACE_AUDIT.md           # CSS documentation
│               ├── components/
│               │   ├── WeeklyHero.svelte        # ⭐ TARGET COMPONENT
│               │   ├── PerformanceSummary.svelte
│               │   ├── AlertsFeed.svelte
│               │   ├── VideoUploadModal.svelte
│               │   └── ... (15 other components)
│               └── utils/
│                   └── formatters.ts
├── tests/
│   ├── e2e/                                      # Playwright E2E tests
│   └── unit/                                     # Vitest unit tests
├── package.json                                  # Dependencies
├── playwright.config.ts                          # Playwright config
└── vite.config.ts                               # Vite + Tailwind config
```

---

## 9. Critical Constraints

### DO NOT BREAK
1. **Props interface** - Parent depends on exact prop names
2. **Event handlers** - `onAddEntry`, `onEditEntry`, `onUploadVideo` must remain
3. **Data flow** - `weeklyContent` must come from `apiWeeklyVideo` transformation
4. **Video URL validation** - Must support Bunny.net, YouTube, Vimeo
5. **Admin controls** - Must respect `isAdmin` prop
6. **Responsive design** - Must work on mobile (breakpoints in `app.css`)

### SAFE TO CHANGE
1. **Internal state** - Refactor `heroTab`, `isCollapsed`, etc.
2. **CSS structure** - Extract to child components or utilities
3. **Video player implementation** - Extract to separate component
4. **Trade plan table** - Extract to separate component
5. **Function names** - Internal functions can be renamed

---

## 10. Next Steps for Implementation

### Immediate Actions
1. **Create VideoPlayer.svelte** - Extract video player logic
2. **Create TradePlanTable.svelte** - Extract trade plan table
3. **Update WeeklyHero.svelte** - Use new child components
4. **Add unit tests** - Test extracted components
5. **Update CSS** - Reduce duplication, use design tokens

### Testing Strategy
1. **Unit tests** - Test video URL validation, embed generation
2. **Component tests** - Test VideoPlayer, TradePlanTable in isolation
3. **Integration tests** - Test WeeklyHero with child components
4. **E2E tests** - Test full video upload → display flow

### Rollout Plan
1. **Phase 1:** Extract VideoPlayer (low risk)
2. **Phase 2:** Extract TradePlanTable (low risk)
3. **Phase 3:** Refactor CSS (medium risk - visual changes)
4. **Phase 4:** Add comprehensive tests (no risk)

---

## 11. Evidence-Based Decisions

### Why Scoped CSS?
- **Evidence:** All 18 components in `/explosive-swings/components/` use scoped CSS
- **Pattern:** Consistent across codebase
- **Recommendation:** Keep scoped CSS, but reduce complexity via composition

### Why No Tailwind in Component?
- **Evidence:** WeeklyHero uses zero Tailwind classes
- **Pattern:** Dashboard components use scoped CSS for complex layouts
- **Recommendation:** Continue pattern, use Tailwind only for utilities

### Why Svelte 5 Runes?
- **Evidence:** All components use `$state`, `$derived`, `$props`
- **Pattern:** Modern Svelte 5 syntax throughout codebase
- **Recommendation:** Continue using runes, avoid legacy stores

### Why No External Stores?
- **Evidence:** WeeklyHero receives all data via props
- **Pattern:** Parent manages state, children are presentational
- **Recommendation:** Keep this pattern, don't introduce store dependencies

---

**End of Audit Report**

This document provides complete context for surgical improvements to WeeklyHero.svelte while maintaining system integrity.
