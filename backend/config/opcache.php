<?php

declare(strict_types=1);

/**
 * OPcache and JIT Configuration (ICT9+ Enterprise Grade)
 *
 * Optimal settings for production Laravel application
 *
 * Apply these settings in php.ini or as CLI arguments
 *
 * @version 1.0.0
 */

return [
    /*
    |--------------------------------------------------------------------------
    | OPcache Configuration
    |--------------------------------------------------------------------------
    |
    | These are the recommended OPcache settings for production
    |
    */
    'opcache' => [
        // Enable OPcache
        'opcache.enable' => 1,
        'opcache.enable_cli' => 1,

        // Memory allocation (increase for larger apps)
        'opcache.memory_consumption' => 256, // MB
        'opcache.interned_strings_buffer' => 64, // MB

        // Maximum cached files
        'opcache.max_accelerated_files' => 50000,

        // Validation settings (disable in production)
        'opcache.validate_timestamps' => 0, // Set to 1 in development
        'opcache.revalidate_freq' => 0,     // Seconds (only if validate_timestamps=1)

        // Optimization level (max performance)
        'opcache.optimization_level' => 0x7FFEBFFF,

        // File cache (persist across restarts)
        'opcache.file_cache' => '/tmp/opcache',
        'opcache.file_cache_only' => 0,
        'opcache.file_cache_consistency_checks' => 1,

        // Preloading (PHP 7.4+)
        'opcache.preload' => base_path('bootstrap/preload.php'),
        'opcache.preload_user' => 'www-data',

        // Fast shutdown
        'opcache.fast_shutdown' => 1,

        // Save comments (needed for annotations)
        'opcache.save_comments' => 1,
    ],

    /*
    |--------------------------------------------------------------------------
    | JIT Configuration (PHP 8.0+)
    |--------------------------------------------------------------------------
    |
    | JIT compilation for additional performance boost
    |
    */
    'jit' => [
        // JIT buffer size
        'opcache.jit_buffer_size' => '128M',

        // JIT mode
        // 1205 = tracing JIT (best for web)
        // 1235 = function JIT (best for CLI)
        'opcache.jit' => 1205,

        // Debug (disable in production)
        'opcache.jit_debug' => 0,
    ],

    /*
    |--------------------------------------------------------------------------
    | Realpath Cache
    |--------------------------------------------------------------------------
    |
    | Cache for resolved file paths
    |
    */
    'realpath_cache' => [
        'realpath_cache_size' => '4096K',
        'realpath_cache_ttl' => 600,
    ],

    /*
    |--------------------------------------------------------------------------
    | Production php.ini recommendations
    |--------------------------------------------------------------------------
    |
    | Copy these to your php.ini for optimal performance
    |
    */
    'php_ini_recommendations' => <<<'INI'
; OPcache settings
opcache.enable=1
opcache.enable_cli=1
opcache.memory_consumption=256
opcache.interned_strings_buffer=64
opcache.max_accelerated_files=50000
opcache.validate_timestamps=0
opcache.revalidate_freq=0
opcache.optimization_level=0x7FFEBFFF
opcache.fast_shutdown=1
opcache.save_comments=1

; JIT settings (PHP 8.0+)
opcache.jit_buffer_size=128M
opcache.jit=1205

; Realpath cache
realpath_cache_size=4096K
realpath_cache_ttl=600

; Memory and execution
memory_limit=512M
max_execution_time=60
max_input_time=60
max_input_vars=5000

; Session handling (use Redis in production)
session.save_handler=redis
session.save_path="tcp://127.0.0.1:6379?auth=&database=1"

; Error handling (production)
display_errors=Off
log_errors=On
error_reporting=E_ALL & ~E_DEPRECATED & ~E_STRICT

; File uploads
upload_max_filesize=64M
post_max_size=64M

; Output buffering
output_buffering=4096
implicit_flush=Off
INI,
];
