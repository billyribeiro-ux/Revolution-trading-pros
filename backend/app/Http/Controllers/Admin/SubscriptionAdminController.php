<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Subscription;
use App\Models\Plan;
use App\Models\User;
use App\Models\Invoice;
use App\Models\Payment;
use App\Services\Subscription\UsageTracker;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Carbon\Carbon;

/**
 * Subscription Admin Controller (ICT9+ Enterprise Grade)
 *
 * Admin API for subscription management:
 * - Dashboard analytics
 * - Subscription management
 * - Revenue reports
 * - Customer insights
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class SubscriptionAdminController extends Controller
{
    public function __construct(
        private UsageTracker $usageTracker
    ) {}

    /**
     * Get subscription dashboard overview
     */
    public function dashboard(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        [$startDate, $endDate] = $this->getPeriodDates($period);

        return response()->json([
            'overview' => $this->getOverviewMetrics($startDate, $endDate),
            'mrr' => $this->getMrrData($startDate, $endDate),
            'churn' => $this->getChurnData($startDate, $endDate),
            'plans' => $this->getPlanDistribution(),
            'recent_subscriptions' => $this->getRecentSubscriptions(10),
            'revenue_chart' => $this->getRevenueChart($period),
        ]);
    }

    /**
     * Get overview metrics
     */
    private function getOverviewMetrics(Carbon $startDate, Carbon $endDate): array
    {
        // Current MRR
        $mrr = Subscription::where('status', 'active')
            ->with('plan')
            ->get()
            ->sum(fn($sub) => $sub->plan?->monthly_price ?? 0);

        // Previous period MRR for comparison
        $previousStart = $startDate->copy()->subDays($startDate->diffInDays($endDate));
        $previousMrr = Subscription::where('status', 'active')
            ->where('created_at', '<', $startDate)
            ->with('plan')
            ->get()
            ->sum(fn($sub) => $sub->plan?->monthly_price ?? 0);

        // Active subscriptions
        $activeSubscriptions = Subscription::where('status', 'active')->count();

        // New subscriptions in period
        $newSubscriptions = Subscription::whereBetween('created_at', [$startDate, $endDate])->count();

        // Churned subscriptions in period
        $churned = Subscription::whereBetween('canceled_at', [$startDate, $endDate])->count();

        // Trial conversions
        $trialConversions = Subscription::where('status', 'active')
            ->whereNotNull('trial_ends_at')
            ->whereBetween('updated_at', [$startDate, $endDate])
            ->count();

        // Revenue in period
        $revenue = Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        return [
            'mrr' => [
                'value' => $mrr,
                'formatted' => '$' . number_format($mrr / 100, 2),
                'change' => $previousMrr > 0 ? round((($mrr - $previousMrr) / $previousMrr) * 100, 1) : 0,
            ],
            'active_subscriptions' => [
                'value' => $activeSubscriptions,
                'change' => $newSubscriptions - $churned,
            ],
            'new_subscriptions' => [
                'value' => $newSubscriptions,
            ],
            'churned' => [
                'value' => $churned,
                'rate' => $activeSubscriptions > 0 ? round(($churned / $activeSubscriptions) * 100, 2) : 0,
            ],
            'trial_conversions' => [
                'value' => $trialConversions,
            ],
            'revenue' => [
                'value' => $revenue,
                'formatted' => '$' . number_format($revenue / 100, 2),
            ],
        ];
    }

    /**
     * Get MRR data
     */
    private function getMrrData(Carbon $startDate, Carbon $endDate): array
    {
        $data = [];
        $current = $startDate->copy();

        while ($current <= $endDate) {
            $mrr = Subscription::where('status', 'active')
                ->where('created_at', '<=', $current)
                ->where(function ($q) use ($current) {
                    $q->whereNull('canceled_at')
                        ->orWhere('canceled_at', '>', $current);
                })
                ->with('plan')
                ->get()
                ->sum(fn($sub) => $sub->plan?->monthly_price ?? 0);

            $data[] = [
                'date' => $current->format('Y-m-d'),
                'mrr' => $mrr,
            ];

            $current->addDay();
        }

        return $data;
    }

    /**
     * Get churn data
     */
    private function getChurnData(Carbon $startDate, Carbon $endDate): array
    {
        $churned = Subscription::whereBetween('canceled_at', [$startDate, $endDate])
            ->select(
                DB::raw('DATE(canceled_at) as date'),
                DB::raw('COUNT(*) as count'),
                'cancellation_reason'
            )
            ->groupBy('date', 'cancellation_reason')
            ->get();

        $reasons = Subscription::whereBetween('canceled_at', [$startDate, $endDate])
            ->select('cancellation_reason', DB::raw('COUNT(*) as count'))
            ->groupBy('cancellation_reason')
            ->pluck('count', 'cancellation_reason')
            ->toArray();

        return [
            'total' => $churned->sum('count'),
            'by_date' => $churned->groupBy('date'),
            'by_reason' => $reasons,
        ];
    }

    /**
     * Get plan distribution
     */
    private function getPlanDistribution(): array
    {
        return Subscription::where('status', 'active')
            ->select('plan_id', DB::raw('COUNT(*) as count'))
            ->groupBy('plan_id')
            ->with('plan:id,name,slug')
            ->get()
            ->map(fn($item) => [
                'plan' => $item->plan?->name ?? 'Unknown',
                'slug' => $item->plan?->slug ?? 'unknown',
                'count' => $item->count,
            ])
            ->toArray();
    }

    /**
     * Get recent subscriptions
     */
    private function getRecentSubscriptions(int $limit = 10): array
    {
        return Subscription::with(['user:id,name,email', 'plan:id,name'])
            ->orderBy('created_at', 'desc')
            ->limit($limit)
            ->get()
            ->map(fn($sub) => [
                'id' => $sub->id,
                'user' => $sub->user?->name,
                'email' => $sub->user?->email,
                'plan' => $sub->plan?->name,
                'status' => $sub->status,
                'created_at' => $sub->created_at->toIso8601String(),
            ])
            ->toArray();
    }

    /**
     * Get revenue chart data
     */
    private function getRevenueChart(string $period): array
    {
        $days = match ($period) {
            '7d' => 7,
            '30d' => 30,
            '90d' => 90,
            '365d' => 365,
            default => 30,
        };

        $data = Invoice::where('status', 'paid')
            ->where('paid_at', '>=', now()->subDays($days))
            ->select(
                DB::raw('DATE(paid_at) as date'),
                DB::raw('SUM(amount) as revenue'),
                DB::raw('COUNT(*) as transactions')
            )
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return $data->map(fn($item) => [
            'date' => $item->date,
            'revenue' => $item->revenue,
            'transactions' => $item->transactions,
        ])->toArray();
    }

    /**
     * List all subscriptions
     */
    public function index(Request $request): JsonResponse
    {
        $query = Subscription::with(['user:id,name,email', 'plan:id,name,slug']);

        // Filters
        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }

        if ($planId = $request->get('plan_id')) {
            $query->where('plan_id', $planId);
        }

        if ($search = $request->get('search')) {
            $query->whereHas('user', function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                    ->orWhere('email', 'like', "%{$search}%");
            });
        }

        // Sorting
        $sortBy = $request->get('sort_by', 'created_at');
        $sortDir = $request->get('sort_dir', 'desc');
        $query->orderBy($sortBy, $sortDir);

        // Pagination
        $perPage = min($request->get('per_page', 25), 100);
        $subscriptions = $query->paginate($perPage);

        return response()->json($subscriptions);
    }

    /**
     * Get single subscription
     */
    public function show(Subscription $subscription): JsonResponse
    {
        $subscription->load(['user', 'plan', 'invoices' => function ($q) {
            $q->latest()->limit(10);
        }]);

        // Get usage data
        $usage = $this->usageTracker->getUsageReport($subscription->user);

        return response()->json([
            'subscription' => $subscription,
            'usage' => $usage,
            'lifetime_value' => $this->calculateLifetimeValue($subscription),
        ]);
    }

    /**
     * Update subscription
     */
    public function update(Request $request, Subscription $subscription): JsonResponse
    {
        $validated = $request->validate([
            'status' => 'sometimes|in:active,paused,past_due,canceled',
            'plan_id' => 'sometimes|exists:plans,id',
            'trial_ends_at' => 'sometimes|nullable|date',
            'cancel_at_period_end' => 'sometimes|boolean',
        ]);

        $subscription->update($validated);

        return response()->json([
            'message' => 'Subscription updated',
            'subscription' => $subscription->fresh(['user', 'plan']),
        ]);
    }

    /**
     * Cancel subscription
     */
    public function cancel(Request $request, Subscription $subscription): JsonResponse
    {
        $immediately = $request->boolean('immediately', false);
        $reason = $request->get('reason', 'Admin cancellation');

        if ($immediately) {
            $subscription->update([
                'status' => 'canceled',
                'canceled_at' => now(),
                'ends_at' => now(),
                'cancellation_reason' => $reason,
            ]);
        } else {
            $subscription->update([
                'cancel_at_period_end' => true,
                'cancellation_reason' => $reason,
            ]);
        }

        return response()->json([
            'message' => 'Subscription canceled',
            'subscription' => $subscription->fresh(),
        ]);
    }

    /**
     * Reactivate subscription
     */
    public function reactivate(Subscription $subscription): JsonResponse
    {
        if (!in_array($subscription->status, ['canceled', 'suspended', 'expired'])) {
            return response()->json([
                'error' => 'Subscription cannot be reactivated',
            ], 400);
        }

        $subscription->update([
            'status' => 'active',
            'canceled_at' => null,
            'ends_at' => null,
            'cancel_at_period_end' => false,
            'suspended_at' => null,
        ]);

        return response()->json([
            'message' => 'Subscription reactivated',
            'subscription' => $subscription->fresh(),
        ]);
    }

    /**
     * Extend trial
     */
    public function extendTrial(Request $request, Subscription $subscription): JsonResponse
    {
        $days = $request->validate(['days' => 'required|integer|min:1|max:90'])['days'];

        $newTrialEnd = ($subscription->trial_ends_at ?? now())->addDays($days);

        $subscription->update([
            'status' => 'trialing',
            'trial_ends_at' => $newTrialEnd,
        ]);

        return response()->json([
            'message' => "Trial extended by {$days} days",
            'subscription' => $subscription->fresh(),
        ]);
    }

    /**
     * Calculate lifetime value
     */
    private function calculateLifetimeValue(Subscription $subscription): array
    {
        $totalPaid = Invoice::where('user_id', $subscription->user_id)
            ->where('status', 'paid')
            ->sum('amount');

        $monthsActive = $subscription->created_at->diffInMonths(now()) ?: 1;
        $avgMonthlyRevenue = $totalPaid / $monthsActive;

        return [
            'total_paid' => $totalPaid,
            'formatted' => '$' . number_format($totalPaid / 100, 2),
            'months_active' => $monthsActive,
            'avg_monthly' => '$' . number_format($avgMonthlyRevenue / 100, 2),
        ];
    }

    /**
     * Get period dates from string
     */
    private function getPeriodDates(string $period): array
    {
        $days = match ($period) {
            '7d' => 7,
            '30d' => 30,
            '90d' => 90,
            '365d' => 365,
            default => 30,
        };

        return [now()->subDays($days), now()];
    }
}
