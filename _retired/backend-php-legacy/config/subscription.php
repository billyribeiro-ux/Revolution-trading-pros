<?php

return [
    /*
    |--------------------------------------------------------------------------
    | Default Payment Provider
    |--------------------------------------------------------------------------
    |
    | The default payment provider to use for processing subscriptions.
    | Supported: "stripe", "paypal", "paddle"
    |
    */
    'default_provider' => env('SUBSCRIPTION_PROVIDER', 'stripe'),

    /*
    |--------------------------------------------------------------------------
    | Business Information
    |--------------------------------------------------------------------------
    |
    | Your business details for invoicing and tax purposes.
    |
    */
    'business_country' => env('BUSINESS_COUNTRY', 'US'),
    'business_vat_id' => env('BUSINESS_VAT_ID'),
    'company_name' => env('COMPANY_NAME', config('app.name')),
    'company_address' => env('COMPANY_ADDRESS'),
    'company_city' => env('COMPANY_CITY'),
    'company_phone' => env('COMPANY_PHONE'),
    'billing_email' => env('BILLING_EMAIL'),
    'invoice_logo' => env('INVOICE_LOGO'),

    /*
    |--------------------------------------------------------------------------
    | Free Tier Features
    |--------------------------------------------------------------------------
    |
    | Features available to users without an active subscription.
    |
    */
    'free_features' => [
        'basic_dashboard',
        'limited_reports',
    ],

    /*
    |--------------------------------------------------------------------------
    | Free Tier Limits
    |--------------------------------------------------------------------------
    |
    | Usage limits for free tier users.
    |
    */
    'free_limits' => [
        'api_calls' => 100,
        'storage_mb' => 100,
        'exports' => 5,
        'ai_requests' => 10,
    ],

    /*
    |--------------------------------------------------------------------------
    | Dunning Configuration
    |--------------------------------------------------------------------------
    |
    | Settings for the payment recovery process.
    |
    */
    'dunning' => [
        'max_attempts' => 4,
        'grace_period_days' => 14,
        'retry_intervals' => [3, 5, 7, 7], // Days between retries
    ],

    /*
    |--------------------------------------------------------------------------
    | Trial Configuration
    |--------------------------------------------------------------------------
    |
    | Default trial settings for new subscriptions.
    |
    */
    'trial' => [
        'default_days' => 14,
        'require_payment_method' => false,
        'reminder_days_before' => 3,
    ],

    /*
    |--------------------------------------------------------------------------
    | Invoice Settings
    |--------------------------------------------------------------------------
    |
    | Configuration for invoice generation.
    |
    */
    'invoice' => [
        'prefix' => 'INV-',
        'footer_text' => 'Thank you for your business!',
        'payment_terms' => 'Due upon receipt',
    ],

    /*
    |--------------------------------------------------------------------------
    | Webhook Settings
    |--------------------------------------------------------------------------
    |
    | Webhook configuration for payment providers.
    |
    */
    'webhooks' => [
        'stripe' => [
            'path' => '/webhooks/stripe',
            'tolerance' => 300, // Signature tolerance in seconds
        ],
        'paypal' => [
            'path' => '/webhooks/paypal',
        ],
        'paddle' => [
            'path' => '/webhooks/paddle',
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Tax Settings
    |--------------------------------------------------------------------------
    |
    | Configuration for automatic tax calculation.
    |
    */
    'tax' => [
        'enabled' => true,
        'validate_vat' => true,
        'inclusive' => false, // Prices include tax
    ],

    /*
    |--------------------------------------------------------------------------
    | Proration
    |--------------------------------------------------------------------------
    |
    | How to handle prorating when changing plans.
    |
    */
    'proration' => [
        'enabled' => true,
        'behavior' => 'create_prorations', // create_prorations, none, always_invoice
    ],

    /*
    |--------------------------------------------------------------------------
    | Cancellation
    |--------------------------------------------------------------------------
    |
    | Cancellation behavior settings.
    |
    */
    'cancellation' => [
        'allow_immediate' => true,
        'refund_policy' => 'prorated', // none, prorated, full
        'survey_enabled' => true,
    ],
];
