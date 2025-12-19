<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * ExperimentAssignment Model
 *
 * Tracks user assignments to experiment variants.
 *
 * @property string $id
 * @property string $experiment_key
 * @property string $user_id
 * @property string $variant
 * @property string|null $session_id
 * @property array|null $context
 * @property array|null $conversions
 * @property \Carbon\Carbon|null $exposed_at
 * @property \Carbon\Carbon|null $converted_at
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class ExperimentAssignment extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'experiment_key',
        'user_id',
        'variant',
        'session_id',
        'context',
        'conversions',
        'exposed_at',
        'converted_at',
    ];

    protected $casts = [
        'context' => 'array',
        'conversions' => 'array',
        'exposed_at' => 'datetime',
        'converted_at' => 'datetime',
    ];

    /**
     * Get the experiment this assignment belongs to
     */
    public function experiment(): BelongsTo
    {
        return $this->belongsTo(Experiment::class, 'experiment_key', 'key');
    }

    /**
     * Get the user this assignment belongs to
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class, 'user_id', 'id');
    }

    /**
     * Check if user has converted
     */
    public function hasConverted(): bool
    {
        return $this->converted_at !== null;
    }

    /**
     * Get total conversion value
     */
    public function getTotalConversionValue(): float
    {
        if (!$this->conversions) {
            return 0.0;
        }

        return collect($this->conversions)
            ->sum('value') ?? 0.0;
    }
}
