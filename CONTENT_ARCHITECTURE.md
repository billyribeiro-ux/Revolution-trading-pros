# Content Architecture - Revolution Trading Pros
**Apple Principal Engineer ICT 7 Grade - January 2026**

Master specification for all content types across trading rooms. This is the SINGLE SOURCE OF TRUTH for content architecture.

---

## Content Types Overview

### 1. Weekly Watchlist
**Purpose:** Stock picks and trade setups for the week
**Scope:** Room-specific (only certain trading rooms display this)
**Database Table:** `room_watchlist` (TBD - to be created)

**Rooms that display Weekly Watchlist:**
- [ ] Day Trading Room
- [ ] Swing Trading Room
- [ ] Small Account Mentorship
- [ ] Explosive Swings
- [ ] SPX Profit Pulse
- [ ] Weekly Watchlist (dedicated room)

**Data Structure:**
```typescript
interface WeeklyWatchlistEntry {
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

---

### 2. Daily Updates
**Purpose:** Daily market analysis and trade updates
**Scope:** Room-specific (each room has its own daily updates)
**Database Table:** `room_resources` (existing, filtered by `resource_type = 'daily_update'`)

**Rooms with Daily Updates:**
- [ ] Day Trading Room
- [ ] Swing Trading Room
- [ ] Small Account Mentorship
- [ ] Explosive Swings
- [ ] SPX Profit Pulse

**Data Structure:**
```typescript
interface DailyUpdate {
  id: number;
  room_slug: string;
  title: string;
  description: string;
  video_url?: string;
  thumbnail_url?: string;
  duration?: string;
  published_at: string;
  resource_type: 'daily_update';
}
```

---

### 3. Weekly Videos (Weekly Breakdown)
**Purpose:** Weekly market breakdown and trade plan overview
**Scope:** Room-specific (each room has its own weekly video)
**Database Table:** `room_weekly_videos` (existing)

**Rooms with Weekly Videos:**
- [x] Explosive Swings (implemented)
- [ ] SPX Profit Pulse
- [ ] Day Trading Room
- [ ] Swing Trading Room
- [ ] Small Account Mentorship

**Data Structure:**
```typescript
interface WeeklyVideo {
  id: number;
  room_id: number;
  room_slug: string;
  week_of: string; // ISO date
  week_title: string;
  video_title: string;
  video_url: string; // Bunny.net iframe URL
  video_platform: string; // 'bunny' | 'youtube' | 'vimeo'
  thumbnail_url?: string;
  duration?: string;
  description?: string;
  is_current: boolean;
  is_published: boolean;
  published_at: string;
}
```

**API Endpoint:** `/api/weekly-video/{room_slug}`

---

### 4. Market Updates
**Purpose:** General market commentary and macro analysis
**Scope:** Global OR room-specific (configurable)
**Database Table:** `market_updates` (TBD - to be created)

**Display Options:**
- [ ] Global (shows on all rooms)
- [ ] Room-specific (admin selects which rooms display each update)

**Data Structure:**
```typescript
interface MarketUpdate {
  id: number;
  title: string;
  description: string;
  video_url?: string;
  thumbnail_url?: string;
  published_at: string;
  is_global: boolean; // true = shows on all rooms
  room_slugs?: string[]; // if not global, which rooms to display on
}
```

---

### 5. Trade Alerts
**Purpose:** Real-time entry/exit/update alerts
**Scope:** Room-specific
**Database Table:** `room_alerts` (existing)

**Data Structure:**
```typescript
interface TradeAlert {
  id: number;
  room_slug: string;
  alert_type: 'ENTRY' | 'EXIT' | 'UPDATE';
  ticker: string;
  title: string;
  message: string;
  price?: number;
  notes?: string;
  published_at: string;
}
```

---

### 6. Trade Plans
**Purpose:** Weekly trade setups with entry/exit targets
**Scope:** Room-specific
**Database Table:** `room_trade_plans` (existing)

**Data Structure:**
```typescript
interface TradePlan {
  id: number;
  room_slug: string;
  week_of: string;
  ticker: string;
  bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
  entry: string;
  target1: string;
  target2?: string;
  target3?: string;
  runner?: string;
  stop: string;
  options_strike?: string;
  options_exp?: string;
  notes?: string;
}
```

---

## Trading Rooms Matrix

| Room | Weekly Watchlist | Daily Updates | Weekly Videos | Market Updates | Trade Alerts | Trade Plans |
|------|------------------|---------------|---------------|----------------|--------------|-------------|
| **Day Trading Room** | TBD | ✅ | TBD | TBD | ✅ | ✅ |
| **Swing Trading Room** | TBD | ✅ | TBD | TBD | ✅ | ✅ |
| **Small Account Mentorship** | TBD | ✅ | TBD | TBD | ✅ | ✅ |
| **Explosive Swings** | TBD | ✅ | ✅ | TBD | ✅ | ✅ |
| **SPX Profit Pulse** | TBD | ✅ | TBD | TBD | ✅ | ✅ |
| **Weekly Watchlist** | ✅ | ❌ | ❌ | TBD | ❌ | ❌ |

---

## Database Schema (Current State)

### Existing Tables
- ✅ `room_weekly_videos` - Weekly breakdown videos
- ✅ `room_alerts` - Trade alerts
- ✅ `room_trade_plans` - Weekly trade setups
- ✅ `room_resources` - General resources (videos, PDFs, etc.)
- ✅ `trading_rooms` - Room metadata

### Tables to Create
- ⏳ `room_watchlist` - Weekly watchlist entries
- ⏳ `market_updates` - Market commentary (global or room-specific)

---

## API Endpoints (Current State)

### Existing
- ✅ `GET /api/weekly-video/{room_slug}` - Get current weekly video
- ✅ `POST /api/admin/room-content/weekly-video` - Create weekly video
- ✅ `GET /api/alerts/{room_slug}` - Get room alerts
- ✅ `GET /api/trade-plans/{room_slug}` - Get trade plans
- ✅ `GET /api/resources/{room_slug}` - Get room resources

### To Create
- ⏳ `GET /api/watchlist/{room_slug}` - Get weekly watchlist
- ⏳ `GET /api/market-updates` - Get market updates (global or filtered by room)

---

## Frontend Components (Current State)

### Explosive Swings Dashboard
- ✅ `WeeklyHero.svelte` - Weekly video player + trade plan tabs
- ✅ `AlertsFeed.svelte` - Live alerts feed
- ✅ `PerformanceSummary.svelte` - Stats and closed trades
- ✅ `VideoGrid.svelte` - Recent video updates

### Shared Components
- ✅ `TradingRoomHeader.svelte` - Room navigation
- ✅ `DashboardBreadcrumbs.svelte` - Breadcrumb navigation

---

## Next Steps (To Be Determined)

1. **Define Weekly Watchlist scope** - Which rooms display it?
2. **Define Market Updates scope** - Global or room-specific?
3. **Create database migrations** for new tables
4. **Build API endpoints** for new content types
5. **Update frontend components** to display new content types

---

**Last Updated:** January 24, 2026
**Status:** 🚧 In Progress - Gathering requirements
