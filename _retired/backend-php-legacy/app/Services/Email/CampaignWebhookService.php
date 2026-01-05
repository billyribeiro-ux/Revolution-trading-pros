<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailCampaign;
use App\Models\EmailWebhook;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * CampaignWebhookService - Real-time Campaign Progress Notifications
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Provides webhook notifications for campaign progress events:
 * - Campaign started
 * - Progress updates (every N% or N emails)
 * - Campaign completed
 * - Campaign failed
 * - Bounce/complaint alerts
 * - Performance milestones
 *
 * @version 1.0.0
 */
class CampaignWebhookService
{
    /**
     * Progress notification intervals
     */
    private const PROGRESS_INTERVALS = [10, 25, 50, 75, 90, 100];

    /**
     * Webhook timeout in seconds
     */
    private const WEBHOOK_TIMEOUT = 10;

    /**
     * Max retry attempts
     */
    private const MAX_RETRIES = 3;

    /**
     * Event types
     */
    public const EVENT_CAMPAIGN_STARTED = 'campaign.started';
    public const EVENT_CAMPAIGN_PROGRESS = 'campaign.progress';
    public const EVENT_CAMPAIGN_COMPLETED = 'campaign.completed';
    public const EVENT_CAMPAIGN_FAILED = 'campaign.failed';
    public const EVENT_CAMPAIGN_PAUSED = 'campaign.paused';
    public const EVENT_CAMPAIGN_RESUMED = 'campaign.resumed';
    public const EVENT_EMAIL_SENT = 'email.sent';
    public const EVENT_EMAIL_BOUNCED = 'email.bounced';
    public const EVENT_EMAIL_COMPLAINED = 'email.complained';
    public const EVENT_EMAIL_OPENED = 'email.opened';
    public const EVENT_EMAIL_CLICKED = 'email.clicked';
    public const EVENT_MILESTONE_REACHED = 'milestone.reached';

    /**
     * Notify campaign started
     */
    public function notifyCampaignStarted(EmailCampaign $campaign): void
    {
        $this->dispatchWebhook(self::EVENT_CAMPAIGN_STARTED, [
            'campaign_id' => $campaign->id,
            'campaign_name' => $campaign->name,
            'total_recipients' => $campaign->total_recipients,
            'started_at' => now()->toIso8601String(),
            'estimated_completion' => $this->estimateCompletion($campaign),
        ], $campaign);
    }

    /**
     * Notify campaign progress
     */
    public function notifyCampaignProgress(EmailCampaign $campaign, int $sent, int $total): void
    {
        $percentage = $total > 0 ? round(($sent / $total) * 100) : 0;

        // Only notify at specific intervals
        $shouldNotify = false;
        foreach (self::PROGRESS_INTERVALS as $interval) {
            $cacheKey = "campaign_progress:{$campaign->id}:{$interval}";
            if ($percentage >= $interval && !Cache::has($cacheKey)) {
                Cache::put($cacheKey, true, 3600);
                $shouldNotify = true;
                break;
            }
        }

        if (!$shouldNotify) {
            return;
        }

        $this->dispatchWebhook(self::EVENT_CAMPAIGN_PROGRESS, [
            'campaign_id' => $campaign->id,
            'campaign_name' => $campaign->name,
            'sent' => $sent,
            'total' => $total,
            'percentage' => $percentage,
            'remaining' => $total - $sent,
            'current_stats' => [
                'opened' => $campaign->opened_count,
                'clicked' => $campaign->clicked_count,
                'bounced' => $campaign->bounced_count,
            ],
            'updated_at' => now()->toIso8601String(),
        ], $campaign);
    }

    /**
     * Notify campaign completed
     */
    public function notifyCampaignCompleted(EmailCampaign $campaign): void
    {
        $this->dispatchWebhook(self::EVENT_CAMPAIGN_COMPLETED, [
            'campaign_id' => $campaign->id,
            'campaign_name' => $campaign->name,
            'stats' => [
                'sent' => $campaign->sent_count,
                'opened' => $campaign->opened_count,
                'clicked' => $campaign->clicked_count,
                'bounced' => $campaign->bounced_count,
                'unsubscribed' => $campaign->unsubscribed_count,
                'open_rate' => $campaign->open_rate,
                'click_rate' => $campaign->click_rate,
                'bounce_rate' => $campaign->bounce_rate,
            ],
            'duration_seconds' => $campaign->sent_at
                ? now()->diffInSeconds($campaign->sent_at)
                : null,
            'completed_at' => now()->toIso8601String(),
        ], $campaign);

        // Clear progress cache
        foreach (self::PROGRESS_INTERVALS as $interval) {
            Cache::forget("campaign_progress:{$campaign->id}:{$interval}");
        }
    }

    /**
     * Notify campaign failed
     */
    public function notifyCampaignFailed(EmailCampaign $campaign, string $error): void
    {
        $this->dispatchWebhook(self::EVENT_CAMPAIGN_FAILED, [
            'campaign_id' => $campaign->id,
            'campaign_name' => $campaign->name,
            'error' => $error,
            'sent_before_failure' => $campaign->sent_count,
            'failed_at' => now()->toIso8601String(),
        ], $campaign);
    }

    /**
     * Notify email event (batch for performance)
     */
    public function notifyEmailEvent(string $event, array $data, EmailCampaign $campaign): void
    {
        // Batch email events to reduce webhook calls
        $batchKey = "email_events:{$campaign->id}:{$event}";
        $batch = Cache::get($batchKey, []);
        $batch[] = $data;

        // Send batch every 100 events or 30 seconds
        if (count($batch) >= 100) {
            $this->dispatchWebhook($event, [
                'campaign_id' => $campaign->id,
                'events' => $batch,
                'count' => count($batch),
            ], $campaign);
            Cache::forget($batchKey);
        } else {
            Cache::put($batchKey, $batch, 30);
        }
    }

    /**
     * Notify milestone reached
     */
    public function notifyMilestone(EmailCampaign $campaign, string $milestone, array $data = []): void
    {
        $this->dispatchWebhook(self::EVENT_MILESTONE_REACHED, array_merge([
            'campaign_id' => $campaign->id,
            'campaign_name' => $campaign->name,
            'milestone' => $milestone,
            'reached_at' => now()->toIso8601String(),
        ], $data), $campaign);
    }

    /**
     * Dispatch webhook to all registered endpoints
     */
    private function dispatchWebhook(string $event, array $payload, EmailCampaign $campaign): void
    {
        // Get registered webhooks for this event
        $webhooks = $this->getWebhooksForEvent($event, $campaign);

        foreach ($webhooks as $webhook) {
            $this->sendWebhook($webhook, $event, $payload);
        }
    }

    /**
     * Get webhooks registered for an event
     */
    private function getWebhooksForEvent(string $event, EmailCampaign $campaign): array
    {
        return Cache::remember("webhooks:email:{$event}", 300, function () use ($event) {
            // Check if EmailWebhook model exists, otherwise use config
            if (class_exists(EmailWebhook::class)) {
                return EmailWebhook::where('is_active', true)
                    ->where(function ($q) use ($event) {
                        $q->whereJsonContains('events', $event)
                          ->orWhereJsonContains('events', '*');
                    })
                    ->get()
                    ->toArray();
            }

            // Fallback to config-based webhooks
            return config('email.webhooks', []);
        });
    }

    /**
     * Send webhook to endpoint
     */
    private function sendWebhook(array $webhook, string $event, array $payload, int $attempt = 1): void
    {
        try {
            $url = $webhook['url'] ?? null;
            if (!$url) {
                return;
            }

            $headers = [
                'Content-Type' => 'application/json',
                'X-Webhook-Event' => $event,
                'X-Webhook-Timestamp' => now()->timestamp,
                'X-Webhook-Signature' => $this->generateSignature($payload, $webhook['secret'] ?? ''),
                'X-Webhook-ID' => (string) Str::uuid(),
            ];

            // Add custom headers
            if (!empty($webhook['headers'])) {
                $headers = array_merge($headers, $webhook['headers']);
            }

            $response = Http::timeout(self::WEBHOOK_TIMEOUT)
                ->withHeaders($headers)
                ->post($url, [
                    'event' => $event,
                    'timestamp' => now()->toIso8601String(),
                    'data' => $payload,
                ]);

            if (!$response->successful()) {
                throw new \Exception("Webhook failed with status {$response->status()}");
            }

            Log::debug('[CampaignWebhookService] Webhook sent', [
                'event' => $event,
                'url' => $url,
                'status' => $response->status(),
            ]);

        } catch (\Throwable $e) {
            Log::warning('[CampaignWebhookService] Webhook failed', [
                'event' => $event,
                'url' => $webhook['url'] ?? 'unknown',
                'attempt' => $attempt,
                'error' => $e->getMessage(),
            ]);

            // Retry with exponential backoff
            if ($attempt < self::MAX_RETRIES) {
                $delay = pow(2, $attempt);
                dispatch(function () use ($webhook, $event, $payload, $attempt) {
                    $this->sendWebhook($webhook, $event, $payload, $attempt + 1);
                })->delay(now()->addSeconds($delay));
            }
        }
    }

    /**
     * Generate webhook signature
     */
    private function generateSignature(array $payload, string $secret): string
    {
        if (empty($secret)) {
            return '';
        }

        return hash_hmac('sha256', json_encode($payload), $secret);
    }

    /**
     * Estimate campaign completion time
     */
    private function estimateCompletion(EmailCampaign $campaign): ?string
    {
        // Estimate based on 100 emails per second
        $emailsPerSecond = 100;
        $estimatedSeconds = $campaign->total_recipients / $emailsPerSecond;

        return now()->addSeconds((int) $estimatedSeconds)->toIso8601String();
    }

    /**
     * Flush any pending batched events
     */
    public function flushBatchedEvents(EmailCampaign $campaign): void
    {
        $events = [
            self::EVENT_EMAIL_SENT,
            self::EVENT_EMAIL_BOUNCED,
            self::EVENT_EMAIL_COMPLAINED,
            self::EVENT_EMAIL_OPENED,
            self::EVENT_EMAIL_CLICKED,
        ];

        foreach ($events as $event) {
            $batchKey = "email_events:{$campaign->id}:{$event}";
            $batch = Cache::get($batchKey, []);

            if (!empty($batch)) {
                $this->dispatchWebhook($event, [
                    'campaign_id' => $campaign->id,
                    'events' => $batch,
                    'count' => count($batch),
                ], $campaign);
                Cache::forget($batchKey);
            }
        }
    }

    /**
     * Register a webhook endpoint
     */
    public function registerWebhook(string $url, array $events = ['*'], string $secret = ''): array
    {
        $webhook = [
            'id' => (string) Str::uuid(),
            'url' => $url,
            'events' => $events,
            'secret' => $secret ?: Str::random(32),
            'is_active' => true,
            'created_at' => now()->toIso8601String(),
        ];

        // Store webhook (in production, use EmailWebhook model)
        $webhooks = config('email.webhooks', []);
        $webhooks[] = $webhook;

        // Clear cache
        foreach ($events as $event) {
            Cache::forget("webhooks:email:{$event}");
        }

        return $webhook;
    }
}
