<?php

/**
 * SEO Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT11+ Configuration System
 *
 * Comprehensive SEO settings for Google-first architecture with:
 * - Data Source Management (Google Search Console, Keyword Planner, Trends)
 * - Predictive Analytics Configuration
 * - Keyword Intelligence Settings
 * - SERP Analysis Options
 * - Third-Party Fallback Configuration (optional)
 *
 * Architecture: Google-First with optional third-party fallbacks
 * Compliance: 100% Google tools for core functionality
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
    | Data Source Configuration (Google-First Architecture)
    |--------------------------------------------------------------------------
    |
    | Controls which data providers are used and their priority.
    | Google tools are always enabled; third-party tools are optional fallbacks.
    |
    | Priority: Lower number = higher priority
    |
    */
    'data_sources' => [
        // Enable Google-only mode (disable all third-party providers)
        'google_only' => env('SEO_GOOGLE_ONLY', true),

        // Enable third-party fallbacks when Google data unavailable
        'enable_third_party' => env('SEO_ENABLE_THIRD_PARTY', false),

        // Provider priority (lower = higher priority)
        'provider_priority' => [
            'google_search_console' => 10,  // Highest - YOUR site's authoritative data
            'google_keyword_planner' => 20, // Official search volume & CPC
            'google_trends' => 30,          // Trending topics & related queries
            'serpapi' => 100,               // Third-party fallback (disabled by default)
        ],

        // Data aggregation strategy
        // Options: 'merge_highest_confidence', 'primary_only', 'average_all'
        'aggregation_strategy' => env('SEO_AGGREGATION_STRATEGY', 'merge_highest_confidence'),

        // Cache aggregated results for better performance
        'cache_aggregated_results' => env('SEO_CACHE_AGGREGATED', true),
    ],

    /*
    |--------------------------------------------------------------------------
    | Google Search Console Configuration
    |--------------------------------------------------------------------------
    |
    | PRIMARY data source for site performance data.
    | Provides: Positions, Clicks, Impressions, CTR, Indexed Pages
    |
    */
    'google_search_console' => [
        'enabled' => env('GSC_ENABLED', true),

        // OAuth credentials (configured in google.php)
        'credentials_path' => env('GOOGLE_APPLICATION_CREDENTIALS'),

        // Default site URL for queries
        'site_url' => env('GSC_SITE_URL', env('APP_URL')),

        // Default date range for queries (days)
        'default_date_range' => env('GSC_DATE_RANGE', 28),

        // Maximum rows to fetch per query
        'max_rows' => env('GSC_MAX_ROWS', 25000),

        // Enable historical data comparison
        'enable_comparison' => env('GSC_ENABLE_COMPARISON', true),

        // Data freshness threshold (hours)
        // GSC data is typically 2-3 days behind
        'freshness_threshold' => env('GSC_FRESHNESS_HOURS', 72),

        // Rate limiting
        'rate_limit' => [
            'requests_per_minute' => env('GSC_RATE_LIMIT', 30),
            'concurrent_requests' => env('GSC_CONCURRENT', 5),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Google Keyword Planner Configuration
    |--------------------------------------------------------------------------
    |
    | Official Google data source for search volume and CPC data.
    | Requires Google Ads account (even free account works).
    |
    */
    'google_keyword_planner' => [
        'enabled' => env('GKP_ENABLED', true),

        // Google Ads customer ID (required)
        'customer_id' => env('GOOGLE_ADS_CUSTOMER_ID'),

        // OAuth credentials
        'developer_token' => env('GOOGLE_ADS_DEVELOPER_TOKEN'),

        // Location and language targeting
        'default_location_ids' => env('GKP_LOCATIONS', '2840'), // US
        'default_language_id' => env('GKP_LANGUAGE', '1000'),   // English

        // Keyword ideas limits
        'max_ideas_per_query' => env('GKP_MAX_IDEAS', 500),

        // Include adult content keywords
        'include_adult_keywords' => false,

        // Rate limiting
        'rate_limit' => [
            'requests_per_day' => env('GKP_DAILY_LIMIT', 1000),
            'requests_per_minute' => env('GKP_RATE_LIMIT', 10),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Google Trends Configuration
    |--------------------------------------------------------------------------
    |
    | Trending topics, related queries, and interest over time data.
    | Uses public endpoints (no API key required).
    |
    */
    'google_trends' => [
        'enabled' => env('GOOGLE_TRENDS_ENABLED', true),

        // Default geographic region
        'default_geo' => env('TRENDS_GEO', 'US'),

        // Default time range for queries
        // Options: 'now 1-H', 'now 4-H', 'now 1-d', 'now 7-d', 'today 1-m', 'today 3-m', 'today 12-m', 'today 5-y'
        'default_time_range' => env('TRENDS_TIME_RANGE', 'today 12-m'),

        // Category filter (optional)
        // See: https://github.com/pat310/google-trends-api/wiki/Google-Trends-Categories
        'default_category' => env('TRENDS_CATEGORY', 0),

        // Rate limiting (be conservative - no official API)
        'rate_limit' => [
            'requests_per_minute' => env('TRENDS_RATE_LIMIT', 5),
            'delay_between_requests_ms' => env('TRENDS_DELAY', 2000),
        ],

        // Cache TTL for trend data (shorter than other caches)
        'cache_ttl' => env('TRENDS_CACHE_TTL', 1800), // 30 minutes
    ],

    /*
    |--------------------------------------------------------------------------
    | Keyword Intelligence Configuration
    |--------------------------------------------------------------------------
    |
    | Settings for keyword research, difficulty scoring, and intent classification.
    |
    */
    'keyword_intelligence' => [
        // Intent classification patterns
        'intent_patterns' => [
            'transactional' => ['buy', 'purchase', 'order', 'price', 'cost', 'cheap', 'discount', 'deal', 'coupon', 'shop', 'store', 'sale'],
            'commercial' => ['best', 'top', 'review', 'comparison', 'vs', 'versus', 'alternative', 'compare', 'option', 'recommend', 'rated'],
            'navigational' => ['login', 'sign in', 'account', 'dashboard', 'portal', 'official', 'website', 'app', 'download', 'contact'],
            'informational' => ['how', 'what', 'why', 'when', 'where', 'who', 'which', 'guide', 'tutorial', 'learn', 'example'],
            'local' => ['near me', 'nearby', 'in my area', 'local', 'closest', 'directions', 'hours', 'open now'],
        ],

        // Difficulty scoring weights
        'difficulty_weights' => [
            'domain_authority' => 0.35,
            'page_authority' => 0.25,
            'top_domains' => 0.15,
            'serp_features' => 0.10,
            'content_depth' => 0.10,
            'freshness' => 0.05,
        ],

        // Keyword clustering settings
        'clustering' => [
            'min_cluster_size' => 2,
            'similarity_threshold' => 0.35,
            'max_clusters' => 50,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Keyword Gap Analysis Configuration
    |--------------------------------------------------------------------------
    |
    | Settings for identifying keyword opportunities (100% Google tools).
    |
    */
    'keyword_gaps' => [
        // Position opportunity thresholds
        'quick_win_position_range' => [11, 20],     // Positions 11-20 for quick wins
        'opportunity_max_position' => 50,            // Max position to consider

        // CTR optimization thresholds
        'ctr_gap_threshold' => 0.05,                 // 5% below expected CTR

        // Decline detection
        'decline_threshold_positions' => 5,          // Position drop to flag
        'decline_threshold_percentage' => 20,        // Traffic decline % to flag

        // Cannibalization detection
        'cannibalization_position_gap' => 10,        // Position difference to flag
        'cannibalization_min_impressions' => 100,    // Minimum impressions

        // Trending opportunity
        'trending_growth_threshold' => 50,           // % growth to flag as trending

        // Results limits
        'max_gaps_per_type' => 100,
        'max_quick_wins' => 20,
    ],

    /*
    |--------------------------------------------------------------------------
    | Predictive Analytics Configuration
    |--------------------------------------------------------------------------
    |
    | Settings for traffic prediction, ROI forecasting, and growth modeling.
    |
    */
    'predictive_analytics' => [
        'enabled' => env('SEO_PREDICTIVE_ENABLED', true),

        // CTR curve data (by position)
        // Based on Advanced Web Ranking studies
        'ctr_curve' => [
            1 => 0.319,
            2 => 0.158,
            3 => 0.100,
            4 => 0.068,
            5 => 0.051,
            6 => 0.042,
            7 => 0.033,
            8 => 0.029,
            9 => 0.025,
            10 => 0.022,
        ],

        // ROI calculation defaults
        'roi_defaults' => [
            'average_order_value' => env('SEO_AOV', 50.00),
            'conversion_rate' => env('SEO_CONVERSION_RATE', 0.02),
            'hourly_seo_cost' => env('SEO_HOURLY_COST', 100.00),
        ],

        // Opportunity scoring weights
        'opportunity_weights' => [
            'traffic_potential' => 0.30,
            'difficulty_inverse' => 0.20,
            'trend_signal' => 0.15,
            'commercial_value' => 0.15,
            'position_headroom' => 0.10,
            'ctr_potential' => 0.10,
        ],

        // Growth trajectory modeling
        'growth_model' => [
            'base_improvement_rate' => 0.10,         // 10% monthly improvement
            'max_improvement_rate' => 0.20,          // 20% max with high effort
            'effort_multiplier' => 400,              // Hours for max rate
        ],

        // Content decay detection
        'content_decay' => [
            'position_drop_threshold' => 10,
            'traffic_decline_threshold' => 0.20,     // 20% decline
            'ctr_erosion_threshold' => 0.05,         // 5% below expected
        ],
    ],

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
    | Google General Settings
    |--------------------------------------------------------------------------
    */
    'google' => [
        // Google Search Console verification
        'verification_code' => env('GOOGLE_SITE_VERIFICATION'),

        // Google Analytics
        'analytics_id' => env('GOOGLE_ANALYTICS_ID'),
        'gtm_id' => env('GOOGLE_TAG_MANAGER_ID'),

        // Google Analytics 4 Measurement Protocol
        'ga4_measurement_id' => env('GA4_MEASUREMENT_ID'),
        'ga4_api_secret' => env('GA4_API_SECRET'),

        // Google Business Profile
        'business_profile_id' => env('GOOGLE_BUSINESS_PROFILE_ID'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Third-Party Providers (Optional Fallbacks)
    |--------------------------------------------------------------------------
    |
    | These providers are DISABLED by default for Google-only compliance.
    | Enable only if you need additional data not available from Google.
    |
    */
    'third_party' => [
        // SerpAPI for SERP features analysis
        'serpapi' => [
            'enabled' => env('SERPAPI_ENABLED', false),
            'api_key' => env('SERPAPI_API_KEY'),
            'rate_limit' => env('SERPAPI_RATE_LIMIT', 10),
            'search_engine' => 'google',
            'cache_ttl' => 21600, // 6 hours
        ],

        // DataForSEO (alternative SERP provider)
        'dataforseo' => [
            'enabled' => env('DATAFORSEO_ENABLED', false),
            'login' => env('DATAFORSEO_LOGIN'),
            'password' => env('DATAFORSEO_PASSWORD'),
        ],

        // Ahrefs (backlink and keyword data)
        'ahrefs' => [
            'enabled' => env('AHREFS_ENABLED', false),
            'api_token' => env('AHREFS_API_TOKEN'),
        ],

        // Moz (domain authority)
        'moz' => [
            'enabled' => env('MOZ_ENABLED', false),
            'access_id' => env('MOZ_ACCESS_ID'),
            'secret_key' => env('MOZ_SECRET_KEY'),
        ],
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

        // Sitemap index settings
        'index' => [
            'enabled' => true,
            'max_urls_per_sitemap' => 10000,
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
            '/checkout/*',
            '/cart/*',
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

        // Product schema (for e-commerce)
        'product' => [
            'enabled' => true,
            'include_reviews' => true,
            'include_availability' => true,
        ],

        // Article schema
        'article' => [
            'enabled' => true,
            'default_type' => 'Article', // Article, NewsArticle, BlogPosting
        ],

        // FAQ schema
        'faq' => [
            'enabled' => true,
            'auto_detect' => true, // Auto-detect FAQ patterns in content
        ],

        // HowTo schema
        'howto' => [
            'enabled' => true,
            'auto_detect' => true, // Auto-detect how-to patterns
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

        // Provider-specific cache TTLs
        'provider_ttl' => [
            'google_search_console' => 3600,    // 1 hour
            'google_keyword_planner' => 86400,  // 24 hours
            'google_trends' => 1800,            // 30 minutes
            'serpapi' => 21600,                 // 6 hours
        ],
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

        // Content scoring weights
        'scoring_weights' => [
            'keyword_usage' => 0.25,
            'content_length' => 0.20,
            'readability' => 0.15,
            'internal_links' => 0.15,
            'headings_structure' => 0.10,
            'meta_optimization' => 0.10,
            'media_usage' => 0.05,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Internal Linking
    |--------------------------------------------------------------------------
    */
    'internal_linking' => [
        'enabled' => env('SEO_INTERNAL_LINKING', true),

        // Maximum links per page
        'max_links_per_page' => 100,

        // Minimum word count between links
        'min_words_between_links' => 100,

        // Anchor text settings
        'anchor_text' => [
            'min_length' => 2,
            'max_length' => 60,
            'avoid_generic' => true, // Avoid "click here", "read more"
        ],

        // Link suggestion settings
        'suggestions' => [
            'enabled' => true,
            'max_suggestions' => 5,
            'similarity_threshold' => 0.3,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | NLP & Entity Analysis (Google Cloud NLP)
    |--------------------------------------------------------------------------
    */
    'nlp' => [
        'enabled' => env('GOOGLE_NLP_ENABLED', true),

        // Google Cloud NLP settings
        'credentials_path' => env('GOOGLE_CLOUD_NLP_CREDENTIALS'),
        'project_id' => env('GOOGLE_CLOUD_PROJECT_ID'),

        // Analysis types to run
        'analyze' => [
            'entities' => true,
            'sentiment' => true,
            'categories' => true,
            'syntax' => false, // More expensive, optional
        ],

        // Rate limiting
        'rate_limit' => [
            'requests_per_minute' => env('NLP_RATE_LIMIT', 30),
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Reporting & Alerts
    |--------------------------------------------------------------------------
    */
    'reporting' => [
        // Email reports
        'email' => [
            'enabled' => env('SEO_EMAIL_REPORTS', false),
            'recipients' => explode(',', env('SEO_REPORT_EMAILS', '')),
            'schedule' => 'weekly', // daily, weekly, monthly
        ],

        // Slack alerts
        'slack' => [
            'enabled' => env('SEO_SLACK_ALERTS', false),
            'webhook_url' => env('SEO_SLACK_WEBHOOK'),
            'channel' => env('SEO_SLACK_CHANNEL', '#seo-alerts'),
        ],

        // Alert thresholds
        'alerts' => [
            'ranking_drop' => 10,           // Alert if position drops by X
            'traffic_decline' => 0.20,      // Alert if traffic drops by X%
            'crawl_errors' => 5,            // Alert if X+ crawl errors
            'index_coverage_drop' => 0.05,  // Alert if indexed pages drop by X%
        ],
    ],

];
