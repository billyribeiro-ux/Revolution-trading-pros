<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class EmailLayout extends Model
{
    protected $fillable = [
        'name',
        'slug',
        'html_structure',
        'default_styles',
        'header_blocks',
        'footer_blocks',
        'is_system',
        'is_default',
    ];

    protected $casts = [
        'default_styles' => 'array',
        'header_blocks' => 'array',
        'footer_blocks' => 'array',
        'is_system' => 'boolean',
        'is_default' => 'boolean',
    ];

    public function templates(): HasMany
    {
        return $this->hasMany(EmailTemplate::class, 'layout_id');
    }

    public function scopeDefault($query)
    {
        return $query->where('is_default', true);
    }

    public function scopeSystem($query)
    {
        return $query->where('is_system', true);
    }
}
