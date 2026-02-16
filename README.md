# Revolution Trading Pros

**Enterprise Trading Education Platform**  
*Apple Principal Engineer ICT Level 7+ Standards*

[![CI Status](https://github.com/revolutiontradingpros/revolution-trading-pros/workflows/CI/badge.svg)](https://github.com/revolutiontradingpros/revolution-trading-pros/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)

---

## 🎯 Overview

Revolution Trading Pros is a production-grade trading education platform delivering real-time trade alerts, comprehensive course content, and advanced analytics to traders worldwide.

### Key Features

- **🚀 Explosive Swings Trading Room** - Real-time alerts, trade tracking, performance analytics
- **📚 Course Management System** - Video streaming, progress tracking, interactive lessons
- **✍️ Enterprise CMS** - 30+ block types, real-time collaboration (Yjs CRDT), AI-powered content
- **💳 Stripe Integration** - Subscriptions, one-time payments, coupon management
- **📊 Analytics Dashboard** - Performance metrics, equity curves, win rate tracking
- **🔐 Enterprise Auth** - OAuth (Google/Apple), MFA (TOTP), session management

---

## 🏗️ Architecture

### Technology Stack

**Frontend**
- **Framework:** SvelteKit 2.x + Svelte 5 (runes)
- **Language:** TypeScript 5.x
- **Styling:** Tailwind CSS v4
- **Deployment:** Cloudflare Pages
- **CDN:** Bunny.net (video), Cloudflare R2 (storage)

**Backend**
- **Framework:** Rust + Axum 0.7
- **Database:** PostgreSQL (Fly.io)
- **Cache:** Redis (Upstash)
- **Deployment:** Fly.io
- **Payments:** Stripe

### Monorepo Structure

```
revolution-trading-pros/
├── frontend/          # SvelteKit application
│   ├── src/
│   │   ├── lib/      # Shared libraries & components
│   │   └── routes/   # SvelteKit file-based routing
│   ├── e2e/          # Playwright E2E tests (363+ tests)
│   └── static/       # Static assets
├── api/              # Rust/Axum backend
│   ├── src/
│   │   ├── routes/   # API endpoints (56+)
│   │   ├── models/   # Database models
│   │   ├── services/ # Business logic
│   │   └── middleware/ # Auth, validation, etc.
│   └── migrations/   # SQL migrations
├── docs/             # Documentation
└── .github/          # CI/CD workflows
```

---

## 🚀 Quick Start

### Prerequisites

- **Node.js** 20.x or later
- **Rust** 1.75 or later
- **PostgreSQL** 15+
- **Redis** 7+
- **npm** 10.x or later

### Installation

```bash
# Clone the repository
git clone https://github.com/revolutiontradingpros/revolution-trading-pros.git
cd revolution-trading-pros

# Install frontend dependencies
cd frontend
npm install

# Install Rust dependencies (backend)
cd ../api
cargo build

# Set up environment variables
cp .env.example .env
# Edit .env with your configuration
```

### Development

```bash
# Terminal 1: Start frontend dev server
cd frontend
npm run dev
# → http://localhost:5174

# Terminal 2: Start backend API
cd api
cargo run
# → http://localhost:8080
```

### Build & Deploy

```bash
# Frontend (Cloudflare Pages)
cd frontend
npm run build:cloudflare
npm run deploy:cloudflare

# Backend (Fly.io)
cd api
fly deploy
```

---

## 📖 Documentation

- **[Setup Guide](SETUP_GUIDE.md)** - Complete environment setup
- **[Deployment Guide](DEPLOYMENT_GUIDE.md)** - Production deployment instructions
- **[API Documentation](docs/api/)** - Complete API reference
- **[Architecture](docs/ARCHITECTURE.md)** - System architecture & design decisions
- **[Contributing](CONTRIBUTING.md)** - Contribution guidelines
- **[Testing Guide](docs/TESTING.md)** - Testing strategy & best practices

### Feature Documentation

- **[Explosive Swings](docs/features/EXPLOSIVE_SWINGS.md)** - Trading room documentation
- **[CMS System](docs/features/CMS.md)** - Content management system
- **[Course System](docs/features/COURSES.md)** - Course delivery platform
- **[Authentication](docs/features/AUTH.md)** - Auth & security

---

## 🧪 Testing

```bash
# Frontend
cd frontend
npm run test              # Unit tests (Vitest)
npm run test:e2e          # E2E tests (Playwright)
npm run check             # Type checking
npm run lint              # Linting

# Backend
cd api
cargo test                # Unit & integration tests
cargo clippy              # Linting
```

**Test Coverage:**
- **E2E Tests:** 363+ tests across blog, CMS, and core features
- **Unit Tests:** Comprehensive coverage of business logic
- **Integration Tests:** API endpoint validation

---

## 📊 Project Status

✅ **Production Ready** - Deployed and serving users  
✅ **Zero Errors/Warnings** - Clean builds enforced  
✅ **ICT Level 7+ Compliance** - Apple engineering standards  
✅ **Full CI/CD** - Automated testing & deployment  
✅ **Comprehensive Documentation** - Complete API & feature docs

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development workflow, coding standards, and pull request process.

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- **Production:** https://revolutiontradingpros.com
- **API:** https://revolution-trading-pros-api.fly.dev
- **Documentation:** https://docs.revolutiontradingpros.com
- **Support:** support@revolutiontradingpros.com

