<?php

declare(strict_types=1);

namespace App\Services\Post\Strategies;

use App\Contracts\RelatedPostStrategyInterface;
use App\Models\Post;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\DB;

class CategoryStrategy implements RelatedPostStrategyInterface
{
    public function getIdentifier(): string
    {
        return 'category';
    }

    public function applyToQuery(Builder $query, Post $sourcePost): Builder
    {
        if (empty($sourcePost->categories)) {
            return $query;
        }

        return $query->whereExists(function ($subquery) use ($sourcePost) {
            $subquery->select(DB::raw(1))
                ->from('posts as p2')
                ->whereColumn('p2.id', 'posts.id');

            foreach ($sourcePost->categories as $category) {
                $subquery->orWhereJsonContains('p2.categories', $category);
            }
        });
    }

    public function calculateRelevanceScore(Post $sourcePost, Post $candidatePost): float
    {
        if (empty($sourcePost->categories) || empty($candidatePost->categories)) {
            return 0.0;
        }

        $sourceCategories = collect($sourcePost->categories);
        $candidateCategories = collect($candidatePost->categories);

        $intersection = $sourceCategories->intersect($candidateCategories)->count();
        $union = $sourceCategories->union($candidateCategories)->unique()->count();

        if ($union === 0) {
            return 0.0;
        }

        return $intersection / $union;
    }

    public function getRelationReason(Post $sourcePost, Post $candidatePost): string
    {
        $sharedCategories = collect($sourcePost->categories ?? [])
            ->intersect($candidatePost->categories ?? [])
            ->values();

        if ($sharedCategories->isEmpty()) {
            return 'Related content';
        }

        if ($sharedCategories->count() === 1) {
            return sprintf('Same category: %s', $sharedCategories->first());
        }

        return sprintf(
            'Shares %d categories: %s',
            $sharedCategories->count(),
            $sharedCategories->take(2)->implode(', ')
        );
    }

    public function getPriority(): int
    {
        return 100;
    }
}
