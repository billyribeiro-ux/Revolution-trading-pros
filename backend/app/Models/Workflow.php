<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

class Workflow extends Model
{
    use SoftDeletes;

    protected $fillable = [
        'user_id',
        'name',
        'description',
        'status',
        'trigger_config',
        'version',
        'execution_count',
        'success_count',
        'failure_count',
        'last_executed_at',
        'avg_execution_time_ms',
    ];

    protected $casts = [
        'trigger_config' => 'array',
        'last_executed_at' => 'datetime',
        'execution_count' => 'integer',
        'success_count' => 'integer',
        'failure_count' => 'integer',
        'avg_execution_time_ms' => 'integer',
        'version' => 'integer',
    ];

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function nodes(): HasMany
    {
        return $this->hasMany(WorkflowNode::class);
    }

    public function edges(): HasMany
    {
        return $this->hasMany(WorkflowEdge::class);
    }

    public function runs(): HasMany
    {
        return $this->hasMany(WorkflowRun::class);
    }

    public function triggers(): HasMany
    {
        return $this->hasMany(WorkflowTrigger::class);
    }

    public function schedules(): HasMany
    {
        return $this->hasMany(WorkflowSchedule::class);
    }

    public function versions(): HasMany
    {
        return $this->hasMany(WorkflowVersion::class);
    }

    public function isActive(): bool
    {
        return $this->status === 'active';
    }

    public function isPaused(): bool
    {
        return $this->status === 'paused';
    }

    public function incrementExecutionCount(): void
    {
        $this->increment('execution_count');
    }

    public function incrementSuccessCount(): void
    {
        $this->increment('success_count');
    }

    public function incrementFailureCount(): void
    {
        $this->increment('failure_count');
    }

    public function updateAverageExecutionTime(int $durationMs): void
    {
        $totalExecutions = $this->execution_count;
        $currentAvg = $this->avg_execution_time_ms ?? 0;
        
        $newAvg = (($currentAvg * ($totalExecutions - 1)) + $durationMs) / $totalExecutions;
        
        $this->update(['avg_execution_time_ms' => round($newAvg)]);
    }

    public function getSuccessRate(): float
    {
        if ($this->execution_count === 0) {
            return 0;
        }
        
        return ($this->success_count / $this->execution_count) * 100;
    }

    public function scopeActive($query)
    {
        return $query->where('status', 'active');
    }

    public function scopeForUser($query, $userId)
    {
        return $query->where('user_id', $userId);
    }
}
