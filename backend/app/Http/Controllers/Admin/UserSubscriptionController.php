<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\UserSubscription;
use App\Models\SubscriptionPlan;
use App\Models\User;
use Illuminate\Http\Request;
use Carbon\Carbon;

class UserSubscriptionController extends Controller
{
    public function index(Request $request)
    {
        $query = UserSubscription::with(['user', 'plan']);

        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        if ($request->has('user_id')) {
            $query->where('user_id', $request->user_id);
        }

        if ($request->has('plan_id')) {
            $query->where('subscription_plan_id', $request->plan_id);
        }

        $subscriptions = $query->orderBy('created_at', 'desc')
            ->paginate($request->get('per_page', 20));

        return response()->json($subscriptions);
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'user_id' => 'required|exists:users,id',
            'subscription_plan_id' => 'required|exists:subscription_plans,id',
            'status' => 'required|in:active,paused,cancelled,expired,pending,trial',
            'trial_days' => 'nullable|integer|min:0',
            'payment_method' => 'nullable|string',
            'payment_id' => 'nullable|string',
            'amount_paid' => 'nullable|numeric|min:0',
            'notes' => 'nullable|string',
            'start_immediately' => 'boolean',
        ]);

        $plan = SubscriptionPlan::findOrFail($validated['subscription_plan_id']);
        $user = User::findOrFail($validated['user_id']);

        $existing = UserSubscription::where('user_id', $user->id)
            ->where('subscription_plan_id', $plan->id)
            ->whereIn('status', ['active', 'trial'])
            ->first();

        if ($existing) {
            return response()->json([
                'message' => 'User already has an active subscription to this plan',
            ], 422);
        }

        $trialDays = $validated['trial_days'] ?? $plan->trial_days;
        $startDate = $request->get('start_immediately', true) ? now() : null;

        $subscription = UserSubscription::create([
            'user_id' => $user->id,
            'subscription_plan_id' => $plan->id,
            'status' => $validated['status'],
            'trial_ends_at' => $trialDays > 0 ? now()->addDays($trialDays) : null,
            'current_period_start' => $startDate,
            'current_period_end' => $startDate ? $this->calculatePeriodEnd($startDate, $plan) : null,
            'payment_method' => $validated['payment_method'] ?? 'manual',
            'payment_id' => $validated['payment_id'] ?? null,
            'amount_paid' => $validated['amount_paid'] ?? 0,
            'notes' => $validated['notes'] ?? null,
        ]);

        return response()->json([
            'message' => 'Subscription created successfully',
            'subscription' => $subscription->load(['user', 'plan']),
        ], 201);
    }

    public function show(string $id)
    {
        $subscription = UserSubscription::with(['user', 'plan', 'payments'])
            ->findOrFail($id);

        return response()->json($subscription);
    }

    public function update(Request $request, string $id)
    {
        $subscription = UserSubscription::findOrFail($id);

        $validated = $request->validate([
            'status' => 'sometimes|in:active,paused,cancelled,expired,pending,trial',
            'trial_ends_at' => 'nullable|date',
            'current_period_start' => 'nullable|date',
            'current_period_end' => 'nullable|date',
            'payment_method' => 'nullable|string',
            'payment_id' => 'nullable|string',
            'amount_paid' => 'nullable|numeric|min:0',
            'notes' => 'nullable|string',
        ]);

        $subscription->update($validated);

        return response()->json([
            'message' => 'Subscription updated successfully',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    public function cancel(string $id)
    {
        $subscription = UserSubscription::findOrFail($id);
        $subscription->cancel();

        return response()->json([
            'message' => 'Subscription cancelled successfully',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    public function pause(string $id)
    {
        $subscription = UserSubscription::findOrFail($id);
        $subscription->pause();

        return response()->json([
            'message' => 'Subscription paused successfully',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    public function resume(string $id)
    {
        $subscription = UserSubscription::findOrFail($id);
        $subscription->resume();

        return response()->json([
            'message' => 'Subscription resumed successfully',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    public function renew(string $id)
    {
        $subscription = UserSubscription::findOrFail($id);
        $subscription->renew();

        return response()->json([
            'message' => 'Subscription renewed successfully',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    public function destroy(string $id)
    {
        $subscription = UserSubscription::findOrFail($id);
        $subscription->delete();

        return response()->json([
            'message' => 'Subscription deleted successfully',
        ]);
    }

    public function userSubscriptions(string $userId)
    {
        $subscriptions = UserSubscription::with('plan')
            ->where('user_id', $userId)
            ->orderBy('created_at', 'desc')
            ->get();

        return response()->json($subscriptions);
    }

    protected function calculatePeriodEnd(Carbon $start, SubscriptionPlan $plan)
    {
        $interval = $plan->billing_interval;
        
        return match($plan->billing_period) {
            'daily' => $start->copy()->addDays($interval),
            'weekly' => $start->copy()->addWeeks($interval),
            'monthly' => $start->copy()->addMonths($interval),
            'quarterly' => $start->copy()->addMonths($interval * 3),
            'yearly' => $start->copy()->addYears($interval),
            default => $start->copy()->addMonth(),
        };
    }
}