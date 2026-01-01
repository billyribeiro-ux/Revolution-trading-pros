<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class WorkflowEdge extends Model
{
    protected $fillable = [
        'workflow_id',
        'from_node_id',
        'to_node_id',
        'condition_type',
        'label',
    ];

    public function workflow(): BelongsTo
    {
        return $this->belongsTo(Workflow::class);
    }

    public function fromNode(): BelongsTo
    {
        return $this->belongsTo(WorkflowNode::class, 'from_node_id');
    }

    public function toNode(): BelongsTo
    {
        return $this->belongsTo(WorkflowNode::class, 'to_node_id');
    }

    public function isConditional(): bool
    {
        return in_array($this->condition_type, ['if_true', 'if_false']);
    }

    public function isParallel(): bool
    {
        return $this->condition_type === 'parallel';
    }

    public function isAlways(): bool
    {
        return $this->condition_type === 'always';
    }
}
