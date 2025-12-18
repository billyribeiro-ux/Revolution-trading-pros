<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class IntentSignal extends Model
{
    use HasUuids;

    public $timestamps = false;

    protected $fillable = [
        'session_id',
        'user_id',
        'signal_type',
        'intent_strength',
        'element',
        'page_url',
        'timestamp',
        'converted',
        'conversion_timestamp',
    ];

    protected $casts = [
        'timestamp' => 'datetime',
        'converted' => 'boolean',
        'conversion_timestamp' => 'datetime',
    ];

    public function session(): BelongsTo
    {
        return $this->belongsTo(BehaviorSession::class, 'session_id');
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
