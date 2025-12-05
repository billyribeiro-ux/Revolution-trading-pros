<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * HasAnalytics Trait
 *
 * Provide analytics functionality for models.
 */
trait HasAnalytics
{
    /**
     * Get analytics data.
     */
    public function getAnalytics(string $period = '30d'): array
    {
        return [
            'period' => $period,
            'views' => $this->views ?? 0,
            'conversions' => $this->conversions ?? 0,
        ];
    }

    /**
     * Record analytics event.
     */
    public function recordAnalyticsEvent(string $event, array $data = []): void
    {
        // Analytics implementation
    }

    /**
     * Get conversion rate.
     */
    public function getConversionRate(): float
    {
        $views = $this->views ?? 0;
        $conversions = $this->conversions ?? 0;

        return $views > 0 ? ($conversions / $views) * 100 : 0;
    }
}
