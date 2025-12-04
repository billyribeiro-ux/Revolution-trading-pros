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

];
