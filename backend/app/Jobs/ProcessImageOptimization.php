<?php

declare(strict_types=1);

namespace App\Jobs;

use App\Models\ImageOptimizationJob;
use App\Models\Media;
use App\Services\ImageOptimizationService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;

/**
 * ProcessImageOptimization Job
 *
 * Background job for processing image optimization tasks.
 */
class ProcessImageOptimization implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 3;
    public int $timeout = 300; // 5 minutes
    public int $backoff = 60; // 1 minute between retries

    protected ImageOptimizationJob $optimizationJob;

    public function __construct(ImageOptimizationJob $optimizationJob)
    {
        $this->optimizationJob = $optimizationJob;
        $this->onQueue(config('image-optimization.queue.queue_name', 'image-optimization'));
    }

    public function handle(ImageOptimizationService $service): void
    {
        $job = $this->optimizationJob;
        $media = $job->media;

        if (!$media || !$media->exists) {
            $job->markAsFailed('Media not found');
            return;
        }

        if (!$media->isImage()) {
            $job->markAsFailed('Media is not an image');
            return;
        }

        try {
            // Mark as processing
            $job->markAsProcessing();

            // Get preset slug if available
            $presetSlug = $job->preset?->slug;

            // Process the image
            $job->updateProgress(10, 'Loading image');
            $media = $service->processImage($media, $presetSlug);

            // Mark as completed
            $job->update([
                'optimized_size' => $media->size,
            ]);
            $job->markAsCompleted([
                'variants_count' => count($media->variants ?? []),
                'optimized_size' => $media->size,
            ]);

            Log::info('Image optimization job completed', [
                'job_id' => $job->id,
                'media_id' => $media->id,
                'processing_time_ms' => $job->processing_time_ms,
            ]);

        } catch (\Exception $e) {
            Log::error('Image optimization job failed', [
                'job_id' => $job->id,
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);

            $job->markAsFailed($e->getMessage(), $e->getTraceAsString());

            // Re-throw if we haven't exhausted retries
            if ($job->canRetry()) {
                throw $e;
            }
        }
    }

    public function failed(\Throwable $exception): void
    {
        $this->optimizationJob->markAsFailed(
            $exception->getMessage(),
            $exception->getTraceAsString()
        );

        Log::error('Image optimization job permanently failed', [
            'job_id' => $this->optimizationJob->id,
            'error' => $exception->getMessage(),
        ]);
    }
}
