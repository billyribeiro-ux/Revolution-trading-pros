<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\IntegrationCredential;
use App\Services\Integration\GoogleSearchConsoleService;
use App\Services\Integration\GoogleAnalyticsService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\Log;

/**
 * Integration Controller
 *
 * Handles OAuth flows and data retrieval for external integrations
 * (Google Search Console, Google Analytics, etc.)
 *
 * @version 1.0.0
 * @level L11 Principal Engineer
 */
class IntegrationController extends Controller
{
    public function __construct(
        private GoogleSearchConsoleService $gscService,
        private GoogleAnalyticsService $gaService
    ) {}

    /**
     * Get status of all integrations for the current user
     */
    public function status(): JsonResponse
    {
        $userId = Auth::id();

        $credentials = IntegrationCredential::where('user_id', $userId)
            ->where('is_active', true)
            ->get()
            ->keyBy('provider');

        return response()->json([
            'google_search_console' => [
                'connected' => $credentials->has('google_search_console'),
                'last_sync' => $credentials->get('google_search_console')?->updated_at,
                'metadata' => $credentials->get('google_search_console')?->metadata,
            ],
            'google_analytics' => [
                'connected' => $credentials->has('google_analytics'),
                'last_sync' => $credentials->get('google_analytics')?->updated_at,
                'metadata' => $credentials->get('google_analytics')?->metadata,
            ],
            'google_tag_manager' => [
                'connected' => $credentials->has('google_tag_manager'),
                'last_sync' => $credentials->get('google_tag_manager')?->updated_at,
            ],
            'google_ads' => [
                'connected' => $credentials->has('google_ads'),
                'last_sync' => $credentials->get('google_ads')?->updated_at,
            ],
        ]);
    }

    /**
     * Get OAuth URL for Google Search Console
     */
    public function gscAuthUrl(Request $request): JsonResponse
    {
        $redirectUri = config('app.url') . '/api/integrations/google_search_console/callback';
        $state = base64_encode(json_encode([
            'user_id' => Auth::id(),
            'provider' => 'google_search_console',
        ]));

        $authUrl = $this->gscService->getAuthUrl($redirectUri, $state);

        return response()->json(['auth_url' => $authUrl]);
    }

    /**
     * Handle OAuth callback for Google Search Console
     */
    public function gscCallback(Request $request): JsonResponse
    {
        $code = $request->get('code');
        $state = json_decode(base64_decode($request->get('state', '')), true);

        if (!$code || !$state || !isset($state['user_id'])) {
            return response()->json(['error' => 'Invalid callback'], 400);
        }

        $redirectUri = config('app.url') . '/api/integrations/google_search_console/callback';
        $success = $this->gscService->handleCallback($state['user_id'], $code, $redirectUri);

        if ($success) {
            // Return HTML that sends message to opener and closes
            return response()->json([
                'success' => true,
                'message' => 'Google Search Console connected successfully',
            ]);
        }

        return response()->json(['error' => 'Failed to connect'], 500);
    }

    /**
     * Get OAuth URL for Google Analytics
     */
    public function gaAuthUrl(Request $request): JsonResponse
    {
        $redirectUri = config('app.url') . '/api/integrations/google_analytics/callback';
        $state = base64_encode(json_encode([
            'user_id' => Auth::id(),
            'provider' => 'google_analytics',
        ]));

        $authUrl = $this->gaService->getAuthUrl($redirectUri, $state);

        return response()->json(['auth_url' => $authUrl]);
    }

    /**
     * Handle OAuth callback for Google Analytics
     */
    public function gaCallback(Request $request): JsonResponse
    {
        $code = $request->get('code');
        $state = json_decode(base64_decode($request->get('state', '')), true);

        if (!$code || !$state || !isset($state['user_id'])) {
            return response()->json(['error' => 'Invalid callback'], 400);
        }

        $redirectUri = config('app.url') . '/api/integrations/google_analytics/callback';
        $success = $this->gaService->handleCallback($state['user_id'], $code, $redirectUri);

        if ($success) {
            return response()->json([
                'success' => true,
                'message' => 'Google Analytics connected successfully',
            ]);
        }

        return response()->json(['error' => 'Failed to connect'], 500);
    }

    /**
     * Disconnect an integration
     */
    public function disconnect(Request $request, string $provider): JsonResponse
    {
        $userId = Auth::id();
        $validProviders = ['google_search_console', 'google_analytics', 'google_tag_manager', 'google_ads'];

        if (!in_array($provider, $validProviders)) {
            return response()->json(['error' => 'Invalid provider'], 400);
        }

        $success = match ($provider) {
            'google_search_console' => $this->gscService->disconnect($userId),
            'google_analytics' => $this->gaService->disconnect($userId),
            default => IntegrationCredential::where('user_id', $userId)
                ->where('provider', $provider)
                ->delete() > 0,
        };

        if ($success) {
            return response()->json(['success' => true]);
        }

        return response()->json(['error' => 'Failed to disconnect'], 500);
    }

    /**
     * Get Google Search Console data
     */
    public function gscData(Request $request): JsonResponse
    {
        $userId = Auth::id();
        $siteUrl = $request->get('site_url');

        if (!$siteUrl) {
            // Try to get from metadata
            $credential = IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_search_console')
                ->first();

            $siteUrl = $credential?->getSiteUrl();
        }

        if (!$siteUrl) {
            return response()->json(['error' => 'Site URL required'], 400);
        }

        $type = $request->get('type', 'performance');
        $days = (int) $request->get('days', 28);

        $data = match ($type) {
            'keywords' => $this->gscService->getKeywordRankings($userId, $siteUrl, $days),
            'pages' => $this->gscService->getTopPages($userId, $siteUrl),
            'indexing' => $this->gscService->getIndexingStatus($userId, $siteUrl),
            default => $this->gscService->getPerformanceSummary($userId, $siteUrl, $days),
        };

        return response()->json($data);
    }

    /**
     * Get Google Analytics data
     */
    public function gaData(Request $request): JsonResponse
    {
        $userId = Auth::id();
        $propertyId = $request->get('property_id');

        if (!$propertyId) {
            // Try to get from metadata
            $credential = IntegrationCredential::where('user_id', $userId)
                ->where('provider', 'google_analytics')
                ->first();

            $propertyId = $credential?->getPropertyId();
        }

        if (!$propertyId) {
            return response()->json(['error' => 'Property ID required'], 400);
        }

        $type = $request->get('type', 'visitors');
        $days = (int) $request->get('days', 30);

        $data = match ($type) {
            'traffic_sources' => $this->gaService->getTrafficSources($userId, $propertyId, $days),
            'top_pages' => $this->gaService->getTopPages($userId, $propertyId),
            'devices' => $this->gaService->getDeviceBreakdown($userId, $propertyId),
            'geo' => $this->gaService->getGeographicData($userId, $propertyId),
            'realtime' => $this->gaService->getRealTimeUsers($userId, $propertyId),
            default => $this->gaService->getVisitorAnalytics($userId, $propertyId, $days),
        };

        return response()->json($data);
    }

    /**
     * Get sites from Google Search Console
     */
    public function gscSites(): JsonResponse
    {
        $userId = Auth::id();
        $sites = $this->gscService->getSites($userId);

        return response()->json(['sites' => $sites]);
    }

    /**
     * Set site URL for Google Search Console
     */
    public function gscSetSite(Request $request): JsonResponse
    {
        $request->validate([
            'site_url' => 'required|url',
        ]);

        $userId = Auth::id();
        $siteUrl = $request->get('site_url');

        $credential = IntegrationCredential::where('user_id', $userId)
            ->where('provider', 'google_search_console')
            ->first();

        if (!$credential) {
            return response()->json(['error' => 'Not connected'], 400);
        }

        $credential->update([
            'metadata' => array_merge($credential->metadata ?? [], [
                'site_url' => $siteUrl,
            ]),
        ]);

        return response()->json(['success' => true]);
    }

    /**
     * Set property ID for Google Analytics
     */
    public function gaSetProperty(Request $request): JsonResponse
    {
        $request->validate([
            'property_id' => 'required|string',
        ]);

        $userId = Auth::id();
        $propertyId = $request->get('property_id');

        $success = $this->gaService->setPropertyId($userId, $propertyId);

        if ($success) {
            return response()->json(['success' => true]);
        }

        return response()->json(['error' => 'Failed to set property'], 500);
    }
}
