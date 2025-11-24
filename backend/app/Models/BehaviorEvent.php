<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class BehaviorEvent extends Model
{
    use HasUuids;

    public $timestamps = false;

    protected $fillable = [
        'session_id',
        'event_type',
        'timestamp',
        'page_url',
        'element',
        'element_selector',
        'coordinates_x',
        'coordinates_y',
        'event_value',
        'event_metadata',
        'local_engagement_score',
        'local_friction_score',
        'sequence_number',
        'time_since_session_start',
        'time_since_last_event',
    ];

    protected $casts = [
        'timestamp' => 'datetime',
        'event_metadata' => 'array',
        'local_engagement_score' => 'float',
        'local_friction_score' => 'float',
    ];

    public function session(): BelongsTo
    {
        return $this->belongsTo(BehaviorSession::class, 'session_id');
    }
}
