#!/bin/sh
set -e

echo "Starting Revolution Trading Pros Backend..."

# Run migrations
echo "Running migrations..."
php artisan migrate --force

# Clear and rebuild caches (in case env changed)
echo "Optimizing for production..."
php artisan config:cache
php artisan route:cache
php artisan view:cache

# Start the application with optimized settings
echo "Starting server on port 8080..."
exec php artisan serve --host=0.0.0.0 --port=8080
