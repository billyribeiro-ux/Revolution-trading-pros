# Revolution Trading Pros - Repository Structure
**Apple Principal Engineer ICT 7 Grade - January 2026**

Generated: January 24, 2026

---

## 📋 Table of Contents
- [Overview](#overview)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Database Schema](#database-schema)
- [API Routes](#api-routes)
- [Environment Variables](#environment-variables)
- [Development Workflow](#development-workflow)

---

## Overview

**Revolution Trading Pros** is a full-stack trading education and alert platform built with:
- **Backend:** Rust (Axum) on Fly.io
- **Frontend:** SvelteKit on Cloudflare Pages
- **Database:** PostgreSQL on Fly.io
- **Cache:** Redis (Upstash)
- **Storage:** Cloudflare R2
- **Video:** Bunny.net Stream

---

## Technology Stack

### Backend (api/)
- **Framework:** Axum 0.7 (Rust)
- **Database:** SQLx with PostgreSQL
- **Authentication:** JWT with Argon2id
- **Payments:** Stripe
- **Email:** Postmark
- **Search:** Meilisearch
- **Video:** Bunny.net Stream API

### Frontend (frontend/)
- **Framework:** SvelteKit 2.x
- **Styling:** Tailwind CSS v4
- **Icons:** Lucide
- **Deployment:** Cloudflare Pages

---

## Project Structure

\`\`\`
Revolution-trading-pros/
├── api/                          # Rust backend (Axum)
│   ├── src/
│   │   ├── routes/              # API route handlers (56 files)
│   │   ├── middleware/          # Auth, validation, CORS
│   │   ├── db/                  # Database connection
│   │   ├── services/            # Business logic
│   │   └── main.rs              # Entry point
│   ├── migrations/              # SQLx migrations (21 files)
│   ├── Dockerfile               # Production build
│   ├── fly.toml                 # Fly.io config
│   └── .env                     # Backend secrets (gitignored)
│
├── frontend/                     # SvelteKit frontend
│   ├── src/
│   │   ├── routes/              # File-based routing
│   │   ├── lib/
│   │   │   ├── components/      # Reusable UI components
│   │   │   ├── stores/          # Svelte stores
│   │   │   └── utils/           # Helper functions
│   │   └── app.css              # Global styles
│   ├── static/                  # Static assets
│   ├── wrangler.toml            # Cloudflare Pages config
│   └── .env.production          # Frontend secrets (gitignored)
│
├── .github/workflows/           # CI/CD pipelines
│   ├── deploy-fly.yml           # Backend deployment
│   └── deploy-cloudflare.yml    # Frontend deployment
│
├── .env.example                 # Master env template
└── REPO_STRUCTURE.md            # This file
\`\`\`

---

## Database Schema

**Total Tables:** 72

### Core Tables
- \`users\` - User accounts and authentication
- \`membership_plans\` - Subscription plans
- \`user_memberships\` - Active subscriptions
- \`trading_rooms\` - Trading room definitions
- \`room_weekly_videos\` - Weekly video content
- \`room_trade_plans\` - Trade setups
- \`room_alerts\` - Real-time trade alerts
- \`watchlist_entries\` - Weekly stock watchlists

### Content Management
- \`courses_enhanced\` - Course catalog
- \`course_sections\` - Course modules
- \`course_lessons\` - Individual lessons
- \`indicators\` - Trading indicators
- \`posts\` - Blog posts
- \`media\` - Media library

### Commerce
- \`orders\` - Order history
- \`order_items\` - Line items
- \`coupons\` - Discount codes
- \`invoices\` - Stripe invoices

### Analytics
- \`analytics_events\` - User events
- \`audit_logs\` - Security audit trail
- \`webhook_deliveries\` - Webhook logs

---

## API Routes

**Total Routes:** 56 route files, ~500+ endpoints

### Authentication & Users
- \`/api/auth/*\` - Login, register, logout, MFA
- \`/api/user/*\` - User profile, settings
- \`/api/users/*\` - User management (admin)

### Trading Rooms
- \`/api/rooms/*\` - Room listings
- \`/api/rooms/:slug/trade-plan\` - Trade plans
- \`/api/rooms/:slug/alerts\` - Trade alerts
- \`/api/rooms/:slug/weekly-video\` - Weekly videos

### Content
- \`/api/courses/*\` - Course catalog
- \`/api/my/courses/*\` - Enrolled courses
- \`/api/indicators/*\` - Indicator library
- \`/api/posts/*\` - Blog posts

### Admin
- \`/api/admin/users/*\` - User management
- \`/api/admin/courses/*\` - Course admin
- \`/api/admin/room-content/*\` - Room content management
- \`/api/admin/bunny/*\` - Video upload

### Commerce
- \`/api/checkout/*\` - Checkout flow
- \`/api/subscriptions/*\` - Subscription management
- \`/api/coupons/*\` - Coupon validation

### Utilities
- \`/health\` - Health check
- \`/api/search\` - Global search
- \`/api/watchlist/:slug\` - Watchlist data

---

## Environment Variables

### Backend (.env)
\`\`\`bash
# Server
PORT=8080
ENVIRONMENT=development

# Database & Cache
DATABASE_URL=postgres://...
REDIS_URL=rediss://...

# Storage
R2_ENDPOINT=https://...
R2_ACCESS_KEY_ID=...
R2_SECRET_ACCESS_KEY=...
R2_BUCKET=revolution-trading-media
R2_PUBLIC_URL=https://pub-xxx.r2.dev

# Authentication
JWT_SECRET=...
JWT_EXPIRES_IN=24

# Services
STRIPE_SECRET=sk_test_...
POSTMARK_TOKEN=...
MEILISEARCH_HOST=https://...
BUNNY_STREAM_LIBRARY_ID=585929
BUNNY_STREAM_API_KEY=...
\`\`\`

### Frontend (.env.production)
\`\`\`bash
# API
VITE_API_URL=https://revolution-trading-pros-api.fly.dev
VITE_CDN_URL=https://pub-xxx.r2.dev

# Analytics
VITE_GTM_ID=GTM-XXXXXXX
PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX
PUBLIC_META_PIXEL_ID=...
\`\`\`

---

## Development Workflow

### Local Development

**Backend:**
\`\`\`bash
cd api
cp .env.example .env  # Fill in values
cargo run
# API runs on http://localhost:8080
\`\`\`

**Frontend:**
\`\`\`bash
cd frontend
cp .env.example .env.local  # Fill in values
pnpm install
pnpm dev
# App runs on http://localhost:5173
\`\`\`

### Deployment

**Backend (Fly.io):**
\`\`\`bash
cd api
fly deploy --strategy immediate
\`\`\`

**Frontend (Cloudflare Pages):**
- Automatic deployment on push to \`main\`
- Or manual: \`pnpm run deploy\`

### Database Migrations

\`\`\`bash
cd api
sqlx migrate run
\`\`\`

Migrations run automatically on API startup.

---

## Key Files

| File | Purpose |
|------|---------|
| \`api/fly.toml\` | Fly.io configuration with health checks |
| \`frontend/wrangler.toml\` | Cloudflare Pages configuration |
| \`.github/workflows/deploy-fly.yml\` | Backend CI/CD |
| \`.github/workflows/deploy-cloudflare.yml\` | Frontend CI/CD |
| \`api/migrations/\` | Database schema versions |
| \`BUNNY_CREDENTIALS.md\` | Video streaming credentials (gitignored) |

---

## Production URLs

- **Frontend:** https://revolution-trading-pros.pages.dev
- **Backend API:** https://revolution-trading-pros-api.fly.dev
- **Health Check:** https://revolution-trading-pros-api.fly.dev/health
- **API Docs:** https://revolution-trading-pros-api.fly.dev/swagger-ui

---

## Security Notes

1. **Secrets:** Never commit \`.env\` files
2. **JWT:** Uses Argon2id with OWASP parameters
3. **CORS:** Configured for production domains only
4. **Rate Limiting:** Implemented via Redis
5. **SQL Injection:** Protected via SQLx compile-time checks

---

**Last Updated:** $(date)
**Maintained By:** Revolution Trading Pros Engineering Team
