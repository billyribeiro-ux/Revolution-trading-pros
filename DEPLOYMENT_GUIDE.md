# Revolution Trading Pros - Deployment Guide

## December 2025 Stack Configuration

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Fly.io - Backend Hosting](#1-flyio---backend-hosting)
3. [Postmark - Transactional Email](#2-postmark---transactional-email)
4. [Stripe - Payment Processing](#3-stripe---payment-processing)
5. [Sentry - Error Tracking](#4-sentry---error-tracking)
6. [Already Configured Services](#5-already-configured-services)
7. [Deployment Steps](#6-deployment-steps)
8. [Environment Variables Reference](#7-environment-variables-reference)

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    REVOLUTION TRADING PROS                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────┐         ┌──────────────────────────────┐  │
│  │   FRONTEND       │         │   BACKEND                    │  │
│  │   Cloudflare     │ ──────► │   Fly.io                     │  │
│  │   Pages          │         │   Laravel 12 + FrankenPHP    │  │
│  │   (FREE)         │         │   ($5-29/mo)                 │  │
│  └──────────────────┘         └──────────────────────────────┘  │
│           │                              │                       │
│           │                              ├── Postgres (included) │
│           │                              ├── Redis (included)    │
│           │                              │                       │
│           ▼                              ▼                       │
│  ┌──────────────────┐         ┌──────────────────────────────┐  │
│  │   STORAGE        │         │   SERVICES                   │  │
│  │   Cloudflare R2  │         │   • Postmark (Email)         │  │
│  │   (FREE 10GB)    │         │   • Stripe (Payments)        │  │
│  └──────────────────┘         │   • Sentry (Monitoring)      │  │
│                               └──────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Monthly Cost Estimate

| Service | Free Tier | Starter | Growth |
|---------|-----------|---------|--------|
| Fly.io | - | $5/mo | $29/mo |
| Postmark | 100 emails | $15/mo (10K) | $50/mo (50K) |
| Stripe | - | 2.9% + $0.30/tx | 2.9% + $0.30/tx |
| Sentry | 5K errors | $26/mo | $80/mo |
| Cloudflare Pages | FREE | FREE | FREE |
| Cloudflare R2 | FREE (10GB) | $0.015/GB | $0.015/GB |
| **TOTAL** | **~$0** | **~$46/mo** | **~$159/mo** |

---

## 1. Fly.io - Backend Hosting

### What It Does
Hosts your Laravel backend with FrankenPHP for sub-millisecond response times. Includes managed Postgres database and Redis cache.

### Sign Up
**URL:** https://fly.io/app/sign-up

### Pricing Tiers

| Plan | Price | Resources | Best For |
|------|-------|-----------|----------|
| **Hobby** | $5/mo | 1 shared CPU, 256MB RAM, 1GB storage | Development/Testing |
| **Launch** | $29/mo | 1 dedicated CPU, 2GB RAM, 10GB storage | Production |
| **Scale** | $79/mo | 2 dedicated CPUs, 4GB RAM, 20GB storage | High Traffic |

### Included with All Plans
- **Postgres Database** - Managed, automatic backups
- **Redis Cache** - For sessions, cache, queues
- **SSL Certificates** - Automatic HTTPS
- **Global Deployment** - 30+ regions
- **Volumes** - Persistent storage

### Setup Instructions

#### Step 1: Install Fly CLI
```bash
# macOS
brew install flyctl

# Linux
curl -L https://fly.io/install.sh | sh

# Windows
powershell -Command "iwr https://fly.io/install.ps1 -useb | iex"
```

#### Step 2: Login
```bash
fly auth login
```

#### Step 3: Run Setup Script
```bash
cd backend
chmod +x scripts/fly-deploy.sh
./scripts/fly-deploy.sh setup
```

This creates:
- Fly.io app
- Postgres database (1GB free tier)
- Redis instance (256MB)
- Persistent volume for storage

#### Step 4: Get Database URL
After setup, note these values:
```bash
fly postgres connect -a revolution-trading-pros-db
# Connection string will be shown
```

---

## 2. Postmark - Transactional Email

### What It Does
Sends transactional emails (password resets, notifications, receipts) with 99%+ deliverability.

### Sign Up
**URL:** https://postmarkapp.com/sign-up

### Pricing Tiers

| Plan | Price | Emails/Month | Features |
|------|-------|--------------|----------|
| **Free** | $0 | 100 | Testing only, Postmark branding |
| **Starter** | $15/mo | 10,000 | No branding, dedicated IP optional |
| **Standard** | $50/mo | 50,000 | Priority support |
| **Plus** | $115/mo | 125,000 | Dedicated IP included |
| **Enterprise** | Custom | Unlimited | SLA, dedicated support |

### Setup Instructions

#### Step 1: Create Account
1. Go to https://postmarkapp.com/sign-up
2. Verify your email address
3. Complete account setup

#### Step 2: Create Server
1. Click "Create Server"
2. Name it: `Revolution Trading Pros`
3. Select "Live" for production (or "Test" for development)

#### Step 3: Get API Token
1. Go to Server → API Tokens
2. Click "Create Token"
3. Copy the token (starts with `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`)

#### Step 4: Verify Sender Domain
1. Go to Sender Signatures → Add Domain
2. Add your domain: `revolutiontradingpros.com`
3. Add DNS records as shown:
   - DKIM record (TXT)
   - Return-Path record (CNAME)
4. Click "Verify"

#### Step 5: Set Environment Variable
```bash
fly secrets set POSTMARK_TOKEN="your-token-here"
```

### Configuration (Already Done)
File: `backend/config/mail.php`
```php
'default' => env('MAIL_MAILER', 'postmark'),
```

File: `backend/config/services.php`
```php
'postmark' => [
    'token' => env('POSTMARK_TOKEN'),
],
```

---

## 3. Stripe - Payment Processing

### What It Does
Processes credit card payments, subscriptions, and invoicing for your trading platform.

### Sign Up
**URL:** https://dashboard.stripe.com/register

### Pricing

| Type | Fee | Notes |
|------|-----|-------|
| **Card Payments** | 2.9% + $0.30 | Per successful transaction |
| **International Cards** | +1.5% | Additional fee |
| **Currency Conversion** | +1% | If converting currencies |
| **Disputes** | $15 | Per chargeback (refunded if won) |
| **Payouts** | FREE | To your bank account |

**No monthly fees. No setup fees. No minimums.**

### Setup Instructions

#### Step 1: Create Account
1. Go to https://dashboard.stripe.com/register
2. Enter email and create password
3. Verify email address

#### Step 2: Complete Business Profile
1. Go to Settings → Business Settings
2. Add:
   - Business name
   - Business address
   - Tax ID (EIN/SSN)
   - Bank account for payouts

#### Step 3: Get API Keys
1. Go to Developers → API Keys
2. Copy these keys:

| Key Type | Format | Usage |
|----------|--------|-------|
| Publishable Key | `pk_live_xxx` | Frontend (public) |
| Secret Key | `sk_live_xxx` | Backend (secret) |

**For Testing:** Use `pk_test_xxx` and `sk_test_xxx` keys first.

#### Step 4: Create Webhook
1. Go to Developers → Webhooks
2. Click "Add endpoint"
3. URL: `https://revolution-trading-pros.fly.dev/api/webhooks/stripe`
4. Select events:
   - `payment_intent.succeeded`
   - `payment_intent.payment_failed`
   - `customer.subscription.created`
   - `customer.subscription.updated`
   - `customer.subscription.deleted`
   - `invoice.paid`
   - `invoice.payment_failed`
5. Click "Add endpoint"
6. Copy the Webhook Signing Secret (`whsec_xxx`)

#### Step 5: Set Environment Variables
```bash
fly secrets set \
  STRIPE_KEY="pk_live_xxxxxxxxxxxx" \
  STRIPE_SECRET="sk_live_xxxxxxxxxxxx" \
  STRIPE_WEBHOOK_SECRET="whsec_xxxxxxxxxxxx"
```

### Configuration (Already Done)
File: `backend/config/services.php`
```php
'stripe' => [
    'key' => env('STRIPE_KEY'),
    'secret' => env('STRIPE_SECRET'),
    'webhook_secret' => env('STRIPE_WEBHOOK_SECRET'),
],
```

---

## 4. Sentry - Error Tracking

### What It Does
Captures errors, exceptions, and performance issues in real-time. Shows stack traces, user context, and trends.

### Sign Up
**URL:** https://sentry.io/signup/

### Pricing Tiers

| Plan | Price | Errors/Month | Features |
|------|-------|--------------|----------|
| **Developer** | FREE | 5,000 | 1 user, 30-day retention |
| **Team** | $26/mo | 50,000 | Unlimited users, 90-day retention |
| **Business** | $80/mo | 100,000 | SSO, custom retention |
| **Enterprise** | Custom | Unlimited | SLA, dedicated support |

### Setup Instructions

#### Step 1: Create Account
1. Go to https://sentry.io/signup/
2. Sign up with email or GitHub
3. Verify email

#### Step 2: Create Project
1. Click "Create Project"
2. Select platform: **Laravel**
3. Name: `revolution-trading-pros`
4. Click "Create Project"

#### Step 3: Get DSN
1. Go to Settings → Projects → revolution-trading-pros
2. Click "Client Keys (DSN)"
3. Copy the DSN (format: `https://xxx@xxx.ingest.sentry.io/xxx`)

#### Step 4: Set Environment Variable
```bash
fly secrets set SENTRY_LARAVEL_DSN="https://xxx@xxx.ingest.sentry.io/xxx"
```

#### Step 5: Configure Alerts (Optional)
1. Go to Alerts → Create Alert
2. Set up alerts for:
   - New errors
   - Error spike (>10 in 1 hour)
   - Performance degradation

### Configuration (Already Done)
File: `backend/config/sentry.php`
```php
return [
    'dsn' => env('SENTRY_LARAVEL_DSN'),
    'traces_sample_rate' => 0.1,
    'profiles_sample_rate' => 0.1,
];
```

---

## 5. Already Configured Services

These services are already set up and working:

### Cloudflare Pages (Frontend)
- **URL:** https://revolution-trading-pros.pages.dev
- **Status:** Deployed and working
- **Cost:** FREE

### Cloudflare R2 (Storage)
- **URL:** https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev
- **Status:** Created and working
- **Cost:** FREE (first 10GB)

---

## 6. Deployment Steps

### Complete Deployment Checklist

```
[ ] 1. Sign up for Fly.io
[ ] 2. Sign up for Postmark
[ ] 3. Sign up for Stripe
[ ] 4. Sign up for Sentry
[ ] 5. Install Fly CLI
[ ] 6. Run setup script
[ ] 7. Set all secrets
[ ] 8. Deploy
[ ] 9. Verify health endpoint
[ ] 10. Test email sending
[ ] 11. Test payment processing
[ ] 12. Verify error tracking
```

### Step-by-Step Commands

```bash
# 1. Install Fly CLI
curl -L https://fly.io/install.sh | sh

# 2. Login to Fly.io
fly auth login

# 3. Navigate to backend
cd backend

# 4. Run setup (creates Postgres, Redis, volumes)
./scripts/fly-deploy.sh setup

# 5. Set all secrets at once
fly secrets set \
  APP_KEY="base64:$(openssl rand -base64 32)" \
  POSTMARK_TOKEN="your-postmark-token" \
  STRIPE_KEY="pk_live_xxx" \
  STRIPE_SECRET="sk_live_xxx" \
  STRIPE_WEBHOOK_SECRET="whsec_xxx" \
  SENTRY_LARAVEL_DSN="https://xxx@xxx.ingest.sentry.io/xxx" \
  CLOUDFLARE_R2_ACCESS_KEY_ID="your-r2-key" \
  CLOUDFLARE_R2_SECRET_ACCESS_KEY="your-r2-secret" \
  CLOUDFLARE_R2_ENDPOINT="https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com" \
  CLOUDFLARE_R2_BUCKET="revolution-trading-media"

# 6. Deploy
./scripts/fly-deploy.sh deploy

# 7. Verify deployment
curl https://revolution-trading-pros.fly.dev/health
```

---

## 7. Environment Variables Reference

### Required Variables

| Variable | Service | How to Get |
|----------|---------|------------|
| `APP_KEY` | Laravel | `openssl rand -base64 32` |
| `POSTMARK_TOKEN` | Postmark | Server → API Tokens |
| `STRIPE_KEY` | Stripe | Developers → API Keys |
| `STRIPE_SECRET` | Stripe | Developers → API Keys |
| `STRIPE_WEBHOOK_SECRET` | Stripe | Developers → Webhooks |
| `SENTRY_LARAVEL_DSN` | Sentry | Settings → Client Keys |

### R2 Storage Variables (Already Have)

| Variable | Value |
|----------|-------|
| `CLOUDFLARE_R2_ACCESS_KEY_ID` | From R2 dashboard |
| `CLOUDFLARE_R2_SECRET_ACCESS_KEY` | From R2 dashboard |
| `CLOUDFLARE_R2_BUCKET` | `revolution-trading-media` |
| `CLOUDFLARE_R2_URL` | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` |

### Auto-Set by Fly.io

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | Postgres connection string |
| `REDIS_URL` | Redis connection string |

---

## Support Links

| Service | Documentation | Support |
|---------|---------------|---------|
| Fly.io | https://fly.io/docs | https://community.fly.io |
| Postmark | https://postmarkapp.com/developer | support@postmarkapp.com |
| Stripe | https://stripe.com/docs | https://support.stripe.com |
| Sentry | https://docs.sentry.io | https://sentry.io/support |

---

**Document Version:** 1.0
**Last Updated:** December 2025
**Stack:** Laravel 12 + FrankenPHP + Octane
