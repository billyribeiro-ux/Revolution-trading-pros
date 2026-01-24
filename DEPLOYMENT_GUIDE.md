# Deployment Guide - Revolution Trading Pros
**Apple Principal Engineer ICT 7 Grade - January 2026**

Complete deployment and configuration status for production infrastructure.

---

## Production Infrastructure

### Backend API (Fly.io)
- **URL:** https://revolution-trading-pros-api.fly.dev
- **Health Check:** https://revolution-trading-pros-api.fly.dev/health
- **Region:** iad (US East)
- **Machine:** 1GB RAM, shared CPU
- **Status:** ✅ Operational

### Frontend (Cloudflare Pages)
- **URL:** https://revolution-trading-pros.pages.dev
- **Build:** SvelteKit + Tailwind CSS v4
- **Deployment:** Auto-deploy on push to `main`
- **Status:** ✅ Operational

### Database (Fly.io PostgreSQL)
- **App:** revolution-db
- **Database:** postgres
- **Tables:** 72 tables
- **Status:** ✅ Operational

### Cache (Upstash Redis)
- **Instance:** gorgeous-bullfrog-15191
- **Region:** Global
- **Status:** ✅ Operational

---

## Deployment Workflow

### Backend Deployment (Fly.io)

#### Manual Deploy
```bash
cd api
fly deploy --strategy immediate
```

#### Auto Deploy (GitHub Actions)
- **Trigger:** Push to `main` branch with changes in `api/**`
- **Workflow:** `.github/workflows/deploy-fly.yml`
- **Health Check:** Verifies `/health` endpoint returns 200

#### Deployment Strategies
```bash
# Immediate (fastest, all machines at once)
fly deploy --strategy immediate

# Rolling (default, one at a time)
fly deploy --strategy rolling

# Canary (safest, test one machine first)
fly deploy --strategy canary
```

### Frontend Deployment (Cloudflare Pages)

#### Auto Deploy
- **Trigger:** Push to `main` branch
- **Build Command:** `pnpm run build`
- **Output Directory:** `.svelte-kit/cloudflare`
- **Framework:** SvelteKit

#### Manual Deploy
```bash
cd frontend
pnpm run build
wrangler pages deploy .svelte-kit/cloudflare
```

---

## Configuration Status

### ✅ Services Configured

#### Bunny.net (Video Streaming)
- Stream Library: 585929
- CDN: vz-5a23b520-193.b-cdn.net
- Storage Zone: revolution-trading-downloads
- Webhook: Configured

#### Cloudflare R2 (File Storage)
- Bucket: revolution-trading-media
- Public URL: https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev
- CORS: Enabled for frontend domain

#### Stripe (Payments)
- Mode: Test
- Webhook: Configured
- Products: Membership plans synced

#### Meilisearch (Search)
- Host: https://ms-275da497c3a5-36675.nyc.meilisearch.io
- Indexes: courses, indicators, posts
- Status: Active

#### Postmark (Email)
- From: noreply@revolution-trading-pros.pages.dev
- Status: Placeholder (not configured)

---

## Environment Variables

### Backend (Fly.io Secrets)
```bash
# View all secrets
fly secrets list -a revolution-trading-pros-api

# Set secret
fly secrets set KEY=value -a revolution-trading-pros-api

# Import from file
fly secrets import < api/.env -a revolution-trading-pros-api
```

**Required Secrets:**
- DATABASE_URL
- REDIS_URL
- JWT_SECRET
- STRIPE_SECRET
- BUNNY_STREAM_API_KEY
- R2_ACCESS_KEY_ID
- R2_SECRET_ACCESS_KEY
- MEILISEARCH_API_KEY

### Frontend (Cloudflare Pages)
**Set in:** Cloudflare Dashboard → Pages → Settings → Environment Variables

**Required Variables:**
- VITE_API_URL
- VITE_CDN_URL
- VITE_GTM_ID
- PUBLIC_GA4_MEASUREMENT_ID

---

## Database Migrations

### Automatic Migrations
Migrations run automatically on API startup via `sqlx::migrate!()`.

### Manual Migrations
```bash
cd api
sqlx migrate run
```

### Migration Status
```bash
# Check applied migrations
fly ssh console -a revolution-trading-pros-api
psql $DATABASE_URL -c "SELECT * FROM _sqlx_migrations ORDER BY version;"
```

### Current Schema
- **Version:** 21 migrations applied
- **Tables:** 72 tables
- **Key Tables:** users, trading_rooms, room_weekly_videos, courses_enhanced

---

## Health Checks

### Backend Health Check
```bash
curl https://revolution-trading-pros-api.fly.dev/health
# Response: {"status":"healthy","version":"0.1.0","environment":"development"}
```

### Detailed Health Check
```bash
curl https://revolution-trading-pros-api.fly.dev/health/detailed
# Returns: DB status, Redis status, Storage status
```

### Fly.io Health Check Configuration
**File:** `api/fly.toml`
```toml
[[http_service.checks]]
  grace_period = "10s"
  interval = "15s"
  method = "GET"
  path = "/health"
  timeout = "5s"
```

---

## Monitoring & Logs

### Backend Logs
```bash
# Real-time logs
fly logs -a revolution-trading-pros-api

# Last 100 lines
fly logs -a revolution-trading-pros-api --no-tail | tail -100

# Filter by level
fly logs -a revolution-trading-pros-api | grep ERROR
```

### Frontend Logs
- **Location:** Cloudflare Dashboard → Pages → Deployments → Logs
- **Real-time:** Available during build and deployment

### Database Logs
```bash
fly logs -a revolution-db
```

---

## Scaling

### Backend Scaling
```bash
# Scale to 2 machines
fly scale count 2 -a revolution-trading-pros-api

# Scale memory
fly scale memory 2048 -a revolution-trading-pros-api

# Add region
fly regions add lhr -a revolution-trading-pros-api
```

### Current Configuration
- **Machines:** 1
- **Memory:** 1GB
- **CPU:** Shared
- **Regions:** iad (US East)

---

## Rollback Procedures

### Backend Rollback
```bash
# List recent deployments
fly releases -a revolution-trading-pros-api

# Rollback to previous version
fly releases rollback -a revolution-trading-pros-api
```

### Frontend Rollback
1. Go to Cloudflare Dashboard → Pages
2. Select deployment
3. Click "Rollback to this deployment"

### Database Rollback
```bash
# Revert last migration
cd api
sqlx migrate revert
```

---

## Security Checklist

- [x] HTTPS enforced on all endpoints
- [x] CORS configured for production domains only
- [x] JWT secrets rotated and secure
- [x] Database credentials in Fly.io secrets
- [x] API rate limiting enabled (Redis)
- [x] SQL injection protection (SQLx compile-time checks)
- [x] XSS protection (Content-Security-Policy headers)
- [x] Secrets never committed to git

---

## Troubleshooting

### API Not Responding
```bash
# Check machine status
fly status -a revolution-trading-pros-api

# Restart machine
fly machine restart MACHINE_ID -a revolution-trading-pros-api

# Check logs for errors
fly logs -a revolution-trading-pros-api | grep ERROR
```

### Database Connection Issues
```bash
# Check database status
fly status -a revolution-db

# Restart database
fly machine restart MACHINE_ID -a revolution-db

# Test connection
fly ssh console -a revolution-trading-pros-api
echo $DATABASE_URL
```

### Frontend Build Failures
1. Check Cloudflare Pages build logs
2. Verify `package.json` scripts
3. Check for missing environment variables
4. Test build locally: `pnpm run build`

---

## Deployment Checklist

### Pre-Deployment
- [ ] All tests passing
- [ ] Migrations tested locally
- [ ] Environment variables updated
- [ ] Secrets configured
- [ ] Health checks verified

### Post-Deployment
- [ ] Health check returns 200
- [ ] Frontend loads correctly
- [ ] Database migrations applied
- [ ] API endpoints responding
- [ ] No errors in logs
- [ ] Video upload working
- [ ] Payments processing

---

**Last Updated:** January 24, 2026
**Deployment Status:** ✅ Production Ready
