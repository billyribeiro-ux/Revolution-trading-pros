# Media Stack Setup Guide

## Step-by-Step Signup Instructions

This guide walks you through setting up the Sharp + Cloudflare R2 + SQLite media stack.

---

## 1. Cloudflare R2 Setup (Object Storage)

### Sign Up

1. **Go to:** https://dash.cloudflare.com/sign-up
2. **Create account** with email (or use existing Cloudflare account)
3. **Verify email** and complete signup

### Choose Plan: FREE Tier

| Feature | Free Tier Limits | Notes |
|---------|------------------|-------|
| Storage | 10 GB/month | Plenty for starting |
| Class A ops (writes) | 1 million/month | PUT, POST, LIST |
| Class B ops (reads) | 10 million/month | GET requests |
| Egress (bandwidth) | **Unlimited FREE** | This is the big win! |

**Cost:** $0/month within free tier

### Create R2 Bucket

1. In Cloudflare Dashboard, go to **R2** in left sidebar
2. Click **Create bucket**
3. **Bucket name:** `revolution-trading-media` (or your preference)
4. **Location:** Choose closest to your users (or "Automatic")
5. Click **Create bucket**

### Get API Credentials

1. Go to **R2 > Overview**
2. Click **Manage R2 API Tokens**
3. Click **Create API token**
4. Configure:
   - **Token name:** `revolution-media-token`
   - **Permissions:** Object Read & Write
   - **Specify bucket:** Select your bucket
   - **TTL:** No expiration (or set as needed)
5. Click **Create API Token**
6. **SAVE THESE VALUES** (shown only once):
   - Access Key ID
   - Secret Access Key
   - Endpoint URL (format: `https://<account_id>.r2.cloudflarestorage.com`)

### Enable Public Access (for CDN)

1. Go to your bucket settings
2. Click **Settings** tab
3. Under **Public access**, click **Allow Access**
4. Choose **Custom Domain** or **R2.dev subdomain**
5. For R2.dev subdomain:
   - Toggle **Enable**
   - Your URL will be: `https://pub-<hash>.r2.dev`

---

## 2. Sharp (npm Package) - FREE

Sharp is an open-source npm package. No signup required!

### Requirements
- Node.js 18.17.0 or later
- npm or pnpm

### Installation (we'll do this in the project)
```bash
npm install sharp
```

**Cost:** $0 (MIT License, open source)

---

## 3. Turso (Optional - Edge SQLite)

### Sign Up

1. **Go to:** https://turso.tech
2. Click **Start for free**
3. Sign up with GitHub or email

### Choose Plan: Starter (FREE)

| Feature | Starter (Free) | Scaler ($29/mo) |
|---------|----------------|-----------------|
| Databases | 500 | 10,000 |
| Locations | 3 | Unlimited |
| Storage | 9 GB total | 24 GB included |
| Rows read | 1 billion/mo | 100 billion/mo |
| Rows written | 25 million/mo | 100 million/mo |

**Cost:** $0/month for Starter tier

### Create Database (if using Turso)

```bash
# Install Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# Login
turso auth login

# Create database
turso db create revolution-media --location ord

# Get connection URL
turso db show revolution-media --url

# Create auth token
turso db tokens create revolution-media
```

**For this project:** We'll use local SQLite initially. Turso is optional for production scaling.

---

## 4. Environment Variables Summary

Add these to your `.env` file:

```env
# Cloudflare R2 Configuration
R2_ACCESS_KEY_ID=your_access_key_id
R2_SECRET_ACCESS_KEY=your_secret_access_key
R2_BUCKET=revolution-trading-media
R2_ENDPOINT=https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com
R2_PUBLIC_URL=https://pub-YOUR_HASH.r2.dev

# Sharp Service (local)
SHARP_SERVICE_URL=http://localhost:3001
SHARP_SERVICE_ENABLED=true

# Turso (optional, for production)
TURSO_DATABASE_URL=libsql://your-db-name-your-org.turso.io
TURSO_AUTH_TOKEN=your_auth_token
```

---

## 5. Cost Summary

| Service | Tier | Monthly Cost |
|---------|------|--------------|
| Cloudflare R2 | Free | $0 |
| Sharp | Open Source | $0 |
| SQLite | Local | $0 |
| Turso (optional) | Starter | $0 |
| **Total** | | **$0** |

### When You'd Pay

- **R2:** Over 10GB storage → $0.015/GB/month
- **R2:** Over 1M writes → $4.50/million
- **Turso:** Over 9GB or need more locations → $29/month

---

## Quick Start Checklist

- [ ] Create Cloudflare account
- [ ] Create R2 bucket
- [ ] Get R2 API credentials
- [ ] Enable public access on bucket
- [ ] Copy credentials to `.env`
- [ ] Install Sharp in project
- [ ] Run the image service
- [ ] Test upload and optimization

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     YOUR APPLICATION                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────────┐        ┌──────────────────────┐              │
│   │   Frontend   │───────▶│   Laravel Backend    │              │
│   │  (SvelteKit) │        │   (Upload Handler)   │              │
│   └──────────────┘        └──────────────────────┘              │
│                                     │                            │
│                                     ▼                            │
│                           ┌──────────────────────┐              │
│                           │   Sharp Service      │              │
│                           │   (Node.js:3001)     │              │
│                           │   - WebP conversion  │              │
│                           │   - AVIF conversion  │              │
│                           │   - Responsive sizes │              │
│                           │   - BlurHash gen     │              │
│                           └──────────────────────┘              │
│                                     │                            │
│                    ┌────────────────┼────────────────┐          │
│                    ▼                ▼                ▼          │
│             ┌──────────┐    ┌─────────────┐   ┌──────────┐     │
│             │  SQLite  │    │ Cloudflare  │   │ Cloudflare│     │
│             │(Metadata)│    │     R2      │   │    CDN    │     │
│             └──────────┘    │  (Storage)  │   │  (Free)   │     │
│                             └─────────────┘   └──────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

Next: Run the implementation scripts to set up everything automatically!
