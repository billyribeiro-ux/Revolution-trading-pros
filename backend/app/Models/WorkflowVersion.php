<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class WorkflowVersion extends Model
{
    public $timestamps = false;

    protected $fillable = [
        'workflow_id',
        'version_number',
        'definition',
        'created_by',
        'created_at',
        'change_description',
    ];

    protected $casts = [
        'definition' => 'array',
        'created_at' => 'datetime',
        'version_number' => 'integer',
    ];

    public function workflow(): BelongsTo
    {
        return $this->belongsTo(Workflow::class);
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }
}
