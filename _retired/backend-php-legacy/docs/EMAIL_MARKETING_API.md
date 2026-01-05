# Email Marketing API Documentation

## Overview

This document provides comprehensive documentation for the Revolution Trading Pros Email Marketing API. The system is an enterprise-grade email marketing platform comparable to ActiveCampaign, Klaviyo, Mailchimp, and HubSpot.

## Table of Contents

1. [Authentication](#authentication)
2. [Campaign Management](#campaign-management)
3. [Template Management](#template-management)
4. [Subscriber Management](#subscriber-management)
5. [Newsletter Subscription](#newsletter-subscription)
6. [Automation & Sequences](#automation--sequences)
7. [Segmentation](#segmentation)
8. [Analytics](#analytics)
9. [AI Features](#ai-features)
10. [Webhooks](#webhooks)
11. [Rate Limits](#rate-limits)
12. [Error Handling](#error-handling)

---

## Authentication

All API endpoints (except public newsletter routes) require authentication via Laravel Sanctum.

### Headers

```http
Authorization: Bearer {your_api_token}
Content-Type: application/json
Accept: application/json
```

### Admin Routes

Admin routes require the `admin` or `super-admin` role and use the prefix `/api/admin/`.

---

## Campaign Management

### List Campaigns

```http
GET /api/admin/email/campaigns
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | Filter by status: `draft`, `scheduled`, `sending`, `sent`, `all` |
| `search` | string | Search by name or subject |
| `from_date` | date | Filter from date (YYYY-MM-DD) |
| `to_date` | date | Filter to date (YYYY-MM-DD) |
| `per_page` | integer | Results per page (max 100, default 20) |

**Response:**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "Welcome Campaign",
      "subject": "Welcome to Revolution Trading!",
      "status": "sent",
      "sent_count": 1500,
      "opened_count": 450,
      "clicked_count": 120,
      "template": {
        "id": 1,
        "name": "Welcome Template"
      }
    }
  ],
  "meta": {
    "current_page": 1,
    "last_page": 5,
    "per_page": 20,
    "total": 100
  }
}
```

### Get Campaign

```http
GET /api/admin/email/campaigns/{id}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Welcome Campaign",
    "subject": "Welcome to Revolution Trading!",
    "from_name": "Revolution Trading",
    "from_email": "hello@example.com",
    "status": "sent",
    "template_id": 1,
    "segment_id": null,
    "total_recipients": 1500,
    "sent_count": 1500,
    "opened_count": 450,
    "clicked_count": 120,
    "bounced_count": 5,
    "unsubscribed_count": 3,
    "open_rate": 30.0,
    "click_rate": 8.0,
    "bounce_rate": 0.33,
    "sent_at": "2024-01-15T10:00:00Z",
    "created_at": "2024-01-14T15:30:00Z"
  }
}
```

### Create Campaign

```http
POST /api/admin/email/campaigns
```

**Request Body:**

```json
{
  "name": "New Product Launch",
  "subject": "Introducing Our New Trading Indicator!",
  "template_id": 1,
  "segment_id": 5,
  "from_name": "Revolution Trading",
  "from_email": "hello@example.com",
  "reply_to": "support@example.com",
  "scheduled_at": "2024-02-01T09:00:00Z",
  "ab_test_config": {
    "enabled": true,
    "subject_b": "Check Out Our Latest Indicator!",
    "split_percentage": 20
  }
}
```

### Send Campaign

```http
POST /api/admin/email/campaigns/{id}/send
```

### Schedule Campaign

```http
POST /api/admin/email/campaigns/{id}/schedule
```

**Request Body:**

```json
{
  "scheduled_at": "2024-02-01T09:00:00Z"
}
```

### Cancel Campaign

```http
POST /api/admin/email/campaigns/{id}/cancel
```

### Duplicate Campaign

```http
POST /api/admin/email/campaigns/{id}/duplicate
```

### Get Campaign Analytics

```http
GET /api/admin/email/campaigns/{id}/analytics
```

**Response:**

```json
{
  "success": true,
  "data": {
    "campaign": {
      "id": 1,
      "name": "Welcome Campaign",
      "sent_count": 1500,
      "opened_count": 450,
      "clicked_count": 120,
      "open_rate": 30.0,
      "click_rate": 8.0
    },
    "hourly_stats": [...],
    "device_stats": [
      {"device_type": "desktop", "count": 280},
      {"device_type": "mobile", "count": 150},
      {"device_type": "tablet", "count": 20}
    ],
    "top_links": [
      {"link_url": "https://example.com/product", "clicks": 85}
    ],
    "geo_stats": [
      {"country": "US", "count": 300},
      {"country": "GB", "count": 80}
    ]
  }
}
```

### Send Test Email

```http
POST /api/admin/email/campaigns/{id}/test
```

**Request Body:**

```json
{
  "email": "test@example.com"
}
```

---

## Template Management

### List Templates

```http
GET /api/admin/email/templates
```

### Get Template

```http
GET /api/admin/email/templates/{id}
```

### Create Template

```http
POST /api/admin/email/templates
```

**Request Body:**

```json
{
  "name": "Monthly Newsletter",
  "subject": "Your Monthly Trading Insights",
  "body_html": "<html>...</html>",
  "body_text": "Plain text version...",
  "category": "newsletter",
  "is_public": false,
  "auto_inline_css": true,
  "responsive_settings": {
    "mobile_friendly": true,
    "dark_mode_support": true
  }
}
```

### Update Template

```http
PUT /api/admin/email/templates/{id}
```

### Delete Template

```http
DELETE /api/admin/email/templates/{id}
```

### Preview Template

```http
POST /api/admin/email/templates/{id}/preview
```

---

## Newsletter Subscription

These endpoints are public and do not require authentication.

### Subscribe

```http
POST /api/newsletter/subscribe
```

**Request Body:**

```json
{
  "email": "subscriber@example.com",
  "name": "John Doe",
  "consent": true,
  "data_processing_consent": true,
  "topics": ["trading", "indicators"],
  "source": "website"
}
```

**Response:**

```json
{
  "success": true,
  "message": "Please check your email to confirm your subscription.",
  "requires_verification": true
}
```

### Confirm Subscription

```http
GET /api/newsletter/confirm?token={verification_token}
```

### Unsubscribe

```http
GET /api/newsletter/unsubscribe?token={unsubscribe_token}
```

---

## Subscriber Management (Admin)

### List Subscribers

```http
GET /api/admin/email/subscribers
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | `active`, `pending`, `unsubscribed`, `bounced`, `complained` |
| `search` | string | Search by email or name |
| `segment` | string | Filter by segment |
| `engagement` | string | `high`, `medium`, `low` |
| `per_page` | integer | Results per page |

### Get Subscriber

```http
GET /api/admin/email/subscribers/{id}
```

### Update Subscriber

```http
PUT /api/admin/email/subscribers/{id}
```

### Add Tags

```http
POST /api/admin/email/subscribers/{id}/tags
```

**Request Body:**

```json
{
  "tags": ["vip", "active-trader"]
}
```

### Remove Tags

```http
DELETE /api/admin/email/subscribers/{id}/tags
```

---

## Segmentation

### List Segments

```http
GET /api/admin/email/segments
```

### Create Segment

```http
POST /api/admin/email/segments
```

**Request Body:**

```json
{
  "name": "High Value Customers",
  "description": "Customers with engagement score > 70",
  "conditions": [
    {
      "field": "engagement_score",
      "operator": "greater_than",
      "value": 70
    },
    {
      "field": "status",
      "operator": "equals",
      "value": "active"
    }
  ],
  "match_type": "all"
}
```

### Get Segment Subscribers

```http
GET /api/admin/email/segments/{id}/subscribers
```

---

## Analytics

### Dashboard Stats

```http
GET /api/admin/email/campaigns/stats
```

**Response:**

```json
{
  "success": true,
  "data": {
    "total_campaigns": 150,
    "total_sent": 250000,
    "total_opened": 75000,
    "total_clicked": 15000,
    "avg_open_rate": 30.0,
    "avg_click_rate": 6.0,
    "avg_bounce_rate": 0.5,
    "drafts": 5,
    "scheduled": 3,
    "sending": 1,
    "sent": 141,
    "recent": {
      "campaigns": 15,
      "sent": 25000,
      "opened": 8500,
      "clicked": 2100
    }
  }
}
```

### Deliverability Report

```http
GET /api/admin/email/deliverability
```

**Response:**

```json
{
  "success": true,
  "data": {
    "domain": "example.com",
    "spf_valid": true,
    "dkim_valid": true,
    "dmarc_valid": true,
    "reputation_score": 95,
    "inbox_placement_rate": 98.5,
    "spam_score": 2,
    "recommendations": []
  }
}
```

---

## AI Features

### Score Email Content

```http
POST /api/admin/email/ai/score
```

**Request Body:**

```json
{
  "subject": "Your Trading Update",
  "html_content": "<html>...</html>",
  "text_content": "Plain text..."
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "overall_score": 85,
    "grade": "B",
    "details": {
      "subject": {
        "score": 90,
        "length": 21,
        "has_personalization": false,
        "issues": ["Consider adding personalization"]
      },
      "spam": {
        "score": 5,
        "risk_level": "low",
        "triggers": []
      },
      "readability": {
        "score": 75,
        "grade_level": "8th grade"
      },
      "mobile": {
        "score": 90,
        "issues": []
      }
    },
    "recommendations": [...]
  }
}
```

### Optimize Subject Line

```http
POST /api/admin/email/ai/optimize-subject
```

**Request Body:**

```json
{
  "subject": "Check out our new indicator"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "suggestions": [
      {
        "subject": "{{first_name}}, check out our new indicator",
        "reason": "Personalization increases open rates by 26%",
        "expected_lift": "+26%"
      },
      {
        "subject": "Want to check out our new indicator?",
        "reason": "Questions create curiosity",
        "expected_lift": "+10%"
      }
    ]
  }
}
```

### Predict Send Time

```http
GET /api/admin/email/ai/predict-send-time/{subscriber_id}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "recommended_hour": 10,
    "recommended_day": 2,
    "timezone": "America/New_York",
    "local_time": "10:00",
    "confidence": 85,
    "expected_open_rate": 35.5
  }
}
```

---

## Webhooks

### Register Webhook

Webhooks can be configured to receive real-time notifications for email events.

**Available Events:**

| Event | Description |
|-------|-------------|
| `campaign.started` | Campaign sending has begun |
| `campaign.progress` | Campaign sending progress (10%, 25%, 50%, 75%, 90%, 100%) |
| `campaign.completed` | Campaign sending finished |
| `campaign.failed` | Campaign sending failed |
| `email.sent` | Individual email sent |
| `email.opened` | Email was opened |
| `email.clicked` | Link in email was clicked |
| `email.bounced` | Email bounced |
| `email.complained` | Spam complaint received |

**Webhook Payload:**

```json
{
  "event": "campaign.completed",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "campaign_id": 1,
    "campaign_name": "Welcome Campaign",
    "stats": {
      "sent": 1500,
      "opened": 450,
      "clicked": 120,
      "open_rate": 30.0,
      "click_rate": 8.0
    },
    "completed_at": "2024-01-15T10:30:00Z"
  }
}
```

**Webhook Headers:**

```http
X-Webhook-Event: campaign.completed
X-Webhook-Timestamp: 1705315800
X-Webhook-Signature: sha256=...
X-Webhook-ID: 550e8400-e29b-41d4-a716-446655440000
```

---

## Rate Limits

The API implements tiered rate limiting:

| Endpoint Type | Limit |
|--------------|-------|
| Default | 100 requests/minute |
| Authentication | 10 requests/minute |
| Newsletter | 5 requests/minute |
| Email Operations | 30 requests/minute |
| Admin | 200 requests/minute |
| Daily Global | 10,000 requests/day per IP |

**Rate Limit Headers:**

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1705315860
X-RateLimit-Window: 60
```

---

## Error Handling

### Error Response Format

```json
{
  "success": false,
  "error": "validation_error",
  "message": "The given data was invalid.",
  "errors": {
    "email": ["The email field is required."]
  }
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 422 | Validation Error |
| 429 | Too Many Requests |
| 500 | Server Error |

---

## GDPR Compliance

The API supports full GDPR compliance:

- **Double Opt-In**: All newsletter subscriptions use double opt-in by default
- **Consent Tracking**: All consent is logged with IP, timestamp, and method
- **Data Export**: Subscribers can request their data export
- **Right to Erasure**: Soft deletes with data anonymization
- **Unsubscribe**: One-click unsubscribe support in all emails

---

## SDK Examples

### JavaScript/TypeScript

```typescript
import { emailApi } from '$lib/api/email';

// Get campaigns
const campaigns = await emailApi.getCampaigns({ status: 'sent' });

// Send campaign
await emailApi.sendCampaign('campaign-id');

// Score content
const score = await emailApi.scoreContent({
  subject: 'Your Subject',
  html_content: '<html>...</html>'
});
```

### PHP (Laravel)

```php
use App\Services\Email\EmailCampaignService;
use App\Services\Email\AIEmailService;

// Send campaign
$campaignService = app(EmailCampaignService::class);
$campaignService->send($campaignId);

// Score content
$aiService = app(AIEmailService::class);
$score = $aiService->scoreContent($subject, $html, $text);
```

---

## Support

For API support, please contact:
- Email: api-support@revolutiontrading.com
- Documentation: https://docs.revolutiontrading.com/email-api
