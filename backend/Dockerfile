################################################################################
# REVOLUTION TRADING PROS - PRODUCTION DOCKERFILE
#
# Stack: Laravel 12 + Octane + FrankenPHP (December 2025)
# Performance: Sub-millisecond response times with persistent workers
################################################################################

# -----------------------------------------------------------------------------
# Stage 1: Composer Dependencies
# -----------------------------------------------------------------------------
FROM composer:2.8 AS composer

WORKDIR /app

# Copy only dependency files for better caching
COPY composer.json composer.lock* ./

# Install dependencies without dev packages
RUN composer install \
    --no-dev \
    --no-scripts \
    --no-interaction \
    --prefer-dist \
    --optimize-autoloader \
    --ignore-platform-reqs

# -----------------------------------------------------------------------------
# Stage 2: Production Image with FrankenPHP
# -----------------------------------------------------------------------------
FROM dunglas/frankenphp:1-php8.3-alpine AS production

# Labels for container registry
LABEL maintainer="Revolution Trading Pros"
LABEL version="2.0.0"
LABEL description="Laravel 12 + Octane + FrankenPHP Production Image"

# Install system dependencies
RUN apk add --no-cache \
    git \
    curl \
    libpng-dev \
    libjpeg-turbo-dev \
    freetype-dev \
    libwebp-dev \
    libzip-dev \
    libxml2-dev \
    postgresql-dev \
    icu-dev \
    oniguruma-dev \
    linux-headers \
    $PHPIZE_DEPS

# Configure and install PHP extensions
RUN docker-php-ext-configure gd \
        --with-freetype \
        --with-jpeg \
        --with-webp \
    && docker-php-ext-install -j$(nproc) \
        pdo \
        pdo_pgsql \
        pgsql \
        mbstring \
        xml \
        bcmath \
        gd \
        zip \
        intl \
        pcntl \
        opcache \
        sockets

# Install Redis extension
RUN pecl install redis \
    && docker-php-ext-enable redis

# Cleanup build dependencies
RUN apk del $PHPIZE_DEPS linux-headers \
    && rm -rf /var/cache/apk/* /tmp/*

# PHP configuration for production
COPY docker/php/php.ini /usr/local/etc/php/conf.d/99-production.ini
COPY docker/php/opcache.ini /usr/local/etc/php/conf.d/opcache.ini

# Set working directory
WORKDIR /app

# Copy Composer dependencies from builder
COPY --from=composer /app/vendor ./vendor

# Copy application code
COPY . .

# Create required Laravel directories
RUN mkdir -p \
    storage/framework/cache/data \
    storage/framework/sessions \
    storage/framework/views \
    storage/logs \
    bootstrap/cache \
    && chmod -R 775 storage bootstrap/cache

# Generate optimized autoloader
COPY --from=composer:2.8 /usr/bin/composer /usr/bin/composer
RUN composer dump-autoload --optimize --classmap-authoritative

# Make entrypoint executable
RUN chmod +x docker-entrypoint.sh

# Create non-root user for security
RUN addgroup -g 1000 laravel \
    && adduser -u 1000 -G laravel -h /app -D laravel \
    && chown -R laravel:laravel /app

# Switch to non-root user
USER laravel

# Expose port (Railway uses PORT env var)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT:-8080}/api/health || exit 1

# Environment variables for Octane
ENV OCTANE_SERVER=frankenphp
ENV APP_ENV=production
ENV LOG_CHANNEL=stderr

# Use entrypoint script
ENTRYPOINT ["/app/docker-entrypoint.sh"]

# -----------------------------------------------------------------------------
# Stage 3: Development Image (optional)
# -----------------------------------------------------------------------------
FROM dunglas/frankenphp:1-php8.3-alpine AS development

# Install all dependencies including dev tools
RUN apk add --no-cache \
    git \
    curl \
    libpng-dev \
    libjpeg-turbo-dev \
    freetype-dev \
    libwebp-dev \
    libzip-dev \
    libxml2-dev \
    postgresql-dev \
    icu-dev \
    oniguruma-dev \
    linux-headers \
    nodejs \
    npm \
    $PHPIZE_DEPS

# Configure and install PHP extensions
RUN docker-php-ext-configure gd \
        --with-freetype \
        --with-jpeg \
        --with-webp \
    && docker-php-ext-install -j$(nproc) \
        pdo \
        pdo_pgsql \
        pgsql \
        mbstring \
        xml \
        bcmath \
        gd \
        zip \
        intl \
        pcntl \
        opcache \
        sockets

# Install Redis and Xdebug for development
RUN pecl install redis xdebug \
    && docker-php-ext-enable redis xdebug

WORKDIR /app

# Install Composer
COPY --from=composer:2.8 /usr/bin/composer /usr/bin/composer

# Default to development mode
ENV APP_ENV=local
ENV APP_DEBUG=true
ENV OCTANE_SERVER=frankenphp

EXPOSE 8000

CMD ["php", "artisan", "octane:start", "--server=frankenphp", "--host=0.0.0.0", "--port=8000", "--watch"]
