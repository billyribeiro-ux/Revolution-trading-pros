#!/bin/sh
set -e

################################################################################
# REVOLUTION TRADING PROS - DOCKER ENTRYPOINT
#
# Stack: Laravel 12 + Octane + FrankenPHP (December 2025)
# Supports: Production, Staging, and Local environments
################################################################################

echo "=============================================="
echo "  Revolution Trading Pros - Laravel Startup"
echo "=============================================="
echo "Environment: ${APP_ENV:-production}"
echo "Port: ${PORT:-8080}"
echo "Server: FrankenPHP + Octane"
echo "Workers: ${OCTANE_WORKERS:-auto}"
echo "Build: 2025-12-14-frankenphp"
echo ""

# Create OPcache file cache directory
mkdir -p /tmp/opcache

# Clear all caches first (safe operation)
echo "[1/6] Clearing caches..."
php artisan config:clear 2>/dev/null || true
php artisan route:clear 2>/dev/null || true
php artisan view:clear 2>/dev/null || true
php artisan event:clear 2>/dev/null || true

# Run migrations only if DATABASE_URL is set
if [ -n "$DATABASE_URL" ] || [ -n "$DB_HOST" ]; then
    echo "[2/6] Running database migrations..."
    php artisan migrate --force || echo "[WARN] Migration had issues, continuing..."
else
    echo "[2/6] No database configured, skipping migrations"
fi

# Cache configuration in production
if [ "$APP_ENV" = "production" ] && [ -n "$APP_KEY" ]; then
    echo "[3/6] Caching configuration for production..."
    php artisan config:cache || echo "[WARN] Config cache failed"
    php artisan route:cache || echo "[WARN] Route cache failed"
    php artisan view:cache || echo "[WARN] View cache failed"
    php artisan event:cache || echo "[WARN] Event cache failed"
else
    echo "[3/6] Skipping cache (non-production or missing APP_KEY)"
fi

# Optimize class loading
echo "[4/6] Optimizing autoloader..."
composer dump-autoload --optimize --classmap-authoritative 2>/dev/null || true

# Warm up Octane (preload commonly used classes)
echo "[5/6] Warming up application..."
php artisan octane:status 2>/dev/null || true

# Start the appropriate server
echo "[6/6] Starting server..."
echo ""

# Determine worker count
WORKERS="${OCTANE_WORKERS:-auto}"
if [ "$WORKERS" = "auto" ]; then
    # Use number of CPU cores, minimum 2
    WORKERS=$(nproc 2>/dev/null || echo 2)
    [ "$WORKERS" -lt 2 ] && WORKERS=2
fi

# Determine task workers (for concurrent tasks)
TASK_WORKERS="${OCTANE_TASK_WORKERS:-$WORKERS}"

# Start based on environment
if [ "$USE_STANDARD_SERVER" = "true" ]; then
    echo "Starting standard Laravel server (non-Octane)..."
    exec php artisan serve --host=0.0.0.0 --port="${PORT:-8080}"
else
    echo "Starting Octane with FrankenPHP..."
    echo "  - Workers: $WORKERS"
    echo "  - Task Workers: $TASK_WORKERS"
    echo "  - Max Requests: ${OCTANE_MAX_REQUESTS:-1000}"
    echo ""

    exec php artisan octane:start \
        --server=frankenphp \
        --host=0.0.0.0 \
        --port="${PORT:-8080}" \
        --workers="$WORKERS" \
        --task-workers="$TASK_WORKERS" \
        --max-requests="${OCTANE_MAX_REQUESTS:-1000}"
fi
