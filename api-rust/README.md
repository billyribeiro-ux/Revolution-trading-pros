# Revolution Trading Pros - Rust API

Ultimate backend built with Rust + Cloudflare Workers for edge deployment.

## Stack

- **Runtime**: Cloudflare Workers (WASM)
- **Language**: Rust
- **Database**: Neon PostgreSQL (serverless)
- **Cache**: Upstash Redis
- **Storage**: Cloudflare R2
- **Search**: Meilisearch
- **Payments**: Stripe
- **Email**: Postmark

## Performance

- **P50 Latency**: < 10ms (edge)
- **P99 Latency**: < 50ms
- **Throughput**: 100k+ req/sec globally

## Prerequisites

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- wasm32 target (`rustup target add wasm32-unknown-unknown`)
- Wrangler CLI (`npm install -g wrangler`)
- Node.js 18+

## Setup

1. **Clone and install dependencies**
   ```bash
   cd api-rust
   cargo build
   ```

2. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env with your credentials
   ```

3. **Set up Cloudflare secrets**
   ```bash
   wrangler secret put DATABASE_URL
   wrangler secret put REDIS_URL
   wrangler secret put JWT_SECRET
   wrangler secret put STRIPE_SECRET_KEY
   wrangler secret put STRIPE_WEBHOOK_SECRET
   wrangler secret put POSTMARK_API_KEY
   wrangler secret put MEILISEARCH_URL
   wrangler secret put MEILISEARCH_API_KEY
   ```

4. **Run database migrations**
   ```bash
   # Connect to Neon and run migrations/001_initial_schema.sql
   psql $DATABASE_URL -f migrations/001_initial_schema.sql
   ```

## Development

```bash
# Start local development server
wrangler dev

# Build for production
wrangler build

# Deploy to Cloudflare Workers
wrangler deploy
```

## API Endpoints

### Public Routes

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/register` | Register new user |
| POST | `/api/login` | Login |
| POST | `/api/login/mfa` | MFA verification |
| POST | `/api/forgot-password` | Request password reset |
| POST | `/api/reset-password` | Reset password |
| GET | `/api/posts` | List published posts |
| GET | `/api/posts/:slug` | Get post by slug |
| GET | `/api/products` | List products |
| GET | `/api/indicators` | List indicators |
| POST | `/api/newsletter/subscribe` | Subscribe to newsletter |

### Protected Routes (require auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/me` | Get current user |
| GET | `/api/me/memberships` | Get user memberships |
| GET | `/api/me/products` | Get user products |
| PUT | `/api/me/password` | Change password |
| GET | `/api/my/subscriptions` | List subscriptions |
| POST | `/api/my/subscriptions` | Create subscription |
| POST | `/api/cart/checkout` | Process checkout |

### Admin Routes (require admin role)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/admin/posts` | List all posts |
| POST | `/api/admin/posts` | Create post |
| PUT | `/api/admin/posts/:id` | Update post |
| DELETE | `/api/admin/posts/:id` | Delete post |
| GET | `/api/admin/users` | List users |
| POST | `/api/admin/users/:id/ban` | Ban user |
| GET | `/api/admin/products` | List products |
| POST | `/api/admin/media/upload` | Upload media |

## Project Structure

```
api-rust/
├── Cargo.toml              # Dependencies
├── wrangler.toml           # Cloudflare Workers config
├── src/
│   ├── lib.rs              # Entry point
│   ├── config.rs           # Configuration
│   ├── error.rs            # Error types
│   ├── db/
│   │   ├── postgres.rs     # Neon PostgreSQL
│   │   └── redis.rs        # Upstash Redis
│   ├── models/
│   │   ├── user.rs         # User models
│   │   ├── post.rs         # Post models
│   │   ├── product.rs      # Product models
│   │   ├── subscription.rs # Subscription models
│   │   └── email.rs        # Email models
│   ├── routes/
│   │   ├── auth.rs         # Auth routes
│   │   ├── users.rs        # User routes
│   │   ├── posts.rs        # Post routes
│   │   ├── products.rs     # Product routes
│   │   └── admin/          # Admin routes
│   ├── services/
│   │   ├── jwt.rs          # JWT auth
│   │   ├── stripe.rs       # Stripe payments
│   │   ├── postmark.rs     # Email service
│   │   ├── r2.rs           # R2 storage
│   │   └── meilisearch.rs  # Search
│   └── middleware/
│       ├── auth.rs         # Auth middleware
│       └── rate_limit.rs   # Rate limiting
└── migrations/
    └── 001_initial_schema.sql
```

## Switching Frontend

Update `frontend/wrangler.toml`:

```toml
# From Laravel
VITE_API_URL = "https://revolution-trading-pros.fly.dev/api"

# To Rust
VITE_API_URL = "https://revolution-trading-pros-api.workers.dev/api"
```

## License

MIT
