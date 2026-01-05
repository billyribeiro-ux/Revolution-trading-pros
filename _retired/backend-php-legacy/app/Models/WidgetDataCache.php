<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class WidgetDataCache extends Model
{
    use HasUuids;

    public $timestamps = false;

    protected $table = 'widget_data_cache';

    protected $fillable = [
        'widget_id',
        'cache_key',
        'data',
        'expires_at',
        'created_at',
    ];

    protected $casts = [
        'data' => 'array',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
    ];

    public function widget(): BelongsTo
    {
        return $this->belongsTo(DashboardWidget::class, 'widget_id');
    }

    public function isExpired(): bool
    {
        return $this->expires_at->isPast();
    }

    public function scopeValid($query)
    {
        return $query->where('expires_at', '>', now());
    }
}
