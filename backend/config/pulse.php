<?php

declare(strict_types=1);

/**
 * Laravel Pulse Configuration
 *
 * Application Performance Monitoring (December 2025)
 * @see https://laravel.com/docs/12.x/pulse
 */

use Laravel\Pulse\Http\Middleware\Authorize;

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
        // Recorders are auto-discovered by Pulse
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
