<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

/**
 * Search Engine Model
 * 
 * @property int $id
 * @property string $key
 * @property string $name
 * @property string|null $api_endpoint
 * @property bool $supports_local
 * @property int $max_results
 * @property bool $is_active
 * @property array|null $config
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class SearchEngine extends Model
{
    use HasFactory;

    protected $fillable = [
        'key',
        'name',
        'api_endpoint',
        'supports_local',
        'max_results',
        'is_active',
        'config',
    ];

    protected $casts = [
        'supports_local' => 'boolean',
        'is_active' => 'boolean',
        'max_results' => 'integer',
        'config' => 'array',
    ];

    /**
     * Search engine keys.
     */
    public const GOOGLE = 'google';
    public const BING = 'bing';
    public const YAHOO = 'yahoo';
    public const DUCKDUCKGO = 'duckduckgo';

    /**
     * Scope to active engines.
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    /**
     * Get engine by key.
     */
    public static function findByKey(string $key): ?self
    {
        return static::where('key', $key)->first();
    }
}
