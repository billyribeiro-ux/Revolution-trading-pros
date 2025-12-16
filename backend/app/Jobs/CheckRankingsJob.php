<?php

namespace App\Jobs;

use App\Models\RankTracking;
use App\Models\RankHistory;
use App\Services\RankTracking\SerpApiService;
use App\Services\RankTracking\RankAlertService;
use App\Events\RankingChanged;
use App\Enums\RankChangeType;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;

/**
 * Job to check rankings for tracked keywords.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class CheckRankingsJob implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 3;
    public int $backoff = 60;

    /**
     * Create a new job instance.
     */
    public function __construct(
        public RankTracking $tracking
    ) {}

    /**
     * Execute the job.
     */
    public function handle(SerpApiService $serpApi, RankAlertService $alertService): void
    {
        try {
            $previousPosition = $this->tracking->current_position;

            $serpData = $serpApi->checkRanking(
                $this->tracking->keyword,
                $this->tracking->url,
                [
                    'search_engine' => $this->tracking->search_engine ?? 'google',
                    'location' => $this->tracking->location ?? 'United States',
                    'device' => $this->tracking->device ?? 'desktop',
                    'language' => $this->tracking->language ?? 'en',
                    'num_results' => 100,
                ]
            );

            // Update tracking
            $this->tracking->update([
                'current_position' => $serpData['position'],
                'previous_position' => $previousPosition,
                'search_volume' => $serpData['search_volume'] ?? null,
                'cpc' => $serpData['cpc'] ?? null,
                'competition' => $serpData['competition'] ?? null,
                'serp_features' => $serpData['serp_features'] ?? [],
                'top_competitors' => array_slice($serpData['top_results'] ?? [], 0, 10),
                'last_checked_at' => now(),
                'last_serp_data' => $serpData,
            ]);

            // Record history
            RankHistory::create([
                'rank_tracking_id' => $this->tracking->id,
                'position' => $serpData['position'],
                'search_volume' => $serpData['search_volume'] ?? null,
                'serp_features' => $serpData['serp_features'] ?? [],
                'competitors' => $serpData['top_results'] ?? [],
                'date' => now()->toDateString(),
                'checked_at' => now(),
            ]);

            // Check for significant changes
            if ($previousPosition !== null && $serpData['position'] !== null) {
                $change = abs($serpData['position'] - $previousPosition);
                
                if ($change >= 3) {
                    $changeType = $serpData['position'] < $previousPosition
                        ? RankChangeType::IMPROVED
                        : RankChangeType::DROPPED;

                    event(new RankingChanged($this->tracking, $previousPosition, $serpData['position'], $changeType));

                    // Create alert for significant drops
                    if ($changeType === RankChangeType::DROPPED && $change >= 10) {
                        $alertService->createDropAlert($this->tracking, $previousPosition, $serpData['position']);
                    }
                }
            }

            Log::info('Ranking checked successfully', [
                'tracking_id' => $this->tracking->id,
                'keyword' => $this->tracking->keyword,
                'position' => $serpData['position'],
            ]);

        } catch (\Exception $e) {
            Log::error('Failed to check ranking', [
                'tracking_id' => $this->tracking->id,
                'keyword' => $this->tracking->keyword,
                'error' => $e->getMessage(),
            ]);

            throw $e;
        }
    }
}
