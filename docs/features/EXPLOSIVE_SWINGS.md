# Explosive Swings Trading Room

**Real-Time Trade Alerts & Performance Tracking**

---

## 📋 Overview

The Explosive Swings trading room is the flagship feature of Revolution Trading Pros, providing members with real-time trade alerts, comprehensive trade tracking, and performance analytics.

### Key Features

- **📢 Real-Time Alerts** - Entry, update, and exit alerts with TOS format support
- **📊 Trade Tracking** - Open/closed positions with P&L calculations
- **📈 Performance Analytics** - Win rate, profit factor, equity curves
- **📅 Trade Plans** - Weekly trade plans with targets and stops
- **🎥 Video Library** - Educational content and weekly watchlists
- **⭐ Favorites** - Save and organize favorite alerts
- **🔍 Search** - Full-text search across all content

---

## 🏗️ Architecture

### Frontend Components

```
frontend/src/routes/dashboard/explosive-swings/
├── +page.svelte              # Main dashboard
├── alerts/+page.svelte       # Alerts feed
├── trades/+page.svelte       # Trade tracker
├── watchlist/+page.svelte    # Weekly watchlist
├── video-library/+page.svelte # Video content
├── favorites/+page.svelte    # Saved alerts
└── start-here/+page.svelte   # Onboarding
```

### Backend API Endpoints

```
GET  /api/room-content/rooms/{slug}/alerts      # List alerts
POST /api/room-content/rooms/{slug}/alerts      # Create alert
GET  /api/room-content/rooms/{slug}/trades      # List trades
POST /api/room-content/rooms/{slug}/trades      # Create trade
GET  /api/room-content/rooms/{slug}/trade-plans # List trade plans
GET  /api/room-content/rooms/{slug}/stats       # Performance stats
GET  /api/room-content/rooms/{slug}/search      # Search content
GET  /api/room-content/rooms/{slug}/export      # Export CSV
```

### Database Schema

**Core Tables:**
- `explosive_swings_alerts` - Trade alerts
- `explosive_swings_trades` - Open/closed trades
- `explosive_swings_trade_plans` - Weekly trade plans
- `explosive_swings_videos` - Video content
- `explosive_swings_favorites` - User favorites

---

## 📢 Alert System

### Alert Types

1. **Entry Alert** - Initial trade entry
2. **Update Alert** - Position adjustments
3. **Exit Alert** - Trade closure

### TOS Format Support

Alerts support ThinkOrSwim (TOS) format for options:

```
SPY 450C 12/15/23 @ $2.50
```

**Format:**
- `SPY` - Symbol
- `450C` - Strike + Type (C=Call, P=Put)
- `12/15/23` - Expiration date
- `@ $2.50` - Price

### Alert Data Model

```typescript
interface Alert {
  id: string;
  room_slug: string;
  alert_type: 'entry' | 'update' | 'exit';
  symbol: string;
  message: string;
  tos_format?: string;
  price?: number;
  quantity?: number;
  created_at: string;
  updated_at: string;
}
```

### Creating Alerts

```typescript
// Frontend
const response = await fetch('/api/room-content/rooms/explosive-swings/alerts', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    alert_type: 'entry',
    symbol: 'SPY',
    message: 'Entering SPY calls',
    tos_format: 'SPY 450C 12/15/23 @ $2.50',
    price: 2.50,
    quantity: 10
  })
});
```

---

## 📊 Trade Tracking

### Trade States

- **Open** - Active position
- **Closed** - Exited position

### P&L Calculation

```typescript
function calculatePnL(trade: Trade): number {
  if (!trade.exit_price) return 0;
  
  const priceDiff = trade.exit_price - trade.entry_price;
  const multiplier = trade.is_option ? 100 : 1;
  
  return priceDiff * trade.quantity * multiplier;
}
```

### Trade Data Model

```typescript
interface Trade {
  id: string;
  room_slug: string;
  symbol: string;
  entry_price: number;
  exit_price?: number;
  quantity: number;
  is_option: boolean;
  strike_price?: number;
  expiration_date?: string;
  option_type?: 'call' | 'put';
  status: 'open' | 'closed';
  pnl?: number;
  created_at: string;
  closed_at?: string;
}
```

---

## 📈 Performance Analytics

### Key Metrics

1. **Win Rate** - Percentage of winning trades
2. **Profit Factor** - Gross profit / Gross loss
3. **Average Win** - Average profit per winning trade
4. **Average Loss** - Average loss per losing trade
5. **Largest Win** - Biggest single win
6. **Largest Loss** - Biggest single loss
7. **Current Streak** - Consecutive wins/losses

### Stats API Response

```json
{
  "total_trades": 150,
  "winning_trades": 95,
  "losing_trades": 55,
  "win_rate": 63.33,
  "total_pnl": 45000.00,
  "gross_profit": 75000.00,
  "gross_loss": 30000.00,
  "profit_factor": 2.5,
  "average_win": 789.47,
  "average_loss": 545.45,
  "largest_win": 5000.00,
  "largest_loss": 2500.00,
  "current_streak": 5,
  "streak_type": "win"
}
```

### Equity Curve

The equity curve shows cumulative P&L over time:

```typescript
interface EquityPoint {
  date: string;
  cumulative_pnl: number;
}

// Example
[
  { date: '2024-01-01', cumulative_pnl: 0 },
  { date: '2024-01-02', cumulative_pnl: 500 },
  { date: '2024-01-03', cumulative_pnl: 1200 },
  // ...
]
```

---

## 📅 Trade Plans

### Weekly Trade Plans

Trade plans provide weekly market analysis and trade setups:

```typescript
interface TradePlan {
  id: string;
  room_slug: string;
  title: string;
  week_start: string;
  week_end: string;
  symbols: string[];
  targets: string[];
  stops: string[];
  notes: string;
  created_at: string;
}
```

---

## 🎥 Video Library

### Video Types

1. **Weekly Watchlist** - Market analysis
2. **Trade Reviews** - Post-trade analysis
3. **Educational** - Strategy lessons

### Video Data Model

```typescript
interface Video {
  id: string;
  room_slug: string;
  title: string;
  description: string;
  video_url: string;
  thumbnail_url: string;
  duration: number;
  video_type: 'watchlist' | 'review' | 'educational';
  published_at: string;
}
```

---

## 🔍 Search

### Full-Text Search

Search across alerts, trades, and trade plans:

```typescript
const results = await fetch(
  '/api/room-content/rooms/explosive-swings/search?q=SPY&type=alerts'
);
```

**Query Parameters:**
- `q` - Search query
- `type` - Content type (alerts, trades, trade_plans, all)
- `limit` - Results per page (default: 20)
- `offset` - Pagination offset

---

## 📥 Export

### CSV Export

Export alerts and trades to CSV:

```typescript
// Export alerts
const csv = await fetch(
  '/api/room-content/rooms/explosive-swings/export?type=alerts&start_date=2024-01-01&end_date=2024-12-31'
);

// Export trades
const csv = await fetch(
  '/api/room-content/rooms/explosive-swings/export?type=trades&start_date=2024-01-01&end_date=2024-12-31'
);
```

---

## 🚀 Performance Optimizations

1. **Redis Caching** - 60s TTL for alerts, 5min for stats
2. **Database Indexing** - Optimized queries
3. **Pagination** - 20 items per page
4. **Lazy Loading** - Virtual scrolling for large lists
5. **CDN Delivery** - Video streaming via Bunny.net

---

## 📊 Metrics & Monitoring

- **Alert latency** - < 100ms
- **Trade calculation** - < 50ms
- **Search response** - < 200ms
- **Video load time** - < 2s
- **Cache hit rate** - > 80%

