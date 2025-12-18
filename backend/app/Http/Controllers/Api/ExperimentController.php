<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Experiment;
use App\Models\ExperimentAssignment;
use App\Models\FeatureFlag;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * ExperimentController - Feature Flags and A/B Testing API
 *
 * Netflix E6 Level Experimentation Infrastructure
 * Supports feature flags, A/B tests, and experiment tracking.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
class ExperimentController extends Controller
{
    /**
     * Get all active experiments and feature flags for a user
     */
    public function getConfig(Request $request): JsonResponse
    {
        $userId = $request->user()?->id ?? $request->input('anonymous_id');
        
        // Get active experiments
        $experiments = Cache::remember('experiments:active', 300, function () {
            return Experiment::where('enabled', true)
                ->where(function ($query) {
                    $query->whereNull('ends_at')
                        ->orWhere('ends_at', '>', now());
                })
                ->get();
        });

        // Get feature flags
        $featureFlags = Cache::remember('feature_flags:active', 300, function () {
            return FeatureFlag::where('enabled', true)->get();
        });

        // Get user's experiment assignments
        $assignments = [];
        foreach ($experiments as $experiment) {
            $assignments[$experiment->key] = $this->getAssignment($experiment, $userId);
        }

        // Evaluate feature flags for user
        $flags = [];
        foreach ($featureFlags as $flag) {
            $flags[$flag->key] = $this->evaluateFlag($flag, $userId);
        }

        return response()->json([
            'experiments' => $assignments,
            'feature_flags' => $flags,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Track experiment exposure
     */
    public function trackExposure(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'experiment_id' => 'required|string|max:100',
            'variant' => 'required|string|max:50',
            'user_id' => 'nullable|string|max:100',
            'session_id' => 'nullable|string|max:100',
        ]);

        $userId = $request->user()?->id ?? $validated['user_id'];

        ExperimentAssignment::updateOrCreate(
            [
                'experiment_key' => $validated['experiment_id'],
                'user_id' => $userId,
            ],
            [
                'variant' => $validated['variant'],
                'session_id' => $validated['session_id'] ?? null,
                'exposed_at' => now(),
                'context' => [
                    'ip' => $request->ip(),
                    'user_agent' => $request->userAgent(),
                    'url' => $request->header('Referer'),
                ],
            ]
        );

        return response()->json(['success' => true]);
    }

    /**
     * Track experiment conversion
     */
    public function trackConversion(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'experiment_id' => 'required|string|max:100',
            'conversion_type' => 'required|string|max:100',
            'conversion_value' => 'nullable|numeric',
            'user_id' => 'nullable|string|max:100',
        ]);

        $userId = $request->user()?->id ?? $validated['user_id'];

        $assignment = ExperimentAssignment::where('experiment_key', $validated['experiment_id'])
            ->where('user_id', $userId)
            ->first();

        if ($assignment) {
            $conversions = $assignment->conversions ?? [];
            $conversions[] = [
                'type' => $validated['conversion_type'],
                'value' => $validated['conversion_value'] ?? null,
                'timestamp' => now()->toIso8601String(),
            ];
            
            $assignment->update([
                'conversions' => $conversions,
                'converted_at' => $assignment->converted_at ?? now(),
            ]);
        }

        return response()->json(['success' => true]);
    }

    /**
     * Get assignment for an experiment
     */
    private function getAssignment(Experiment $experiment, ?string $userId): array
    {
        if (!$userId) {
            return [
                'variant' => $experiment->default_variant,
                'assigned' => false,
            ];
        }

        // Check for existing assignment
        $existing = ExperimentAssignment::where('experiment_key', $experiment->key)
            ->where('user_id', $userId)
            ->first();

        if ($existing) {
            return [
                'variant' => $existing->variant,
                'assigned' => true,
            ];
        }

        // Assign based on hash
        $hash = crc32("{$experiment->key}:{$userId}");
        $bucket = abs($hash) % 100;

        // Check if user is in experiment rollout
        if ($bucket >= $experiment->rollout_percentage) {
            return [
                'variant' => $experiment->default_variant,
                'assigned' => false,
            ];
        }

        // Assign variant based on weights
        $variants = $experiment->variants ?? ['control', 'treatment'];
        $weights = $experiment->variant_weights ?? array_fill(0, count($variants), 100 / count($variants));
        
        $variantBucket = abs(crc32("{$experiment->key}:{$userId}:variant")) % 100;
        $cumulative = 0;
        $assignedVariant = $variants[0];

        foreach ($variants as $index => $variant) {
            $cumulative += $weights[$index] ?? (100 / count($variants));
            if ($variantBucket < $cumulative) {
                $assignedVariant = $variant;
                break;
            }
        }

        return [
            'variant' => $assignedVariant,
            'assigned' => true,
        ];
    }

    /**
     * Evaluate a feature flag for a user
     */
    private function evaluateFlag(FeatureFlag $flag, ?string $userId): bool
    {
        if (!$flag->enabled) {
            return false;
        }

        // Check rollout percentage
        if ($flag->rollout_percentage < 100) {
            if (!$userId) {
                return false;
            }

            $hash = crc32("{$flag->key}:{$userId}");
            $bucket = abs($hash) % 100;

            if ($bucket >= $flag->rollout_percentage) {
                return false;
            }
        }

        // Check targeting rules if defined
        if ($flag->targeting_rules && is_array($flag->targeting_rules)) {
            $user = $userId ? \App\Models\User::find($userId) : null;
            $context = $this->buildEvaluationContext($user);

            if (!$this->evaluateTargetingRules($flag->targeting_rules, $context)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Build evaluation context for targeting rules
     */
    private function buildEvaluationContext(?\App\Models\User $user): array
    {
        return [
            'user' => $user ? [
                'id' => $user->id,
                'email' => $user->email,
                'name' => $user->name,
                'created_at' => $user->created_at?->toDateString(),
                'email_verified' => $user->email_verified_at !== null,
                'roles' => $user->roles ?? [],
                'subscription_status' => $user->subscription_status ?? 'free',
                'total_orders' => $user->orders_count ?? 0,
            ] : null,
            'environment' => app()->environment(),
            'date' => now()->toDateString(),
            'time' => now()->format('H:i'),
            'day_of_week' => strtolower(now()->format('l')),
            'is_weekend' => now()->isWeekend(),
        ];
    }

    /**
     * Evaluate targeting rules against context
     *
     * Rule format:
     * [
     *   'operator' => 'AND' | 'OR',
     *   'conditions' => [
     *     ['field' => 'user.email', 'operator' => 'ends_with', 'value' => '@company.com'],
     *     ['field' => 'user.subscription_status', 'operator' => 'in', 'value' => ['premium', 'enterprise']],
     *   ]
     * ]
     */
    private function evaluateTargetingRules(array $rules, array $context): bool
    {
        // Default to AND if no operator specified
        $operator = $rules['operator'] ?? 'AND';
        $conditions = $rules['conditions'] ?? $rules;

        if (empty($conditions)) {
            return true;
        }

        // If conditions is a simple array of conditions
        if (!isset($conditions[0]['field']) && !isset($rules['conditions'])) {
            // This is a single condition object
            return $this->evaluateSingleCondition($rules, $context);
        }

        $results = [];
        foreach ($conditions as $condition) {
            // Handle nested rule groups
            if (isset($condition['operator']) && isset($condition['conditions'])) {
                $results[] = $this->evaluateTargetingRules($condition, $context);
            } else {
                $results[] = $this->evaluateSingleCondition($condition, $context);
            }
        }

        return $operator === 'AND'
            ? !in_array(false, $results, true)
            : in_array(true, $results, true);
    }

    /**
     * Evaluate a single targeting condition
     */
    private function evaluateSingleCondition(array $condition, array $context): bool
    {
        $field = $condition['field'] ?? null;
        $operator = $condition['operator'] ?? 'eq';
        $value = $condition['value'] ?? null;

        if (!$field) {
            return true;
        }

        // Get field value from context using dot notation
        $fieldValue = data_get($context, $field);

        return match ($operator) {
            'eq', 'equals' => $fieldValue == $value,
            'neq', 'not_equals' => $fieldValue != $value,
            'gt', 'greater_than' => $fieldValue > $value,
            'gte', 'greater_than_or_equals' => $fieldValue >= $value,
            'lt', 'less_than' => $fieldValue < $value,
            'lte', 'less_than_or_equals' => $fieldValue <= $value,
            'in' => in_array($fieldValue, (array) $value, false),
            'not_in' => !in_array($fieldValue, (array) $value, false),
            'contains' => is_string($fieldValue) && str_contains($fieldValue, $value),
            'not_contains' => is_string($fieldValue) && !str_contains($fieldValue, $value),
            'starts_with' => is_string($fieldValue) && str_starts_with($fieldValue, $value),
            'ends_with' => is_string($fieldValue) && str_ends_with($fieldValue, $value),
            'regex' => is_string($fieldValue) && preg_match($value, $fieldValue),
            'is_null' => $fieldValue === null,
            'is_not_null' => $fieldValue !== null,
            'is_true' => $fieldValue === true || $fieldValue === 'true' || $fieldValue === 1,
            'is_false' => $fieldValue === false || $fieldValue === 'false' || $fieldValue === 0,
            'before' => $fieldValue && \Carbon\Carbon::parse($fieldValue)->isBefore(\Carbon\Carbon::parse($value)),
            'after' => $fieldValue && \Carbon\Carbon::parse($fieldValue)->isAfter(\Carbon\Carbon::parse($value)),
            default => $fieldValue == $value,
        };
    }

    // =========================================================================
    // ADMIN ENDPOINTS
    // =========================================================================

    /**
     * List all experiments (admin)
     */
    public function index(Request $request): JsonResponse
    {
        $experiments = Experiment::query()
            ->when($request->input('status'), function ($query, $status) {
                if ($status === 'active') {
                    $query->where('enabled', true);
                } elseif ($status === 'completed') {
                    $query->where('enabled', false);
                }
            })
            ->orderBy('created_at', 'desc')
            ->paginate($request->input('per_page', 20));

        return response()->json($experiments);
    }

    /**
     * Create a new experiment (admin)
     */
    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'key' => 'required|string|max:100|unique:experiments,key',
            'description' => 'nullable|string',
            'variants' => 'required|array|min:2',
            'variants.*' => 'string|max:50',
            'variant_weights' => 'nullable|array',
            'default_variant' => 'required|string|max:50',
            'rollout_percentage' => 'integer|min:0|max:100',
            'enabled' => 'boolean',
            'starts_at' => 'nullable|date',
            'ends_at' => 'nullable|date|after:starts_at',
        ]);

        $experiment = Experiment::create($validated);

        Cache::forget('experiments:active');

        return response()->json($experiment, 201);
    }

    /**
     * Get experiment details with analytics (admin)
     */
    public function show(string $id): JsonResponse
    {
        $experiment = Experiment::findOrFail($id);

        // Get assignment stats
        $stats = ExperimentAssignment::where('experiment_key', $experiment->key)
            ->selectRaw('variant, COUNT(*) as count, COUNT(converted_at) as conversions')
            ->groupBy('variant')
            ->get()
            ->keyBy('variant');

        return response()->json([
            'experiment' => $experiment,
            'stats' => $stats,
        ]);
    }

    /**
     * Update an experiment (admin)
     */
    public function update(Request $request, string $id): JsonResponse
    {
        $experiment = Experiment::findOrFail($id);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'description' => 'nullable|string',
            'rollout_percentage' => 'integer|min:0|max:100',
            'enabled' => 'boolean',
            'ends_at' => 'nullable|date',
        ]);

        $experiment->update($validated);

        Cache::forget('experiments:active');

        return response()->json($experiment);
    }

    /**
     * Delete an experiment (admin)
     */
    public function destroy(string $id): JsonResponse
    {
        $experiment = Experiment::findOrFail($id);
        $experiment->delete();

        Cache::forget('experiments:active');

        return response()->json(['success' => true]);
    }

    // =========================================================================
    // FEATURE FLAGS ADMIN
    // =========================================================================

    /**
     * List all feature flags (admin)
     */
    public function listFlags(Request $request): JsonResponse
    {
        $flags = FeatureFlag::query()
            ->orderBy('key')
            ->paginate($request->input('per_page', 50));

        return response()->json($flags);
    }

    /**
     * Create a feature flag (admin)
     */
    public function storeFlag(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'key' => 'required|string|max:100|unique:feature_flags,key',
            'name' => 'required|string|max:255',
            'description' => 'nullable|string',
            'enabled' => 'boolean',
            'rollout_percentage' => 'integer|min:0|max:100',
            'targeting_rules' => 'nullable|array',
        ]);

        $flag = FeatureFlag::create($validated);

        Cache::forget('feature_flags:active');

        return response()->json($flag, 201);
    }

    /**
     * Update a feature flag (admin)
     */
    public function updateFlag(Request $request, string $id): JsonResponse
    {
        $flag = FeatureFlag::findOrFail($id);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'description' => 'nullable|string',
            'enabled' => 'boolean',
            'rollout_percentage' => 'integer|min:0|max:100',
            'targeting_rules' => 'nullable|array',
        ]);

        $flag->update($validated);

        Cache::forget('feature_flags:active');

        return response()->json($flag);
    }

    /**
     * Delete a feature flag (admin)
     */
    public function destroyFlag(string $id): JsonResponse
    {
        $flag = FeatureFlag::findOrFail($id);
        $flag->delete();

        Cache::forget('feature_flags:active');

        return response()->json(['success' => true]);
    }
}
