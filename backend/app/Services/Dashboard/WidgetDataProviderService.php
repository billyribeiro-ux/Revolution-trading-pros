<?php

namespace App\Services\Dashboard;

use App\Models\DashboardWidget;
use App\Models\User;
use App\Models\SystemHealthMetric;
use App\Models\DashboardActivityLog;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;

class WidgetDataProviderService
{
    public function __construct(
        private DashboardCacheService $cacheService
    ) {}

    /**
     * Get widget data with caching
     */
    public function getWidgetData(DashboardWidget $widget): array
    {
        $cacheKey = "widget_data_{$widget->id}";
        $ttl = $widget->refresh_interval ?? 300;

        return $this->cacheService->remember($cacheKey, $ttl, function () use ($widget) {
            return $this->fetchWidgetData($widget);
        });
    }

    /**
     * Fetch fresh widget data based on type
     */
    private function fetchWidgetData(DashboardWidget $widget): array
    {
        return match ($widget->widget_type) {
            'system_health' => $this->getSystemHealthData($widget),
            'revenue_mrr' => $this->getRevenueMRRData($widget),
            'user_growth' => $this->getUserGrowthData($widget),
            'subscription_churn' => $this->getSubscriptionChurnData($widget),
            'recent_activity' => $this->getRecentActivityData($widget),
            'email_performance' => $this->getEmailPerformanceData($widget),
            'crm_pipeline' => $this->getCRMPipelineData($widget),
            'subscription_status' => $this->getSubscriptionStatusData($widget),
            'recent_courses' => $this->getRecentCoursesData($widget),
            'trading_performance' => $this->getTradingPerformanceData($widget),
            'notifications' => $this->getNotificationsData($widget),
            'funnel_conversion' => $this->getFunnelConversionData($widget),
            'behavior_friction' => $this->getBehaviorFrictionData($widget),
            'attribution_model' => $this->getAttributionModelData($widget),
            'automation_runs' => $this->getAutomationRunsData($widget),
            'form_submissions' => $this->getFormSubmissionsData($widget),
            'popup_performance' => $this->getPopupPerformanceData($widget),
            'website_speed' => $this->getWebsiteSpeedData($widget),
            'integration_health' => $this->getIntegrationHealthData($widget),
            default => ['error' => 'Unknown widget type'],
        };
    }

    /**
     * System Health Widget Data
     */
    private function getSystemHealthData(DashboardWidget $widget): array
    {
        $services = ['email', 'crm', 'analytics', 'subscriptions', 'media', 'api'];
        $healthData = [];

        foreach ($services as $service) {
            $healthData[$service] = SystemHealthMetric::getServiceHealth($service);
        }

        $overallStatus = collect($healthData)->contains('overall_status', 'critical') 
            ? 'critical' 
            : (collect($healthData)->contains('overall_status', 'warning') ? 'warning' : 'healthy');

        return [
            'overall_status' => $overallStatus,
            'services' => $healthData,
            'last_updated' => now()->toIso8601String(),
        ];
    }

    /**
     * Revenue MRR Widget Data
     */
    private function getRevenueMRRData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $currentMRR = DB::table('subscriptions')
            ->where('status', 'active')
            ->sum('price');

        $previousMRR = DB::table('subscriptions')
            ->where('status', 'active')
            ->where('created_at', '<', now()->subDays($days))
            ->sum('price');

        $growth = $previousMRR > 0 ? (($currentMRR - $previousMRR) / $previousMRR) * 100 : 0;

        $chartData = DB::table('subscriptions')
            ->selectRaw('DATE(created_at) as date, SUM(price) as revenue')
            ->where('created_at', '>=', now()->subDays($days))
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return [
            'current_mrr' => round($currentMRR, 2),
            'previous_mrr' => round($previousMRR, 2),
            'growth_percentage' => round($growth, 2),
            'chart_data' => $chartData,
            'period' => $period,
        ];
    }

    /**
     * User Growth Widget Data
     */
    private function getUserGrowthData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $currentUsers = User::where('created_at', '>=', now()->subDays($days))->count();
        $previousUsers = User::where('created_at', '>=', now()->subDays($days * 2))
            ->where('created_at', '<', now()->subDays($days))
            ->count();

        $growth = $previousUsers > 0 ? (($currentUsers - $previousUsers) / $previousUsers) * 100 : 0;

        $chartData = User::selectRaw('DATE(created_at) as date, COUNT(*) as count')
            ->where('created_at', '>=', now()->subDays($days))
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return [
            'current_users' => $currentUsers,
            'previous_users' => $previousUsers,
            'growth_percentage' => round($growth, 2),
            'total_users' => User::count(),
            'chart_data' => $chartData,
        ];
    }

    /**
     * Subscription Churn Widget Data
     */
    private function getSubscriptionChurnData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $startOfPeriod = now()->subDays($days);
        
        $activeAtStart = DB::table('subscriptions')
            ->where('created_at', '<', $startOfPeriod)
            ->where(function ($query) use ($startOfPeriod) {
                $query->whereNull('cancelled_at')
                    ->orWhere('cancelled_at', '>=', $startOfPeriod);
            })
            ->count();

        $churned = DB::table('subscriptions')
            ->where('cancelled_at', '>=', $startOfPeriod)
            ->where('cancelled_at', '<=', now())
            ->count();

        $churnRate = $activeAtStart > 0 ? ($churned / $activeAtStart) * 100 : 0;

        return [
            'churn_rate' => round($churnRate, 2),
            'churned_count' => $churned,
            'active_at_start' => $activeAtStart,
            'period' => $period,
        ];
    }

    /**
     * Recent Activity Widget Data
     */
    private function getRecentActivityData(DashboardWidget $widget): array
    {
        $limit = $widget->config['limit'] ?? 20;

        $activities = DashboardActivityLog::with('user')
            ->recent($limit)
            ->get()
            ->map(function ($activity) {
                return [
                    'id' => $activity->id,
                    'user' => $activity->user ? [
                        'id' => $activity->user->id,
                        'name' => $activity->user->name,
                        'email' => $activity->user->email,
                    ] : null,
                    'entity_type' => $activity->entity_type,
                    'entity_id' => $activity->entity_id,
                    'action' => $activity->action,
                    'description' => $activity->description,
                    'created_at' => $activity->created_at->toIso8601String(),
                ];
            });

        return [
            'activities' => $activities,
            'total_count' => DashboardActivityLog::count(),
        ];
    }

    /**
     * Email Performance Widget Data
     */
    private function getEmailPerformanceData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '7d';
        $days = (int) str_replace('d', '', $period);

        // This would integrate with your email system
        return [
            'sent' => 1250,
            'delivered' => 1200,
            'opened' => 480,
            'clicked' => 120,
            'bounced' => 50,
            'open_rate' => 40.0,
            'click_rate' => 10.0,
            'bounce_rate' => 4.0,
            'period' => $period,
        ];
    }

    /**
     * CRM Pipeline Widget Data
     */
    private function getCRMPipelineData(DashboardWidget $widget): array
    {
        // This would integrate with your CRM system
        return [
            'stages' => [
                ['name' => 'Lead', 'count' => 45, 'value' => 45000],
                ['name' => 'Qualified', 'count' => 23, 'value' => 69000],
                ['name' => 'Proposal', 'count' => 12, 'value' => 48000],
                ['name' => 'Negotiation', 'count' => 8, 'value' => 40000],
                ['name' => 'Closed Won', 'count' => 15, 'value' => 75000],
            ],
            'total_value' => 277000,
            'total_deals' => 103,
        ];
    }

    /**
     * Subscription Status Widget Data (User Dashboard)
     */
    private function getSubscriptionStatusData(DashboardWidget $widget): array
    {
        $userId = $widget->dashboard->user_id;

        $subscription = DB::table('subscriptions')
            ->where('user_id', $userId)
            ->where('status', 'active')
            ->first();

        if (!$subscription) {
            return ['status' => 'none', 'message' => 'No active subscription'];
        }

        return [
            'status' => $subscription->status,
            'plan_name' => $subscription->plan_name ?? 'Premium',
            'price' => $subscription->price,
            'billing_cycle' => $subscription->billing_cycle ?? 'monthly',
            'next_billing_date' => $subscription->next_billing_date,
            'features' => ['Trading Rooms', 'All Courses', 'Priority Support'],
        ];
    }

    /**
     * Recent Courses Widget Data (User Dashboard)
     */
    private function getRecentCoursesData(DashboardWidget $widget): array
    {
        $userId = $widget->dashboard->user_id;
        $limit = $widget->config['limit'] ?? 5;

        // This would integrate with your course system
        return [
            'courses' => [
                ['id' => 1, 'title' => 'Day Trading Basics', 'progress' => 75, 'last_accessed' => now()->subDays(1)->toIso8601String()],
                ['id' => 2, 'title' => 'Options Strategies', 'progress' => 45, 'last_accessed' => now()->subDays(3)->toIso8601String()],
                ['id' => 3, 'title' => 'Technical Analysis', 'progress' => 90, 'last_accessed' => now()->subDays(5)->toIso8601String()],
            ],
        ];
    }

    /**
     * Trading Performance Widget Data (User Dashboard)
     */
    private function getTradingPerformanceData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';

        // This would integrate with your trading system
        return [
            'total_trades' => 45,
            'winning_trades' => 32,
            'losing_trades' => 13,
            'win_rate' => 71.1,
            'total_profit' => 5420.50,
            'average_profit' => 120.46,
            'period' => $period,
        ];
    }

    /**
     * Notifications Widget Data
     */
    private function getNotificationsData(DashboardWidget $widget): array
    {
        $userId = $widget->dashboard->user_id;
        $limit = $widget->config['limit'] ?? 10;

        // This would integrate with your notification system
        return [
            'unread_count' => 5,
            'notifications' => [
                ['id' => 1, 'title' => 'New course available', 'message' => 'Check out Advanced Options', 'read' => false, 'created_at' => now()->subHours(2)->toIso8601String()],
                ['id' => 2, 'title' => 'Trading alert', 'message' => 'SPX signal triggered', 'read' => false, 'created_at' => now()->subHours(5)->toIso8601String()],
            ],
        ];
    }

    /**
     * Funnel Conversion Widget Data
     */
    private function getFunnelConversionData(DashboardWidget $widget): array
    {
        return [
            'stages' => [
                ['name' => 'Visitors', 'count' => 10000, 'conversion' => 100],
                ['name' => 'Signups', 'count' => 1500, 'conversion' => 15],
                ['name' => 'Trial', 'count' => 450, 'conversion' => 30],
                ['name' => 'Paid', 'count' => 180, 'conversion' => 40],
            ],
            'overall_conversion' => 1.8,
        ];
    }

    /**
     * Behavior Friction Widget Data
     */
    private function getBehaviorFrictionData(DashboardWidget $widget): array
    {
        return [
            'friction_score' => 32,
            'high_friction_pages' => [
                ['page' => '/checkout', 'score' => 68],
                ['page' => '/signup', 'score' => 45],
            ],
        ];
    }

    /**
     * Attribution Model Widget Data
     */
    private function getAttributionModelData(DashboardWidget $widget): array
    {
        return [
            'channels' => [
                ['name' => 'Organic Search', 'conversions' => 45, 'value' => 13500],
                ['name' => 'Paid Ads', 'conversions' => 32, 'value' => 9600],
                ['name' => 'Social Media', 'conversions' => 18, 'value' => 5400],
                ['name' => 'Email', 'conversions' => 25, 'value' => 7500],
            ],
        ];
    }

    /**
     * Automation Runs Widget Data
     */
    private function getAutomationRunsData(DashboardWidget $widget): array
    {
        return [
            'total_runs' => 1250,
            'successful' => 1180,
            'failed' => 70,
            'success_rate' => 94.4,
        ];
    }

    /**
     * Form Submissions Widget Data
     */
    private function getFormSubmissionsData(DashboardWidget $widget): array
    {
        return [
            'total_submissions' => 450,
            'conversion_rate' => 12.5,
            'top_forms' => [
                ['name' => 'Contact Form', 'submissions' => 180],
                ['name' => 'Newsletter Signup', 'submissions' => 150],
            ],
        ];
    }

    /**
     * Popup Performance Widget Data
     */
    private function getPopupPerformanceData(DashboardWidget $widget): array
    {
        return [
            'impressions' => 5000,
            'conversions' => 250,
            'conversion_rate' => 5.0,
            'top_popups' => [
                ['name' => 'Exit Intent Offer', 'conversions' => 120],
                ['name' => 'Welcome Discount', 'conversions' => 80],
            ],
        ];
    }

    /**
     * Website Speed Widget Data
     */
    private function getWebsiteSpeedData(DashboardWidget $widget): array
    {
        return [
            'avg_load_time' => 1.2,
            'performance_score' => 92,
            'slow_pages' => [
                ['page' => '/courses', 'load_time' => 2.8],
                ['page' => '/dashboard', 'load_time' => 2.1],
            ],
        ];
    }

    /**
     * Integration Health Widget Data
     */
    private function getIntegrationHealthData(DashboardWidget $widget): array
    {
        return [
            'integrations' => [
                ['name' => 'Stripe', 'status' => 'healthy', 'last_sync' => now()->subMinutes(5)->toIso8601String()],
                ['name' => 'SendGrid', 'status' => 'healthy', 'last_sync' => now()->subMinutes(10)->toIso8601String()],
                ['name' => 'Analytics', 'status' => 'warning', 'last_sync' => now()->subHours(2)->toIso8601String()],
            ],
        ];
    }
}
