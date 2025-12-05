<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * Trackable Trait
 *
 * Track views, clicks, and other metrics.
 */
trait Trackable
{
    /**
     * Increment view count.
     */
    public function incrementViews(): void
    {
        if (isset($this->views)) {
            $this->increment('views');
        }
    }

    /**
     * Increment click count.
     */
    public function incrementClicks(): void
    {
        if (isset($this->clicks)) {
            $this->increment('clicks');
        }
    }

    /**
     * Record a tracking event.
     */
    public function track(string $event, array $data = []): void
    {
        // Tracking implementation
    }

    /**
     * Get tracking stats.
     */
    public function getTrackingStats(): array
    {
        return [
            'views' => $this->views ?? 0,
            'clicks' => $this->clicks ?? 0,
        ];
    }
}
