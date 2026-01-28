# Explosive Swings API Documentation

> Complete API reference for the Explosive Swings trading room endpoints.
> Version 1.0.0 | Last Updated: January 2026

## Table of Contents

1. [Overview](#overview)
2. [Authentication](#authentication)
3. [Base URLs](#base-urls)
4. [Alerts API](#alerts-api)
5. [Trades API](#trades-api)
6. [Trade Plans API](#trade-plans-api)
7. [Stats API](#stats-api)
8. [Search API](#search-api)
9. [Export API](#export-api)
10. [Analytics API](#analytics-api)
11. [Error Handling](#error-handling)
12. [Rate Limiting](#rate-limiting)

---

## Overview

The Explosive Swings API provides programmatic access to trading room data including:

- **Alerts**: Entry, Update, and Exit alerts with full TOS (ThinkOrSwim) format support
- **Trades**: Open, close, and track positions with P&L calculations
- **Trade Plans**: Weekly trade plans with targets, stops, and options details
- **Stats**: Performance statistics including win rate, profit factor, and streaks
- **Search**: Full-text search across alerts, trades, and trade plans
- **Export**: CSV exports for alerts and trades
- **Analytics**: Comprehensive analytics with equity curves and performance breakdowns

### Response Format

All successful responses follow this structure:

```json
{
  "data": { ... },
  "meta": {
    "current_page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

---

## Authentication

All endpoints require Bearer token authentication:

```
Authorization: Bearer <your-jwt-token>
```

### Obtaining a Token

Tokens are obtained through the authentication flow. Contact support for API access.

---

## Base URLs

| Environment | URL |
|-------------|-----|
| Production  | `https://revolution-trading-pros-api.fly.dev` |
| Development | `http://localhost:8080` |

---

## Alerts API

Trading alerts with full TOS format support for options and shares.

### GET /api/room-content/rooms/{room_slug}/alerts

Retrieve paginated alerts for a trading room.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier (e.g., "explosive-swings") |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `per_page` | integer | 20 | Items per page (max: 100) |

**Example Request:**

```bash
curl -X GET "https://revolution-trading-pros-api.fly.dev/api/room-content/rooms/explosive-swings/alerts?page=1&per_page=10" \
  -H "Authorization: Bearer <token>"
```

**Example Response:**

```json
{
  "data": [
    {
      "id": 1234,
      "room_id": 1,
      "room_slug": "explosive-swings",
      "alert_type": "ENTRY",
      "ticker": "NVDA",
      "title": "NVDA Call Entry",
      "message": "Entering NVDA calls on strength above 950",
      "notes": "ICT setup - bullish order block",
      "trade_type": "options",
      "action": "BUY",
      "quantity": 5,
      "option_type": "CALL",
      "strike": 950.0,
      "expiration": "2026-02-21",
      "contract_type": "Weeklys",
      "order_type": "MKT",
      "limit_price": null,
      "fill_price": 12.50,
      "tos_string": "BUY +5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
      "entry_alert_id": null,
      "trade_plan_id": 45,
      "is_new": true,
      "is_published": true,
      "is_pinned": false,
      "published_at": "2026-01-28T14:30:00Z",
      "created_at": "2026-01-28T14:30:00Z",
      "updated_at": "2026-01-28T14:30:00Z"
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 10,
    "total": 156,
    "total_pages": 16
  }
}
```

---

### POST /api/room-content/alerts

Create a new trading alert (Admin only).

**Request Body:**

```json
{
  "room_slug": "explosive-swings",
  "alert_type": "ENTRY",
  "ticker": "NVDA",
  "title": "NVDA Call Entry",
  "message": "Entering NVDA calls on strength above 950",
  "notes": "ICT setup - bullish order block",
  "trade_type": "options",
  "action": "BUY",
  "quantity": 5,
  "option_type": "CALL",
  "strike": 950.0,
  "expiration": "2026-02-21",
  "contract_type": "Weeklys",
  "order_type": "MKT",
  "fill_price": 12.50,
  "tos_string": "BUY +5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
  "trade_plan_id": 45,
  "is_published": true
}
```

**Alert Types:**

| Type | Description |
|------|-------------|
| `ENTRY` | New position entry |
| `UPDATE` | Position update (scaling, adjustment) |
| `EXIT` | Position exit (full or partial) |

**Trade Types:**

| Type | Description |
|------|-------------|
| `options` | Options contract |
| `shares` | Stock shares |

**Option Types:**

| Type | Description |
|------|-------------|
| `CALL` | Call option |
| `PUT` | Put option |

**Contract Types:**

| Type | Description |
|------|-------------|
| `Weeklys` | Weekly expiration |
| `Monthly` | Monthly expiration |
| `LEAPS` | Long-term options |

---

### PUT /api/room-content/alerts/{id}

Update an existing alert (Admin only).

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Alert ID |

**Request Body:** Same as POST, all fields optional.

---

### DELETE /api/room-content/alerts/{id}

Soft delete an alert (Admin only).

**Response:**

```json
{
  "success": true,
  "message": "Alert deleted"
}
```

---

## Trades API

Trade tracking with P&L calculations and position management.

### GET /api/room-content/rooms/{room_slug}/trades

Retrieve paginated trades for a trading room.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `per_page` | integer | 20 | Items per page (max: 100) |
| `status` | string | - | Filter by status: `open`, `closed`, `invalidated` |
| `ticker` | string | - | Filter by ticker symbol |

**Example Response:**

```json
{
  "data": [
    {
      "id": 567,
      "room_id": 1,
      "room_slug": "explosive-swings",
      "ticker": "NVDA",
      "trade_type": "options",
      "direction": "long",
      "quantity": 5,
      "option_type": "CALL",
      "strike": 950.0,
      "expiration": "2026-02-21",
      "contract_type": "Weeklys",
      "entry_alert_id": 1234,
      "entry_price": 12.50,
      "entry_date": "2026-01-28",
      "entry_tos_string": "BUY +5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
      "exit_alert_id": 1235,
      "exit_price": 18.75,
      "exit_date": "2026-01-29",
      "exit_tos_string": "SELL -5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
      "setup": "ICT Order Block",
      "status": "closed",
      "result": "WIN",
      "pnl": 3125.00,
      "pnl_percent": 50.0,
      "holding_days": 1,
      "notes": "Clean setup, hit first target",
      "was_updated": false,
      "invalidation_reason": null,
      "created_at": "2026-01-28T14:30:00Z",
      "updated_at": "2026-01-29T10:15:00Z"
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 20,
    "total": 89,
    "total_pages": 5
  }
}
```

---

### POST /api/room-content/trades

Create a new trade (Admin only).

**Request Body:**

```json
{
  "room_slug": "explosive-swings",
  "ticker": "NVDA",
  "trade_type": "options",
  "direction": "long",
  "quantity": 5,
  "option_type": "CALL",
  "strike": 950.0,
  "expiration": "2026-02-21",
  "contract_type": "Weeklys",
  "entry_alert_id": 1234,
  "entry_price": 12.50,
  "entry_date": "2026-01-28",
  "entry_tos_string": "BUY +5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
  "setup": "ICT Order Block",
  "notes": "Clean setup on morning session"
}
```

**Direction Types:**

| Type | Description |
|------|-------------|
| `long` | Long position (bullish) |
| `short` | Short position (bearish) |

---

### PUT /api/room-content/trades/{id}/close

Close a trade and calculate P&L (Admin only).

**Request Body:**

```json
{
  "exit_alert_id": 1235,
  "exit_price": 18.75,
  "exit_date": "2026-01-29",
  "exit_tos_string": "SELL -5 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT",
  "notes": "Hit first target, taking profits"
}
```

**P&L Calculation:**

For long positions: `(exit_price - entry_price) * quantity`
For short positions: `(entry_price - exit_price) * quantity`

---

### POST /api/room-content/trades/{id}/invalidate

Invalidate a trade that didn't trigger (Admin only).

**Request Body:**

```json
{
  "reason": "Price never reached entry level"
}
```

---

## Trade Plans API

Weekly trade plans with targets, stops, and options details.

### GET /api/room-content/rooms/{room_slug}/trade-plan

Retrieve trade plans for a trading room.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `per_page` | integer | 50 | Items per page (max: 100) |
| `week_of` | string | current | Filter by week (YYYY-MM-DD format) |

**Example Response:**

```json
{
  "data": [
    {
      "id": 45,
      "room_id": 1,
      "room_slug": "explosive-swings",
      "week_of": "2026-01-27",
      "ticker": "NVDA",
      "bias": "BULLISH",
      "entry": "945-950",
      "target1": "970",
      "target2": "985",
      "target3": "1000",
      "runner": "1050",
      "runner_stop": "960",
      "stop": "930",
      "options_strike": "950",
      "options_exp": "2026-02-21",
      "notes": "Strong momentum, watching for pullback to order block",
      "sort_order": 1,
      "is_active": true,
      "created_at": "2026-01-27T08:00:00Z",
      "updated_at": "2026-01-27T08:00:00Z"
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 50,
    "total": 8,
    "total_pages": 1
  }
}
```

---

### POST /api/room-content/trade-plan

Create a new trade plan entry (Admin only).

**Request Body:**

```json
{
  "room_slug": "explosive-swings",
  "week_of": "2026-01-27",
  "ticker": "NVDA",
  "bias": "BULLISH",
  "entry": "945-950",
  "target1": "970",
  "target2": "985",
  "target3": "1000",
  "runner": "1050",
  "runner_stop": "960",
  "stop": "930",
  "options_strike": "950",
  "options_exp": "2026-02-21",
  "notes": "Strong momentum, watching for pullback to order block",
  "sort_order": 1
}
```

**Bias Types:**

| Type | Description |
|------|-------------|
| `BULLISH` | Expecting price to rise |
| `BEARISH` | Expecting price to fall |

---

## Stats API

Performance statistics with win rate, P&L, and streak tracking.

### GET /api/room-content/rooms/{room_slug}/stats

Retrieve performance statistics for a trading room.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Example Response:**

```json
{
  "data": {
    "id": 1,
    "room_id": 1,
    "room_slug": "explosive-swings",
    "win_rate": 72.5,
    "weekly_profit": "+$4,250",
    "monthly_profit": "+$18,750",
    "active_trades": 3,
    "closed_this_week": 8,
    "total_trades": 156,
    "wins": 113,
    "losses": 43,
    "avg_win": 850.00,
    "avg_loss": 325.00,
    "profit_factor": 2.61,
    "avg_holding_days": 2.3,
    "largest_win": 4500.00,
    "largest_loss": 1200.00,
    "current_streak": 4,
    "calculated_at": "2026-01-28T14:00:00Z"
  }
}
```

---

## Search API

Full-text search across alerts, trades, and trade plans.

### GET /api/search/{room_slug}

Search room content.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `q` | string | - | Search query (required) |
| `limit` | integer | 20 | Max results per category |
| `offset` | integer | 0 | Results offset |

**Example Request:**

```bash
curl -X GET "https://revolution-trading-pros-api.fly.dev/api/search/explosive-swings?q=NVDA%20call" \
  -H "Authorization: Bearer <token>"
```

**Example Response:**

```json
{
  "alerts": [
    {
      "id": 1234,
      "ticker": "NVDA",
      "title": "NVDA Call Entry",
      "alert_type": "ENTRY",
      "message": "Entering NVDA calls on strength",
      "published_at": "2026-01-28T14:30:00Z",
      "relevance_score": 0.95,
      "highlight": "Entering <mark>NVDA</mark> <mark>call</mark>s on strength"
    }
  ],
  "trades": [
    {
      "id": 567,
      "ticker": "NVDA",
      "direction": "long",
      "status": "closed",
      "entry_price": 12.50,
      "exit_price": 18.75,
      "entry_date": "2026-01-28",
      "pnl_percent": 50.0,
      "result": "WIN",
      "relevance_score": 0.88,
      "highlight": "<mark>NVDA</mark> <mark>call</mark> trade"
    }
  ],
  "trade_plans": [
    {
      "id": 45,
      "ticker": "NVDA",
      "bias": "BULLISH",
      "entry": "945-950",
      "week_of": "2026-01-27",
      "is_active": true,
      "relevance_score": 0.82,
      "highlight": "<mark>NVDA</mark> bullish setup"
    }
  ],
  "total_count": 15,
  "query": "NVDA call",
  "took_ms": 45,
  "pagination": {
    "limit": 20,
    "offset": 0,
    "has_more": false
  }
}
```

---

## Export API

CSV exports for alerts and trades.

### GET /api/export/{room_slug}/alerts

Export alerts as CSV.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `start_date` | string | - | Filter start date (YYYY-MM-DD) |
| `end_date` | string | - | Filter end date (YYYY-MM-DD) |

**Response:** CSV file download with headers:

```csv
id,ticker,alert_type,title,message,tos_string,published_at
1234,NVDA,ENTRY,"NVDA Call Entry","Entering calls","BUY +5 NVDA...","2026-01-28T14:30:00Z"
```

---

### GET /api/export/{room_slug}/trades

Export trades as CSV.

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `start_date` | string | - | Filter start date |
| `end_date` | string | - | Filter end date |
| `status` | string | - | Filter by status |

**Response:** CSV file download with P&L data.

---

## Analytics API

Comprehensive analytics with equity curves and performance metrics.

### GET /api/analytics/{room_slug}

Retrieve full analytics for a trading room.

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `room_slug` | string | Yes | Room identifier |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `period` | string | 30d | Time period: `7d`, `30d`, `90d`, `1y`, `all` |

**Example Response:**

```json
{
  "summary": {
    "total_alerts": 312,
    "total_trades": 156,
    "win_rate": 72.5,
    "profit_factor": 2.61,
    "sharpe_ratio": 1.85,
    "max_drawdown": 3500.00,
    "max_drawdown_percent": 8.5,
    "avg_holding_days": 2.3,
    "total_pnl": 45000.00,
    "total_pnl_percent": 112.5,
    "avg_win_percent": 42.5,
    "avg_loss_percent": -18.2,
    "largest_win_percent": 125.0,
    "largest_loss_percent": -35.0,
    "risk_reward_ratio": 2.33,
    "expectancy": 185.50
  },
  "performance_by_ticker": [
    {
      "ticker": "NVDA",
      "total_trades": 45,
      "wins": 34,
      "losses": 11,
      "win_rate": 75.5,
      "total_pnl": 18500.00,
      "total_pnl_percent": 46.25,
      "avg_pnl": 411.11,
      "avg_pnl_percent": 10.28,
      "avg_holding_days": 1.8
    }
  ],
  "performance_by_setup": [
    {
      "setup": "ICT Order Block",
      "total_trades": 68,
      "wins": 52,
      "losses": 16,
      "win_rate": 76.5,
      "total_pnl": 22000.00,
      "profit_factor": 3.1
    }
  ],
  "monthly_performance": [
    {
      "year": 2026,
      "month": 1,
      "month_name": "January",
      "total_trades": 24,
      "wins": 18,
      "losses": 6,
      "win_rate": 75.0,
      "pnl": 8500.00,
      "pnl_percent": 21.25,
      "is_positive": true
    }
  ],
  "daily_pnl": [
    {
      "date": "2026-01-28",
      "pnl": 1250.00,
      "pnl_percent": 3.12,
      "cumulative_pnl": 45000.00,
      "cumulative_pnl_percent": 112.5,
      "trade_count": 3
    }
  ],
  "alert_effectiveness": {
    "total_alerts": 312,
    "alerts_with_trades": 156,
    "alerts_without_trades": 156,
    "conversion_rate": 50.0,
    "profitable_conversion_rate": 72.5,
    "avg_time_to_trade_hours": 2.5,
    "entry_alerts": 156,
    "update_alerts": 89,
    "exit_alerts": 67
  },
  "streak_analysis": {
    "current_streak": 4,
    "current_streak_type": "WIN",
    "max_win_streak": 12,
    "max_loss_streak": 4,
    "avg_win_streak": 3.5,
    "avg_loss_streak": 1.8
  }
}
```

---

## Error Handling

### Error Response Format

```json
{
  "success": false,
  "error": "Detailed error message"
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Missing or invalid token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource doesn't exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

### Common Errors

| Error | Description | Solution |
|-------|-------------|----------|
| `Invalid room_slug` | Room not found | Check room slug spelling |
| `Invalid date format` | Date parsing failed | Use YYYY-MM-DD format |
| `Unauthorized` | Token invalid or expired | Refresh authentication token |
| `Rate limit exceeded` | Too many requests | Wait and retry |

---

## Rate Limiting

API requests are rate-limited to prevent abuse:

| Endpoint Type | Limit |
|--------------|-------|
| Read operations | 100 requests/minute |
| Write operations | 30 requests/minute |
| Export operations | 10 requests/minute |
| Search operations | 50 requests/minute |

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1706460000
```

---

## Caching

### Cache TTLs

| Resource | TTL |
|----------|-----|
| Alerts | 60 seconds |
| Trades | 5 minutes |
| Trade Plans | 5 minutes |
| Stats | 5 minutes |
| Weekly Videos | 1 hour |

### Cache Invalidation

Caches are automatically invalidated when:
- A new resource is created
- An existing resource is updated
- A resource is deleted

---

## Changelog

### Version 1.0.0 (January 2026)

- Initial API release
- Full TOS format support for alerts
- Trade tracking with P&L calculations
- Redis caching with graceful degradation
- Full-text search
- CSV exports
- Comprehensive analytics

---

## Support

For API support, contact:
- Email: support@revolutiontradingpros.com
- Documentation: https://docs.revolutiontradingpros.com
