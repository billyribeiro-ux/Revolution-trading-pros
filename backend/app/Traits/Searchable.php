<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Database\Eloquent\Builder;

/**
 * Searchable Trait
 *
 * Add search functionality to models.
 */
trait Searchable
{
    /**
     * Get searchable attributes.
     */
    public function getSearchableAttributes(): array
    {
        return $this->searchable ?? ['name', 'title', 'description'];
    }

    /**
     * Scope for searching.
     */
    public function scopeSearch(Builder $query, string $term): Builder
    {
        $searchable = $this->getSearchableAttributes();

        return $query->where(function ($q) use ($term, $searchable) {
            foreach ($searchable as $attribute) {
                $q->orWhere($attribute, 'LIKE', "%{$term}%");
            }
        });
    }
}
