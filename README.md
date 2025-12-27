# Revolution Trading Pros

High-performance trading platform built with Rust + Axum.

## Architecture

| Component | Technology |
|-----------|------------|
| **Frontend** | SvelteKit 5 + TailwindCSS |
| **Backend API** | Rust + Axum |
| **Database** | Neon PostgreSQL (Serverless) |
| **Cache** | Upstash Redis |
| **Storage** | Cloudflare R2 |
| **Search** | Meilisearch |
| **Payments** | Stripe |
| **Email** | Postmark |

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
│   │   ├── services/       # Business logic
│   │   └── middleware/     # Auth, rate limiting
│   └── migrations/         # SQL migrations
├── frontend/               # SvelteKit 5 frontend
│   ├── Do's/               # ⚠️ CRITICAL: Dashboard reference files
│   │   └── README.md       # MUST READ before dashboard work
│   ├── .ai-context.md      # AI agent context and guidelines
│   └── ARCHITECTURE.md     # Frontend architecture guide
├── image-service/          # Node.js image processing microservice
└── DEPLOYMENT_GUIDE_RUST.md
```

**⚠️ For AI Agents:** Before working on dashboard pages, consult `frontend/Do's/` for HTML reference files and `frontend/.ai-context.md` for essential context.

## Quick Start

### Prerequisites

- Rust 1.80+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Node.js 20+
- PostgreSQL or Neon account
- Redis or Upstash account

### Backend Setup

```bash
cd api
cp .env.example .env
# Edit .env with your credentials

cargo build
cargo run
```

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

## API Documentation

See [api/README.md](api/README.md) for full API documentation.

## Deployment

See [DEPLOYMENT_GUIDE_RUST.md](DEPLOYMENT_GUIDE_RUST.md) for production deployment instructions.

## License

MIT
