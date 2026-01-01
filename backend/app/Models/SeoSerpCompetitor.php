<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class SeoSerpCompetitor extends Model
{
    protected $fillable = [
        'keyword_id',
        'domain',
        'appearances',
        'avg_position',
        'domain_authority',
        'ranking_urls',
    ];

    protected $casts = [
        'avg_position' => 'decimal:2',
        'ranking_urls' => 'array',
    ];

    /**
     * Get the keyword.
     */
    public function keyword(): BelongsTo
    {
        return $this->belongsTo(SeoKeyword::class, 'keyword_id');
    }

    /**
     * Scope: Top competitors.
     */
    public function scopeTopCompetitors($query, int $limit = 10)
    {
        return $query->orderByDesc('appearances')
                     ->orderBy('avg_position')
                     ->limit($limit);
    }

    /**
     * Scope: High authority.
     */
    public function scopeHighAuthority($query, int $threshold = 50)
    {
        return $query->where('domain_authority', '>=', $threshold);
    }
}
