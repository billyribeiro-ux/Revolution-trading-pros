<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;

/**
 * EmailVariable Model
 * 
 * Manages dynamic variables available for email templates
 * 
 * @property int $id
 * @property string $category
 * @property string $variable_key
 * @property string $variable_name
 * @property string|null $description
 * @property string $data_type
 * @property string|null $default_value
 * @property bool $is_system
 * @property string|null $resolver_class
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class EmailVariable extends Model
{
    use HasFactory;

    protected $fillable = [
        'category',
        'variable_key',
        'variable_name',
        'description',
        'data_type',
        'default_value',
        'is_system',
        'resolver_class',
    ];

    protected $casts = [
        'is_system' => 'boolean',
    ];

    /**
     * Scope: Filter by category
     */
    public function scopeByCategory(Builder $query, string $category): Builder
    {
        return $query->where('category', $category);
    }

    /**
     * Scope: System variables only
     */
    public function scopeSystemOnly(Builder $query): Builder
    {
        return $query->where('is_system', true);
    }

    /**
     * Scope: Custom variables only
     */
    public function scopeCustomOnly(Builder $query): Builder
    {
        return $query->where('is_system', false);
    }

    /**
     * Get formatted variable for display
     */
    public function getFormattedVariableAttribute(): string
    {
        return "{{" . $this->variable_key . "}}";
    }
}
