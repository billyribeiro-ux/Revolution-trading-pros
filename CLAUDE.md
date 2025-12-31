# CLAUDE.md - Revolution Trading Pros

This file provides context and guidelines for Claude Code when working on this project.

## Project Overview

Revolution Trading Pros is a high-performance trading platform for educational trading content, live trading rooms, courses, and membership management.

## Architecture

| Component | Technology | Location |
|-----------|------------|----------|
| **Frontend** | SvelteKit 5, Svelte 5, TailwindCSS 4, TypeScript 5 | `/frontend` |
| **Backend API** | Rust + Axum | `/api` |
| **Database** | Neon PostgreSQL (Serverless) | - |
| **Cache** | Upstash Redis | - |
| **Storage** | Cloudflare R2 | - |
| **Search** | Meilisearch | - |
| **Payments** | Stripe | - |
| **Email** | Postmark | - |
| **Image Service** | Node.js | `/image-service` |

## Quick Commands

### Frontend (from `/frontend`)
```bash
npm run dev          # Start dev server
npm run build        # Production build
npm run check        # Type checking
npm run lint         # Lint check
npm run format       # Format code
npm run test         # E2E tests (Playwright)
npm run test:unit    # Unit tests (Vitest)
```

### Backend (from `/api`)
```bash
cargo build          # Build
cargo run            # Run development server
cargo test           # Run tests
cargo clippy         # Lint
```

## Critical Reference Files

### Dashboard Reference Files (SOURCE OF TRUTH)
**Location:** `/frontend/Do's/`

Before working on any dashboard pages, you MUST consult the HTML reference files in this directory. They define the exact structure, styling, and functionality expected for each dashboard page.

| File | Page | Route |
|------|------|-------|
| `DashboardHome` | Main dashboard landing | `/dashboard` |
| `DayTradingRoomDashboard` | Day trading room | `/dashboard/day-trading-room` |
| `LatestUpdates` | Latest updates | `/dashboard/latest-updates` |
| `Learning-Center` | Learning center | `/dashboard/learning-center` |
| `Platform-Tutorials` | Tutorials | `/dashboard/platform-tutorials` |
| `Premium-Daily-Videos` | Videos listing | `/dashboard/premium-daily-videos` |
| `Premium-Daily-Videos-Clicked` | Video detail | `/dashboard/premium-daily-videos/[id]` |
| `wwclickememberdashboard` | Member dashboard | `/dashboard/member` |

### Key Documentation
- `/frontend/.ai-context.md` - AI agent context and guidelines
- `/frontend/ARCHITECTURE.md` - Frontend architecture guide
- `/frontend/CSS_ARCHITECTURE.md` - CSS and styling guidelines
- `/frontend/PERFORMANCE.md` - Performance optimization guidelines
- `/docs/REVOLUTION-ENGINEERING-SSOT.md` - Engineering single source of truth

## Project Structure

```
.
├── api/                    # Rust + Axum Backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── config/         # Configuration
│   │   ├── db/             # Database layer
│   │   ├── models/         # Data models
│   │   ├── routes/         # API endpoints
│   │   ├── services/       # Business logic (redis, stripe, email, storage, search)
│   │   ├── middleware/     # Auth, rate limiting
│   │   └── queue/          # Background job processing
│   └── migrations/         # SQL migrations
├── frontend/               # SvelteKit 5 frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── api/        # API clients
│   │   │   ├── components/ # UI components (ui/, patterns/, layout/, charts/)
│   │   │   ├── stores/     # Svelte stores
│   │   │   ├── utils/      # Utility functions
│   │   │   ├── types/      # TypeScript types
│   │   │   └── observability/ # Metrics, feature flags, telemetry
│   │   └── routes/         # SvelteKit routes
│   └── Do's/               # Dashboard reference files (CRITICAL)
├── image-service/          # Node.js image processing microservice
├── docs/                   # Project documentation
├── k8s/                    # Kubernetes configurations
└── .github/workflows/      # CI/CD (Cloudflare, Fly.io, E2E tests)
```

## Code Patterns

### Frontend Component Imports
```typescript
// Import from main barrel
import { Button, DataTable, StatCard } from '$lib/components';

// Or from specific category
import { Button, Input } from '$lib/components/ui';
import { DataTable, EmptyState } from '$lib/components/patterns';
import { MarketingNav } from '$lib/components/layout';
```

### Frontend Design Tokens
Use color tokens defined in `app.css` with `@theme` directive:
```svelte
<div class="bg-rtp-surface text-rtp-text border-rtp-border">
  <!-- Content -->
</div>
```

### API Routes (Rust)
Routes are modular in `/api/src/routes/`. Key domains:
- `auth.rs` - Authentication
- `users.rs` - User management
- `courses.rs` - Course content
- `products.rs` - Products/shop
- `checkout.rs` - Checkout flow
- `payments.rs` - Stripe payments
- `subscriptions.rs` - Subscription management
- `videos.rs` - Video content
- `indicators.rs` - Trading indicators
- `admin.rs` - Admin endpoints

## Testing

### E2E Tests (Playwright)
```bash
cd frontend
npm run test:e2e              # All E2E tests
npm run test:e2e:smoke        # Smoke tests only
npm run test:e2e:auth         # Auth tests
npm run test:e2e:trading      # Trading room tests
npm run test:e2e:checkout     # Checkout tests
npm run test:e2e:debug        # Debug mode with browser
```

### Unit Tests
```bash
cd frontend
npm run test:unit             # Vitest unit tests
```

### Backend Tests
```bash
cd api
cargo test                    # Rust tests
```

## Deployment

- **Frontend**: Cloudflare Pages (`npm run deploy:cloudflare`)
- **Backend**: Fly.io (see `/api/fly.toml` and `Dockerfile`)
- **CI/CD**: GitHub Actions (`.github/workflows/`)

## Key Principles

1. **Reference First**: Always check `/frontend/Do's/` before implementing dashboard features
2. **Component Reuse**: Use components from `$lib/components` (ui, patterns, layout)
3. **Design Tokens**: Use color tokens from `app.css`
4. **Performance**: Code-split heavy libraries, lazy load components
5. **Type Safety**: Full TypeScript in frontend, strong typing in Rust backend
6. **Security**: JWT auth, rate limiting, input validation

## Environment Variables

### Frontend (`.env` files in `/frontend`)
- `PUBLIC_API_URL` - Backend API URL
- `PUBLIC_STRIPE_KEY` - Stripe publishable key
- See `.env.example` for full list

### Backend (`.env` in `/api`)
- `DATABASE_URL` - Neon PostgreSQL connection
- `REDIS_URL` - Upstash Redis connection
- `STRIPE_SECRET_KEY` - Stripe secret key
- `R2_*` - Cloudflare R2 storage credentials
- `MEILISEARCH_*` - Meilisearch credentials
- See `.env.example` for full list
