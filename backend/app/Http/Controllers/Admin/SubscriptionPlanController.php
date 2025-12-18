<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\SubscriptionPlan;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class SubscriptionPlanController extends Controller
{
    public function index()
    {
        $plans = SubscriptionPlan::withCount(['subscriptions' => function ($query) {
            $query->where('status', 'active');
        }])
            ->orderBy('sort_order')
            ->orderBy('created_at', 'desc')
            ->get();

        return response()->json($plans);
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:subscription_plans,slug',
            'description' => 'nullable|string',
            'price' => 'required|numeric|min:0',
            'billing_period' => 'required|in:daily,weekly,monthly,quarterly,yearly',
            'billing_interval' => 'required|integer|min:1',
            'trial_days' => 'nullable|integer|min:0',
            'signup_fee' => 'nullable|numeric|min:0',
            'max_users' => 'nullable|integer|min:1',
            'features' => 'nullable|array',
            'is_active' => 'boolean',
            'is_featured' => 'boolean',
            'sort_order' => 'nullable|integer',
        ]);

        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $plan = SubscriptionPlan::create($validated);

        return response()->json([
            'message' => 'Subscription plan created successfully',
            'plan' => $plan,
        ], 201);
    }

    public function show(string $id)
    {
        $plan = SubscriptionPlan::with('planFeatures')
            ->withCount(['subscriptions' => function ($query) {
                $query->where('status', 'active');
            }])
            ->findOrFail($id);

        return response()->json($plan);
    }

    public function update(Request $request, string $id)
    {
        $plan = SubscriptionPlan::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'slug' => 'sometimes|string|max:255|unique:subscription_plans,slug,' . $plan->id,
            'description' => 'nullable|string',
            'price' => 'sometimes|numeric|min:0',
            'billing_period' => 'sometimes|in:daily,weekly,monthly,quarterly,yearly',
            'billing_interval' => 'sometimes|integer|min:1',
            'trial_days' => 'nullable|integer|min:0',
            'signup_fee' => 'nullable|numeric|min:0',
            'max_users' => 'nullable|integer|min:1',
            'features' => 'nullable|array',
            'is_active' => 'boolean',
            'is_featured' => 'boolean',
            'sort_order' => 'nullable|integer',
        ]);

        $plan->update($validated);

        return response()->json([
            'message' => 'Subscription plan updated successfully',
            'plan' => $plan->fresh(),
        ]);
    }

    public function destroy(string $id)
    {
        $plan = SubscriptionPlan::findOrFail($id);

        $activeCount = $plan->subscriptions()->where('status', 'active')->count();
        if ($activeCount > 0) {
            return response()->json([
                'message' => "Cannot delete plan with {$activeCount} active subscription(s). Please cancel or migrate them first.",
            ], 422);
        }

        $plan->delete();

        return response()->json([
            'message' => 'Subscription plan deleted successfully',
        ]);
    }

    public function stats()
    {
        $totalPlans = SubscriptionPlan::count();
        $activePlans = SubscriptionPlan::where('is_active', true)->count();
        $totalSubscriptions = \App\Models\UserSubscription::count();
        $activeSubscriptions = \App\Models\UserSubscription::where('status', 'active')->count();
        $monthlyRevenue = \App\Models\UserSubscription::where('status', 'active')
            ->join('subscription_plans', 'user_subscriptions.subscription_plan_id', '=', 'subscription_plans.id')
            ->where('subscription_plans.billing_period', 'monthly')
            ->sum('subscription_plans.price');

        return response()->json([
            'total_plans' => $totalPlans,
            'active_plans' => $activePlans,
            'total_subscriptions' => $totalSubscriptions,
            'active_subscriptions' => $activeSubscriptions,
            'monthly_revenue' => $monthlyRevenue,
        ]);
    }
}
