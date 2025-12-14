#!/bin/sh
set -e

echo "=== Revolution Trading Pros - Laravel Startup ==="
echo "Environment: ${APP_ENV:-production}"
echo "Port: ${PORT:-8080}"
echo "Build: 2025-12-14-v2"

# Clear all caches first (safe operation)
echo "Clearing caches..."
php artisan config:clear || true
php artisan route:clear || true
php artisan view:clear || true

# Run migrations only if DATABASE_URL is set
if [ -n "$DATABASE_URL" ]; then
    echo "Database URL detected, running migrations..."
    php artisan migrate --force || echo "[WARN] Migration had issues, continuing..."
else
    echo "[INFO] No DATABASE_URL set, skipping migrations"
fi

# Only cache in production if we have all required env vars
if [ "$APP_ENV" = "production" ] && [ -n "$APP_KEY" ]; then
    echo "Caching configuration for production..."
    php artisan config:cache || echo "[WARN] Config cache failed, using uncached"
    php artisan route:cache || echo "[WARN] Route cache failed, using uncached"
fi

echo "Starting Laravel server on 0.0.0.0:${PORT:-8080}..."
exec php artisan serve --host=0.0.0.0 --port="${PORT:-8080}"
