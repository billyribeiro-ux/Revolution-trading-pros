<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class WorkflowSchedule extends Model
{
    protected $fillable = [
        'workflow_id',
        'schedule_type',
        'schedule_config',
        'next_run_at',
        'last_run_at',
        'is_active',
    ];

    protected $casts = [
        'schedule_config' => 'array',
        'next_run_at' => 'datetime',
        'last_run_at' => 'datetime',
        'is_active' => 'boolean',
    ];

    public function workflow(): BelongsTo
    {
        return $this->belongsTo(Workflow::class);
    }

    public function updateNextRun(\DateTime $nextRun): void
    {
        $this->update([
            'next_run_at' => $nextRun,
            'last_run_at' => now(),
        ]);
    }

    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    public function scopeDue($query)
    {
        return $query->where('next_run_at', '<=', now())
                    ->where('is_active', true);
    }
}
