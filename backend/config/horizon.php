<?php

declare(strict_types=1);

/**
 * Laravel Horizon Configuration
 *
 * Queue Management Dashboard (December 2025)
 * @see https://laravel.com/docs/12.x/horizon
 */

use Illuminate\Support\Str;

return [

    /*
    |--------------------------------------------------------------------------
    | Horizon Domain
    |--------------------------------------------------------------------------
    |
    | This is the subdomain where Horizon will be accessible from. If this
    | setting is null, Horizon will reside under the same domain as the
    | application. Otherwise, this value will serve as the subdomain.
    |
    */

    'domain' => env('HORIZON_DOMAIN'),

    /*
    |--------------------------------------------------------------------------
    | Horizon Path
    |--------------------------------------------------------------------------
    |
    | This is the URI path where Horizon will be accessible from. Feel free
    | to change this path to anything you like. Note that the URI will not
    | affect the paths of its internal API that aren't exposed to users.
    |
    */

    'path' => env('HORIZON_PATH', 'horizon'),

    /*
    |--------------------------------------------------------------------------
    | Horizon Redis Connection
    |--------------------------------------------------------------------------
    |
    | This is the name of the Redis connection where Horizon will store the
    | meta information required for it to function. It includes the list
    | of supervisors, failed jobs, job metrics, and other information.
    |
    */

    'use' => 'default',

    /*
    |--------------------------------------------------------------------------
    | Horizon Redis Prefix
    |--------------------------------------------------------------------------
    |
    | This prefix will be used when storing all Horizon data in Redis. You
    | may modify the prefix when you are running multiple installations
    | of Horizon on the same server so that they don't have problems.
    |
    */

    'prefix' => env(
        'HORIZON_PREFIX',
        Str::slug(env('APP_NAME', 'laravel'), '_').'_horizon:'
    ),

    /*
    |--------------------------------------------------------------------------
    | Horizon Route Middleware
    |--------------------------------------------------------------------------
    |
    | These middleware will get attached onto each Horizon route, giving you
    | the chance to add your own middleware to this list or change any of
    | the existing middleware. Or, you can simply stick with this list.
    |
    */

    'middleware' => ['web'],

    /*
    |--------------------------------------------------------------------------
    | Queue Wait Time Thresholds
    |--------------------------------------------------------------------------
    |
    | This option allows you to configure when the LongWaitDetected event
    | will be fired. Every connection / queue combination may have its
    | own, unique threshold (in seconds) before this event is fired.
    |
    */

    'waits' => [
        'redis:default' => 60,
        'redis:high' => 30,
        'redis:emails' => 120,
    ],

    /*
    |--------------------------------------------------------------------------
    | Job Trimming Times
    |--------------------------------------------------------------------------
    |
    | Here you can configure for how long (in minutes) you desire Horizon to
    | persist the recent and failed jobs. Typically, recent jobs are kept
    | for one hour while all failed jobs are stored for an entire week.
    |
    */

    'trim' => [
        'recent' => 60,         // 1 hour
        'pending' => 60,        // 1 hour
        'completed' => 60,      // 1 hour
        'recent_failed' => 10080, // 7 days
        'failed' => 10080,      // 7 days
        'monitored' => 10080,   // 7 days
    ],

    /*
    |--------------------------------------------------------------------------
    | Silenced Jobs
    |--------------------------------------------------------------------------
    |
    | Jobs listed here will not appear in the Horizon dashboard. This is
    | useful for high-frequency jobs that would otherwise clutter the UI.
    |
    */

    'silenced' => [
        // App\Jobs\PruneOldRecords::class,
    ],

    /*
    |--------------------------------------------------------------------------
    | Metrics
    |--------------------------------------------------------------------------
    |
    | Here you can configure how many snapshots should be kept to display in
    | the metrics graph. This will get used in combination with Horizon's
    | `horizon:snapshot` schedule to define how long to retain metrics.
    |
    */

    'metrics' => [
        'trim_snapshots' => [
            'job' => 24,     // 24 hours
            'queue' => 24,   // 24 hours
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Fast Termination
    |--------------------------------------------------------------------------
    |
    | When this option is enabled, Horizon's "terminate" command will not
    | wait on all of the workers to terminate unless the --wait option
    | is provided. Fast termination can shorten deployment wait times.
    |
    */

    'fast_termination' => false,

    /*
    |--------------------------------------------------------------------------
    | Memory Limit (MB)
    |--------------------------------------------------------------------------
    |
    | This value describes the maximum amount of memory the Horizon master
    | supervisor may consume before it is terminated and restarted. For
    | configuring these limits on your workers, see the next section.
    |
    */

    'memory_limit' => 128,

    /*
    |--------------------------------------------------------------------------
    | Queue Worker Configuration
    |--------------------------------------------------------------------------
    |
    | Here you may define the queue worker settings used by your application
    | in all environments. These supervisors and settings handle all your
    | queued jobs and will be provisioned by Horizon during deployment.
    |
    */

    'defaults' => [
        'supervisor-1' => [
            'connection' => 'redis',
            'queue' => ['default'],
            'balance' => 'auto',
            'autoScalingStrategy' => 'time',
            'maxProcesses' => 10,
            'maxTime' => 0,
            'maxJobs' => 0,
            'memory' => 256,
            'tries' => 3,
            'timeout' => 60,
            'nice' => 0,
        ],
    ],

    'environments' => [
        'production' => [
            'supervisor-default' => [
                'connection' => 'redis',
                'queue' => ['default'],
                'balance' => 'auto',
                'autoScalingStrategy' => 'time',
                'minProcesses' => 2,
                'maxProcesses' => 10,
                'balanceMaxShift' => 1,
                'balanceCooldown' => 3,
                'memory' => 256,
                'tries' => 3,
                'timeout' => 90,
                'nice' => 0,
            ],
            'supervisor-high' => [
                'connection' => 'redis',
                'queue' => ['high'],
                'balance' => 'auto',
                'autoScalingStrategy' => 'time',
                'minProcesses' => 1,
                'maxProcesses' => 5,
                'memory' => 256,
                'tries' => 3,
                'timeout' => 60,
                'nice' => 0,
            ],
            'supervisor-emails' => [
                'connection' => 'redis',
                'queue' => ['emails', 'notifications'],
                'balance' => 'auto',
                'autoScalingStrategy' => 'size',
                'minProcesses' => 1,
                'maxProcesses' => 8,
                'memory' => 256,
                'tries' => 3,
                'timeout' => 120,
                'nice' => 0,
            ],
            'supervisor-long-running' => [
                'connection' => 'redis',
                'queue' => ['long-running', 'images', 'exports'],
                'balance' => 'simple',
                'minProcesses' => 1,
                'maxProcesses' => 3,
                'memory' => 512,
                'tries' => 1,
                'timeout' => 600, // 10 minutes
                'nice' => 10,
            ],
        ],

        'staging' => [
            'supervisor-default' => [
                'connection' => 'redis',
                'queue' => ['default', 'high', 'emails'],
                'balance' => 'auto',
                'minProcesses' => 1,
                'maxProcesses' => 3,
                'memory' => 256,
                'tries' => 3,
                'timeout' => 90,
                'nice' => 0,
            ],
        ],

        'local' => [
            'supervisor-default' => [
                'connection' => 'redis',
                'queue' => ['default', 'high', 'emails', 'long-running'],
                'balance' => 'simple',
                'minProcesses' => 1,
                'maxProcesses' => 3,
                'memory' => 256,
                'tries' => 1,
                'timeout' => 60,
                'nice' => 0,
            ],
        ],
    ],
];
