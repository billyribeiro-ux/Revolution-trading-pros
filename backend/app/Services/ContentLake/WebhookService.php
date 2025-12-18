<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Throwable;

/**
 * Webhook Service
 *
 * Implements Sanity-style webhooks for real-time integrations.
 *
 * Features:
 * - Event-based triggers
 * - Payload filtering and projection
 * - Signature verification (HMAC-SHA256)
 * - Retry with exponential backoff
 * - Delivery tracking
 * - Rate limiting
 */
class WebhookService
{
    private const MAX_PAYLOAD_SIZE = 1048576; // 1MB
    private const DEFAULT_TIMEOUT = 30000; // 30 seconds

    /**
     * Webhook events
     */
    public const EVENTS = [
        'document.created',
        'document.updated',
        'document.deleted',
        'document.published',
        'document.unpublished',
        'media.uploaded',
        'media.deleted',
        'release.published',
        'user.created',
        'user.updated',
    ];

    /**
     * Dispatch event to all matching webhooks
     */
    public function dispatch(string $event, array $payload): int
    {
        if (!in_array($event, self::EVENTS, true)) {
            Log::warning("Unknown webhook event: {$event}");
        }

        $webhooks = $this->getMatchingWebhooks($event, $payload);
        $dispatched = 0;

        foreach ($webhooks as $webhook) {
            $this->queueDelivery($webhook, $event, $payload);
            $dispatched++;
        }

        return $dispatched;
    }

    /**
     * Get webhooks matching the event
     */
    private function getMatchingWebhooks(string $event, array $payload): array
    {
        $webhooks = DB::table('webhooks')
            ->where('is_active', true)
            ->whereNull('deleted_at')
            ->get();

        return $webhooks->filter(function ($webhook) use ($event, $payload) {
            $events = json_decode($webhook->events, true);

            // Check if event matches
            if (!in_array($event, $events, true) && !in_array('*', $events, true)) {
                return false;
            }

            // Check filter if present
            if ($webhook->filter) {
                $filter = json_decode($webhook->filter, true);
                if (!$this->matchesFilter($payload, $filter)) {
                    return false;
                }
            }

            return true;
        })->values()->toArray();
    }

    /**
     * Check if payload matches filter
     */
    private function matchesFilter(array $payload, array $filter): bool
    {
        foreach ($filter as $field => $value) {
            $actualValue = data_get($payload, $field);

            if (is_array($value)) {
                if (isset($value['$eq']) && $actualValue !== $value['$eq']) {
                    return false;
                }
                if (isset($value['$ne']) && $actualValue === $value['$ne']) {
                    return false;
                }
                if (isset($value['$in']) && !in_array($actualValue, $value['$in'], true)) {
                    return false;
                }
                if (isset($value['$nin']) && in_array($actualValue, $value['$nin'], true)) {
                    return false;
                }
                if (isset($value['$regex']) && !preg_match($value['$regex'], $actualValue)) {
                    return false;
                }
            } elseif ($actualValue !== $value) {
                return false;
            }
        }

        return true;
    }

    /**
     * Queue webhook delivery
     */
    private function queueDelivery(object $webhook, string $event, array $payload): void
    {
        $deliveryId = Str::uuid()->toString();

        // Apply projection if specified
        $projectedPayload = $webhook->projection
            ? $this->applyProjection($payload, json_decode($webhook->projection, true))
            : $payload;

        // Add metadata
        $fullPayload = [
            '_id' => $deliveryId,
            '_event' => $event,
            '_timestamp' => now()->toIso8601String(),
            '_webhookId' => $webhook->webhook_id,
            'data' => $projectedPayload,
        ];

        // Check payload size
        $payloadJson = json_encode($fullPayload);
        if (strlen($payloadJson) > self::MAX_PAYLOAD_SIZE) {
            Log::warning("Webhook payload too large", [
                'webhook_id' => $webhook->webhook_id,
                'size' => strlen($payloadJson),
            ]);
            $fullPayload['data'] = ['_truncated' => true, '_id' => $payload['_id'] ?? null];
            $payloadJson = json_encode($fullPayload);
        }

        // Create delivery record
        DB::table('webhook_deliveries')->insert([
            'delivery_id' => $deliveryId,
            'webhook_id' => $webhook->id,
            'event_type' => $event,
            'document_id' => $payload['documentId'] ?? $payload['_id'] ?? null,
            'document_type' => $payload['documentType'] ?? $payload['_type'] ?? null,
            'payload' => $payloadJson,
            'status' => 'pending',
            'attempt_count' => 0,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        // Dispatch job (in production, use queue)
        $this->processDelivery($deliveryId);
    }

    /**
     * Apply projection to payload
     */
    private function applyProjection(array $payload, array $projection): array
    {
        $result = [];

        foreach ($projection as $field) {
            if (str_contains($field, '.')) {
                data_set($result, $field, data_get($payload, $field));
            } else {
                if (array_key_exists($field, $payload)) {
                    $result[$field] = $payload[$field];
                }
            }
        }

        return $result;
    }

    /**
     * Process a webhook delivery
     */
    public function processDelivery(string $deliveryId): bool
    {
        $delivery = DB::table('webhook_deliveries')
            ->where('delivery_id', $deliveryId)
            ->first();

        if (!$delivery || $delivery->status === 'success') {
            return false;
        }

        $webhook = DB::table('webhooks')
            ->where('id', $delivery->webhook_id)
            ->first();

        if (!$webhook || !$webhook->is_active) {
            DB::table('webhook_deliveries')
                ->where('delivery_id', $deliveryId)
                ->update([
                    'status' => 'failed',
                    'error_message' => 'Webhook is inactive or deleted',
                    'updated_at' => now(),
                ]);
            return false;
        }

        $attemptCount = $delivery->attempt_count + 1;
        $payload = $delivery->payload;

        // Update attempt count
        DB::table('webhook_deliveries')
            ->where('delivery_id', $deliveryId)
            ->update([
                'attempt_count' => $attemptCount,
                'status' => 'retrying',
                'updated_at' => now(),
            ]);

        try {
            $startTime = microtime(true);

            // Build headers
            $headers = [
                'Content-Type' => 'application/json',
                'User-Agent' => 'Revolution-CMS-Webhook/1.0',
                'X-Webhook-ID' => $webhook->webhook_id,
                'X-Delivery-ID' => $deliveryId,
                'X-Event-Type' => $delivery->event_type,
            ];

            // Add signature if secret is set
            if ($webhook->secret) {
                $signature = $this->generateSignature($payload, $webhook->secret);
                $headers['X-Webhook-Signature'] = $signature;
                $headers['X-Webhook-Signature-256'] = 'sha256=' . $signature;
            }

            // Add custom headers
            if ($webhook->headers) {
                $customHeaders = json_decode($webhook->headers, true);
                $headers = array_merge($headers, $customHeaders);
            }

            // Make request
            $response = Http::withHeaders($headers)
                ->timeout($webhook->timeout_ms / 1000)
                ->withBody($payload, 'application/json')
                ->{strtolower($webhook->http_method)}($webhook->url);

            $duration = (int) ((microtime(true) - $startTime) * 1000);

            // Update delivery record
            $status = $response->successful() ? 'success' : 'failed';

            DB::table('webhook_deliveries')
                ->where('delivery_id', $deliveryId)
                ->update([
                    'response_headers' => json_encode($response->headers()),
                    'response_body' => substr($response->body(), 0, 10000),
                    'response_status' => $response->status(),
                    'status' => $status,
                    'duration_ms' => $duration,
                    'delivered_at' => $status === 'success' ? now() : null,
                    'error_message' => $status === 'failed' ? "HTTP {$response->status()}" : null,
                    'updated_at' => now(),
                ]);

            if (!$response->successful() && $attemptCount < $webhook->max_retries) {
                $this->scheduleRetry($deliveryId, $attemptCount, $webhook->retry_delay_ms);
            }

            return $response->successful();

        } catch (Throwable $e) {
            $duration = (int) ((microtime(true) - $startTime) * 1000);

            DB::table('webhook_deliveries')
                ->where('delivery_id', $deliveryId)
                ->update([
                    'status' => 'failed',
                    'duration_ms' => $duration,
                    'error_message' => $e->getMessage(),
                    'updated_at' => now(),
                ]);

            if ($attemptCount < $webhook->max_retries) {
                $this->scheduleRetry($deliveryId, $attemptCount, $webhook->retry_delay_ms);
            }

            Log::error("Webhook delivery failed", [
                'delivery_id' => $deliveryId,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Schedule retry with exponential backoff
     */
    private function scheduleRetry(string $deliveryId, int $attemptCount, int $baseDelayMs): void
    {
        $delay = $baseDelayMs * pow(2, $attemptCount - 1);
        $nextRetry = now()->addMilliseconds($delay);

        DB::table('webhook_deliveries')
            ->where('delivery_id', $deliveryId)
            ->update([
                'status' => 'retrying',
                'next_retry_at' => $nextRetry,
                'updated_at' => now(),
            ]);
    }

    /**
     * Generate HMAC signature
     */
    private function generateSignature(string $payload, string $secret): string
    {
        return hash_hmac('sha256', $payload, $secret);
    }

    /**
     * Verify webhook signature
     */
    public function verifySignature(string $payload, string $signature, string $secret): bool
    {
        $expected = $this->generateSignature($payload, $secret);
        return hash_equals($expected, $signature);
    }

    /**
     * Create a new webhook
     */
    public function create(array $data): array
    {
        $webhookId = Str::uuid()->toString();
        $secret = $data['secret'] ?? Str::random(32);

        $id = DB::table('webhooks')->insertGetId([
            'webhook_id' => $webhookId,
            'name' => $data['name'],
            'url' => $data['url'],
            'secret' => $secret,
            'events' => json_encode($data['events'] ?? ['*']),
            'filter' => isset($data['filter']) ? json_encode($data['filter']) : null,
            'projection' => isset($data['projection']) ? json_encode($data['projection']) : null,
            'http_method' => $data['http_method'] ?? 'POST',
            'headers' => isset($data['headers']) ? json_encode($data['headers']) : null,
            'is_active' => $data['is_active'] ?? true,
            'timeout_ms' => $data['timeout_ms'] ?? self::DEFAULT_TIMEOUT,
            'max_retries' => $data['max_retries'] ?? 3,
            'retry_delay_ms' => $data['retry_delay_ms'] ?? 1000,
            'created_by' => auth()->id(),
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        return [
            'id' => $id,
            'webhook_id' => $webhookId,
            'secret' => $secret,
        ];
    }

    /**
     * Update webhook
     */
    public function update(string $webhookId, array $data): bool
    {
        $updateData = array_filter([
            'name' => $data['name'] ?? null,
            'url' => $data['url'] ?? null,
            'events' => isset($data['events']) ? json_encode($data['events']) : null,
            'filter' => isset($data['filter']) ? json_encode($data['filter']) : null,
            'projection' => isset($data['projection']) ? json_encode($data['projection']) : null,
            'http_method' => $data['http_method'] ?? null,
            'headers' => isset($data['headers']) ? json_encode($data['headers']) : null,
            'is_active' => $data['is_active'] ?? null,
            'timeout_ms' => $data['timeout_ms'] ?? null,
            'max_retries' => $data['max_retries'] ?? null,
            'retry_delay_ms' => $data['retry_delay_ms'] ?? null,
            'updated_at' => now(),
        ], fn($v) => $v !== null);

        return DB::table('webhooks')
            ->where('webhook_id', $webhookId)
            ->update($updateData) > 0;
    }

    /**
     * Delete webhook (soft delete)
     */
    public function delete(string $webhookId): bool
    {
        return DB::table('webhooks')
            ->where('webhook_id', $webhookId)
            ->update([
                'is_active' => false,
                'deleted_at' => now(),
                'updated_at' => now(),
            ]) > 0;
    }

    /**
     * List webhooks
     */
    public function list(array $options = []): array
    {
        $query = DB::table('webhooks')
            ->whereNull('deleted_at');

        if (isset($options['is_active'])) {
            $query->where('is_active', $options['is_active']);
        }

        $total = $query->count();
        $page = $options['page'] ?? 1;
        $perPage = $options['perPage'] ?? 20;

        $items = $query
            ->orderBy('created_at', 'desc')
            ->skip(($page - 1) * $perPage)
            ->take($perPage)
            ->get();

        return [
            'items' => $items->map(fn($w) => [
                'id' => $w->webhook_id,
                'name' => $w->name,
                'url' => $w->url,
                'events' => json_decode($w->events, true),
                'is_active' => (bool) $w->is_active,
                'created_at' => $w->created_at,
            ])->toArray(),
            'total' => $total,
            'page' => $page,
            'perPage' => $perPage,
        ];
    }

    /**
     * Get webhook delivery history
     */
    public function getDeliveries(string $webhookId, array $options = []): array
    {
        $webhook = DB::table('webhooks')
            ->where('webhook_id', $webhookId)
            ->first();

        if (!$webhook) {
            return ['items' => [], 'total' => 0];
        }

        $query = DB::table('webhook_deliveries')
            ->where('webhook_id', $webhook->id);

        if (isset($options['status'])) {
            $query->where('status', $options['status']);
        }

        $total = $query->count();
        $page = $options['page'] ?? 1;
        $perPage = $options['perPage'] ?? 50;

        $items = $query
            ->orderBy('created_at', 'desc')
            ->skip(($page - 1) * $perPage)
            ->take($perPage)
            ->get();

        return [
            'items' => $items->map(fn($d) => [
                'id' => $d->delivery_id,
                'event' => $d->event_type,
                'documentId' => $d->document_id,
                'status' => $d->status,
                'responseStatus' => $d->response_status,
                'attemptCount' => $d->attempt_count,
                'durationMs' => $d->duration_ms,
                'error' => $d->error_message,
                'deliveredAt' => $d->delivered_at,
                'createdAt' => $d->created_at,
            ])->toArray(),
            'total' => $total,
            'page' => $page,
            'perPage' => $perPage,
        ];
    }

    /**
     * Retry failed deliveries
     */
    public function retryFailed(string $webhookId): int
    {
        $webhook = DB::table('webhooks')
            ->where('webhook_id', $webhookId)
            ->first();

        if (!$webhook) {
            return 0;
        }

        $failed = DB::table('webhook_deliveries')
            ->where('webhook_id', $webhook->id)
            ->where('status', 'failed')
            ->get();

        $retried = 0;
        foreach ($failed as $delivery) {
            DB::table('webhook_deliveries')
                ->where('delivery_id', $delivery->delivery_id)
                ->update([
                    'status' => 'pending',
                    'attempt_count' => 0,
                    'error_message' => null,
                    'updated_at' => now(),
                ]);
            $this->processDelivery($delivery->delivery_id);
            $retried++;
        }

        return $retried;
    }

    /**
     * Test webhook with sample payload
     */
    public function test(string $webhookId): array
    {
        $webhook = DB::table('webhooks')
            ->where('webhook_id', $webhookId)
            ->first();

        if (!$webhook) {
            throw new \InvalidArgumentException("Webhook not found: {$webhookId}");
        }

        $testPayload = [
            'documentId' => 'test-' . Str::random(8),
            'documentType' => 'post',
            '_test' => true,
            'timestamp' => now()->toIso8601String(),
        ];

        $deliveryId = Str::uuid()->toString();

        DB::table('webhook_deliveries')->insert([
            'delivery_id' => $deliveryId,
            'webhook_id' => $webhook->id,
            'event_type' => 'test',
            'document_id' => $testPayload['documentId'],
            'document_type' => 'post',
            'payload' => json_encode($testPayload),
            'status' => 'pending',
            'attempt_count' => 0,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        $success = $this->processDelivery($deliveryId);

        $delivery = DB::table('webhook_deliveries')
            ->where('delivery_id', $deliveryId)
            ->first();

        return [
            'success' => $success,
            'deliveryId' => $deliveryId,
            'responseStatus' => $delivery->response_status,
            'durationMs' => $delivery->duration_ms,
            'error' => $delivery->error_message,
        ];
    }
}
