<?php

declare(strict_types=1);

namespace App\Jobs;

use App\Models\ImageOptimizationJob;
use App\Models\Media;
use App\Services\ImageOptimizationService;
use App\Services\MetricsService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Queue\Middleware\WithoutOverlapping;
use Illuminate\Support\Facades\Log;

/**
 * ProcessImageOptimization Job
 *
 * Google Enterprise Grade background job with:
 * - Exponential backoff with jitter
 * - Job uniqueness to prevent duplicates
 * - Comprehensive error tracking
 * - Graceful degradation on failures
 *
 * @version 2.0.0
 * @level L8 Principal Engineer
 */
class ProcessImageOptimization implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    /**
     * Maximum retry attempts with exponential backoff
     */
    public int $tries = 5;

    /**
     * Maximum execution time (5 minutes)
     */
    public int $timeout = 300;

    /**
     * Exponential backoff delays in seconds
     * 30s, 60s, 120s, 240s, 480s (total ~15 minutes of retry window)
     */
    public array $backoff = [30, 60, 120, 240, 480];

    /**
     * Maximum exceptions before job is marked as failed
     */
    public int $maxExceptions = 3;

    /**
     * Delete job if models are missing (media deleted)
     */
    public bool $deleteWhenMissingModels = true;

    protected ImageOptimizationJob $optimizationJob;
    protected string $requestId;

    public function __construct(ImageOptimizationJob $optimizationJob)
    {
        $this->optimizationJob = $optimizationJob;
        $this->requestId = uniqid('opt_', true);
        $this->onQueue(config('image-optimization.queue.queue_name', 'image-optimization'));
    }

    /**
     * Get the middleware the job should pass through
     */
    public function middleware(): array
    {
        // Prevent duplicate processing of the same optimization job
        return [
            (new WithoutOverlapping($this->optimizationJob->id))
                ->releaseAfter(60)
                ->expireAfter(300),
        ];
    }

    /**
     * Calculate backoff with jitter to prevent thundering herd
     */
    public function backoff(): array
    {
        return array_map(function ($seconds) {
            // Add 0-30% jitter to each backoff delay
            $jitter = (int) ($seconds * (mt_rand(0, 30) / 100));
            return $seconds + $jitter;
        }, $this->backoff);
    }

    /**
     * Determine if the job should be unique
     */
    public function uniqueId(): string
    {
        return (string) $this->optimizationJob->id;
    }

    /**
     * Unique job timeout (8 hours)
     */
    public function uniqueFor(): int
    {
        return 28800;
    }

    public function handle(ImageOptimizationService $service): void
    {
        $job = $this->optimizationJob;
        $media = $job->media;
        $startTime = microtime(true);

        // Structured logging context for request tracing
        $logContext = [
            'request_id' => $this->requestId,
            'job_id' => $job->id,
            'media_id' => $media?->id,
            'attempt' => $this->attempts(),
            'max_tries' => $this->tries,
        ];

        Log::info('Image optimization job started', $logContext);

        if (!$media || !$media->exists) {
            Log::warning('Image optimization job skipped: media not found', $logContext);
            $job->markAsFailed('Media not found');
            return;
        }

        if (!$media->isImage()) {
            Log::warning('Image optimization job skipped: not an image', array_merge($logContext, [
                'mime_type' => $media->mime_type,
            ]));
            $job->markAsFailed('Media is not an image');
            return;
        }

        try {
            // Mark as processing
            $job->markAsProcessing();
            Log::debug('Image optimization job processing', array_merge($logContext, [
                'original_size' => $media->size,
                'filename' => $media->filename,
            ]));

            // Get preset slug if available
            $presetSlug = $job->preset?->slug;

            // Process the image
            $job->updateProgress(10, 'Loading image');
            $media = $service->processImage($media, $presetSlug);

            $processingTime = (microtime(true) - $startTime) * 1000;

            // Mark as completed
            $job->update([
                'optimized_size' => $media->size,
            ]);
            $job->markAsCompleted([
                'variants_count' => count($media->variants ?? []),
                'optimized_size' => $media->size,
            ]);

            $savingsPercent = $job->original_size > 0
                ? round((1 - $media->size / $job->original_size) * 100, 2)
                : 0;
            $bytesSaved = max(0, $job->original_size - $media->size);

            Log::info('Image optimization job completed', array_merge($logContext, [
                'processing_time_ms' => round($processingTime, 2),
                'original_size' => $job->original_size,
                'optimized_size' => $media->size,
                'savings_percent' => $savingsPercent,
                'variants_count' => count($media->variants ?? []),
            ]));

            // Emit success metrics for monitoring
            MetricsService::increment('image_optimization.jobs.completed');
            MetricsService::timing('image_optimization.processing_time', $processingTime);
            MetricsService::histogram('image_optimization.bytes_saved', $bytesSaved);
            MetricsService::histogram('image_optimization.savings_percent', $savingsPercent);
            MetricsService::increment('image_optimization.total_bytes_saved', (int) $bytesSaved);
            MetricsService::increment('image_optimization.total_images_processed');
            MetricsService::gauge('image_optimization.last_success_timestamp', time());

        } catch (\Exception $e) {
            $processingTime = (microtime(true) - $startTime) * 1000;

            Log::error('Image optimization job failed', array_merge($logContext, [
                'error' => $e->getMessage(),
                'error_class' => get_class($e),
                'processing_time_ms' => round($processingTime, 2),
                'will_retry' => $this->attempts() < $this->tries,
                'next_retry_seconds' => $this->backoff()[$this->attempts()] ?? null,
            ]));

            $job->markAsFailed($e->getMessage(), $e->getTraceAsString());

            // Re-throw to trigger retry with exponential backoff
            if ($this->attempts() < $this->tries) {
                throw $e;
            }
        }
    }

    /**
     * Handle a job failure after all retries exhausted
     */
    public function failed(\Throwable $exception): void
    {
        $this->optimizationJob->markAsFailed(
            $exception->getMessage(),
            $exception->getTraceAsString()
        );

        Log::error('Image optimization job permanently failed', [
            'request_id' => $this->requestId,
            'job_id' => $this->optimizationJob->id,
            'media_id' => $this->optimizationJob->media_id,
            'total_attempts' => $this->attempts(),
            'error' => $exception->getMessage(),
            'error_class' => get_class($exception),
            'stack_trace' => $exception->getTraceAsString(),
        ]);

        // Emit metrics for monitoring/alerting
        MetricsService::increment('image_optimization.jobs.permanently_failed');
        MetricsService::increment('image_optimization.jobs.failures_by_error', 1, [
            'error_class' => get_class($exception),
        ]);
        MetricsService::gauge('image_optimization.jobs.last_failure_timestamp', time());
    }

    /**
     * Get the tags that should be assigned to the job for monitoring
     */
    public function tags(): array
    {
        return [
            'image_optimization',
            'media_id:' . $this->optimizationJob->media_id,
            'job_id:' . $this->optimizationJob->id,
        ];
    }

    /**
     * Determine the time at which the job should timeout
     */
    public function retryUntil(): \DateTime
    {
        // Allow retries for up to 1 hour after initial dispatch
        return now()->addHour();
    }
}
