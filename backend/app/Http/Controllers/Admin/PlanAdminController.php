<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Plan;
use App\Models\Subscription;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Plan Admin Controller (ICT9+ Enterprise Grade)
 *
 * Admin API for plan management:
 * - CRUD operations
 * - Feature management
 * - Pricing management
 * - Plan analytics
 *
 * @version 1.0.0
 */
class PlanAdminController extends Controller
{
    /**
     * List all plans
     */
    public function index(Request $request): JsonResponse
    {
        $query = Plan::query();

        if ($request->boolean('active_only')) {
            $query->where('is_active', true);
        }

        $query->withCount(['subscriptions', 'subscriptions as active_subscriptions_count' => function ($q) {
            $q->where('status', 'active');
        }]);

        $plans = $query->orderBy('sort_order')->get();

        return response()->json([
            'plans' => $plans,
            'stats' => $this->getPlanStats(),
        ]);
    }

    /**
     * Get single plan
     */
    public function show(Plan $plan): JsonResponse
    {
        $plan->loadCount(['subscriptions', 'subscriptions as active_subscriptions_count' => function ($q) {
            $q->where('status', 'active');
        }]);

        return response()->json([
            'plan' => $plan,
            'mrr' => $this->calculatePlanMrr($plan),
            'conversion_rate' => $this->calculateConversionRate($plan),
        ]);
    }

    /**
     * Create new plan
     */
    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:plans,slug',
            'description' => 'nullable|string',
            'monthly_price' => 'required|integer|min:0',
            'yearly_price' => 'nullable|integer|min:0',
            'currency' => 'sometimes|string|size:3',
            'features' => 'sometimes|array',
            'limits' => 'sometimes|array',
            'trial_days' => 'sometimes|integer|min:0',
            'is_active' => 'sometimes|boolean',
            'is_featured' => 'sometimes|boolean',
            'sort_order' => 'sometimes|integer',
            'stripe_price_id' => 'nullable|string',
            'stripe_yearly_price_id' => 'nullable|string',
            'paddle_price_id' => 'nullable|string',
            'paypal_plan_id' => 'nullable|string',
        ]);

        // Generate slug if not provided
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        // Set defaults
        $validated['currency'] = $validated['currency'] ?? 'USD';
        $validated['trial_days'] = $validated['trial_days'] ?? 0;
        $validated['is_active'] = $validated['is_active'] ?? true;

        $plan = Plan::create($validated);

        return response()->json([
            'message' => 'Plan created successfully',
            'plan' => $plan,
        ], 201);
    }

    /**
     * Update plan
     */
    public function update(Request $request, Plan $plan): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'slug' => 'sometimes|string|max:255|unique:plans,slug,' . $plan->id,
            'description' => 'nullable|string',
            'monthly_price' => 'sometimes|integer|min:0',
            'yearly_price' => 'nullable|integer|min:0',
            'currency' => 'sometimes|string|size:3',
            'features' => 'sometimes|array',
            'limits' => 'sometimes|array',
            'trial_days' => 'sometimes|integer|min:0',
            'is_active' => 'sometimes|boolean',
            'is_featured' => 'sometimes|boolean',
            'sort_order' => 'sometimes|integer',
            'stripe_price_id' => 'nullable|string',
            'stripe_yearly_price_id' => 'nullable|string',
            'paddle_price_id' => 'nullable|string',
            'paypal_plan_id' => 'nullable|string',
        ]);

        $plan->update($validated);

        return response()->json([
            'message' => 'Plan updated successfully',
            'plan' => $plan->fresh(),
        ]);
    }

    /**
     * Delete plan (soft delete or archive)
     */
    public function destroy(Plan $plan): JsonResponse
    {
        // Check if plan has active subscriptions
        $activeCount = $plan->subscriptions()->where('status', 'active')->count();

        if ($activeCount > 0) {
            return response()->json([
                'error' => 'Cannot delete plan with active subscriptions',
                'active_subscriptions' => $activeCount,
            ], 400);
        }

        // Soft delete by marking inactive
        $plan->update(['is_active' => false]);

        return response()->json([
            'message' => 'Plan archived successfully',
        ]);
    }

    /**
     * Duplicate plan
     */
    public function duplicate(Plan $plan): JsonResponse
    {
        $newPlan = $plan->replicate();
        $newPlan->name = $plan->name . ' (Copy)';
        $newPlan->slug = $plan->slug . '-copy-' . time();
        $newPlan->is_active = false;
        $newPlan->stripe_price_id = null;
        $newPlan->stripe_yearly_price_id = null;
        $newPlan->paddle_price_id = null;
        $newPlan->paypal_plan_id = null;
        $newPlan->save();

        return response()->json([
            'message' => 'Plan duplicated successfully',
            'plan' => $newPlan,
        ], 201);
    }

    /**
     * Update plan features
     */
    public function updateFeatures(Request $request, Plan $plan): JsonResponse
    {
        $validated = $request->validate([
            'features' => 'required|array',
        ]);

        $plan->update(['features' => $validated['features']]);

        return response()->json([
            'message' => 'Features updated successfully',
            'plan' => $plan->fresh(),
        ]);
    }

    /**
     * Update plan limits
     */
    public function updateLimits(Request $request, Plan $plan): JsonResponse
    {
        $validated = $request->validate([
            'limits' => 'required|array',
        ]);

        $plan->update(['limits' => $validated['limits']]);

        return response()->json([
            'message' => 'Limits updated successfully',
            'plan' => $plan->fresh(),
        ]);
    }

    /**
     * Reorder plans
     */
    public function reorder(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'order' => 'required|array',
            'order.*' => 'integer|exists:plans,id',
        ]);

        foreach ($validated['order'] as $index => $planId) {
            Plan::where('id', $planId)->update(['sort_order' => $index]);
        }

        return response()->json([
            'message' => 'Plans reordered successfully',
        ]);
    }

    /**
     * Get plan statistics
     */
    private function getPlanStats(): array
    {
        return [
            'total_plans' => Plan::count(),
            'active_plans' => Plan::where('is_active', true)->count(),
            'total_subscriptions' => Subscription::count(),
            'active_subscriptions' => Subscription::where('status', 'active')->count(),
        ];
    }

    /**
     * Calculate MRR for a plan
     */
    private function calculatePlanMrr(Plan $plan): int
    {
        return Subscription::where('plan_id', $plan->id)
            ->where('status', 'active')
            ->count() * ($plan->monthly_price ?? 0);
    }

    /**
     * Calculate conversion rate for a plan
     */
    private function calculateConversionRate(Plan $plan): float
    {
        $trials = Subscription::where('plan_id', $plan->id)
            ->whereNotNull('trial_ends_at')
            ->count();

        if ($trials === 0) {
            return 0;
        }

        $converted = Subscription::where('plan_id', $plan->id)
            ->whereNotNull('trial_ends_at')
            ->where('status', 'active')
            ->count();

        return round(($converted / $trials) * 100, 2);
    }
}
