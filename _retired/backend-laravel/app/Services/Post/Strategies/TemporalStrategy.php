<?php

declare(strict_types=1);

namespace App\Services\Post\Strategies;

use App\Contracts\RelatedPostStrategyInterface;
use App\Models\Post;
use Illuminate\Database\Eloquent\Builder;

class TemporalStrategy implements RelatedPostStrategyInterface
{
    private int $decayDays;

    public function __construct(?int $decayDays = null)
    {
        $this->decayDays = $decayDays ?? config('posts.related_posts.scoring.temporal_decay_days', 90);
    }

    public function getIdentifier(): string
    {
        return 'temporal';
    }

    public function applyToQuery(Builder $query, Post $sourcePost): Builder
    {
        return $query->orderByDesc('published_at');
    }

    public function calculateRelevanceScore(Post $sourcePost, Post $candidatePost): float
    {
        if (!$candidatePost->published_at) {
            return 0.0;
        }

        $daysDifference = abs($sourcePost->published_at->diffInDays($candidatePost->published_at));

        $score = exp(-$daysDifference / $this->decayDays);

        if ($daysDifference <= 7) {
            $score *= 1.3;
        }

        return min($score, 1.0);
    }

    public function getRelationReason(Post $sourcePost, Post $candidatePost): string
    {
        if (!$candidatePost->published_at) {
            return 'Recent content';
        }

        $daysDifference = abs($sourcePost->published_at->diffInDays($candidatePost->published_at));

        if ($daysDifference === 0) {
            return 'Published the same day';
        }

        if ($daysDifference <= 7) {
            return 'Published around the same time';
        }

        if ($daysDifference <= 30) {
            return sprintf('Published %d days apart', $daysDifference);
        }

        return 'From the same period';
    }

    public function getPriority(): int
    {
        return 30;
    }
}
