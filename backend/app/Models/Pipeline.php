<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Pipeline extends Model
{
    use HasUuids;

    protected $fillable = [
        'name', 'description', 'is_default', 'is_active',
        'deals_count', 'total_value', 'win_rate', 'avg_deal_size', 'avg_sales_cycle',
        'color', 'icon', 'position'
    ];

    protected $casts = [
        'is_default' => 'boolean',
        'is_active' => 'boolean',
        'total_value' => 'decimal:2',
        'win_rate' => 'decimal:2',
        'avg_deal_size' => 'decimal:2',
    ];

    public function stages(): HasMany
    {
        return $this->hasMany(Stage::class)->orderBy('position');
    }

    public function deals(): HasMany
    {
        return $this->hasMany(Deal::class);
    }
}
