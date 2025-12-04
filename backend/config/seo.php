<?php

/**
 * SEO Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive SEO settings for Google, Bing, and other search engines.
 */

return [

    /*
    |--------------------------------------------------------------------------
    | General SEO Settings
    |--------------------------------------------------------------------------
    */
    'site_name' => env('APP_NAME', 'Revolution Trading Pros'),
    'site_description' => env('SEO_SITE_DESCRIPTION', ''),
    'default_image' => env('SEO_DEFAULT_IMAGE', '/images/og-image.jpg'),
    'twitter_handle' => env('SEO_TWITTER_HANDLE', ''),
    'facebook_app_id' => env('SEO_FACEBOOK_APP_ID', ''),

    /*
    |--------------------------------------------------------------------------
    | Bing SEO Settings - COMPETITIVE ADVANTAGE
    |--------------------------------------------------------------------------
    |
    | Most websites completely ignore Bing optimization. These settings give
    | you a significant advantage in Bing search results.
    |
    */
    'bing' => [
        // IndexNow API key for instant URL indexing
        // URLs are indexed in MINUTES instead of days/weeks
        'indexnow_key' => env('BING_INDEXNOW_KEY'),

        // Bing Webmaster Tools API key for advanced features
        // Get from: https://www.bing.com/webmasters/
        'api_key' => env('BING_WEBMASTER_API_KEY'),

        // Site verification code from Bing Webmaster Tools
        // Add to verify site ownership
        'verification_code' => env('BING_VERIFICATION_CODE'),

        // Auto-submit URLs to IndexNow on content publish
        'auto_submit' => env('BING_AUTO_SUBMIT', true),

        // Batch submission settings
        'batch_size' => env('BING_BATCH_SIZE', 100),

        // Rate limiting (requests per minute)
        'rate_limit' => env('BING_RATE_LIMIT', 10),
    ],

    /*
    |--------------------------------------------------------------------------
    | Google SEO Settings
    |--------------------------------------------------------------------------
    */
    'google' => [
        // Google Search Console verification
        'verification_code' => env('GOOGLE_SITE_VERIFICATION'),

        // Google Analytics
        'analytics_id' => env('GOOGLE_ANALYTICS_ID'),
        'gtm_id' => env('GOOGLE_TAG_MANAGER_ID'),

        // Google Business Profile
        'business_profile_id' => env('GOOGLE_BUSINESS_PROFILE_ID'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Sitemap Settings
    |--------------------------------------------------------------------------
    */
    'sitemap' => [
        'enabled' => env('SITEMAP_ENABLED', true),
        'max_entries' => env('SITEMAP_MAX_ENTRIES', 50000),
        'update_frequency' => env('SITEMAP_UPDATE_FREQUENCY', 'daily'),

        // Content types to include
        'include' => [
            'posts' => true,
            'pages' => true,
            'products' => true,
            'categories' => true,
            'tags' => true,
        ],

        // Priority settings (0.0 to 1.0)
        'priorities' => [
            'homepage' => 1.0,
            'posts' => 0.8,
            'pages' => 0.7,
            'products' => 0.9,
            'categories' => 0.6,
            'tags' => 0.5,
        ],

        // Auto-ping search engines on sitemap update
        'ping_on_update' => [
            'google' => true,
            'bing' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Robots.txt Settings
    |--------------------------------------------------------------------------
    */
    'robots' => [
        'enabled' => true,

        // Disallow paths
        'disallow' => [
            '/admin/*',
            '/api/*',
            '/login',
            '/register',
            '/password/*',
            '/*.json$',
        ],

        // Allow specific paths (overrides disallow)
        'allow' => [
            '/api/posts',
            '/api/products',
        ],

        // Crawl delay (seconds)
        'crawl_delay' => [
            'googlebot' => null,  // Google ignores this
            'bingbot' => 1,
            'default' => 2,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Structured Data Settings
    |--------------------------------------------------------------------------
    */
    'schema' => [
        'organization' => [
            'enabled' => true,
            'name' => env('ORGANIZATION_NAME', env('APP_NAME')),
            'legal_name' => env('ORGANIZATION_LEGAL_NAME'),
            'url' => env('APP_URL'),
            'logo' => env('ORGANIZATION_LOGO'),
            'founding_date' => env('ORGANIZATION_FOUNDING_DATE'),
            'founders' => [],
            'contact' => [
                'telephone' => env('ORGANIZATION_PHONE'),
                'email' => env('ORGANIZATION_EMAIL'),
                'address' => [
                    'street' => env('ORGANIZATION_STREET'),
                    'city' => env('ORGANIZATION_CITY'),
                    'state' => env('ORGANIZATION_STATE'),
                    'postal_code' => env('ORGANIZATION_ZIP'),
                    'country' => env('ORGANIZATION_COUNTRY', 'US'),
                ],
            ],
            'social' => [
                'facebook' => env('SOCIAL_FACEBOOK'),
                'twitter' => env('SOCIAL_TWITTER'),
                'linkedin' => env('SOCIAL_LINKEDIN'),
                'youtube' => env('SOCIAL_YOUTUBE'),
                'instagram' => env('SOCIAL_INSTAGRAM'),
            ],
        ],

        'website' => [
            'enabled' => true,
            'search_enabled' => true,
            'search_target' => env('APP_URL') . '/search?q={search_term_string}',
        ],

        'breadcrumbs' => [
            'enabled' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Meta Tag Defaults
    |--------------------------------------------------------------------------
    */
    'meta' => [
        'title_separator' => ' | ',
        'title_suffix' => env('APP_NAME'),
        'default_description_length' => 160,
        'default_title_length' => 60,

        // Open Graph defaults
        'og_type' => 'website',
        'og_locale' => 'en_US',

        // Twitter Card defaults
        'twitter_card' => 'summary_large_image',
    ],

    /*
    |--------------------------------------------------------------------------
    | Performance & Caching
    |--------------------------------------------------------------------------
    */
    'cache' => [
        'enabled' => env('SEO_CACHE_ENABLED', true),
        'ttl' => env('SEO_CACHE_TTL', 3600), // 1 hour
        'prefix' => 'seo:',

        // Cache specific items
        'cache_sitemaps' => true,
        'cache_meta_tags' => true,
        'cache_schemas' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | Content Analysis
    |--------------------------------------------------------------------------
    */
    'analysis' => [
        // Minimum content length for SEO scoring
        'min_content_length' => 300,

        // Target keyword density (percentage)
        'keyword_density' => [
            'min' => 1.0,
            'max' => 3.0,
        ],

        // Readability targets
        'readability' => [
            'flesch_reading_ease_min' => 60,
            'gunning_fog_max' => 12,
        ],
    ],

];
