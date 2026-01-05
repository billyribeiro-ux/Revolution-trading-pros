<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Invoice;
use App\Models\Payment;
use App\Models\Subscription;
use App\Models\Plan;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Carbon\Carbon;

/**
 * Revenue Report Controller (ICT9+ Enterprise Grade)
 *
 * Comprehensive revenue analytics:
 * - MRR/ARR tracking
 * - Revenue breakdown
 * - Cohort analysis
 * - Forecasting
 *
 * @version 1.0.0
 */
class RevenueReportController extends Controller
{
    /**
     * Get comprehensive revenue report
     */
    public function index(Request $request): JsonResponse
    {
        $startDate = $request->get('start_date')
            ? Carbon::parse($request->get('start_date'))
            : now()->subDays(30);

        $endDate = $request->get('end_date')
            ? Carbon::parse($request->get('end_date'))
            : now();

        return response()->json([
            'summary' => $this->getSummary($startDate, $endDate),
            'mrr_history' => $this->getMrrHistory($startDate, $endDate),
            'revenue_by_plan' => $this->getRevenueByPlan($startDate, $endDate),
            'revenue_by_source' => $this->getRevenueBySource($startDate, $endDate),
            'refunds' => $this->getRefundData($startDate, $endDate),
            'ltv' => $this->getLtvData(),
            'forecast' => $this->getForecast(),
        ]);
    }

    /**
     * Get summary metrics
     */
    private function getSummary(Carbon $startDate, Carbon $endDate): array
    {
        // Current MRR
        $mrr = $this->calculateMrr();

        // ARR
        $arr = $mrr * 12;

        // Revenue in period
        $revenue = Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        // Previous period revenue
        $prevPeriodDays = $startDate->diffInDays($endDate);
        $prevStartDate = $startDate->copy()->subDays($prevPeriodDays);
        $prevRevenue = Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$prevStartDate, $startDate])
            ->sum('amount');

        // Growth rate
        $growthRate = $prevRevenue > 0
            ? round((($revenue - $prevRevenue) / $prevRevenue) * 100, 2)
            : 0;

        // Average revenue per user (ARPU)
        $activeUsers = Subscription::where('status', 'active')->count();
        $arpu = $activeUsers > 0 ? round($mrr / $activeUsers) : 0;

        return [
            'mrr' => [
                'value' => $mrr,
                'formatted' => '$' . number_format($mrr / 100, 2),
            ],
            'arr' => [
                'value' => $arr,
                'formatted' => '$' . number_format($arr / 100, 2),
            ],
            'period_revenue' => [
                'value' => $revenue,
                'formatted' => '$' . number_format($revenue / 100, 2),
                'growth_rate' => $growthRate,
            ],
            'arpu' => [
                'value' => $arpu,
                'formatted' => '$' . number_format($arpu / 100, 2),
            ],
            'active_subscriptions' => $activeUsers,
        ];
    }

    /**
     * Calculate current MRR
     */
    private function calculateMrr(): int
    {
        return Subscription::where('status', 'active')
            ->join('plans', 'subscriptions.plan_id', '=', 'plans.id')
            ->sum('plans.monthly_price');
    }

    /**
     * Get MRR history
     */
    private function getMrrHistory(Carbon $startDate, Carbon $endDate): array
    {
        $data = [];
        $current = $startDate->copy();

        while ($current <= $endDate) {
            $mrr = DB::table('subscriptions')
                ->join('plans', 'subscriptions.plan_id', '=', 'plans.id')
                ->where('subscriptions.status', 'active')
                ->where('subscriptions.created_at', '<=', $current)
                ->where(function ($q) use ($current) {
                    $q->whereNull('subscriptions.canceled_at')
                        ->orWhere('subscriptions.canceled_at', '>', $current);
                })
                ->sum('plans.monthly_price');

            $data[] = [
                'date' => $current->format('Y-m-d'),
                'mrr' => $mrr,
            ];

            $current->addDay();
        }

        return $data;
    }

    /**
     * Get revenue by plan
     */
    private function getRevenueByPlan(Carbon $startDate, Carbon $endDate): array
    {
        return Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->join('subscriptions', 'invoices.subscription_id', '=', 'subscriptions.id')
            ->join('plans', 'subscriptions.plan_id', '=', 'plans.id')
            ->select('plans.name', 'plans.slug', DB::raw('SUM(invoices.amount) as total'))
            ->groupBy('plans.id', 'plans.name', 'plans.slug')
            ->orderByDesc('total')
            ->get()
            ->map(fn($item) => [
                'plan' => $item->name,
                'slug' => $item->slug,
                'revenue' => $item->total,
                'formatted' => '$' . number_format($item->total / 100, 2),
            ])
            ->toArray();
    }

    /**
     * Get revenue by source
     */
    private function getRevenueBySource(Carbon $startDate, Carbon $endDate): array
    {
        // New subscriptions
        $newRevenue = Invoice::where('status', 'paid')
            ->where('billing_reason', 'subscription_create')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        // Renewals
        $renewalRevenue = Invoice::where('status', 'paid')
            ->where('billing_reason', 'subscription_cycle')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        // Upgrades
        $upgradeRevenue = Invoice::where('status', 'paid')
            ->where('billing_reason', 'subscription_update')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        // One-time
        $oneTimeRevenue = Invoice::where('status', 'paid')
            ->whereNull('subscription_id')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        return [
            'new_subscriptions' => [
                'revenue' => $newRevenue,
                'formatted' => '$' . number_format($newRevenue / 100, 2),
            ],
            'renewals' => [
                'revenue' => $renewalRevenue,
                'formatted' => '$' . number_format($renewalRevenue / 100, 2),
            ],
            'upgrades' => [
                'revenue' => $upgradeRevenue,
                'formatted' => '$' . number_format($upgradeRevenue / 100, 2),
            ],
            'one_time' => [
                'revenue' => $oneTimeRevenue,
                'formatted' => '$' . number_format($oneTimeRevenue / 100, 2),
            ],
        ];
    }

    /**
     * Get refund data
     */
    private function getRefundData(Carbon $startDate, Carbon $endDate): array
    {
        $refunds = Payment::where('status', 'refunded')
            ->whereBetween('updated_at', [$startDate, $endDate])
            ->sum('refunded_amount');

        $totalRevenue = Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->sum('amount');

        return [
            'total_refunded' => $refunds,
            'formatted' => '$' . number_format($refunds / 100, 2),
            'refund_rate' => $totalRevenue > 0 ? round(($refunds / $totalRevenue) * 100, 2) : 0,
            'count' => Payment::where('status', 'refunded')
                ->whereBetween('updated_at', [$startDate, $endDate])
                ->count(),
        ];
    }

    /**
     * Get LTV data
     */
    private function getLtvData(): array
    {
        // Average subscription lifetime in months
        $avgLifetimeMonths = DB::table('subscriptions')
            ->whereNotNull('canceled_at')
            ->selectRaw('AVG(TIMESTAMPDIFF(MONTH, created_at, canceled_at)) as avg_months')
            ->value('avg_months') ?? 12;

        // Average monthly revenue
        $avgMonthlyRevenue = DB::table('invoices')
            ->where('status', 'paid')
            ->whereNotNull('subscription_id')
            ->selectRaw('AVG(amount) as avg_amount')
            ->value('avg_amount') ?? 0;

        $ltv = (int) round($avgMonthlyRevenue * $avgLifetimeMonths);

        // LTV by plan
        $ltvByPlan = DB::table('subscriptions')
            ->join('plans', 'subscriptions.plan_id', '=', 'plans.id')
            ->join('invoices', 'subscriptions.id', '=', 'invoices.subscription_id')
            ->where('invoices.status', 'paid')
            ->select('plans.name', 'plans.slug')
            ->selectRaw('AVG(invoices.amount * TIMESTAMPDIFF(MONTH, subscriptions.created_at, COALESCE(subscriptions.canceled_at, NOW()))) as ltv')
            ->groupBy('plans.id', 'plans.name', 'plans.slug')
            ->get();

        return [
            'average_ltv' => $ltv,
            'formatted' => '$' . number_format($ltv / 100, 2),
            'avg_lifetime_months' => round($avgLifetimeMonths, 1),
            'by_plan' => $ltvByPlan->map(fn($item) => [
                'plan' => $item->name,
                'ltv' => (int) $item->ltv,
                'formatted' => '$' . number_format(($item->ltv ?? 0) / 100, 2),
            ])->toArray(),
        ];
    }

    /**
     * Get revenue forecast
     */
    private function getForecast(): array
    {
        $currentMrr = $this->calculateMrr();

        // Calculate growth rate from last 3 months
        $threeMonthsAgo = now()->subMonths(3);
        $oldMrr = DB::table('subscriptions')
            ->join('plans', 'subscriptions.plan_id', '=', 'plans.id')
            ->where('subscriptions.status', 'active')
            ->where('subscriptions.created_at', '<=', $threeMonthsAgo)
            ->sum('plans.monthly_price');

        $monthlyGrowthRate = $oldMrr > 0
            ? (pow($currentMrr / $oldMrr, 1/3) - 1)
            : 0.05; // Default 5% growth

        // Project next 12 months
        $forecast = [];
        $projectedMrr = $currentMrr;

        for ($i = 1; $i <= 12; $i++) {
            $projectedMrr = (int) round($projectedMrr * (1 + $monthlyGrowthRate));
            $forecast[] = [
                'month' => now()->addMonths($i)->format('Y-m'),
                'projected_mrr' => $projectedMrr,
                'formatted' => '$' . number_format($projectedMrr / 100, 2),
            ];
        }

        return [
            'monthly_growth_rate' => round($monthlyGrowthRate * 100, 2),
            'projected_arr' => $projectedMrr * 12,
            'projections' => $forecast,
        ];
    }

    /**
     * Export revenue report
     */
    public function export(Request $request): JsonResponse
    {
        $startDate = $request->get('start_date')
            ? Carbon::parse($request->get('start_date'))
            : now()->subDays(30);

        $endDate = $request->get('end_date')
            ? Carbon::parse($request->get('end_date'))
            : now();

        $invoices = Invoice::where('status', 'paid')
            ->whereBetween('paid_at', [$startDate, $endDate])
            ->with(['user:id,name,email', 'subscription.plan:id,name'])
            ->get()
            ->map(fn($invoice) => [
                'date' => $invoice->paid_at->format('Y-m-d'),
                'invoice_number' => $invoice->invoice_number ?? $invoice->id,
                'customer' => $invoice->user?->name,
                'email' => $invoice->user?->email,
                'plan' => $invoice->subscription?->plan?->name,
                'amount' => $invoice->amount / 100,
                'currency' => $invoice->currency ?? 'USD',
                'status' => $invoice->status,
            ]);

        return response()->json([
            'data' => $invoices,
            'period' => [
                'start' => $startDate->format('Y-m-d'),
                'end' => $endDate->format('Y-m-d'),
            ],
            'total_records' => $invoices->count(),
        ]);
    }
}
