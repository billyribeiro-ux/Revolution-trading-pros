# EXPLOSIVE SWINGS TRADING ROOM - NUCLEAR WORKFLOW AUDIT

## Apple Principal Engineer ICT 7+ Grade | January 2026

---

# TABLE OF CONTENTS

1. [Route Map](#1-route-map)
2. [API Endpoints](#2-api-endpoints)
3. [Component Hierarchy](#3-component-hierarchy)
4. [Data Flow Diagrams](#4-data-flow-diagrams)
5. [Member User Journey](#5-member-user-journey)
6. [Admin User Journey](#6-admin-user-journey)
7. [Gap Analysis](#7-gap-analysis)
8. [Fix Priority](#8-fix-priority)

---

# 1. ROUTE MAP

## 1.1 Full Directory Structure

```
/dashboard/explosive-swings/
├── +page.svelte                    # Main dashboard
├── +page.server.ts                 # SSR data loading
├── +page.ts                        # Client-side load
├── page.state.svelte.ts            # Centralized state module
├── page.api.ts                     # API call functions
├── types.ts                        # Type definitions (466 lines)
├── constants.ts                    # Room constants
│
├── data/
│   └── fallbacks.ts                # Fallback/mock data
│
├── utils/
│   └── formatters.ts               # Utility formatters
│
├── components/
│   ├── ActivePositionCard.svelte   # Position display with admin menu
│   ├── AddTradeModal.svelte        # Admin: Create new trade
│   ├── AlertCard.svelte            # Individual alert display
│   ├── AlertsFeed.svelte           # Alerts list component
│   ├── ClosePositionModal.svelte   # Admin: Close trade with P&L
│   ├── ErrorBanner.svelte          # Error display
│   ├── InvalidatePositionModal.svelte # Admin: Mark trade invalid
│   ├── LatestUpdatesCard.svelte    # Updates display
│   ├── PerformanceCard.svelte      # Stats card
│   ├── PerformanceSummary.svelte   # Main performance section
│   ├── ResourceLinks.svelte        # Resource links
│   ├── Sidebar.svelte              # Sidebar component
│   ├── TickerPill.svelte           # Closed trade pill
│   ├── TradeEntryModal.svelte      # Admin: Trade plan entry
│   ├── UpdatePositionModal.svelte  # Admin: Update position
│   ├── VideoCard.svelte            # Video card
│   ├── VideoGrid.svelte            # Video grid
│   ├── VideoModal.svelte           # Video player modal
│   ├── VideoUploadModal.svelte     # Admin: Upload video
│   ├── WeeklyHero.svelte           # Hero section with video + trade plan
│   └── WeeklyVideoCard.svelte      # Weekly video card
│
├── alerts/
│   ├── +page.svelte                # Alerts archive page
│   └── +page.server.ts             # SSR alerts data
│
├── trades/
│   ├── +page.svelte                # Trades history page
│   ├── +page.server.ts             # SSR trades data
│   └── components/                 # Trade-specific components
│
├── start-here/
│   └── +page.svelte                # Onboarding page
│
├── favorites/
│   └── ...                         # User favorites
│
├── video/
│   └── [slug]/                     # Individual video pages
│
├── video-library/
│   └── ...                         # Video archive
│
└── watchlist/
    └── ...                         # Watchlist feature
```

## 1.2 Route Purpose Matrix

| Route                                       | Purpose                                            | Status                |
| ------------------------------------------- | -------------------------------------------------- | --------------------- |
| `/dashboard/explosive-swings`               | Main dashboard with performance, alerts, positions | ✅ WORKING            |
| `/dashboard/explosive-swings/alerts`        | Full alerts archive with filters/search            | ✅ WORKING            |
| `/dashboard/explosive-swings/trades`        | Trade history with stats                           | ✅ EXISTS             |
| `/dashboard/explosive-swings/start-here`    | Onboarding content                                 | ✅ EXISTS             |
| `/dashboard/explosive-swings/video/[slug]`  | Individual video playback                          | ✅ EXISTS             |
| `/dashboard/explosive-swings/video-library` | Video archive                                      | ⚠️ NEEDS VERIFICATION |
| `/dashboard/explosive-swings/watchlist`     | Watchlist feature                                  | ⚠️ NEEDS VERIFICATION |
| `/dashboard/explosive-swings/favorites`     | User favorites                                     | ⚠️ NEEDS VERIFICATION |

---

# 2. API ENDPOINTS

## 2.1 Frontend API Calls (from page.api.ts)

| Function           | Endpoint                   | Method | Purpose                        |
| ------------------ | -------------------------- | ------ | ------------------------------ |
| `fetchAlerts`      | `/api/alerts/{slug}`       | GET    | Get paginated alerts           |
| `fetchTradePlan`   | `/api/trade-plans/{slug}`  | GET    | Get trade plan entries         |
| `fetchStats`       | `/api/stats/{slug}`        | GET    | Get room statistics            |
| `fetchAllTrades`   | `/api/trades/{slug}`       | GET    | Get all trades (open + closed) |
| `fetchWeeklyVideo` | `/api/weekly-video/{slug}` | GET    | Get current weekly video       |
| `checkAdminStatus` | `/api/auth/me`             | GET    | Check if user is admin         |

## 2.2 Admin Action Endpoints (from modals)

| Action                  | Endpoint                            | Method | Source                  |
| ----------------------- | ----------------------------------- | ------ | ----------------------- |
| Create Alert            | `/api/alerts/{slug}`                | POST   | TradeAlertModal         |
| Update Alert            | `/api/alerts/{slug}/{id}`           | PUT    | TradeAlertModal         |
| Delete Alert            | `/api/alerts/{slug}/{id}`           | DELETE | AlertCard               |
| Create Trade            | `/api/trades/{slug}`                | POST   | AddTradeModal           |
| Update Trade            | `/api/trades/{slug}/{id}`           | PUT    | ClosePositionModal      |
| Delete Trade            | `/api/admin/trades/{id}`            | DELETE | page.state.svelte.ts    |
| Invalidate Trade        | `/api/admin/trades/{id}/invalidate` | POST   | InvalidatePositionModal |
| Create Trade Plan Entry | `/api/trade-plans/{slug}`           | POST   | TradeEntryModal         |
| Upload Video            | `/api/weekly-video/{slug}`          | POST   | VideoUploadModal        |

## 2.3 Backend Rust Handlers (room_content.rs)

```rust
// Main router structure from room_content.rs

pub fn routes() -> Router<AppState> {
    Router::new()
        // Trade Plans
        .route("/trade-plans/:slug", get(list_trade_plan))
        .route("/trade-plans/:slug", post(create_trade_plan_entry))
        .route("/trade-plans/:slug/:id", put(update_trade_plan_entry))
        .route("/trade-plans/:slug/:id", delete(delete_trade_plan_entry))

        // Alerts
        .route("/alerts/:slug", get(list_alerts))
        .route("/alerts/:slug", post(create_alert))
        .route("/alerts/:slug/:id", get(get_alert))
        .route("/alerts/:slug/:id", put(update_alert))
        .route("/alerts/:slug/:id", delete(delete_alert))

        // Weekly Videos
        .route("/weekly-video/:slug", get(get_weekly_video))
        .route("/weekly-video/:slug", post(create_weekly_video))
        .route("/weekly-video/:slug/:id", put(update_weekly_video))

        // Stats
        .route("/stats/:slug", get(get_stats))

        // Trades
        .route("/trades/:slug", get(list_trades))
        .route("/trades/:slug", post(create_trade))
        .route("/trades/:slug/:id", put(update_trade))
        .route("/trades/:slug/:id", delete(delete_trade))
}
```

## 2.4 Room Constants

```typescript
// From constants.ts
export const ROOM_SLUG = 'explosive-swings';
export const ROOM_CONTENT_ID = 4; // For room_content API
export const ROOM_RESOURCES_ID = 2; // For room_resources API
export const ALERTS_PER_PAGE = 10;
export const TRADES_PER_PAGE = 50;
```

---

# 3. COMPONENT HIERARCHY

## 3.1 Main Dashboard (+page.svelte)

```
+page.svelte
│
├── TradingRoomHeader
│   └── Props: roomName, startHereUrl
│
├── PerformanceSummary
│   ├── Props: performance, closedTrades, activePositions, isLoading, isAdmin
│   ├── Callbacks: onClosePosition, onUpdatePosition, onInvalidatePosition,
│   │              onDeletePosition, onAddTrade
│   │
│   ├── Win Rate Badge
│   │
│   ├── Closed Trades Section
│   │   └── TickerPill[] (for each closed trade)
│   │       └── Props: trade
│   │
│   └── Active Positions Section
│       └── ActivePositionCard[] (for each position)
│           ├── Props: position, isAdmin, onUpdate, onInvalidate, onClose, onDelete
│           └── Admin Menu (⋮)
│               ├── Update Position → UpdatePositionModal
│               ├── Invalidate → InvalidatePositionModal
│               ├── Close Trade → ClosePositionModal
│               └── Delete (with confirm)
│
├── WeeklyHero
│   ├── Props: weeklyContent, tradePlan, isAdmin, roomSlug
│   ├── Callbacks: onAddEntry, onEditEntry, onUploadVideo
│   │
│   ├── Tab: Video Breakdown
│   │   ├── Video Player / Thumbnail
│   │   └── Video Info Panel
│   │       └── Admin: Upload Video Button → VideoUploadModal
│   │
│   └── Tab: Trade Plan & Entries
│       ├── Trade Plan Table (6 entries typical)
│       │   └── Per row: Ticker, Bias, Entry, T1, T2, T3, Runner, Stop, Options, Exp, Notes
│       │       └── Admin: Edit Button → TradeEntryModal
│       └── Admin: Add Entry Button → TradeEntryModal
│
├── Main Grid
│   ├── Alerts Section
│   │   ├── Section Header
│   │   │   └── Admin: "+ New Alert" Button → TradeAlertModal
│   │   │
│   │   ├── AlertFilters (All | Entry | Exit | Update)
│   │   │
│   │   ├── AlertCard[] (for each alert)
│   │   │   ├── Props: alert, index, isAdmin, isNotesExpanded, isCopied
│   │   │   ├── Callbacks: onToggleNotes, onCopy, onEdit, onDelete
│   │   │   └── Expandable notes section
│   │   │
│   │   └── Pagination
│   │
│   └── Sidebar
│       ├── 30-Day Performance Card
│       ├── Weekly Video Thumbnail
│       └── Resources/Documents
│
└── Modals (conditional render)
    ├── TradeAlertModal (from $lib)
    ├── TradeEntryModal (local)
    ├── VideoUploadModal (local)
    ├── ClosePositionModal (local)
    ├── AddTradeModal (local)
    ├── UpdatePositionModal (local)
    └── InvalidatePositionModal (local)
```

## 3.2 Modal Components Summary

| Modal                   | File                      | Purpose                        | API Endpoint                           |
| ----------------------- | ------------------------- | ------------------------------ | -------------------------------------- |
| TradeAlertModal         | $lib/components/dashboard | Create/Edit alerts             | POST/PUT /api/alerts/{slug}            |
| TradeEntryModal         | ./components              | Create/Edit trade plan entries | POST/PUT /api/trade-plans/{slug}       |
| VideoUploadModal        | ./components              | Upload weekly video            | POST /api/weekly-video/{slug}          |
| AddTradeModal           | ./components              | Create new trade/position      | POST /api/trades/{slug}                |
| UpdatePositionModal     | ./components              | Update existing position       | PATCH /api/trades/{slug}/{id}          |
| ClosePositionModal      | ./components              | Close position with exit price | PUT /api/trades/{slug}/{id}            |
| InvalidatePositionModal | ./components              | Mark trade as invalidated      | POST /api/admin/trades/{id}/invalidate |

---

# 4. DATA FLOW DIAGRAMS

## 4.1 Alert Lifecycle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           ALERT LIFECYCLE                                    │
└─────────────────────────────────────────────────────────────────────────────┘

                              ADMIN ACTION
                                   │
                                   ▼
┌─────────────┐  POST   ┌─────────────────┐  INSERT  ┌─────────────────┐
│ TradeAlert  │────────▶│ /api/alerts/    │─────────▶│ PostgreSQL      │
│ Modal       │         │ explosive-swings│          │ room_alerts     │
└─────────────┘         └─────────────────┘          └─────────────────┘
                                                              │
                              ┌────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │ Backend validates│
                    │ & stores alert   │
                    │ with TOS string  │
                    └─────────────────┘
                              │
                              ▼
┌─────────────┐  GET    ┌─────────────────┐  SELECT  ┌─────────────────┐
│ Member      │◀────────│ page.api.ts     │◀─────────│ PostgreSQL      │
│ Dashboard   │         │ fetchAlerts()   │          │ room_alerts     │
└─────────────┘         └─────────────────┘          └─────────────────┘
       │
       ▼
┌─────────────┐
│ AlertCard   │
│ component   │
└─────────────┘

ALERT TYPES:
┌──────────┐     ┌──────────┐     ┌──────────┐
│  ENTRY   │────▶│  UPDATE  │────▶│   EXIT   │
│  (teal)  │     │ (amber)  │     │(grn/red) │
└──────────┘     └──────────┘     └──────────┘

DATA FIELDS:
├── id, room_id, room_slug
├── alert_type (ENTRY|UPDATE|EXIT)
├── ticker, title, message, notes
├── TOS fields: trade_type, action, quantity, option_type, strike, expiration
├── tos_string (full ThinkOrSwim format)
├── entry_alert_id (links UPDATE/EXIT to ENTRY)
├── is_new, is_published, is_pinned
└── published_at, created_at, updated_at
```

## 4.2 Trade/Position Lifecycle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        TRADE/POSITION LIFECYCLE                              │
└─────────────────────────────────────────────────────────────────────────────┘

                    ┌─────────────────────────────────────────┐
                    │              ADMIN ACTIONS               │
                    └─────────────────────────────────────────┘
                                       │
           ┌───────────────────────────┼───────────────────────────┐
           │                           │                           │
           ▼                           ▼                           ▼
    ┌─────────────┐            ┌─────────────┐            ┌─────────────┐
    │ AddTrade    │            │ Update      │            │ Close/      │
    │ Modal       │            │ Position    │            │ Invalidate  │
    └─────────────┘            └─────────────┘            └─────────────┘
           │                           │                           │
           ▼                           ▼                           ▼
    ┌─────────────┐            ┌─────────────┐            ┌─────────────┐
    │ POST /api/  │            │ PUT /api/   │            │ PUT /api/   │
    │ trades/     │            │ trades/     │            │ trades/     │
    │ {slug}      │            │ {slug}/{id} │            │ {slug}/{id} │
    └─────────────┘            └─────────────┘            │ + status    │
           │                           │                  └─────────────┘
           └───────────────────────────┼───────────────────────────┘
                                       │
                                       ▼
                            ┌─────────────────┐
                            │   PostgreSQL    │
                            │  room_trades    │
                            └─────────────────┘
                                       │
                                       ▼
                            ┌─────────────────┐
                            │ Trade appears   │
                            │ in dashboard    │
                            └─────────────────┘

TRADE STATUS FLOW:
┌──────────┐     ┌──────────┐     ┌──────────────┐
│   OPEN   │────▶│ UPDATED  │────▶│   CLOSED     │
│ (active) │     │(wasUpdate)│    │ (win/loss)   │
└──────────┘     └──────────┘     └──────────────┘
     │                                   │
     └───────────▶ INVALIDATED ◀─────────┘
                 (reason stored)

DISPLAY MAPPING:
├── status: 'open' → ActivePositionCard (in PerformanceSummary)
├── status: 'closed' → TickerPill (in PerformanceSummary)
│                    → TradesTable (on /trades page)
├── wasUpdated: true → Shows "UPDATED" badge on card
└── status: 'invalidated' → ??? (NOT CURRENTLY DISPLAYED)
```

## 4.3 Weekly Content Lifecycle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       WEEKLY CONTENT LIFECYCLE                               │
└─────────────────────────────────────────────────────────────────────────────┘

WEEK N (Current)                          WEEK N-1 (Archived)
┌─────────────────────────┐               ┌─────────────────────────┐
│                         │               │                         │
│  ┌─────────────────┐   │   NEW WEEK    │  ┌─────────────────┐   │
│  │ Weekly Video    │   │   CREATED     │  │ Archived Video  │   │
│  │ (WeeklyHero)    │───┼──────────────▶│  │ (video-library) │   │
│  │ is_current=true │   │               │  │ is_current=false│   │
│  └─────────────────┘   │               │  └─────────────────┘   │
│                         │               │                         │
│  ┌─────────────────┐   │               │  ┌─────────────────┐   │
│  │ Trade Plan      │───┼──────────────▶│  │ Archived Plan   │   │
│  │ week_of=current │   │               │  │ week_of=prev    │   │
│  └─────────────────┘   │               │  └─────────────────┘   │
│                         │               │                         │
│  ┌─────────────────┐   │               │  ┌─────────────────┐   │
│  │ Live Alerts     │───┼──────────────▶│  │ Archived Alerts │   │
│  │ (new + active)  │   │               │  │ (read-only)     │   │
│  └─────────────────┘   │               │  └─────────────────┘   │
│                         │               │                         │
│  ┌─────────────────┐   │               │  ┌─────────────────┐   │
│  │ Active Positions│   │               │  │ Closed Trades   │   │
│  │ status='open'   │───┼──────────────▶│  │ status='closed' │   │
│  └─────────────────┘   │               │  └─────────────────┘   │
│                         │               │                         │
└─────────────────────────┘               └─────────────────────────┘

WEEKLY ROLLOVER MECHANISM:
├── Trigger: MANUAL (admin uploads new weekly video)
├── When new video with is_current=true is created:
│   └── Backend sets previous video is_current=false
├── Trade plans filtered by week_of date
├── Alerts persist in database (no auto-archive)
└── Trades persist with their status (open → closed)

⚠️ CURRENT GAPS:
├── No dedicated "archive" page for past weeks
├── No weekly grouping of alerts in archive
├── No link between weekly video and alerts batch
└── Manual process only - no automation
```

## 4.4 State Management Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    STATE MANAGEMENT (page.state.svelte.ts)                   │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                            createPageState()                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │ FILTER STATE    │  │ ADMIN STATE     │  │ MODAL STATE     │             │
│  ├─────────────────┤  ├─────────────────┤  ├─────────────────┤             │
│  │ selectedFilter  │  │ isAdmin         │  │ isAlertModalOpen│             │
│  │ currentPage     │  │                 │  │ editingAlert    │             │
│  │ pagination      │  │                 │  │ isTradeEntry... │             │
│  └─────────────────┘  └─────────────────┘  │ isVideoUpload...│             │
│                                            │ isClosePosition.│             │
│  ┌─────────────────┐  ┌─────────────────┐  │ isAddTrade...   │             │
│  │ DATA STATE      │  │ LOADING STATE   │  │ isUpdatePositio.│             │
│  ├─────────────────┤  ├─────────────────┤  │ isInvalidate... │             │
│  │ apiAlerts       │  │ isLoadingAlerts │  └─────────────────┘             │
│  │ apiTradePlan    │  │ isLoadingTrades │                                   │
│  │ apiStats        │  │ isLoadingStats  │  ┌─────────────────┐             │
│  │ apiOpenTrades   │  │ isLoadingVideos │  │ ERROR STATE     │             │
│  │ apiClosedTrades │  │                 │  ├─────────────────┤             │
│  │ apiWeeklyVideo  │  │                 │  │ alertsError     │             │
│  └─────────────────┘  └─────────────────┘  │ tradesError     │             │
│                                            │ statsError      │             │
│  ┌─────────────────────────────────────┐   │ videosError     │             │
│  │          DERIVED STATE              │   └─────────────────┘             │
│  ├─────────────────────────────────────┤                                    │
│  │ alerts (API data → fallback)        │                                    │
│  │ filteredAlerts (filter applied)     │                                    │
│  │ tradePlan (formatted)               │                                    │
│  │ stats (API data → fallback)         │                                    │
│  │ weeklyPerformance (computed)        │                                    │
│  │ closedTrades (from apiClosedTrades) │                                    │
│  │ activePositions (from apiOpenTrades)│                                    │
│  │ weeklyContent (formatted video)     │                                    │
│  │ totalPages, showingFrom, showingTo  │                                    │
│  └─────────────────────────────────────┘                                    │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                              ACTIONS                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│  Data Fetching:          Modal Actions:          UI Actions:                │
│  ├── fetchAlerts()       ├── openAlertModal()    ├── setFilter()           │
│  ├── fetchTradePlan()    ├── closeAlertModal()   ├── goToPage()            │
│  ├── fetchStats()        ├── openTradeEntry...   ├── toggleNotes()         │
│  ├── fetchAllTrades()    ├── openClosePositio... ├── copyTradeDetails()    │
│  ├── fetchWeeklyVideo()  ├── openAddTrade...     └── initializeData()      │
│  └── checkAdminStatus()  ├── openUpdatePositi...                            │
│                          ├── openInvalidate...                              │
│                          └── deletePosition()                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

# 5. MEMBER USER JOURNEY

## 5.1 Dashboard Landing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MEMBER LANDS ON DASHBOARD                                 │
└─────────────────────────────────────────────────────────────────────────────┘

URL: /dashboard/explosive-swings

1. PAGE LOAD
   ├── onMount() → ps.initializeData()
   │   ├── checkAdminStatus()  → sets isAdmin flag
   │   ├── fetchAlerts()       → populates alerts feed
   │   ├── fetchTradePlan()    → populates trade plan table
   │   ├── fetchStats()        → populates performance metrics
   │   ├── fetchAllTrades()    → populates positions & closed trades
   │   └── fetchWeeklyVideo()  → populates hero video
   │
   └── If API fails → fallback data from data/fallbacks.ts

2. WHAT MEMBER SEES
   ┌─────────────────────────────────────────────────────────────┐
   │ TradingRoomHeader                                           │
   │   "Explosive Swings"    [Start Here →]                      │
   ├─────────────────────────────────────────────────────────────┤
   │ PerformanceSummary                                          │
   │   ┌──────────────────┐  ┌───────────────────────────────┐  │
   │   │ This Week's      │  │         82% Win Rate          │  │
   │   │ Performance      │  │         6/7 trades            │  │
   │   └──────────────────┘  └───────────────────────────────┘  │
   │                                                             │
   │   Closed This Week: [MSFT +8.2%] [AAPL +5.1%] [AMD -2.1%]  │
   │                                                             │
   │   Active Positions:                                         │
   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
   │   │ NVDA ENTRY  │ │ TSLA WATCH  │ │ META ACTIVE │          │
   │   │ +0.9%       │ │ —           │ │ +2.1%       │          │
   │   │ E:142.50    │ │ Zone:180-185│ │ E:585→597   │          │
   │   └─────────────┘ └─────────────┘ └─────────────┘          │
   ├─────────────────────────────────────────────────────────────┤
   │ WeeklyHero                                                  │
   │   [Video Breakdown] [Trade Plan & Entries]                  │
   │   ┌───────────────────┬─────────────────────────────────┐  │
   │   │   ▶ PLAY VIDEO    │  Week of January 13, 2026       │  │
   │   │   [24:35]         │  Weekly Swing Breakdown         │  │
   │   │                   │  [Watch Full Video →]           │  │
   │   └───────────────────┴─────────────────────────────────┘  │
   ├─────────────────────────────────────────────────────────────┤
   │ Main Grid                                                   │
   │ ┌─────────────────────────────┐ ┌────────────────────────┐ │
   │ │ Live Alerts                 │ │ Sidebar                │ │
   │ │ [All] [Entry] [Exit] [Update]│ │ 30-Day Performance    │ │
   │ │                              │ │ 82% Win Rate          │ │
   │ │ ┌─────────────────────────┐ │ │                        │ │
   │ │ │ ENTRY │ NVDA            │ │ │ Weekly Video          │ │
   │ │ │ Opening Swing Position  │ │ │ [thumbnail]           │ │
   │ │ │ Today at 10:32 AM       │ │ │                        │ │
   │ │ │ [📋 Copy] [▼ Notes]     │ │ └────────────────────────┘ │
   │ │ └─────────────────────────┘ │                           │
   │ │                              │                           │
   │ │ [View All Alerts →]         │                           │
   │ └─────────────────────────────┘                           │
   └─────────────────────────────────────────────────────────────┘

3. MEMBER ACTIONS
   ├── Watch weekly video → Click play button
   ├── View trade plan → Switch to "Trade Plan & Entries" tab
   ├── Expand alert notes → Click ▼ toggle
   ├── Copy trade details → Click 📋 button
   ├── Filter alerts → Click filter buttons
   ├── Navigate to alerts archive → Click "View All Alerts"
   ├── Navigate to start-here → Click header link
   └── View trade history → Navigate to /trades
```

## 5.2 Alerts Archive Page

```
URL: /dashboard/explosive-swings/alerts

┌─────────────────────────────────────────────────────────────────┐
│ Alerts Archive                                                   │
├─────────────────────────────────────────────────────────────────┤
│ Filters: [All Alerts] [Trade Alerts] [Market Updates]           │
│          🔍 Search alerts...                                     │
├─────────────────────────────────────────────────────────────────┤
│ Showing 8 alerts                                                 │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐    │
│ │ TRADE ALERT     │ │ MARKET UPDATE   │ │ TRADE ALERT     │    │
│ │ NVDA    [Open]  │ │        [Info]   │ │ TSLA   [Closed] │    │
│ │ NVDA Swing Setup│ │ Weekly Outlook  │ │ Position Closed │    │
│ │ Jan 10 2:30 PM  │ │ Jan 10 9:00 AM  │ │ Jan 9 3:45 PM   │    │
│ │                 │ │                 │ │ +$1,250         │    │
│ │ [View Details →]│ │ [View Details →]│ │ [View Details →]│    │
│ └─────────────────┘ └─────────────────┘ └─────────────────┘    │
└─────────────────────────────────────────────────────────────────┘

FEATURES:
├── SSR data loading via +page.server.ts
├── Client-side filtering (All/Trades/Updates)
├── Client-side search
├── Card-based grid layout
├── Status badges (Open/Closed/Info)
├── Profit/loss display for closed trades
└── Links to individual alert detail pages
```

---

# 6. ADMIN USER JOURNEY

## 6.1 Admin Capabilities Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          ADMIN CAPABILITIES                                  │
└─────────────────────────────────────────────────────────────────────────────┘

Admin detected via: /api/auth/me → is_admin=true OR role='admin'|'super_admin'

CONDITIONAL UI ELEMENTS:
├── PerformanceSummary
│   └── [+ Add Trade] button → AddTradeModal
│
├── ActivePositionCard (each card)
│   └── ⋮ Menu
│       ├── Update Position → UpdatePositionModal
│       ├── Invalidate → InvalidatePositionModal
│       ├── Close Trade → ClosePositionModal
│       └── Delete (with confirm dialog)
│
├── WeeklyHero
│   ├── Video tab: [Upload Video] button → VideoUploadModal
│   ├── Trade Plan tab: [+ Add Entry] button → TradeEntryModal
│   └── Each row: Edit button → TradeEntryModal (edit mode)
│
└── Alerts Section
    └── [+ New Alert] button → TradeAlertModal
```

## 6.2 Create New Alert Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ADMIN: CREATE NEW ALERT                               │
└─────────────────────────────────────────────────────────────────────────────┘

1. Admin clicks [+ New Alert]
   └── ps.openAlertModal() called

2. TradeAlertModal opens
   ├── Form fields:
   │   ├── Ticker (required)
   │   ├── Alert Type: ENTRY | UPDATE | EXIT
   │   ├── Title (required)
   │   ├── Message (required)
   │   ├── Notes (optional, expandable)
   │   ├── TOS Fields (optional):
   │   │   ├── Trade Type: Shares | Options
   │   │   ├── Action: BUY | SELL
   │   │   ├── Quantity
   │   │   ├── Option Type: CALL | PUT
   │   │   ├── Strike Price
   │   │   ├── Expiration
   │   │   └── Order Type: MKT | LMT
   │   └── Entry Alert Link (for UPDATE/EXIT types)
   │
   └── TOS String auto-generated from fields

3. Admin submits form
   └── POST /api/alerts/explosive-swings
       Body: { alert_type, ticker, title, message, notes, tos_string, ... }

4. Backend processes
   ├── Validates required fields
   ├── Inserts into room_alerts table
   ├── Sets is_new = true, is_published = true
   └── Returns { success: true, data: newAlert }

5. Frontend updates
   ├── handleSaveAlert() receives success
   ├── Calls ps.fetchAlerts() to refresh
   └── Closes modal

6. New alert appears in Live Alerts feed
```

## 6.3 Add Trade Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ADMIN: ADD NEW TRADE                                  │
└─────────────────────────────────────────────────────────────────────────────┘

1. Admin clicks [+ Add Trade] in PerformanceSummary
   └── ps.openAddTradeModal() called

2. AddTradeModal opens
   ├── Trade Setup:
   │   ├── Ticker Symbol (required)
   │   └── Trade Type: Shares | Options
   │
   ├── Position Details:
   │   ├── Direction: Long | Short
   │   └── Quantity (required)
   │
   ├── Options Details (if options selected):
   │   ├── Option Type: Call | Put
   │   ├── Strike Price
   │   ├── Expiration Date
   │   └── Contract Type: Weekly | Monthly | LEAP
   │
   ├── Entry & Risk:
   │   ├── Entry Price (required)
   │   └── Entry Date (required)
   │
   └── Additional Info:
       ├── Trade Setup: Breakout | Momentum | Reversal | Earnings | Pullback
       └── Notes

3. Admin submits form
   └── POST /api/trades/explosive-swings
       Body: { ticker, trade_type, direction, quantity, entry_price, entry_date, ... }

4. Backend processes
   ├── Validates required fields
   ├── Inserts into room_trades table with status='open'
   ├── Updates room_stats cache
   └── Returns { success: true, data: newTrade }

5. Frontend updates
   ├── onSuccess callback triggers
   ├── ps.fetchAllTrades() refreshes positions
   ├── ps.fetchStats() refreshes metrics
   └── Closes modal

6. New position appears in Active Positions section
```

## 6.4 Close Position Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ADMIN: CLOSE POSITION                                 │
└─────────────────────────────────────────────────────────────────────────────┘

1. Admin clicks ⋮ menu on position card
   └── Selects "Close Trade"

2. ClosePositionModal opens
   ├── Shows position summary:
   │   ├── Ticker
   │   ├── Entry Price
   │   ├── Current Price
   │   └── Unrealized P&L
   │
   ├── Form fields:
   │   ├── Exit Price (required)
   │   ├── Exit Date
   │   └── Exit Notes
   │
   └── Live P&L Preview:
       ├── WIN or LOSS badge
       ├── Percentage gain/loss
       └── Dollar difference per share

3. Admin enters exit price
   └── P&L preview updates in real-time

4. Admin submits form
   ├── First: GET /api/trades/explosive-swings?status=open&ticker=XXX
   │   └── Finds matching trade by ticker + entry price
   │
   └── Then: PUT /api/trades/explosive-swings/{trade_id}
       Body: { exit_price, exit_date, notes, status: 'closed' }

5. Backend processes
   ├── Updates trade record
   ├── Calculates pnl and pnl_percent
   ├── Sets status = 'closed'
   ├── Updates room_stats cache
   └── Returns { success: true }

6. Frontend updates
   ├── onSuccess triggers fetchAllTrades() + fetchStats()
   ├── Position moves from Active to Closed
   └── Appears as TickerPill with P&L percentage
```

## 6.5 Invalidate Position Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      ADMIN: INVALIDATE POSITION                              │
└─────────────────────────────────────────────────────────────────────────────┘

1. Admin clicks ⋮ menu → "Invalidate"
   └── ps.openInvalidatePositionModal(position)

2. InvalidatePositionModal opens
   ├── Info text explaining purpose
   ├── Preset reasons dropdown:
   │   ├── Price moved away before entry
   │   ├── Setup invalidated - pattern broken
   │   ├── Market conditions changed
   │   ├── Better opportunity elsewhere
   │   └── Other (specify in notes)
   └── Additional notes textarea

3. Admin selects reason and submits
   └── POST /api/admin/trades/{position.id}/invalidate
       Body: { reason: "Selected reason - additional notes" }

4. Backend processes
   ├── Sets trade status to 'invalidated'
   ├── Stores invalidation_reason
   └── Returns { success: true }

5. Frontend updates
   ├── fetchAllTrades() + fetchStats()
   └── Position removed from Active Positions

⚠️ GAP: Invalidated trades don't appear anywhere visible!
```

---

# 7. GAP ANALYSIS

## 7.1 Working Features ✅

| Feature                      | Route/Component             | API Endpoint                           | Status     |
| ---------------------------- | --------------------------- | -------------------------------------- | ---------- |
| View dashboard               | /dashboard/explosive-swings | Multiple                               | ✅ Working |
| View alerts                  | AlertCard + AlertsFeed      | /api/alerts/{slug}                     | ✅ Working |
| Filter alerts                | AlertFilters                | Client-side                            | ✅ Working |
| Paginate alerts              | Pagination                  | Query params                           | ✅ Working |
| Expand alert notes           | AlertCard                   | Client-side                            | ✅ Working |
| Copy trade details           | AlertCard                   | Clipboard API                          | ✅ Working |
| View trade plan              | WeeklyHero                  | /api/trade-plans/{slug}                | ✅ Working |
| View weekly video            | WeeklyHero                  | /api/weekly-video/{slug}               | ✅ Working |
| View performance stats       | PerformanceSummary          | /api/stats/{slug}                      | ✅ Working |
| View active positions        | ActivePositionCard          | /api/trades/{slug}                     | ✅ Working |
| View closed trades           | TickerPill                  | /api/trades/{slug}                     | ✅ Working |
| Create alert (admin)         | TradeAlertModal             | POST /api/alerts/{slug}                | ✅ Working |
| Edit alert (admin)           | TradeAlertModal             | PUT /api/alerts/{slug}/{id}            | ✅ Working |
| Delete alert (admin)         | AlertCard                   | DELETE /api/alerts/{slug}/{id}         | ✅ Working |
| Add trade (admin)            | AddTradeModal               | POST /api/trades/{slug}                | ✅ Working |
| Close position (admin)       | ClosePositionModal          | PUT /api/trades/{slug}/{id}            | ✅ Working |
| Update position (admin)      | UpdatePositionModal         | PUT /api/trades/{slug}/{id}            | ✅ Working |
| Invalidate position (admin)  | InvalidatePositionModal     | POST /api/admin/trades/{id}/invalidate | ✅ Working |
| Delete position (admin)      | Confirm dialog              | DELETE /api/admin/trades/{id}          | ✅ Working |
| Add trade plan entry (admin) | TradeEntryModal             | POST /api/trade-plans/{slug}           | ✅ Working |
| Upload video (admin)         | VideoUploadModal            | POST /api/weekly-video/{slug}          | ✅ Working |
| Alerts archive page          | /alerts/+page.svelte        | /api/alerts/{slug}                     | ✅ Working |

## 7.2 Partially Working ⚠️

| Feature               | Issue                                        | Missing Piece                                       |
| --------------------- | -------------------------------------------- | --------------------------------------------------- |
| Update position badge | "UPDATED" badge exists in ActivePositionCard | Need to verify `wasUpdated` is being set by backend |
| Video library         | Route exists at /video-library               | Need to verify content and functionality            |
| Watchlist page        | Route exists at /watchlist                   | Need to verify functionality                        |
| Favorites page        | Route exists at /favorites                   | Need to verify functionality                        |

## 7.3 Missing Features ❌

| Feature                           | Expected Behavior                                     | Current State                          |
| --------------------------------- | ----------------------------------------------------- | -------------------------------------- |
| **Invalidated trades display**    | Show invalidated trades with reason somewhere visible | Trades disappear - nowhere to see them |
| **Weekly archive page**           | Browse past weeks with videos + alerts grouped        | No dedicated archive page              |
| **Alert grouping by week**        | Group alerts by week in archive                       | All alerts in flat list                |
| **Video ↔ Alert linking**         | Link weekly video to associated alerts                | No linking mechanism                   |
| **Auto weekly rollover**          | Automated weekly content rotation                     | Manual admin process only              |
| **Trade ↔ Alert linking**         | Connect ENTRY/EXIT alerts to trades                   | Fields exist but not utilized in UI    |
| **Individual alert detail pages** | /alerts/[id] route                                    | Links exist but pages may not          |

## 7.4 Technical Debt

| Issue                 | File                      | Impact                               |
| --------------------- | ------------------------- | ------------------------------------ |
| Menu dropdown z-index | ActivePositionCard.svelte | z-index: 50 (should be higher)       |
| Fallback data showing | All API failures          | Users see mock data instead of error |
| Missing error states  | Some modals               | Poor UX when API fails               |
| Type inconsistencies  | types.ts vs API           | Some fields not matching backend     |

---

# 8. FIX PRIORITY

## P0 - Critical (Blocking Admin Workflow)

1. **Invalidated trades visibility**
   - Add "Invalidated" section or badge to show invalidated trades
   - Show invalidation reason
   - Consider adding to trades archive page

## P1 - High (Important for Complete Workflow)

2. **Verify UpdatePositionModal sets wasUpdated flag**
   - Backend must set `was_updated = true` on trade record
   - Frontend already checks for this in ActivePositionCard

3. **Add individual alert detail pages**
   - Create /alerts/[id]/+page.svelte
   - Display full alert with all TOS fields

4. **Menu z-index fix**
   - Change ActivePositionCard menu dropdown z-index from 50 to 100+
   - Ensure menu doesn't get hidden behind other elements

## P2 - Medium (Enhanced User Experience)

5. **Weekly archive functionality**
   - Create /archive or /weekly-archive route
   - Group content by week_of date
   - Link video to associated alerts

6. **Trade ↔ Alert linking in UI**
   - Display linked alerts on trade cards
   - Show trade status from linked alert

7. **Improve error handling**
   - Show proper error messages instead of fallback data
   - Add retry buttons on API failures

## P3 - Low (Nice to Have)

8. **Auto weekly rollover**
   - Cron job or admin "Start New Week" button
   - Archive previous week automatically

9. **Video library verification**
   - Verify /video-library route works
   - Ensure proper video browsing

10. **Enhance alerts archive**
    - Add week grouping
    - Add date range filter
    - Add ticker filter

---

# APPENDIX A: TYPE DEFINITIONS

## Key Types from types.ts

```typescript
// Alert Types
export type AlertType = 'ENTRY' | 'UPDATE' | 'EXIT';
export type AlertFilter = 'all' | 'entry' | 'exit' | 'update';

// Position Types
export type PositionStatus = 'ENTRY' | 'WATCHING' | 'ACTIVE';
export type TradeBias = 'BULLISH' | 'BEARISH' | 'NEUTRAL';

// API Response Types
export interface ApiTrade {
	id: number;
	ticker: string;
	status: 'open' | 'closed';
	entry_price: number;
	exit_price: number | null;
	pnl_percent: number | null;
	entry_date: string;
	exit_date: string | null;
	direction: string;
	setup?: string;
	notes?: string;
}

export interface ApiWeeklyVideo {
	id: number;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
	week_title: string;
}

export interface QuickStats {
	winRate: number;
	weeklyProfit: string;
	activeTrades: number;
	closedThisWeek: number;
}
```

---

# APPENDIX B: ROOM CONSTANTS

```typescript
// From constants.ts
export const ROOM_SLUG = 'explosive-swings';
export const ROOM_CONTENT_ID = 4; // Backend room_content.rooms table
export const ROOM_RESOURCES_ID = 2; // Backend room_resources.rooms table
export const ALERTS_PER_PAGE = 10;
export const TRADES_PER_PAGE = 50;
export const ROOM_NAME = 'Explosive Swings';
export const ROOM_DESCRIPTION = 'Swing trading opportunities with explosive profit potential';
```

---

# APPENDIX C: BACKEND DATABASE TABLES

Based on room_content.rs Rust models:

```sql
-- Trade Plan Entries
CREATE TABLE room_trade_plans (
  id BIGSERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL,
  room_slug TEXT NOT NULL,
  week_of DATE NOT NULL,
  ticker TEXT NOT NULL,
  bias TEXT NOT NULL,
  entry TEXT,
  target1 TEXT, target2 TEXT, target3 TEXT,
  runner TEXT, runner_stop TEXT, stop TEXT,
  options_strike TEXT, options_exp DATE,
  notes TEXT,
  sort_order INT DEFAULT 0,
  is_active BOOLEAN DEFAULT TRUE,
  created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ
);

-- Alerts
CREATE TABLE room_alerts (
  id BIGSERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL,
  room_slug TEXT NOT NULL,
  alert_type TEXT NOT NULL,
  ticker TEXT NOT NULL,
  title TEXT, message TEXT NOT NULL, notes TEXT,
  -- TOS Fields
  trade_type TEXT, action TEXT, quantity INT,
  option_type TEXT, strike DECIMAL, expiration DATE,
  contract_type TEXT, order_type TEXT,
  limit_price DECIMAL, fill_price DECIMAL,
  tos_string TEXT,
  -- Linking
  entry_alert_id BIGINT, trade_plan_id BIGINT,
  -- Status
  is_new BOOLEAN DEFAULT TRUE,
  is_published BOOLEAN DEFAULT TRUE,
  is_pinned BOOLEAN DEFAULT FALSE,
  published_at TIMESTAMPTZ, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ
);

-- Trades
CREATE TABLE room_trades (
  id BIGSERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL,
  room_slug TEXT NOT NULL,
  ticker TEXT NOT NULL,
  trade_type TEXT NOT NULL,
  direction TEXT NOT NULL,
  quantity INT NOT NULL,
  option_type TEXT, strike DECIMAL, expiration DATE, contract_type TEXT,
  entry_alert_id BIGINT, entry_price DECIMAL NOT NULL, entry_date DATE NOT NULL,
  entry_tos_string TEXT,
  exit_alert_id BIGINT, exit_price DECIMAL, exit_date DATE, exit_tos_string TEXT,
  setup TEXT, status TEXT DEFAULT 'open',
  result TEXT, pnl DECIMAL, pnl_percent DECIMAL, holding_days INT,
  notes TEXT, was_updated BOOLEAN, invalidation_reason TEXT,
  created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ
);

-- Weekly Videos
CREATE TABLE room_weekly_videos (
  id BIGSERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL,
  room_slug TEXT NOT NULL,
  week_of DATE NOT NULL,
  week_title TEXT NOT NULL,
  video_title TEXT NOT NULL,
  video_url TEXT NOT NULL,
  video_platform TEXT,
  thumbnail_url TEXT, duration TEXT, description TEXT,
  is_current BOOLEAN DEFAULT TRUE,
  is_published BOOLEAN DEFAULT TRUE,
  published_at TIMESTAMPTZ, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ
);

-- Stats Cache
CREATE TABLE room_stats (
  id BIGSERIAL PRIMARY KEY,
  room_id BIGINT NOT NULL,
  room_slug TEXT NOT NULL,
  win_rate DECIMAL, weekly_profit TEXT, monthly_profit TEXT,
  active_trades INT, closed_this_week INT, total_trades INT,
  wins INT, losses INT, avg_win DECIMAL, avg_loss DECIMAL,
  profit_factor DECIMAL, avg_holding_days DECIMAL,
  largest_win DECIMAL, largest_loss DECIMAL, current_streak INT,
  calculated_at TIMESTAMPTZ
);
```

---

**Document Generated:** January 26, 2026
**Version:** 1.0.0
**Author:** Cascade AI (Apple Principal Engineer ICT 7+ Grade)
