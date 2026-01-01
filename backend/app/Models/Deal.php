<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Deal extends Model
{
    use HasUuids;

    protected $fillable = [
        'name', 'contact_id', 'company_id', 'pipeline_id', 'stage_id',
        'amount', 'currency', 'probability', 'owner_id', 'status',
        'close_date', 'expected_close_date', 'lost_reason', 'won_details',
        'stage_changes_count', 'custom_fields', 'tags', 'priority',
        'source_channel', 'source_campaign', 'stage_entered_at', 'closed_at'
    ];

    protected $casts = [
        'amount' => 'decimal:2',
        'custom_fields' => 'array',
        'tags' => 'array',
        'close_date' => 'date',
        'expected_close_date' => 'date',
        'stage_entered_at' => 'datetime',
        'closed_at' => 'datetime',
    ];

    protected $appends = ['weighted_value', 'days_in_stage', 'days_in_pipeline'];

    public function getWeightedValueAttribute(): float
    {
        return $this->amount * ($this->probability / 100);
    }

    public function getDaysInStageAttribute(): int
    {
        return $this->stage_entered_at->diffInDays(now());
    }

    public function getDaysInPipelineAttribute(): int
    {
        return $this->created_at->diffInDays(now());
    }

    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }

    public function pipeline(): BelongsTo
    {
        return $this->belongsTo(Pipeline::class);
    }

    public function stage(): BelongsTo
    {
        return $this->belongsTo(Stage::class);
    }

    public function owner(): BelongsTo
    {
        return $this->belongsTo(User::class, 'owner_id');
    }

    public function activities(): HasMany
    {
        return $this->hasMany(CrmActivity::class, 'subject_id')
            ->where('subject_type', 'deal');
    }

    public function notes(): HasMany
    {
        return $this->hasMany(CrmNote::class);
    }

    public function stageHistory(): HasMany
    {
        return $this->hasMany(DealStageHistory::class);
    }
}
