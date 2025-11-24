<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class SeoEntityMention extends Model
{
    protected $fillable = [
        'entity_id',
        'content_type',
        'content_id',
        'salience',
        'mention_count',
        'first_mention_position',
        'context',
        'sentiment',
    ];

    protected $casts = [
        'salience' => 'decimal:4',
    ];

    /**
     * Get the entity this mention belongs to.
     */
    public function entity(): BelongsTo
    {
        return $this->belongsTo(SeoEntity::class, 'entity_id');
    }

    /**
     * Get the content this mention is in (polymorphic).
     */
    public function content()
    {
        return $this->morphTo('content', 'content_type', 'content_id');
    }

    /**
     * Scope: Get mentions for specific content.
     */
    public function scopeForContent($query, string $type, int $id)
    {
        return $query->where('content_type', $type)
                     ->where('content_id', $id);
    }

    /**
     * Scope: Get high-salience mentions.
     */
    public function scopeHighSalience($query, float $threshold = 0.5)
    {
        return $query->where('salience', '>=', $threshold);
    }
}
