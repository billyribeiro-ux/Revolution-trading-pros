<?php

use Illuminate\Support\Str;

return [

    /*
    |--------------------------------------------------------------------------
    | Default Cache Store
    |--------------------------------------------------------------------------
    |
    | This option controls the default cache store that will be used by the
    | framework. This connection is utilized if another isn't explicitly
    | specified when running a cache operation inside the application.
    |
    | LIGHTNING STACK: Use 'upstash' for edge-optimized caching
    |
    */

    'default' => env('CACHE_STORE', 'database'),

    /*
    |--------------------------------------------------------------------------
    | Cache Stores
    |--------------------------------------------------------------------------
    |
    | Below you may define all of the cache "stores" for your application as
    | well as their drivers. You may even define multiple stores for the
    | same cache driver to group types of items stored in your caches.
    |
    | Supported drivers: "array", "database", "file", "memcached",
    |                    "redis", "dynamodb", "octane",
    |                    "failover", "null"
    |
    */

    'stores' => [

        'array' => [
            'driver' => 'array',
            'serialize' => false,
        ],

        'database' => [
            'driver' => 'database',
            'connection' => env('DB_CACHE_CONNECTION'),
            'table' => env('DB_CACHE_TABLE', 'cache'),
            'lock_connection' => env('DB_CACHE_LOCK_CONNECTION'),
            'lock_table' => env('DB_CACHE_LOCK_TABLE'),
        ],

        'file' => [
            'driver' => 'file',
            'path' => storage_path('framework/cache/data'),
            'lock_path' => storage_path('framework/cache/data'),
        ],

        'memcached' => [
            'driver' => 'memcached',
            'persistent_id' => env('MEMCACHED_PERSISTENT_ID'),
            'sasl' => [
                env('MEMCACHED_USERNAME'),
                env('MEMCACHED_PASSWORD'),
            ],
            'options' => [
                // Memcached::OPT_CONNECT_TIMEOUT => 2000,
            ],
            'servers' => [
                [
                    'host' => env('MEMCACHED_HOST', '127.0.0.1'),
                    'port' => env('MEMCACHED_PORT', 11211),
                    'weight' => 100,
                ],
            ],
        ],

        'redis' => [
            'driver' => 'redis',
            'connection' => env('REDIS_CACHE_CONNECTION', 'cache'),
            'lock_connection' => env('REDIS_CACHE_LOCK_CONNECTION', 'default'),
        ],

        /*
        |--------------------------------------------------------------------------
        | Upstash Redis - Edge-Optimized Caching (LIGHTNING STACK)
        |--------------------------------------------------------------------------
        |
        | Serverless Redis with global replication for <1ms reads.
        | - Edge-optimized for global low latency
        | - Pay-per-request pricing
        | - Zero maintenance
        | - Automatic failover
        |
        | Get credentials: https://console.upstash.com
        |
        */

        'upstash' => [
            'driver' => 'redis',
            'connection' => 'upstash',
            'lock_connection' => 'upstash',
        ],

        'dynamodb' => [
            'driver' => 'dynamodb',
            'key' => env('AWS_ACCESS_KEY_ID'),
            'secret' => env('AWS_SECRET_ACCESS_KEY'),
            'region' => env('AWS_DEFAULT_REGION', 'us-east-1'),
            'table' => env('DYNAMODB_CACHE_TABLE', 'cache'),
            'endpoint' => env('DYNAMODB_ENDPOINT'),
        ],

        'octane' => [
            'driver' => 'octane',
        ],

        /*
        |--------------------------------------------------------------------------
        | Failover Cache (LIGHTNING STACK - PRODUCTION)
        |--------------------------------------------------------------------------
        |
        | Automatic failover from Upstash Redis to database cache.
        | Ensures high availability even if Redis is unavailable.
        |
        */

        'failover' => [
            'driver' => 'failover',
            'stores' => [
                'upstash',
                'database',
                'array',
            ],
        ],

    ],

    /*
    |--------------------------------------------------------------------------
    | Cache Key Prefix
    |--------------------------------------------------------------------------
    |
    | When utilizing the APC, database, memcached, Redis, and DynamoDB cache
    | stores, there might be other applications using the same cache. For
    | that reason, you may prefix every cache key to avoid collisions.
    |
    */

    'prefix' => env('CACHE_PREFIX', Str::slug((string) env('APP_NAME', 'laravel')).'-cache-'),

    /*
    |--------------------------------------------------------------------------
    | Cache TTL Presets (LIGHTNING STACK)
    |--------------------------------------------------------------------------
    |
    | Predefined TTL values for different content types.
    | Use these for consistent caching across the application.
    |
    */

    'ttl' => [
        'static' => env('CACHE_TTL_STATIC', 86400),      // 24 hours - rarely changing content
        'posts' => env('CACHE_TTL_POSTS', 3600),         // 1 hour - blog posts
        'api' => env('CACHE_TTL_API', 300),              // 5 minutes - API responses
        'session' => env('CACHE_TTL_SESSION', 7200),     // 2 hours - user sessions
        'realtime' => env('CACHE_TTL_REALTIME', 60),     // 1 minute - frequently updated
    ],

];
