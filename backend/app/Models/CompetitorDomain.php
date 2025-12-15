<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

/**
 * Competitor Domain Model
 * 
 * @property int $id
 * @property string $domain
 * @property string|null $name
 * @property int|null $estimated_authority
 * @property int|null $estimated_traffic
 * @property array|null $tracked_keywords
 * @property array|null $metadata
 * @property bool $is_active
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class CompetitorDomain extends Model
{
    use HasFactory;

    protected $fillable = [
        'domain',
        'name',
        'estimated_authority',
        'estimated_traffic',
        'tracked_keywords',
        'metadata',
        'is_active',
    ];

    protected $casts = [
        'tracked_keywords' => 'array',
        'metadata' => 'array',
        'is_active' => 'boolean',
        'estimated_authority' => 'integer',
        'estimated_traffic' => 'integer',
    ];

    /**
     * Scope to active competitors.
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    /**
     * Get the display name.
     */
    public function getDisplayNameAttribute(): string
    {
        return $this->name ?? $this->domain;
    }
}
