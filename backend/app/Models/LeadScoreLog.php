<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class LeadScoreLog extends Model
{
    use HasUuids;

    public $timestamps = false;

    protected $fillable = [
        'contact_id', 'previous_score', 'new_score', 'change', 'reason', 'contributing_factors'
    ];

    protected $casts = [
        'contributing_factors' => 'array',
        'created_at' => 'datetime',
    ];

    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }
}
