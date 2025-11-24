<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class SeoTopic extends Model
{
    protected $fillable = [
        'topic_name',
        'description',
        'keywords',
        'content_count',
        'avg_relevance',
    ];

    protected $casts = [
        'keywords' => 'array',
        'avg_relevance' => 'decimal:4',
    ];

    /**
     * Get topic coverage instances.
     */
    public function coverage(): HasMany
    {
        return $this->hasMany(SeoTopicCoverage::class, 'topic_id');
    }

    /**
     * Get keywords associated with this topic.
     */
    public function relatedKeywords(): HasMany
    {
        return $this->hasMany(SeoKeyword::class, 'parent_topic_id');
    }

    /**
     * Increment content count and update average relevance.
     */
    public function recordCoverage(float $relevance): void
    {
        $this->increment('content_count');
        
        // Update running average
        $this->avg_relevance = (
            ($this->avg_relevance * ($this->content_count - 1)) + $relevance
        ) / $this->content_count;
        
        $this->save();
    }

    /**
     * Scope: Most covered topics.
     */
    public function scopeMostCovered($query, int $limit = 10)
    {
        return $query->orderByDesc('content_count')->limit($limit);
    }

    /**
     * Scope: Highest relevance.
     */
    public function scopeHighestRelevance($query, int $limit = 10)
    {
        return $query->orderByDesc('avg_relevance')->limit($limit);
    }
}
