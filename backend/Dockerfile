FROM php:8.2-cli

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    libpng-dev \
    libonig-dev \
    libxml2-dev \
    libpq-dev \
    zip \
    unzip \
    && docker-php-ext-install pdo pdo_pgsql pgsql mbstring xml bcmath \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Set working directory
WORKDIR /app

# Copy composer files first for better caching
COPY composer.json composer.lock* ./

# Install dependencies (no dev, no scripts)
RUN composer install --no-dev --no-scripts --no-interaction --optimize-autoloader || \
    composer install --no-dev --no-scripts --no-interaction --ignore-platform-reqs

# Copy application code
COPY . .

# Create required Laravel directories
RUN mkdir -p storage/framework/cache/data \
    && mkdir -p storage/framework/sessions \
    && mkdir -p storage/framework/views \
    && mkdir -p storage/logs \
    && mkdir -p bootstrap/cache

# Set permissions
RUN chmod -R 775 storage bootstrap/cache \
    && chown -R www-data:www-data storage bootstrap/cache 2>/dev/null || true

# Generate autoloader
RUN composer dump-autoload --optimize

# Expose port (Railway uses PORT env var)
EXPOSE 8080

# Start script with error handling
CMD sh -c '\
    echo "Starting Laravel application..." && \
    php artisan config:clear && \
    php artisan route:clear && \
    php artisan view:clear && \
    if [ -n "$DATABASE_URL" ]; then \
        echo "Running migrations..." && \
        php artisan migrate --force || echo "Migration warning (may be ok)"; \
    fi && \
    echo "Starting server on port ${PORT:-8080}..." && \
    php artisan serve --host=0.0.0.0 --port=${PORT:-8080} \
'
