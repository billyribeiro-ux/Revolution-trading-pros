# Revolution Trading Pros - API Keys Investigation Report

**Date:** December 29, 2025
**Purpose:** Complete documentation of all API keys required for website functionality

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [Required API Keys by Service](#required-api-keys-by-service)
4. [Environment Configuration Files](#environment-configuration-files)
5. [Setup Instructions by Service](#setup-instructions-by-service)
6. [Priority Classification](#priority-classification)
7. [Security Recommendations](#security-recommendations)

---

## Executive Summary

The Revolution Trading Pros website requires **20+ environment variables** across multiple services to function fully. These are distributed across three main components:

| Component | Location | Primary Services |
|-----------|----------|------------------|
| Backend API | `/api/` | Database, Cache, Storage, Payments, Email, Search |
| Frontend | `/frontend/` | API URLs, Analytics, Marketing Pixels |
| Image Service | `/image-service/` | Cloud Storage |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           FRONTEND (SvelteKit)                          │
│                        Cloudflare Pages Deployment                      │
│                                                                         │
│  Required Keys:                                                         │
│  • VITE_API_URL (API connection)                                        │
│  • PUBLIC_GA4_MEASUREMENT_ID (analytics)                                │
│  • PUBLIC_META_PIXEL_ID (marketing)                                     │
└──────────────────────────────┬──────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          BACKEND API (Rust/Axum)                        │
│                           Fly.io Deployment                             │
│                                                                         │
│  Required Keys:                                                         │
│  • DATABASE_URL (Neon PostgreSQL)                                       │
│  • REDIS_URL (Upstash Redis)                                            │
│  • STRIPE_SECRET + STRIPE_WEBHOOK_SECRET (Payments)                     │
│  • JWT_SECRET (Authentication)                                          │
│  • POSTMARK_TOKEN (Email)                                               │
│  • MEILISEARCH_HOST + MEILISEARCH_API_KEY (Search)                      │
│  • R2_* keys (Storage)                                                  │
└─────────────┬───────────────────────────────────────────┬───────────────┘
              │                                           │
              ▼                                           ▼
┌─────────────────────────┐                 ┌─────────────────────────────┐
│    Neon PostgreSQL      │                 │      Upstash Redis          │
│    (Database)           │                 │      (Cache/Sessions)       │
└─────────────────────────┘                 └─────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                        IMAGE SERVICE (Node.js)                          │
│                                                                         │
│  Required Keys:                                                         │
│  • R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY                               │
│  • R2_ENDPOINT, R2_BUCKET                                               │
└──────────────────────────────┬──────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         Cloudflare R2 Storage                           │
│                        (Media/Assets Storage)                           │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Required API Keys by Service

### 1. 🗄️ DATABASE - Neon PostgreSQL (CRITICAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `DATABASE_URL` | `postgresql://user:password@ep-xxx.us-east-2.aws.neon.tech/neondb?sslmode=require` | [Neon Console](https://console.neon.tech) |

**Used in:** `/api/.env`

**Setup Steps:**
1. Create account at [neon.tech](https://neon.tech)
2. Create a new project
3. Copy the connection string from dashboard
4. Replace placeholder in `DATABASE_URL`

---

### 2. ⚡ CACHE - Upstash Redis (CRITICAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `REDIS_URL` | `rediss://default:xxx@xxx.upstash.io:6379` | [Upstash Console](https://console.upstash.com) |

**Used in:** `/api/.env`, `/image-service/.env` (optional)

**Setup Steps:**
1. Create account at [upstash.com](https://upstash.com)
2. Create a new Redis database
3. Copy the Redis URL (use TLS-enabled `rediss://` URL)
4. Paste into `REDIS_URL`

---

### 3. 📦 STORAGE - Cloudflare R2 (CRITICAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `R2_ENDPOINT` | `https://ACCOUNT_ID.r2.cloudflarestorage.com` | Cloudflare Dashboard > R2 |
| `R2_ACCESS_KEY_ID` | `your-access-key-id` | R2 > Manage R2 API Tokens |
| `R2_SECRET_ACCESS_KEY` | `your-secret-access-key` | R2 > Manage R2 API Tokens |
| `R2_BUCKET` | `revolution-trading-media` | Your bucket name |
| `R2_PUBLIC_URL` | `https://pub-xxx.r2.dev` | R2 Bucket Settings > Public Access |

**Used in:** `/api/.env`, `/image-service/.env`

**Setup Steps:**
1. Log into [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Navigate to R2 Object Storage
3. Create a bucket named `revolution-trading-media`
4. Go to "Manage R2 API Tokens" → Create API Token
5. Grant read/write permissions to your bucket
6. Copy the Access Key ID and Secret Access Key
7. Enable public access and copy the public URL

---

### 4. 💳 PAYMENTS - Stripe (CRITICAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `STRIPE_SECRET` | `sk_test_xxx` (test) or `sk_live_xxx` (production) | [Stripe Dashboard](https://dashboard.stripe.com/apikeys) |
| `STRIPE_WEBHOOK_SECRET` | `whsec_xxx` | Stripe Webhooks Configuration |

**Used in:** `/api/.env`

**Setup Steps:**
1. Create account at [stripe.com](https://stripe.com)
2. Go to Developers → API Keys
3. Copy the **Secret Key** (starts with `sk_test_` or `sk_live_`)
4. Go to Developers → Webhooks
5. Add endpoint: `https://your-api-domain.fly.dev/webhooks/stripe`
6. Select events: `checkout.session.completed`, `invoice.paid`, `customer.subscription.*`
7. Copy the **Signing Secret** (starts with `whsec_`)

---

### 5. 🔐 AUTHENTICATION - JWT (CRITICAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `JWT_SECRET` | `your-super-secret-jwt-key-min-32-chars` | Generate yourself |
| `JWT_EXPIRES_IN` | `24` | Configure as needed (hours) |

**Used in:** `/api/.env`

**Setup Steps:**
1. Generate a secure random string (minimum 32 characters)
2. Use command: `openssl rand -base64 32`
3. Never share or commit this value

---

### 6. 📧 EMAIL - Postmark (REQUIRED for email features)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `POSTMARK_TOKEN` | `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` | [Postmark App](https://postmarkapp.com) |
| `FROM_EMAIL` | `noreply@revolutiontradingpros.com` | Your verified domain |

**Used in:** `/api/.env`

**Setup Steps:**
1. Create account at [postmarkapp.com](https://postmarkapp.com)
2. Create a new server
3. Go to API Tokens tab
4. Copy the Server API Token
5. Verify your sending domain in Sender Signatures

---

### 7. 🔍 SEARCH - Meilisearch (REQUIRED for search features)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `MEILISEARCH_HOST` | `https://ms-xxx.meilisearch.io` | [Meilisearch Cloud](https://cloud.meilisearch.com) |
| `MEILISEARCH_API_KEY` | `your-master-api-key` | Meilisearch Dashboard |

**Used in:** `/api/.env`

**Setup Steps:**
1. Create account at [cloud.meilisearch.com](https://cloud.meilisearch.com)
2. Create a new project
3. Copy the Host URL and API Key from dashboard

---

### 8. 📊 ANALYTICS - Google Analytics 4 (OPTIONAL)

| Variable | Example Format | Where to Get |
|----------|---------------|--------------|
| `PUBLIC_GA4_MEASUREMENT_ID` | `G-XXXXXXXXXX` | [Google Analytics](https://analytics.google.com) |
| `VITE_GTM_ID` | `GTM-XXXXXXX` | [Google Tag Manager](https://tagmanager.google.com) |
| `VITE_GTAG_ID` | `G-XXXXXXXXXX` | Google Analytics |

**Used in:** `/frontend/.env`

**Setup Steps:**
1. Go to [analytics.google.com](https://analytics.google.com)
2. Create a new GA4 property
3. Copy the Measurement ID (format: G-XXXXXXXXXX)
4. (Optional) Set up Google Tag Manager for advanced tracking

---

### 9. 📱 MARKETING PIXELS (OPTIONAL)

| Variable | Platform | Where to Get |
|----------|----------|--------------|
| `PUBLIC_META_PIXEL_ID` | Facebook/Instagram | [Meta Events Manager](https://business.facebook.com/events_manager) |
| `PUBLIC_TIKTOK_PIXEL_ID` | TikTok | [TikTok Ads Manager](https://ads.tiktok.com) |
| `PUBLIC_TWITTER_PIXEL_ID` | Twitter/X | [Twitter Ads](https://ads.twitter.com) |
| `PUBLIC_LINKEDIN_PARTNER_ID` | LinkedIn | [LinkedIn Campaign Manager](https://www.linkedin.com/campaignmanager) |
| `PUBLIC_PINTEREST_TAG_ID` | Pinterest | [Pinterest Business](https://ads.pinterest.com) |
| `PUBLIC_REDDIT_PIXEL_ID` | Reddit | [Reddit Ads](https://ads.reddit.com) |

**Used in:** `/frontend/.env`

---

## Environment Configuration Files

### Backend API (`/api/.env`)

```bash
# Copy from /api/.env.example and fill in:
PORT=8080
ENVIRONMENT=development

# Database (Neon PostgreSQL)
DATABASE_URL=postgresql://user:password@ep-xxx.us-east-2.aws.neon.tech/neondb?sslmode=require

# Cache (Upstash Redis)
REDIS_URL=rediss://default:xxx@xxx.upstash.io:6379

# Storage (Cloudflare R2)
R2_ENDPOINT=https://xxx.r2.cloudflarestorage.com
R2_ACCESS_KEY_ID=your-access-key
R2_SECRET_ACCESS_KEY=your-secret-key
R2_BUCKET=revolution-trading-media
R2_PUBLIC_URL=https://pub-xxx.r2.dev

# Authentication
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
JWT_EXPIRES_IN=24

# Payments (Stripe)
STRIPE_SECRET=sk_test_xxx
STRIPE_WEBHOOK_SECRET=whsec_xxx

# Email (Postmark)
POSTMARK_TOKEN=xxx
FROM_EMAIL=noreply@example.com

# Search (Meilisearch)
MEILISEARCH_HOST=https://ms-xxx.meilisearch.io
MEILISEARCH_API_KEY=your-meilisearch-master-key

# CORS
CORS_ORIGINS=https://revolution-trading-pros.pages.dev,https://revolutiontradingpros.com
```

### Frontend (`/frontend/.env.local`)

```bash
# Copy from /frontend/.env.example and fill in:

# API Configuration (REQUIRED)
VITE_API_URL=https://revolution-trading-pros-api.fly.dev/api
VITE_API_BASE_URL=https://revolution-trading-pros-api.fly.dev/api
VITE_WS_URL=wss://revolution-trading-pros-api.fly.dev
VITE_CDN_URL=https://pub-xxx.r2.dev

# Site Metadata
VITE_SITE_URL=https://revolution-trading-pros.pages.dev
VITE_SITE_NAME=Revolution Trading Pros

# Analytics (OPTIONAL)
PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX
VITE_GTM_ID=GTM-XXXXXXX

# Marketing Pixels (OPTIONAL)
PUBLIC_META_PIXEL_ID=
PUBLIC_TIKTOK_PIXEL_ID=
PUBLIC_TWITTER_PIXEL_ID=
PUBLIC_LINKEDIN_PARTNER_ID=
```

### Image Service (`/image-service/.env`)

```bash
# Copy from /image-service/.env.example and fill in:
PORT=3001

# Cloudflare R2 (same as backend API)
R2_ACCESS_KEY_ID=your_access_key_id
R2_SECRET_ACCESS_KEY=your_secret_access_key
R2_BUCKET=revolution-trading-media
R2_ENDPOINT=https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com
R2_PUBLIC_URL=https://pub-YOUR_HASH.r2.dev

# Optional
REDIS_URL=rediss://default:xxx@xxx.upstash.io:6379
PREGENERATE_VARIANTS=false
```

---

## Priority Classification

### 🔴 CRITICAL (Website Cannot Function Without)

| # | Variable | Service | Component |
|---|----------|---------|-----------|
| 1 | `DATABASE_URL` | Neon PostgreSQL | Backend API |
| 2 | `REDIS_URL` | Upstash Redis | Backend API |
| 3 | `JWT_SECRET` | Authentication | Backend API |
| 4 | `STRIPE_SECRET` | Stripe Payments | Backend API |
| 5 | `STRIPE_WEBHOOK_SECRET` | Stripe Webhooks | Backend API |
| 6 | `R2_ACCESS_KEY_ID` | Cloudflare R2 | Backend API, Image Service |
| 7 | `R2_SECRET_ACCESS_KEY` | Cloudflare R2 | Backend API, Image Service |
| 8 | `R2_ENDPOINT` | Cloudflare R2 | Backend API, Image Service |
| 9 | `VITE_API_URL` | API Connection | Frontend |

### 🟡 HIGHLY RECOMMENDED (Core Features)

| # | Variable | Service | Feature |
|---|----------|---------|---------|
| 1 | `POSTMARK_TOKEN` | Postmark | Email notifications |
| 2 | `MEILISEARCH_HOST` | Meilisearch | Search functionality |
| 3 | `MEILISEARCH_API_KEY` | Meilisearch | Search functionality |
| 4 | `R2_PUBLIC_URL` | Cloudflare R2 | Media CDN delivery |
| 5 | `VITE_CDN_URL` | CDN | Asset loading |

### 🟢 OPTIONAL (Analytics & Marketing)

| # | Variable | Service | Feature |
|---|----------|---------|---------|
| 1 | `PUBLIC_GA4_MEASUREMENT_ID` | Google Analytics | Traffic analytics |
| 2 | `VITE_GTM_ID` | Google Tag Manager | Tag management |
| 3 | `PUBLIC_META_PIXEL_ID` | Facebook Pixel | Meta ads tracking |
| 4 | `PUBLIC_TIKTOK_PIXEL_ID` | TikTok Pixel | TikTok ads tracking |
| 5 | `PUBLIC_TWITTER_PIXEL_ID` | Twitter Pixel | Twitter ads tracking |
| 6 | `PUBLIC_LINKEDIN_PARTNER_ID` | LinkedIn | LinkedIn ads tracking |
| 7 | `PUBLIC_PINTEREST_TAG_ID` | Pinterest | Pinterest ads tracking |
| 8 | `PUBLIC_REDDIT_PIXEL_ID` | Reddit | Reddit ads tracking |

---

## Security Recommendations

### ✅ DO

1. **Use environment variables** - Never hardcode secrets in source code
2. **Use `.env.example` files** - Template files are safe to commit
3. **Rotate keys regularly** - Especially production keys
4. **Use test keys for development** - Stripe provides test mode keys
5. **Limit key permissions** - R2 tokens should only access necessary buckets
6. **Set up webhook verification** - Validate Stripe webhook signatures

### ❌ DON'T

1. **Never commit `.env` files** - They are in `.gitignore` for a reason
2. **Never share secrets in chat/email** - Use secure password managers
3. **Never use production keys in development** - Use test/sandbox keys
4. **Never expose secret keys to frontend** - Only `PUBLIC_*` and `VITE_*` vars

### 🔒 Key Management Locations

| Environment | Platform | How to Set |
|-------------|----------|------------|
| Development | Local machine | `.env` files |
| Production API | Fly.io | `fly secrets set VAR=value` |
| Production Frontend | Cloudflare Pages | Dashboard > Settings > Environment Variables |
| CI/CD | GitHub Actions | Settings > Secrets and variables > Actions |

---

## Quick Setup Checklist

```
Backend API (/api/.env):
[ ] DATABASE_URL - Neon PostgreSQL connection string
[ ] REDIS_URL - Upstash Redis connection string
[ ] JWT_SECRET - Generate: openssl rand -base64 32
[ ] STRIPE_SECRET - From Stripe Dashboard
[ ] STRIPE_WEBHOOK_SECRET - From Stripe Webhooks
[ ] R2_ENDPOINT - Cloudflare R2 endpoint
[ ] R2_ACCESS_KEY_ID - Cloudflare R2 token
[ ] R2_SECRET_ACCESS_KEY - Cloudflare R2 token
[ ] POSTMARK_TOKEN - From Postmark dashboard
[ ] MEILISEARCH_HOST - From Meilisearch Cloud
[ ] MEILISEARCH_API_KEY - From Meilisearch Cloud

Frontend (/frontend/.env.local):
[ ] VITE_API_URL - Your API endpoint URL
[ ] VITE_CDN_URL - Your R2 public URL
[ ] PUBLIC_GA4_MEASUREMENT_ID - Google Analytics (optional)

Image Service (/image-service/.env):
[ ] R2_* variables - Same as backend API
```

---

## Summary

| Category | Count | Status |
|----------|-------|--------|
| Critical Keys | 9 | Must configure before launch |
| Recommended Keys | 5 | Configure for full features |
| Optional Keys | 8+ | Configure for marketing/analytics |
| **Total** | **22+** | Variables to configure |

For production deployment, ensure all **CRITICAL** keys are configured in:
- **Fly.io Secrets** (for Backend API)
- **Cloudflare Pages Environment Variables** (for Frontend)
- **Container Environment** (for Image Service)

---

*Report generated by automated investigation - December 2025*
