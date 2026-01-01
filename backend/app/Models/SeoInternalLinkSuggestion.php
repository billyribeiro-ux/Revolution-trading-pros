<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class SeoInternalLinkSuggestion extends Model
{
    protected $fillable = [
        'source_content_type',
        'source_content_id',
        'target_content_type',
        'target_content_id',
        'suggested_anchor_text',
        'context_snippet',
        'relevance_score',
        'priority',
        'reasoning',
        'status',
        'applied_at',
    ];

    protected $casts = [
        'applied_at' => 'datetime',
    ];

    /**
     * Get the source content (polymorphic).
     */
    public function sourceContent()
    {
        return $this->morphTo('source_content', 'source_content_type', 'source_content_id');
    }

    /**
     * Get the target content (polymorphic).
     */
    public function targetContent()
    {
        return $this->morphTo('target_content', 'target_content_type', 'target_content_id');
    }

    /**
     * Mark as applied.
     */
    public function apply(): void
    {
        $this->update([
            'status' => 'applied',
            'applied_at' => now(),
        ]);
    }

    /**
     * Scope: For source content.
     */
    public function scopeForSource($query, string $type, int $id)
    {
        return $query->where('source_content_type', $type)
                     ->where('source_content_id', $id);
    }

    /**
     * Scope: Pending.
     */
    public function scopePending($query)
    {
        return $query->where('status', 'pending');
    }

    /**
     * Scope: High priority.
     */
    public function scopeHighPriority($query)
    {
        return $query->where('priority', 'high');
    }

    /**
     * Scope: High relevance.
     */
    public function scopeHighRelevance($query, int $threshold = 70)
    {
        return $query->where('relevance_score', '>=', $threshold);
    }
}
