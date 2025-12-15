<?php

namespace App\Events;

use App\Models\RankTracking;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Event fired when a keyword ranking drops significantly.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class RankingDropped
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    /**
     * Create a new event instance.
     */
    public function __construct(
        public RankTracking $tracking,
        public int $previousPosition,
        public int $currentPosition,
        public string $severity = 'warning'
    ) {}

    /**
     * Get the drop magnitude.
     */
    public function getDropMagnitude(): int
    {
        return $this->currentPosition - $this->previousPosition;
    }

    /**
     * Check if the ranking fell out of top N.
     */
    public function fellOutOfTop(int $n): bool
    {
        return $this->previousPosition <= $n && $this->currentPosition > $n;
    }

    /**
     * Check if this is a critical drop.
     */
    public function isCritical(): bool
    {
        return $this->severity === 'critical' || $this->getDropMagnitude() >= 10;
    }

    /**
     * Get alert data for notifications.
     */
    public function getAlertData(): array
    {
        return [
            'tracking_id' => $this->tracking->id,
            'keyword' => $this->tracking->keyword,
            'url' => $this->tracking->url,
            'previous_position' => $this->previousPosition,
            'current_position' => $this->currentPosition,
            'drop' => $this->getDropMagnitude(),
            'severity' => $this->severity,
            'fell_out_of_top_10' => $this->fellOutOfTop(10),
        ];
    }
}
