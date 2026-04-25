# Setup Guide - Revolution Trading Pros
**Apple Principal Engineer ICT 7 Grade - January 2026**

Complete setup instructions for all services and infrastructure.

---

## Table of Contents
1. [Bunny.net Video Streaming](#bunnynet-video-streaming)
2. [Cloudflare R2 Storage](#cloudflare-r2-storage)
3. [Stripe Payments](#stripe-payments)
4. [Database & Redis](#database--redis)
5. [Course System](#course-system)
6. [Secrets Configuration](#secrets-configuration)

---

## Bunny.net Video Streaming

### Stream Library (Primary Video Hosting)
**Library:** revolution-trading-courses
**Library ID:** 585929
**API Key:** 3982c5b8-6dea-4c37-b707db888834-cbb6-4a82
**CDN:** vz-5a23b520-193.b-cdn.net

### Storage Zone (Backup/Downloads)
**Zone:** revolution-trading-downloads
**API Key:** a9fb06e7-4976-4ef5-bbf3d2a9f71d-2d1d-4a2d
**Hostname:** ny.storage.bunnycdn.com

### Setup Steps
1. Go to https://bunny.net
2. Navigate to Stream → Libraries
3. Copy Library ID and API Key
4. Add to `.env`:
   ```bash
   BUNNY_STREAM_LIBRARY_ID=585929
   BUNNY_STREAM_API_KEY=your-key
   BUNNY_CDN_URL=https://vz-5a23b520-193.b-cdn.net
   ```

### Webhook Configuration
**Endpoint:** `https://revolution-trading-pros-api.fly.dev/api/webhooks/bunny/video-status`
**Events:** Video encoding complete, upload complete

---

## Cloudflare R2 Storage

### Bucket Configuration
**Bucket:** revolution-trading-media
**Account ID:** 9c72eb0d1b0b7891aca6532fe709cacc
**Public URL:** https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev

### API Credentials
**Access Key ID:** 6875e6c6fc0081a7634455b2e22e2d51
**Secret Access Key:** cfb7f20231faa0fb67823c6208c422a253e8f390a5813719cb0a65dce2bfd9fb

### Setup Steps
1. Go to Cloudflare Dashboard → R2
2. Create bucket: `revolution-trading-media`
3. Generate API token with R2 permissions
4. Add to `.env`:
   ```bash
   R2_ENDPOINT=https://9c72eb0d1b0b7891aca6532fe709cacc.r2.cloudflarestorage.com
   R2_ACCESS_KEY_ID=your-access-key
   R2_SECRET_ACCESS_KEY=your-secret-key
   R2_BUCKET=revolution-trading-media
   R2_PUBLIC_URL=https://pub-xxx.r2.dev
   ```

### CLI Setup (Optional)
```bash
# Install Wrangler
npm install -g wrangler

# Login
wrangler login

# Create R2 bucket
wrangler r2 bucket create revolution-trading-media

# Generate API token
wrangler r2 token create revolution-trading-media-token --read --write
```

---

## Stripe Payments

### Test Mode Credentials
**Secret Key:** `sk_test_xxx` (get from Stripe dashboard)
**Publishable Key:** `pk_test_xxx` (get from Stripe dashboard)
**Webhook Secret:** `whsec_xxx` (get from Stripe dashboard)

### Setup Steps
1. Go to https://dashboard.stripe.com
2. Toggle to Test Mode
3. Get API keys from Developers → API keys
4. Add to `.env`:
   ```bash
   STRIPE_SECRET=sk_test_xxx
   STRIPE_PUBLISHABLE_KEY=pk_test_xxx
   STRIPE_WEBHOOK_SECRET=whsec_xxx
   ```

### Webhook Configuration
**Endpoint:** `https://revolution-trading-pros-api.fly.dev/api/webhooks/stripe`
**Events:**
- `checkout.session.completed`
- `invoice.payment_succeeded`
- `customer.subscription.updated`
- `customer.subscription.deleted`

---

## Database & Redis

### PostgreSQL (Fly.io)
**App:** revolution-db
**Connection String:** `postgres://postgres:PASSWORD@revolution-db.flycast:5432/postgres?sslmode=disable`

#### Get Credentials
```bash
# Connect to database
fly postgres connect -a revolution-db

# Get connection string
fly secrets list -a revolution-trading-pros-api | grep DATABASE_URL
```

### Redis (Upstash)
**URL:** `rediss://default:TOKEN@gorgeous-bullfrog-15191.upstash.io:6379`

#### Get Credentials
1. Go to https://console.upstash.com
2. Select your Redis database
3. Copy connection string
4. Add to `.env`:
   ```bash
   REDIS_URL=rediss://default:xxx@xxx.upstash.io:6379
   ```

---

## Course System

### Database Tables
- `courses_enhanced` - Course catalog
- `course_sections` - Course modules
- `course_lessons` - Individual lessons
- `user_course_enrollments` - Student enrollments
- `user_lesson_progress` - Progress tracking

### Setup Steps
1. Run migrations: `cd api && sqlx migrate run`
2. Seed sample courses (optional):
   ```sql
   INSERT INTO courses_enhanced (title, slug, description, is_published)
   VALUES ('Sample Course', 'sample-course', 'Description', true);
   ```

### Video Integration
- Videos stored in Bunny.net Stream
- `bunny_video_guid` links lessons to videos
- Automatic encoding status updates via webhook

---

## Secrets Configuration

### Backend Secrets (Fly.io)
```bash
# Set individual secrets
fly secrets set DATABASE_URL="postgres://..." -a revolution-trading-pros-api
fly secrets set REDIS_URL="rediss://..." -a revolution-trading-pros-api
fly secrets set JWT_SECRET="your-secret" -a revolution-trading-pros-api

# Import from .env file
fly secrets import < api/.env -a revolution-trading-pros-api
```

### Frontend Secrets (Cloudflare Pages)
1. Go to Cloudflare Dashboard → Pages
2. Select `revolution-trading-pros`
3. Settings → Environment Variables
4. Add production variables:
   - `VITE_API_URL`
   - `VITE_CDN_URL`
   - `VITE_GTM_ID`
   - `PUBLIC_GA4_MEASUREMENT_ID`

### Security Best Practices
1. **Never commit `.env` files**
2. **Rotate secrets every 90 days**
3. **Use different keys for test/production**
4. **Enable MFA on all service accounts**
5. **Audit access logs monthly**

---

## Verification Checklist

- [ ] Bunny.net videos uploading successfully
- [ ] R2 storage accessible from backend
- [ ] Stripe test payments working
- [ ] Database migrations applied
- [ ] Redis cache responding
- [ ] Course system functional
- [ ] All secrets configured in Fly.io
- [ ] Frontend env vars set in Cloudflare

---

**Last Updated:** January 24, 2026
