<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class WorkflowRunLog extends Model
{
    public $timestamps = false;

    protected $fillable = [
        'workflow_run_id',
        'node_id',
        'status',
        'executed_at',
        'input_data',
        'output_data',
        'error',
        'duration_ms',
    ];

    protected $casts = [
        'executed_at' => 'datetime',
        'input_data' => 'array',
        'output_data' => 'array',
        'duration_ms' => 'integer',
    ];

    public function workflowRun(): BelongsTo
    {
        return $this->belongsTo(WorkflowRun::class);
    }

    public function node(): BelongsTo
    {
        return $this->belongsTo(WorkflowNode::class);
    }

    public function isCompleted(): bool
    {
        return $this->status === 'completed';
    }

    public function isFailed(): bool
    {
        return $this->status === 'failed';
    }

    public function isSkipped(): bool
    {
        return $this->status === 'skipped';
    }
}
