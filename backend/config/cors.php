<?php

/**
 * CORS Configuration
 *
 * Uses CORS_ALLOWED_ORIGINS env var (comma-separated list)
 * Example: CORS_ALLOWED_ORIGINS=http://localhost:5173,https://revolution-trading-pros.pages.dev
 */

// Parse CORS_ALLOWED_ORIGINS from environment (comma-separated)
$allowedOrigins = env('CORS_ALLOWED_ORIGINS')
    ? array_map('trim', explode(',', env('CORS_ALLOWED_ORIGINS')))
    : [
        env('FRONTEND_URL', 'https://revolution-trading-pros.pages.dev'),
        'https://revolution-trading-pros.pages.dev',
        'https://revolutiontradingpros.com',
        'https://www.revolutiontradingpros.com',
        'http://localhost:5173',
        'http://localhost:5174',
        'http://127.0.0.1:5173',
        'http://127.0.0.1:5174'
    ];

return [
    'paths' => ['api/*', 'sanctum/csrf-cookie', 'health/*', 'health'],
    'allowed_methods' => ['*'],
    'allowed_origins' => $allowedOrigins,
    'allowed_origins_patterns' => [],
    'allowed_headers' => ['*'],
    'exposed_headers' => [],
    'max_age' => 86400, // Cache preflight for 24 hours
    'supports_credentials' => true,
];
