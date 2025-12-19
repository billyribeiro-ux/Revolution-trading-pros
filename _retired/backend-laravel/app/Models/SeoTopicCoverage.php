<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class SeoTopicCoverage extends Model
{
    protected $table = 'seo_topic_coverage';

    protected $fillable = [
        'topic_id',
        'content_type',
        'content_id',
        'relevance_score',
        'keyword_count',
        'keywords_found',
    ];

    protected $casts = [
        'relevance_score' => 'decimal:4',
        'keywords_found' => 'array',
    ];

    /**
     * Get the topic.
     */
    public function topic(): BelongsTo
    {
        return $this->belongsTo(SeoTopic::class, 'topic_id');
    }

    /**
     * Get the content (polymorphic).
     */
    public function content()
    {
        return $this->morphTo('content', 'content_type', 'content_id');
    }

    /**
     * Scope: For specific content.
     */
    public function scopeForContent($query, string $type, int $id)
    {
        return $query->where('content_type', $type)
                     ->where('content_id', $id);
    }

    /**
     * Scope: High relevance.
     */
    public function scopeHighRelevance($query, float $threshold = 0.5)
    {
        return $query->where('relevance_score', '>=', $threshold);
    }
}
