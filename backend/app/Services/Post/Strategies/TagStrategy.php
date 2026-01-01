<?php

declare(strict_types=1);

namespace App\Services\Post\Strategies;

use App\Contracts\RelatedPostStrategyInterface;
use App\Models\Post;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\DB;

class TagStrategy implements RelatedPostStrategyInterface
{
    public function getIdentifier(): string
    {
        return 'tags';
    }

    public function applyToQuery(Builder $query, Post $sourcePost): Builder
    {
        if (empty($sourcePost->tags)) {
            return $query;
        }

        return $query->whereExists(function ($subquery) use ($sourcePost) {
            $subquery->select(DB::raw(1))
                ->from('posts as p2')
                ->whereColumn('p2.id', 'posts.id');

            foreach ($sourcePost->tags as $tag) {
                $subquery->orWhereJsonContains('p2.tags', $tag);
            }
        });
    }

    public function calculateRelevanceScore(Post $sourcePost, Post $candidatePost): float
    {
        if (empty($sourcePost->tags) || empty($candidatePost->tags)) {
            return 0.0;
        }

        $sourceTags = collect($sourcePost->tags);
        $candidateTags = collect($candidatePost->tags);

        $intersection = $sourceTags->intersect($candidateTags)->count();
        $union = $sourceTags->union($candidateTags)->unique()->count();

        if ($union === 0) {
            return 0.0;
        }

        $jaccardSimilarity = $intersection / $union;

        if ($intersection >= 3) {
            $jaccardSimilarity *= 1.2;
        }

        return min($jaccardSimilarity, 1.0);
    }

    public function getRelationReason(Post $sourcePost, Post $candidatePost): string
    {
        $sharedTags = collect($sourcePost->tags ?? [])
            ->intersect($candidatePost->tags ?? [])
            ->values();

        if ($sharedTags->isEmpty()) {
            return 'Related content';
        }

        if ($sharedTags->count() === 1) {
            return sprintf('Tagged: %s', $sharedTags->first());
        }

        return sprintf(
            'Shares %d tags: %s',
            $sharedTags->count(),
            $sharedTags->take(3)->implode(', ')
        );
    }

    public function getPriority(): int
    {
        return 90;
    }
}
