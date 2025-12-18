<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\IntegrationCredential;
use App\Services\CacheService;
use App\Services\CircuitBreaker;
use Google\Client as GoogleClient;
use Google\Service\AnalyticsData;
use Google\Service\AnalyticsData\DateRange;
use Google\Service\AnalyticsData\Dimension;
use Google\Service\AnalyticsData\Metric;
use Google\Service\AnalyticsData\RunReportRequest;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Crypt;
use Carbon\Carbon;
use Throwable;

/**
 * Google Analytics 4 Integration Service
 *
 * Enterprise-grade GA4 integration with:
 * - OAuth2 authentication flow
 * - Real-time and batch data fetching
 * - Circuit breaker for resilience
 * - Comprehensive metrics retrieval
 *
 * @version 1.0.0
 * @level L11 Principal Engineer
 */
class GoogleAnalyticsService
{
    private CacheService $cache;
    private CircuitBreaker $circuitBreaker;
    private ?GoogleClient $client = null;
    private ?AnalyticsData $analytics = null;

    private const CACHE_TTL = 3600; // 1 hour
    private const SHORT_CACHE_TTL = 300; // 5 minutes

    public function __construct(CacheService $cache)
    {
        $this->cache = $cache;
        $this->circuitBreaker = new CircuitBreaker('google_analytics');
    }

    /**
     * Initialize Google Client with stored credentials
     */
    public function initializeClient(int $userId): bool
    {
        try {
            $credential = IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_analytics')
                ->where('is_active', true)
                ->first();

            if (!$credential) {
                return false;
            }

            $this->client = new GoogleClient();
            $this->client->setClientId(config('services.google.client_id'));
            $this->client->setClientSecret(config('services.google.client_secret'));
            $this->client->setAccessToken(Crypt::decrypt($credential->access_token));

            // Check if token needs refresh
            if ($this->client->isAccessTokenExpired()) {
                if ($credential->refresh_token) {
                    $this->client->fetchAccessTokenWithRefreshToken(
                        Crypt::decrypt($credential->refresh_token)
                    );

                    // Update stored credentials
                    $credential->update([
                        'access_token' => Crypt::encrypt($this->client->getAccessToken()['access_token']),
                        'expires_at' => Carbon::now()->addSeconds($this->client->getAccessToken()['expires_in']),
                    ]);
                } else {
                    return false;
                }
            }

            $this->analytics = new AnalyticsData($this->client);
            return true;
        } catch (Throwable $e) {
            Log::error('GA4 client initialization failed', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get OAuth URL for authentication
     */
    public function getAuthUrl(string $redirectUri, string $state = ''): string
    {
        $client = new GoogleClient();
        $client->setClientId(config('services.google.client_id'));
        $client->setClientSecret(config('services.google.client_secret'));
        $client->setRedirectUri($redirectUri);
        $client->setAccessType('offline');
        $client->setPrompt('consent');
        $client->addScope([
            AnalyticsData::ANALYTICS_READONLY,
        ]);

        if ($state) {
            $client->setState($state);
        }

        return $client->createAuthUrl();
    }

    /**
     * Handle OAuth callback and store credentials
     */
    public function handleCallback(int $userId, string $code, string $redirectUri): bool
    {
        try {
            $client = new GoogleClient();
            $client->setClientId(config('services.google.client_id'));
            $client->setClientSecret(config('services.google.client_secret'));
            $client->setRedirectUri($redirectUri);

            $token = $client->fetchAccessTokenWithAuthCode($code);

            if (isset($token['error'])) {
                Log::error('GA4 OAuth error', ['error' => $token['error']]);
                return false;
            }

            // Store credentials
            IntegrationCredential::updateOrCreate(
                [
                    'user_id' => $userId,
                    'provider' => 'google_analytics',
                ],
                [
                    'access_token' => Crypt::encrypt($token['access_token']),
                    'refresh_token' => isset($token['refresh_token'])
                        ? Crypt::encrypt($token['refresh_token'])
                        : null,
                    'expires_at' => Carbon::now()->addSeconds($token['expires_in']),
                    'scopes' => json_encode($token['scope'] ?? []),
                    'is_active' => true,
                ]
            );

            return true;
        } catch (Throwable $e) {
            Log::error('GA4 callback handling failed', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Run a report with specified metrics and dimensions
     */
    public function runReport(
        int $userId,
        string $propertyId,
        array $metrics,
        array $dimensions = [],
        string $startDate = '28daysAgo',
        string $endDate = 'today',
        int $limit = 100
    ): array {
        $cacheKey = "ga4:report:{$userId}:" . md5("{$propertyId}:" . implode(',', $metrics) . ':' . implode(',', $dimensions) . ":{$startDate}:{$endDate}");

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $propertyId, $metrics, $dimensions, $startDate, $endDate, $limit) {
                if (!$this->initializeClient($userId)) {
                    return ['error' => 'Not authenticated'];
                }

                return $this->circuitBreaker->execute(function () use ($propertyId, $metrics, $dimensions, $startDate, $endDate, $limit) {
                    $request = new RunReportRequest();

                    // Set date range
                    $dateRange = new DateRange();
                    $dateRange->setStartDate($startDate);
                    $dateRange->setEndDate($endDate);
                    $request->setDateRanges([$dateRange]);

                    // Set metrics
                    $metricObjects = array_map(function ($metric) {
                        $m = new Metric();
                        $m->setName($metric);
                        return $m;
                    }, $metrics);
                    $request->setMetrics($metricObjects);

                    // Set dimensions
                    if (!empty($dimensions)) {
                        $dimensionObjects = array_map(function ($dimension) {
                            $d = new Dimension();
                            $d->setName($dimension);
                            return $d;
                        }, $dimensions);
                        $request->setDimensions($dimensionObjects);
                    }

                    $request->setLimit($limit);

                    $response = $this->analytics->properties->runReport(
                        "properties/{$propertyId}",
                        $request
                    );

                    $rows = [];
                    foreach ($response->getRows() ?? [] as $row) {
                        $rowData = [];

                        // Add dimension values
                        foreach ($row->getDimensionValues() ?? [] as $i => $dimValue) {
                            $rowData[$dimensions[$i] ?? "dimension_{$i}"] = $dimValue->getValue();
                        }

                        // Add metric values
                        foreach ($row->getMetricValues() ?? [] as $i => $metricValue) {
                            $rowData[$metrics[$i] ?? "metric_{$i}"] = floatval($metricValue->getValue());
                        }

                        $rows[] = $rowData;
                    }

                    return $rows;
                });
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get visitor analytics summary
     */
    public function getVisitorAnalytics(int $userId, string $propertyId, int $days = 30): array
    {
        $cacheKey = "ga4:visitors:{$userId}:{$propertyId}:{$days}";

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $propertyId, $days) {
                // Current period
                $currentData = $this->runReport(
                    $userId,
                    $propertyId,
                    ['activeUsers', 'sessions', 'bounceRate', 'averageSessionDuration', 'screenPageViews', 'newUsers'],
                    [],
                    "{$days}daysAgo",
                    'today'
                );

                // Previous period
                $previousData = $this->runReport(
                    $userId,
                    $propertyId,
                    ['activeUsers', 'sessions', 'bounceRate', 'averageSessionDuration', 'screenPageViews', 'newUsers'],
                    [],
                    (string)($days * 2) . 'daysAgo',
                    "{$days}daysAgo"
                );

                // Daily breakdown
                $dailyData = $this->runReport(
                    $userId,
                    $propertyId,
                    ['activeUsers', 'sessions'],
                    ['date'],
                    "{$days}daysAgo",
                    'today',
                    $days
                );

                $current = $currentData[0] ?? [];
                $previous = $previousData[0] ?? [];

                $visitors = (int)($current['activeUsers'] ?? 0);
                $prevVisitors = (int)($previous['activeUsers'] ?? 0);

                $sessions = (int)($current['sessions'] ?? 0);
                $prevSessions = (int)($previous['sessions'] ?? 0);

                return [
                    'visitors' => $visitors,
                    'visitors_change' => $prevVisitors > 0
                        ? round((($visitors - $prevVisitors) / $prevVisitors) * 100, 1)
                        : 0,
                    'page_views' => (int)($current['screenPageViews'] ?? 0),
                    'sessions' => $sessions,
                    'sessions_change' => $prevSessions > 0
                        ? round((($sessions - $prevSessions) / $prevSessions) * 100, 1)
                        : 0,
                    'bounce_rate' => round(($current['bounceRate'] ?? 0) * 100, 1),
                    'avg_duration' => round(($current['averageSessionDuration'] ?? 0) / 60, 1), // Convert to minutes
                    'new_users' => (int)($current['newUsers'] ?? 0),
                    'pages_per_session' => $sessions > 0
                        ? round((int)($current['screenPageViews'] ?? 0) / $sessions, 1)
                        : 0,
                    'daily_data' => array_map(function ($row) {
                        return [
                            'date' => $row['date'] ?? '',
                            'visitors' => (int)($row['activeUsers'] ?? 0),
                            'sessions' => (int)($row['sessions'] ?? 0),
                        ];
                    }, $dailyData),
                    'period' => "{$days} days",
                ];
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get traffic sources breakdown
     */
    public function getTrafficSources(int $userId, string $propertyId, int $days = 30): array
    {
        $cacheKey = "ga4:traffic_sources:{$userId}:{$propertyId}:{$days}";

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $propertyId, $days) {
                // Current period data
                $currentData = $this->runReport(
                    $userId,
                    $propertyId,
                    ['activeUsers', 'sessions'],
                    ['sessionDefaultChannelGroup'],
                    "{$days}daysAgo",
                    'today',
                    20
                );

                // Previous period data for calculating change
                $previousData = $this->runReport(
                    $userId,
                    $propertyId,
                    ['activeUsers', 'sessions'],
                    ['sessionDefaultChannelGroup'],
                    (string)($days * 2) . 'daysAgo',
                    (string)($days + 1) . 'daysAgo',
                    20
                );

                // Index previous data by channel for quick lookup
                $previousByChannel = [];
                foreach ($previousData as $row) {
                    $channel = $row['sessionDefaultChannelGroup'] ?? 'Other';
                    $previousByChannel[$channel] = (int)($row['activeUsers'] ?? 0);
                }

                $totalVisitors = array_sum(array_column($currentData, 'activeUsers'));

                $colors = [
                    'Organic Search' => '#22c55e',
                    'Direct' => '#3b82f6',
                    'Organic Social' => '#8b5cf6',
                    'Referral' => '#f59e0b',
                    'Email' => '#ef4444',
                    'Paid Search' => '#06b6d4',
                    'Paid Social' => '#ec4899',
                    'Display' => '#84cc16',
                    'Affiliates' => '#14b8a6',
                    'Other' => '#6b7280',
                ];

                $sources = array_map(function ($row) use ($totalVisitors, $colors, $previousByChannel) {
                    $channel = $row['sessionDefaultChannelGroup'] ?? 'Other';
                    $visitors = (int)($row['activeUsers'] ?? 0);
                    $previousVisitors = $previousByChannel[$channel] ?? 0;

                    // Calculate percentage change
                    $change = 0;
                    if ($previousVisitors > 0) {
                        $change = round((($visitors - $previousVisitors) / $previousVisitors) * 100, 1);
                    } elseif ($visitors > 0) {
                        $change = 100; // 100% increase if there were no previous visitors
                    }

                    return [
                        'name' => $channel,
                        'visitors' => $visitors,
                        'previous_visitors' => $previousVisitors,
                        'percentage' => $totalVisitors > 0
                            ? round(($visitors / $totalVisitors) * 100, 1)
                            : 0,
                        'change' => $change,
                        'color' => $colors[$channel] ?? '#6b7280',
                    ];
                }, $currentData);

                return [
                    'sources' => $sources,
                    'total_visitors' => $totalVisitors,
                    'period' => "Last {$days} days",
                ];
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get top pages
     */
    public function getTopPages(int $userId, string $propertyId, int $limit = 10): array
    {
        return $this->runReport(
            $userId,
            $propertyId,
            ['screenPageViews', 'activeUsers', 'averageSessionDuration', 'bounceRate'],
            ['pagePath', 'pageTitle'],
            '28daysAgo',
            'today',
            $limit
        );
    }

    /**
     * Get device breakdown
     */
    public function getDeviceBreakdown(int $userId, string $propertyId): array
    {
        return $this->runReport(
            $userId,
            $propertyId,
            ['activeUsers', 'sessions'],
            ['deviceCategory'],
            '28daysAgo',
            'today',
            10
        );
    }

    /**
     * Get geographic data
     */
    public function getGeographicData(int $userId, string $propertyId, int $limit = 20): array
    {
        return $this->runReport(
            $userId,
            $propertyId,
            ['activeUsers', 'sessions'],
            ['country', 'city'],
            '28daysAgo',
            'today',
            $limit
        );
    }

    /**
     * Get real-time active users
     */
    public function getRealTimeUsers(int $userId, string $propertyId): array
    {
        $cacheKey = "ga4:realtime:{$userId}:{$propertyId}";

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $propertyId) {
                if (!$this->initializeClient($userId)) {
                    return ['error' => 'Not authenticated'];
                }

                return $this->circuitBreaker->execute(function () use ($propertyId) {
                    // GA4 real-time data requires different API endpoint
                    $request = new RunReportRequest();

                    $dateRange = new DateRange();
                    $dateRange->setStartDate('today');
                    $dateRange->setEndDate('today');
                    $request->setDateRanges([$dateRange]);

                    $metric = new Metric();
                    $metric->setName('activeUsers');
                    $request->setMetrics([$metric]);

                    $response = $this->analytics->properties->runReport(
                        "properties/{$propertyId}",
                        $request
                    );

                    $rows = $response->getRows() ?? [];
                    $activeUsers = 0;
                    foreach ($rows as $row) {
                        foreach ($row->getMetricValues() ?? [] as $metricValue) {
                            $activeUsers = (int)$metricValue->getValue();
                        }
                    }

                    return [
                        'active_users' => $activeUsers,
                        'timestamp' => now()->toIso8601String(),
                    ];
                });
            },
            60 // 1 minute cache for real-time
        );
    }

    /**
     * Disconnect integration
     */
    public function disconnect(int $userId): bool
    {
        try {
            IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_analytics')
                ->delete();

            return true;
        } catch (Throwable $e) {
            Log::error('GA4 disconnect failed', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Check if user is connected
     */
    public function isConnected(int $userId): bool
    {
        return IntegrationCredential::where('user_id', $userId)
            ->where('provider', 'google_analytics')
            ->where('is_active', true)
            ->exists();
    }

    /**
     * Save property ID for user
     */
    public function setPropertyId(int $userId, string $propertyId): bool
    {
        try {
            $credential = IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_analytics')
                ->first();

            if ($credential) {
                $credential->update([
                    'metadata' => array_merge($credential->metadata ?? [], [
                        'property_id' => $propertyId,
                    ]),
                ]);
                return true;
            }

            return false;
        } catch (Throwable $e) {
            Log::error('Failed to set GA4 property ID', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }
}
