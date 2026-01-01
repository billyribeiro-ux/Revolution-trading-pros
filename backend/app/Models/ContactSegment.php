<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;

class ContactSegment extends Model
{
    use HasUuids;

    protected $fillable = [
        'name', 'description', 'conditions', 'is_dynamic',
        'contacts_count', 'last_calculated_at', 'created_by_id', 'is_shared'
    ];

    protected $casts = [
        'conditions' => 'array',
        'is_dynamic' => 'boolean',
        'is_shared' => 'boolean',
        'last_calculated_at' => 'datetime',
    ];

    public function contacts(): BelongsToMany
    {
        return $this->belongsToMany(Contact::class, 'contact_segment_members');
    }

    public function createdBy(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by_id');
    }
}
