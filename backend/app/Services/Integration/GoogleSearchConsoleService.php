<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\IntegrationCredential;
use App\Services\CacheService;
use App\Services\CircuitBreaker;
use Google\Client as GoogleClient;
use Google\Service\SearchConsole;
use Google\Service\SearchConsole\SearchAnalyticsQueryRequest;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Crypt;
use Carbon\Carbon;
use Throwable;

/**
 * Google Search Console Integration Service
 *
 * Enterprise-grade GSC integration with:
 * - OAuth2 authentication flow
 * - Data fetching with caching
 * - Circuit breaker for resilience
 * - Comprehensive metrics retrieval
 *
 * @version 1.0.0
 * @level L11 Principal Engineer
 */
class GoogleSearchConsoleService
{
    private CacheService $cache;
    private CircuitBreaker $circuitBreaker;
    private ?GoogleClient $client = null;
    private ?SearchConsole $searchConsole = null;

    private const CACHE_TTL = 3600; // 1 hour
    private const SHORT_CACHE_TTL = 300; // 5 minutes

    public function __construct(CacheService $cache)
    {
        $this->cache = $cache;
        $this->circuitBreaker = new CircuitBreaker('google_search_console');
    }

    /**
     * Initialize Google Client with stored credentials
     */
    public function initializeClient(int $userId): bool
    {
        try {
            $credential = IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_search_console')
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

            $this->searchConsole = new SearchConsole($this->client);
            return true;
        } catch (Throwable $e) {
            Log::error('GSC client initialization failed', [
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
            SearchConsole::WEBMASTERS_READONLY,
            SearchConsole::WEBMASTERS,
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
                Log::error('GSC OAuth error', ['error' => $token['error']]);
                return false;
            }

            // Store credentials
            IntegrationCredential::updateOrCreate(
                [
                    'user_id' => $userId,
                    'provider' => 'google_search_console',
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
            Log::error('GSC callback handling failed', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get list of connected sites
     */
    public function getSites(int $userId): array
    {
        return $this->cache->get(
            "gsc:sites:{$userId}",
            function () use ($userId) {
                if (!$this->initializeClient($userId)) {
                    return [];
                }

                return $this->circuitBreaker->execute(function () {
                    $sites = $this->searchConsole->sites->listSites();
                    return array_map(function ($site) {
                        return [
                            'url' => $site->getSiteUrl(),
                            'permission_level' => $site->getPermissionLevel(),
                        ];
                    }, $sites->getSiteEntry() ?? []);
                });
            },
            self::CACHE_TTL
        );
    }

    /**
     * Get search analytics data
     */
    public function getSearchAnalytics(
        int $userId,
        string $siteUrl,
        string $startDate,
        string $endDate,
        array $dimensions = ['query'],
        int $rowLimit = 100
    ): array {
        $cacheKey = "gsc:analytics:{$userId}:" . md5("{$siteUrl}:{$startDate}:{$endDate}:" . implode(',', $dimensions));

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $siteUrl, $startDate, $endDate, $dimensions, $rowLimit) {
                if (!$this->initializeClient($userId)) {
                    return ['error' => 'Not authenticated'];
                }

                return $this->circuitBreaker->execute(function () use ($siteUrl, $startDate, $endDate, $dimensions, $rowLimit) {
                    $request = new SearchAnalyticsQueryRequest();
                    $request->setStartDate($startDate);
                    $request->setEndDate($endDate);
                    $request->setDimensions($dimensions);
                    $request->setRowLimit($rowLimit);
                    $request->setDataState('all');

                    $response = $this->searchConsole->searchanalytics->query($siteUrl, $request);

                    return array_map(function ($row) use ($dimensions) {
                        $data = [
                            'clicks' => $row->getClicks(),
                            'impressions' => $row->getImpressions(),
                            'ctr' => round($row->getCtr() * 100, 2),
                            'position' => round($row->getPosition(), 1),
                        ];

                        $keys = $row->getKeys();
                        foreach ($dimensions as $index => $dimension) {
                            $data[$dimension] = $keys[$index] ?? null;
                        }

                        return $data;
                    }, $response->getRows() ?? []);
                });
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get keyword rankings
     */
    public function getKeywordRankings(int $userId, string $siteUrl, int $days = 28): array
    {
        $endDate = Carbon::now()->format('Y-m-d');
        $startDate = Carbon::now()->subDays($days)->format('Y-m-d');
        $previousEndDate = Carbon::now()->subDays($days)->format('Y-m-d');
        $previousStartDate = Carbon::now()->subDays($days * 2)->format('Y-m-d');

        $cacheKey = "gsc:keywords:{$userId}:" . md5("{$siteUrl}:{$startDate}:{$endDate}");

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $siteUrl, $startDate, $endDate, $previousStartDate, $previousEndDate) {
                // Current period
                $current = $this->getSearchAnalytics(
                    $userId,
                    $siteUrl,
                    $startDate,
                    $endDate,
                    ['query', 'page'],
                    500
                );

                // Previous period for comparison
                $previous = $this->getSearchAnalytics(
                    $userId,
                    $siteUrl,
                    $previousStartDate,
                    $previousEndDate,
                    ['query'],
                    500
                );

                $previousMap = [];
                foreach ($previous as $row) {
                    $previousMap[$row['query']] = $row['position'];
                }

                $keywords = [];
                $improved = 0;
                $declined = 0;
                $top10 = 0;
                $positionSum = 0;

                foreach ($current as $row) {
                    $previousPosition = $previousMap[$row['query']] ?? null;
                    $change = $previousPosition ? round($previousPosition - $row['position'], 1) : 0;

                    if ($change > 0) {
                        $improved++;
                        $trend = 'up';
                    } elseif ($change < 0) {
                        $declined++;
                        $trend = 'down';
                    } else {
                        $trend = 'stable';
                    }

                    if ($row['position'] <= 10) {
                        $top10++;
                    }

                    $positionSum += $row['position'];

                    $keywords[] = [
                        'keyword' => $row['query'],
                        'position' => round($row['position'], 1),
                        'previous_position' => $previousPosition ? round($previousPosition, 1) : null,
                        'change' => $change,
                        'trend' => $trend,
                        'url' => $row['page'] ?? '',
                        'clicks' => $row['clicks'],
                        'impressions' => $row['impressions'],
                        'ctr' => $row['ctr'],
                        'search_volume' => $row['impressions'], // Using impressions as proxy
                    ];
                }

                return [
                    'keywords' => $keywords,
                    'total_keywords' => count($keywords),
                    'avg_position' => count($keywords) > 0 ? round($positionSum / count($keywords), 1) : 0,
                    'top_10_count' => $top10,
                    'improved_count' => $improved,
                    'declined_count' => $declined,
                ];
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get performance summary
     */
    public function getPerformanceSummary(int $userId, string $siteUrl, int $days = 28): array
    {
        $endDate = Carbon::now()->format('Y-m-d');
        $startDate = Carbon::now()->subDays($days)->format('Y-m-d');
        $previousEndDate = Carbon::now()->subDays($days)->format('Y-m-d');
        $previousStartDate = Carbon::now()->subDays($days * 2)->format('Y-m-d');

        $cacheKey = "gsc:performance:{$userId}:" . md5("{$siteUrl}:{$startDate}:{$endDate}");

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $siteUrl, $startDate, $endDate, $previousStartDate, $previousEndDate) {
                // Current period by date
                $current = $this->getSearchAnalytics(
                    $userId,
                    $siteUrl,
                    $startDate,
                    $endDate,
                    ['date'],
                    1000
                );

                // Previous period totals
                $previous = $this->getSearchAnalytics(
                    $userId,
                    $siteUrl,
                    $previousStartDate,
                    $previousEndDate,
                    [],
                    1
                );

                $totalClicks = 0;
                $totalImpressions = 0;
                $totalCtr = 0;
                $totalPosition = 0;
                $dailyData = [];

                foreach ($current as $row) {
                    $totalClicks += $row['clicks'];
                    $totalImpressions += $row['impressions'];
                    $totalCtr += $row['ctr'];
                    $totalPosition += $row['position'];

                    $dailyData[] = [
                        'date' => $row['date'],
                        'clicks' => $row['clicks'],
                        'impressions' => $row['impressions'],
                    ];
                }

                $count = count($current);
                $avgCtr = $count > 0 ? round($totalCtr / $count, 2) : 0;
                $avgPosition = $count > 0 ? round($totalPosition / $count, 1) : 0;

                // Calculate changes
                $prevClicks = $previous[0]['clicks'] ?? 0;
                $prevImpressions = $previous[0]['impressions'] ?? 0;

                return [
                    'total_clicks' => $totalClicks,
                    'total_impressions' => $totalImpressions,
                    'avg_ctr' => $avgCtr,
                    'avg_position' => $avgPosition,
                    'clicks_change' => $prevClicks > 0
                        ? round((($totalClicks - $prevClicks) / $prevClicks) * 100, 1)
                        : 0,
                    'impressions_change' => $prevImpressions > 0
                        ? round((($totalImpressions - $prevImpressions) / $prevImpressions) * 100, 1)
                        : 0,
                    'daily_data' => $dailyData,
                    'period' => "{$days} days",
                ];
            },
            self::SHORT_CACHE_TTL
        );
    }

    /**
     * Get top pages
     */
    public function getTopPages(int $userId, string $siteUrl, int $limit = 20): array
    {
        $endDate = Carbon::now()->format('Y-m-d');
        $startDate = Carbon::now()->subDays(28)->format('Y-m-d');

        return $this->getSearchAnalytics(
            $userId,
            $siteUrl,
            $startDate,
            $endDate,
            ['page'],
            $limit
        );
    }

    /**
     * Get indexing status (requires Indexing API)
     */
    public function getIndexingStatus(int $userId, string $siteUrl): array
    {
        $cacheKey = "gsc:indexing:{$userId}:" . md5($siteUrl);

        return $this->cache->get(
            $cacheKey,
            function () use ($userId, $siteUrl) {
                if (!$this->initializeClient($userId)) {
                    return ['error' => 'Not authenticated'];
                }

                // Note: Full indexing data requires Search Console API v1
                // This is a simplified version
                return $this->circuitBreaker->execute(function () use ($siteUrl) {
                    // Get site verification status
                    $site = $this->searchConsole->sites->get($siteUrl);

                    return [
                        'site_url' => $site->getSiteUrl(),
                        'permission_level' => $site->getPermissionLevel(),
                        'is_verified' => $site->getPermissionLevel() !== 'siteUnverifiedUser',
                    ];
                });
            },
            self::CACHE_TTL
        );
    }

    /**
     * Disconnect integration
     */
    public function disconnect(int $userId): bool
    {
        try {
            IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_search_console')
                ->delete();

            // Clear cache
            $this->cache->forget("gsc:sites:{$userId}");

            return true;
        } catch (Throwable $e) {
            Log::error('GSC disconnect failed', [
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
            ->where('provider', 'google_search_console')
            ->where('is_active', true)
            ->exists();
    }
}
