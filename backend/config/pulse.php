<?php

declare(strict_types=1);

/**
 * Laravel Pulse Configuration
 *
 * Application Performance Monitoring (December 2025)
 * @see https://laravel.com/docs/12.x/pulse
 */

use Laravel\Pulse\Http\Middleware\Authorize;
use Laravel\Pulse\Pulse;
use Laravel\Pulse\Recorders;

return [

    /*
    |--------------------------------------------------------------------------
    | Pulse Domain
    |--------------------------------------------------------------------------
    |
    | This is the subdomain where Pulse will be accessible from. If this
    | setting is null, Pulse will reside under the same domain as the
    | application.
    |
    */

    'domain' => env('PULSE_DOMAIN'),

    /*
    |--------------------------------------------------------------------------
    | Pulse Path
    |--------------------------------------------------------------------------
    |
    | This is the URI path where Pulse will be accessible from. Feel free
    | to change this path to anything you like.
    |
    */

    'path' => env('PULSE_PATH', 'pulse'),

    /*
    |--------------------------------------------------------------------------
    | Pulse Master Switch
    |--------------------------------------------------------------------------
    |
    | This option determines if Pulse is enabled. When disabled, no data
    | will be recorded and the dashboard will be inaccessible.
    |
    */

    'enabled' => env('PULSE_ENABLED', true),

    /*
    |--------------------------------------------------------------------------
    | Pulse Storage Driver
    |--------------------------------------------------------------------------
    |
    | This option controls the storage driver used by Pulse. The database
    | driver is recommended for production. Redis can be used for higher
    | throughput environments.
    |
    */

    'storage' => [
        'driver' => env('PULSE_STORAGE_DRIVER', 'database'),

        'database' => [
            'connection' => env('PULSE_DB_CONNECTION'),
            'chunk' => 1000,
        ],

        'redis' => [
            'connection' => env('PULSE_REDIS_CONNECTION', 'default'),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Pulse Ingest Driver
    |--------------------------------------------------------------------------
    |
    | This option controls the ingest driver used by Pulse. The 'storage'
    | driver ingests data directly, while 'redis' queues data for async
    | processing (recommended for high-traffic sites).
    |
    */

    'ingest' => [
        'driver' => env('PULSE_INGEST_DRIVER', 'storage'),

        'redis' => [
            'connection' => env('PULSE_INGEST_REDIS_CONNECTION', 'default'),
            'chunk' => 1000,
        ],

        'buffer' => env('PULSE_INGEST_BUFFER', 5000),

        'trim' => [
            'lottery' => [1, 1000],
            'keep' => '7 days',
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Pulse Cache Driver
    |--------------------------------------------------------------------------
    |
    | This option controls the cache driver used by Pulse to store dashboard
    | statistics. Using Redis is recommended for production.
    |
    */

    'cache' => env('PULSE_CACHE_DRIVER'),

    /*
    |--------------------------------------------------------------------------
    | Pulse Route Middleware
    |--------------------------------------------------------------------------
    |
    | These middleware will get attached onto each Pulse route, giving you
    | the chance to add your own middleware to this list.
    |
    */

    'middleware' => [
        'web',
        Authorize::class,
    ],

    /*
    |--------------------------------------------------------------------------
    | Pulse Recorders
    |--------------------------------------------------------------------------
    |
    | Configure which recorders Pulse should use to collect application
    | metrics. Each recorder can be individually enabled/disabled.
    |
    */

    'recorders' => [
        // Application Performance
        Recorders\Requests::class => [
            'enabled' => env('PULSE_REQUESTS_ENABLED', true),
            'sample_rate' => env('PULSE_REQUESTS_SAMPLE_RATE', 1),
            'ignore' => [
                '#^/pulse$#',
                '#^/horizon#',
                '#^/telescope#',
                '#^/_debugbar#',
                '#^/api/health#',
            ],
        ],

        // Slow Queries
        Recorders\SlowQueries::class => [
            'enabled' => env('PULSE_SLOW_QUERIES_ENABLED', true),
            'threshold' => env('PULSE_SLOW_QUERY_THRESHOLD', 100), // ms
            'sample_rate' => env('PULSE_SLOW_QUERIES_SAMPLE_RATE', 1),
            'ignore' => [
                '/telescope/',
                '/pulse/',
            ],
        ],

        // Slow Jobs
        Recorders\SlowJobs::class => [
            'enabled' => env('PULSE_SLOW_JOBS_ENABLED', true),
            'threshold' => env('PULSE_SLOW_JOB_THRESHOLD', 1000), // ms
            'sample_rate' => 1,
        ],

        // Slow Outgoing Requests
        Recorders\SlowOutgoingRequests::class => [
            'enabled' => env('PULSE_SLOW_OUTGOING_ENABLED', true),
            'threshold' => env('PULSE_SLOW_OUTGOING_THRESHOLD', 500), // ms
            'sample_rate' => 1,
            'groups' => [
                '#https://api\.stripe\.com/#' => 'Stripe API',
                '#https://api\.paypal\.com/#' => 'PayPal API',
                '#https://.*\.sentry\.io/#' => 'Sentry',
                '#https://api\.openai\.com/#' => 'OpenAI',
                '#https://.*\.r2\.cloudflarestorage\.com/#' => 'Cloudflare R2',
            ],
        ],

        // Exception Tracking
        Recorders\Exceptions::class => [
            'enabled' => env('PULSE_EXCEPTIONS_ENABLED', true),
            'sample_rate' => 1,
            'ignore' => [
                '/Illuminate\\\\Auth\\\\AuthenticationException/',
                '/Illuminate\\\\Validation\\\\ValidationException/',
                '/Symfony\\\\Component\\\\HttpKernel\\\\Exception\\\\NotFoundHttpException/',
            ],
        ],

        // Queue Metrics
        Recorders\Queues::class => [
            'enabled' => env('PULSE_QUEUES_ENABLED', true),
            'sample_rate' => 1,
        ],

        // Cache Statistics
        Recorders\CacheInteractions::class => [
            'enabled' => env('PULSE_CACHE_ENABLED', true),
            'sample_rate' => env('PULSE_CACHE_SAMPLE_RATE', 0.1), // 10%
            'groups' => [
                '/^posts:/' => 'Posts Cache',
                '/^users:/' => 'Users Cache',
                '/^sessions:/' => 'Sessions',
                '/^config:/' => 'Config Cache',
            ],
        ],

        // Server Resources
        Recorders\Servers::class => [
            'enabled' => env('PULSE_SERVERS_ENABLED', true),
            'sample_rate' => 1,
        ],

        // User Activity
        Recorders\UserRequests::class => [
            'enabled' => env('PULSE_USER_REQUESTS_ENABLED', true),
            'sample_rate' => env('PULSE_USER_REQUESTS_SAMPLE_RATE', 1),
        ],

        // User Jobs
        Recorders\UserJobs::class => [
            'enabled' => env('PULSE_USER_JOBS_ENABLED', true),
            'sample_rate' => 1,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Pulse Client Resolution
    |--------------------------------------------------------------------------
    |
    | Configure how Pulse identifies unique users/clients for tracking.
    |
    */

    'client' => [
        'resolver' => function ($request) {
            return $request->user()?->id
                ?? $request->ip()
                ?? 'anonymous';
        },
    ],

    /*
    |--------------------------------------------------------------------------
    | Pulse User Resolution
    |--------------------------------------------------------------------------
    |
    | Configure how Pulse resolves user information for display.
    |
    */

    'user' => [
        'resolver' => function ($id) {
            return \App\Models\User::find($id)?->only(['name', 'email']);
        },
    ],

];
