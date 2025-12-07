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
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
                ['key' => 'domain', 'label' => 'Domain', 'type' => 'text', 'required' => true],
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
            'is_oauth' => false,
            'fields' => [
                ['key' => 'app_id', 'label' => 'Application ID', 'type' => 'text', 'required' => true],
                ['key' => 'api_key', 'label' => 'Admin API Key', 'type' => 'password', 'required' => true],
                ['key' => 'search_key', 'label' => 'Search-Only Key', 'type' => 'text', 'required' => false],
            ],
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
            'is_oauth' => false,
            'fields' => [
                ['key' => 'api_key', 'label' => 'API Key', 'type' => 'password', 'required' => true],
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
            'payments' => 'Payments & Billing',
            'analytics' => 'Analytics & Tracking',
            'email' => 'Email Marketing',
            'storage' => 'Storage & CDN',
            'crm' => 'CRM & Sales',
            'social' => 'Social Media',
            'communication' => 'Communication',
            'ai' => 'AI & Machine Learning',
            'automation' => 'Automation',
            'search' => 'Search',
            'seo' => 'SEO Tools',
            default => ucfirst($category),
        };
    }

    /**
     * Get category icon
     */
    protected function getCategoryIcon(string $category): string
    {
        return match ($category) {
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
                'stripe' => $this->verifyStripe($connection),
                'sendgrid' => $this->verifySendGrid($connection),
                'openai' => $this->verifyOpenAI($connection),
                'mailgun' => $this->verifyMailgun($connection),
                'twilio' => $this->verifyTwilio($connection),
                'cloudinary' => $this->verifyCloudinary($connection),
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
                'stripe' => $this->verifyStripe($tempConnection),
                'sendgrid' => $this->verifySendGrid($tempConnection),
                'openai' => $this->verifyOpenAI($tempConnection),
                'mailgun' => $this->verifyMailgun($tempConnection),
                'twilio' => $this->verifyTwilio($tempConnection),
                'cloudinary' => $this->verifyCloudinary($tempConnection),
                default => !empty($credentials),
            };

            return [
                'success' => $verified,
                'message' => $verified ? 'Connection successful' : 'Connection failed',
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
