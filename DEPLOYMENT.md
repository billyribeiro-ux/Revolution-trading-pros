# Revolution Trading Pros - Deployment Guide

## Apple Principal Engineer ICT11+ Cloud Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PRODUCTION ARCHITECTURE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐    ┌─────────────────────────────────────────────────┐     │
│  │   Users     │───▶│        Cloudflare (Global Edge)                │     │
│  └─────────────┘    │  • CDN for static assets                        │     │
│                     │  • WAF protection                               │     │
│                     │  • DDoS mitigation                              │     │
│                     │  • Edge caching                                 │     │
│                     └───────────────────┬─────────────────────────────┘     │
│                                         │                                    │
│                     ┌───────────────────┴─────────────────────────────┐     │
│                     │                                                  │     │
│                     ▼                                                  ▼     │
│    ┌──────────────────────────────┐       ┌──────────────────────────────┐ │
│    │  Cloudflare Pages (Frontend) │       │    Railway (Backend API)     │ │
│    │  • SvelteKit SSR             │◀─────▶│    • Laravel PHP 8.2         │ │
│    │  • Edge rendering            │  API  │    • Redis queues            │ │
│    │  • Static optimization       │       │    • Job workers             │ │
│    └──────────────────────────────┘       └──────────────┬───────────────┘ │
│                                                          │                  │
│                     ┌────────────────────────────────────┼──────────────┐   │
│                     │                                    │              │   │
│                     ▼                                    ▼              ▼   │
│    ┌──────────────────────────┐  ┌─────────────────────────┐  ┌─────────┐  │
│    │   Cloudflare R2          │  │   Neon PostgreSQL       │  │ Upstash │  │
│    │   (Media Storage)        │  │   (Serverless DB)       │  │ (Redis) │  │
│    └──────────────────────────┘  └─────────────────────────┘  └─────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### 1. Prerequisites

```bash
# Install CLIs
npm install -g wrangler @railway/cli

# Login to services
wrangler login
railway login
```

### 2. Environment Setup

```bash
# Frontend
cp frontend/.env.example frontend/.env.local

# Backend
cp backend/.env.example backend/.env
```

### 3. Deploy

```bash
# Frontend (Cloudflare Pages)
cd frontend && npm run build && wrangler pages deploy

# Backend (Railway)
cd backend && railway up
```

---

## Frontend Deployment (Cloudflare Pages)

### Option A: GitHub Integration (Recommended)

1. **Connect Repository**
   - Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
   - Workers & Pages → Create → Connect to Git
   - Select `billyribeiro-ux/Revolution-trading-pros`

2. **Configure Build Settings**
   ```
   Framework preset: SvelteKit
   Build command: npm run build
   Build output directory: .svelte-kit/cloudflare
   Root directory: frontend
   ```

3. **Set Environment Variables**
   - Go to Settings → Environment variables
   - Add all variables from `frontend/.env.production.example`
   - Mark sensitive values as "Encrypted"

4. **Configure Custom Domain**
   - Go to Custom domains → Add custom domain
   - Add `revolutiontradingpros.com`
   - Configure DNS (CNAME to `revolution-trading-pros.pages.dev`)

### Option B: Manual Deployment (Wrangler CLI)

```bash
cd frontend

# Build for Cloudflare
DEPLOY_TARGET=cloudflare npm run build

# Deploy to production
wrangler pages deploy .svelte-kit/cloudflare \
  --project-name=revolution-trading-pros

# Deploy to preview
wrangler pages deploy .svelte-kit/cloudflare \
  --project-name=revolution-trading-pros \
  --branch=preview
```

### Environment Variables (Cloudflare Dashboard)

| Variable | Description | Example |
|----------|-------------|---------|
| `VITE_API_URL` | Backend API URL | `https://revolution-trading-backend.up.railway.app/api` |
| `VITE_SITE_URL` | Production URL | `https://revolutiontradingpros.com` |
| `VITE_GTM_ID` | Google Tag Manager | `GTM-XXXXXXX` |
| `VITE_GTAG_ID` | Google Analytics | `G-XXXXXXXXXX` |
| `PUBLIC_GA4_MEASUREMENT_ID` | GA4 ID | `G-XXXXXXXXXX` |
| `PUBLIC_META_PIXEL_ID` | Facebook Pixel | `123456789` |

---

## Backend Deployment (Railway)

### Option A: GitHub Integration (Recommended)

1. **Create Project**
   - Go to [Railway Dashboard](https://railway.app)
   - New Project → Deploy from GitHub repo
   - Select repository and `backend` directory

2. **Add Services**
   ```
   Backend Service: revolution-trading-backend
   Root Directory: backend
   Build Command: (auto-detected from Dockerfile)
   ```

3. **Add Database (Neon PostgreSQL)**
   - Go to [Neon Console](https://console.neon.tech)
   - Create database: `revolution_trading_prod`
   - Copy connection string

4. **Add Redis (Upstash)**
   - Go to [Upstash Console](https://console.upstash.com)
   - Create Redis database
   - Copy connection URL

5. **Set Environment Variables**
   ```bash
   # Required
   APP_KEY=base64:... (run php artisan key:generate --show)
   APP_ENV=production
   APP_DEBUG=false
   APP_URL=https://revolution-trading-backend.up.railway.app

   # Database (Neon)
   DATABASE_URL=postgresql://user:pass@ep-xxx.neon.tech/neondb?sslmode=require

   # Cache (Upstash)
   REDIS_URL=redis://default:xxx@xxx.upstash.io:6379

   # CORS
   SANCTUM_STATEFUL_DOMAINS=revolutiontradingpros.com,www.revolutiontradingpros.com
   CORS_ALLOWED_ORIGINS=https://revolutiontradingpros.com
   ```

### Option B: Railway CLI

```bash
cd backend

# Link to project
railway link

# Deploy
railway up

# Run migrations
railway run php artisan migrate --force

# Cache config
railway run php artisan config:cache
railway run php artisan route:cache
```

---

## CI/CD Pipeline

### GitHub Actions Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `deploy-cloudflare.yml` | Push to main/develop | Deploy frontend |
| `deploy-railway.yml` | Push to main/develop | Deploy backend |
| `e2e.yml` | PR/Push to main | E2E tests |

### Required Secrets (GitHub)

```
# Cloudflare
CLOUDFLARE_API_TOKEN
CLOUDFLARE_ACCOUNT_ID
CLOUDFLARE_ZONE_ID

# Railway
RAILWAY_TOKEN
RAILWAY_PROJECT_ID

# Optional
LHCI_GITHUB_APP_TOKEN (Lighthouse CI)
```

### Creating Secrets

1. **Cloudflare API Token**
   - Cloudflare Dashboard → My Profile → API Tokens
   - Create Token → Edit Cloudflare Workers
   - Add permissions: `Pages:Edit`, `Zone:Cache Purge`

2. **Railway Token**
   - Railway Dashboard → Account Settings → Tokens
   - Create new token

---

## Cloudflare R2 Setup (Media Storage)

### 1. Create Bucket

```bash
wrangler r2 bucket create revolution-trading-media
```

### 2. Configure CORS

```json
{
  "CORSRules": [
    {
      "AllowedOrigins": ["https://revolutiontradingpros.com"],
      "AllowedMethods": ["GET", "PUT", "POST", "DELETE"],
      "AllowedHeaders": ["*"],
      "MaxAgeSeconds": 3600
    }
  ]
}
```

### 3. Enable Public Access

- Dashboard → R2 → Bucket → Settings → Public access
- Enable with custom domain or `pub-xxx.r2.dev`

### 4. Backend Configuration

```env
FILESYSTEM_DISK=r2
R2_ACCESS_KEY_ID=your-access-key
R2_SECRET_ACCESS_KEY=your-secret-key
R2_BUCKET=revolution-trading-media
R2_ENDPOINT=https://ACCOUNT_ID.r2.cloudflarestorage.com
R2_PUBLIC_URL=https://pub-xxx.r2.dev
```

---

## Custom Domain Configuration

### Cloudflare DNS Setup

```
Type    Name    Content                               Proxy
CNAME   @       revolution-trading-pros.pages.dev    Proxied
CNAME   www     revolution-trading-pros.pages.dev    Proxied
CNAME   api     revolution-trading-backend.up.railway.app   DNS Only
```

### SSL/TLS Configuration

- Cloudflare Dashboard → SSL/TLS → Full (strict)
- Enable HSTS
- Enable automatic HTTPS rewrites

---

## Performance Optimization

### Cloudflare Page Rules

| URL Pattern | Settings |
|-------------|----------|
| `*revolutiontradingpros.com/_app/*` | Cache Level: Cache Everything, Edge TTL: 1 year |
| `*revolutiontradingpros.com/images/*` | Cache Level: Cache Everything, Edge TTL: 1 month |
| `*revolutiontradingpros.com/api/*` | Cache Level: Bypass |

### Cache-Control Headers

```
/_app/*     → public, max-age=31536000, immutable
/images/*   → public, max-age=86400, stale-while-revalidate=604800
/api/*      → no-store, private
```

---

## Monitoring & Observability

### Recommended Tools

| Tool | Purpose | Setup |
|------|---------|-------|
| Cloudflare Analytics | Frontend metrics | Built-in |
| Railway Observability | Backend logs | Built-in |
| Sentry | Error tracking | Add `SENTRY_DSN` env var |
| Upstash QStash | Job scheduling | Webhook triggers |

---

## Rollback Procedures

### Frontend Rollback

```bash
# List deployments
wrangler pages deployments list --project-name=revolution-trading-pros

# Rollback to specific deployment
wrangler pages deployments rollback --project-name=revolution-trading-pros <deployment-id>
```

### Backend Rollback

```bash
# Railway Dashboard → Deployments → Click on previous deployment → Rollback
# Or via CLI:
railway rollback
```

---

## Troubleshooting

### Common Issues

1. **Build fails on Cloudflare**
   - Check Node.js version (use 20)
   - Ensure `DEPLOY_TARGET=cloudflare` is set
   - Check `wrangler.toml` syntax

2. **API connection fails**
   - Verify `VITE_API_URL` is correct
   - Check CORS settings in Laravel
   - Ensure Railway service is running

3. **Database connection issues**
   - Verify `DATABASE_URL` format
   - Check Neon connection limits
   - Ensure SSL is enabled (`?sslmode=require`)

### Health Checks

```bash
# Frontend
curl https://revolutiontradingpros.com

# Backend API
curl https://revolution-trading-backend.up.railway.app/api/health/live

# Database
curl https://revolution-trading-backend.up.railway.app/api/health/ready
```

---

## Security Checklist

- [ ] All secrets stored in environment variables (not in code)
- [ ] HTTPS enforced everywhere
- [ ] CORS configured correctly
- [ ] Rate limiting enabled
- [ ] WAF rules configured
- [ ] CSP headers set
- [ ] HSTS enabled
- [ ] Database backups configured
- [ ] Error messages don't leak sensitive info
