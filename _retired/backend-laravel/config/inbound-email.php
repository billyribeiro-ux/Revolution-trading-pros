<?php

declare(strict_types=1);

return [
    /*
    |--------------------------------------------------------------------------
    | Inbound Email Configuration
    |--------------------------------------------------------------------------
    |
    | Configuration for the inbound email processing system.
    |
    */

    /*
    |--------------------------------------------------------------------------
    | Default Provider
    |--------------------------------------------------------------------------
    |
    | The default email provider to use for inbound processing.
    | Supported: "postmark", "ses", "sendgrid"
    |
    */
    'default_provider' => env('INBOUND_EMAIL_PROVIDER', 'postmark'),

    /*
    |--------------------------------------------------------------------------
    | Webhook Security
    |--------------------------------------------------------------------------
    |
    | Security settings for webhook endpoints.
    |
    */
    'webhook' => [
        // Require signature verification (should be true in production)
        'require_signature' => env('INBOUND_WEBHOOK_REQUIRE_SIGNATURE', true),

        // Rate limit per IP (requests per minute)
        'rate_limit' => env('INBOUND_WEBHOOK_RATE_LIMIT', 60),

        // Allowed IP addresses (empty = allow all)
        'allowed_ips' => array_filter(explode(',', env('INBOUND_WEBHOOK_ALLOWED_IPS', ''))),
    ],

    /*
    |--------------------------------------------------------------------------
    | Postmark Configuration
    |--------------------------------------------------------------------------
    */
    'postmark' => [
        'signature_key' => env('POSTMARK_INBOUND_SIGNATURE_KEY'),
    ],

    /*
    |--------------------------------------------------------------------------
    | AWS SES Configuration
    |--------------------------------------------------------------------------
    */
    'ses' => [
        'sns_topic_arn' => env('SES_INBOUND_SNS_TOPIC_ARN'),
        'verify_sns_signature' => env('SES_VERIFY_SNS_SIGNATURE', true),
    ],

    /*
    |--------------------------------------------------------------------------
    | Spam Detection
    |--------------------------------------------------------------------------
    */
    'spam' => [
        // Spam score threshold (0-10, emails above this are marked as spam)
        'threshold' => env('INBOUND_EMAIL_SPAM_THRESHOLD', 5.0),

        // Auto-quarantine emails above threshold
        'auto_quarantine' => env('INBOUND_EMAIL_AUTO_QUARANTINE', true),
    ],

    /*
    |--------------------------------------------------------------------------
    | Contact Management
    |--------------------------------------------------------------------------
    */
    'contacts' => [
        // Auto-create contacts for unknown senders
        'auto_create' => env('INBOUND_EMAIL_AUTO_CREATE_CONTACTS', true),

        // Default lifecycle stage for auto-created contacts
        'default_lifecycle_stage' => env('INBOUND_EMAIL_DEFAULT_LIFECYCLE_STAGE', 'lead'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Attachment Settings
    |--------------------------------------------------------------------------
    */
    'attachments' => [
        // Maximum attachment size in bytes (25MB default)
        'max_size' => env('INBOUND_EMAIL_MAX_ATTACHMENT_SIZE', 26214400),

        // Storage disk for attachments
        'disk' => env('INBOUND_EMAIL_ATTACHMENTS_DISK', 'r2'),

        // Enable virus scanning (requires ClamAV)
        'virus_scanning' => env('INBOUND_EMAIL_VIRUS_SCANNING', true),

        // ClamAV socket path
        'clamav_socket' => env('CLAMAV_SOCKET', '/var/run/clamav/clamd.ctl'),

        // Blocked file extensions
        'blocked_extensions' => [
            'exe', 'dll', 'bat', 'cmd', 'com', 'pif', 'scr', 'vbs', 'vbe',
            'js', 'jse', 'ws', 'wsf', 'wsc', 'wsh', 'ps1', 'ps1xml', 'ps2',
            'ps2xml', 'psc1', 'psc2', 'msc', 'msi', 'msp', 'mst', 'hta',
            'cpl', 'mshxml', 'gadget', 'application', 'appref-ms',
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Threading Configuration
    |--------------------------------------------------------------------------
    */
    'threading' => [
        // Enable subject-based threading fallback
        'use_subject_matching' => env('INBOUND_EMAIL_SUBJECT_MATCHING', true),

        // Time window for subject matching (hours)
        'subject_match_window_hours' => env('INBOUND_EMAIL_SUBJECT_MATCH_WINDOW', 48),

        // Maximum thread depth
        'max_depth' => env('INBOUND_EMAIL_MAX_THREAD_DEPTH', 100),
    ],

    /*
    |--------------------------------------------------------------------------
    | Reply Address Pattern
    |--------------------------------------------------------------------------
    |
    | Pattern for generating reply-to addresses with routing hash.
    | Use {hash} as placeholder for the mailbox hash.
    |
    */
    'reply_address_pattern' => env(
        'INBOUND_EMAIL_ADDRESS_PATTERN',
        'reply+{hash}@mail.revolutiontradingpros.com'
    ),

    /*
    |--------------------------------------------------------------------------
    | Circuit Breaker Settings
    |--------------------------------------------------------------------------
    */
    'circuit_breaker' => [
        'failure_threshold' => env('INBOUND_EMAIL_CB_FAILURE_THRESHOLD', 5),
        'success_threshold' => env('INBOUND_EMAIL_CB_SUCCESS_THRESHOLD', 2),
        'timeout_seconds' => env('INBOUND_EMAIL_CB_TIMEOUT', 60),
    ],

    /*
    |--------------------------------------------------------------------------
    | Metrics & Observability
    |--------------------------------------------------------------------------
    */
    'metrics' => [
        // Enable Prometheus metrics
        'enabled' => env('INBOUND_EMAIL_METRICS_ENABLED', true),

        // Metrics endpoint path
        'path' => env('INBOUND_EMAIL_METRICS_PATH', '/metrics'),
    ],
];
