<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class FrictionPoint extends Model
{
    use HasUuids;

    protected $fillable = [
        'session_id',
        'page_url',
        'friction_type',
        'severity',
        'element',
        'element_selector',
        'description',
        'event_count',
        'first_occurred_at',
        'last_occurred_at',
        'user_segment',
        'device_type',
        'resolved',
        'resolved_at',
        'resolution_notes',
    ];

    protected $casts = [
        'first_occurred_at' => 'datetime',
        'last_occurred_at' => 'datetime',
        'resolved' => 'boolean',
        'resolved_at' => 'datetime',
    ];

    public function session(): BelongsTo
    {
        return $this->belongsTo(BehaviorSession::class, 'session_id');
    }
}
