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

    /*
    |--------------------------------------------------------------------------
    | Postmark Email Service
    |--------------------------------------------------------------------------
    |
    | Transactional email service with excellent deliverability.
    | Get your API key: https://postmarkapp.com
    |
    | Pricing: Free (100/mo) → $15/mo (10K emails)
    |
    */
    'postmark' => [
        'token' => env('POSTMARK_TOKEN'),
        'message_stream_id' => env('POSTMARK_MESSAGE_STREAM_ID', 'outbound'),
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
    | Configuration for Stripe payment processing.
    | Get your API keys: https://dashboard.stripe.com/apikeys
    |
    | Webhook endpoint: /api/webhooks/stripe
    | Pricing: 2.9% + $0.30 per transaction
    |
    | Setup:
    | 1. Get keys from Stripe Dashboard → Developers → API Keys
    | 2. Create webhook at Developers → Webhooks → Add endpoint
    | 3. Select events: payment_intent.*, customer.*, invoice.*
    |
    */
    'stripe' => [
        'key' => env('STRIPE_KEY'),                    // pk_live_xxx or pk_test_xxx
        'secret' => env('STRIPE_SECRET'),              // sk_live_xxx or sk_test_xxx
        'webhook_secret' => env('STRIPE_WEBHOOK_SECRET'), // whsec_xxx

        // Legacy keys (for backwards compatibility)
        'test_mode' => env('STRIPE_TEST_MODE', false),
        'secret_key' => env('STRIPE_SECRET'),
        'publishable_key' => env('STRIPE_KEY'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Sentry Error Tracking
    |--------------------------------------------------------------------------
    |
    | Real-time error tracking and performance monitoring.
    | Get your DSN: https://sentry.io
    |
    | Pricing: Free (5K errors/mo) → $26/mo (Team)
    |
    | Setup:
    | 1. Create account at sentry.io
    | 2. Create Laravel project
    | 3. Copy DSN from Settings → Client Keys
    |
    */
    'sentry' => [
        'dsn' => env('SENTRY_LARAVEL_DSN'),
        'traces_sample_rate' => env('SENTRY_TRACES_SAMPLE_RATE', 0.1),
        'profiles_sample_rate' => env('SENTRY_PROFILES_SAMPLE_RATE', 0.1),
        'send_default_pii' => env('SENTRY_SEND_DEFAULT_PII', false),
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
        | Cloudflare Images (NOT USED - Using Sharp Service Instead)
        |----------------------------------------------------------------------
        | We use the built-in Sharp image processing service which provides:
        | - WebP/AVIF conversion at upload time (more efficient)
        | - BlurHash placeholders
        | - Responsive image generation
        | - Zero monthly cost
        |
        | See: backend/services/sharp-image-processor/
        | Config: services.sharp
        |
        */
        'images_enabled' => false, // Disabled - using Sharp service

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
