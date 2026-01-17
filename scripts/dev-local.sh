#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════════
# LOCAL DEVELOPMENT SCRIPT - Revolution Trading Pros
# Apple ICT 7 Grade - January 2026
#
# Usage:
#   ./scripts/dev-local.sh start   - Start local DB and run migrations
#   ./scripts/dev-local.sh stop    - Stop local DB
#   ./scripts/dev-local.sh reset   - Reset DB and re-run migrations
#   ./scripts/dev-local.sh api     - Start API server locally
#   ./scripts/dev-local.sh test    - Run tests against local DB
# ═══════════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Local DB config
export DATABASE_URL="postgres://postgres:postgres@localhost:5433/revolution_dev"
export REDIS_URL="redis://localhost:6380"

log() { echo -e "${GREEN}[DEV]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

start_db() {
    log "Starting local PostgreSQL and Redis..."
    docker-compose -f docker-compose.dev.yml up -d
    
    log "Waiting for PostgreSQL to be ready..."
    sleep 3
    
    until docker exec revolution-db-local pg_isready -U postgres > /dev/null 2>&1; do
        sleep 1
    done
    
    log "PostgreSQL is ready!"
    
    log "Running migrations..."
    cd api && cargo sqlx migrate run
    cd ..
    
    log "✓ Local environment ready!"
    log "  DATABASE_URL=$DATABASE_URL"
    log "  REDIS_URL=$REDIS_URL"
}

stop_db() {
    log "Stopping local services..."
    docker-compose -f docker-compose.dev.yml down
    log "✓ Services stopped"
}

reset_db() {
    log "Resetting database..."
    docker-compose -f docker-compose.dev.yml down -v
    start_db
}

run_api() {
    log "Starting API server locally..."
    log "DATABASE_URL=$DATABASE_URL"
    cd api
    
    # Set environment variables for local dev
    export JWT_SECRET="local-dev-secret-key-change-in-production"
    export RUST_LOG="debug"
    export ENVIRONMENT="development"
    
    cargo run
}

run_tests() {
    log "Running tests against local database..."
    cd api
    cargo test -- --test-threads=1
}

case "$1" in
    start)
        start_db
        ;;
    stop)
        stop_db
        ;;
    reset)
        reset_db
        ;;
    api)
        run_api
        ;;
    test)
        run_tests
        ;;
    *)
        echo "Usage: $0 {start|stop|reset|api|test}"
        echo ""
        echo "Commands:"
        echo "  start  - Start local DB and run migrations"
        echo "  stop   - Stop local DB"
        echo "  reset  - Reset DB and re-run migrations"
        echo "  api    - Start API server locally"
        echo "  test   - Run tests against local DB"
        exit 1
        ;;
esac
