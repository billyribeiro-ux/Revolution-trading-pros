<?php

/**
 * Sentry Error Tracking Configuration
 *
 * Real-time error tracking and performance monitoring.
 * Get your DSN: https://sentry.io
 *
 * December 2025 - Revolution Trading Pros
 */

return [

    /*
    |--------------------------------------------------------------------------
    | Sentry DSN
    |--------------------------------------------------------------------------
    |
    | The DSN tells Sentry where to send the events. Copy this from:
    | sentry.io → Your Project → Settings → Client Keys (DSN)
    |
    */

    'dsn' => env('SENTRY_LARAVEL_DSN'),

    /*
    |--------------------------------------------------------------------------
    | Release Version
    |--------------------------------------------------------------------------
    |
    | The version of your application for release tracking.
    | Automatically detected from git if not specified.
    |
    */

    'release' => env('SENTRY_RELEASE', null),

    /*
    |--------------------------------------------------------------------------
    | Environment
    |--------------------------------------------------------------------------
    |
    | The environment name for your application.
    | Typically: production, staging, local
    |
    */

    'environment' => env('SENTRY_ENVIRONMENT', env('APP_ENV', 'production')),

    /*
    |--------------------------------------------------------------------------
    | Sample Rates
    |--------------------------------------------------------------------------
    |
    | Control how much data is sent to Sentry:
    | - traces_sample_rate: Performance monitoring (0.0 to 1.0)
    | - profiles_sample_rate: Profiling data (0.0 to 1.0)
    |
    | Recommended for production:
    | - traces: 0.1 (10% of requests)
    | - profiles: 0.1 (10% of traced requests)
    |
    */

    'traces_sample_rate' => (float) env('SENTRY_TRACES_SAMPLE_RATE', 0.1),

    'profiles_sample_rate' => (float) env('SENTRY_PROFILES_SAMPLE_RATE', 0.1),

    /*
    |--------------------------------------------------------------------------
    | Breadcrumbs
    |--------------------------------------------------------------------------
    |
    | Breadcrumbs are a trail of events that happened prior to an issue.
    | These help you understand what led to the error.
    |
    */

    'breadcrumbs' => [
        // Capture SQL queries
        'sql_queries' => true,

        // Capture query bindings (parameter values)
        'sql_bindings' => env('APP_DEBUG', false),

        // Capture queue job information
        'queue_info' => true,

        // Capture log messages
        'logs' => true,

        // Capture Laravel cache operations
        'cache' => false,

        // Capture HTTP client requests
        'http_client_requests' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | Tracing
    |--------------------------------------------------------------------------
    |
    | Performance monitoring configuration for detailed traces.
    |
    */

    'tracing' => [
        // Continue traces from frontend (browser JS SDK)
        'continue_from_headers' => true,

        // Trace queue jobs
        'queue_job_transactions' => true,

        // Trace console commands
        'command_transactions' => true,

        // Default tracing (HTTP requests)
        'default_integrations' => true,

        // Trace database queries
        'sql_query_origins' => env('APP_DEBUG', false),

        // Set a transaction name for commands
        'console_commands' => [
            'horizon:work',
            'queue:work',
            'schedule:run',
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Personal Identifiable Information (PII)
    |--------------------------------------------------------------------------
    |
    | Control whether user data is sent with events.
    | Set to true only if needed for debugging.
    |
    */

    'send_default_pii' => env('SENTRY_SEND_DEFAULT_PII', false),

    /*
    |--------------------------------------------------------------------------
    | Error Reporting
    |--------------------------------------------------------------------------
    |
    | Control which errors are reported to Sentry.
    |
    */

    // Exceptions to ignore (never send to Sentry)
    'ignore_exceptions' => [
        Illuminate\Auth\AuthenticationException::class,
        Illuminate\Auth\Access\AuthorizationException::class,
        Symfony\Component\HttpKernel\Exception\NotFoundHttpException::class,
        Illuminate\Database\Eloquent\ModelNotFoundException::class,
        Illuminate\Validation\ValidationException::class,
        Illuminate\Session\TokenMismatchException::class,
        Symfony\Component\HttpKernel\Exception\TooManyRequestsHttpException::class,
    ],

    // Transactions to ignore (never trace)
    'ignore_transactions' => [
        'health',
        'health/*',
        'horizon/*',
        'pulse/*',
        'sanctum/csrf-cookie',
    ],

    /*
    |--------------------------------------------------------------------------
    | Context
    |--------------------------------------------------------------------------
    |
    | Additional context sent with every event.
    |
    */

    'tags' => [
        'php_version' => PHP_VERSION,
    ],

    /*
    |--------------------------------------------------------------------------
    | Before Send
    |--------------------------------------------------------------------------
    |
    | A callback executed before sending events to Sentry.
    | Use to filter or modify events.
    |
    | Example (in AppServiceProvider):
    | \Sentry\configureScope(function (\Sentry\State\Scope $scope): void {
    |     $scope->setTag('custom_tag', 'value');
    | });
    |
    */

];
