# API Documentation

**Revolution Trading Pros - Complete API Reference**

---

## 📋 Overview

The Revolution Trading Pros API is a RESTful API built with Rust and Axum, providing access to all platform features including authentication, courses, trading rooms, and content management.

### Base URLs

| Environment | URL |
|-------------|-----|
| **Production** | `https://revolution-trading-pros-api.fly.dev` |
| **Development** | `http://localhost:8080` |

### API Version

Current version: **v1.0.0**

---

## 🔐 Authentication

All API requests (except public endpoints) require authentication using a Bearer token:

```bash
curl -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  https://revolution-trading-pros-api.fly.dev/api/users/me
```

### Obtaining a Token

```bash
# Login with email/password
curl -X POST https://revolution-trading-pros-api.fly.dev/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "your-password"
  }'

# Response
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "user"
  }
}
```

---

## 📚 API Endpoints

### Authentication

- [Authentication API](AUTH_API.md) - Login, registration, OAuth, MFA

### Users

- [Users API](USERS_API.md) - User management, profiles, settings

### Courses

- [Courses API](COURSES_API.md) - Course catalog, enrollment, progress

### Trading Rooms

- [Explosive Swings API](EXPLOSIVE_SWINGS_API.md) - Alerts, trades, analytics

### Content Management

- [CMS API](CMS_API.md) - Blog posts, content blocks, publishing
- [Media API](MEDIA_API.md) - Image/video uploads, CDN delivery

### Payments

- [Payments API](PAYMENTS_API.md) - Stripe integration, subscriptions, orders

### Admin

- [Admin API](ADMIN_API.md) - Admin operations, analytics, settings

---

## 📊 Response Format

### Success Response

```json
{
  "data": {
    "id": "123",
    "name": "Example"
  },
  "meta": {
    "current_page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

### Error Response

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": {
      "field": "email",
      "value": "invalid-email"
    }
  }
}
```

---

## 🚦 HTTP Status Codes

| Code | Meaning | Description |
|------|---------|-------------|
| **200** | OK | Request successful |
| **201** | Created | Resource created |
| **204** | No Content | Request successful, no content to return |
| **400** | Bad Request | Invalid request parameters |
| **401** | Unauthorized | Missing or invalid authentication |
| **403** | Forbidden | Insufficient permissions |
| **404** | Not Found | Resource not found |
| **422** | Unprocessable Entity | Validation error |
| **429** | Too Many Requests | Rate limit exceeded |
| **500** | Internal Server Error | Server error |
| **503** | Service Unavailable | Service temporarily unavailable |

---

## 🔒 Rate Limiting

API requests are rate-limited to prevent abuse:

| Endpoint Type | Limit |
|--------------|-------|
| **Authentication** | 5 requests/minute |
| **Read operations** | 100 requests/minute |
| **Write operations** | 30 requests/minute |
| **Upload operations** | 10 requests/minute |

### Rate Limit Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1706460000
```

### Rate Limit Exceeded

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please try again later.",
    "retry_after": 60
  }
}
```

---

## 📄 Pagination

List endpoints support pagination:

### Query Parameters

- `page` - Page number (default: 1)
- `per_page` - Items per page (default: 20, max: 100)

### Example Request

```bash
curl "https://revolution-trading-pros-api.fly.dev/api/posts?page=2&per_page=50"
```

### Example Response

```json
{
  "data": [...],
  "meta": {
    "current_page": 2,
    "per_page": 50,
    "total": 250,
    "total_pages": 5,
    "has_next": true,
    "has_prev": true
  }
}
```

---

## 🔍 Filtering & Sorting

### Filtering

```bash
# Filter by status
curl "https://revolution-trading-pros-api.fly.dev/api/posts?status=published"

# Multiple filters
curl "https://revolution-trading-pros-api.fly.dev/api/posts?status=published&category=trading"
```

### Sorting

```bash
# Sort by created_at descending
curl "https://revolution-trading-pros-api.fly.dev/api/posts?sort=-created_at"

# Sort by title ascending
curl "https://revolution-trading-pros-api.fly.dev/api/posts?sort=title"
```

---

## 🔎 Search

Full-text search is available on select endpoints:

```bash
curl "https://revolution-trading-pros-api.fly.dev/api/search?q=trading+strategies&type=posts"
```

---

## 📦 Webhooks

Subscribe to events via webhooks:

### Supported Events

- `user.created` - New user registered
- `subscription.created` - New subscription
- `subscription.updated` - Subscription changed
- `subscription.cancelled` - Subscription cancelled
- `payment.succeeded` - Payment successful
- `payment.failed` - Payment failed

### Webhook Payload

```json
{
  "event": "subscription.created",
  "timestamp": "2026-02-16T12:00:00Z",
  "data": {
    "id": "sub_123",
    "user_id": "user_456",
    "plan": "premium",
    "status": "active"
  }
}
```

---

## 🛠️ SDKs & Libraries

### Official SDKs

- **TypeScript/JavaScript** - `@revolution-trading-pros/sdk`
- **Python** - `revolution-trading-pros`
- **Rust** - `revolution-api-client`

### Example Usage (TypeScript)

```typescript
import { RevolutionAPI } from '@revolution-trading-pros/sdk';

const api = new RevolutionAPI({
  apiKey: 'your-api-key',
  baseUrl: 'https://revolution-trading-pros-api.fly.dev'
});

const user = await api.users.me();
console.log(user);
```

---

## 📊 OpenAPI/Swagger

Interactive API documentation is available at:

**Production:** https://revolution-trading-pros-api.fly.dev/swagger-ui/  
**Development:** http://localhost:8080/swagger-ui/

---

## 🐛 Error Codes

| Code | Description |
|------|-------------|
| `VALIDATION_ERROR` | Request validation failed |
| `AUTHENTICATION_ERROR` | Invalid or missing authentication |
| `AUTHORIZATION_ERROR` | Insufficient permissions |
| `NOT_FOUND` | Resource not found |
| `CONFLICT` | Resource conflict (e.g., duplicate email) |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `INTERNAL_ERROR` | Server error |

---

## 📞 Support

- **Email:** api-support@revolutiontradingpros.com
- **Documentation:** https://docs.revolutiontradingpros.com
- **Status Page:** https://status.revolutiontradingpros.com

---

## 📝 Changelog

See [CHANGELOG.md](../../CHANGELOG.md) for API version history.

---

**Last Updated:** February 16, 2026  
**API Version:** 1.0.0

