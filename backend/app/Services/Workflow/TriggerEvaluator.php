<?php

namespace App\Services\Workflow;

use App\Models\Workflow;
use Illuminate\Support\Facades\Log;

class TriggerEvaluator
{
    /**
     * Evaluate if a trigger matches the event
     */
    public function matches(array $triggerConfig, array $eventData): bool
    {
        $triggerType = $triggerConfig['type'] ?? null;
        $eventType = $eventData['type'] ?? null;

        if ($triggerType !== $eventType) {
            return false;
        }

        // Check additional conditions
        $conditions = $triggerConfig['conditions'] ?? [];
        
        foreach ($conditions as $condition) {
            if (!$this->evaluateCondition($condition, $eventData)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Evaluate a single condition
     */
    private function evaluateCondition(array $condition, array $data): bool
    {
        $field = $condition['field'] ?? null;
        $operator = $condition['operator'] ?? 'equals';
        $value = $condition['value'] ?? null;

        $actualValue = data_get($data, $field);

        return match ($operator) {
            'equals' => $actualValue == $value,
            'not_equals' => $actualValue != $value,
            'greater_than' => $actualValue > $value,
            'less_than' => $actualValue < $value,
            'greater_or_equal' => $actualValue >= $value,
            'less_or_equal' => $actualValue <= $value,
            'contains' => str_contains((string)$actualValue, (string)$value),
            'not_contains' => !str_contains((string)$actualValue, (string)$value),
            'starts_with' => str_starts_with((string)$actualValue, (string)$value),
            'ends_with' => str_ends_with((string)$actualValue, (string)$value),
            'is_empty' => empty($actualValue),
            'is_not_empty' => !empty($actualValue),
            'in_array' => in_array($actualValue, (array)$value),
            'not_in_array' => !in_array($actualValue, (array)$value),
            default => false,
        };
    }

    /**
     * Find workflows that should be triggered by this event
     */
    public function findMatchingWorkflows(array $eventData): array
    {
        $eventType = $eventData['type'] ?? null;
        
        if (!$eventType) {
            return [];
        }

        $workflows = Workflow::active()
            ->whereHas('triggers', function ($query) use ($eventType) {
                $query->where('trigger_type', $eventType)
                      ->where('is_active', true);
            })
            ->get();

        return $workflows->filter(function ($workflow) use ($eventData) {
            return $this->matches($workflow->trigger_config, $eventData);
        })->all();
    }
}
