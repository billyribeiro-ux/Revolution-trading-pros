# Explosive Swings Dashboard - Real Data Implementation Guide

## Overview

This document outlines how to convert the current mock data in the Explosive Swings tester dashboard into a fully functional, real-time system with live data.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (SvelteKit)                     │
│  /dashboard/explosive-swings/tester/+page.svelte            │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ├─ SSR Load Function (+page.server.ts)
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│              Backend API (Node.js/Express)                   │
│         https://revolution-trading-pros-api.fly.dev          │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ├─ REST Endpoints
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│                    Database (PostgreSQL)                     │
│              + Redis Cache for real-time data                │
└─────────────────────────────────────────────────────────────┘
```

---

## 1. Database Schema

### Tables Needed

#### `weekly_videos`

```sql
CREATE TABLE weekly_videos (
  id SERIAL PRIMARY KEY,
  week_of DATE NOT NULL,
  title VARCHAR(255) NOT NULL,
  video_url TEXT NOT NULL,
  thumbnail_url TEXT,
  duration VARCHAR(10),
  published_at TIMESTAMP NOT NULL,
  is_featured BOOLEAN DEFAULT false,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_weekly_videos_week ON weekly_videos(week_of DESC);
CREATE INDEX idx_weekly_videos_featured ON weekly_videos(is_featured, week_of DESC);
```

#### `trade_plan`

```sql
CREATE TABLE trade_plan (
  id SERIAL PRIMARY KEY,
  week_of DATE NOT NULL,
  ticker VARCHAR(10) NOT NULL,
  bias VARCHAR(20) NOT NULL, -- BULLISH, BEARISH, NEUTRAL
  entry_price DECIMAL(10,2),
  target1_price DECIMAL(10,2),
  target2_price DECIMAL(10,2),
  target3_price DECIMAL(10,2),
  runner_price VARCHAR(20),
  stop_price DECIMAL(10,2),
  options_strike VARCHAR(50),
  options_expiration DATE,
  notes TEXT,
  status VARCHAR(20) DEFAULT 'active', -- active, watching, closed
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_trade_plan_week ON trade_plan(week_of DESC);
CREATE INDEX idx_trade_plan_ticker ON trade_plan(ticker);
CREATE INDEX idx_trade_plan_status ON trade_plan(status);
```

#### `alerts`

```sql
CREATE TABLE alerts (
  id SERIAL PRIMARY KEY,
  type VARCHAR(20) NOT NULL, -- ENTRY, EXIT, UPDATE
  ticker VARCHAR(10) NOT NULL,
  title VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  published_at TIMESTAMP NOT NULL,
  is_new BOOLEAN DEFAULT true,
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_alerts_published ON alerts(published_at DESC);
CREATE INDEX idx_alerts_type ON alerts(type);
CREATE INDEX idx_alerts_ticker ON alerts(ticker);
```

#### `video_updates`

```sql
CREATE TABLE video_updates (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  excerpt TEXT,
  video_url TEXT NOT NULL,
  thumbnail_url TEXT,
  duration VARCHAR(10),
  published_at TIMESTAMP NOT NULL,
  ticker VARCHAR(10),
  update_type VARCHAR(20), -- entry, exit, update, short
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_video_updates_published ON video_updates(published_at DESC);
CREATE INDEX idx_video_updates_ticker ON video_updates(ticker);
```

#### `performance_stats`

```sql
CREATE TABLE performance_stats (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  win_rate DECIMAL(5,2),
  total_profit DECIMAL(12,2),
  total_trades INTEGER,
  wins INTEGER,
  losses INTEGER,
  avg_hold_days DECIMAL(4,1),
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_performance_date ON performance_stats(date DESC);
```

#### `trades_history`

```sql
CREATE TABLE trades_history (
  id SERIAL PRIMARY KEY,
  ticker VARCHAR(10) NOT NULL,
  entry_date DATE NOT NULL,
  exit_date DATE,
  entry_price DECIMAL(10,2),
  exit_price DECIMAL(10,2),
  profit_amount DECIMAL(10,2),
  profit_percent DECIMAL(6,2),
  setup_type VARCHAR(50), -- Breakout, Momentum, Earnings, Reversal
  result VARCHAR(10), -- WIN, LOSS
  duration_days INTEGER,
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_trades_ticker ON trades_history(ticker);
CREATE INDEX idx_trades_exit_date ON trades_history(exit_date DESC);
```

---

## 2. Backend API Endpoints

### Base URL: `https://revolution-trading-pros-api.fly.dev/api/explosive-swings`

#### GET `/weekly-content`

Returns the featured video and trade plan for the current week.

**Response:**

```json
{
	"video": {
		"title": "Week of January 13, 2026",
		"videoTitle": "Weekly Breakdown: Top Swing Setups",
		"videoUrl": "https://player.vimeo.com/video/123456789",
		"thumbnail": "https://cdn.simplertrading.com/...",
		"duration": "24:35",
		"publishedDate": "2026-01-13T09:00:00Z"
	},
	"tradePlan": [
		{
			"ticker": "NVDA",
			"bias": "BULLISH",
			"entry": "$142.50",
			"target1": "$148.00",
			"target2": "$152.00",
			"target3": "$158.00",
			"runner": "$165.00+",
			"stop": "$136.00",
			"optionsStrike": "$145 Call",
			"optionsExp": "Jan 24, 2026",
			"notes": "Breakout above consolidation..."
		}
	]
}
```

**Implementation:**

```javascript
// backend/routes/explosive-swings.js
router.get('/weekly-content', async (req, res) => {
	try {
		const currentWeek = getCurrentWeekStart(); // Helper function

		// Get featured video
		const video = await db.query(
			`SELECT * FROM weekly_videos 
       WHERE week_of = $1 AND is_featured = true 
       LIMIT 1`,
			[currentWeek]
		);

		// Get trade plan
		const tradePlan = await db.query(
			`SELECT * FROM trade_plan 
       WHERE week_of = $1 
       ORDER BY ticker`,
			[currentWeek]
		);

		res.json({
			video: video.rows[0],
			tradePlan: tradePlan.rows
		});
	} catch (error) {
		res.status(500).json({ error: error.message });
	}
});
```

#### GET `/stats`

Returns current performance statistics.

**Response:**

```json
{
	"winRate": 82,
	"weeklyProfit": "+$4,850",
	"activeTrades": 4,
	"closedThisWeek": 2,
	"totalProfit30Days": 18750,
	"totalTrades30Days": 28,
	"avgHoldDays": 4.2
}
```

**Implementation:**

```javascript
router.get('/stats', async (req, res) => {
	try {
		const thirtyDaysAgo = new Date();
		thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);

		// Get 30-day stats
		const stats = await db.query(
			`SELECT 
        AVG(win_rate) as win_rate,
        SUM(total_profit) as total_profit,
        SUM(total_trades) as total_trades,
        AVG(avg_hold_days) as avg_hold_days
       FROM performance_stats 
       WHERE date >= $1`,
			[thirtyDaysAgo]
		);

		// Get active trades count
		const activeTrades = await db.query(
			`SELECT COUNT(*) FROM trade_plan 
       WHERE status = 'active'`
		);

		// Get this week's closed trades
		const weekStart = getCurrentWeekStart();
		const closedThisWeek = await db.query(
			`SELECT COUNT(*) FROM trades_history 
       WHERE exit_date >= $1`,
			[weekStart]
		);

		res.json({
			winRate: Math.round(stats.rows[0].win_rate),
			totalProfit30Days: stats.rows[0].total_profit,
			totalTrades30Days: stats.rows[0].total_trades,
			avgHoldDays: parseFloat(stats.rows[0].avg_hold_days).toFixed(1),
			activeTrades: parseInt(activeTrades.rows[0].count),
			closedThisWeek: parseInt(closedThisWeek.rows[0].count)
		});
	} catch (error) {
		res.status(500).json({ error: error.message });
	}
});
```

#### GET `/alerts?type=all&limit=10`

Returns recent alerts with optional filtering.

**Query Params:**

- `type`: all, entry, exit, update
- `limit`: number of alerts to return (default: 10)

**Response:**

```json
{
	"alerts": [
		{
			"id": 1,
			"type": "ENTRY",
			"ticker": "NVDA",
			"title": "Opening NVDA Swing Position",
			"time": "2026-01-13T10:32:00Z",
			"message": "Entering NVDA at $142.50...",
			"isNew": true
		}
	]
}
```

**Implementation:**

```javascript
router.get('/alerts', async (req, res) => {
	try {
		const { type = 'all', limit = 10 } = req.query;

		let query = `SELECT * FROM alerts WHERE 1=1`;
		const params = [];

		if (type !== 'all') {
			params.push(type.toUpperCase());
			query += ` AND type = $${params.length}`;
		}

		query += ` ORDER BY published_at DESC LIMIT $${params.length + 1}`;
		params.push(limit);

		const alerts = await db.query(query, params);

		res.json({ alerts: alerts.rows });
	} catch (error) {
		res.status(500).json({ error: error.message });
	}
});
```

#### GET `/video-updates?limit=6`

Returns recent video updates.

**Response:**

```json
{
	"updates": [
		{
			"id": 1,
			"title": "NVDA Entry Alert - Opening Swing Position",
			"date": "2026-01-13T10:32:00Z",
			"excerpt": "Entering NVDA at $142.50...",
			"href": "/dashboard/explosive-swings/updates/nvda-entry-011326",
			"image": "https://cdn.simplertrading.com/...",
			"isVideo": true,
			"duration": "8:45"
		}
	]
}
```

#### GET `/performance-chart?days=30`

Returns performance data for charting.

**Response:**

```json
{
	"data": [
		{ "date": "2025-12-15", "profit": 2450 },
		{ "date": "2025-12-22", "profit": 5200 },
		{ "date": "2025-12-29", "profit": 8750 }
	]
}
```

---

## 3. Frontend Integration

### Update `+page.server.ts`

```typescript
import type { ServerLoadEvent } from '@sveltejs/kit';

const API_BASE = 'https://revolution-trading-pros-api.fly.dev/api/explosive-swings';

export async function load({ fetch }: ServerLoadEvent) {
	try {
		// Fetch all data in parallel
		const [weeklyContent, stats, alerts, videoUpdates] = await Promise.all([
			fetch(`${API_BASE}/weekly-content`).then((r) => r.json()),
			fetch(`${API_BASE}/stats`).then((r) => r.json()),
			fetch(`${API_BASE}/alerts?limit=10`).then((r) => r.json()),
			fetch(`${API_BASE}/video-updates?limit=6`).then((r) => r.json())
		]);

		return {
			weeklyContent,
			stats,
			alerts: alerts.alerts,
			videoUpdates: videoUpdates.updates
		};
	} catch (error) {
		console.error('Failed to load dashboard data:', error);

		// Return empty/fallback data
		return {
			weeklyContent: { video: null, tradePlan: [] },
			stats: {},
			alerts: [],
			videoUpdates: []
		};
	}
}
```

### Update `+page.svelte`

```typescript
<script lang="ts">
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	// Get SSR data
	let { data } = $props();

	// Use real data from server
	const weeklyContent = data.weeklyContent.video;
	const tradePlan = data.weeklyContent.tradePlan;
	const stats = data.stats;
	const alerts = data.alerts;
	const latestUpdates = data.videoUpdates;

	// State
	let heroTab = $state<'video' | 'entries'>('video');
	let selectedFilter = $state('all');

	// Filtered alerts
	const filteredAlerts = $derived(
		selectedFilter === 'all'
			? alerts
			: alerts.filter(a => a.type.toLowerCase() === selectedFilter)
	);
</script>
```

---

## 4. Real-Time Updates

### WebSocket Implementation (Optional but Recommended)

For real-time alert notifications:

```javascript
// backend/websocket.js
const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });

// Broadcast new alert to all connected clients
function broadcastAlert(alert) {
	wss.clients.forEach((client) => {
		if (client.readyState === WebSocket.OPEN) {
			client.send(
				JSON.stringify({
					type: 'NEW_ALERT',
					data: alert
				})
			);
		}
	});
}

module.exports = { broadcastAlert };
```

```typescript
// frontend: +page.svelte
onMount(() => {
	const ws = new WebSocket('wss://revolution-trading-pros-api.fly.dev/ws');

	ws.onmessage = (event) => {
		const message = JSON.parse(event.data);
		if (message.type === 'NEW_ALERT') {
			// Add new alert to the list
			alerts = [message.data, ...alerts];
		}
	};

	return () => ws.close();
});
```

---

## 5. Admin Panel for Content Management

Create an admin interface to manage all content:

### Routes Needed:

- `/admin/explosive-swings/weekly-video` - Upload/edit weekly video
- `/admin/explosive-swings/trade-plan` - Manage trade plan entries
- `/admin/explosive-swings/alerts` - Create/send alerts
- `/admin/explosive-swings/video-updates` - Upload video updates

### Example: Create Alert Form

```svelte
<!-- /admin/explosive-swings/alerts/+page.svelte -->
<script lang="ts">
	let alertForm = $state({
		type: 'ENTRY',
		ticker: '',
		title: '',
		message: ''
	});

	async function createAlert() {
		const response = await fetch('/api/explosive-swings/alerts', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(alertForm)
		});

		if (response.ok) {
			// Success - maybe trigger WebSocket broadcast
			alert('Alert created!');
		}
	}
</script>

<form onsubmit={createAlert}>
	<select bind:value={alertForm.type}>
		<option value="ENTRY">Entry</option>
		<option value="EXIT">Exit</option>
		<option value="UPDATE">Update</option>
	</select>

	<input bind:value={alertForm.ticker} placeholder="Ticker (e.g., NVDA)" />
	<input bind:value={alertForm.title} placeholder="Alert Title" />
	<textarea bind:value={alertForm.message} placeholder="Alert Message"></textarea>

	<button type="submit">Create Alert</button>
</form>
```

---

## 6. Video Storage & CDN

### Recommended Setup:

1. **Video Hosting**: Vimeo Pro or Wistia
2. **Thumbnails**: Cloudinary or AWS S3 + CloudFront
3. **Process**:
   - Upload video to Vimeo
   - Get embed URL and thumbnail
   - Store URLs in database
   - Frontend fetches and displays

---

## 7. Caching Strategy

### Redis Cache

```javascript
const redis = require('redis');
const client = redis.createClient();

// Cache weekly content for 1 hour
router.get('/weekly-content', async (req, res) => {
	const cacheKey = `weekly-content:${getCurrentWeekStart()}`;

	// Try cache first
	const cached = await client.get(cacheKey);
	if (cached) {
		return res.json(JSON.parse(cached));
	}

	// Fetch from DB
	const data = await fetchWeeklyContent();

	// Cache for 1 hour
	await client.setex(cacheKey, 3600, JSON.stringify(data));

	res.json(data);
});
```

---

## 8. Deployment Checklist

- [ ] Set up PostgreSQL database
- [ ] Set up Redis for caching
- [ ] Deploy backend API to Fly.io
- [ ] Configure environment variables
- [ ] Set up video hosting (Vimeo/Wistia)
- [ ] Set up CDN for images (Cloudinary)
- [ ] Create admin panel
- [ ] Test all API endpoints
- [ ] Implement WebSocket for real-time updates
- [ ] Set up monitoring (Sentry, LogRocket)
- [ ] Configure CORS properly
- [ ] Add rate limiting
- [ ] Set up automated backups

---

## 9. Migration Path

### Phase 1: Database Setup (Week 1)

1. Create all tables
2. Seed with initial data
3. Test queries

### Phase 2: API Development (Week 2)

1. Build all endpoints
2. Add authentication/authorization
3. Test with Postman

### Phase 3: Frontend Integration (Week 3)

1. Update `+page.server.ts`
2. Update `+page.svelte` to use real data
3. Test thoroughly

### Phase 4: Admin Panel (Week 4)

1. Build admin UI
2. Add content management features
3. Train team on usage

### Phase 5: Real-Time Features (Week 5)

1. Implement WebSocket
2. Add push notifications
3. Test live updates

---

## 10. Environment Variables

```env
# .env
DATABASE_URL=postgresql://user:password@host:5432/dbname
REDIS_URL=redis://host:6379
VIMEO_ACCESS_TOKEN=your_vimeo_token
CLOUDINARY_URL=cloudinary://key:secret@cloud_name
JWT_SECRET=your_jwt_secret
API_BASE_URL=https://revolution-trading-pros-api.fly.dev
```

---

## Summary

To make everything real:

1. **Database**: PostgreSQL with 7 tables
2. **Backend**: Node.js/Express API with 6 main endpoints
3. **Frontend**: Update SvelteKit to fetch from API
4. **Admin**: Build content management interface
5. **Real-time**: WebSocket for live alerts
6. **Storage**: Vimeo for videos, Cloudinary for images
7. **Cache**: Redis for performance

The mock data structure is already perfect - just need to wire it to real sources!
