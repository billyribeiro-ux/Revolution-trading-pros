# Revolution Trading Pros - Rust API

> ICT 11+ Principal Engineer Grade - High-Performance Rust/Axum Backend

## Overview

This is a complete Rust/Axum replacement for the Laravel PHP backend, designed for maximum performance and reliability.

## Tech Stack

- **Framework:** Axum 0.8
- **Runtime:** Tokio 1.43
- **Database:** PostgreSQL with SQLx (compile-time verified queries)
- **Authentication:** JWT with Argon2 password hashing
- **Payments:** Stripe integration
- **Caching:** Redis (optional)
- **Deployment:** Fly.io

## Performance Targets

| Metric | Target | Laravel Baseline |
|--------|--------|------------------|
| p50 Response | < 10ms | ~50ms |
| p95 Response | < 50ms | ~200ms |
| p99 Response | < 100ms | ~500ms |
| Memory | < 50MB | ~100MB |
| Concurrent | 10,000+ | ~1,000 |

## Quick Start

### Prerequisites

- Rust 1.84+
- PostgreSQL 15+
- Redis (optional)

### Development

```bash
# Clone and setup
cd backend-rust
cp .env.example .env

# Edit .env with your database credentials

# Run migrations
sqlx database create
sqlx migrate run

# Start development server
cargo run

# Or with hot reload
cargo watch -x run
```

### Production Build

```bash
# Optimized release build
cargo build --release

# Run production binary
./target/release/revolution-trading-pros-api
```

## Deployment (Fly.io)

```bash
# Install Fly CLI
curl -L https://fly.io/install.sh | sh

# Login to Fly
fly auth login

# Create app (first time)
fly apps create revolution-trading-pros-api

# Set secrets
fly secrets set DATABASE_URL="postgres://..."
fly secrets set JWT_SECRET="your-secret-key"
fly secrets set STRIPE_SECRET_KEY="sk_live_..."
fly secrets set STRIPE_WEBHOOK_SECRET="whsec_..."

# Deploy
fly deploy

# Check status
fly status
fly logs
```

## API Endpoints

### Public Routes

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/health/live` | Liveness probe |
| GET | `/api/health/ready` | Readiness probe |
| GET | `/api/posts` | List blog posts |
| GET | `/api/videos` | List videos |
| POST | `/api/newsletter/subscribe` | Newsletter subscription |

### Authentication

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/auth/register` | User registration |
| POST | `/api/auth/login` | User login |
| POST | `/api/auth/logout` | User logout |
| POST | `/api/auth/refresh` | Refresh token |

### Protected Routes (require JWT)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/me` | Current user profile |
| GET | `/api/my/orders` | User orders |
| GET | `/api/my/subscriptions` | User subscriptions |
| GET | `/api/user/payment-methods` | Payment methods |

### Admin Routes (require admin role)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/admin/users` | List users |
| GET | `/api/admin/members` | List members |
| GET | `/api/admin/subscriptions` | List subscriptions |

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library root
├── config/              # Configuration
├── models/              # Database models
├── repositories/        # Data access layer
├── services/            # Business logic
├── handlers/            # HTTP handlers (controllers)
├── middleware/          # Auth, CORS, etc.
├── extractors/          # Custom Axum extractors
├── responses/           # API response types
├── routes/              # Route definitions
├── errors/              # Error handling
├── utils/               # Utilities
└── validation/          # Request validation
```

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Run specific test
cargo test test_login
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint with Clippy (zero warnings policy)
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## License

Proprietary - Revolution Trading Pros
