<?php

declare(strict_types=1);

namespace App\Services\Performance;

use Illuminate\Support\Facades\Queue;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Bus\Batch;
use Illuminate\Support\Facades\Bus;
use Throwable;

/**
 * Async Processor Service (ICT9+ Enterprise Grade)
 *
 * Optimizes async processing through:
 * - Job batching for bulk operations
 * - Priority queuing
 * - Rate limiting for external APIs
 * - Retry strategies with exponential backoff
 * - Dead letter queue handling
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class AsyncProcessor
{
    /**
     * Queue priorities
     */
    public const PRIORITY_CRITICAL = 'critical';  // Process immediately
    public const PRIORITY_HIGH = 'high';          // Process within seconds
    public const PRIORITY_DEFAULT = 'default';    // Normal processing
    public const PRIORITY_LOW = 'low';            // Background tasks
    public const PRIORITY_BULK = 'bulk';          // Large batch operations

    /**
     * Dispatch a job with optimal settings
     */
    public function dispatch(object $job, string $priority = self::PRIORITY_DEFAULT): void
    {
        $queue = $this->getQueueForPriority($priority);

        dispatch($job)
            ->onQueue($queue)
            ->afterCommit(); // Only dispatch after DB transaction commits
    }

    /**
     * Dispatch multiple jobs as a batch
     */
    public function batch(array $jobs, ?string $name = null, ?callable $onComplete = null): ?Batch
    {
        if (empty($jobs)) {
            return null;
        }

        $batch = Bus::batch($jobs)
            ->name($name ?? 'Batch-' . now()->format('Y-m-d-H-i-s'))
            ->allowFailures()
            ->onQueue(self::PRIORITY_BULK);

        if ($onComplete) {
            $batch->then($onComplete);
        }

        $batch->catch(function (Batch $batch, Throwable $e) {
            Log::error('Batch job failed', [
                'batch_id' => $batch->id,
                'batch_name' => $batch->name,
                'error' => $e->getMessage(),
                'failed_jobs' => $batch->failedJobs,
            ]);
        });

        return $batch->dispatch();
    }

    /**
     * Dispatch with rate limiting
     */
    public function dispatchRateLimited(
        object $job,
        string $limiterKey,
        int $maxPerMinute = 60,
        string $priority = self::PRIORITY_DEFAULT
    ): void {
        $executed = Cache::get("rate_limit:{$limiterKey}:count", 0);

        if ($executed >= $maxPerMinute) {
            // Delay until next minute
            $delay = 60 - (time() % 60);
            dispatch($job)
                ->onQueue($this->getQueueForPriority($priority))
                ->delay(now()->addSeconds($delay));

            Log::info('Job rate limited, delayed', [
                'job' => get_class($job),
                'delay_seconds' => $delay,
                'limiter' => $limiterKey,
            ]);
        } else {
            Cache::increment("rate_limit:{$limiterKey}:count");
            Cache::put("rate_limit:{$limiterKey}:count", $executed + 1, 60);

            dispatch($job)->onQueue($this->getQueueForPriority($priority));
        }
    }

    /**
     * Dispatch with exponential backoff retry
     */
    public function dispatchWithRetry(
        object $job,
        int $maxAttempts = 3,
        array $backoffSeconds = [10, 60, 300]
    ): void {
        // Set retry configuration on job if it supports it
        if (method_exists($job, 'tries')) {
            $job->tries = $maxAttempts;
        }

        if (method_exists($job, 'backoff')) {
            $job->backoff = $backoffSeconds;
        }

        dispatch($job)->afterCommit();
    }

    /**
     * Dispatch after response is sent (fire and forget)
     */
    public function dispatchAfterResponse(callable $callback): void
    {
        dispatch($callback)->afterResponse();
    }

    /**
     * Chain jobs to run sequentially
     */
    public function chain(array $jobs, string $priority = self::PRIORITY_DEFAULT): void
    {
        Bus::chain($jobs)
            ->onQueue($this->getQueueForPriority($priority))
            ->dispatch();
    }

    /**
     * Get optimal queue for priority
     */
    private function getQueueForPriority(string $priority): string
    {
        return match ($priority) {
            self::PRIORITY_CRITICAL => 'critical',
            self::PRIORITY_HIGH => 'high',
            self::PRIORITY_LOW => 'low',
            self::PRIORITY_BULK => 'bulk',
            default => 'default',
        };
    }

    /**
     * Get queue health status
     */
    public function getQueueHealth(): array
    {
        $queues = ['critical', 'high', 'default', 'low', 'bulk'];
        $health = [];

        foreach ($queues as $queue) {
            try {
                $size = Queue::size($queue);
                $health[$queue] = [
                    'size' => $size,
                    'status' => $size < 1000 ? 'healthy' : ($size < 5000 ? 'warning' : 'critical'),
                ];
            } catch (Throwable $e) {
                $health[$queue] = ['size' => -1, 'status' => 'unknown', 'error' => $e->getMessage()];
            }
        }

        return $health;
    }

    /**
     * Purge failed jobs older than specified days
     */
    public function purgeFailedJobs(int $olderThanDays = 7): int
    {
        return \DB::table('failed_jobs')
            ->where('failed_at', '<', now()->subDays($olderThanDays))
            ->delete();
    }
}
