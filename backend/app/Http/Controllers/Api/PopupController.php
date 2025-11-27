<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Popup;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Gate;

class PopupController extends Controller
{
    /**
     * List all popups for the admin UI.
     * Requires admin authentication (defense in depth - also enforced at route level).
     */
    public function index(Request $request): JsonResponse
    {
        // Defense in depth: verify admin access even though route middleware should handle this
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $popups = Popup::query()
            ->orderByDesc('id')
            ->get()
            ->map(fn (Popup $popup) => $this->toFrontendPopup($popup));

        return response()->json([
            'popups' => $popups,
        ]);
    }

    /**
     * Create a new popup from the raw frontend config.
     * Requires admin authentication.
     */
    public function store(Request $request): JsonResponse
    {
        // Defense in depth: verify admin access
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $data = $request->all();

        $popup = Popup::create([
            'name' => (string) Arr::get($data, 'name', 'Untitled Popup'),
            'status' => Arr::get($data, 'status', 'draft'),
            'is_active' => Arr::get($data, 'isActive', true),
            'type' => Arr::get($data, 'type', 'modal'),
            'ab_test_id' => Arr::get($data, 'abTestId'),
            'variant_title' => Arr::get($data, 'variantTitle'),
            'priority' => (int) Arr::get($data, 'priority', 0),
            'attention_animation' => Arr::get($data, 'attentionAnimation'),
            'countdown_timer' => Arr::get($data, 'countdownTimer'),
            'video_embed' => Arr::get($data, 'videoEmbed'),
            'display_rules' => Arr::get($data, 'displayRules'),
            'form_fields' => Arr::get($data, 'formFields'),
            'design' => Arr::get($data, 'design'),
            'config' => $data,
        ]);

        return response()->json($this->toFrontendPopup($popup), 201);
    }

    /**
     * Update an existing popup.
     * Requires admin authentication.
     */
    public function update(Request $request, Popup $popup): JsonResponse
    {
        // Defense in depth: verify admin access
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $data = $request->all();

        $popup->update([
            'name' => (string) Arr::get($data, 'name', $popup->name),
            'status' => Arr::get($data, 'status', $popup->status),
            'is_active' => Arr::get($data, 'isActive', $popup->is_active),
            'type' => Arr::get($data, 'type', $popup->type),
            'ab_test_id' => Arr::get($data, 'abTestId'),
            'variant_title' => Arr::get($data, 'variantTitle'),
            'priority' => (int) Arr::get($data, 'priority', $popup->priority),
            'attention_animation' => Arr::get($data, 'attentionAnimation'),
            'countdown_timer' => Arr::get($data, 'countdownTimer'),
            'video_embed' => Arr::get($data, 'videoEmbed'),
            'display_rules' => Arr::get($data, 'displayRules'),
            'form_fields' => Arr::get($data, 'formFields'),
            'design' => Arr::get($data, 'design'),
            'config' => $data,
        ]);

        return response()->json($this->toFrontendPopup($popup));
    }

    /**
     * Delete a popup.
     * Requires admin authentication.
     */
    public function destroy(Request $request, Popup $popup): JsonResponse
    {
        // Defense in depth: verify admin access
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $popup->delete();

        return response()->json([
            'status' => 'ok',
        ]);
    }

    /**
     * Return active popups for the current page.
     *
     * For now we simply return all popups; the frontend service applies
     * advanced targeting and scheduling logic.
     */
    public function active(Request $request): JsonResponse
    {
        $popups = Popup::query()
            ->orderByDesc('id')
            ->get()
            ->filter(function (Popup $popup) {
                $config = $popup->config ?? [];
                $status = Arr::get($config, 'status');
                $isActive = Arr::get($config, 'isActive', true);

                // Treat "published" status + isActive=true as active
                return $isActive && ($status === null || $status === 'published');
            })
            ->values()
            ->map(fn (Popup $popup) => $this->toFrontendPopup($popup));

        return response()->json([
            'popups' => $popups,
        ]);
    }

    /**
     * Record a popup impression.
     */
    public function impression(Request $request, Popup $popup): JsonResponse
    {
        $payload = $request->validate([
            'sessionId' => ['nullable', 'string', 'max:191'],
            'timestamp' => ['nullable', 'date'],
        ]);

        $popup->impressions++;
        $popup->last_impression_at = now();

        $config = $popup->config ?? [];
        $config['impressions'] = ($config['impressions'] ?? 0) + 1;
        $popup->config = $config;

        $popup->save();

        Log::info('popup_impression', [
            'popup_id' => $popup->id,
            'session_id' => $payload['sessionId'] ?? null,
            'timestamp' => $payload['timestamp'] ?? now()->toIso8601String(),
            'ip' => $request->ip(),
            'user_id' => optional($request->user())->id,
        ]);

        return response()->json(['status' => 'ok']);
    }

    /**
     * Record a popup conversion.
     */
    public function conversion(Request $request, Popup $popup): JsonResponse
    {
        $payload = $request->validate([
            'sessionId' => ['nullable', 'string', 'max:191'],
            'conversionTime' => ['nullable', 'integer'],
            'action' => ['nullable', 'string', 'max:191'],
            'value' => ['nullable'],
            'revenue' => ['nullable', 'numeric'],
            'metadata' => ['nullable', 'array'],
        ]);

        $popup->conversions++;
        $popup->last_conversion_at = now();

        $config = $popup->config ?? [];
        $config['conversions'] = ($config['conversions'] ?? 0) + 1;
        $popup->config = $config;

        $popup->save();

        Log::info('popup_conversion', [
            'popup_id' => $popup->id,
            'session_id' => $payload['sessionId'] ?? null,
            'conversion_time' => $payload['conversionTime'] ?? null,
            'action' => $payload['action'] ?? null,
            'value' => $payload['value'] ?? null,
            'revenue' => $payload['revenue'] ?? null,
            'metadata' => $payload['metadata'] ?? null,
            'ip' => $request->ip(),
            'user_id' => optional($request->user())->id,
        ]);

        return response()->json(['status' => 'ok']);
    }

    /**
     * Return basic analytics for a popup matching PopupAnalytics interface.
     */
    public function analytics(Popup $popup): JsonResponse
    {
        $config = $popup->config ?? [];

        $impressions = (int) ($config['impressions'] ?? $popup->impressions ?? 0);
        $conversions = (int) ($config['conversions'] ?? $popup->conversions ?? 0);
        $uniqueImpressions = (int) ($config['uniqueImpressions'] ?? $impressions);

        $conversionRate = $impressions > 0
            ? round(($conversions / $impressions) * 100, 2)
            : 0.0;

        $analytics = [
            'impressions' => $impressions,
            'uniqueImpressions' => $uniqueImpressions,
            'conversions' => $conversions,
            'conversionRate' => $conversionRate,
            'revenue' => (float) Arr::get($config, 'analytics.revenue', 0),
            'engagement' => Arr::get($config, 'analytics.engagement', [
                'clicks' => 0,
                'hovers' => 0,
                'scrolls' => 0,
                'videoPlays' => 0,
                'formStarts' => 0,
                'formCompletions' => 0,
                'shares' => 0,
            ]),
            'devices' => Arr::get($config, 'analytics.devices', [
                'desktop' => 0,
                'mobile' => 0,
                'tablet' => 0,
            ]),
            'sources' => Arr::get($config, 'analytics.sources', []),
        ];

        return response()->json($analytics);
    }

    /**
     * Batch endpoint for generic popup events (interaction, close, etc.).
     * This is primarily to prevent 404s from PopupEngagementService::flushEventBuffer.
     */
    public function events(Request $request): JsonResponse
    {
        $data = $request->validate([
            'events' => ['required', 'array'],
            'events.*.type' => ['required', 'string', 'max:50'],
            'events.*.popupId' => ['required', 'string', 'max:191'],
            'events.*.timestamp' => ['required', 'date'],
            'events.*.sessionId' => ['nullable', 'string', 'max:191'],
            'events.*.userId' => ['nullable'],
            'events.*.data' => ['nullable', 'array'],
        ]);

        foreach ($data['events'] as $event) {
            Log::info('popup_event', $event + [
                'ip' => $request->ip(),
                'user_id' => optional($request->user())->id,
            ]);
        }

        return response()->json(['status' => 'ok']);
    }

    /**
     * Transform a Popup model into the structure expected by the frontend
     * Popup / EnhancedPopup types (config merged with metrics and id).
     */
    private function toFrontendPopup(Popup $popup): array
    {
        $config = $popup->config ?? [];

        // Merge dedicated columns with config, giving priority to dedicated columns
        $result = array_merge($config, array_filter([
            'id' => (string) $popup->id,
            'name' => $popup->name,
            'status' => $popup->status,
            'isActive' => $popup->is_active,
            'type' => $popup->type,
            'abTestId' => $popup->ab_test_id,
            'variantTitle' => $popup->variant_title,
            'priority' => $popup->priority,
            'attentionAnimation' => $popup->attention_animation,
            'countdownTimer' => $popup->countdown_timer,
            'videoEmbed' => $popup->video_embed,
            'displayRules' => $popup->display_rules,
            'formFields' => $popup->form_fields,
            'design' => $popup->design,
            'impressions' => $popup->impressions ?? 0,
            'conversions' => $popup->conversions ?? 0,
        ], fn($value) => $value !== null));

        return $result;
    }
}
