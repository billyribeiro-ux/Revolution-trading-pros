<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Stage extends Model
{
    use HasUuids;

    protected $fillable = [
        'pipeline_id', 'name', 'description', 'position', 'probability',
        'is_closed_won', 'is_closed_lost', 'auto_advance_after_days', 'required_activities',
        'deals_count', 'total_value', 'avg_time_in_stage', 'conversion_rate', 'color'
    ];

    protected $casts = [
        'is_closed_won' => 'boolean',
        'is_closed_lost' => 'boolean',
        'required_activities' => 'array',
        'total_value' => 'decimal:2',
        'conversion_rate' => 'decimal:2',
    ];

    public function pipeline(): BelongsTo
    {
        return $this->belongsTo(Pipeline::class);
    }

    public function deals(): HasMany
    {
        return $this->hasMany(Deal::class);
    }
}
