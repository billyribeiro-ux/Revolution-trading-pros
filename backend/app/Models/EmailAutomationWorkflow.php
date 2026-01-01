<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class EmailAutomationWorkflow extends Model
{
    protected $fillable = [
        'name',
        'trigger_event',
        'trigger_conditions',
        'template_id',
        'delay_minutes',
        'is_active',
        'priority',
        'send_once_per_user',
        'max_sends_per_user',
        'statistics',
    ];

    protected $casts = [
        'trigger_conditions' => 'array',
        'statistics' => 'array',
        'is_active' => 'boolean',
        'send_once_per_user' => 'boolean',
    ];

    public function template(): BelongsTo
    {
        return $this->belongsTo(EmailTemplate::class, 'template_id');
    }

    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    public function scopeForEvent($query, string $event)
    {
        return $query->where('trigger_event', $event);
    }

    public function scopeByPriority($query)
    {
        return $query->orderByDesc('priority');
    }

    public function incrementStat(string $stat): void
    {
        $statistics = $this->statistics ?? [];
        $statistics[$stat] = ($statistics[$stat] ?? 0) + 1;
        $this->update(['statistics' => $statistics]);
    }
}
