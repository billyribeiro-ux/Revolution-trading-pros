<?php

return [
    /*
    |--------------------------------------------------------------------------
    | SEO Intelligence Cache Configuration
    |--------------------------------------------------------------------------
    |
    | Google L8 Enterprise-Grade Redis Cache Strategy
    | Optimized for AI/NLP operations with sub-millisecond performance
    |
    */

    'driver' => env('SEO_CACHE_DRIVER', 'redis'),

    'redis' => [
        'client' => env('REDIS_CLIENT', 'phpredis'),
        'cluster' => env('REDIS_CLUSTER', false),
        
        'default' => [
            'url' => env('REDIS_URL'),
            'host' => env('REDIS_HOST', '127.0.0.1'),
            'password' => env('REDIS_PASSWORD'),
            'port' => env('REDIS_PORT', 6379),
            'database' => env('REDIS_SEO_DB', 2), // Dedicated database for SEO cache
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Cache TTL Configuration (in seconds)
    |--------------------------------------------------------------------------
    */

    'ttl' => [
        // NLP Operations (expensive, cache longer)
        'nlp_entity_extraction' => 86400, // 24 hours
        'nlp_topic_modeling' => 86400, // 24 hours
        'nlp_sentiment_analysis' => 86400, // 24 hours
        
        // SERP Data (changes frequently)
        'serp_results' => 21600, // 6 hours
        'serp_features' => 21600, // 6 hours
        'serp_competitors' => 43200, // 12 hours
        
        // Keyword Intelligence
        'keyword_difficulty' => 43200, // 12 hours
        'keyword_intent' => 86400, // 24 hours
        'keyword_clusters' => 86400, // 24 hours
        
        // AI Suggestions (regenerate frequently for freshness)
        'ai_title_suggestions' => 3600, // 1 hour
        'ai_meta_suggestions' => 3600, // 1 hour
        'ai_content_suggestions' => 3600, // 1 hour
        'ai_outline_generation' => 7200, // 2 hours
        
        // Internal Links
        'link_graph' => 3600, // 1 hour
        'link_suggestions' => 3600, // 1 hour
        'link_orphans' => 3600, // 1 hour
        
        // Schema
        'schema_templates' => 0, // No expiration, invalidate on edit
        'schema_instances' => 86400, // 24 hours
        
        // Content Analysis
        'content_analysis' => 43200, // 12 hours
        'entity_coverage' => 43200, // 12 hours
        'topic_coverage' => 43200, // 12 hours
    ],

    /*
    |--------------------------------------------------------------------------
    | Cache Key Prefixes
    |--------------------------------------------------------------------------
    */

    'prefixes' => [
        'nlp' => 'seo:nlp',
        'entities' => 'seo:entities',
        'topics' => 'seo:topics',
        'keywords' => 'seo:keywords',
        'serp' => 'seo:serp',
        'ai' => 'seo:ai',
        'links' => 'seo:links',
        'schema' => 'seo:schema',
        'analysis' => 'seo:analysis',
    ],

    /*
    |--------------------------------------------------------------------------
    | Cache Tags Configuration
    |--------------------------------------------------------------------------
    */

    'tags' => [
        'enabled' => true,
        'separator' => ':',
    ],

    /*
    |--------------------------------------------------------------------------
    | Performance Targets
    |--------------------------------------------------------------------------
    */

    'performance' => [
        'target_hit_rate' => 0.90, // 90% cache hit rate target
        'max_response_time_ms' => 100, // Max 100ms for cached operations
        'max_key_size_bytes' => 1048576, // 1MB max per cache entry
    ],

    /*
    |--------------------------------------------------------------------------
    | Cache Warming Configuration
    |--------------------------------------------------------------------------
    */

    'warming' => [
        'enabled' => env('SEO_CACHE_WARMING_ENABLED', true),
        'schedule' => '0 2 * * *', // 2 AM daily
        'batch_size' => 100,
        'operations' => [
            'popular_keywords' => true,
            'top_content' => true,
            'link_graph' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Cache Invalidation Rules
    |--------------------------------------------------------------------------
    */

    'invalidation' => [
        // Invalidate on content update
        'on_content_update' => [
            'nlp_entity_extraction',
            'nlp_topic_modeling',
            'entity_coverage',
            'topic_coverage',
            'link_suggestions',
            'schema_instances',
        ],
        
        // Invalidate on keyword update
        'on_keyword_update' => [
            'keyword_difficulty',
            'keyword_clusters',
            'serp_results',
        ],
        
        // Invalidate on link change
        'on_link_change' => [
            'link_graph',
            'link_suggestions',
            'link_orphans',
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Monitoring & Metrics
    |--------------------------------------------------------------------------
    */

    'monitoring' => [
        'enabled' => env('SEO_CACHE_MONITORING_ENABLED', true),
        'metrics' => [
            'hit_rate' => true,
            'miss_rate' => true,
            'eviction_rate' => true,
            'memory_usage' => true,
            'operation_latency' => true,
        ],
        'alerts' => [
            'low_hit_rate_threshold' => 0.80, // Alert if < 80%
            'high_memory_threshold' => 0.90, // Alert if > 90% memory used
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Serialization
    |--------------------------------------------------------------------------
    */

    'serialization' => [
        'format' => 'json', // json, msgpack, igbinary
        'compression' => env('SEO_CACHE_COMPRESSION', false),
        'compression_level' => 6, // 1-9 (higher = better compression, slower)
    ],
];
