<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class SeoAiSuggestion extends Model
{
    protected $fillable = [
        'content_type',
        'content_id',
        'suggestion_type',
        'original_text',
        'suggested_text',
        'reasoning',
        'impact_score',
        'confidence_score',
        'status',
        'applied_at',
        'metadata',
    ];

    protected $casts = [
        'metadata' => 'array',
        'applied_at' => 'datetime',
    ];

    /**
     * Get the content (polymorphic).
     */
    public function content()
    {
        return $this->morphTo('content', 'content_type', 'content_id');
    }

    /**
     * Mark suggestion as accepted.
     */
    public function accept(): void
    {
        $this->update(['status' => 'accepted']);
    }

    /**
     * Mark suggestion as rejected.
     */
    public function reject(): void
    {
        $this->update(['status' => 'rejected']);
    }

    /**
     * Mark suggestion as applied.
     */
    public function apply(): void
    {
        $this->update([
            'status' => 'applied',
            'applied_at' => now(),
        ]);
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
     * Scope: By type.
     */
    public function scopeOfType($query, string $type)
    {
        return $query->where('suggestion_type', $type);
    }

    /**
     * Scope: Pending suggestions.
     */
    public function scopePending($query)
    {
        return $query->where('status', 'pending');
    }

    /**
     * Scope: High impact.
     */
    public function scopeHighImpact($query, int $threshold = 70)
    {
        return $query->where('impact_score', '>=', $threshold);
    }

    /**
     * Scope: High confidence.
     */
    public function scopeHighConfidence($query, int $threshold = 80)
    {
        return $query->where('confidence_score', '>=', $threshold);
    }

    /**
     * Scope: Best suggestions (high impact + high confidence).
     */
    public function scopeBest($query)
    {
        return $query->where('impact_score', '>=', 70)
                     ->where('confidence_score', '>=', 80)
                     ->orderByDesc('impact_score');
    }
}
