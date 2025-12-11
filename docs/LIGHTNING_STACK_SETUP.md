# ⚡ LIGHTNING STACK SETUP GUIDE

## Revolution Trading Pros - Ultimate Performance Configuration
### ICT 11+ Principal Engineer Implementation

---

## Table of Contents

1. [Overview](#1-overview)
2. [Cost Summary](#2-cost-summary)
3. [Step 1: Cloudflare Account Setup](#3-step-1-cloudflare-account-setup)
4. [Step 2: Cloudflare R2 Storage](#4-step-2-cloudflare-r2-storage)
5. [Step 3: Sharp Image Processing (Already Configured)](#5-step-3-sharp-image-processing-already-configured)
6. [Step 4: Cloudflare Pages (Frontend)](#6-step-4-cloudflare-pages-frontend)
7. [Step 5: Neon PostgreSQL Database](#7-step-5-neon-postgresql-database)
8. [Step 6: Upstash Redis Cache](#8-step-6-upstash-redis-cache)
9. [Step 7: Backend Deployment](#9-step-7-backend-deployment)
10. [Step 8: DNS & Domain Configuration](#10-step-8-dns--domain-configuration)
11. [Step 9: Final Configuration](#11-step-9-final-configuration)
12. [Verification & Testing](#12-verification--testing)
13. [Troubleshooting](#13-troubleshooting)

---

## 1. Overview

The Lightning Stack provides **sub-50ms response times globally** using edge computing and serverless infrastructure.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    ⚡ LIGHTNING STACK                            │
└─────────────────────────────────────────────────────────────────┘

User (anywhere) ──► Cloudflare Edge (300+ locations)
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
   Cloudflare         Sharp Service      Cloudflare
     Pages             (Images)              R2
   (Frontend)      (WebP/AVIF/Blur)      (Storage)
         │                 │                 │
         └─────────────────┴─────────────────┘
                           │
                           ▼
                    Your Laravel API
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
        Upstash Redis              Neon PostgreSQL
       (Edge Caching)            (Serverless DB)
```

### Why Sharp + R2 Instead of Cloudflare Images?

Your project already includes a **high-performance Sharp image processing service** that provides:

| Feature | Sharp (Your System) | Cloudflare Images |
|---------|---------------------|-------------------|
| Processing | At upload time (once) | On-the-fly (every request) |
| Cost | FREE (self-hosted) | $5/month + usage |
| Control | Full control | Limited |
| Formats | WebP, AVIF, PNG, JPG | WebP, AVIF |
| BlurHash | ✅ Built-in | ❌ Not available |
| Responsive | ✅ Pre-generated | ✅ On-demand |
| Speed | Instant (pre-cached) | Fast (edge computed) |

**Recommendation:** Keep Sharp for processing, R2 for storage. This gives you the best of both worlds with zero monthly image processing costs.

### Performance Targets

| Metric | Before | After |
|--------|--------|-------|
| Time to First Byte | 200-500ms | **10-30ms** |
| Page Load Time | 2-4s | **0.5-1s** |
| Image Load | 200-400ms | **15-50ms** |
| API Response | 100-300ms | **20-50ms** |
| Lighthouse Score | 60-80 | **95-100** |

---

## 2. Cost Summary

### Monthly Costs by Usage

| Traffic Level | Monthly Cost |
|---------------|--------------|
| Starting (< 10K visitors) | **$0-5** |
| Growing (10K-50K visitors) | **$15-30** |
| Popular (50K-100K visitors) | **$45-75** |
| High Traffic (100K+ visitors) | **$95-145** |

### Service Breakdown

| Service | Free Tier | Paid Tier |
|---------|-----------|-----------|
| Cloudflare Pages | Unlimited | - |
| Cloudflare R2 | 10GB storage | $0.015/GB |
| Sharp Processing | FREE (included) | - |
| Neon PostgreSQL | 0.5GB | $19/month (10GB) |
| Upstash Redis | 10K req/day | $10/month (unlimited) |

**Note:** By using your existing Sharp service instead of Cloudflare Images, you save **$5/month** while maintaining full control over image processing.

---

## 3. Step 1: Cloudflare Account Setup

### 3.1 Create Cloudflare Account

**Link:** https://dash.cloudflare.com/sign-up

1. Go to https://dash.cloudflare.com/sign-up
2. Enter your email and create a password
3. Verify your email address
4. Complete the account setup

### 3.2 Get Your Account ID

1. Log into Cloudflare Dashboard
2. Your Account ID is in the URL: `https://dash.cloudflare.com/ACCOUNT_ID`
3. Or find it on the Overview page (right sidebar)
4. **Save this ID** - you'll need it for all services

### 3.3 Create API Token

**Link:** https://dash.cloudflare.com/profile/api-tokens

1. Go to **Profile** → **API Tokens**
2. Click **Create Token**
3. Use **Custom Token** template
4. Configure permissions:

```
Permissions:
├── Account > Workers R2 Storage > Edit
├── Account > Workers Scripts > Edit
└── Zone > Zone > Read (for your domain)
```

5. Click **Continue to Summary** → **Create Token**
6. **COPY THE TOKEN NOW** - it won't be shown again!

```
Save as: CLOUDFLARE_API_TOKEN=your-token-here
```

---

## 4. Step 2: Cloudflare R2 Storage

### 4.1 Enable R2

**Link:** https://dash.cloudflare.com/?to=/:account/r2

1. Go to **R2** in the sidebar
2. Click **Get Started** or **Create Bucket**

### 4.2 Create Media Bucket

1. Click **Create Bucket**
2. Name: `revolution-trading-media`
3. Location: **Automatic** (recommended) or choose nearest region
4. Click **Create Bucket**

### 4.3 Create R2 API Token

1. Go to **R2** → **Manage R2 API Tokens**
2. Click **Create API Token**
3. Token name: `revolution-trading-r2`
4. Permissions: **Object Read & Write**
5. Specify bucket: `revolution-trading-media`
6. Click **Create API Token**

**Save these values:**
```
R2_ACCESS_KEY_ID=your-access-key
R2_SECRET_ACCESS_KEY=your-secret-key
R2_ENDPOINT=https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com
R2_BUCKET=revolution-trading-media
```

### 4.4 Enable Public Access (Optional but Recommended)

1. Go to your bucket → **Settings**
2. Under **Public Access**, click **Allow Access**
3. You'll get a public URL like: `https://pub-xxx.r2.dev`

**OR** Set up Custom Domain:
1. Go to bucket → **Settings** → **Custom Domains**
2. Add: `media.revolutiontradingpros.com`
3. Cloudflare will auto-configure DNS

```
R2_PUBLIC_URL=https://media.revolutiontradingpros.com
```

### 4.5 Configure CORS (Required for Uploads)

1. Go to your bucket → **Settings** → **CORS Policy**
2. Add this configuration:

```json
[
  {
    "AllowedOrigins": [
      "https://revolutiontradingpros.com",
      "https://www.revolutiontradingpros.com",
      "https://api.revolutiontradingpros.com"
    ],
    "AllowedMethods": ["GET", "PUT", "POST", "DELETE"],
    "AllowedHeaders": ["*"],
    "MaxAgeSeconds": 3600
  }
]
```

---

## 5. Step 3: Sharp Image Processing (Already Configured)

### 5.1 Overview

Your project includes a **Node.js Sharp image processing service** that automatically:

- Converts images to WebP and AVIF formats
- Generates responsive image sizes (thumbnail, small, medium, large, xlarge)
- Creates BlurHash placeholders for instant loading
- Uploads processed images to R2 storage

### 5.2 Service Location

The Sharp service is located at:
```
backend/services/sharp-image-processor/
```

### 5.3 Configuration

The Sharp service is configured in `backend/config/services.php`:

```php
'sharp' => [
    'enabled' => env('SHARP_SERVICE_ENABLED', true),
    'url' => env('SHARP_SERVICE_URL', 'http://localhost:3001'),
    'timeout' => env('SHARP_SERVICE_TIMEOUT', 60),
],
```

### 5.4 Environment Variables

Add these to your `.env`:

```env
SHARP_SERVICE_ENABLED=true
SHARP_SERVICE_URL=http://localhost:3001
SHARP_SERVICE_TIMEOUT=60
```

### 5.5 Image Sizes Generated

When you upload an image, Sharp automatically generates:

| Size | Dimensions | Use Case |
|------|------------|----------|
| thumbnail | 150x150 | Grid thumbnails |
| small | 320px wide | Mobile views |
| medium | 640px wide | Tablet views |
| large | 1280px wide | Desktop views |
| xlarge | 1920px wide | Full-screen/hero |
| blurhash | 4x3 hash | Placeholder |

### 5.6 Output Formats

Each image is converted to:
- **WebP** - Modern browsers (85% smaller than JPG)
- **AVIF** - Latest browsers (95% smaller than JPG)
- **Original format** - Fallback for older browsers

### 5.7 How It Works

1. **Upload**: User uploads image through admin panel
2. **Process**: Sharp service processes the image
3. **Generate**: Creates all sizes in WebP/AVIF
4. **BlurHash**: Generates placeholder hash
5. **Upload to R2**: Stores all variants in Cloudflare R2
6. **Cache**: URLs cached for instant access

```
Upload Flow:
┌──────────┐     ┌──────────────┐     ┌────────────┐
│  Admin   │ ──► │ Sharp Service │ ──► │  R2 Bucket │
│  Panel   │     │  (Process)    │     │  (Store)   │
└──────────┘     └──────────────┘     └────────────┘
                        │
                        ▼
                 ┌──────────────┐
                 │   Outputs:    │
                 │ - WebP sizes  │
                 │ - AVIF sizes  │
                 │ - BlurHash    │
                 └──────────────┘
```

---

## 6. Step 4: Cloudflare Pages (Frontend)

### 6.1 Connect GitHub Repository

**Link:** https://dash.cloudflare.com/?to=/:account/pages

1. Go to **Pages** in sidebar
2. Click **Create a Project** → **Connect to Git**
3. Authorize Cloudflare to access your GitHub
4. Select your repository: `Revolution-trading-pros`

### 6.2 Configure Build Settings

```
Framework preset: SvelteKit
Build command: npm run build:cloudflare
Build output directory: .svelte-kit/cloudflare
Root directory: frontend
```

### 6.3 Set Environment Variables

Click **Environment Variables** and add:

| Variable | Value |
|----------|-------|
| `PUBLIC_API_URL` | `https://api.revolutiontradingpros.com` |
| `PUBLIC_APP_NAME` | `Revolution Trading Pros` |
| `DEPLOY_TARGET` | `cloudflare` |

### 6.4 Deploy

1. Click **Save and Deploy**
2. Wait for build to complete (~2-3 minutes)
3. Your site is live at: `revolution-trading-pros.pages.dev`

### 6.5 Add Custom Domain

1. Go to your project → **Custom Domains**
2. Add: `revolutiontradingpros.com`
3. Add: `www.revolutiontradingpros.com`
4. Cloudflare will auto-configure DNS and SSL

---

## 7. Step 5: Neon PostgreSQL Database

### 7.1 Create Neon Account

**Link:** https://console.neon.tech

1. Go to https://console.neon.tech
2. Sign up with GitHub, Google, or email
3. Complete verification

### 7.2 Create Project

1. Click **Create Project**
2. Project name: `revolution-trading-pros`
3. Region: Choose closest to your users (e.g., `US East (Ohio)`)
4. PostgreSQL version: **16** (latest)
5. Click **Create Project**

### 7.3 Get Connection Details

1. On the dashboard, find **Connection Details**
2. Select **Connection String** tab
3. Copy the connection string

**Your connection string looks like:**
```
postgres://username:password@ep-xxx-yyy.us-east-2.aws.neon.tech/neondb?sslmode=require
```

**Save these values:**
```
NEON_DATABASE_URL=postgres://username:password@ep-xxx.us-east-2.aws.neon.tech/neondb?sslmode=require
NEON_DB_HOST=ep-xxx.us-east-2.aws.neon.tech
NEON_DB_DATABASE=neondb
NEON_DB_USERNAME=your-username
NEON_DB_PASSWORD=your-password
```

### 7.4 Choose Plan

| Plan | Price | Storage | Features |
|------|-------|---------|----------|
| Free | $0 | 0.5GB | 1 project, auto-suspend |
| Launch | $19/mo | 10GB | No suspend, branches |
| Scale | $69/mo | 50GB | High availability |

**Recommendation:** Start with **Free**, upgrade to **Launch** when you have paying users.

**Upgrade Link:** https://console.neon.tech/app/billing

---

## 8. Step 6: Upstash Redis Cache

### 8.1 Create Upstash Account

**Link:** https://console.upstash.com

1. Go to https://console.upstash.com
2. Sign up with GitHub, Google, or email

### 8.2 Create Redis Database

1. Click **Create Database**
2. Name: `revolution-trading-cache`
3. Type: **Regional** or **Global**
   - **Regional:** Lower cost, good for single region
   - **Global:** Best latency worldwide (recommended)
4. Primary Region: Choose closest to your API server
5. Click **Create**

### 8.3 Get Connection Details

1. On database dashboard, find **Connect to your database**
2. Copy the credentials

**Save these values:**
```
UPSTASH_REDIS_URL=rediss://default:xxx@xxx.upstash.io:6379
UPSTASH_REDIS_HOST=xxx.upstash.io
UPSTASH_REDIS_PASSWORD=your-password
UPSTASH_REDIS_PORT=6379

# REST API (for edge functions)
UPSTASH_REDIS_REST_URL=https://xxx.upstash.io
UPSTASH_REDIS_REST_TOKEN=your-rest-token
```

### 8.4 Choose Plan

| Plan | Price | Requests |
|------|-------|----------|
| Free | $0 | 10,000/day |
| Pay-as-you-go | $0.2/100K | Unlimited |
| Pro | $10/mo | Unlimited |

**Recommendation:** Start with **Free**, upgrade to **Pay-as-you-go** or **Pro** for production.

**Upgrade Link:** https://console.upstash.com/billing

---

## 9. Step 7: Backend Deployment

### 9.1 Update Environment Variables

Copy `.env.lightning-stack.example` to `.env` and fill in all values:

```bash
cd backend
cp .env.lightning-stack.example .env
```

Edit `.env` with your credentials from steps 4-8.

### 9.2 Recommended Backend Hosting Options

| Provider | Price | Best For |
|----------|-------|----------|
| **Railway** | $5/mo | Simplest deployment |
| **Render** | $7/mo | Good free tier |
| **DigitalOcean App Platform** | $12/mo | Reliable, scalable |
| **Laravel Forge + DO** | $12/mo | Most control |
| **Fly.io** | $0-10/mo | Edge deployment |

### 9.3 Deploy to Railway (Recommended)

**Link:** https://railway.app

1. Go to https://railway.app
2. Click **New Project** → **Deploy from GitHub**
3. Select your repository
4. Set root directory: `backend`
5. Add environment variables from your `.env`
6. Deploy!

### 9.4 Deploy Sharp Service

The Sharp service needs to run alongside your Laravel backend:

**Option A: Same Server (Docker Compose)**
```yaml
services:
  laravel:
    # ... your Laravel config

  sharp:
    build: ./services/sharp-image-processor
    ports:
      - "3001:3001"
    environment:
      - R2_ACCESS_KEY_ID=${R2_ACCESS_KEY_ID}
      - R2_SECRET_ACCESS_KEY=${R2_SECRET_ACCESS_KEY}
      - R2_BUCKET=${R2_BUCKET}
      - R2_ENDPOINT=${R2_ENDPOINT}
```

**Option B: Separate Service (Railway)**
1. Create a new Railway project for Sharp service
2. Point it to `backend/services/sharp-image-processor`
3. Add R2 environment variables
4. Update `SHARP_SERVICE_URL` in your Laravel `.env`

### 9.5 Run Migrations

After deployment, run:

```bash
php artisan migrate --force
php artisan db:seed --force  # If needed
php artisan config:cache
php artisan route:cache
php artisan view:cache
```

---

## 10. Step 8: DNS & Domain Configuration

### 10.1 Add Your Domain to Cloudflare

**Link:** https://dash.cloudflare.com/?to=/:account/add-site

1. Click **Add a Site**
2. Enter: `revolutiontradingpros.com`
3. Select **Free** plan (or upgrade for more features)
4. Cloudflare will scan existing DNS records
5. Update nameservers at your registrar to Cloudflare's

### 10.2 Configure DNS Records

| Type | Name | Content | Proxy |
|------|------|---------|-------|
| CNAME | @ | revolution-trading-pros.pages.dev | ✅ |
| CNAME | www | revolution-trading-pros.pages.dev | ✅ |
| CNAME | api | your-backend-url.railway.app | ✅ |
| CNAME | media | pub-xxx.r2.dev | ✅ |

### 10.3 Enable Security Features

1. Go to **SSL/TLS** → Set to **Full (strict)**
2. Go to **Security** → Enable **Bot Fight Mode**
3. Go to **Speed** → Enable **Auto Minify** for JS, CSS, HTML
4. Go to **Caching** → Set **Browser Cache TTL** to **1 year**

---

## 11. Step 9: Final Configuration

### 11.1 Update Backend .env

```env
# Application
APP_URL=https://api.revolutiontradingpros.com

# Database
DB_CONNECTION=neon

# Cache
CACHE_STORE=upstash

# Storage
FILESYSTEM_DISK=r2
MEDIA_DISK=r2

# Enable services
NEON_ENABLED=true
UPSTASH_ENABLED=true
CLOUDFLARE_R2_ENABLED=true

# Sharp Service
SHARP_SERVICE_ENABLED=true
SHARP_SERVICE_URL=http://sharp:3001  # or your deployed URL
```

### 11.2 Update Frontend Environment

Create `frontend/.env.production`:

```env
PUBLIC_API_URL=https://api.revolutiontradingpros.com
PUBLIC_APP_NAME=Revolution Trading Pros
PUBLIC_MEDIA_URL=https://media.revolutiontradingpros.com
```

### 11.3 Clear All Caches

```bash
# On your backend server
php artisan cache:clear
php artisan config:clear
php artisan config:cache
php artisan route:cache
php artisan view:cache
```

### 11.4 Redeploy Frontend

```bash
cd frontend
npm run deploy:cloudflare
```

---

## 12. Verification & Testing

### 12.1 Test Checklist

- [ ] Frontend loads at https://revolutiontradingpros.com
- [ ] API responds at https://api.revolutiontradingpros.com/api/posts
- [ ] Images load from R2 (check Network tab for `media.` domain)
- [ ] WebP/AVIF images served to modern browsers
- [ ] Blog posts display correctly
- [ ] Admin panel works
- [ ] Image uploads work (Sharp + R2)
- [ ] SSL certificates are valid (green padlock)

### 12.2 Verify Image Processing

1. Upload a test image in admin panel
2. Check R2 bucket for generated variants:
   - `images/{id}/thumbnail.webp`
   - `images/{id}/small.webp`
   - `images/{id}/medium.webp`
   - `images/{id}/large.webp`
   - `images/{id}/xlarge.webp`
   - `images/{id}/thumbnail.avif`
   - etc.

3. Verify BlurHash is stored in database

### 12.3 Performance Test

Run a Lighthouse test:

1. Open Chrome DevTools (F12)
2. Go to **Lighthouse** tab
3. Check **Performance**, **Accessibility**, **Best Practices**, **SEO**
4. Click **Analyze page load**

**Target Scores:**
- Performance: 90+
- Accessibility: 90+
- Best Practices: 90+
- SEO: 90+

### 12.4 Speed Test Tools

- **GTmetrix:** https://gtmetrix.com
- **PageSpeed Insights:** https://pagespeed.web.dev
- **WebPageTest:** https://webpagetest.org

---

## 13. Troubleshooting

### Images Not Loading

1. Check R2 bucket public access is enabled
2. Verify R2_PUBLIC_URL is correct
3. Check CORS settings in R2 bucket
4. Verify Sharp service is running

### Sharp Service Not Processing

1. Check Sharp service logs: `docker logs sharp`
2. Verify R2 credentials in Sharp environment
3. Test Sharp endpoint: `curl http://localhost:3001/health`
4. Check Laravel can reach Sharp: verify `SHARP_SERVICE_URL`

### Database Connection Failed

1. Verify NEON_DATABASE_URL is correct
2. Ensure `?sslmode=require` is in the URL
3. Check if IP needs whitelisting (Neon usually doesn't require this)

### Redis Connection Failed

1. Verify UPSTASH_REDIS_URL includes `rediss://` (with double s)
2. Check password is correct
3. Try REST API as fallback

### Frontend Build Failed

1. Run `npm install` in frontend directory
2. Check Node.js version (18+ required)
3. Verify DEPLOY_TARGET is set

### CORS Errors

1. Update CORS_ALLOWED_ORIGINS in backend .env
2. Add your domain to SANCTUM_STATEFUL_DOMAINS
3. Clear config cache: `php artisan config:cache`

---

## Quick Links Reference

| Service | Dashboard | Docs |
|---------|-----------|------|
| Cloudflare | https://dash.cloudflare.com | https://developers.cloudflare.com |
| Cloudflare R2 | https://dash.cloudflare.com/?to=/:account/r2 | https://developers.cloudflare.com/r2 |
| Cloudflare Pages | https://dash.cloudflare.com/?to=/:account/pages | https://developers.cloudflare.com/pages |
| Neon | https://console.neon.tech | https://neon.tech/docs |
| Upstash | https://console.upstash.com | https://docs.upstash.com |

---

## Architecture Summary

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        LIGHTNING STACK v3.0                             │
│                    (Sharp + R2 Configuration)                           │
└─────────────────────────────────────────────────────────────────────────┘

                              ┌─────────────────┐
                              │    User (Web)   │
                              └────────┬────────┘
                                       │
                                       ▼
                       ┌───────────────────────────────┐
                       │    Cloudflare Edge Network    │
                       │     (300+ global locations)   │
                       └───────────────┬───────────────┘
                                       │
              ┌────────────────────────┼────────────────────────┐
              │                        │                        │
              ▼                        ▼                        ▼
    ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
    │ Cloudflare Pages│     │  Cloudflare R2  │     │  Laravel API    │
    │   (Frontend)    │     │    (Media)      │     │   (Backend)     │
    │                 │     │                 │     │                 │
    │ • SvelteKit SSR │     │ • Zero egress   │     │ • REST API      │
    │ • Edge rendered │     │ • Global CDN    │     │ • Auth/Admin    │
    │ • <50ms TTFB    │     │ • WebP/AVIF     │     │ • Business logic│
    └─────────────────┘     └─────────────────┘     └────────┬────────┘
                                       ▲                     │
                                       │                     │
                            ┌──────────┴──────────┐         │
                            │   Sharp Service     │         │
                            │ (Image Processing)  │         │
                            │                     │◄────────┘
                            │ • WebP conversion   │
                            │ • AVIF conversion   │
                            │ • Responsive sizes  │
                            │ • BlurHash          │
                            └─────────────────────┘
                                                            │
                      ┌─────────────────────────────────────┤
                      │                                     │
                      ▼                                     ▼
            ┌─────────────────┐                   ┌─────────────────┐
            │  Upstash Redis  │                   │ Neon PostgreSQL │
            │    (Cache)      │                   │   (Database)    │
            │                 │                   │                 │
            │ • Session store │                   │ • Serverless    │
            │ • API cache     │                   │ • Auto-scaling  │
            │ • <1ms reads    │                   │ • Branching     │
            └─────────────────┘                   └─────────────────┘


Cost Breakdown (Monthly):
──────────────────────────
Cloudflare Pages:    FREE (unlimited)
Cloudflare R2:       FREE (up to 10GB) or $0.015/GB
Sharp Processing:    FREE (self-hosted)
Neon PostgreSQL:     FREE (0.5GB) or $19/mo (10GB)
Upstash Redis:       FREE (10K/day) or $10/mo (unlimited)
──────────────────────────
TOTAL:               $0-29/month
```

---

## Support

If you encounter issues:

1. Check the [Troubleshooting](#13-troubleshooting) section
2. Review service-specific documentation
3. Contact service support directly

---

**Document Version:** 3.1.0
**Last Updated:** December 2024
**Author:** Revolution Trading Pros Engineering Team

---

## Changelog

### v3.1.0 (Current)
- Removed Cloudflare Images (replaced with existing Sharp service)
- Updated architecture diagram for Sharp + R2 workflow
- Added Sharp service deployment instructions
- Reduced monthly costs by $5/month
- Added image processing verification steps

### v3.0.0
- Initial Lightning Stack implementation
- Cloudflare Pages, R2, Neon, Upstash integration
