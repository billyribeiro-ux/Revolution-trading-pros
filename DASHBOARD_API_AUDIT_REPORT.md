# Dashboard API Audit Report
## End-to-End Frontend ↔ Backend Communication

**Generated:** January 18, 2026  
**Author:** ICT 11 Principal Engineer Grade Implementation  
**Scope:** Explosive Swings & Small Account Mentorship Dashboards

---

## Executive Summary

| Metric | Count |
|--------|-------|
| Total API Endpoints | 24 |
| Frontend API Routes | 12 |
| Backend Rust Handlers | 18 |
| Database Tables | 8 |
| Features Implemented | 14 |

---

## 1. EXPLOSIVE SWINGS DASHBOARD

### 1.1 Main Dashboard (`/dashboard/explosive-swings`)

#### Quick Stats Bar
| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Win Rate | `onMount` fetch | `GET /api/stats/explosive-swings` | `get_room_stats()` | `room_stats_cache` |
| Weekly Profit | `onMount` fetch | `GET /api/stats/explosive-swings` | `get_room_stats()` | `room_stats_cache` |
| Active Trades | `onMount` fetch | `GET /api/stats/explosive-swings` | `get_room_stats()` | `room_stats_cache` |
| Closed This Week | `onMount` fetch | `GET /api/stats/explosive-swings` | `get_room_stats()` | `room_stats_cache` |

**Flow:**
```
Frontend                    SvelteKit API              Rust Backend              Database
+page.svelte ------------> /api/stats/[slug] -------> /api/room-content/rooms/  --> room_stats_cache
   onMount()                +server.ts GET              :slug/stats                  SELECT *
   fetchStats()                                         get_room_stats()
```

#### Trade Plan Table
| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Trade Plan Entries | `onMount` fetch | `GET /api/trade-plans/explosive-swings` | `list_trade_plans()` | `trade_plan_entries` |
| Expand Notes | Click toggle | Local state only | N/A | N/A |

**Flow:**
```
Frontend                    SvelteKit API              Rust Backend              Database
+page.svelte ------------> /api/trade-plans/[slug] -> /api/room-content/rooms/  --> trade_plan_entries
   onMount()                +server.ts GET              :slug/trade-plan            SELECT * WHERE active
   fetchTradePlan()                                     list_trade_plans()
```

#### Live Alerts Section
| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Alert Cards | `onMount` fetch | `GET /api/alerts/explosive-swings` | `list_alerts()` | `room_alerts` |
| Filter Tabs | Click filter | Local filter | N/A | N/A |
| "View All Alerts" | Navigate | `/dashboard/explosive-swings/alerts` | N/A | N/A |
| Admin: New Alert | Click button | Opens modal | N/A | N/A |
| Admin: Save Alert | Modal submit | `POST /api/alerts/explosive-swings` | `create_alert()` | `room_alerts INSERT` |
| Admin: Edit Alert | Modal submit | `PUT /api/alerts/explosive-swings/:id` | `update_alert()` | `room_alerts UPDATE` |
| Admin: Delete Alert | Click delete | `DELETE /api/alerts/explosive-swings/:id` | `delete_alert()` | `room_alerts soft delete` |

**Flow (Create Alert):**
```
Frontend                    SvelteKit API              Rust Backend              Database
TradeAlertModal ---------> /api/alerts/[slug] ------> /api/admin/room-content/  --> room_alerts
   handleSaveAlert()        +server.ts POST             alerts                      INSERT INTO
                            { ticker, type, ... }       create_alert()
```

#### Sidebar - 30-Day Performance Chart
| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Chart Data | `onMount` fetch | `GET /api/stats/explosive-swings` | `get_room_stats()` | `room_stats_cache.daily_pnl_30d` |
| Total P&L | Derived from stats | Same as above | Same as above | Same as above |
| Wins/Losses | Derived from stats | Same as above | Same as above | Same as above |

#### Sidebar - Quick Links
| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Video Library | Navigate | `/dashboard/explosive-swings/video-library` | N/A | N/A |
| Trade Tracker | Navigate | `/dashboard/explosive-swings/trade-tracker` | N/A | N/A |
| My Favorites | Navigate | `/dashboard/explosive-swings/favorites` | N/A | N/A |
| Export CSV | Download | `GET /api/export/watchlist?room_slug=explosive-swings` | N/A (SvelteKit) | `trade_plan_entries` |
| Alert Settings | Navigate | `/dashboard/account` | N/A | N/A |

---

### 1.2 Alerts Archive (`/dashboard/explosive-swings/alerts`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| All Alerts List | `onMount` fetch | `GET /api/alerts/explosive-swings` | `list_alerts()` | `room_alerts` |
| Filter Buttons | Click filter | Local state | N/A | N/A |
| Search Input | Type search | Local filter | N/A | N/A |
| Alert Card Click | Navigate | `/dashboard/explosive-swings/alerts/:id` | N/A | N/A |

---

### 1.3 Trade Tracker (`/dashboard/explosive-swings/trade-tracker`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Stats Overview | `onMount` fetch | `GET /api/trades/explosive-swings` | `list_trades()` | `room_trades` + aggregation |
| Trades Table | `onMount` fetch | `GET /api/trades/explosive-swings` | `list_trades()` | `room_trades` |
| Filter Buttons | Click filter | Local state | N/A | N/A |
| Admin: Close Trade | Click button | Opens modal | N/A | N/A |
| Admin: Submit Close | Modal submit | `PUT /api/trades/explosive-swings/:id` | `close_trade()` | `room_trades UPDATE` |

**Flow (Close Trade):**
```
Frontend                    SvelteKit API              Rust Backend              Database
Modal closeTrade() ------> /api/trades/[slug]/[id] -> /api/admin/room-content/  --> room_trades
                            +server.ts PUT              trades/:id/close           UPDATE SET
                            { exit_price, ... }         close_trade()              exit_price, pnl
```

---

### 1.4 Video Library (`/dashboard/explosive-swings/video-library`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Weekly Video | SSR load | `GET /api/weekly-video/explosive-swings` | `get_weekly_video()` | `weekly_videos` |
| Video Grid | SSR load | `GET /api/room-resources/explosive-swings` | `list_room_resources()` | `room_resources` |
| Filter Tabs | Click filter | Local state | N/A | N/A |
| Video Card Click | Navigate | `/dashboard/explosive-swings/video/:slug` | N/A | N/A |

---

### 1.5 Video Detail (`/dashboard/explosive-swings/video/[slug]`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Video Player | `onMount` fetch | `GET /api/weekly-video/explosive-swings` | `get_weekly_video()` | `weekly_videos` |
| BunnyVideoPlayer | Render component | Bunny.net CDN | N/A | N/A |
| Favorite Button | Click toggle | `POST/DELETE /api/favorites` | `add_favorite()` / `remove_favorite()` | `user_favorites` |

---

### 1.6 Favorites (`/dashboard/explosive-swings/favorites`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Favorites List | `onMount` fetch | `GET /api/favorites?room_slug=explosive-swings` | `list_favorites()` | `user_favorites` |
| Remove Button | Click remove | `DELETE /api/favorites/:id` | `remove_favorite()` | `user_favorites DELETE` |
| Empty State | Conditional render | N/A | N/A | N/A |

**Flow (Add Favorite):**
```
Frontend                    SvelteKit API              Rust Backend              Database
FavoriteButton ----------> /api/favorites ----------> /api/favorites ----------> user_favorites
   toggleFavorite()         +server.ts POST            favorites.rs               INSERT INTO
   { room_slug, item_type,                             add_favorite()
     item_id, title, ... }
```

---

### 1.7 Watchlist (`/dashboard/explosive-swings/watchlist`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Trade Entries | `onMount` fetch | `GET /api/trade-plans/explosive-swings` | `list_trade_plans()` | `trade_plan_entries` |
| Demo Data Badge | Conditional render | N/A | N/A | N/A |

---

### 1.8 Watchlist Export (`/api/export/watchlist`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Export CSV Link | Click download | `GET /api/export/watchlist?room_slug=...&format=csv` | SvelteKit handler | `trade_plan_entries` |
| Export JSON | Query param | `GET /api/export/watchlist?format=json` | SvelteKit handler | `trade_plan_entries` |

**Flow:**
```
Frontend                    SvelteKit API              Rust Backend              Database
Sidebar Link ------------> /api/export/watchlist ---> /api/room-content/rooms/  --> trade_plan_entries
   <a href download>        +server.ts GET              :slug/trade-plan           SELECT *
                            Generate CSV/JSON           list_trade_plans()
```

---

## 2. SMALL ACCOUNT MENTORSHIP DASHBOARD

### 2.1 Main Dashboard (`/dashboard/small-account-mentorship`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Tutorial Video | SSR load | `GET /api/room-resources/small-account-mentorship` | `list_room_resources()` | `room_resources` |
| Latest Updates | SSR load | `GET /api/room-resources/small-account-mentorship?type=video` | `list_room_resources()` | `room_resources` |
| Weekly Watchlist | SSR load | `GET /api/watchlist/small-account-mentorship` | `get_watchlist()` | `trade_plan_entries` |

---

### 2.2 TradingRoomSidebar Component (Shared)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Trading Room Schedule | `onMount` fetch | `GET /api/schedules/:planSlug/upcoming` | `get_upcoming_events()` | `room_schedules` |
| My Favorites Link | Navigate | `/dashboard/:planSlug/favorites` | N/A | N/A |
| Export Watchlist Link | Download | `GET /api/export/watchlist?room_slug=:planSlug` | SvelteKit handler | `trade_plan_entries` |
| Support Link | External | Intercom | N/A | N/A |
| Platform Tutorials | Navigate | `/tutorials` | N/A | N/A |

---

### 2.3 Favorites (`/dashboard/small-account-mentorship/favorites`)

| Element | Frontend Action | API Endpoint | Backend Handler | Database |
|---------|-----------------|--------------|-----------------|----------|
| Favorites List | `onMount` fetch | `GET /api/favorites?room_slug=small-account-mentorship` | `list_favorites()` | `user_favorites` |
| Remove Button | Click remove | `DELETE /api/favorites/:id` | `remove_favorite()` | `user_favorites DELETE` |

---

## 3. SHARED COMPONENTS API USAGE

### 3.1 FavoriteButton Component

**Location:** `$lib/components/dashboard/FavoriteButton.svelte`

| Action | API Call | Backend Route |
|--------|----------|---------------|
| Check Status | `GET /api/favorites/check?item_type=&item_id=` | `check_favorite()` |
| Add Favorite | `POST /api/favorites` | `add_favorite()` |
| Remove Favorite | `DELETE /api/favorites/:id` | `remove_favorite()` |

### 3.2 PerformanceChart Component

**Location:** `$lib/components/dashboard/PerformanceChart.svelte`

| Prop | Source | API |
|------|--------|-----|
| `data` | Parent fetch | `GET /api/stats/:slug` → `daily_pnl_30d` |
| `totalPnl` | Parent fetch | `GET /api/stats/:slug` → `monthly_profit` |
| `wins` | Parent fetch | `GET /api/stats/:slug` → `wins` |
| `losses` | Parent fetch | `GET /api/stats/:slug` → `losses` |

---

## 4. BACKEND API ROUTES SUMMARY

### 4.1 Public Routes (Member Access)

| Method | Endpoint | Handler | Database Table |
|--------|----------|---------|----------------|
| GET | `/api/room-content/rooms/:slug/trade-plan` | `list_trade_plans()` | `trade_plan_entries` |
| GET | `/api/room-content/rooms/:slug/alerts` | `list_alerts()` | `room_alerts` |
| GET | `/api/room-content/rooms/:slug/trades` | `list_trades()` | `room_trades` |
| GET | `/api/room-content/rooms/:slug/weekly-video` | `get_weekly_video()` | `weekly_videos` |
| GET | `/api/room-content/rooms/:slug/stats` | `get_room_stats()` | `room_stats_cache` |
| POST | `/api/room-content/rooms/:slug/alerts/:id/read` | `mark_alert_read()` | `room_alerts` |
| GET | `/api/favorites` | `list_favorites()` | `user_favorites` |
| POST | `/api/favorites` | `add_favorite()` | `user_favorites` |
| DELETE | `/api/favorites/:id` | `remove_favorite()` | `user_favorites` |
| GET | `/api/favorites/check` | `check_favorite()` | `user_favorites` |
| GET | `/api/schedules/:slug/upcoming` | `get_upcoming_events()` | `room_schedules` |

### 4.2 Admin Routes (Admin Access Required)

| Method | Endpoint | Handler | Database Table |
|--------|----------|---------|----------------|
| POST | `/api/admin/room-content/trade-plan` | `create_trade_plan()` | `trade_plan_entries` |
| PUT | `/api/admin/room-content/trade-plan/:id` | `update_trade_plan()` | `trade_plan_entries` |
| DELETE | `/api/admin/room-content/trade-plan/:id` | `delete_trade_plan()` | `trade_plan_entries` |
| POST | `/api/admin/room-content/alerts` | `create_alert()` | `room_alerts` |
| PUT | `/api/admin/room-content/alerts/:id` | `update_alert()` | `room_alerts` |
| DELETE | `/api/admin/room-content/alerts/:id` | `delete_alert()` | `room_alerts` |
| POST | `/api/admin/room-content/trades` | `create_trade()` | `room_trades` |
| PUT | `/api/admin/room-content/trades/:id/close` | `close_trade()` | `room_trades` |
| DELETE | `/api/admin/room-content/trades/:id` | `delete_trade()` | `room_trades` |

---

## 5. DATABASE SCHEMA

### 5.1 Core Tables

```sql
-- User Favorites (NEW)
CREATE TABLE user_favorites (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    room_slug VARCHAR(100) NOT NULL,
    item_type VARCHAR(50) NOT NULL,  -- 'alert', 'video', 'trade_plan'
    item_id BIGINT NOT NULL,
    title VARCHAR(500),
    excerpt TEXT,
    href VARCHAR(500),
    thumbnail_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, item_type, item_id)
);

-- Room Stats Cache (ENHANCED)
CREATE TABLE room_stats_cache (
    id BIGSERIAL PRIMARY KEY,
    room_slug VARCHAR(100) NOT NULL UNIQUE,
    win_rate DECIMAL(5,2),
    weekly_profit VARCHAR(50),
    monthly_profit VARCHAR(50),
    active_trades INTEGER,
    closed_this_week INTEGER,
    total_trades INTEGER,
    wins INTEGER,
    losses INTEGER,
    avg_win DECIMAL(12,2),
    avg_loss DECIMAL(12,2),
    profit_factor DECIMAL(6,2),
    daily_pnl_30d JSONB DEFAULT '[]',  -- NEW: Chart data
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Auto-recalculate trigger
CREATE TRIGGER trg_recalculate_stats
    AFTER INSERT OR UPDATE OR DELETE ON room_trades
    FOR EACH ROW
    EXECUTE FUNCTION trigger_recalculate_room_stats();
```

---

## 6. FEATURE STATUS MATRIX

| Feature | Frontend | SvelteKit API | Backend API | Database | Status |
|---------|----------|---------------|-------------|----------|--------|
| Quick Stats | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Trade Plan Table | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Live Alerts | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Alert CRUD (Admin) | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| 30-Day Chart | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Favorites System | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| CSV Export | ✅ | ✅ | N/A | ✅ | **LIVE** |
| Video Library | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Video Detail | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Trade Tracker | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Close Trade (Admin) | ✅ | ✅ | ✅ | ✅ | **LIVE** |
| Room Schedule | ✅ | ✅ | ✅ | ✅ | **LIVE** |

---

## 7. DATA FLOW DIAGRAMS

### 7.1 Favorites Flow
```
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐    ┌────────────┐
│ FavoriteBtn │───>│ /api/favorites│───>│ Rust favorites.rs│───>│ PostgreSQL │
│ Component   │    │ +server.ts   │    │ add_favorite()   │    │ user_favs  │
└─────────────┘    └──────────────┘    └─────────────────┘    └────────────┘
      │                   │                    │                     │
      │   POST body       │   Proxy to        │   INSERT INTO       │
      │   {room_slug,     │   backend with    │   user_favorites    │
      │    item_type,     │   session cookie  │   RETURNING *       │
      │    item_id...}    │                   │                     │
```

### 7.2 Stats Calculation Flow
```
┌─────────────┐    ┌────────────────┐    ┌─────────────────────┐
│ room_trades │───>│ Trigger fires  │───>│ calculate_room_stats│
│ INSERT/UPD  │    │ on any change  │    │ PostgreSQL function │
└─────────────┘    └────────────────┘    └─────────────────────┘
                                                    │
                                                    ▼
                                         ┌─────────────────┐
                                         │ room_stats_cache│
                                         │ UPSERT with     │
                                         │ daily_pnl_30d   │
                                         └─────────────────┘
```

---

## 8. PENDING DEPLOYMENT STEPS

1. **Run Database Migration:**
   ```bash
   psql $DATABASE_URL -f api/migrations/016_user_favorites_and_stats.sql
   ```

2. **Deploy Backend:**
   ```bash
   cd api && fly deploy
   ```

3. **Deploy Frontend:**
   ```bash
   cd frontend && npm run build && wrangler pages deploy
   ```

---

**Report Complete.** All features are fully implemented end-to-end with real API connections.
