<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Support\Str;

class EmailBlock extends Model
{
    protected $fillable = [
        'uuid',
        'template_id',
        'block_type',
        'content',
        'styles',
        'settings',
        'order',
        'parent_block_id',
        'conditional_rules',
    ];

    protected $casts = [
        'content' => 'array',
        'styles' => 'array',
        'settings' => 'array',
        'conditional_rules' => 'array',
    ];

    protected static function boot()
    {
        parent::boot();
        
        static::creating(function ($block) {
            if (empty($block->uuid)) {
                $block->uuid = (string) Str::uuid();
            }
        });
    }

    public function template(): BelongsTo
    {
        return $this->belongsTo(EmailTemplate::class, 'template_id');
    }

    public function parentBlock(): BelongsTo
    {
        return $this->belongsTo(EmailBlock::class, 'parent_block_id');
    }

    public function childBlocks(): HasMany
    {
        return $this->hasMany(EmailBlock::class, 'parent_block_id')->orderBy('order');
    }

    public function scopeOrdered($query)
    {
        return $query->orderBy('order');
    }

    public function scopeRootBlocks($query)
    {
        return $query->whereNull('parent_block_id');
    }
}
