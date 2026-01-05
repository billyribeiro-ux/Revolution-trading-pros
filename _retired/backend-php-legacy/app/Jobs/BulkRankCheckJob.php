<?php

namespace App\Jobs;

use App\Models\RankTracking;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;

/**
 * Job to check rankings for multiple keywords in bulk.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class BulkRankCheckJob implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 1;
    public int $timeout = 3600; // 1 hour for bulk operations

    /**
     * Create a new job instance.
     */
    public function __construct(
        public array $trackingIds,
        public array $options = []
    ) {}

    /**
     * Execute the job.
     */
    public function handle(): void
    {
        $batchSize = $this->options['batch_size'] ?? 10;
        $delay = $this->options['delay_between_checks'] ?? 2; // seconds

        $trackings = RankTracking::whereIn('id', $this->trackingIds)->get();

        Log::info('Starting bulk rank check', [
            'total_trackings' => $trackings->count(),
            'batch_size' => $batchSize,
        ]);

        $chunks = $trackings->chunk($batchSize);

        foreach ($chunks as $index => $chunk) {
            foreach ($chunk as $tracking) {
                // Dispatch individual check job with delay to avoid rate limiting
                CheckRankingsJob::dispatch($tracking)
                    ->delay(now()->addSeconds($index * $batchSize * $delay));
            }
        }

        Log::info('Bulk rank check jobs dispatched', [
            'total_jobs' => $trackings->count(),
        ]);
    }
}
