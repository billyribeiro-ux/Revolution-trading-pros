<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailWebhook;
use App\Services\Email\CampaignWebhookService;
use App\Services\Email\AuditService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * EmailWebhookController
 *
 * Enterprise webhook management for email marketing events.
 *
 * Features:
 * - CRUD operations for webhooks
 * - Event subscription management
 * - Webhook testing
 * - Delivery logs
 * - Retry management
 * - Secret rotation
 *
 * @version 1.0.0
 */
class EmailWebhookController extends Controller
{
    public function __construct(
        private readonly CampaignWebhookService $webhookService,
        private readonly AuditService $auditService
    ) {}

    /**
     * Available webhook events
     */
    private const AVAILABLE_EVENTS = [
        'campaign.created' => 'When a new campaign is created',
        'campaign.scheduled' => 'When a campaign is scheduled',
        'campaign.started' => 'When campaign sending begins',
        'campaign.progress' => 'Campaign sending progress updates',
        'campaign.completed' => 'When campaign sending finishes',
        'campaign.failed' => 'When campaign sending fails',
        'email.sent' => 'When individual email is sent',
        'email.opened' => 'When email is opened',
        'email.clicked' => 'When link in email is clicked',
        'email.bounced' => 'When email bounces',
        'email.complained' => 'When spam complaint is received',
        'subscriber.created' => 'When new subscriber is added',
        'subscriber.verified' => 'When subscriber confirms email',
        'subscriber.unsubscribed' => 'When subscriber unsubscribes',
        'template.created' => 'When template is created',
        'template.updated' => 'When template is modified',
    ];

    /**
     * List all webhooks
     */
    public function index(Request $request): JsonResponse
    {
        $query = EmailWebhook::query()->latest();

        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }

        if ($event = $request->get('event')) {
            $query->whereJsonContains('events', $event);
        }

        $webhooks = $query->paginate($request->integer('per_page', 20));

        return response()->json([
            'success' => true,
            'data' => $webhooks->items(),
            'meta' => [
                'current_page' => $webhooks->currentPage(),
                'last_page' => $webhooks->lastPage(),
                'per_page' => $webhooks->perPage(),
                'total' => $webhooks->total(),
            ],
        ]);
    }

    /**
     * Get webhook statistics
     */
    public function stats(): JsonResponse
    {
        $stats = [
            'total' => EmailWebhook::count(),
            'active' => EmailWebhook::where('status', 'active')->count(),
            'inactive' => EmailWebhook::where('status', 'inactive')->count(),
            'failing' => EmailWebhook::where('consecutive_failures', '>=', 3)->count(),
        ];

        // Delivery stats from last 24 hours
        $deliveries = \DB::table('email_webhook_deliveries')
            ->where('created_at', '>=', now()->subDay())
            ->selectRaw('
                COUNT(*) as total,
                SUM(CASE WHEN status = "success" THEN 1 ELSE 0 END) as successful,
                SUM(CASE WHEN status = "failed" THEN 1 ELSE 0 END) as failed,
                AVG(response_time_ms) as avg_response_time
            ')
            ->first();

        $stats['deliveries_24h'] = [
            'total' => $deliveries->total ?? 0,
            'successful' => $deliveries->successful ?? 0,
            'failed' => $deliveries->failed ?? 0,
            'success_rate' => $deliveries->total > 0
                ? round(($deliveries->successful / $deliveries->total) * 100, 2)
                : 100,
            'avg_response_time_ms' => round($deliveries->avg_response_time ?? 0, 2),
        ];

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get available events
     */
    public function events(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => self::AVAILABLE_EVENTS,
        ]);
    }

    /**
     * Get single webhook
     */
    public function show(string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::with('deliveries')->findOrFail((int) $id);

        // Get recent delivery stats
        $recentDeliveries = $webhook->deliveries()
            ->latest()
            ->limit(10)
            ->get();

        return response()->json([
            'success' => true,
            'data' => [
                'webhook' => $webhook,
                'recent_deliveries' => $recentDeliveries,
            ],
        ]);
    }

    /**
     * Create new webhook
     */
    public function store(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'name' => 'required|string|max:255',
            'url' => 'required|url|max:500',
            'events' => 'required|array|min:1',
            'events.*' => ['string', Rule::in(array_keys(self::AVAILABLE_EVENTS))],
            'secret' => 'nullable|string|min:32|max:64',
            'headers' => 'nullable|array',
            'description' => 'nullable|string|max:1000',
            'active' => 'boolean',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $this->auditService->startTiming();

        $secret = $request->get('secret') ?? Str::random(48);

        $webhook = EmailWebhook::create([
            'name' => $request->get('name'),
            'url' => $request->get('url'),
            'events' => $request->get('events'),
            'secret' => $secret,
            'headers' => $request->get('headers', []),
            'description' => $request->get('description'),
            'status' => $request->boolean('active', true) ? 'active' : 'inactive',
            'created_by' => auth()->id(),
        ]);

        $this->auditService->log(
            'create',
            'webhook',
            $webhook->id,
            $webhook->name,
            null,
            $webhook->toArray()
        );

        return response()->json([
            'success' => true,
            'data' => $webhook,
            'message' => 'Webhook created successfully',
        ], 201);
    }

    /**
     * Update webhook
     */
    public function update(Request $request, string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'name' => 'sometimes|string|max:255',
            'url' => 'sometimes|url|max:500',
            'events' => 'sometimes|array|min:1',
            'events.*' => ['string', Rule::in(array_keys(self::AVAILABLE_EVENTS))],
            'headers' => 'nullable|array',
            'description' => 'nullable|string|max:1000',
            'status' => ['nullable', Rule::in(['active', 'inactive'])],
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $this->auditService->startTiming();
        $oldValues = $webhook->toArray();

        $webhook->update($request->only([
            'name', 'url', 'events', 'headers', 'description', 'status',
        ]));

        $this->auditService->log(
            'update',
            'webhook',
            $webhook->id,
            $webhook->name,
            $oldValues,
            $webhook->fresh()->toArray()
        );

        return response()->json([
            'success' => true,
            'data' => $webhook->fresh(),
            'message' => 'Webhook updated successfully',
        ]);
    }

    /**
     * Delete webhook
     */
    public function destroy(string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $this->auditService->startTiming();
        $oldData = $webhook->toArray();

        $webhook->delete();

        $this->auditService->log(
            'delete',
            'webhook',
            (int) $id,
            $oldData['name'],
            $oldData,
            null
        );

        return response()->json([
            'success' => true,
            'message' => 'Webhook deleted successfully',
        ]);
    }

    /**
     * Test webhook
     */
    public function test(string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $testPayload = [
            'event' => 'webhook.test',
            'timestamp' => now()->toIso8601String(),
            'webhook_id' => $webhook->id,
            'data' => [
                'message' => 'This is a test webhook delivery',
                'test' => true,
            ],
        ];

        $signature = hash_hmac('sha256', json_encode($testPayload), $webhook->secret);

        try {
            $startTime = microtime(true);

            $response = Http::timeout(10)
                ->withHeaders(array_merge([
                    'Content-Type' => 'application/json',
                    'X-Webhook-Event' => 'webhook.test',
                    'X-Webhook-Signature' => "sha256={$signature}",
                    'X-Webhook-ID' => (string) Str::uuid(),
                    'X-Webhook-Timestamp' => (string) now()->timestamp,
                ], $webhook->headers ?? []))
                ->post($webhook->url, $testPayload);

            $responseTime = (int) ((microtime(true) - $startTime) * 1000);

            // Log delivery
            \DB::table('email_webhook_deliveries')->insert([
                'webhook_id' => $webhook->id,
                'event' => 'webhook.test',
                'payload' => json_encode($testPayload),
                'status' => $response->successful() ? 'success' : 'failed',
                'response_code' => $response->status(),
                'response_body' => substr($response->body(), 0, 1000),
                'response_time_ms' => $responseTime,
                'created_at' => now(),
            ]);

            if ($response->successful()) {
                return response()->json([
                    'success' => true,
                    'data' => [
                        'status_code' => $response->status(),
                        'response_time_ms' => $responseTime,
                        'response_body' => $response->json() ?? $response->body(),
                    ],
                    'message' => 'Webhook test successful',
                ]);
            }

            return response()->json([
                'success' => false,
                'data' => [
                    'status_code' => $response->status(),
                    'response_time_ms' => $responseTime,
                    'response_body' => $response->body(),
                ],
                'message' => 'Webhook test failed - received non-2xx response',
            ], 422);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Webhook test failed: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Rotate webhook secret
     */
    public function rotateSecret(string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $oldSecret = $webhook->secret;
        $newSecret = Str::random(48);

        $webhook->update([
            'secret' => $newSecret,
            'previous_secret' => $oldSecret,
            'secret_rotated_at' => now(),
        ]);

        $this->auditService->log(
            'secret_rotation',
            'webhook',
            $webhook->id,
            $webhook->name,
            ['secret' => '***'],
            ['secret' => '***'],
            ['rotated_at' => now()->toIso8601String()]
        );

        return response()->json([
            'success' => true,
            'data' => [
                'secret' => $newSecret,
                'rotated_at' => now()->toIso8601String(),
                'grace_period_until' => now()->addHours(24)->toIso8601String(),
            ],
            'message' => 'Secret rotated. Previous secret valid for 24 hours.',
        ]);
    }

    /**
     * Get webhook delivery history
     */
    public function deliveries(Request $request, string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $query = \DB::table('email_webhook_deliveries')
            ->where('webhook_id', $id)
            ->latest('created_at');

        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }

        if ($event = $request->get('event')) {
            $query->where('event', $event);
        }

        $deliveries = $query->paginate($request->integer('per_page', 25));

        return response()->json([
            'success' => true,
            'data' => $deliveries->items(),
            'meta' => [
                'current_page' => $deliveries->currentPage(),
                'last_page' => $deliveries->lastPage(),
                'per_page' => $deliveries->perPage(),
                'total' => $deliveries->total(),
            ],
        ]);
    }

    /**
     * Retry failed delivery
     */
    public function retryDelivery(string|int $deliveryId): JsonResponse
    {
        $delivery = \DB::table('email_webhook_deliveries')
            ->where('id', $deliveryId)
            ->first();

        if (!$delivery) {
            return response()->json([
                'success' => false,
                'message' => 'Delivery not found',
            ], 404);
        }

        $webhook = EmailWebhook::findOrFail($delivery->webhook_id);

        if ($webhook->status !== 'active') {
            return response()->json([
                'success' => false,
                'message' => 'Webhook is not active',
            ], 400);
        }

        $payload = json_decode($delivery->payload, true);
        $signature = hash_hmac('sha256', $delivery->payload, $webhook->secret);

        try {
            $startTime = microtime(true);

            $response = Http::timeout(10)
                ->withHeaders(array_merge([
                    'Content-Type' => 'application/json',
                    'X-Webhook-Event' => $delivery->event,
                    'X-Webhook-Signature' => "sha256={$signature}",
                    'X-Webhook-ID' => (string) Str::uuid(),
                    'X-Webhook-Retry' => 'true',
                ], $webhook->headers ?? []))
                ->post($webhook->url, $payload);

            $responseTime = (int) ((microtime(true) - $startTime) * 1000);

            // Log retry
            \DB::table('email_webhook_deliveries')->insert([
                'webhook_id' => $webhook->id,
                'event' => $delivery->event,
                'payload' => $delivery->payload,
                'status' => $response->successful() ? 'success' : 'failed',
                'response_code' => $response->status(),
                'response_body' => substr($response->body(), 0, 1000),
                'response_time_ms' => $responseTime,
                'retry_of' => $deliveryId,
                'created_at' => now(),
            ]);

            return response()->json([
                'success' => $response->successful(),
                'data' => [
                    'status_code' => $response->status(),
                    'response_time_ms' => $responseTime,
                ],
                'message' => $response->successful() ? 'Retry successful' : 'Retry failed',
            ]);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Retry failed: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Toggle webhook status
     */
    public function toggle(string|int $id): JsonResponse
    {
        $webhook = EmailWebhook::findOrFail((int) $id);

        $newStatus = $webhook->status === 'active' ? 'inactive' : 'active';
        $webhook->update(['status' => $newStatus]);

        // Reset failure count when activating
        if ($newStatus === 'active') {
            $webhook->update(['consecutive_failures' => 0]);
        }

        return response()->json([
            'success' => true,
            'data' => $webhook->fresh(),
            'message' => "Webhook {$newStatus}",
        ]);
    }
}
