<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

/**
 * Experiment Model
 *
 * Represents an A/B test or experiment configuration.
 *
 * @property string $id
 * @property string $key
 * @property string $name
 * @property string|null $description
 * @property array $variants
 * @property array|null $variant_weights
 * @property string $default_variant
 * @property int $rollout_percentage
 * @property bool $enabled
 * @property \Carbon\Carbon|null $starts_at
 * @property \Carbon\Carbon|null $ends_at
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class Experiment extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'key',
        'name',
        'description',
        'variants',
        'variant_weights',
        'default_variant',
        'rollout_percentage',
        'enabled',
        'starts_at',
        'ends_at',
    ];

    protected $casts = [
        'variants' => 'array',
        'variant_weights' => 'array',
        'rollout_percentage' => 'integer',
        'enabled' => 'boolean',
        'starts_at' => 'datetime',
        'ends_at' => 'datetime',
    ];

    protected $attributes = [
        'variants' => '["control", "treatment"]',
        'rollout_percentage' => 100,
        'enabled' => false,
        'default_variant' => 'control',
    ];

    /**
     * Get the assignments for this experiment
     */
    public function assignments(): HasMany
    {
        return $this->hasMany(ExperimentAssignment::class, 'experiment_key', 'key');
    }

    /**
     * Check if experiment is currently active
     */
    public function isActive(): bool
    {
        if (!$this->enabled) {
            return false;
        }

        $now = now();

        if ($this->starts_at && $now->lt($this->starts_at)) {
            return false;
        }

        if ($this->ends_at && $now->gt($this->ends_at)) {
            return false;
        }

        return true;
    }

    /**
     * Get conversion rate for a variant
     */
    public function getConversionRate(string $variant): float
    {
        $assignments = $this->assignments()
            ->where('variant', $variant)
            ->get();

        if ($assignments->isEmpty()) {
            return 0.0;
        }

        $conversions = $assignments->whereNotNull('converted_at')->count();

        return ($conversions / $assignments->count()) * 100;
    }
}
