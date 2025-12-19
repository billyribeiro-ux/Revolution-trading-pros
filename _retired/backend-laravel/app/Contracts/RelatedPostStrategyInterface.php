<?php

declare(strict_types=1);

namespace App\Contracts;

use App\Models\Post;
use Illuminate\Database\Eloquent\Builder;

interface RelatedPostStrategyInterface
{
    public function getIdentifier(): string;
    public function applyToQuery(Builder $query, Post $sourcePost): Builder;
    public function calculateRelevanceScore(Post $sourcePost, Post $candidatePost): float;
    public function getRelationReason(Post $sourcePost, Post $candidatePost): string;
    public function getPriority(): int;
}
