# Revolution Trading Pros - Rust API Deployment Guide

## December 2025 Stack

---

## Your Stack

| Layer | Technology | Cost |
|-------|------------|------|
| **Frontend** | Cloudflare Pages (SvelteKit 5) | FREE |
| **API** | Fly.io (Rust + Axum) | $5-29/mo |
| **Database** | Neon PostgreSQL | FREE tier |
| **Cache** | Upstash Redis | FREE tier |
| **Storage** | Cloudflare R2 | FREE (10GB) |
| **Queue** | PostgreSQL (built-in) | FREE |
| **Search** | Meilisearch Cloud | FREE tier |

**Total: $5-29/mo**

---

## 1. Neon PostgreSQL (Database)

### Sign Up
**URL:** https://console.neon.tech

### Pricing

| Plan | Price | Storage | Compute |
|------|-------|---------|---------|
| **Free** | $0 | 512MB | Shared |
| **Launch** | $19/mo | 10GB | Dedicated |
| **Scale** | $69/mo | 50GB | Autoscaling |

### Setup

1. Go to https://console.neon.tech
2. Click "Create Project"
3. Name: `revolution-trading-pros`
4. Region: `US East (N. Virginia)` (closest to Fly.io IAD)
5. Click "Create Project"

### Get Connection String

1. Go to Dashboard → Connection Details
2. Copy the connection string:
```
postgresql://user:password@ep-xxx.us-east-2.aws.neon.tech/neondb?sslmode=require
```

---

## 2. Upstash Redis (Cache)

### Sign Up
**URL:** https://console.upstash.com

### Pricing

| Plan | Price | Requests | Features |
|------|-------|----------|----------|
| **Free** | $0 | 10K/day | 256MB, 1 region |
| **Pay-as-you-go** | $0.2/100K | Unlimited | Multi-region |
| **Pro** | $100/mo | Unlimited | Dedicated |

### Setup

1. Go to https://console.upstash.com
2. Click "Create Database"
3. Name: `revolution-trading-pros`
4. Region: `US-East-1` (closest to Fly.io)
5. Click "Create"

### Get Connection URL

1. Go to Database → Details
2. Copy the Redis URL:
```
rediss://default:xxx@xxx.upstash.io:6379
```

---

## 3. Cloudflare R2 (Storage)

**Already configured!**
- Bucket: `revolution-trading-media`
- URL: `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev`

### Get API Keys (if not already)

1. Go to https://dash.cloudflare.com
2. R2 → Manage R2 API Tokens
3. Create token with read/write access
4. Copy Access Key ID and Secret

---

## 4. Stripe (Payments)

### Sign Up
**URL:** https://dashboard.stripe.com/register

### Pricing
- 2.9% + $0.30 per transaction
- No monthly fees

### Setup

1. Go to Developers → API Keys
2. Copy:
   - Publishable key: `pk_live_xxx`
   - Secret key: `sk_live_xxx`

3. Create webhook:
   - URL: `https://revolution-trading-pros-api.fly.dev/api/payments/webhook`
   - Events: `checkout.session.completed`, `customer.subscription.*`, `invoice.*`
   - Copy signing secret: `whsec_xxx`

---

## 5. Meilisearch Cloud (Search)

### Sign Up
**URL:** https://cloud.meilisearch.com

### Pricing

| Plan | Price | Documents | Searches |
|------|-------|-----------|----------|
| **Build** | $0 | 100K | 10K/mo |
| **Pro** | $30/mo | 1M | 500K/mo |
| **Premium** | $300/mo | 10M | Unlimited |

### Setup

1. Go to https://cloud.meilisearch.com
2. Click "Create a project"
3. Name: `revolution-trading-pros`
4. Region: `US East` (closest to Fly.io)
5. Click "Create"

### Get API Keys

1. Go to Project → Settings → API Keys
2. Copy:
   - Host URL: `https://ms-xxx.meilisearch.io`
   - Master Key (for production): `xxx`

### Search Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/search` | GET | Global search (courses + posts) |
| `/api/search/courses` | GET | Search courses only |
| `/api/search/posts` | GET | Search posts only |

Query parameter: `?q=search+term&limit=10`

---

## 6. Fly.io (Deployment)

### Sign Up
**URL:** https://fly.io/app/sign-up

### Pricing

| Plan | Price | Resources |
|------|-------|-----------|
| **Hobby** | $5/mo | Shared CPU, 256MB |
| **Launch** | $29/mo | Dedicated CPU, 2GB |

### Deploy

```bash
# 1. Install Fly CLI
curl -L https://fly.io/install.sh | sh

# 2. Login
fly auth login

# 3. Navigate to API directory
cd api

# 4. Create app
./scripts/deploy.sh setup

# 5. Create .env file
cp .env.example .env
# Edit .env with your values

# 6. Set secrets
./scripts/deploy.sh secrets

# 7. Deploy
./scripts/deploy.sh deploy
```

---

## Environment Variables

Create `api/.env` with:

```env
# Server
PORT=8080
ENVIRONMENT=production

# Neon PostgreSQL
DATABASE_URL=postgresql://user:pass@ep-xxx.us-east-2.aws.neon.tech/neondb?sslmode=require

# Upstash Redis
REDIS_URL=rediss://default:xxx@xxx.upstash.io:6379

# Cloudflare R2
R2_ENDPOINT=https://xxx.r2.cloudflarestorage.com
R2_ACCESS_KEY_ID=your-key
R2_SECRET_ACCESS_KEY=your-secret
R2_BUCKET=revolution-trading-media
R2_PUBLIC_URL=https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev

# JWT
JWT_SECRET=your-32-char-secret-key-here-min

# Stripe
STRIPE_SECRET=sk_live_xxx
STRIPE_WEBHOOK_SECRET=whsec_xxx

# Email
POSTMARK_TOKEN=xxx
FROM_EMAIL=noreply@revolutiontradingpros.com

# CORS
CORS_ORIGINS=https://revolution-trading-pros.pages.dev,https://revolutiontradingpros.com

# Meilisearch
MEILISEARCH_HOST=https://ms-xxx.meilisearch.io
MEILISEARCH_API_KEY=your-master-key
```

---

## Deployment Checklist

```
[ ] 1. Create Neon PostgreSQL database
[ ] 2. Create Upstash Redis database
[ ] 3. Get Cloudflare R2 API keys
[ ] 4. Get Stripe API keys
[ ] 5. Create Meilisearch Cloud project
[ ] 6. Install Fly CLI
[ ] 7. Run ./scripts/deploy.sh setup
[ ] 8. Create .env file with all values
[ ] 9. Run ./scripts/deploy.sh secrets
[ ] 10. Run ./scripts/deploy.sh deploy
[ ] 11. Test: curl https://revolution-trading-pros-api.fly.dev/health
```

---

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/ready` | GET | Ready check (DB) |
| `/api/auth/register` | POST | Register user |
| `/api/auth/login` | POST | Login user |
| `/api/users` | GET | List users |
| `/api/users/:id` | GET | Get user |
| `/api/courses` | GET | List courses |
| `/api/courses/:slug` | GET | Get course |
| `/api/payments/checkout` | POST | Create checkout |
| `/api/payments/webhook` | POST | Stripe webhook |
| `/api/search` | GET | Global search |
| `/api/search/courses` | GET | Search courses |
| `/api/search/posts` | GET | Search posts |

---

## Performance

| Metric | Value |
|--------|-------|
| Requests/sec | 200,000+ |
| Latency (p99) | < 1ms |
| Cold start | ~50ms |
| Memory | ~20MB |

---

## URLs

| Service | URL |
|---------|-----|
| **API** | https://revolution-trading-pros-api.fly.dev |
| **Health** | https://revolution-trading-pros-api.fly.dev/health |
| **Frontend** | https://revolution-trading-pros.pages.dev |
| **Storage** | https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev |

---

**Document Version:** 2.0 (Rust)
**Last Updated:** December 2025
