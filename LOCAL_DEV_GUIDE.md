# Local Development Guide
## Revolution Trading Pros - ICT 7 Grade

This guide explains how to develop and test new features **locally** without deploying to Fly.io.

---

## Quick Start

```bash
# 1. Start local PostgreSQL + Redis
./scripts/dev-local.sh start

# 2. Run API server locally
./scripts/dev-local.sh api

# 3. Test your changes locally at http://localhost:3000/api

# 4. When done, stop local services
./scripts/dev-local.sh stop
```

---

## Prerequisites

- Docker Desktop installed and running
- Rust toolchain (`rustup`)
- `cargo-sqlx` installed: `cargo install sqlx-cli`

---

## Commands

| Command | Description |
|---------|-------------|
| `./scripts/dev-local.sh start` | Start local DB and run all migrations |
| `./scripts/dev-local.sh stop` | Stop local DB |
| `./scripts/dev-local.sh reset` | Reset DB (wipe data) and re-run migrations |
| `./scripts/dev-local.sh api` | Start API server locally |
| `./scripts/dev-local.sh test` | Run tests against local DB |

---

## Workflow for New Features

### 1. Start Local Environment
```bash
./scripts/dev-local.sh start
```

### 2. Create New Migration (if needed)
```bash
cd api
cargo sqlx migrate add my_new_feature
# Edit: api/migrations/YYYYMMDD_XXXXXX_my_new_feature.sql
```

### 3. Run Migration Locally
```bash
cd api
DATABASE_URL="postgres://postgres:postgres@localhost:5433/revolution_dev" cargo sqlx migrate run
```

### 4. Develop & Test
```bash
# Terminal 1: Run API
./scripts/dev-local.sh api

# Terminal 2: Test endpoints
curl http://localhost:3000/api/your-endpoint
```

### 5. When Ready to Deploy
```bash
git add -A
git commit -m "feat(feature): description"
git push
# CI/CD will run migrations on Fly.io automatically
```

---

## Environment Variables

Local development uses:
```
DATABASE_URL=postgres://postgres:postgres@localhost:5433/revolution_dev
REDIS_URL=redis://localhost:6380
JWT_SECRET=local-dev-secret-key
RUST_LOG=debug
ENVIRONMENT=development
```

---

## Migration Best Practices

1. **Always use `CREATE TABLE IF NOT EXISTS`** - Prevents errors on re-run
2. **Always use `ALTER TABLE ... ADD COLUMN IF NOT EXISTS`** - Safe column additions
3. **Test migrations locally first** before pushing
4. **One migration per feature** - Keep migrations focused
5. **Use sequential numbering** - `016_`, `017_`, etc.

---

## Ports

| Service | Local Port |
|---------|------------|
| PostgreSQL | 5433 |
| Redis | 6380 |
| API Server | 3000 |

Note: Using non-standard ports (5433, 6380) to avoid conflicts with system PostgreSQL/Redis.

---

## Troubleshooting

### Database connection refused
```bash
# Check if Docker containers are running
docker ps

# Restart containers
./scripts/dev-local.sh reset
```

### Migration errors
```bash
# Reset database and re-run all migrations
./scripts/dev-local.sh reset
```

### Port already in use
```bash
# Find and kill process
lsof -i :5433
kill -9 <PID>
```

---

## Files

| File | Purpose |
|------|---------|
| `docker-compose.dev.yml` | Local DB/Redis containers |
| `scripts/dev-local.sh` | Development helper script |
| `api/migrations/` | SQL migrations |
| `FRONTEND_BACKEND_API_AUDIT.md` | API endpoint reference |

---

*ICT 7 Principal Engineer - Apple Standards*
