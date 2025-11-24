<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsAttributionTouchpoint;
use App\Models\Analytics\AnalyticsAttributionConversion;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * AttributionEngine - Multi-Touch Attribution Analysis
 *
 * Implements multiple attribution models for marketing channel analysis.
 *
 * @package App\Services\Analytics
 */
class AttributionEngine
{
    /**
     * Default lookback window in days
     */
    private const LOOKBACK_WINDOW_DAYS = 30;

    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Record a touchpoint from an event
     */
    public function recordTouchpoint(AnalyticsEvent $event): ?AnalyticsAttributionTouchpoint
    {
        // Only record touchpoints for marketing-relevant events
        if (!$event->channel || $event->channel === AnalyticsEvent::CHANNEL_DIRECT) {
            return null;
        }

        // Get or create conversion ID for this user journey
        $conversionId = $this->getOrCreateConversionId($event);

        // Count existing touchpoints
        $touchpointCount = AnalyticsAttributionTouchpoint::where('conversion_id', $conversionId)->count();

        return AnalyticsAttributionTouchpoint::create([
            'user_id' => $event->user_id,
            'anonymous_id' => $event->anonymous_id,
            'conversion_id' => $conversionId,
            'channel' => $event->channel,
            'source' => $event->utm_source,
            'medium' => $event->utm_medium,
            'campaign' => $event->utm_campaign,
            'content' => $event->utm_content,
            'term' => $event->utm_term,
            'touchpoint_at' => $event->event_timestamp,
            'touchpoint_number' => $touchpointCount + 1,
            'is_first_touch' => $touchpointCount === 0,
            'event_id' => $event->id,
        ]);
    }

    /**
     * Record a conversion and calculate attribution credits
     */
    public function recordConversion(AnalyticsEvent $event): ?AnalyticsAttributionConversion
    {
        if (!$event->is_conversion) {
            return null;
        }

        $conversionId = $this->getOrCreateConversionId($event);

        // Get all touchpoints for this conversion
        $touchpoints = AnalyticsAttributionTouchpoint::where('conversion_id', $conversionId)
            ->orderBy('touchpoint_at')
            ->get();

        if ($touchpoints->isEmpty()) {
            return null;
        }

        // Mark first and last touch
        $touchpoints->first()->update(['is_first_touch' => true]);
        $touchpoints->last()->update(['is_last_touch' => true, 'is_converting_touch' => true]);

        // Calculate attribution credits for each model
        $this->calculateAttributionCredits($touchpoints, $event->revenue ?? 0);

        // Create conversion record
        return AnalyticsAttributionConversion::create([
            'conversion_id' => $conversionId,
            'user_id' => $event->user_id,
            'anonymous_id' => $event->anonymous_id,
            'conversion_type' => $event->event_type,
            'conversion_event' => $event->event_name,
            'converted_at' => $event->event_timestamp,
            'conversion_value' => $event->revenue ?? 0,
            'currency' => $event->currency,
            'touchpoint_count' => $touchpoints->count(),
            'days_to_convert' => $touchpoints->first()->touchpoint_at->diffInDays($event->event_timestamp),
            'first_touch_channel' => $touchpoints->first()->channel,
            'last_touch_channel' => $touchpoints->last()->channel,
            'entity_type' => $event->entity_type,
            'entity_id' => $event->entity_id,
        ]);
    }

    /**
     * Calculate attribution credits for all models
     */
    protected function calculateAttributionCredits(Collection $touchpoints, float $conversionValue): void
    {
        $count = $touchpoints->count();

        if ($count === 0) {
            return;
        }

        // First Touch - 100% to first touchpoint
        // Last Touch - 100% to last touchpoint
        // Linear - Equal credit to all touchpoints
        // Time Decay - More credit to recent touchpoints
        // Position Based - 40% first, 40% last, 20% middle

        $linearCredit = 1 / $count;

        // Time decay half-life: 7 days
        $halfLife = 7;
        $lastTouchTime = $touchpoints->last()->touchpoint_at;
        $decayWeights = [];
        $totalDecayWeight = 0;

        foreach ($touchpoints as $touchpoint) {
            $daysDiff = $touchpoint->touchpoint_at->diffInDays($lastTouchTime);
            $weight = pow(2, -$daysDiff / $halfLife);
            $decayWeights[$touchpoint->id] = $weight;
            $totalDecayWeight += $weight;
        }

        foreach ($touchpoints as $index => $touchpoint) {
            $isFirst = $index === 0;
            $isLast = $index === $count - 1;

            // First touch
            $firstTouchCredit = $isFirst ? 1.0 : 0.0;

            // Last touch
            $lastTouchCredit = $isLast ? 1.0 : 0.0;

            // Linear
            $linearCreditValue = $linearCredit;

            // Time decay
            $timeDecayCredit = $totalDecayWeight > 0
                ? $decayWeights[$touchpoint->id] / $totalDecayWeight
                : $linearCredit;

            // Position based
            if ($count === 1) {
                $positionCredit = 1.0;
            } elseif ($count === 2) {
                $positionCredit = 0.5;
            } else {
                if ($isFirst || $isLast) {
                    $positionCredit = 0.4;
                } else {
                    $positionCredit = 0.2 / ($count - 2);
                }
            }

            $touchpoint->update([
                'first_touch_credit' => round($firstTouchCredit, 4),
                'last_touch_credit' => round($lastTouchCredit, 4),
                'linear_credit' => round($linearCreditValue, 4),
                'time_decay_credit' => round($timeDecayCredit, 4),
                'position_credit' => round($positionCredit, 4),
            ]);
        }
    }

    /**
     * Get or create conversion ID for user journey
     */
    protected function getOrCreateConversionId(AnalyticsEvent $event): string
    {
        $identifier = $event->user_id ?? $event->anonymous_id;

        // Look for existing unconverted journey
        $existingTouchpoint = AnalyticsAttributionTouchpoint::query()
            ->where(function ($query) use ($event) {
                $query->where('user_id', $event->user_id)
                    ->orWhere('anonymous_id', $event->anonymous_id);
            })
            ->where('touchpoint_at', '>=', now()->subDays(self::LOOKBACK_WINDOW_DAYS))
            ->whereDoesntHave('conversion')
            ->orderByDesc('touchpoint_at')
            ->first();

        return $existingTouchpoint?->conversion_id ?? Str::uuid()->toString();
    }

    /**
     * Get channel attribution report
     */
    public function getChannelAttribution(
        string $model,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $cacheKey = "attribution:channel:{$model}:{$startDate->format('Ymd')}:{$endDate->format('Ymd')}";

        return $this->cacheManager->remember(
            $cacheKey,
            fn () => $this->computeChannelAttribution($model, $startDate, $endDate),
            1800
        );
    }

    /**
     * Compute channel attribution
     */
    protected function computeChannelAttribution(
        string $model,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $creditField = match ($model) {
            'first_touch' => 'first_touch_credit',
            'last_touch' => 'last_touch_credit',
            'linear' => 'linear_credit',
            'time_decay' => 'time_decay_credit',
            'position_based' => 'position_credit',
            default => 'linear_credit',
        };

        // Get conversions in date range
        $conversions = AnalyticsAttributionConversion::query()
            ->whereBetween('converted_at', [$startDate, $endDate])
            ->pluck('conversion_id');

        // Get touchpoints with credits
        $attribution = AnalyticsAttributionTouchpoint::query()
            ->whereIn('conversion_id', $conversions)
            ->join('analytics_attribution_conversions', 'analytics_attribution_touchpoints.conversion_id', '=', 'analytics_attribution_conversions.conversion_id')
            ->select(
                'analytics_attribution_touchpoints.channel',
                DB::raw("SUM({$creditField}) as attributed_conversions"),
                DB::raw("SUM({$creditField} * analytics_attribution_conversions.conversion_value) as attributed_revenue"),
                DB::raw('COUNT(DISTINCT analytics_attribution_touchpoints.conversion_id) as assisted_conversions'),
                DB::raw('COUNT(*) as touchpoints')
            )
            ->groupBy('analytics_attribution_touchpoints.channel')
            ->orderByDesc('attributed_conversions')
            ->get();

        $totalConversions = $attribution->sum('attributed_conversions');
        $totalRevenue = $attribution->sum('attributed_revenue');

        return [
            'model' => $model,
            'period' => [
                'start' => $startDate->format('Y-m-d'),
                'end' => $endDate->format('Y-m-d'),
            ],
            'summary' => [
                'total_conversions' => round($totalConversions, 2),
                'total_revenue' => round($totalRevenue, 2),
            ],
            'channels' => $attribution->map(fn ($row) => [
                'channel' => $row->channel,
                'attributed_conversions' => round($row->attributed_conversions, 2),
                'attributed_revenue' => round($row->attributed_revenue, 2),
                'assisted_conversions' => $row->assisted_conversions,
                'touchpoints' => $row->touchpoints,
                'conversion_share' => $totalConversions > 0
                    ? round(($row->attributed_conversions / $totalConversions) * 100, 2)
                    : 0,
                'revenue_share' => $totalRevenue > 0
                    ? round(($row->attributed_revenue / $totalRevenue) * 100, 2)
                    : 0,
            ])->toArray(),
        ];
    }

    /**
     * Get campaign attribution report
     */
    public function getCampaignAttribution(
        string $model,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $creditField = match ($model) {
            'first_touch' => 'first_touch_credit',
            'last_touch' => 'last_touch_credit',
            'linear' => 'linear_credit',
            'time_decay' => 'time_decay_credit',
            'position_based' => 'position_credit',
            default => 'linear_credit',
        };

        $conversions = AnalyticsAttributionConversion::query()
            ->whereBetween('converted_at', [$startDate, $endDate])
            ->pluck('conversion_id');

        return AnalyticsAttributionTouchpoint::query()
            ->whereIn('conversion_id', $conversions)
            ->whereNotNull('campaign')
            ->join('analytics_attribution_conversions', 'analytics_attribution_touchpoints.conversion_id', '=', 'analytics_attribution_conversions.conversion_id')
            ->select(
                'analytics_attribution_touchpoints.campaign',
                'analytics_attribution_touchpoints.source',
                DB::raw("SUM({$creditField}) as attributed_conversions"),
                DB::raw("SUM({$creditField} * analytics_attribution_conversions.conversion_value) as attributed_revenue")
            )
            ->groupBy('analytics_attribution_touchpoints.campaign', 'analytics_attribution_touchpoints.source')
            ->orderByDesc('attributed_conversions')
            ->limit(50)
            ->get()
            ->toArray();
    }

    /**
     * Get conversion path analysis
     */
    public function getConversionPaths(Carbon $startDate, Carbon $endDate, int $limit = 20): array
    {
        $conversions = AnalyticsAttributionConversion::query()
            ->whereBetween('converted_at', [$startDate, $endDate])
            ->pluck('conversion_id');

        // Get paths
        $paths = [];

        foreach ($conversions as $conversionId) {
            $touchpoints = AnalyticsAttributionTouchpoint::where('conversion_id', $conversionId)
                ->orderBy('touchpoint_number')
                ->pluck('channel')
                ->implode(' → ');

            if (!isset($paths[$touchpoints])) {
                $paths[$touchpoints] = [
                    'path' => $touchpoints,
                    'conversions' => 0,
                    'revenue' => 0,
                ];
            }

            $conversion = AnalyticsAttributionConversion::where('conversion_id', $conversionId)->first();
            $paths[$touchpoints]['conversions']++;
            $paths[$touchpoints]['revenue'] += $conversion->conversion_value ?? 0;
        }

        // Sort by conversions and limit
        usort($paths, fn ($a, $b) => $b['conversions'] <=> $a['conversions']);

        return array_slice(array_values($paths), 0, $limit);
    }

    /**
     * Compare attribution models
     */
    public function compareModels(Carbon $startDate, Carbon $endDate): array
    {
        $models = ['first_touch', 'last_touch', 'linear', 'time_decay', 'position_based'];
        $comparison = [];

        foreach ($models as $model) {
            $attribution = $this->computeChannelAttribution($model, $startDate, $endDate);

            $comparison[$model] = [
                'model' => $model,
                'top_channel' => $attribution['channels'][0]['channel'] ?? null,
                'top_channel_share' => $attribution['channels'][0]['conversion_share'] ?? 0,
                'channels' => collect($attribution['channels'])->keyBy('channel')->toArray(),
            ];
        }

        return $comparison;
    }
}
