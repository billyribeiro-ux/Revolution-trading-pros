<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class SeoEntity extends Model
{
    protected $fillable = [
        'name',
        'type',
        'salience_avg',
        'mention_count',
        'wikipedia_url',
        'knowledge_graph_id',
        'metadata',
    ];

    protected $casts = [
        'salience_avg' => 'decimal:4',
        'metadata' => 'array',
    ];

    /**
     * Get all mentions of this entity.
     */
    public function mentions(): HasMany
    {
        return $this->hasMany(SeoEntityMention::class, 'entity_id');
    }

    /**
     * Increment mention count and update average salience.
     */
    public function recordMention(float $salience): void
    {
        $this->increment('mention_count');
        
        // Update running average
        $this->salience_avg = (
            ($this->salience_avg * ($this->mention_count - 1)) + $salience
        ) / $this->mention_count;
        
        $this->save();
    }

    /**
     * Scope: Get entities by type.
     */
    public function scopeOfType($query, string $type)
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Get most salient entities.
     */
    public function scopeMostSalient($query, int $limit = 10)
    {
        return $query->orderByDesc('salience_avg')->limit($limit);
    }

    /**
     * Scope: Get most mentioned entities.
     */
    public function scopeMostMentioned($query, int $limit = 10)
    {
        return $query->orderByDesc('mention_count')->limit($limit);
    }
}
