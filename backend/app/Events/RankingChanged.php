<?php

namespace App\Events;

use App\Models\RankTracking;
use App\Enums\RankChangeType;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Event fired when a keyword ranking changes significantly.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class RankingChanged
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    /**
     * Create a new event instance.
     */
    public function __construct(
        public RankTracking $tracking,
        public ?int $previousPosition,
        public ?int $currentPosition,
        public RankChangeType $changeType
    ) {}

    /**
     * Get the change magnitude.
     */
    public function getChangeMagnitude(): int
    {
        if ($this->previousPosition === null || $this->currentPosition === null) {
            return 0;
        }

        return abs($this->currentPosition - $this->previousPosition);
    }

    /**
     * Check if this is a significant change.
     */
    public function isSignificant(int $threshold = 5): bool
    {
        return $this->getChangeMagnitude() >= $threshold;
    }

    /**
     * Check if the ranking improved.
     */
    public function isImproved(): bool
    {
        return $this->changeType === RankChangeType::IMPROVED;
    }

    /**
     * Check if the ranking dropped.
     */
    public function isDropped(): bool
    {
        return $this->changeType === RankChangeType::DROPPED;
    }
}
