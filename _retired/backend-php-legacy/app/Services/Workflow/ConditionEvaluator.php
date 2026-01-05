<?php

namespace App\Services\Workflow;

class ConditionEvaluator
{
    /**
     * Evaluate condition configuration against context
     */
    public function evaluate(array $config, array $context): bool
    {
        $logic = $config['logic'] ?? 'AND';
        $conditions = $config['conditions'] ?? [];

        if (empty($conditions)) {
            return true;
        }

        $results = array_map(
            fn($condition) => $this->evaluateSingle($condition, $context),
            $conditions
        );

        return match ($logic) {
            'AND' => !in_array(false, $results, true),
            'OR' => in_array(true, $results, true),
            'NOT' => !$results[0],
            default => true,
        };
    }

    /**
     * Evaluate a single condition
     */
    private function evaluateSingle(array $condition, array $context): bool
    {
        $type = $condition['type'] ?? 'field';

        return match ($type) {
            'field' => $this->evaluateFieldCondition($condition, $context),
            'behavior' => $this->evaluateBehaviorCondition($condition, $context),
            'temporal' => $this->evaluateTemporalCondition($condition, $context),
            'aggregate' => $this->evaluateAggregateCondition($condition, $context),
            default => false,
        };
    }

    /**
     * Evaluate field condition
     */
    private function evaluateFieldCondition(array $condition, array $context): bool
    {
        $field = $condition['field'] ?? null;
        $operator = $condition['operator'] ?? 'equals';
        $value = $condition['value'] ?? null;

        $actualValue = data_get($context, $field);

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
            'regex' => preg_match($value, (string)$actualValue),
            default => false,
        };
    }

    /**
     * Evaluate behavior condition
     */
    private function evaluateBehaviorCondition(array $condition, array $context): bool
    {
        // Placeholder for behavior-based conditions
        // e.g., page visits, click counts, time on site
        return true;
    }

    /**
     * Evaluate temporal condition
     */
    private function evaluateTemporalCondition(array $condition, array $context): bool
    {
        $type = $condition['temporal_type'] ?? 'time';
        $operator = $condition['operator'] ?? 'equals';
        $value = $condition['value'] ?? null;

        $now = now();

        return match ($type) {
            'time' => $this->compareTime($now->format('H:i'), $operator, $value),
            'date' => $this->compareDate($now->format('Y-m-d'), $operator, $value),
            'day_of_week' => $now->dayOfWeek == $value,
            'hour' => $this->compareValue($now->hour, $operator, $value),
            default => false,
        };
    }

    /**
     * Evaluate aggregate condition
     */
    private function evaluateAggregateCondition(array $condition, array $context): bool
    {
        // Placeholder for aggregate conditions
        // e.g., count, sum, average calculations
        return true;
    }

    /**
     * Compare time values
     */
    private function compareTime(string $actual, string $operator, string $expected): bool
    {
        return match ($operator) {
            'equals' => $actual === $expected,
            'before' => $actual < $expected,
            'after' => $actual > $expected,
            'between' => $this->isBetween($actual, $expected),
            default => false,
        };
    }

    /**
     * Compare date values
     */
    private function compareDate(string $actual, string $operator, string $expected): bool
    {
        return match ($operator) {
            'equals' => $actual === $expected,
            'before' => $actual < $expected,
            'after' => $actual > $expected,
            default => false,
        };
    }

    /**
     * Compare numeric values
     */
    private function compareValue($actual, string $operator, $expected): bool
    {
        return match ($operator) {
            'equals' => $actual == $expected,
            'greater_than' => $actual > $expected,
            'less_than' => $actual < $expected,
            'greater_or_equal' => $actual >= $expected,
            'less_or_equal' => $actual <= $expected,
            default => false,
        };
    }

    /**
     * Check if value is between range
     */
    private function isBetween($value, $range): bool
    {
        [$min, $max] = explode(',', $range);
        return $value >= $min && $value <= $max;
    }
}
