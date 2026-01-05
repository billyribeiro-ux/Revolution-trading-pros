<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class SeoKeyword extends Model
{
    protected $fillable = [
        'keyword',
        'search_volume',
        'difficulty_score',
        'opportunity_score',
        'intent',
        'cpc',
        'competition',
        'trend_direction',
        'parent_topic_id',
        'metadata',
        'last_updated',
    ];

    protected $casts = [
        'cpc' => 'decimal:2',
        'competition' => 'decimal:4',
        'metadata' => 'array',
        'last_updated' => 'datetime',
    ];

    /**
     * Get the parent topic.
     */
    public function parentTopic(): BelongsTo
    {
        return $this->belongsTo(SeoTopic::class, 'parent_topic_id');
    }

    /**
     * Get SERP results for this keyword.
     */
    public function serpResults(): HasMany
    {
        return $this->hasMany(SeoSerpResult::class, 'keyword_id');
    }

    /**
     * Get competitors for this keyword.
     */
    public function competitors(): HasMany
    {
        return $this->hasMany(SeoSerpCompetitor::class, 'keyword_id');
    }

    /**
     * Calculate opportunity score based on volume and difficulty.
     */
    public function calculateOpportunityScore(): int
    {
        if ($this->difficulty_score === 0) {
            return 0;
        }

        // Higher volume + lower difficulty = higher opportunity
        $volumeScore = min(100, ($this->search_volume / 1000) * 10);
        $difficultyPenalty = $this->difficulty_score;
        
        return (int) max(0, $volumeScore - ($difficultyPenalty * 0.5));
    }

    /**
     * Scope: High opportunity keywords.
     */
    public function scopeHighOpportunity($query, int $threshold = 50)
    {
        return $query->where('opportunity_score', '>=', $threshold);
    }

    /**
     * Scope: By intent.
     */
    public function scopeByIntent($query, string $intent)
    {
        return $query->where('intent', $intent);
    }

    /**
     * Scope: Trending up.
     */
    public function scopeTrendingUp($query)
    {
        return $query->where('trend_direction', 'up');
    }

    /**
     * Scope: Low competition.
     */
    public function scopeLowCompetition($query, float $threshold = 0.3)
    {
        return $query->where('competition', '<=', $threshold);
    }
}
