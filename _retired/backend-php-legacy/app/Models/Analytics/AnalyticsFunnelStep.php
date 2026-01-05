<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * AnalyticsFunnelStep Model - Individual Funnel Steps
 *
 * @property int $id
 * @property int $funnel_id
 * @property int $step_number
 * @property string $name
 * @property string $event_name
 */
class AnalyticsFunnelStep extends Model
{
    protected $table = 'analytics_funnel_steps';

    protected $fillable = [
        'funnel_id',
        'step_number',
        'name',
        'description',
        'event_name',
        'event_conditions',
        'is_optional',
        'time_limit_hours',
    ];

    protected $casts = [
        'funnel_id' => 'integer',
        'step_number' => 'integer',
        'event_conditions' => 'array',
        'is_optional' => 'boolean',
        'time_limit_hours' => 'integer',
    ];

    protected $attributes = [
        'is_optional' => false,
    ];

    public function funnel(): BelongsTo
    {
        return $this->belongsTo(AnalyticsFunnel::class, 'funnel_id');
    }
}
