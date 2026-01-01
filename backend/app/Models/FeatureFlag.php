<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

/**
 * FeatureFlag Model
 *
 * Represents a feature flag for gradual rollouts.
 *
 * @property string $id
 * @property string $key
 * @property string $name
 * @property string|null $description
 * @property bool $enabled
 * @property int $rollout_percentage
 * @property array|null $targeting_rules
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class FeatureFlag extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'key',
        'name',
        'description',
        'enabled',
        'rollout_percentage',
        'targeting_rules',
    ];

    protected $casts = [
        'enabled' => 'boolean',
        'rollout_percentage' => 'integer',
        'targeting_rules' => 'array',
    ];

    protected $attributes = [
        'enabled' => false,
        'rollout_percentage' => 100,
    ];

    /**
     * Check if flag is enabled for a user
     */
    public function isEnabledFor(?string $userId): bool
    {
        if (!$this->enabled) {
            return false;
        }

        if ($this->rollout_percentage >= 100) {
            return true;
        }

        if (!$userId) {
            return false;
        }

        $hash = crc32("{$this->key}:{$userId}");
        $bucket = abs($hash) % 100;

        return $bucket < $this->rollout_percentage;
    }

    /**
     * Evaluate targeting rules
     */
    public function evaluateTargeting(array $context): bool
    {
        if (!$this->targeting_rules || empty($this->targeting_rules)) {
            return true;
        }

        foreach ($this->targeting_rules as $rule) {
            if (!$this->evaluateRule($rule, $context)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Evaluate a single targeting rule
     */
    private function evaluateRule(array $rule, array $context): bool
    {
        $field = $rule['field'] ?? null;
        $operator = $rule['operator'] ?? 'equals';
        $value = $rule['value'] ?? null;

        if (!$field || !isset($context[$field])) {
            return false;
        }

        $contextValue = $context[$field];

        return match ($operator) {
            'equals' => $contextValue === $value,
            'not_equals' => $contextValue !== $value,
            'contains' => is_string($contextValue) && str_contains($contextValue, $value),
            'in' => is_array($value) && in_array($contextValue, $value),
            'not_in' => is_array($value) && !in_array($contextValue, $value),
            'greater_than' => is_numeric($contextValue) && $contextValue > $value,
            'less_than' => is_numeric($contextValue) && $contextValue < $value,
            default => false,
        };
    }
}
