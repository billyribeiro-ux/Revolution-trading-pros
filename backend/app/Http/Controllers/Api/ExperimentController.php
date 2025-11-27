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

        // Check targeting rules
        if ($flag->targeting_rules && is_array($flag->targeting_rules)) {
            // TODO: Implement targeting rule evaluation
        }

        return true;
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
