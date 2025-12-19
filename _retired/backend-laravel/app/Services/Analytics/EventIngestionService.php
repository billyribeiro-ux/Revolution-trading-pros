<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsSession;
use App\Models\Analytics\AnalyticsUserProfile;
use App\Models\Analytics\AnalyticsFunnelConversion;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Illuminate\Http\Request;

/**
 * EventIngestionService - Enterprise Event Ingestion Pipeline
 *
 * Core service for ingesting and processing analytics events from all sources.
 * Handles event enrichment, session management, and real-time processing.
 *
 * @package App\Services\Analytics
 */
class EventIngestionService
{
    /**
     * Session cache prefix
     */
    private const SESSION_CACHE_PREFIX = 'analytics:session:';

    /**
     * Session timeout in minutes
     */
    private const SESSION_TIMEOUT = 30;

    /**
     * Batch size for bulk operations
     */
    private const BATCH_SIZE = 100;

    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
        private readonly AttributionEngine $attributionEngine,
    ) {}

    /**
     * Ingest a single event
     */
    public function ingest(array $eventData, ?Request $request = null): AnalyticsEvent
    {
        // Enrich event with request context
        if ($request) {
            $eventData = $this->enrichFromRequest($eventData, $request);
        }

        // Generate or validate session
        $eventData = $this->ensureSession($eventData);

        // Detect and set channel if not provided
        if (empty($eventData['channel'])) {
            $eventData['channel'] = $this->detectChannel($eventData);
        }

        // Parse user agent
        if (isset($eventData['user_agent']) && empty($eventData['device_type'])) {
            $eventData = array_merge($eventData, $this->parseUserAgent($eventData['user_agent']));
        }

        // Create the event
        $event = AnalyticsEvent::create($eventData);

        // Process event asynchronously or inline
        $this->processEvent($event);

        // Update real-time cache
        $this->cacheManager->incrementRealTimeCounter($event->event_name);

        return $event;
    }

    /**
     * Ingest multiple events in batch
     */
    public function ingestBatch(array $events, ?Request $request = null): int
    {
        $processed = 0;

        DB::transaction(function () use ($events, $request, &$processed) {
            $chunks = array_chunk($events, self::BATCH_SIZE);

            foreach ($chunks as $chunk) {
                foreach ($chunk as $eventData) {
                    try {
                        $this->ingest($eventData, $request);
                        $processed++;
                    } catch (\Exception $e) {
                        Log::error('Event ingestion failed', [
                            'event' => $eventData,
                            'error' => $e->getMessage(),
                        ]);
                    }
                }
            }
        });

        return $processed;
    }

    /**
     * Track a page view event
     */
    public function trackPageView(array $data, ?Request $request = null): AnalyticsEvent
    {
        return $this->ingest(array_merge($data, [
            'event_name' => 'page_view',
            'event_category' => AnalyticsEvent::CATEGORY_PAGE_VIEW,
            'event_type' => AnalyticsEvent::TYPE_VIEW,
        ]), $request);
    }

    /**
     * Track a click event
     */
    public function trackClick(string $element, array $data = [], ?Request $request = null): AnalyticsEvent
    {
        return $this->ingest(array_merge($data, [
            'event_name' => 'click_' . Str::snake($element),
            'event_category' => AnalyticsEvent::CATEGORY_USER_ACTION,
            'event_type' => AnalyticsEvent::TYPE_CLICK,
            'properties' => array_merge($data['properties'] ?? [], ['element' => $element]),
        ]), $request);
    }

    /**
     * Track a conversion event
     */
    public function trackConversion(string $type, array $data = [], ?Request $request = null): AnalyticsEvent
    {
        return $this->ingest(array_merge($data, [
            'event_name' => 'conversion_' . Str::snake($type),
            'event_category' => AnalyticsEvent::CATEGORY_CONVERSION,
            'event_type' => $type,
            'is_conversion' => true,
        ]), $request);
    }

    /**
     * Track a purchase event
     */
    public function trackPurchase(
        float $revenue,
        array $data = [],
        ?Request $request = null
    ): AnalyticsEvent {
        $event = $this->ingest(array_merge($data, [
            'event_name' => 'purchase',
            'event_category' => AnalyticsEvent::CATEGORY_TRANSACTION,
            'event_type' => AnalyticsEvent::TYPE_PURCHASE,
            'revenue' => $revenue,
            'event_value' => $revenue,
            'is_conversion' => true,
        ]), $request);

        // Track attribution conversion
        $this->attributionEngine->recordConversion($event);

        return $event;
    }

    /**
     * Track a signup event
     */
    public function trackSignup(array $data = [], ?Request $request = null): AnalyticsEvent
    {
        return $this->ingest(array_merge($data, [
            'event_name' => 'signup',
            'event_category' => AnalyticsEvent::CATEGORY_CONVERSION,
            'event_type' => AnalyticsEvent::TYPE_SIGNUP,
            'is_conversion' => true,
            'journey_stage' => AnalyticsEvent::STAGE_DECISION,
        ]), $request);
    }

    /**
     * Track a custom event
     */
    public function track(string $eventName, array $data = [], ?Request $request = null): AnalyticsEvent
    {
        return $this->ingest(array_merge($data, [
            'event_name' => $eventName,
            'event_category' => $data['event_category'] ?? AnalyticsEvent::CATEGORY_USER_ACTION,
            'event_type' => $data['event_type'] ?? AnalyticsEvent::TYPE_VIEW,
        ]), $request);
    }

    /**
     * Track entity-specific events (posts, videos, products, etc.)
     */
    public function trackEntity(
        string $eventName,
        string $entityType,
        int $entityId,
        array $data = [],
        ?Request $request = null
    ): AnalyticsEvent {
        return $this->ingest(array_merge($data, [
            'event_name' => $eventName,
            'entity_type' => $entityType,
            'entity_id' => $entityId,
        ]), $request);
    }

    /**
     * Enrich event data from HTTP request
     */
    protected function enrichFromRequest(array $eventData, Request $request): array
    {
        return array_merge([
            'ip_address' => $request->ip(),
            'user_agent' => $request->userAgent(),
            'referrer' => $request->header('referer'),
            'page_url' => $request->fullUrl(),
            'page_path' => $request->path(),
            'user_id' => $request->user()?->id,
            'utm_source' => $request->input('utm_source') ?? $request->cookie('utm_source'),
            'utm_medium' => $request->input('utm_medium') ?? $request->cookie('utm_medium'),
            'utm_campaign' => $request->input('utm_campaign') ?? $request->cookie('utm_campaign'),
            'utm_term' => $request->input('utm_term') ?? $request->cookie('utm_term'),
            'utm_content' => $request->input('utm_content') ?? $request->cookie('utm_content'),
        ], $eventData);
    }

    /**
     * Ensure session exists and is active
     */
    protected function ensureSession(array $eventData): array
    {
        $sessionId = $eventData['session_id'] ?? null;
        $userId = $eventData['user_id'] ?? null;
        $anonymousId = $eventData['anonymous_id'] ?? $this->generateAnonymousId($eventData);

        // Check for existing session in cache
        $cacheKey = self::SESSION_CACHE_PREFIX . ($sessionId ?? $anonymousId);
        $cachedSession = Cache::get($cacheKey);

        if ($cachedSession && !$this->isSessionExpired($cachedSession)) {
            $sessionId = $cachedSession['session_id'];
        } else {
            // Create new session
            $sessionId = Str::uuid()->toString();

            $session = AnalyticsSession::create([
                'session_id' => $sessionId,
                'user_id' => $userId,
                'anonymous_id' => $anonymousId,
                'started_at' => now(),
                'landing_page' => $eventData['page_url'] ?? null,
                'entry_source' => $eventData['referrer'] ?? null,
                'utm_source' => $eventData['utm_source'] ?? null,
                'utm_medium' => $eventData['utm_medium'] ?? null,
                'utm_campaign' => $eventData['utm_campaign'] ?? null,
                'channel' => $eventData['channel'] ?? null,
                'referrer' => $eventData['referrer'] ?? null,
                'device_type' => $eventData['device_type'] ?? null,
                'browser' => $eventData['browser'] ?? null,
                'os' => $eventData['os'] ?? null,
                'country_code' => $eventData['country_code'] ?? null,
            ]);

            Cache::put($cacheKey, [
                'session_id' => $sessionId,
                'last_activity' => now()->timestamp,
            ], now()->addMinutes(self::SESSION_TIMEOUT));
        }

        // Update cache with last activity
        Cache::put($cacheKey, [
            'session_id' => $sessionId,
            'last_activity' => now()->timestamp,
        ], now()->addMinutes(self::SESSION_TIMEOUT));

        $eventData['session_id'] = $sessionId;
        $eventData['anonymous_id'] = $anonymousId;

        return $eventData;
    }

    /**
     * Check if cached session is expired
     */
    protected function isSessionExpired(array $cachedSession): bool
    {
        $lastActivity = $cachedSession['last_activity'] ?? 0;
        $expiryTime = now()->subMinutes(self::SESSION_TIMEOUT)->timestamp;

        return $lastActivity < $expiryTime;
    }

    /**
     * Generate anonymous ID from request fingerprint
     */
    protected function generateAnonymousId(array $eventData): string
    {
        $fingerprint = implode('|', [
            $eventData['ip_address'] ?? '',
            $eventData['user_agent'] ?? '',
            date('Y-m-d'),
        ]);

        return hash('sha256', $fingerprint);
    }

    /**
     * Detect marketing channel from event data
     */
    protected function detectChannel(array $eventData): string
    {
        $utmSource = strtolower($eventData['utm_source'] ?? '');
        $utmMedium = strtolower($eventData['utm_medium'] ?? '');
        $referrer = $eventData['referrer'] ?? '';

        // Check UTM source
        if ($utmSource) {
            if (in_array($utmSource, ['google', 'bing', 'yahoo', 'duckduckgo'])) {
                return $utmMedium === 'cpc' ? AnalyticsEvent::CHANNEL_PAID : AnalyticsEvent::CHANNEL_ORGANIC;
            }

            if (in_array($utmSource, ['facebook', 'instagram', 'twitter', 'linkedin', 'tiktok'])) {
                return AnalyticsEvent::CHANNEL_SOCIAL;
            }

            if (str_contains($utmSource, 'email') || str_contains($utmSource, 'newsletter')) {
                return AnalyticsEvent::CHANNEL_EMAIL;
            }
        }

        // Check UTM medium
        if (in_array($utmMedium, ['cpc', 'ppc', 'paid', 'display'])) {
            return AnalyticsEvent::CHANNEL_PAID;
        }

        if (in_array($utmMedium, ['email', 'newsletter'])) {
            return AnalyticsEvent::CHANNEL_EMAIL;
        }

        // Check referrer
        if ($referrer) {
            $referrerDomain = parse_url($referrer, PHP_URL_HOST);

            if ($referrerDomain) {
                // Search engines
                $searchEngines = ['google.', 'bing.', 'yahoo.', 'duckduckgo.', 'baidu.'];
                foreach ($searchEngines as $engine) {
                    if (str_contains($referrerDomain, $engine)) {
                        return AnalyticsEvent::CHANNEL_ORGANIC;
                    }
                }

                // Social networks
                $socialNetworks = ['facebook.', 'instagram.', 'twitter.', 't.co', 'linkedin.', 'youtube.'];
                foreach ($socialNetworks as $network) {
                    if (str_contains($referrerDomain, $network)) {
                        return AnalyticsEvent::CHANNEL_SOCIAL;
                    }
                }

                return AnalyticsEvent::CHANNEL_REFERRAL;
            }
        }

        return AnalyticsEvent::CHANNEL_DIRECT;
    }

    /**
     * Parse user agent string
     */
    protected function parseUserAgent(string $userAgent): array
    {
        $result = [
            'device_type' => 'desktop',
            'browser' => 'unknown',
            'os' => 'unknown',
        ];

        // Device type detection
        $mobileKeywords = ['mobile', 'android', 'iphone', 'ipod', 'blackberry', 'windows phone'];
        $tabletKeywords = ['tablet', 'ipad', 'kindle', 'playbook'];

        $userAgentLower = strtolower($userAgent);

        foreach ($tabletKeywords as $keyword) {
            if (str_contains($userAgentLower, $keyword)) {
                $result['device_type'] = 'tablet';
                break;
            }
        }

        if ($result['device_type'] === 'desktop') {
            foreach ($mobileKeywords as $keyword) {
                if (str_contains($userAgentLower, $keyword)) {
                    $result['device_type'] = 'mobile';
                    break;
                }
            }
        }

        // Browser detection
        $browsers = [
            'Chrome' => 'chrome',
            'Firefox' => 'firefox',
            'Safari' => 'safari',
            'Edge' => 'edge',
            'Opera' => 'opera',
            'MSIE' => 'ie',
            'Trident' => 'ie',
        ];

        foreach ($browsers as $pattern => $browser) {
            if (str_contains($userAgent, $pattern)) {
                $result['browser'] = $browser;
                break;
            }
        }

        // OS detection
        $operatingSystems = [
            'Windows' => 'windows',
            'Mac OS' => 'macos',
            'Linux' => 'linux',
            'Android' => 'android',
            'iOS' => 'ios',
            'iPhone' => 'ios',
            'iPad' => 'ios',
        ];

        foreach ($operatingSystems as $pattern => $os) {
            if (str_contains($userAgent, $pattern)) {
                $result['os'] = $os;
                break;
            }
        }

        return $result;
    }

    /**
     * Process event after creation
     */
    protected function processEvent(AnalyticsEvent $event): void
    {
        // Update session
        $session = AnalyticsSession::where('session_id', $event->session_id)->first();
        if ($session) {
            $session->recordEvent($event);
        }

        // Update user profile
        if ($event->user_id) {
            $profile = AnalyticsUserProfile::findOrCreateForUser($event->user_id);
            $profile->recordEvent($event);
        }

        // Track funnel progress
        if ($event->funnel_id) {
            $this->updateFunnelProgress($event);
        }

        // Record attribution touchpoint
        $this->attributionEngine->recordTouchpoint($event);
    }

    /**
     * Update funnel progress for user
     */
    protected function updateFunnelProgress(AnalyticsEvent $event): void
    {
        $conversion = AnalyticsFunnelConversion::where('funnel_id', $event->funnel_id)
            ->where(function ($query) use ($event) {
                $query->where('user_id', $event->user_id)
                    ->orWhere('anonymous_id', $event->anonymous_id);
            })
            ->where('is_converted', false)
            ->first();

        if ($conversion && $event->funnel_step !== null) {
            $conversion->advanceToStep($event->funnel_step);
        }
    }

    /**
     * End expired sessions
     */
    public function endExpiredSessions(): int
    {
        $expiredSessions = AnalyticsSession::active()
            ->where('updated_at', '<', now()->subMinutes(self::SESSION_TIMEOUT))
            ->get();

        foreach ($expiredSessions as $session) {
            $session->endSession();
        }

        return $expiredSessions->count();
    }
}
