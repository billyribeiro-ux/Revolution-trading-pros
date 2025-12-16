<?php

namespace App\Services\RankTracking;

use App\Models\RankTracking;
use App\Models\RankHistory;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * Service for analyzing rank tracking data and trends.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class RankAnalysisService
{
    private const CACHE_TTL = 3600; // 1 hour

    /**
     * Analyze ranking trends for a tracking.
     */
    public function analyzeTrends(RankTracking $tracking, int $days = 30): array
    {
        $cacheKey = "rank_trends:{$tracking->id}:{$days}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($tracking, $days) {
            $history = RankHistory::where('rank_tracking_id', $tracking->id)
                ->where('date', '>=', now()->subDays($days))
                ->orderBy('date')
                ->get();

            if ($history->isEmpty()) {
                return [
                    'trend' => 'no_data',
                    'average_position' => null,
                    'best_position' => null,
                    'worst_position' => null,
                    'volatility' => null,
                    'change' => 0,
                ];
            }

            $positions = $history->pluck('position')->filter()->values();
            
            if ($positions->isEmpty()) {
                return [
                    'trend' => 'not_ranking',
                    'average_position' => null,
                    'best_position' => null,
                    'worst_position' => null,
                    'volatility' => null,
                    'change' => 0,
                ];
            }

            $avgPosition = $positions->avg();
            $firstPosition = $positions->first();
            $lastPosition = $positions->last();
            $change = $firstPosition - $lastPosition; // Positive = improved

            // Calculate volatility (standard deviation)
            $variance = $positions->map(fn($p) => pow($p - $avgPosition, 2))->avg();
            $volatility = sqrt($variance);

            // Determine trend
            $trend = 'stable';
            if ($change > 5) {
                $trend = 'improving';
            } elseif ($change < -5) {
                $trend = 'declining';
            }

            return [
                'trend' => $trend,
                'average_position' => round($avgPosition, 1),
                'best_position' => $positions->min(),
                'worst_position' => $positions->max(),
                'volatility' => round($volatility, 2),
                'change' => $change,
                'data_points' => $positions->count(),
            ];
        });
    }

    /**
     * Get ranking velocity (rate of change).
     */
    public function getVelocity(RankTracking $tracking, int $days = 7): float
    {
        $history = RankHistory::where('rank_tracking_id', $tracking->id)
            ->where('date', '>=', now()->subDays($days))
            ->orderBy('date')
            ->get();

        if ($history->count() < 2) {
            return 0;
        }

        $positions = $history->pluck('position')->filter()->values();
        
        if ($positions->count() < 2) {
            return 0;
        }

        $firstPosition = $positions->first();
        $lastPosition = $positions->last();
        
        return ($firstPosition - $lastPosition) / $days;
    }

    /**
     * Predict future ranking based on trends.
     */
    public function predictPosition(RankTracking $tracking, int $daysAhead = 7): ?int
    {
        $velocity = $this->getVelocity($tracking);
        $currentPosition = $tracking->current_position;

        if ($currentPosition === null) {
            return null;
        }

        $predictedPosition = $currentPosition - ($velocity * $daysAhead);
        
        // Clamp to reasonable bounds
        return max(1, min(100, (int) round($predictedPosition)));
    }

    /**
     * Calculate ranking health score.
     */
    public function calculateHealthScore(RankTracking $tracking): int
    {
        $score = 100;
        $trends = $this->analyzeTrends($tracking);

        // Position factor (lower is better)
        $position = $tracking->current_position ?? 100;
        if ($position > 10) $score -= 10;
        if ($position > 20) $score -= 10;
        if ($position > 50) $score -= 20;

        // Trend factor
        if ($trends['trend'] === 'declining') $score -= 20;
        if ($trends['trend'] === 'improving') $score += 10;

        // Volatility factor
        if (($trends['volatility'] ?? 0) > 10) $score -= 10;

        return max(0, min(100, $score));
    }
}
