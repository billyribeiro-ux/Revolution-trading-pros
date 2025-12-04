<?php

/**
 * Rate Limiting Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Configuration for the GlobalApiRateLimiter middleware.
 * Adjust these settings based on your application's needs.
 */

return [
    /*
    |--------------------------------------------------------------------------
    | Whitelisted IPs
    |--------------------------------------------------------------------------
    |
    | IP addresses that bypass rate limiting entirely. Useful for:
    | - Internal services
    | - Monitoring systems
    | - Trusted partners
    |
    */
    'whitelist' => array_filter(array_map('trim', explode(',', env('RATE_LIMIT_WHITELIST', '127.0.0.1,::1')))),

    /*
    |--------------------------------------------------------------------------
    | Rate Limits by Endpoint Type
    |--------------------------------------------------------------------------
    |
    | Define requests per window (in seconds) for different endpoint categories.
    |
    */
    'limits' => [
        'default' => [
            'requests' => (int) env('RATE_LIMIT_DEFAULT_REQUESTS', 100),
            'window' => (int) env('RATE_LIMIT_DEFAULT_WINDOW', 60),
        ],
        'auth' => [
            'requests' => (int) env('RATE_LIMIT_AUTH_REQUESTS', 10),
            'window' => (int) env('RATE_LIMIT_AUTH_WINDOW', 60),
        ],
        'newsletter' => [
            'requests' => (int) env('RATE_LIMIT_NEWSLETTER_REQUESTS', 5),
            'window' => (int) env('RATE_LIMIT_NEWSLETTER_WINDOW', 60),
        ],
        'email' => [
            'requests' => (int) env('RATE_LIMIT_EMAIL_REQUESTS', 30),
            'window' => (int) env('RATE_LIMIT_EMAIL_WINDOW', 60),
        ],
        'webhook' => [
            'requests' => (int) env('RATE_LIMIT_WEBHOOK_REQUESTS', 200),
            'window' => (int) env('RATE_LIMIT_WEBHOOK_WINDOW', 60),
        ],
        'admin' => [
            'requests' => (int) env('RATE_LIMIT_ADMIN_REQUESTS', 200),
            'window' => (int) env('RATE_LIMIT_ADMIN_WINDOW', 60),
        ],
        'public' => [
            'requests' => (int) env('RATE_LIMIT_PUBLIC_REQUESTS', 60),
            'window' => (int) env('RATE_LIMIT_PUBLIC_WINDOW', 60),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Burst Limits
    |--------------------------------------------------------------------------
    |
    | Short-term spike protection. Prevents rapid fire requests.
    |
    */
    'burst' => [
        'default' => [
            'requests' => (int) env('BURST_LIMIT_DEFAULT_REQUESTS', 20),
            'window' => (int) env('BURST_LIMIT_DEFAULT_WINDOW', 5),
        ],
        'auth' => [
            'requests' => (int) env('BURST_LIMIT_AUTH_REQUESTS', 5),
            'window' => (int) env('BURST_LIMIT_AUTH_WINDOW', 10),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Daily Global Limit
    |--------------------------------------------------------------------------
    |
    | Maximum requests per IP per day across all endpoints.
    |
    */
    'daily_limit' => (int) env('RATE_LIMIT_DAILY', 10000),

    /*
    |--------------------------------------------------------------------------
    | Suspicious Activity Detection
    |--------------------------------------------------------------------------
    |
    | Settings for detecting and handling suspicious behavior.
    |
    */
    'suspicious' => [
        // Number of rate limit violations before flagging
        'threshold' => (int) env('SUSPICIOUS_THRESHOLD', 5),

        // Ban duration in seconds (default: 1 hour)
        'ban_duration' => (int) env('BAN_DURATION', 3600),
    ],

    /*
    |--------------------------------------------------------------------------
    | Logging
    |--------------------------------------------------------------------------
    |
    | Enable detailed logging for rate limiting events.
    |
    */
    'logging' => [
        'enabled' => env('RATE_LIMIT_LOGGING', true),
        'channel' => env('RATE_LIMIT_LOG_CHANNEL', 'stack'),
    ],
];
