<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Dashboard extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'user_id',
        'dashboard_type',
        'name',
        'description',
        'is_default',
        'is_public',
        'grid_columns',
        'grid_row_height',
        'grid_gap',
        'shared_with',
        'required_role',
        'last_viewed_at',
    ];

    protected $casts = [
        'is_default' => 'boolean',
        'is_public' => 'boolean',
        'grid_columns' => 'integer',
        'grid_row_height' => 'integer',
        'grid_gap' => 'integer',
        'shared_with' => 'array',
        'last_viewed_at' => 'datetime',
    ];

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function widgets(): HasMany
    {
        return $this->hasMany(DashboardWidget::class);
    }

    public function scopeForUser($query, $userId)
    {
        return $query->where(function ($q) use ($userId) {
            $q->where('user_id', $userId)
              ->orWhere('is_public', true)
              ->orWhereJsonContains('shared_with', $userId);
        });
    }

    public function scopeByType($query, $type)
    {
        return $query->where('dashboard_type', $type);
    }
}
