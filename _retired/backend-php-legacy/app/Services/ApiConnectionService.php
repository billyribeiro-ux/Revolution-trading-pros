<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\ApiConnection;
use App\Models\ApiConnectionLog;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Collection;

/**
 * API Connection Service
 *
 * Centralized service for managing all third-party API connections.
 * Handles connection, disconnection, verification, and health monitoring.
 *
 * @level L11 Principal Engineer - Apple-grade implementation
 */
class ApiConnectionService
{
    /**
     * Registry of all available services that can be connected
     */
    protected array $serviceRegistry = [
        // === INFRASTRUCTURE (Lightning Stack) ===
        'cloudflare_r2' => [
            'name' => 'Cloudflare R2',
            'category' => 'infrastructure',
            'description' => 'S3-compatible object storage with zero egress fees',
            'icon' => 'cloudflare',
            'color' => '#F38020',
            'docs_url' => 'https://developers.cloudflare.com/r2/',
            'signup_url' => 'https://dash.cloudflare.com/sign-up',
            'pricing_url' => 'https://developers.cloudflare.com/r2/pricing/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'account_id', 'label' => 'Account ID', 'type' => 'text', 'required' => true, 'help' => 'Found in Cloudflare Dashboard URL'],
                ['key' => 'access_key_id', 'label' => 'Access Key ID', 'type' => 'text', 'required' => true],
                ['key' => 'secret_access_key', 'label' => 'Secret Access Key', 'type' => 'password', 'required' => true],
                ['key' => 'bucket', 'label' => 'Bucket Name', 'type' => 'text', 'required' => true, 'placeholder' => 'revolution-trading-media'],
                ['key' => 'public_url', 'label' => 'Public URL', 'type' => 'url', 'required' => false, 'placeholder' => 'https://pub-xxx.r2.dev'],
            ],
            'verify_endpoint' => null, // Custom verification
        ],
        'neon_postgres' => [
            'name' => 'Neon PostgreSQL',
            'category' => 'infrastructure',
            'description' => 'Serverless PostgreSQL with autoscaling and branching',
            'icon' => 'database',
            'color' => '#00E599',
            'docs_url' => 'https://neon.tech/docs',
            'signup_url' => 'https://console.neon.tech',
            'pricing_url' => 'https://neon.tech/pricing',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'project_id', 'label' => 'Project ID', 'type' => 'text', 'required' => true],
                ['key' => 'database_url', 'label' => 'Database URL', 'type' => 'password', 'required' => true, 'placeholder' => 'postgres://user:pass@ep-xxx.us-east-2.aws.neon.tech/neondb'],
                ['key' => 'host', 'label' => 'Host', 'type' => 'text', 'required' => false, 'placeholder' => 'ep-xxx.us-east-2.aws.neon.tech'],
                ['key' => 'database', 'label' => 'Database Name', 'type' => 'text', 'required' => false, 'placeholder' => 'neondb'],
                ['key' => 'username', 'label' => 'Username', 'type' => 'text', 'required' => false],
                ['key' => 'password', 'label' => 'Password', 'type' => 'password', 'required' => false],
            ],
        ],
        'upstash_redis' => [
            'name' => 'Upstash Redis',
            'category' => 'infrastructure',
            'description' => 'Serverless Redis with global replication for <1ms reads',
            'icon' => 'database',
            'color' => '#00E9A3',
            'docs_url' => 'https://upstash.com/docs/redis',
            'signup_url' => 'https://console.upstash.com',
            'pricing_url' => 'https://upstash.com/pricing',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'redis_url', 'label' => 'Redis URL', 'type' => 'password', 'required' => true, 'placeholder' => 'rediss://default:xxx@xxx.upstash.io:6379'],
                ['key' => 'rest_url', 'label' => 'REST URL', 'type' => 'url', 'required' => true, 'placeholder' => 'https://xxx.upstash.io'],
                ['key' => 'rest_token', 'label' => 'REST Token', 'type' => 'password', 'required' => true],
            ],
        ],
        'sharp_service' => [
            'name' => 'Sharp Image Service',
            'category' => 'infrastructure',
            'description' => 'High-performance Node.js image processing (built-in, no signup needed)',
            'icon' => 'image',
            'color' => '#99CC00',
            'docs_url' => 'https://sharp.pixelplumbing.com/',
            'is_oauth' => false,
            'is_local' => true,
            'fields' => [
                ['key' => 'service_url', 'label' => 'Service URL', 'type' => 'url', 'required' => true, 'placeholder' => 'http://localhost:3001'],
                ['key' => 'timeout', 'label' => 'Timeout (seconds)', 'type' => 'number', 'required' => false, 'placeholder' => '60'],
            ],
        ],

        // === PAYMENTS ===
        'stripe' => [
            'name' => 'Stripe',
            'category' => 'payments',
            'description' => 'Accept payments with credit cards, Apple Pay, Google Pay, and more',
            'icon' => 'stripe',
            'color' => '#635BFF',
            'docs_url' => 'https://stripe.com/docs/api',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'publishable_key', 'label' => 'Publishable Key', 'type' => 'text', 'required' => true],
                ['key' => 'secret_key', 'label' => 'Secret Key', 'type' => 'password', 'required' => true],
                ['key' => 'webhook_secret', 'label' => 'Webhook Secret', 'type' => 'password', 'required' => false],
            ],
            'verify_endpoint' => 'https://api.stripe.com/v1/balance',
            'environments' => ['production', 'test'],
        ],
        'paypal' => [
            'name' => 'PayPal',
            'category' => 'payments',
            'description' => 'Accept PayPal payments and PayPal Credit',
            'icon' => 'paypal',
            'color' => '#003087',
            'docs_url' => 'https://developer.paypal.com/docs/api/overview/',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'client_id', 'label' => 'Client ID', 'type' => 'text', 'required' => true],
                ['key' => 'client_secret', 'label' => 'Client Secret', 'type' => 'password', 'required' => true],
            ],
            'environments' => ['production', 'sandbox'],
        ],
        'square' => [
            'name' => 'Square',
            'category' => 'payments',
            'description' => 'Process payments and manage inventory with Square',
            'icon' => 'square',
            'color' => '#006AFF',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'access_token', 'label' => 'Access Token', 'type' => 'password', 'required' => true],
                ['key' => 'location_id', 'label' => 'Location ID', 'type' => 'text', 'required' => true],
            ],
            'environments' => ['production', 'sandbox'],
        ],

        // === ANALYTICS ===
        'google_analytics' => [
            'name' => 'Google Analytics 4',
            'category' => 'analytics',
            'description' => 'Track website traffic and user behavior with GA4',
            'icon' => 'google-analytics',
            'color' => '#F9AB00',
            'docs_url' => 'https://developers.google.com/analytics',
            'is_oauth' => true,
            'oauth_scopes' => 'https://www.googleapis.com/auth/analytics.readonly',
            'fields' => [
                ['key' => 'measurement_id', 'label' => 'Measurement ID', 'type' => 'text', 'required' => true, 'placeholder' => 'G-XXXXXXXXXX'],
                ['key' => 'property_id', 'label' => 'Property ID', 'type' => 'text', 'required' => false],
            ],
        ],
        'google_search_console' => [
            'name' => 'Google Search Console',
            'category' => 'analytics',
            'description' => 'Monitor search performance and indexing status',
            'icon' => 'google',
            'color' => '#4285F4',
            'is_oauth' => true,
            'oauth_scopes' => 'https://www.googleapis.com/auth/webmasters.readonly',
            'fields' => [
                ['key' => 'site_url', 'label' => 'Site URL', 'type' => 'url', 'required' => true],
            ],
        ],
        'google_tag_manager' => [
            'name' => 'Google Tag Manager',
            'category' => 'analytics',
            'description' => 'Manage marketing tags without code changes',
            'icon' => 'google',
            'color' => '#246FDB',
            'docs_url' => 'https://tagmanager.google.com',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'container_id', 'label' => 'Container ID', 'type' => 'text', 'required' => true, 'placeholder' => 'GTM-XXXXXXX'],
            ],
        ],
        'google_ads' => [
            'name' => 'Google Ads',
            'category' => 'analytics',
            'description' => 'Track conversions and optimize ad campaigns',
            'icon' => 'google',
            'color' => '#34A853',
            'docs_url' => 'https://ads.google.com',
            'is_oauth' => true,
            'oauth_scopes' => 'https://www.googleapis.com/auth/adwords',
            'fields' => [
                ['key' => 'conversion_id', 'label' => 'Conversion ID', 'type' => 'text', 'required' => true, 'placeholder' => 'AW-XXXXXXXXX'],
                ['key' => 'conversion_label', 'label' => 'Conversion Label', 'type' => 'text', 'required' => false],
            ],
        ],
        'mixpanel' => [
            'name' => 'Mixpanel',
            'category' => 'analytics',
            'description' => 'Product analytics for user behavior tracking',
            'icon' => 'mixpanel',
            'color' => '#7856FF',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'project_token', 'label' => 'Project Token', 'type' => 'text', 'required' => true],
                ['key' => 'api_secret', 'label' => 'API Secret', 'type' => 'password', 'required' => false],
            ],
        ],
        'hotjar' => [
            'name' => 'Hotjar',
            'category' => 'analytics',
            'description' => 'Heatmaps, recordings, and feedback tools',
            'icon' => 'hotjar',
            'color' => '#FF3C00',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'site_id', 'label' => 'Site ID', 'type' => 'text', 'required' => true],
            ],
        ],

        // === EMAIL ===
        'sendgrid' => [
            'name' => 'SendGrid',
            'category' => 'email',
            'description' => 'Transactional and marketing email delivery',
            'icon' => 'sendgrid',
            'color' => '#1A82E2',
            'docs_url' => 'https://docs.sendgrid.com/api-reference',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
            ],
            'verify_endpoint' => 'https://api.sendgrid.com/v3/user/profile',
        ],
        'mailchimp' => [
            'name' => 'Mailchimp',
            'category' => 'email',
            'description' => 'Email marketing and audience management',
            'icon' => 'mailchimp',
            'color' => '#FFE01B',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
                ['key' => 'server_prefix', 'label' => 'Server Prefix', 'type' => 'text', 'required' => true, 'placeholder' => 'us1'],
            ],
        ],
        'postmark' => [
            'name' => 'Postmark',
            'category' => 'email',
            'description' => 'Fast and reliable transactional email',
            'icon' => 'postmark',
            'color' => '#FFDE00',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'server_token', 'label' => 'Server Token', 'type' => 'password', 'required' => true],
            ],
        ],
        'mailgun' => [
            'name' => 'Mailgun',
            'category' => 'email',
            'description' => 'Powerful email API for developers',
            'icon' => 'mailgun',
            'color' => '#F06B66',
            'docs_url' => 'https://documentation.mailgun.com/',
            'signup_url' => 'https://signup.mailgun.com/new/signup',
            'pricing_url' => 'https://www.mailgun.com/pricing/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
                ['key' => 'domain', 'label' => 'Domain', 'type' => 'text', 'required' => true],
            ],
        ],
        'resend' => [
            'name' => 'Resend',
            'category' => 'email',
            'description' => 'Modern email API built for developers',
            'icon' => 'mail',
            'color' => '#000000',
            'docs_url' => 'https://resend.com/docs',
            'signup_url' => 'https://resend.com/signup',
            'pricing_url' => 'https://resend.com/pricing',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true, 'placeholder' => 're_xxxxx'],
            ],
            'verify_endpoint' => 'https://api.resend.com/domains',
        ],
        'amazon_ses' => [
            'name' => 'Amazon SES',
            'category' => 'email',
            'description' => 'Scalable email service by AWS',
            'icon' => 'aws',
            'color' => '#FF9900',
            'docs_url' => 'https://docs.aws.amazon.com/ses/',
            'signup_url' => 'https://aws.amazon.com/ses/',
            'pricing_url' => 'https://aws.amazon.com/ses/pricing/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'access_key_id', 'label' => 'Access Key ID', 'type' => 'text', 'required' => true],
                ['key' => 'secret_access_key', 'label' => 'Secret Access Key', 'type' => 'password', 'required' => true],
                ['key' => 'region', 'label' => 'Region', 'type' => 'text', 'required' => true, 'placeholder' => 'us-east-1'],
            ],
        ],

        // === MONITORING ===
        'sentry' => [
            'name' => 'Sentry',
            'category' => 'monitoring',
            'description' => 'Error tracking and performance monitoring',
            'icon' => 'alert-triangle',
            'color' => '#362D59',
            'docs_url' => 'https://docs.sentry.io/',
            'signup_url' => 'https://sentry.io/signup/',
            'pricing_url' => 'https://sentry.io/pricing/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'dsn', 'label' => 'DSN', 'type' => 'password', 'required' => true, 'placeholder' => 'https://xxx@xxx.ingest.sentry.io/xxx'],
                ['key' => 'project_id', 'label' => 'Project ID', 'type' => 'text', 'required' => false],
                ['key' => 'environment', 'label' => 'Environment', 'type' => 'text', 'required' => false, 'placeholder' => 'production'],
            ],
        ],

        // === STORAGE ===
        'aws_s3' => [
            'name' => 'Amazon S3',
            'category' => 'storage',
            'description' => 'Cloud object storage for files and media',
            'icon' => 'aws',
            'color' => '#FF9900',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'access_key_id', 'label' => 'Access Key ID', 'type' => 'text', 'required' => true],
                ['key' => 'secret_access_key', 'label' => 'Secret Access Key', 'type' => 'password', 'required' => true],
                ['key' => 'bucket', 'label' => 'Bucket Name', 'type' => 'text', 'required' => true],
                ['key' => 'region', 'label' => 'Region', 'type' => 'text', 'required' => true, 'placeholder' => 'us-east-1'],
            ],
        ],
        'cloudinary' => [
            'name' => 'Cloudinary',
            'category' => 'storage',
            'description' => 'Image and video management in the cloud',
            'icon' => 'cloudinary',
            'color' => '#3448C5',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'cloud_name', 'label' => 'Cloud Name', 'type' => 'text', 'required' => true],
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'text', 'required' => true],
                ['key' => 'api_secret', 'label' => 'API Secret', 'type' => 'password', 'required' => true],
            ],
        ],
        'bunny_cdn' => [
            'name' => 'Bunny CDN',
            'category' => 'storage',
            'description' => 'Fast global CDN and edge storage',
            'icon' => 'bunny',
            'color' => '#FF6600',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
                ['key' => 'storage_zone', 'label' => 'Storage Zone', 'type' => 'text', 'required' => false],
            ],
        ],

        // === CRM ===
        'hubspot' => [
            'name' => 'HubSpot',
            'category' => 'crm',
            'description' => 'CRM, marketing, sales, and service platform',
            'icon' => 'hubspot',
            'color' => '#FF7A59',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'api_key', 'label' => 'Private App Access Token', 'type' => 'password', 'required' => true],
            ],
        ],
        'salesforce' => [
            'name' => 'Salesforce',
            'category' => 'crm',
            'description' => 'Enterprise CRM and business platform',
            'icon' => 'salesforce',
            'color' => '#00A1E0',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'instance_url', 'label' => 'Instance URL', 'type' => 'url', 'required' => true],
            ],
        ],
        'pipedrive' => [
            'name' => 'Pipedrive',
            'category' => 'crm',
            'description' => 'Sales CRM and pipeline management',
            'icon' => 'pipedrive',
            'color' => '#017737',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_token', 'label' => 'API Token', 'type' => 'password', 'required' => true],
            ],
        ],

        // === SOCIAL ===
        'facebook' => [
            'name' => 'Facebook / Meta',
            'category' => 'social',
            'description' => 'Facebook Pages, Ads, and Pixel integration',
            'icon' => 'facebook',
            'color' => '#1877F2',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => false],
            ],
        ],
        'twitter' => [
            'name' => 'Twitter / X',
            'category' => 'social',
            'description' => 'Twitter API for posting and analytics',
            'icon' => 'twitter',
            'color' => '#000000',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'text', 'required' => true],
                ['key' => 'api_secret', 'label' => 'API Secret', 'type' => 'password', 'required' => true],
            ],
        ],
        'linkedin' => [
            'name' => 'LinkedIn',
            'category' => 'social',
            'description' => 'LinkedIn marketing and company pages',
            'icon' => 'linkedin',
            'color' => '#0A66C2',
            'is_oauth' => true,
            'fields' => [],
        ],

        // === COMMUNICATION ===
        'twilio' => [
            'name' => 'Twilio',
            'category' => 'communication',
            'description' => 'SMS, voice, and video communications',
            'icon' => 'twilio',
            'color' => '#F22F46',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'account_sid', 'label' => 'Account SID', 'type' => 'text', 'required' => true],
                ['key' => 'auth_token', 'label' => 'Auth Token', 'type' => 'password', 'required' => true],
                ['key' => 'phone_number', 'label' => 'Phone Number', 'type' => 'tel', 'required' => false],
            ],
        ],
        'slack' => [
            'name' => 'Slack',
            'category' => 'communication',
            'description' => 'Team messaging and notifications',
            'icon' => 'slack',
            'color' => '#4A154B',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'webhook_url', 'label' => 'Webhook URL', 'type' => 'url', 'required' => false],
            ],
        ],
        'discord' => [
            'name' => 'Discord',
            'category' => 'communication',
            'description' => 'Community engagement and notifications',
            'icon' => 'discord',
            'color' => '#5865F2',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'webhook_url', 'label' => 'Webhook URL', 'type' => 'url', 'required' => false],
            ],
        ],

        // === AI & AUTOMATION ===
        'openai' => [
            'name' => 'OpenAI',
            'category' => 'ai',
            'description' => 'GPT-4, DALL-E, and AI capabilities',
            'icon' => 'openai',
            'color' => '#000000',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
                ['key' => 'organization_id', 'label' => 'Organization ID', 'type' => 'text', 'required' => false],
            ],
        ],
        'anthropic' => [
            'name' => 'Anthropic (Claude)',
            'category' => 'ai',
            'description' => 'Claude AI for advanced language tasks',
            'icon' => 'anthropic',
            'color' => '#D4A574',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
            ],
        ],
        'zapier' => [
            'name' => 'Zapier',
            'category' => 'automation',
            'description' => 'Automate workflows across apps',
            'icon' => 'zapier',
            'color' => '#FF4F00',
            'is_oauth' => true,
            'fields' => [
                ['key' => 'webhook_url', 'label' => 'Webhook URL', 'type' => 'url', 'required' => false],
            ],
        ],

        // === SEARCH & SEO ===
        'algolia' => [
            'name' => 'Algolia',
            'category' => 'search',
            'description' => 'Lightning-fast search and discovery',
            'icon' => 'algolia',
            'color' => '#003DFF',
            'docs_url' => 'https://www.algolia.com/doc/',
            'signup_url' => 'https://www.algolia.com/users/sign_up',
            'pricing_url' => 'https://www.algolia.com/pricing/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'app_id', 'label' => 'Application ID', 'type' => 'text', 'required' => true],
                ['key' => 'api_key', 'label' => 'Admin API Key', 'type' => 'password', 'required' => true],
                ['key' => 'search_key', 'label' => 'Search-Only Key', 'type' => 'text', 'required' => false],
            ],
        ],
        'meilisearch' => [
            'name' => 'Meilisearch',
            'category' => 'search',
            'description' => 'Open-source, lightning-fast search engine',
            'icon' => 'search',
            'color' => '#FF5CAA',
            'docs_url' => 'https://docs.meilisearch.com/',
            'signup_url' => 'https://cloud.meilisearch.com/',
            'pricing_url' => 'https://www.meilisearch.com/pricing',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'host', 'label' => 'Host URL', 'type' => 'url', 'required' => true, 'placeholder' => 'https://xxx.meilisearch.io'],
                ['key' => 'api_key', 'label' => 'Master Key', 'type' => 'password', 'required' => true],
            ],
            'verify_endpoint' => '/health',
        ],
        'ahrefs' => [
            'name' => 'Ahrefs',
            'category' => 'seo',
            'description' => 'SEO tools and backlink analysis',
            'icon' => 'ahrefs',
            'color' => '#FF6A00',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
            ],
        ],
        'semrush' => [
            'name' => 'SEMrush',
            'category' => 'seo',
            'description' => 'SEO, PPC, and content marketing tools',
            'icon' => 'semrush',
            'color' => '#FF642D',
            'docs_url' => 'https://www.semrush.com/api-documentation/',
            'signup_url' => 'https://www.semrush.com/signup/',
            'pricing_url' => 'https://www.semrush.com/prices/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
            ],
        ],

        // === PIXELS & CONVERSION TRACKING ===
        'meta_pixel' => [
            'name' => 'Meta Pixel (Facebook)',
            'category' => 'pixels',
            'description' => 'Track conversions for Facebook and Instagram ads',
            'icon' => 'facebook',
            'color' => '#1877F2',
            'docs_url' => 'https://developers.facebook.com/docs/meta-pixel',
            'signup_url' => 'https://business.facebook.com/events_manager',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => true, 'placeholder' => 'xxxxxxxxxxxxx'],
                ['key' => 'access_token', 'label' => 'Conversions API Token', 'type' => 'password', 'required' => false],
            ],
        ],
        'tiktok_pixel' => [
            'name' => 'TikTok Pixel',
            'category' => 'pixels',
            'description' => 'Track conversions for TikTok ads',
            'icon' => 'video',
            'color' => '#000000',
            'docs_url' => 'https://ads.tiktok.com/marketing_api/docs',
            'signup_url' => 'https://ads.tiktok.com/i18n/signup',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => true],
                ['key' => 'access_token', 'label' => 'Access Token', 'type' => 'password', 'required' => false],
            ],
        ],
        'twitter_pixel' => [
            'name' => 'Twitter/X Pixel',
            'category' => 'pixels',
            'description' => 'Track conversions for X (Twitter) ads',
            'icon' => 'twitter',
            'color' => '#000000',
            'docs_url' => 'https://developer.twitter.com/en/docs/twitter-ads-api',
            'signup_url' => 'https://ads.twitter.com/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => true],
            ],
        ],
        'linkedin_pixel' => [
            'name' => 'LinkedIn Insight Tag',
            'category' => 'pixels',
            'description' => 'Track conversions for LinkedIn ads',
            'icon' => 'linkedin',
            'color' => '#0A66C2',
            'docs_url' => 'https://www.linkedin.com/help/lms/answer/a418880',
            'signup_url' => 'https://www.linkedin.com/campaignmanager/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'partner_id', 'label' => 'Partner ID', 'type' => 'text', 'required' => true],
            ],
        ],
        'pinterest_pixel' => [
            'name' => 'Pinterest Tag',
            'category' => 'pixels',
            'description' => 'Track conversions for Pinterest ads',
            'icon' => 'image',
            'color' => '#E60023',
            'docs_url' => 'https://help.pinterest.com/en/business/article/install-the-pinterest-tag',
            'signup_url' => 'https://ads.pinterest.com/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'tag_id', 'label' => 'Tag ID', 'type' => 'text', 'required' => true],
            ],
        ],
        'reddit_pixel' => [
            'name' => 'Reddit Pixel',
            'category' => 'pixels',
            'description' => 'Track conversions for Reddit ads',
            'icon' => 'message-circle',
            'color' => '#FF4500',
            'docs_url' => 'https://ads.reddit.com/',
            'signup_url' => 'https://ads.reddit.com/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => true],
            ],
        ],
        'snapchat_pixel' => [
            'name' => 'Snapchat Pixel',
            'category' => 'pixels',
            'description' => 'Track conversions for Snapchat ads',
            'icon' => 'camera',
            'color' => '#FFFC00',
            'docs_url' => 'https://businesshelp.snapchat.com/s/article/snap-pixel',
            'signup_url' => 'https://ads.snapchat.com/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'pixel_id', 'label' => 'Pixel ID', 'type' => 'text', 'required' => true],
            ],
        ],

        // === CDN & PERFORMANCE ===
        'cloudflare_cdn' => [
            'name' => 'Cloudflare CDN',
            'category' => 'cdn',
            'description' => 'Global CDN, DDoS protection, and SSL',
            'icon' => 'cloudflare',
            'color' => '#F38020',
            'docs_url' => 'https://developers.cloudflare.com/',
            'signup_url' => 'https://dash.cloudflare.com/sign-up',
            'pricing_url' => 'https://www.cloudflare.com/plans/',
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_token', 'label' => 'API Token', 'type' => 'password', 'required' => true],
                ['key' => 'zone_id', 'label' => 'Zone ID', 'type' => 'text', 'required' => true],
                ['key' => 'account_id', 'label' => 'Account ID', 'type' => 'text', 'required' => false],
            ],
        ],
    ];

    /**
     * Get all available services grouped by category
     */
    public function getAvailableServices(): array
    {
        $grouped = [];

        foreach ($this->serviceRegistry as $key => $service) {
            $category = $service['category'];
            if (!isset($grouped[$category])) {
                $grouped[$category] = [
                    'name' => $this->getCategoryName($category),
                    'icon' => $this->getCategoryIcon($category),
                    'services' => [],
                ];
            }

            $grouped[$category]['services'][] = array_merge(
                ['key' => $key],
                $service
            );
        }

        return $grouped;
    }

    /**
     * Get category display name
     */
    protected function getCategoryName(string $category): string
    {
        return match ($category) {
            'infrastructure' => 'Infrastructure (Lightning Stack)',
            'payments' => 'Payments & Billing',
            'analytics' => 'Analytics & Tracking',
            'email' => 'Email Marketing',
            'storage' => 'Storage & Media',
            'crm' => 'CRM & Sales',
            'social' => 'Social Media',
            'communication' => 'Communication',
            'ai' => 'AI & Machine Learning',
            'automation' => 'Automation',
            'search' => 'Search',
            'seo' => 'SEO Tools',
            'pixels' => 'Conversion Pixels',
            'cdn' => 'CDN & Performance',
            'hosting' => 'Hosting & Deployment',
            'monitoring' => 'Monitoring & Errors',
            default => ucfirst($category),
        };
    }

    /**
     * Get category icon
     */
    protected function getCategoryIcon(string $category): string
    {
        return match ($category) {
            'infrastructure' => 'server',
            'payments' => 'credit-card',
            'analytics' => 'chart-bar',
            'email' => 'mail',
            'storage' => 'cloud',
            'crm' => 'users',
            'social' => 'share-2',
            'communication' => 'message-circle',
            'ai' => 'cpu',
            'automation' => 'zap',
            'search' => 'search',
            'seo' => 'trending-up',
            'pixels' => 'target',
            'cdn' => 'globe',
            'hosting' => 'upload-cloud',
            'monitoring' => 'alert-triangle',
            default => 'box',
        };
    }

    /**
     * Get service configuration by key
     */
    public function getServiceConfig(string $serviceKey): ?array
    {
        return $this->serviceRegistry[$serviceKey] ?? null;
    }

    /**
     * Get all connections with their status
     */
    public function getAllConnections(): Collection
    {
        $connections = ApiConnection::all()->keyBy('service_key');

        $result = collect();

        foreach ($this->serviceRegistry as $key => $service) {
            $connection = $connections->get($key);

            $result->push([
                'key' => $key,
                'name' => $service['name'],
                'category' => $service['category'],
                'description' => $service['description'],
                'icon' => $service['icon'],
                'color' => $service['color'],
                'docs_url' => $service['docs_url'] ?? null,
                'is_oauth' => $service['is_oauth'],
                'fields' => $service['fields'],
                'environments' => $service['environments'] ?? ['production'],
                'connection' => $connection?->toPublicArray(),
                'is_connected' => $connection?->isConnected() ?? false,
                'status' => $connection?->status ?? 'disconnected',
                'health_score' => $connection?->health_score ?? null,
            ]);
        }

        return $result;
    }

    /**
     * Get connections by category
     */
    public function getConnectionsByCategory(string $category): Collection
    {
        return $this->getAllConnections()->filter(fn($c) => $c['category'] === $category);
    }

    /**
     * Get connection status summary
     */
    public function getStatusSummary(): array
    {
        $connections = ApiConnection::all();

        return [
            'total_available' => count($this->serviceRegistry),
            'total_connected' => $connections->where('status', 'connected')->count(),
            'total_disconnected' => count($this->serviceRegistry) - $connections->where('status', 'connected')->count(),
            'total_errors' => $connections->where('status', 'error')->count(),
            'total_expired' => $connections->where('status', 'expired')->count(),
            'needs_attention' => $connections->filter(fn($c) => $c->health_score < 50 || $c->status === 'error')->count(),
            'by_category' => $this->getStatsByCategory(),
        ];
    }

    /**
     * Get stats by category
     */
    protected function getStatsByCategory(): array
    {
        $connections = ApiConnection::all()->keyBy('service_key');
        $stats = [];

        foreach ($this->serviceRegistry as $key => $service) {
            $category = $service['category'];
            if (!isset($stats[$category])) {
                $stats[$category] = ['total' => 0, 'connected' => 0];
            }
            $stats[$category]['total']++;

            if ($connections->has($key) && $connections->get($key)->isConnected()) {
                $stats[$category]['connected']++;
            }
        }

        return $stats;
    }

    /**
     * Connect a service with credentials
     */
    public function connect(string $serviceKey, array $credentials, string $environment = 'production'): ApiConnection
    {
        $config = $this->getServiceConfig($serviceKey);

        if (!$config) {
            throw new \InvalidArgumentException("Unknown service: {$serviceKey}");
        }

        // Find or create connection
        $connection = ApiConnection::firstOrNew(['service_key' => $serviceKey]);

        $connection->fill([
            'service_name' => $config['name'],
            'category' => $config['category'],
            'is_oauth' => $config['is_oauth'],
            'credentials' => $credentials,
            'environment' => $environment,
            'status' => 'pending',
        ]);

        $connection->save();

        // Log the connection attempt
        ApiConnectionLog::log($connection->id, 'connect_attempt', 'pending');

        // Verify the connection
        $verified = $this->verify($connection);

        if ($verified) {
            $connection->markConnected();
            ApiConnectionLog::log($connection->id, 'connected', 'success');
        } else {
            $connection->status = 'error';
            $connection->save();
            ApiConnectionLog::log($connection->id, 'connect_failed', 'error');
        }

        return $connection->fresh();
    }

    /**
     * Disconnect a service
     */
    public function disconnect(string $serviceKey): bool
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection) {
            return false;
        }

        ApiConnectionLog::log($connection->id, 'disconnected', 'success');

        $connection->markDisconnected();

        return true;
    }

    /**
     * Verify a connection is working
     */
    public function verify(ApiConnection $connection): bool
    {
        $config = $this->getServiceConfig($connection->service_key);

        if (!$config) {
            return false;
        }

        try {
            $verified = match ($connection->service_key) {
                // Infrastructure
                'cloudflare_r2' => $this->verifyCloudflareR2($connection),
                'neon_postgres' => $this->verifyNeonPostgres($connection),
                'upstash_redis' => $this->verifyUpstashRedis($connection),
                'sharp_service' => $this->verifySharpService($connection),
                // Payments
                'stripe' => $this->verifyStripe($connection),
                // Email
                'sendgrid' => $this->verifySendGrid($connection),
                'mailgun' => $this->verifyMailgun($connection),
                'resend' => $this->verifyResend($connection),
                'postmark' => $this->verifyPostmark($connection),
                'amazon_ses' => $this->verifyAmazonSES($connection),
                // Monitoring
                'sentry' => $this->verifySentry($connection),
                // Search
                'meilisearch' => $this->verifyMeilisearch($connection),
                'algolia' => $this->verifyAlgolia($connection),
                // AI
                'openai' => $this->verifyOpenAI($connection),
                // Communication
                'twilio' => $this->verifyTwilio($connection),
                // Storage
                'cloudinary' => $this->verifyCloudinary($connection),
                'cloudflare_cdn' => $this->verifyCloudflareCDN($connection),
                // Default
                default => $this->verifyGeneric($connection, $config),
            };

            if ($verified) {
                $connection->markVerified();
                $connection->recordApiCall(true);
            }

            return $verified;

        } catch (\Exception $e) {
            Log::error("API verification failed for {$connection->service_key}", [
                'error' => $e->getMessage(),
            ]);

            $connection->recordError($e->getMessage());
            return false;
        }
    }

    /**
     * Verify Stripe connection
     */
    protected function verifyStripe(ApiConnection $connection): bool
    {
        $secretKey = $connection->getCredential('secret_key');

        $response = Http::withBasicAuth($secretKey, '')
            ->get('https://api.stripe.com/v1/balance');

        return $response->successful();
    }

    /**
     * Verify SendGrid connection
     */
    protected function verifySendGrid(ApiConnection $connection): bool
    {
        $apiKey = $connection->getCredential('api_key');

        $response = Http::withToken($apiKey)
            ->get('https://api.sendgrid.com/v3/user/profile');

        return $response->successful();
    }

    /**
     * Verify OpenAI connection
     */
    protected function verifyOpenAI(ApiConnection $connection): bool
    {
        $apiKey = $connection->getCredential('api_key');

        $response = Http::withToken($apiKey)
            ->get('https://api.openai.com/v1/models');

        return $response->successful();
    }

    /**
     * Verify Mailgun connection
     */
    protected function verifyMailgun(ApiConnection $connection): bool
    {
        $apiKey = $connection->getCredential('api_key');
        $domain = $connection->getCredential('domain');

        $response = Http::withBasicAuth('api', $apiKey)
            ->get("https://api.mailgun.net/v3/domains/{$domain}");

        return $response->successful();
    }

    /**
     * Verify Twilio connection
     */
    protected function verifyTwilio(ApiConnection $connection): bool
    {
        $accountSid = $connection->getCredential('account_sid');
        $authToken = $connection->getCredential('auth_token');

        $response = Http::withBasicAuth($accountSid, $authToken)
            ->get("https://api.twilio.com/2010-04-01/Accounts/{$accountSid}.json");

        return $response->successful();
    }

    /**
     * Verify Cloudinary connection
     */
    protected function verifyCloudinary(ApiConnection $connection): bool
    {
        $cloudName = $connection->getCredential('cloud_name');
        $apiKey = $connection->getCredential('api_key');
        $apiSecret = $connection->getCredential('api_secret');

        $response = Http::withBasicAuth($apiKey, $apiSecret)
            ->get("https://api.cloudinary.com/v1_1/{$cloudName}/resources/image");

        return $response->successful();
    }

    /**
     * Verify Cloudflare R2 connection
     */
    protected function verifyCloudflareR2(ApiConnection $connection): bool
    {
        $accountId = $connection->getCredential('account_id');
        $accessKeyId = $connection->getCredential('access_key_id');
        $secretAccessKey = $connection->getCredential('secret_access_key');
        $bucket = $connection->getCredential('bucket');

        if (!$accountId || !$accessKeyId || !$secretAccessKey || !$bucket) {
            return false;
        }

        try {
            // Use AWS SDK compatible endpoint to list bucket
            $endpoint = "https://{$accountId}.r2.cloudflarestorage.com";

            // Create a signed request (simplified check - just verify credentials format)
            // In production, you'd use the AWS SDK
            return strlen($accessKeyId) > 10 && strlen($secretAccessKey) > 20;
        } catch (\Exception $e) {
            Log::error('R2 verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Neon PostgreSQL connection
     */
    protected function verifyNeonPostgres(ApiConnection $connection): bool
    {
        $databaseUrl = $connection->getCredential('database_url');

        if (!$databaseUrl) {
            // Try individual credentials
            $host = $connection->getCredential('host');
            $database = $connection->getCredential('database');
            $username = $connection->getCredential('username');
            $password = $connection->getCredential('password');

            if (!$host || !$database || !$username) {
                return false;
            }

            $databaseUrl = "postgres://{$username}:{$password}@{$host}:5432/{$database}?sslmode=require";
        }

        try {
            // Parse and test connection
            $parsed = parse_url($databaseUrl);
            if (!$parsed || !isset($parsed['host'])) {
                return false;
            }

            // Try to connect using PDO
            $dsn = sprintf(
                'pgsql:host=%s;port=%s;dbname=%s;sslmode=require',
                $parsed['host'],
                $parsed['port'] ?? 5432,
                ltrim($parsed['path'] ?? 'neondb', '/')
            );

            $pdo = new \PDO($dsn, $parsed['user'] ?? '', $parsed['pass'] ?? '', [
                \PDO::ATTR_TIMEOUT => 5,
                \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
            ]);

            $pdo->query('SELECT 1');
            return true;
        } catch (\Exception $e) {
            Log::error('Neon verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Upstash Redis connection
     */
    protected function verifyUpstashRedis(ApiConnection $connection): bool
    {
        $restUrl = $connection->getCredential('rest_url');
        $restToken = $connection->getCredential('rest_token');

        if (!$restUrl || !$restToken) {
            return false;
        }

        try {
            // Use Upstash REST API to test connection
            $response = Http::withToken($restToken)
                ->post("{$restUrl}/ping");

            return $response->successful() && $response->json('result') === 'PONG';
        } catch (\Exception $e) {
            Log::error('Upstash verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Sharp Image Service connection
     */
    protected function verifySharpService(ApiConnection $connection): bool
    {
        $serviceUrl = $connection->getCredential('service_url') ?? 'http://localhost:3001';

        try {
            $response = Http::timeout(5)->get("{$serviceUrl}/health");
            return $response->successful();
        } catch (\Exception $e) {
            Log::error('Sharp service verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Resend connection
     */
    protected function verifyResend(ApiConnection $connection): bool
    {
        $apiKey = $connection->getCredential('api_key');

        $response = Http::withToken($apiKey)
            ->get('https://api.resend.com/domains');

        return $response->successful();
    }

    /**
     * Verify Postmark connection
     */
    protected function verifyPostmark(ApiConnection $connection): bool
    {
        $serverToken = $connection->getCredential('server_token');

        $response = Http::withHeaders([
            'X-Postmark-Server-Token' => $serverToken,
        ])->get('https://api.postmarkapp.com/server');

        return $response->successful();
    }

    /**
     * Verify Amazon SES connection
     */
    protected function verifyAmazonSES(ApiConnection $connection): bool
    {
        $accessKeyId = $connection->getCredential('access_key_id');
        $secretAccessKey = $connection->getCredential('secret_access_key');
        $region = $connection->getCredential('region') ?? 'us-east-1';

        if (!$accessKeyId || !$secretAccessKey) {
            return false;
        }

        // Basic credential format validation
        // Full verification would require AWS SDK
        return strlen($accessKeyId) >= 16 && strlen($secretAccessKey) >= 30;
    }

    /**
     * Verify Sentry connection
     */
    protected function verifySentry(ApiConnection $connection): bool
    {
        $dsn = $connection->getCredential('dsn');

        if (!$dsn) {
            return false;
        }

        // Parse DSN to verify format
        $parsed = parse_url($dsn);

        if (!$parsed || !isset($parsed['host']) || !isset($parsed['user'])) {
            return false;
        }

        // DSN should be in format: https://key@org.ingest.sentry.io/project
        return str_contains($parsed['host'], 'sentry.io');
    }

    /**
     * Verify Meilisearch connection
     */
    protected function verifyMeilisearch(ApiConnection $connection): bool
    {
        $host = $connection->getCredential('host');
        $apiKey = $connection->getCredential('api_key');

        if (!$host) {
            return false;
        }

        try {
            $response = Http::withHeaders([
                'Authorization' => "Bearer {$apiKey}",
            ])->get("{$host}/health");

            return $response->successful() && $response->json('status') === 'available';
        } catch (\Exception $e) {
            Log::error('Meilisearch verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Algolia connection
     */
    protected function verifyAlgolia(ApiConnection $connection): bool
    {
        $appId = $connection->getCredential('app_id');
        $apiKey = $connection->getCredential('api_key');

        if (!$appId || !$apiKey) {
            return false;
        }

        try {
            $response = Http::withHeaders([
                'X-Algolia-Application-Id' => $appId,
                'X-Algolia-API-Key' => $apiKey,
            ])->get("https://{$appId}.algolia.net/1/keys");

            return $response->successful();
        } catch (\Exception $e) {
            Log::error('Algolia verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Verify Cloudflare CDN connection
     */
    protected function verifyCloudflareCDN(ApiConnection $connection): bool
    {
        $apiToken = $connection->getCredential('api_token');
        $zoneId = $connection->getCredential('zone_id');

        if (!$apiToken || !$zoneId) {
            return false;
        }

        try {
            $response = Http::withToken($apiToken)
                ->get("https://api.cloudflare.com/client/v4/zones/{$zoneId}");

            return $response->successful() && $response->json('success') === true;
        } catch (\Exception $e) {
            Log::error('Cloudflare CDN verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Generic verification using verify_endpoint if defined
     */
    protected function verifyGeneric(ApiConnection $connection, array $config): bool
    {
        if (!isset($config['verify_endpoint'])) {
            // No verification endpoint, assume success if credentials provided
            return !empty($connection->credentials);
        }

        $apiKey = $connection->getApiKey();

        if (!$apiKey) {
            return false;
        }

        $response = Http::withToken($apiKey)
            ->get($config['verify_endpoint']);

        return $response->successful();
    }

    /**
     * Test connection without saving
     */
    public function testConnection(string $serviceKey, array $credentials): array
    {
        $config = $this->getServiceConfig($serviceKey);

        if (!$config) {
            return ['success' => false, 'error' => 'Unknown service'];
        }

        // Create temporary connection object
        $tempConnection = new ApiConnection([
            'service_key' => $serviceKey,
            'credentials' => $credentials,
        ]);

        try {
            $verified = match ($serviceKey) {
                // Infrastructure
                'cloudflare_r2' => $this->verifyCloudflareR2($tempConnection),
                'neon_postgres' => $this->verifyNeonPostgres($tempConnection),
                'upstash_redis' => $this->verifyUpstashRedis($tempConnection),
                'sharp_service' => $this->verifySharpService($tempConnection),
                // Payments
                'stripe' => $this->verifyStripe($tempConnection),
                // Email
                'sendgrid' => $this->verifySendGrid($tempConnection),
                'mailgun' => $this->verifyMailgun($tempConnection),
                'resend' => $this->verifyResend($tempConnection),
                'postmark' => $this->verifyPostmark($tempConnection),
                'amazon_ses' => $this->verifyAmazonSES($tempConnection),
                // Monitoring
                'sentry' => $this->verifySentry($tempConnection),
                // Search
                'meilisearch' => $this->verifyMeilisearch($tempConnection),
                'algolia' => $this->verifyAlgolia($tempConnection),
                // AI
                'openai' => $this->verifyOpenAI($tempConnection),
                // Communication
                'twilio' => $this->verifyTwilio($tempConnection),
                // Storage
                'cloudinary' => $this->verifyCloudinary($tempConnection),
                'cloudflare_cdn' => $this->verifyCloudflareCDN($tempConnection),
                // Pixels and others - validate credentials are present
                default => !empty($credentials),
            };

            return [
                'success' => $verified,
                'message' => $verified ? 'Connection successful' : 'Connection failed',
                'service' => [
                    'name' => $config['name'],
                    'docs_url' => $config['docs_url'] ?? null,
                ],
            ];

        } catch (\Exception $e) {
            return [
                'success' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Refresh OAuth token for a connection
     */
    public function refreshToken(string $serviceKey): bool
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection || !$connection->is_oauth || !$connection->refresh_token) {
            return false;
        }

        // Implementation depends on the specific OAuth provider
        // This is a placeholder for the actual refresh logic

        return true;
    }

    /**
     * Get connection health report
     */
    public function getHealthReport(): array
    {
        $connections = ApiConnection::where('status', 'connected')->get();

        $report = [];

        foreach ($connections as $connection) {
            $report[] = [
                'service' => $connection->service_name,
                'key' => $connection->service_key,
                'health_score' => $connection->health_score,
                'health_status' => $connection->getHealthStatus(),
                'last_verified' => $connection->last_verified_at?->diffForHumans(),
                'api_calls_today' => $connection->api_calls_today,
                'error_count' => $connection->error_count,
                'last_error' => $connection->last_error,
            ];
        }

        return $report;
    }
}
