<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Popup;
use App\Services\Fluent\FluentEcosystemService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Gate;

class PopupController extends Controller
{
    public function __construct(
        private readonly FluentEcosystemService $fluentEcosystem,
    ) {}
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
     * Uses dedicated columns only (not config JSON) to prevent dual storage inconsistency.
     */
    public function impression(Request $request, Popup $popup): JsonResponse
    {
        $payload = $request->validate([
            'sessionId' => ['nullable', 'string', 'max:191'],
            'timestamp' => ['nullable', 'date'],
        ]);

        // Use dedicated columns only - avoid dual storage in config JSON
        $popup->increment('impressions');
        $popup->update(['last_impression_at' => now()]);

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
     * Uses dedicated columns only (not config JSON) to prevent dual storage inconsistency.
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

        // Use dedicated columns only - avoid dual storage in config JSON
        $popup->increment('conversions');
        $popup->update(['last_conversion_at' => now()]);

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
     * Process popup form submission with FluentCRM opt-in integration.
     *
     * This endpoint handles the full opt-in flow:
     * 1. Validates form data
     * 2. Records form submission in popup analytics
     * 3. Creates/updates contact in FluentCRM
     * 4. Applies tags and segments
     * 5. Triggers automations
     */
    public function formSubmit(Request $request, Popup $popup): JsonResponse
    {
        $payload = $request->validate([
            'formData' => ['required', 'array'],
            'formData.email' => ['required', 'email', 'max:255'],
            'formData.name' => ['nullable', 'string', 'max:255'],
            'formData.first_name' => ['nullable', 'string', 'max:100'],
            'formData.last_name' => ['nullable', 'string', 'max:100'],
            'formData.phone' => ['nullable', 'string', 'max:50'],
            'formData.company' => ['nullable', 'string', 'max:255'],
            'formData.consent' => ['nullable', 'boolean'],
            'sessionId' => ['nullable', 'string', 'max:191'],
            'metadata' => ['nullable', 'array'],
        ]);

        $formData = $payload['formData'];
        $metadata = array_merge($payload['metadata'] ?? [], [
            'ip' => $request->ip(),
            'user_agent' => $request->userAgent(),
            'session_id' => $payload['sessionId'] ?? null,
            'page_url' => $request->header('Referer'),
        ]);

        // Record form submission in popup analytics
        $popup->increment('form_submissions');
        $popup->increment('conversions');
        $popup->update(['last_conversion_at' => now()]);

        // Process opt-in through FluentEcosystem
        try {
            $result = $this->fluentEcosystem->processPopupOptIn($popup, $formData, $metadata);

            Log::info('popup_form_submission', [
                'popup_id' => $popup->id,
                'popup_name' => $popup->name,
                'email' => $formData['email'],
                'contact_id' => $result['contact_id'],
                'contact_created' => $result['contact_created'],
                'tags_applied' => $result['tags_applied'],
                'automations_triggered' => $result['automations_triggered'],
                'session_id' => $payload['sessionId'] ?? null,
            ]);

            return response()->json([
                'status' => 'ok',
                'message' => $result['double_optin_sent']
                    ? 'Please check your email to confirm your subscription'
                    : 'Successfully subscribed',
                'contact_id' => $result['contact_id'],
                'double_optin_required' => $result['double_optin_sent'],
            ]);

        } catch (\Exception $e) {
            Log::error('popup_form_submission_failed', [
                'popup_id' => $popup->id,
                'email' => $formData['email'],
                'error' => $e->getMessage(),
            ]);

            // Still return success to user (subscription recorded even if CRM sync failed)
            return response()->json([
                'status' => 'ok',
                'message' => 'Successfully subscribed',
            ]);
        }
    }

    /**
     * Get popup opt-in configuration for admin.
     */
    public function getOptInConfig(Request $request, Popup $popup): JsonResponse
    {
        // Defense in depth: verify admin access
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $config = $this->fluentEcosystem->getPopupOptInConfig($popup);

        return response()->json($config);
    }

    /**
     * Update popup opt-in configuration.
     */
    public function updateOptInConfig(Request $request, Popup $popup): JsonResponse
    {
        // Defense in depth: verify admin access
        if (!$request->user() || !$request->user()->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Unauthorized access to popup management');
        }

        $config = $request->validate([
            'enabled' => ['nullable', 'boolean'],
            'double_optin' => ['nullable', 'boolean'],
            'lists' => ['nullable', 'array'],
            'segments' => ['nullable', 'array'],
            'tags' => ['nullable', 'array'],
            'automations' => ['nullable', 'array'],
            'consent_text' => ['nullable', 'string', 'max:500'],
            'privacy_policy_url' => ['nullable', 'string', 'max:255'],
            'gdpr_enabled' => ['nullable', 'boolean'],
        ]);

        $popup = $this->fluentEcosystem->updatePopupOptInConfig($popup, $config);

        return response()->json([
            'status' => 'ok',
            'popup' => $this->toFrontendPopup($popup),
        ]);
    }

    /**
     * Return basic analytics for a popup matching PopupAnalytics interface.
     * Uses dedicated columns as source of truth - config JSON is for extended metadata only.
     */
    public function analytics(Popup $popup): JsonResponse
    {
        $config = $popup->config ?? [];

        // Use dedicated columns as source of truth for core metrics
        $impressions = (int) ($popup->impressions ?? 0);
        $views = (int) ($popup->views ?? $impressions);
        $conversions = (int) ($popup->conversions ?? 0);
        $closes = (int) ($popup->closes ?? 0);
        $formSubmissions = (int) ($popup->form_submissions ?? 0);

        $conversionRate = $views > 0
            ? round(($conversions / $views) * 100, 2)
            : 0.0;

        $closeRate = $views > 0
            ? round(($closes / $views) * 100, 2)
            : 0.0;

        $analytics = [
            'impressions' => $impressions,
            'uniqueImpressions' => $views,
            'conversions' => $conversions,
            'conversionRate' => $conversionRate,
            'closes' => $closes,
            'closeRate' => $closeRate,
            'formSubmissions' => $formSubmissions,
            'avgTimeToConversion' => (float) ($popup->avg_time_to_conversion ?? 0),
            'revenue' => (float) Arr::get($config, 'analytics.revenue', 0),
            'engagement' => Arr::get($config, 'analytics.engagement', [
                'clicks' => 0,
                'hovers' => 0,
                'scrolls' => 0,
                'videoPlays' => 0,
                'formStarts' => 0,
                'formCompletions' => $formSubmissions,
                'shares' => 0,
            ]),
            'devices' => Arr::get($config, 'analytics.devices', [
                'desktop' => 0,
                'mobile' => 0,
                'tablet' => 0,
            ]),
            'sources' => Arr::get($config, 'analytics.sources', []),
            'lastImpressionAt' => $popup->last_impression_at?->toIso8601String(),
            'lastConversionAt' => $popup->last_conversion_at?->toIso8601String(),
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
