<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class BehaviorSession extends Model
{
    use HasUuids;

    protected $fillable = [
        'user_id',
        'visitor_id',
        'session_fingerprint',
        'started_at',
        'ended_at',
        'duration_seconds',
        'page_count',
        'event_count',
        'engagement_score',
        'intent_score',
        'friction_score',
        'churn_risk_score',
        'has_rage_clicks',
        'has_form_abandonment',
        'has_speed_scrolls',
        'has_exit_intent',
        'has_dead_clicks',
        'device_type',
        'browser',
        'viewport_width',
        'viewport_height',
        'entry_url',
        'exit_url',
        'referrer',
        'utm_source',
        'utm_campaign',
        'utm_medium',
    ];

    protected $casts = [
        'started_at' => 'datetime',
        'ended_at' => 'datetime',
        'engagement_score' => 'float',
        'intent_score' => 'float',
        'friction_score' => 'float',
        'churn_risk_score' => 'float',
        'has_rage_clicks' => 'boolean',
        'has_form_abandonment' => 'boolean',
        'has_speed_scrolls' => 'boolean',
        'has_exit_intent' => 'boolean',
        'has_dead_clicks' => 'boolean',
    ];

    public function events(): HasMany
    {
        return $this->hasMany(BehaviorEvent::class, 'session_id');
    }

    public function frictionPoints(): HasMany
    {
        return $this->hasMany(FrictionPoint::class, 'session_id');
    }

    public function intentSignals(): HasMany
    {
        return $this->hasMany(IntentSignal::class, 'session_id');
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
