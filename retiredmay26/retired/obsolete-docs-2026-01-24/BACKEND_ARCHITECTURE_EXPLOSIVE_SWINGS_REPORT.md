# Backend Architecture Report: Explosive Swings & Feature Integration

**Date:** January 20, 2026
**Analysis Scope:** Complete backend architecture review for video lessons, video watchlist, market updates, trades, alerts, video library, trade tracker, and their integration with Explosive Swings.

---

## Executive Summary

Revolution Trading Pros uses a **dual-stack architecture**:
- **Frontend:** SvelteKit 2.x with Svelte 5, TypeScript, Tailwind CSS
- **Backend:** Rust Axum web framework deployed on Fly.io
- **Database:** PostgreSQL (managed)
- **Caching:** Redis (Upstash)
- **Video Delivery:** Bunny.net CDN
- **File Storage:** Cloudflare R2 (S3-compatible)

All features for Explosive Swings connect through a unified room-based architecture where `room_id=4` and `room_slug='explosive-swings'` serve as the primary identifiers.

---

## Table of Contents

1. [Overall Architecture](#1-overall-architecture)
2. [Trading Rooms System](#2-trading-rooms-system)
3. [Video Lessons & Video Library](#3-video-lessons--video-library)
4. [Video Watchlist](#4-video-watchlist)
5. [Market Updates (Daily Videos)](#5-market-updates-daily-videos)
6. [Alerts System](#6-alerts-system)
7. [Trade Tracker](#7-trade-tracker)
8. [Room Resources](#8-room-resources)
9. [Performance Stats](#9-performance-stats)
10. [Explosive Swings Integration Map](#10-explosive-swings-integration-map)
11. [Database Schema Summary](#11-database-schema-summary)
12. [API Endpoints Reference](#12-api-endpoints-reference)
13. [Key Files Reference](#13-key-files-reference)

---

## 1. Overall Architecture

### Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Frontend Framework | SvelteKit 2.x + Svelte 5 | Server-side rendering, routing |
| Styling | Tailwind CSS v4 | Utility-first CSS |
| Backend Framework | Axum 0.7 (Rust) | High-performance API server |
| Database | PostgreSQL | Primary data store |
| Cache | Redis (Upstash) | Session cache, rate limiting |
| Video CDN | Bunny.net | Video streaming (114 global edges) |
| File Storage | Cloudflare R2 | PDFs, images, documents |
| Payments | Stripe | Subscription management |
| Email | Postmark | Transactional email |
| Search | Meilisearch | Full-text search |

### Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         User Browser                                 │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│              SvelteKit Frontend (SSR + Client)                       │
│  /frontend/src/routes/                                               │
│  ├── dashboard/explosive-swings/   ← Explosive Swings pages         │
│  ├── api/alerts/[slug]/            ← API proxy routes                │
│  └── api/trades/[slug]/            ← API proxy routes                │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Rust Axum Backend API                             │
│  /api/src/routes/                                                    │
│  ├── room_content.rs        ← Alerts, Trades, Trade Plans            │
│  ├── watchlist.rs           ← Weekly Watchlist                       │
│  ├── videos.rs              ← Video Library                          │
│  └── room_resources.rs      ← PDFs, Documents                        │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
          ┌─────────────────────┼─────────────────────┐
          ▼                     ▼                     ▼
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│   PostgreSQL    │   │   Bunny.net     │   │  Cloudflare R2  │
│   Database      │   │   Video CDN     │   │  File Storage   │
└─────────────────┘   └─────────────────┘   └─────────────────┘
```

---

## 2. Trading Rooms System

### Room Configuration

The platform operates 6 trading rooms, stored in `trading_rooms` table:

| ID | Slug | Name | Type |
|----|------|------|------|
| 1 | `day-trading-room` | Day Trading Room | room |
| 2 | `swing-trading-room` | Swing Trading Room | room |
| 3 | `small-account-mentorship` | Small Account Mentorship | mentorship |
| **4** | **`explosive-swings`** | **Explosive Swings** | **alert_service** |
| 5 | `spx-profit-pulse` | SPX Profit Pulse | alert_service |
| 6 | `high-octane-scanner` | High Octane Scanner | alert_service |

### Explosive Swings Configuration

```typescript
// /frontend/src/lib/config/rooms.ts
{
  id: 'explosive-swings',
  slug: 'explosive-swings',
  name: 'Explosive Swings',
  shortName: 'ES',
  type: 'alerts-only',
  membershipId: 4,
  color: '#ef4444',
  icon: '💥',
  features: {
    tos_alerts: true,      // TOS-formatted alerts
    trade_plan: true,      // Weekly trade plans
    weekly_video: true,    // Weekly video breakdowns
    trade_tracker: true    // P&L tracking
  }
}
```

---

## 3. Video Lessons & Video Library

### Database Schema

**Table:** `unified_videos`
```sql
CREATE TABLE unified_videos (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) UNIQUE NOT NULL,
    description TEXT,
    video_url VARCHAR(500),
    video_platform VARCHAR(50) DEFAULT 'bunny',  -- bunny|vimeo|youtube|wistia
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    thumbnail_url VARCHAR(500),
    duration INTEGER,                            -- seconds
    views_count INTEGER DEFAULT 0,
    content_type VARCHAR(50),                    -- daily_video|weekly_watchlist|learning_center|room_archive
    room_id BIGINT,
    room_slug VARCHAR(100),
    is_featured BOOLEAN DEFAULT false,
    is_published BOOLEAN DEFAULT true,
    published_at TIMESTAMPTZ,
    tags JSONB DEFAULT '[]',
    difficulty_level VARCHAR(20),                -- beginner|intermediate|advanced
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Video Content Types

| Type | Purpose | Usage |
|------|---------|-------|
| `daily_video` | Daily market analysis | Market updates section |
| `weekly_watchlist` | Weekly trade ideas | Watchlist page |
| `learning_center` | Educational content | Learning section |
| `room_archive` | Historical recordings | Video library |

### Bunny.net Integration

**File:** `/api/src/services/bunny.rs`

```rust
// Embed URL generation
pub fn get_embed_url(library_id: i64, guid: &str) -> String {
    format!("https://iframe.mediadelivery.net/embed/{}/{}", library_id, guid)
}

// Thumbnail URL
pub fn get_thumbnail_url(library_id: i64, guid: &str) -> String {
    format!("https://vz-{}.b-cdn.net/{}/thumbnail.jpg", library_id, guid)
}
```

### API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/videos` | List published videos |
| GET | `/api/videos/:id_or_slug` | Get single video |
| POST | `/api/videos/:id/track` | Track video events |
| POST | `/api/admin/videos` | Create video |
| PUT | `/api/admin/videos/:id` | Update video |
| DELETE | `/api/admin/videos/:id` | Soft delete |

### Explosive Swings Video Library

**Page:** `/dashboard/explosive-swings/video-library`

```typescript
// Server loader fetches room-specific videos
const videos = await fetch(`${BACKEND_URL}/trading-rooms/explosive-swings/videos`);
```

---

## 4. Video Watchlist

### Database Schema

**Table:** `watchlist_entries`
```sql
CREATE TABLE watchlist_entries (
    id BIGSERIAL PRIMARY KEY,
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    trader VARCHAR(255) NOT NULL,
    trader_image VARCHAR(500),
    week_of DATE NOT NULL,
    video_url VARCHAR(500) NOT NULL,
    video_platform VARCHAR(50) DEFAULT 'bunny',
    spreadsheet_url VARCHAR(500),
    description TEXT,
    status VARCHAR(20) DEFAULT 'published',      -- published|draft|archived
    rooms JSONB DEFAULT '[]',                    -- Multi-room targeting
    tags JSONB DEFAULT '[]',
    difficulty_level VARCHAR(20),
    views_count INTEGER DEFAULT 0,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Multi-Room Targeting

Watchlist entries can target multiple rooms via JSONB `rooms` field:
```json
["explosive-swings", "swing-trading-room"]
```

### API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/watchlist` | List all watchlist entries |
| GET | `/api/watchlist/:slug` | Get single entry |
| POST | `/api/watchlist` | Create entry |
| PUT | `/api/watchlist/:slug` | Update entry |
| DELETE | `/api/watchlist/:slug` | Delete entry |

### Watchlist Query Parameters

```
?room=explosive-swings    # Filter by room
&status=published         # Filter by status
&search=NVDA              # Search by ticker/title
&page=1&per_page=10       # Pagination
```

---

## 5. Market Updates (Daily Videos)

### Implementation

Daily videos are stored in `unified_videos` with `content_type = 'daily_video'`.

### Room-Specific Daily Videos

**Server Loader Pattern:**
```typescript
// /dashboard/explosive-swings/daily-videos/+page.server.ts
export const load: PageServerLoad = async ({ fetch, cookies }) => {
    const response = await fetch(`${BACKEND_URL}/trading-rooms/explosive-swings/videos?content_type=daily_video`);
    return { videos: await response.json() };
};
```

### Scheduled Publishing

```sql
-- Scheduled content support
CREATE TABLE scheduled_content (
    id BIGSERIAL PRIMARY KEY,
    content_type VARCHAR(50) NOT NULL,
    content_id BIGINT NOT NULL,
    scheduled_action VARCHAR(50) NOT NULL,  -- publish|unpublish
    scheduled_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) DEFAULT 'scheduled'
);
```

### Explosive Swings Daily Videos

**Page:** `/dashboard/explosive-swings/daily-videos`
- Displays room-specific daily market analysis
- Videos filtered by `room_slug = 'explosive-swings'`
- Supports search, pagination, and date filtering

---

## 6. Alerts System

### Database Schema

**Table:** `room_alerts`
```sql
CREATE TABLE room_alerts (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    alert_type VARCHAR(20) NOT NULL,         -- ENTRY|EXIT|UPDATE
    ticker VARCHAR(10) NOT NULL,
    title VARCHAR(500),
    message TEXT NOT NULL,
    notes TEXT,

    -- TOS (ThinkOrSwim) Format Fields
    trade_type VARCHAR(20),                   -- options|shares
    action VARCHAR(10),                       -- BUY|SELL
    quantity INTEGER,
    option_type VARCHAR(10),                  -- CALL|PUT
    strike DECIMAL(10,2),
    expiration DATE,
    contract_type VARCHAR(20),                -- Weeklys|Monthly|LEAPS
    order_type VARCHAR(10),                   -- MKT|LMT
    limit_price DECIMAL(10,2),
    fill_price DECIMAL(10,2),
    tos_string TEXT,                          -- Full TOS command string

    -- Linking
    entry_alert_id BIGINT REFERENCES room_alerts(id),
    trade_plan_id BIGINT,

    -- Status
    is_new BOOLEAN DEFAULT true,
    is_published BOOLEAN DEFAULT true,
    is_pinned BOOLEAN DEFAULT false,
    published_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
```

### Alert Types

| Type | Description | Example |
|------|-------------|---------|
| `ENTRY` | New trade entry signal | "BUY +150 NVDA @142.50 LMT" |
| `UPDATE` | Trade status update | "NVDA approaching target 1" |
| `EXIT` | Exit signal | "SELL -150 NVDA @148.75 LMT" |

### TOS String Format

ThinkOrSwim-compatible command strings:
```
// Shares
"BUY +150 NVDA @142.50 LMT"
"SELL -200 AMD @125.00 LMT"

// Options
"BUY +2 META 100 (Weeklys) 24 JAN 26 590 CALL @12.50 LMT"
```

### API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/alerts/explosive-swings` | List alerts |
| POST | `/api/alerts/explosive-swings` | Create alert |
| GET | `/api/alerts/explosive-swings/:id` | Get single alert |
| PUT | `/api/alerts/explosive-swings/:id` | Update alert |
| DELETE | `/api/alerts/explosive-swings/:id` | Delete alert |

---

## 7. Trade Tracker

### Database Schema

**Table:** `room_trades`
```sql
CREATE TABLE room_trades (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    ticker VARCHAR(10) NOT NULL,
    trade_type VARCHAR(20) NOT NULL DEFAULT 'shares',
    direction VARCHAR(10) NOT NULL DEFAULT 'long',
    quantity INTEGER NOT NULL,

    -- Options specific
    option_type VARCHAR(10),
    strike DECIMAL(10,2),
    expiration DATE,
    contract_type VARCHAR(20),

    -- Entry
    entry_alert_id BIGINT REFERENCES room_alerts(id),
    entry_price DECIMAL(10,2) NOT NULL,
    entry_date DATE NOT NULL,
    entry_tos_string TEXT,

    -- Exit (null if open)
    exit_alert_id BIGINT REFERENCES room_alerts(id),
    exit_price DECIMAL(10,2),
    exit_date DATE,
    exit_tos_string TEXT,

    -- Analysis
    setup VARCHAR(50),                        -- Breakout|Momentum|Reversal|Earnings|Pullback
    status VARCHAR(20) DEFAULT 'open',        -- open|closed|partial
    result VARCHAR(10),                       -- WIN|LOSS|null

    -- Performance
    pnl DECIMAL(12,2),
    pnl_percent DECIMAL(6,2),
    holding_days INTEGER,
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
```

### P&L Calculation Logic

```rust
// Long trade P&L
pnl = (exit_price - entry_price) * quantity

// Short trade P&L
pnl = (entry_price - exit_price) * quantity

// Percentage
pnl_percent = ((exit_price - entry_price) / entry_price) * 100

// Holding period
holding_days = (exit_date - entry_date).num_days()

// Result
result = if pnl > 0 { "WIN" } else { "LOSS" }
```

### Alert-to-Trade Flow

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Trade Plan    │────▶│   Entry Alert   │────▶│     Trade       │
│  (room_trade_   │     │  (room_alerts)  │     │  (room_trades)  │
│    plans)       │     │  type='ENTRY'   │     │  status='open'  │
└─────────────────┘     └─────────────────┘     └────────┬────────┘
                                                         │
                        ┌─────────────────┐              │
                        │   Exit Alert    │◀─────────────┘
                        │  (room_alerts)  │
                        │  type='EXIT'    │
                        └────────┬────────┘
                                 │
                        ┌────────▼────────┐
                        │  Trade Closed   │
                        │  P&L Calculated │
                        │  Stats Updated  │
                        └─────────────────┘
```

### API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/trades/explosive-swings` | List trades |
| POST | `/api/trades/explosive-swings` | Create trade |
| PUT | `/api/trades/explosive-swings/:id` | Update/close trade |
| DELETE | `/api/trades/explosive-swings/:id` | Delete trade |

---

## 8. Room Resources

### Database Schema

**Table:** `room_resources`
```sql
CREATE TABLE room_resources (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) NOT NULL,
    description TEXT,
    trading_room_id BIGINT NOT NULL,
    resource_type VARCHAR(50) NOT NULL,       -- video|pdf|document|image|spreadsheet|archive
    content_type VARCHAR(50) NOT NULL,        -- tutorial|daily_video|guide|chart|etc
    section VARCHAR(50),                      -- introduction|latest_updates|learning_center|etc
    file_url VARCHAR(500),
    embed_url VARCHAR(500),
    mime_type VARCHAR(100),
    file_size BIGINT,

    -- Video fields
    video_platform VARCHAR(50),
    bunny_video_guid VARCHAR(100),
    bunny_library_id VARCHAR(50),
    duration INTEGER,
    thumbnail_url VARCHAR(500),

    -- Status
    is_published BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    is_pinned BOOLEAN DEFAULT false,
    views_count INTEGER DEFAULT 0,
    downloads_count INTEGER DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
```

### Resource Types

| Type | Description | Storage |
|------|-------------|---------|
| `video` | Video content | Bunny.net |
| `pdf` | PDF documents | Cloudflare R2 |
| `document` | Word/text docs | Cloudflare R2 |
| `image` | Charts, screenshots | Cloudflare R2 |
| `spreadsheet` | Excel/CSV files | Cloudflare R2 |
| `archive` | ZIP files | Cloudflare R2 |

### API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/room-resources` | List resources |
| GET | `/api/room-resources/:id` | Get single resource |
| POST | `/api/room-resources/:id/download` | Track download |
| POST | `/api/admin/room-resources` | Create resource |
| PUT | `/api/admin/room-resources/:id` | Update resource |
| DELETE | `/api/admin/room-resources/:id` | Delete resource |

---

## 9. Performance Stats

### Database Schema

**Table:** `room_stats_cache`
```sql
CREATE TABLE room_stats_cache (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT,
    room_slug VARCHAR(100) UNIQUE NOT NULL,

    -- Win/Loss
    win_rate DECIMAL(5,2),
    total_trades INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,

    -- Profitability
    weekly_profit VARCHAR(50),                -- '+$4,850'
    monthly_profit VARCHAR(50),               -- '+$18,750'
    avg_win DECIMAL(12,2),
    avg_loss DECIMAL(12,2),
    profit_factor DECIMAL(6,2),               -- (wins*avg_win)/(losses*avg_loss)

    -- Activity
    active_trades INTEGER DEFAULT 0,
    closed_this_week INTEGER DEFAULT 0,

    -- Duration
    avg_holding_days DECIMAL(6,2),

    -- Extremes
    largest_win DECIMAL(12,2),
    largest_loss DECIMAL(12,2),

    -- Streak
    current_streak INTEGER DEFAULT 0,

    -- Chart data
    daily_pnl_30d JSONB DEFAULT '[]',

    calculated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Auto-Recalculation Trigger

```sql
-- Trigger on room_trades changes
CREATE OR REPLACE FUNCTION calculate_room_stats(p_room_slug VARCHAR)
RETURNS void AS $$
BEGIN
    -- Calculate all metrics from room_trades
    -- Update room_stats_cache
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_recalculate_stats
AFTER INSERT OR UPDATE OR DELETE ON room_trades
FOR EACH ROW EXECUTE FUNCTION calculate_room_stats(NEW.room_slug);
```

### Explosive Swings Stats (Seeded Data)

```sql
INSERT INTO room_stats_cache (room_slug, win_rate, weekly_profit, monthly_profit, ...)
VALUES (
    'explosive-swings',
    82.00,                    -- 82% win rate
    '+$4,850',                -- Weekly profit
    '+$18,750',               -- Monthly profit
    4,                        -- Active trades
    2,                        -- Closed this week
    28,                       -- Total trades
    23,                       -- Wins
    5,                        -- Losses
    2.78                      -- Profit factor
);
```

---

## 10. Explosive Swings Integration Map

### Complete Feature Integration

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         EXPLOSIVE SWINGS (room_id=4)                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐ │
│  │ Trade Plans  │──▶│   Alerts     │──▶│   Trades     │──▶│    Stats     │ │
│  │(room_trade_  │   │(room_alerts) │   │(room_trades) │   │(room_stats_  │ │
│  │  plans)      │   │              │   │              │   │   cache)     │ │
│  └──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘ │
│         │                  │                  │                  │          │
│         └──────────────────┴──────────────────┴──────────────────┘          │
│                                    │                                         │
│                         ┌──────────┴──────────┐                             │
│                         ▼                      ▼                             │
│                  ┌──────────────┐       ┌──────────────┐                    │
│                  │   Videos     │       │  Resources   │                    │
│                  │(unified_     │       │(room_        │                    │
│                  │  videos)     │       │  resources)  │                    │
│                  └──────────────┘       └──────────────┘                    │
│                         │                      │                             │
│                         └──────────┬───────────┘                             │
│                                    ▼                                         │
│                            ┌──────────────┐                                 │
│                            │  Watchlist   │                                 │
│                            │(watchlist_   │                                 │
│                            │  entries)    │                                 │
│                            └──────────────┘                                 │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Frontend Routes

| Route | Page | Purpose |
|-------|------|---------|
| `/dashboard/explosive-swings` | Main Dashboard | Overview with stats, alerts, trade plans |
| `/dashboard/explosive-swings/alerts` | Alerts | Full alerts listing |
| `/dashboard/explosive-swings/trade-tracker` | Trade Tracker | P&L tracking |
| `/dashboard/explosive-swings/video-library` | Video Library | All room videos |
| `/dashboard/explosive-swings/watchlist` | Watchlist | Weekly trade ideas |
| `/dashboard/explosive-swings/favorites` | Favorites | Saved items |
| `/dashboard/explosive-swings/start-here` | Start Here | Getting started guide |
| `/dashboard/explosive-swings/video/[slug]` | Video Player | Individual video |

### Dashboard Data Loading

**File:** `/frontend/src/routes/dashboard/explosive-swings/+page.server.ts`

```typescript
export const load: PageServerLoad = async ({ fetch, cookies }) => {
    const [stats, alerts, tradePlans, weeklyVideo, resources] = await Promise.all([
        fetch(`/api/stats/explosive-swings`),
        fetch(`/api/alerts/explosive-swings?limit=5`),
        fetch(`/api/trade-plans/explosive-swings`),
        fetch(`/api/weekly-video/explosive-swings`),
        fetch(`/api/room-resources?room_id=4&is_featured=true`)
    ]);

    return { stats, alerts, tradePlans, weeklyVideo, resources };
};
```

---

## 11. Database Schema Summary

### Core Tables

| Table | Purpose | Key Fields |
|-------|---------|------------|
| `trading_rooms` | Room definitions | id, slug, name, type, features |
| `unified_videos` | All video content | id, slug, content_type, room_slug, bunny_guid |
| `watchlist_entries` | Weekly watchlist | id, slug, week_of, rooms, video_url |
| `room_alerts` | Trade alerts | id, room_slug, alert_type, ticker, tos_string |
| `room_trade_plans` | Weekly trade plans | id, room_slug, week_of, ticker, entry, targets |
| `room_trades` | Trade execution | id, room_slug, ticker, pnl, status, result |
| `room_stats_cache` | Performance metrics | room_slug, win_rate, profit_factor |
| `room_resources` | PDFs, docs, etc | id, trading_room_id, resource_type |

### Relationships

```
trading_rooms (1) ──────┬───────▶ (N) unified_videos
                        ├───────▶ (N) room_alerts
                        ├───────▶ (N) room_trade_plans
                        ├───────▶ (N) room_trades
                        ├───────▶ (1) room_stats_cache
                        └───────▶ (N) room_resources

room_alerts (1) ────────┬───────▶ (1) room_alerts (entry_alert_id)
                        └───────▶ (N) room_trades (entry_alert_id, exit_alert_id)

room_trade_plans (1) ───────────▶ (N) room_alerts (trade_plan_id)
```

---

## 12. API Endpoints Reference

### Public Endpoints

```
# Videos
GET  /api/videos
GET  /api/videos/:id_or_slug
POST /api/videos/:id/track

# Alerts (room-specific)
GET  /api/alerts/:room_slug
GET  /api/alerts/:room_slug/:id

# Trades (room-specific)
GET  /api/trades/:room_slug
GET  /api/trades/:room_slug/:id

# Trade Plans
GET  /api/trade-plans/:room_slug

# Stats
GET  /api/stats/:room_slug

# Weekly Videos
GET  /api/weekly-video/:room_slug

# Watchlist
GET  /api/watchlist
GET  /api/watchlist/:slug

# Resources
GET  /api/room-resources
GET  /api/room-resources/:id
POST /api/room-resources/:id/download
```

### Admin Endpoints

```
# Videos
POST   /api/admin/videos
PUT    /api/admin/videos/:id
DELETE /api/admin/videos/:id
POST   /api/admin/videos/bulk/publish
POST   /api/admin/videos/bulk/delete
POST   /api/admin/videos/bulk/assign

# Alerts
POST   /api/admin/room-content/alerts
PUT    /api/admin/room-content/alerts/:id
DELETE /api/admin/room-content/alerts/:id

# Trade Plans
POST   /api/admin/room-content/trade-plans
PUT    /api/admin/room-content/trade-plans/:id

# Trades
POST   /api/admin/room-content/trades
PUT    /api/admin/room-content/trades/:id
DELETE /api/admin/room-content/trades/:id

# Watchlist
POST   /api/watchlist
PUT    /api/watchlist/:slug
DELETE /api/watchlist/:slug

# Resources
POST   /api/admin/room-resources
PUT    /api/admin/room-resources/:id
DELETE /api/admin/room-resources/:id
```

---

## 13. Key Files Reference

### Backend (Rust)

| File | Purpose |
|------|---------|
| `/api/src/main.rs` | Application entry, router setup |
| `/api/src/routes/room_content.rs` | Alerts, trades, trade plans |
| `/api/src/routes/videos.rs` | Public video routes |
| `/api/src/routes/admin_videos.rs` | Admin video CRUD |
| `/api/src/routes/watchlist.rs` | Watchlist CRUD |
| `/api/src/routes/room_resources.rs` | Resource management |
| `/api/src/services/bunny.rs` | Bunny.net integration |
| `/api/src/services/storage.rs` | R2 storage integration |

### Database Migrations

| File | Purpose |
|------|---------|
| `/api/migrations/015_consolidated_schema.sql` | Core schema (videos, rooms, resources) |
| `/api/migrations/016_user_favorites_and_stats.sql` | Stats cache, favorites |
| `/api/migrations/018_explosive_swings_complete.sql` | Alerts, trades, TOS support |

### Frontend (SvelteKit)

| File | Purpose |
|------|---------|
| `/frontend/src/routes/dashboard/explosive-swings/+page.svelte` | Main dashboard |
| `/frontend/src/routes/dashboard/explosive-swings/+page.server.ts` | Dashboard data loader |
| `/frontend/src/routes/dashboard/explosive-swings/alerts/` | Alerts page |
| `/frontend/src/routes/dashboard/explosive-swings/trade-tracker/` | Trade tracker |
| `/frontend/src/routes/dashboard/explosive-swings/video-library/` | Video library |
| `/frontend/src/routes/dashboard/explosive-swings/watchlist/` | Watchlist |
| `/frontend/src/lib/api/room-resources.ts` | Resource API client |
| `/frontend/src/lib/api/video.ts` | Video API client |
| `/frontend/src/lib/types/trading.ts` | Trading type definitions |
| `/frontend/src/lib/config/rooms.ts` | Room configuration |

### Configuration

| File | Purpose |
|------|---------|
| `/api/.env.example` | Backend environment template |
| `/frontend/src/lib/config/trading-rooms.ts` | Room features config |
| `/frontend/src/lib/data/products.ts` | Pricing/products |

---

## Summary

The Explosive Swings feature is fully integrated into the Revolution Trading Pros platform through:

1. **Unified Room Architecture** - All features filter by `room_slug='explosive-swings'`
2. **TOS-Compatible Alerts** - Full ThinkOrSwim command string support
3. **Automated P&L Tracking** - Trades linked to entry/exit alerts with auto-calculation
4. **Real-time Stats** - Database triggers auto-update performance metrics
5. **Multi-Platform Videos** - Bunny.net primary, with Vimeo/YouTube fallback
6. **Resource Management** - PDFs, documents, spreadsheets via Cloudflare R2
7. **Watchlist Integration** - Weekly trade ideas with multi-room targeting

The architecture supports:
- High-performance video delivery (Bunny.net 114 edge locations)
- Scalable file storage (Cloudflare R2)
- Real-time analytics tracking
- Soft-delete data preservation
- Role-based access control
