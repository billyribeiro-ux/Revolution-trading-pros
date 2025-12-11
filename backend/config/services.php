<?php

return [

    /*
    |--------------------------------------------------------------------------
    | Third Party Services
    |--------------------------------------------------------------------------
    |
    | This file is for storing the credentials for third party services such
    | as Mailgun, Postmark, AWS and more. This file provides the de facto
    | location for this type of information, allowing packages to have
    | a conventional file to locate the various service credentials.
    |
    */

    'postmark' => [
        'key' => env('POSTMARK_API_KEY'),
    ],

    'resend' => [
        'key' => env('RESEND_API_KEY'),
    ],

    'ses' => [
        'key' => env('AWS_ACCESS_KEY_ID'),
        'secret' => env('AWS_SECRET_ACCESS_KEY'),
        'region' => env('AWS_DEFAULT_REGION', 'us-east-1'),
    ],

    'slack' => [
        'notifications' => [
            'bot_user_oauth_token' => env('SLACK_BOT_USER_OAUTH_TOKEN'),
            'channel' => env('SLACK_BOT_USER_DEFAULT_CHANNEL'),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Stripe Payment Gateway
    |--------------------------------------------------------------------------
    |
    | Configuration for Stripe payment processing. Supports both test and
    | live modes. Matches Fluent Cart Pro payment gateway features.
    |
    */
    'stripe' => [
        'test_mode' => env('STRIPE_TEST_MODE', true),
        'secret_key' => env('STRIPE_SECRET_KEY'),
        'publishable_key' => env('STRIPE_PUBLISHABLE_KEY'),
        'test_secret_key' => env('STRIPE_TEST_SECRET_KEY', env('STRIPE_SECRET_KEY')),
        'test_publishable_key' => env('STRIPE_TEST_PUBLISHABLE_KEY', env('STRIPE_PUBLISHABLE_KEY')),
        'webhook_secret' => env('STRIPE_WEBHOOK_SECRET'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Sharp Image Processing Service
    |--------------------------------------------------------------------------
    |
    | High-performance Node.js image processing service using Sharp (libvips).
    | 10-15x faster than PHP-based image processing.
    |
    | Features:
    | - WebP/AVIF conversion
    | - Responsive image generation
    | - BlurHash placeholders
    | - Cloudflare R2 integration
    |
    */
    'sharp' => [
        'enabled' => env('SHARP_SERVICE_ENABLED', true),
        'url' => env('SHARP_SERVICE_URL', 'http://localhost:3001'),
        'timeout' => env('SHARP_SERVICE_TIMEOUT', 60),
    ],

    /*
    |--------------------------------------------------------------------------
    | Cloudflare Services (LIGHTNING STACK)
    |--------------------------------------------------------------------------
    |
    | Configuration for Cloudflare services including R2 Storage, Images,
    | Workers, and KV. These services provide edge-optimized performance.
    |
    | Setup Guide: docs/LIGHTNING_STACK_SETUP.md
    |
    */

    'cloudflare' => [
        // Account ID - Found in Cloudflare Dashboard URL or Overview page
        'account_id' => env('CLOUDFLARE_ACCOUNT_ID'),

        // Global API Token - For general API access
        'api_token' => env('CLOUDFLARE_API_TOKEN'),

        /*
        |----------------------------------------------------------------------
        | Cloudflare Images
        |----------------------------------------------------------------------
        | High-performance image delivery with automatic optimization.
        | - Auto WebP/AVIF conversion
        | - On-the-fly resizing
        | - Global CDN (<20ms delivery)
        |
        | Get credentials: https://dash.cloudflare.com > Images
        |
        */
        'images_enabled' => env('CLOUDFLARE_IMAGES_ENABLED', false),
        'images_token' => env('CLOUDFLARE_IMAGES_TOKEN'),
        'images_hash' => env('CLOUDFLARE_IMAGES_HASH'),

        /*
        |----------------------------------------------------------------------
        | Cloudflare R2 Storage
        |----------------------------------------------------------------------
        | S3-compatible object storage with zero egress fees.
        | Already configured in config/filesystems.php
        |
        | Get credentials: https://dash.cloudflare.com > R2
        |
        */
        'r2_enabled' => env('CLOUDFLARE_R2_ENABLED', false),

        /*
        |----------------------------------------------------------------------
        | Cloudflare Workers KV (Optional)
        |----------------------------------------------------------------------
        | Edge key-value storage for caching at 300+ locations.
        |
        */
        'kv_enabled' => env('CLOUDFLARE_KV_ENABLED', false),
        'kv_namespace_id' => env('CLOUDFLARE_KV_NAMESPACE_ID'),

        /*
        |----------------------------------------------------------------------
        | Cloudflare Workers (Optional)
        |----------------------------------------------------------------------
        | Edge compute for API responses at edge locations.
        |
        */
        'workers_enabled' => env('CLOUDFLARE_WORKERS_ENABLED', false),
        'workers_route' => env('CLOUDFLARE_WORKERS_ROUTE'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Upstash Redis (LIGHTNING STACK)
    |--------------------------------------------------------------------------
    |
    | Serverless Redis with global replication for <1ms reads.
    | Configuration is in config/database.php (redis.upstash)
    |
    | Get credentials: https://console.upstash.com
    |
    */

    'upstash' => [
        'enabled' => env('UPSTASH_ENABLED', false),
        'rest_url' => env('UPSTASH_REDIS_REST_URL'),
        'rest_token' => env('UPSTASH_REDIS_REST_TOKEN'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Neon PostgreSQL (LIGHTNING STACK)
    |--------------------------------------------------------------------------
    |
    | Serverless PostgreSQL with autoscaling and branching.
    | Configuration is in config/database.php (neon)
    |
    | Get credentials: https://console.neon.tech
    |
    */

    'neon' => [
        'enabled' => env('NEON_ENABLED', false),
        'project_id' => env('NEON_PROJECT_ID'),
    ],

];
