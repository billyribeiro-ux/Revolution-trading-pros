<?php

declare(strict_types=1);

namespace App\Jobs;

use App\Contracts\Email\InboundEmailProcessorInterface;
use App\DataTransferObjects\Email\InboundEmailPayload;
use App\Events\InboundEmailReceived;
use App\Exceptions\Email\InboundEmailException;
use App\Exceptions\Email\SpamDetectedException;
use App\Models\EmailMessage;
use App\Support\Logging\CorrelationContext;
use App\Support\Logging\StructuredLogger;
use App\Support\Metrics\MetricsCollector;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\Middleware\WithoutOverlapping;
use Illuminate\Queue\SerializesModels;
use Throwable;

/**
 * Async job for processing inbound emails.
 *
 * Uses exponential backoff for retries and ensures
 * idempotent processing with message ID uniqueness.
 *
 * @version 1.0.0
 */
class ProcessInboundEmailJob implements ShouldQueue, ShouldBeUnique
{
    use Dispatchable;
    use InteractsWithQueue;
    use Queueable;
    use SerializesModels;

    /**
     * Maximum retry attempts.
     */
    public int $tries = 5;

    /**
     * Maximum exceptions before failing.
     */
    public int $maxExceptions = 3;

    /**
     * Timeout in seconds.
     */
    public int $timeout = 120;

    /**
     * Delete job if models are missing.
     */
    public bool $deleteWhenMissingModels = true;

    /**
     * Create a new job instance.
     *
     * @param array<string, mixed> $payloadData Serialized payload data
     * @param string $provider Email provider name
     * @param string $correlationId Correlation ID for tracing
     */
    public function __construct(
        public readonly array $payloadData,
        public readonly string $provider,
        public readonly string $correlationId,
    ) {
        $this->onQueue('emails');
    }

    /**
     * Get the unique ID for the job.
     */
    public function uniqueId(): string
    {
        return $this->payloadData['messageId'] ?? md5(json_encode($this->payloadData));
    }

    /**
     * Get the middleware the job should pass through.
     *
     * @return array<object>
     */
    public function middleware(): array
    {
        return [
            // Prevent overlapping processing of same message
            new WithoutOverlapping($this->uniqueId()),
        ];
    }

    /**
     * Calculate the number of seconds to wait before retrying.
     *
     * @return array<int>
     */
    public function backoff(): array
    {
        // Exponential backoff: 10s, 30s, 60s, 120s, 300s
        return [10, 30, 60, 120, 300];
    }

    /**
     * Determine the time at which the job should timeout.
     */
    public function retryUntil(): \DateTime
    {
        return now()->addHours(2);
    }

    /**
     * Execute the job.
     */
    public function handle(
        InboundEmailProcessorInterface $processor,
        MetricsCollector $metrics,
    ): void {
        // Restore correlation context
        CorrelationContext::setCorrelationId($this->correlationId);
        $logger = StructuredLogger::email()->withContext([
            'job' => self::class,
            'provider' => $this->provider,
            'message_id' => $this->payloadData['messageId'] ?? 'unknown',
        ]);

        $startTime = microtime(true);
        $logger->operationStart('process_inbound_email');

        try {
            // Reconstruct payload from serialized data
            $payload = $this->reconstructPayload();

            // Check for duplicate processing
            if ($this->isDuplicate($payload->messageId)) {
                $logger->info('Skipping duplicate message', [
                    'message_id' => $payload->messageId,
                ]);
                return;
            }

            // Process the email
            $message = $processor->process($payload);

            // Record metrics
            $duration = (microtime(true) - $startTime) * 1000;
            $metrics->recordEmailProcessed($this->provider, 'success');
            $metrics->recordEmailProcessingDuration($duration / 1000, $this->provider);

            $logger->operationComplete('process_inbound_email', $duration, [
                'message_id' => $message->id,
                'conversation_id' => $message->conversation_id,
            ]);

            // Dispatch event
            event(new InboundEmailReceived($message));

        } catch (SpamDetectedException $e) {
            // Spam is not a failure - log and complete
            $metrics->recordEmailProcessed($this->provider, 'spam');
            $metrics->recordSpamDetection(true, $this->provider);

            $logger->warning('Email classified as spam', [
                'spam_score' => $e->getSpamScore(),
                'threshold' => $e->getThreshold(),
            ]);

        } catch (Throwable $e) {
            $duration = (microtime(true) - $startTime) * 1000;
            $metrics->recordEmailProcessed($this->provider, 'error');

            $logger->operationFailed('process_inbound_email', $e->getMessage(), $duration, [
                'exception' => get_class($e),
                'attempt' => $this->attempts(),
            ]);

            throw $e;
        }
    }

    /**
     * Handle a job failure.
     */
    public function failed(?Throwable $exception): void
    {
        $logger = StructuredLogger::email();

        $logger->error('Inbound email processing failed permanently', [
            'provider' => $this->provider,
            'message_id' => $this->payloadData['messageId'] ?? 'unknown',
            'exception' => $exception?->getMessage(),
            'attempts' => $this->attempts(),
            'correlation_id' => $this->correlationId,
        ]);

        // Could dispatch to dead letter queue or alert system here
    }

    /**
     * Reconstruct the payload from serialized data.
     */
    private function reconstructPayload(): InboundEmailPayload
    {
        return new InboundEmailPayload(
            messageId: $this->payloadData['messageId'],
            fromEmail: $this->payloadData['fromEmail'],
            fromName: $this->payloadData['fromName'] ?? null,
            toEmail: $this->payloadData['toEmail'],
            toName: $this->payloadData['toName'] ?? null,
            subject: $this->payloadData['subject'],
            textBody: $this->payloadData['textBody'] ?? null,
            htmlBody: $this->payloadData['htmlBody'] ?? null,
            receivedAt: \Carbon\CarbonImmutable::parse($this->payloadData['receivedAt']),
            inReplyTo: $this->payloadData['inReplyTo'] ?? null,
            references: $this->payloadData['references'] ?? [],
            headers: [], // Headers reconstructed separately if needed
            attachments: [], // Attachments handled separately
            mailboxHash: $this->payloadData['mailboxHash'] ?? null,
            spamScore: $this->payloadData['spamScore'] ?? 0.0,
            spamStatus: $this->payloadData['spamStatus'] ?? null,
            rawPayload: $this->payloadData['rawPayload'] ?? [],
            provider: $this->provider,
        );
    }

    /**
     * Check if this message has already been processed.
     */
    private function isDuplicate(string $messageId): bool
    {
        return EmailMessage::where('message_id', $messageId)->exists();
    }
}
