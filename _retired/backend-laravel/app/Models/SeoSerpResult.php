<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class SeoSerpResult extends Model
{
    protected $fillable = [
        'keyword_id',
        'position',
        'url',
        'title',
        'description',
        'domain',
        'domain_authority',
        'page_authority',
        'word_count',
        'entities_found',
        'topics_covered',
        'schema_types',
        'analyzed_at',
        'serp_features',
    ];

    protected $casts = [
        'entities_found' => 'array',
        'topics_covered' => 'array',
        'schema_types' => 'array',
        'serp_features' => 'array',
        'analyzed_at' => 'datetime',
    ];

    /**
     * Get the keyword.
     */
    public function keyword(): BelongsTo
    {
        return $this->belongsTo(SeoKeyword::class, 'keyword_id');
    }

    /**
     * Scope: Top positions.
     */
    public function scopeTopPositions($query, int $limit = 10)
    {
        return $query->where('position', '<=', $limit)
                     ->orderBy('position');
    }

    /**
     * Scope: Has featured snippet.
     */
    public function scopeWithFeaturedSnippet($query)
    {
        return $query->whereJsonContains('serp_features', 'featured_snippet');
    }

    /**
     * Scope: Recent analysis.
     */
    public function scopeRecent($query, int $hours = 24)
    {
        return $query->where('analyzed_at', '>=', now()->subHours($hours));
    }
}
