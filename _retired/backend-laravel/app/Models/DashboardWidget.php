<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class DashboardWidget extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'dashboard_id',
        'widget_type',
        'position_x',
        'position_y',
        'width',
        'height',
        'config',
        'data_provider',
        'refresh_interval',
        'title',
        'is_visible',
    ];

    protected $casts = [
        'position_x' => 'integer',
        'position_y' => 'integer',
        'width' => 'integer',
        'height' => 'integer',
        'config' => 'array',
        'refresh_interval' => 'integer',
        'is_visible' => 'boolean',
    ];

    public function dashboard(): BelongsTo
    {
        return $this->belongsTo(Dashboard::class);
    }

    public function cacheEntries(): HasMany
    {
        return $this->hasMany(WidgetDataCache::class, 'widget_id');
    }
}
