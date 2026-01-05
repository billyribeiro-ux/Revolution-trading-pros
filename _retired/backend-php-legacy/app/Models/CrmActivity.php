<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\MorphTo;

class CrmActivity extends Model
{
    use HasUuids;

    protected $fillable = [
        'subject_type', 'subject_id', 'type', 'title', 'description', 'metadata',
        'created_by_id', 'assigned_to_id', 'due_date', 'completed_at', 'priority',
        'email_id', 'form_submission_id', 'popup_id', 'behavior_session_id', 'subscription_id',
        'occurred_at'
    ];

    protected $casts = [
        'metadata' => 'array',
        'due_date' => 'datetime',
        'completed_at' => 'datetime',
        'occurred_at' => 'datetime',
    ];

    public function subject(): MorphTo
    {
        return $this->morphTo();
    }

    public function createdBy(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by_id');
    }

    public function assignedTo(): BelongsTo
    {
        return $this->belongsTo(User::class, 'assigned_to_id');
    }
}
