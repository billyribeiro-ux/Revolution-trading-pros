<?php

namespace App\Services\RankTracking;

use App\Models\RankTracking;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Notification;

/**
 * Service for managing rank tracking alerts.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class RankAlertService
{
    /**
     * Alert thresholds configuration.
     */
    private array $defaultThresholds = [
        'drop_warning' => 5,      // Warn if drops 5+ positions
        'drop_critical' => 10,    // Critical if drops 10+ positions
        'out_of_top_10' => 10,    // Alert if falls out of top 10
        'out_of_top_50' => 50,    // Alert if falls out of top 50
    ];

    /**
     * Create an alert for significant ranking drop.
     */
    public function createDropAlert(RankTracking $tracking, ?int $previousPosition, int $currentPosition): void
    {
        if ($previousPosition === null) {
            return;
        }

        $drop = $currentPosition - $previousPosition;
        
        if ($drop <= 0) {
            return; // Position improved, no alert needed
        }

        $severity = $this->determineSeverity($drop, $previousPosition, $currentPosition);

        Log::warning('Ranking drop detected', [
            'tracking_id' => $tracking->id,
            'keyword' => $tracking->keyword,
            'url' => $tracking->url,
            'previous_position' => $previousPosition,
            'current_position' => $currentPosition,
            'drop' => $drop,
            'severity' => $severity,
        ]);

        // Store alert in tracking metadata or separate alerts table
        $this->storeAlert($tracking, [
            'type' => 'ranking_drop',
            'severity' => $severity,
            'previous_position' => $previousPosition,
            'current_position' => $currentPosition,
            'drop' => $drop,
            'created_at' => now()->toISOString(),
        ]);
    }

    /**
     * Create an alert for entering/exiting top positions.
     */
    public function createPositionThresholdAlert(
        RankTracking $tracking,
        ?int $previousPosition,
        int $currentPosition,
        int $threshold
    ): void {
        $enteredTop = ($previousPosition === null || $previousPosition > $threshold) && $currentPosition <= $threshold;
        $exitedTop = $previousPosition !== null && $previousPosition <= $threshold && $currentPosition > $threshold;

        if ($enteredTop) {
            Log::info('Entered top positions', [
                'tracking_id' => $tracking->id,
                'keyword' => $tracking->keyword,
                'threshold' => $threshold,
                'position' => $currentPosition,
            ]);

            $this->storeAlert($tracking, [
                'type' => 'entered_top',
                'severity' => 'info',
                'threshold' => $threshold,
                'position' => $currentPosition,
                'created_at' => now()->toISOString(),
            ]);
        }

        if ($exitedTop) {
            Log::warning('Exited top positions', [
                'tracking_id' => $tracking->id,
                'keyword' => $tracking->keyword,
                'threshold' => $threshold,
                'previous_position' => $previousPosition,
                'current_position' => $currentPosition,
            ]);

            $this->storeAlert($tracking, [
                'type' => 'exited_top',
                'severity' => 'warning',
                'threshold' => $threshold,
                'previous_position' => $previousPosition,
                'current_position' => $currentPosition,
                'created_at' => now()->toISOString(),
            ]);
        }
    }

    /**
     * Determine alert severity based on drop magnitude.
     */
    private function determineSeverity(int $drop, int $previousPosition, int $currentPosition): string
    {
        // Critical if dropped out of first page
        if ($previousPosition <= 10 && $currentPosition > 10) {
            return 'critical';
        }

        // Critical for large drops
        if ($drop >= $this->defaultThresholds['drop_critical']) {
            return 'critical';
        }

        // Warning for moderate drops
        if ($drop >= $this->defaultThresholds['drop_warning']) {
            return 'warning';
        }

        return 'info';
    }

    /**
     * Store alert in tracking metadata.
     */
    private function storeAlert(RankTracking $tracking, array $alert): void
    {
        $alerts = $tracking->alerts ?? [];
        $alerts[] = $alert;

        // Keep only last 50 alerts
        if (count($alerts) > 50) {
            $alerts = array_slice($alerts, -50);
        }

        $tracking->update(['alerts' => $alerts]);
    }

    /**
     * Get active alerts for a tracking.
     */
    public function getAlerts(RankTracking $tracking, int $limit = 10): array
    {
        $alerts = $tracking->alerts ?? [];
        return array_slice(array_reverse($alerts), 0, $limit);
    }

    /**
     * Clear alerts for a tracking.
     */
    public function clearAlerts(RankTracking $tracking): void
    {
        $tracking->update(['alerts' => []]);
    }
}
