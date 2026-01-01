<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class WorkflowNode extends Model
{
    protected $fillable = [
        'workflow_id',
        'node_type',
        'config',
        'position_x',
        'position_y',
        'parent_node_id',
        'order',
    ];

    protected $casts = [
        'config' => 'array',
        'position_x' => 'integer',
        'position_y' => 'integer',
        'order' => 'integer',
    ];

    public function workflow(): BelongsTo
    {
        return $this->belongsTo(Workflow::class);
    }

    public function parent(): BelongsTo
    {
        return $this->belongsTo(WorkflowNode::class, 'parent_node_id');
    }

    public function children(): HasMany
    {
        return $this->hasMany(WorkflowNode::class, 'parent_node_id');
    }

    public function outgoingEdges(): HasMany
    {
        return $this->hasMany(WorkflowEdge::class, 'from_node_id');
    }

    public function incomingEdges(): HasMany
    {
        return $this->hasMany(WorkflowEdge::class, 'to_node_id');
    }

    public function runLogs(): HasMany
    {
        return $this->hasMany(WorkflowRunLog::class, 'node_id');
    }

    public function isTrigger(): bool
    {
        return $this->node_type === 'trigger';
    }

    public function isAction(): bool
    {
        return $this->node_type === 'action';
    }

    public function isCondition(): bool
    {
        return $this->node_type === 'condition';
    }

    public function isDelay(): bool
    {
        return $this->node_type === 'delay';
    }

    public function isBranch(): bool
    {
        return $this->node_type === 'branch';
    }

    public function isParallel(): bool
    {
        return $this->node_type === 'parallel';
    }
}
