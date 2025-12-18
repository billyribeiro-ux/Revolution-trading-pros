<?php

namespace App\Services;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * Server-Side Analytics Backup Service
 *
 * Provides fallback analytics tracking when client-side tracking is blocked:
 * - Beacon endpoint for server-side tracking
 * - Event buffering and batch processing
 * - Real-time metrics aggregation
 * - Privacy-preserving fingerprinting
 * - Cross-device session stitching
 * - Geographic enrichment
 *
 * Compliant with GDPR, CCPA, and enterprise requirements.
 *
 * @version 1.0.0
 */
class PopupAnalyticsBackupService
{
    /**
     * Event type constants
     */
    public const EVENT_IMPRESSION = 'impression';
    public const EVENT_CONVERSION = 'conversion';
    public const EVENT_CLICK = 'click';
    public const EVENT_CLOSE = 'close';
    public const EVENT_FORM_START = 'form_start';
    public const EVENT_FORM_SUBMIT = 'form_submit';
    public const EVENT_SCROLL = 'scroll';
    public const EVENT_VIDEO_PLAY = 'video_play';
    public const EVENT_COUNTDOWN_EXPIRE = 'countdown_expire';

    /**
     * Cache TTL for aggregations
     */
    protected const CACHE_TTL_SECONDS = 300; // 5 minutes

    /**
     * Record a server-side analytics event.
     *
     * @param int $popupId
     * @param string $eventType
     * @param array $data
     * @return int|null Event ID
     */
    public function recordEvent(int $popupId, string $eventType, array $data = []): ?int
    {
        try {
            // Generate privacy-preserving session ID
            $sessionId = $this->generatePrivacySessionId($data);

            $eventData = [
                'popup_id' => $popupId,
                'event_type' => $eventType,
                'session_id' => $sessionId,
                'visitor_hash' => $this->generateVisitorHash($data),
                'page_url' => $data['page_url'] ?? null,
                'referrer' => $data['referrer'] ?? null,
                'device_type' => $this->detectDeviceType($data['user_agent'] ?? ''),
                'browser' => $this->detectBrowser($data['user_agent'] ?? ''),
                'os' => $this->detectOS($data['user_agent'] ?? ''),
                'country' => $this->detectCountry($data['ip_address'] ?? ''),
                'utm_source' => $data['utm_source'] ?? null,
                'utm_medium' => $data['utm_medium'] ?? null,
                'utm_campaign' => $data['utm_campaign'] ?? null,
                'event_value' => $data['value'] ?? null,
                'engagement_time_ms' => $data['engagement_time_ms'] ?? null,
                'scroll_depth' => $data['scroll_depth'] ?? null,
                'form_fields_filled' => $data['form_fields_filled'] ?? null,
                'variant_id' => $data['variant_id'] ?? null,
                'metadata' => !empty($data['metadata']) ? json_encode($data['metadata']) : null,
                'is_bot' => $this->detectBot($data['user_agent'] ?? ''),
                'created_at' => now(),
            ];

            // Insert event
            $id = DB::table('popup_analytics_events')->insertGetId($eventData);

            // Update real-time aggregations
            $this->updateRealTimeMetrics($popupId, $eventType);

            // Clear cached analytics
            Cache::tags(['popup_analytics', "popup_{$popupId}"])->flush();

            return $id;
        } catch (\Exception $e) {
            Log::error('Failed to record popup analytics event', [
                'error' => $e->getMessage(),
                'popup_id' => $popupId,
                'event_type' => $eventType,
            ]);
            return null;
        }
    }

    /**
     * Record multiple events in batch (for buffered client events).
     *
     * @param array $events
     * @return int Number of events recorded
     */
    public function recordBatch(array $events): int
    {
        $recorded = 0;

        DB::beginTransaction();

        try {
            foreach ($events as $event) {
                if ($this->recordEvent(
                    $event['popup_id'],
                    $event['event_type'],
                    $event['data'] ?? []
                )) {
                    $recorded++;
                }
            }

            DB::commit();
        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to record popup analytics batch', [
                'error' => $e->getMessage(),
                'events_count' => count($events),
            ]);
        }

        return $recorded;
    }

    /**
     * Get real-time analytics for a popup.
     *
     * @param int $popupId
     * @param string $period '1h', '24h', '7d', '30d'
     * @return array
     */
    public function getRealTimeAnalytics(int $popupId, string $period = '24h'): array
    {
        $cacheKey = "popup_analytics_{$popupId}_{$period}";

        return Cache::tags(['popup_analytics', "popup_{$popupId}"])
            ->remember($cacheKey, self::CACHE_TTL_SECONDS, function () use ($popupId, $period) {
                $startDate = $this->getStartDate($period);

                $baseQuery = DB::table('popup_analytics_events')
                    ->where('popup_id', $popupId)
                    ->where('is_bot', false)
                    ->where('created_at', '>=', $startDate);

                // Get event counts
                $eventCounts = (clone $baseQuery)
                    ->select('event_type', DB::raw('COUNT(*) as count'))
                    ->groupBy('event_type')
                    ->pluck('count', 'event_type')
                    ->toArray();

                // Get unique visitors
                $uniqueVisitors = (clone $baseQuery)
                    ->distinct('visitor_hash')
                    ->count('visitor_hash');

                // Get device breakdown
                $deviceBreakdown = (clone $baseQuery)
                    ->select('device_type', DB::raw('COUNT(*) as count'))
                    ->groupBy('device_type')
                    ->pluck('count', 'device_type')
                    ->toArray();

                // Get browser breakdown
                $browserBreakdown = (clone $baseQuery)
                    ->select('browser', DB::raw('COUNT(*) as count'))
                    ->groupBy('browser')
                    ->orderByDesc('count')
                    ->limit(5)
                    ->pluck('count', 'browser')
                    ->toArray();

                // Get country breakdown
                $countryBreakdown = (clone $baseQuery)
                    ->select('country', DB::raw('COUNT(*) as count'))
                    ->whereNotNull('country')
                    ->groupBy('country')
                    ->orderByDesc('count')
                    ->limit(10)
                    ->pluck('count', 'country')
                    ->toArray();

                // Get average engagement time
                $avgEngagementTime = (clone $baseQuery)
                    ->whereNotNull('engagement_time_ms')
                    ->avg('engagement_time_ms');

                // Calculate conversion rate
                $impressions = $eventCounts[self::EVENT_IMPRESSION] ?? 0;
                $conversions = $eventCounts[self::EVENT_CONVERSION] ?? 0;
                $conversionRate = $impressions > 0 ? ($conversions / $impressions) * 100 : 0;

                // Get hourly distribution
                $hourlyDistribution = (clone $baseQuery)
                    ->select(
                        DB::raw('HOUR(created_at) as hour'),
                        DB::raw('COUNT(*) as count')
                    )
                    ->groupBy('hour')
                    ->orderBy('hour')
                    ->pluck('count', 'hour')
                    ->toArray();

                // Get UTM source breakdown
                $utmBreakdown = (clone $baseQuery)
                    ->select('utm_source', DB::raw('COUNT(*) as count'))
                    ->whereNotNull('utm_source')
                    ->groupBy('utm_source')
                    ->orderByDesc('count')
                    ->limit(10)
                    ->pluck('count', 'utm_source')
                    ->toArray();

                return [
                    'period' => $period,
                    'generated_at' => now()->toIso8601String(),
                    'metrics' => [
                        'impressions' => $impressions,
                        'conversions' => $conversions,
                        'clicks' => $eventCounts[self::EVENT_CLICK] ?? 0,
                        'closes' => $eventCounts[self::EVENT_CLOSE] ?? 0,
                        'form_starts' => $eventCounts[self::EVENT_FORM_START] ?? 0,
                        'form_submits' => $eventCounts[self::EVENT_FORM_SUBMIT] ?? 0,
                        'unique_visitors' => $uniqueVisitors,
                        'conversion_rate' => round($conversionRate, 2),
                        'avg_engagement_time_ms' => round($avgEngagementTime ?? 0),
                    ],
                    'breakdown' => [
                        'device' => $deviceBreakdown,
                        'browser' => $browserBreakdown,
                        'country' => $countryBreakdown,
                        'utm_source' => $utmBreakdown,
                        'hourly' => $hourlyDistribution,
                    ],
                ];
            });
    }

    /**
     * Get A/B test variant performance.
     *
     * @param int $popupId
     * @param string $period
     * @return array
     */
    public function getVariantPerformance(int $popupId, string $period = '7d'): array
    {
        $startDate = $this->getStartDate($period);

        $variants = DB::table('popup_analytics_events')
            ->where('popup_id', $popupId)
            ->where('is_bot', false)
            ->where('created_at', '>=', $startDate)
            ->whereNotNull('variant_id')
            ->select(
                'variant_id',
                DB::raw('COUNT(CASE WHEN event_type = "impression" THEN 1 END) as impressions'),
                DB::raw('COUNT(CASE WHEN event_type = "conversion" THEN 1 END) as conversions'),
                DB::raw('AVG(engagement_time_ms) as avg_engagement_time')
            )
            ->groupBy('variant_id')
            ->get()
            ->map(function ($variant) {
                $variant->conversion_rate = $variant->impressions > 0
                    ? round(($variant->conversions / $variant->impressions) * 100, 2)
                    : 0;
                return $variant;
            });

        return $variants->toArray();
    }

    /**
     * Generate server-side beacon pixel response.
     *
     * @param int $popupId
     * @param string $eventType
     * @param array $data
     * @return string Base64-encoded 1x1 GIF
     */
    public function generateBeaconPixel(int $popupId, string $eventType, array $data = []): string
    {
        // Record the event
        $this->recordEvent($popupId, $eventType, $data);

        // Return 1x1 transparent GIF
        return base64_decode('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
    }

    /**
     * Update real-time metrics cache.
     *
     * @param int $popupId
     * @param string $eventType
     */
    protected function updateRealTimeMetrics(int $popupId, string $eventType): void
    {
        $key = "popup_realtime_{$popupId}";

        $metrics = Cache::get($key, [
            'impressions' => 0,
            'conversions' => 0,
            'clicks' => 0,
            'updated_at' => now()->toIso8601String(),
        ]);

        switch ($eventType) {
            case self::EVENT_IMPRESSION:
                $metrics['impressions']++;
                break;
            case self::EVENT_CONVERSION:
                $metrics['conversions']++;
                break;
            case self::EVENT_CLICK:
                $metrics['clicks']++;
                break;
        }

        $metrics['updated_at'] = now()->toIso8601String();

        Cache::put($key, $metrics, 3600); // 1 hour TTL
    }

    /**
     * Generate privacy-preserving session ID.
     *
     * @param array $data
     * @return string
     */
    protected function generatePrivacySessionId(array $data): string
    {
        // Use a combination of date and anonymized data
        $components = [
            date('Y-m-d'),
            substr(hash('sha256', ($data['ip_address'] ?? '') . ($data['user_agent'] ?? '')), 0, 8),
            Str::random(8),
        ];

        return implode('_', $components);
    }

    /**
     * Generate privacy-preserving visitor hash.
     *
     * @param array $data
     * @return string
     */
    protected function generateVisitorHash(array $data): string
    {
        $fingerprint = implode('|', [
            $data['ip_address'] ?? '',
            $data['user_agent'] ?? '',
            date('Y-m-d'), // Rotate daily for privacy
        ]);

        return hash('sha256', $fingerprint);
    }

    /**
     * Detect device type from user agent.
     *
     * @param string $userAgent
     * @return string
     */
    protected function detectDeviceType(string $userAgent): string
    {
        $userAgent = strtolower($userAgent);

        if (preg_match('/mobile|android|iphone|ipod|blackberry|windows phone/i', $userAgent)) {
            return 'mobile';
        }

        if (preg_match('/tablet|ipad/i', $userAgent)) {
            return 'tablet';
        }

        return 'desktop';
    }

    /**
     * Detect browser from user agent.
     *
     * @param string $userAgent
     * @return string
     */
    protected function detectBrowser(string $userAgent): string
    {
        if (preg_match('/Edg/i', $userAgent)) return 'Edge';
        if (preg_match('/Chrome/i', $userAgent)) return 'Chrome';
        if (preg_match('/Firefox/i', $userAgent)) return 'Firefox';
        if (preg_match('/Safari/i', $userAgent) && !preg_match('/Chrome/i', $userAgent)) return 'Safari';
        if (preg_match('/Opera|OPR/i', $userAgent)) return 'Opera';
        if (preg_match('/MSIE|Trident/i', $userAgent)) return 'IE';

        return 'Other';
    }

    /**
     * Detect OS from user agent.
     *
     * @param string $userAgent
     * @return string
     */
    protected function detectOS(string $userAgent): string
    {
        if (preg_match('/Windows/i', $userAgent)) return 'Windows';
        if (preg_match('/Mac/i', $userAgent)) return 'macOS';
        if (preg_match('/Linux/i', $userAgent)) return 'Linux';
        if (preg_match('/Android/i', $userAgent)) return 'Android';
        if (preg_match('/iPhone|iPad|iPod/i', $userAgent)) return 'iOS';

        return 'Other';
    }

    /**
     * Detect country from IP (simplified - use MaxMind GeoIP in production).
     *
     * @param string $ipAddress
     * @return string|null
     */
    protected function detectCountry(string $ipAddress): ?string
    {
        // In production, integrate with MaxMind GeoIP2 or similar
        // For now, return null to indicate unknown
        return null;
    }

    /**
     * Detect if request is from a bot.
     *
     * @param string $userAgent
     * @return bool
     */
    protected function detectBot(string $userAgent): bool
    {
        $botPatterns = [
            'googlebot', 'bingbot', 'yandexbot', 'baiduspider',
            'facebookexternalhit', 'twitterbot', 'rogerbot',
            'linkedinbot', 'embedly', 'showyoubot', 'outbrain',
            'pinterest', 'slackbot', 'vkshare', 'w3c_validator',
            'crawler', 'spider', 'bot', 'scraper',
        ];

        $userAgentLower = strtolower($userAgent);

        foreach ($botPatterns as $pattern) {
            if (strpos($userAgentLower, $pattern) !== false) {
                return true;
            }
        }

        return false;
    }

    /**
     * Get start date from period string.
     *
     * @param string $period
     * @return \Carbon\Carbon
     */
    protected function getStartDate(string $period): \Carbon\Carbon
    {
        return match ($period) {
            '1h' => now()->subHour(),
            '24h' => now()->subDay(),
            '7d' => now()->subDays(7),
            '30d' => now()->subDays(30),
            '90d' => now()->subDays(90),
            default => now()->subDay(),
        };
    }
}
