<?php

namespace App\Services;

use App\Models\Deal;
use App\Models\Pipeline;
use Illuminate\Support\Collection;

class DealForecastingService
{
    public function getForecast(string $period = 'this_month'): array
    {
        $deals = $this->getDealsForPeriod($period);
        
        return [
            'period' => $period,
            'commit' => $this->calculateCommit($deals),
            'best_case' => $this->calculateBestCase($deals),
            'pipeline' => $this->calculatePipeline($deals),
            'worst_case' => $this->calculateWorstCase($deals),
            'deals_count' => $deals->count(),
        ];
    }

    private function getDealsForPeriod(string $period): Collection
    {
        $query = Deal::where('status', 'open');

        switch ($period) {
            case 'this_month':
                $query->whereMonth('expected_close_date', now()->month)
                      ->whereYear('expected_close_date', now()->year);
                break;
            case 'next_month':
                $query->whereMonth('expected_close_date', now()->addMonth()->month)
                      ->whereYear('expected_close_date', now()->addMonth()->year);
                break;
            case 'this_quarter':
                $query->whereBetween('expected_close_date', [
                    now()->startOfQuarter(),
                    now()->endOfQuarter()
                ]);
                break;
            case 'next_quarter':
                $query->whereBetween('expected_close_date', [
                    now()->addQuarter()->startOfQuarter(),
                    now()->addQuarter()->endOfQuarter()
                ]);
                break;
        }

        return $query->with(['contact', 'stage', 'pipeline'])->get();
    }

    private function calculateCommit(Collection $deals): float
    {
        return $deals->filter(function ($deal) {
            return $this->calculateDealProbability($deal) >= 80;
        })->sum(function ($deal) {
            return $this->calculateWeightedValue($deal);
        });
    }

    private function calculateBestCase(Collection $deals): float
    {
        return $deals->filter(function ($deal) {
            return $this->calculateDealProbability($deal) >= 60;
        })->sum(function ($deal) {
            return $this->calculateWeightedValue($deal);
        });
    }

    private function calculatePipeline(Collection $deals): float
    {
        return $deals->sum(function ($deal) {
            return $this->calculateWeightedValue($deal);
        });
    }

    private function calculateWorstCase(Collection $deals): float
    {
        return $deals->filter(function ($deal) {
            return $this->calculateDealProbability($deal) >= 40;
        })->sum(function ($deal) {
            return $this->calculateWeightedValue($deal);
        });
    }

    private function calculateWeightedValue(Deal $deal): float
    {
        $probability = $this->calculateDealProbability($deal);
        return $deal->amount * ($probability / 100);
    }

    private function calculateDealProbability(Deal $deal): int
    {
        $stageProbability = $deal->stage->probability * 0.40;
        $historicalWinRate = $this->getHistoricalWinRate($deal) * 0.30;
        $dealAgeFactor = $this->getDealAgeFactor($deal) * 0.15;
        $ownerPerformance = $this->getOwnerPerformance($deal) * 0.10;
        $contactScoreFactor = $this->getContactScoreFactor($deal) * 0.05;

        return min(100, (int) round(
            $stageProbability + $historicalWinRate + $dealAgeFactor + 
            $ownerPerformance + $contactScoreFactor
        ));
    }

    private function getHistoricalWinRate(Deal $deal): int
    {
        $closedDeals = Deal::where('pipeline_id', $deal->pipeline_id)
            ->whereIn('status', ['won', 'lost'])
            ->where('created_at', '>=', now()->subDays(90))
            ->get();

        if ($closedDeals->count() < 10) {
            return $deal->stage->probability;
        }

        $wonCount = $closedDeals->where('status', 'won')->count();
        return (int) round(($wonCount / $closedDeals->count()) * 100);
    }

    private function getDealAgeFactor(Deal $deal): int
    {
        $daysInPipeline = $deal->created_at->diffInDays(now());

        if ($daysInPipeline < 30) {
            return 80; // Too new
        } elseif ($daysInPipeline <= 60) {
            return 100; // Optimal
        } elseif ($daysInPipeline <= 90) {
            return 90;
        } elseif ($daysInPipeline <= 120) {
            return 70;
        } else {
            return 50; // Stale
        }
    }

    private function getOwnerPerformance(Deal $deal): int
    {
        $ownerDeals = Deal::where('owner_id', $deal->owner_id)
            ->whereIn('status', ['won', 'lost'])
            ->where('created_at', '>=', now()->subDays(90))
            ->get();

        if ($ownerDeals->count() < 5) {
            return 100;
        }

        $wonCount = $ownerDeals->where('status', 'won')->count();
        return (int) round(($wonCount / $ownerDeals->count()) * 100);
    }

    private function getContactScoreFactor(Deal $deal): int
    {
        $leadScore = $deal->contact->lead_score;

        if ($leadScore >= 75) {
            return 120; // 1.2x multiplier
        } elseif ($leadScore >= 50) {
            return 100; // 1.0x
        } else {
            return 80; // 0.8x
        }
    }
}
