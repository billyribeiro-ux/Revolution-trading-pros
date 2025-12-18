<?php

declare(strict_types=1);

namespace App\Services\Post\Strategies;

use App\Contracts\RelatedPostStrategyInterface;
use App\Models\Post;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;

class HybridStrategy implements RelatedPostStrategyInterface
{
    private Collection $strategies;
    private array $weights;

    public function __construct(array $strategies, ?array $weights = null)
    {
        $this->strategies = collect($strategies);
        $this->weights = $weights ?? config('posts.related_posts.hybrid_weights', [
            'category' => 0.40,
            'tags' => 0.30,
            'temporal' => 0.20,
            'semantic' => 0.10,
        ]);

        $this->normalizeWeights();
    }

    public function getIdentifier(): string
    {
        return 'hybrid';
    }

    public function applyToQuery(Builder $query, Post $sourcePost): Builder
    {
        $appliedStrategies = 0;

        foreach ($this->strategies as $strategy) {
            $weight = $this->weights[$strategy->getIdentifier()] ?? 0;
            
            if ($weight > 0) {
                $query = $strategy->applyToQuery($query, $sourcePost);
                $appliedStrategies++;
            }
        }

        if ($appliedStrategies === 0) {
            $query->orderByDesc('published_at');
        }

        return $query;
    }

    public function calculateRelevanceScore(Post $sourcePost, Post $candidatePost): float
    {
        $totalScore = 0.0;
        $totalWeight = 0.0;

        foreach ($this->strategies as $strategy) {
            $identifier = $strategy->getIdentifier();
            $weight = $this->weights[$identifier] ?? 0;

            if ($weight <= 0) {
                continue;
            }

            $strategyScore = $strategy->calculateRelevanceScore($sourcePost, $candidatePost);
            $totalScore += $strategyScore * $weight;
            $totalWeight += $weight;
        }

        if ($totalWeight === 0) {
            return 0.0;
        }

        $normalizedScore = $totalScore / $totalWeight;

        if ($sourcePost->author_id && 
            $candidatePost->author_id && 
            $sourcePost->author_id === $candidatePost->author_id) {
            $boost = config('posts.related_posts.scoring.same_author_boost', 1.15);
            $normalizedScore *= $boost;
        }

        return min($normalizedScore, 1.0);
    }

    public function getRelationReason(Post $sourcePost, Post $candidatePost): string
    {
        $bestStrategy = null;
        $bestScore = 0.0;

        foreach ($this->strategies as $strategy) {
            $weight = $this->weights[$strategy->getIdentifier()] ?? 0;
            if ($weight <= 0) {
                continue;
            }

            $score = $strategy->calculateRelevanceScore($sourcePost, $candidatePost);
            if ($score > $bestScore) {
                $bestScore = $score;
                $bestStrategy = $strategy;
            }
        }

        if ($bestStrategy) {
            return $bestStrategy->getRelationReason($sourcePost, $candidatePost);
        }

        return 'Related content';
    }

    public function getPriority(): int
    {
        return 100;
    }

    private function normalizeWeights(): void
    {
        $sum = array_sum($this->weights);

        if ($sum <= 0) {
            return;
        }

        foreach ($this->weights as $key => $weight) {
            $this->weights[$key] = $weight / $sum;
        }
    }

    public function getStrategyBreakdown(Post $sourcePost, Post $candidatePost): array
    {
        $breakdown = [];

        foreach ($this->strategies as $strategy) {
            $identifier = $strategy->getIdentifier();
            $weight = $this->weights[$identifier] ?? 0;

            if ($weight <= 0) {
                continue;
            }

            $breakdown[$identifier] = [
                'score' => $strategy->calculateRelevanceScore($sourcePost, $candidatePost),
                'weight' => $weight,
                'weighted_score' => $strategy->calculateRelevanceScore($sourcePost, $candidatePost) * $weight,
            ];
        }

        return $breakdown;
    }
}
