<?php

declare(strict_types=1);

/**
 * Laravel Octane Configuration
 *
 * Stack: Laravel 12 + FrankenPHP (December 2025)
 * @see https://laravel.com/docs/12.x/octane
 */

use Laravel\Octane\Contracts\OperationTerminated;
use Laravel\Octane\Events\RequestHandled;
use Laravel\Octane\Events\RequestReceived;
use Laravel\Octane\Events\RequestTerminated;
use Laravel\Octane\Events\TaskReceived;
use Laravel\Octane\Events\TaskTerminated;
use Laravel\Octane\Events\TickReceived;
use Laravel\Octane\Events\TickTerminated;
use Laravel\Octane\Events\WorkerErrorOccurred;
use Laravel\Octane\Events\WorkerStarting;
use Laravel\Octane\Events\WorkerStopping;

return [

    /*
    |--------------------------------------------------------------------------
    | Octane Server
    |--------------------------------------------------------------------------
    |
    | FrankenPHP provides the best performance as of December 2025:
    | - 0.88ms response times (vs 2.61ms RoadRunner, 4.94ms Swoole)
    | - Modern Caddy-based architecture
    | - Automatic HTTPS with Let's Encrypt
    | - HTTP/2 and HTTP/3 support
    |
    */

    'server' => env('OCTANE_SERVER', 'frankenphp'),

    /*
    |--------------------------------------------------------------------------
    | Force HTTPS
    |--------------------------------------------------------------------------
    |
    | FrankenPHP can automatically handle HTTPS. Enable this in production
    | when not behind a load balancer that terminates SSL.
    |
    */

    'https' => env('OCTANE_HTTPS', false),

    /*
    |--------------------------------------------------------------------------
    | Octane Listeners
    |--------------------------------------------------------------------------
    |
    | Event listeners for Octane lifecycle events. Use these to warm caches,
    | flush state, or handle errors.
    |
    */

    'listeners' => [
        WorkerStarting::class => [
            // Warm up commonly used services when worker starts
            // App\Listeners\WarmCacheOnWorkerStart::class,
        ],

        RequestReceived::class => [
            //
        ],

        RequestHandled::class => [
            // Log request metrics, analytics, etc.
        ],

        RequestTerminated::class => [
            // Cleanup after request
        ],

        TaskReceived::class => [
            //
        ],

        TaskTerminated::class => [
            // Cleanup after background task
        ],

        TickReceived::class => [
            // Periodic tick handling
        ],

        TickTerminated::class => [
            // Cleanup after tick
        ],

        OperationTerminated::class => [
            Laravel\Octane\Listeners\FlushUploadedFiles::class,
            Laravel\Octane\Listeners\FlushTemporaryContainerInstances::class,
            Laravel\Octane\Listeners\DisconnectFromDatabases::class,
            Laravel\Octane\Listeners\CollectGarbage::class,
        ],

        WorkerErrorOccurred::class => [
            Laravel\Octane\Listeners\ReportException::class,
            Laravel\Octane\Listeners\StopWorkerIfNecessary::class,
        ],

        WorkerStopping::class => [
            // Graceful shutdown tasks
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Warm / Flush Bindings
    |--------------------------------------------------------------------------
    |
    | Services to resolve when a worker starts (warm) or re-resolve on each
    | request (flush). This optimizes memory and prevents stale state.
    |
    */

    'warm' => [
        // Services to resolve once when the worker boots
        // These stay in memory across requests
        // 'hash',
        // 'router',
    ],

    'flush' => [
        // Services to re-resolve on every request
        // Critical for stateful services
        'auth',
        'session',
        'session.store',
        'cookie',
        'translator',
        // 'url' and 'request' removed - causes null request error
    ],

    /*
    |--------------------------------------------------------------------------
    | Persistent Database Connections
    |--------------------------------------------------------------------------
    |
    | Keep database connections open across requests for better performance.
    | Note: Some edge cases may require connection refresh.
    |
    */

    'persistent_database_connections' => env('OCTANE_PERSISTENT_DB', true),

    /*
    |--------------------------------------------------------------------------
    | Garbage Collection
    |--------------------------------------------------------------------------
    |
    | Configure garbage collection to prevent memory leaks.
    | Lower threshold = more aggressive collection = more CPU, less memory.
    |
    */

    'garbage' => 50, // Collect garbage every 50 requests

    /*
    |--------------------------------------------------------------------------
    | Max Execution Time
    |--------------------------------------------------------------------------
    |
    | Maximum time a single request can run before being terminated.
    | Set lower than PHP's max_execution_time.
    |
    */

    'max_execution_time' => 30, // seconds

    /*
    |--------------------------------------------------------------------------
    | Octane Cache
    |--------------------------------------------------------------------------
    |
    | High-performance in-memory cache that persists across requests.
    | Use for hot data like feature flags, config, etc.
    |
    */

    'cache' => [
        'enabled' => env('OCTANE_CACHE_ENABLED', true),
        'rows' => 1000,
        'bytes' => 10000,
    ],

    /*
    |--------------------------------------------------------------------------
    | Octane Tables
    |--------------------------------------------------------------------------
    |
    | Shared memory tables for cross-request data. Useful for counters,
    | rate limiting, and other shared state.
    |
    */

    'tables' => [
        // Example: Rate limiting table
        // 'rate_limits' => [
        //     'rows' => 1000,
        //     'columns' => [
        //         ['name' => 'hits', 'type' => 'int'],
        //         ['name' => 'reset_at', 'type' => 'int'],
        //     ],
        // ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Concurrent Tasks
    |--------------------------------------------------------------------------
    |
    | Configure concurrent task execution for CPU-bound operations.
    |
    */

    'concurrent_tasks' => [
        'max' => env('OCTANE_CONCURRENT_TASKS', 10),
        'timeout' => 30, // seconds
    ],

    /*
    |--------------------------------------------------------------------------
    | File Watching (Development Only)
    |--------------------------------------------------------------------------
    |
    | Automatically reload when files change. Uses chokidar under the hood.
    | Only enable in development!
    |
    */

    'watch' => [
        'app',
        'bootstrap',
        'config',
        'database',
        'public/**/*.php',
        'resources/**/*.php',
        'routes',
        'composer.lock',
        '.env',
    ],

    /*
    |--------------------------------------------------------------------------
    | State Handling
    |--------------------------------------------------------------------------
    |
    | Configure how Octane handles application state between requests.
    |
    */

    'state' => [
        // Reset the auth state on each request
        'reset_auth' => true,

        // Reset the database query log
        'reset_query_log' => true,

        // Reset all resolved instances (safe mode)
        'reset_instances' => false,
    ],

];
