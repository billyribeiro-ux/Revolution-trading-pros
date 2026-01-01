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
            // SEO & Analytics widgets (RankMath Pro style)
            'seo_overview' => $this->getSeoOverviewData($widget),
            'keyword_rankings' => $this->getKeywordRankingsData($widget),
            'visitor_analytics' => $this->getVisitorAnalyticsData($widget),
            'traffic_sources' => $this->getTrafficSourcesData($widget),
            'top_pages' => $this->getTopPagesData($widget),
            'seo_score' => $this->getSeoScoreData($widget),
            'indexing_status' => $this->getIndexingStatusData($widget),
            'core_web_vitals' => $this->getCoreWebVitalsData($widget),
            'backlink_overview' => $this->getBacklinkOverviewData($widget),
            'content_performance' => $this->getContentPerformanceData($widget),
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

    // ========================================
    // SEO & ANALYTICS WIDGETS (RankMath Pro Style)
    // ========================================

    /**
     * SEO Overview Widget Data (RankMath-style dashboard)
     */
    private function getSeoOverviewData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        // Get SEO analysis data
        $analyses = DB::table('seo_analyses')
            ->where('created_at', '>=', now()->subDays($days))
            ->selectRaw('
                AVG(overall_score) as avg_score,
                COUNT(*) as total_analyses,
                SUM(CASE WHEN overall_score >= 80 THEN 1 ELSE 0 END) as good_count,
                SUM(CASE WHEN overall_score >= 50 AND overall_score < 80 THEN 1 ELSE 0 END) as average_count,
                SUM(CASE WHEN overall_score < 50 THEN 1 ELSE 0 END) as poor_count
            ')
            ->first();

        // Get keyword rankings trend
        $rankingsTrend = DB::table('rank_tracking')
            ->where('checked_at', '>=', now()->subDays($days))
            ->selectRaw('DATE(checked_at) as date, AVG(position) as avg_position')
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return [
            'overall_score' => round($analyses->avg_score ?? 0, 1),
            'score_breakdown' => [
                'good' => (int) ($analyses->good_count ?? 0),
                'average' => (int) ($analyses->average_count ?? 0),
                'poor' => (int) ($analyses->poor_count ?? 0),
            ],
            'total_pages_analyzed' => (int) ($analyses->total_analyses ?? 0),
            'rankings_trend' => $rankingsTrend,
            'quick_stats' => [
                ['label' => 'Indexed Pages', 'value' => $this->getIndexedPagesCount(), 'trend' => 'up'],
                ['label' => 'Avg Position', 'value' => $this->getAvgKeywordPosition(), 'trend' => 'up'],
                ['label' => 'Organic Traffic', 'value' => $this->getOrganicTraffic($days), 'trend' => 'up'],
                ['label' => 'Backlinks', 'value' => $this->getTotalBacklinks(), 'trend' => 'stable'],
            ],
            'period' => $period,
        ];
    }

    /**
     * Keyword Rankings Widget Data
     */
    private function getKeywordRankingsData(DashboardWidget $widget): array
    {
        $limit = $widget->config['limit'] ?? 10;
        $period = $widget->config['period'] ?? '7d';

        $keywords = DB::table('rank_tracking')
            ->select([
                'keyword',
                'position',
                'previous_position',
                'url',
                'search_volume',
            ])
            ->whereIn('id', function ($query) {
                $query->selectRaw('MAX(id)')
                    ->from('rank_tracking')
                    ->groupBy('keyword');
            })
            ->orderBy('search_volume', 'desc')
            ->limit($limit)
            ->get()
            ->map(function ($item) {
                $change = ($item->previous_position ?? 0) - $item->position;
                return [
                    'keyword' => $item->keyword,
                    'position' => $item->position,
                    'previous_position' => $item->previous_position,
                    'change' => $change,
                    'trend' => $change > 0 ? 'up' : ($change < 0 ? 'down' : 'stable'),
                    'url' => $item->url,
                    'search_volume' => $item->search_volume,
                ];
            });

        $avgPosition = DB::table('rank_tracking')
            ->whereIn('id', function ($query) {
                $query->selectRaw('MAX(id)')
                    ->from('rank_tracking')
                    ->groupBy('keyword');
            })
            ->avg('position');

        return [
            'keywords' => $keywords,
            'total_keywords' => DB::table('rank_tracking')->distinct('keyword')->count(),
            'avg_position' => round($avgPosition ?? 0, 1),
            'top_10_count' => DB::table('rank_tracking')
                ->whereIn('id', function ($query) {
                    $query->selectRaw('MAX(id)')
                        ->from('rank_tracking')
                        ->groupBy('keyword');
                })
                ->where('position', '<=', 10)
                ->count(),
            'improved_count' => $keywords->filter(fn($k) => $k['change'] > 0)->count(),
            'declined_count' => $keywords->filter(fn($k) => $k['change'] < 0)->count(),
        ];
    }

    /**
     * Visitor Analytics Widget Data
     */
    private function getVisitorAnalyticsData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        // Get visitor data from behavior sessions
        $current = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->selectRaw('
                COUNT(DISTINCT visitor_id) as unique_visitors,
                COUNT(*) as total_sessions,
                AVG(duration) as avg_duration,
                SUM(page_count) as total_pageviews,
                AVG(page_count) as avg_pages_per_session
            ')
            ->first();

        $previous = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days * 2))
            ->where('started_at', '<', now()->subDays($days))
            ->selectRaw('
                COUNT(DISTINCT visitor_id) as unique_visitors,
                COUNT(*) as total_sessions
            ')
            ->first();

        $visitorGrowth = ($previous->unique_visitors ?? 0) > 0
            ? (($current->unique_visitors - $previous->unique_visitors) / $previous->unique_visitors) * 100
            : 0;

        // Daily visitors trend
        $dailyTrend = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->selectRaw('DATE(started_at) as date, COUNT(DISTINCT visitor_id) as visitors, COUNT(*) as sessions')
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        // Hourly distribution
        $hourlyDistribution = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays(7))
            ->selectRaw('HOUR(started_at) as hour, COUNT(*) as sessions')
            ->groupBy('hour')
            ->orderBy('hour')
            ->get();

        return [
            'unique_visitors' => (int) ($current->unique_visitors ?? 0),
            'total_sessions' => (int) ($current->total_sessions ?? 0),
            'total_pageviews' => (int) ($current->total_pageviews ?? 0),
            'avg_session_duration' => round($current->avg_duration ?? 0, 2),
            'avg_pages_per_session' => round($current->avg_pages_per_session ?? 0, 2),
            'visitor_growth' => round($visitorGrowth, 2),
            'bounce_rate' => $this->calculateBounceRate($days),
            'daily_trend' => $dailyTrend,
            'hourly_distribution' => $hourlyDistribution,
            'period' => $period,
        ];
    }

    /**
     * Traffic Sources Widget Data
     */
    private function getTrafficSourcesData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $sources = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->selectRaw('
                COALESCE(utm_source, CASE
                    WHEN referrer IS NULL OR referrer = "" THEN "direct"
                    WHEN referrer LIKE "%google%" THEN "organic"
                    WHEN referrer LIKE "%facebook%" OR referrer LIKE "%instagram%" OR referrer LIKE "%twitter%" THEN "social"
                    ELSE "referral"
                END) as source,
                COUNT(*) as sessions,
                COUNT(DISTINCT visitor_id) as visitors
            ')
            ->groupBy('source')
            ->orderByDesc('sessions')
            ->get();

        $total = $sources->sum('sessions');

        return [
            'sources' => $sources->map(function ($s) use ($total) {
                return [
                    'name' => ucfirst($s->source),
                    'sessions' => $s->sessions,
                    'visitors' => $s->visitors,
                    'percentage' => $total > 0 ? round(($s->sessions / $total) * 100, 1) : 0,
                ];
            }),
            'total_sessions' => $total,
            'period' => $period,
        ];
    }

    /**
     * Top Pages Widget Data
     */
    private function getTopPagesData(DashboardWidget $widget): array
    {
        $limit = $widget->config['limit'] ?? 10;
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $pages = DB::table('behavior_events')
            ->where('event_type', 'page_view')
            ->where('occurred_at', '>=', now()->subDays($days))
            ->selectRaw('
                event_data->"$.page_path" as page,
                COUNT(*) as views,
                COUNT(DISTINCT session_id) as unique_views,
                AVG(event_data->"$.time_on_page") as avg_time
            ')
            ->groupBy('page')
            ->orderByDesc('views')
            ->limit($limit)
            ->get();

        return [
            'pages' => $pages->map(function ($p) {
                return [
                    'page' => $p->page,
                    'views' => $p->views,
                    'unique_views' => $p->unique_views,
                    'avg_time_on_page' => round($p->avg_time ?? 0, 2),
                ];
            }),
            'period' => $period,
        ];
    }

    /**
     * SEO Score Widget Data (RankMath-style score breakdown)
     */
    private function getSeoScoreData(DashboardWidget $widget): array
    {
        $contentType = $widget->config['content_type'] ?? 'post';

        $latestAnalyses = DB::table('seo_analyses')
            ->where('content_type', $contentType)
            ->orderByDesc('created_at')
            ->limit(50)
            ->get();

        $avgScores = [
            'overall' => $latestAnalyses->avg('overall_score'),
            'title' => $latestAnalyses->avg('title_score'),
            'content' => $latestAnalyses->avg('content_score'),
            'readability' => $latestAnalyses->avg('readability_score'),
            'technical' => $latestAnalyses->avg('technical_score'),
        ];

        $distribution = [
            'excellent' => $latestAnalyses->filter(fn($a) => $a->overall_score >= 80)->count(),
            'good' => $latestAnalyses->filter(fn($a) => $a->overall_score >= 60 && $a->overall_score < 80)->count(),
            'needs_improvement' => $latestAnalyses->filter(fn($a) => $a->overall_score >= 40 && $a->overall_score < 60)->count(),
            'poor' => $latestAnalyses->filter(fn($a) => $a->overall_score < 40)->count(),
        ];

        return [
            'average_scores' => array_map(fn($s) => round($s ?? 0, 1), $avgScores),
            'distribution' => $distribution,
            'total_analyzed' => $latestAnalyses->count(),
            'content_type' => $contentType,
            'recommendations' => $this->getSeoRecommendations($avgScores),
        ];
    }

    /**
     * Indexing Status Widget Data
     */
    private function getIndexingStatusData(DashboardWidget $widget): array
    {
        $totalPages = DB::table('posts')->where('status', 'published')->count()
            + DB::table('products')->where('is_active', true)->count();

        // This would integrate with Google Search Console API
        return [
            'total_pages' => $totalPages,
            'indexed_pages' => $this->getIndexedPagesCount(),
            'not_indexed' => max(0, $totalPages - $this->getIndexedPagesCount()),
            'pending_indexing' => 0,
            'indexing_rate' => $totalPages > 0
                ? round(($this->getIndexedPagesCount() / $totalPages) * 100, 1)
                : 0,
            'status_breakdown' => [
                ['status' => 'Indexed', 'count' => $this->getIndexedPagesCount(), 'color' => '#22c55e'],
                ['status' => 'Discovered', 'count' => 5, 'color' => '#f59e0b'],
                ['status' => 'Crawled', 'count' => 3, 'color' => '#3b82f6'],
                ['status' => 'Excluded', 'count' => 2, 'color' => '#ef4444'],
            ],
            'last_crawl' => now()->subHours(4)->toIso8601String(),
        ];
    }

    /**
     * Core Web Vitals Widget Data
     */
    private function getCoreWebVitalsData(DashboardWidget $widget): array
    {
        // This would integrate with real performance monitoring
        return [
            'lcp' => [
                'value' => 2.1,
                'unit' => 's',
                'status' => 'good',
                'threshold' => ['good' => 2.5, 'needs_improvement' => 4.0],
                'label' => 'Largest Contentful Paint',
            ],
            'fid' => [
                'value' => 85,
                'unit' => 'ms',
                'status' => 'good',
                'threshold' => ['good' => 100, 'needs_improvement' => 300],
                'label' => 'First Input Delay',
            ],
            'cls' => [
                'value' => 0.08,
                'unit' => '',
                'status' => 'good',
                'threshold' => ['good' => 0.1, 'needs_improvement' => 0.25],
                'label' => 'Cumulative Layout Shift',
            ],
            'ttfb' => [
                'value' => 420,
                'unit' => 'ms',
                'status' => 'good',
                'threshold' => ['good' => 800, 'needs_improvement' => 1800],
                'label' => 'Time to First Byte',
            ],
            'overall_status' => 'good',
            'mobile_friendly' => true,
            'https_enabled' => true,
            'last_test' => now()->subHours(1)->toIso8601String(),
        ];
    }

    /**
     * Backlink Overview Widget Data
     */
    private function getBacklinkOverviewData(DashboardWidget $widget): array
    {
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $backlinks = DB::table('backlinks')
            ->selectRaw('
                COUNT(*) as total,
                SUM(CASE WHEN status = "active" THEN 1 ELSE 0 END) as active,
                SUM(CASE WHEN is_dofollow = 1 THEN 1 ELSE 0 END) as dofollow,
                AVG(domain_authority) as avg_da,
                COUNT(DISTINCT source_domain) as referring_domains
            ')
            ->first();

        $newBacklinks = DB::table('backlinks')
            ->where('first_seen_at', '>=', now()->subDays($days))
            ->count();

        $lostBacklinks = DB::table('backlinks')
            ->where('status', 'lost')
            ->where('updated_at', '>=', now()->subDays($days))
            ->count();

        $topReferrers = DB::table('backlinks')
            ->where('status', 'active')
            ->selectRaw('source_domain, domain_authority, COUNT(*) as links')
            ->groupBy('source_domain', 'domain_authority')
            ->orderByDesc('domain_authority')
            ->limit(5)
            ->get();

        return [
            'total_backlinks' => (int) ($backlinks->total ?? 0),
            'active_backlinks' => (int) ($backlinks->active ?? 0),
            'dofollow_links' => (int) ($backlinks->dofollow ?? 0),
            'nofollow_links' => (int) (($backlinks->total ?? 0) - ($backlinks->dofollow ?? 0)),
            'referring_domains' => (int) ($backlinks->referring_domains ?? 0),
            'avg_domain_authority' => round($backlinks->avg_da ?? 0, 1),
            'new_backlinks' => $newBacklinks,
            'lost_backlinks' => $lostBacklinks,
            'net_change' => $newBacklinks - $lostBacklinks,
            'top_referrers' => $topReferrers,
            'period' => $period,
        ];
    }

    /**
     * Content Performance Widget Data
     */
    private function getContentPerformanceData(DashboardWidget $widget): array
    {
        $limit = $widget->config['limit'] ?? 10;
        $period = $widget->config['period'] ?? '30d';
        $days = (int) str_replace('d', '', $period);

        $posts = DB::table('posts')
            ->leftJoin('behavior_events', function ($join) use ($days) {
                $join->on(DB::raw('behavior_events.event_data->"$.page_path"'), 'LIKE', DB::raw('CONCAT("/blog/", posts.slug)'))
                    ->where('behavior_events.event_type', 'page_view')
                    ->where('behavior_events.occurred_at', '>=', now()->subDays($days));
            })
            ->leftJoin('seo_analyses', function ($join) {
                $join->on('posts.id', '=', 'seo_analyses.content_id')
                    ->where('seo_analyses.content_type', 'post');
            })
            ->where('posts.status', 'published')
            ->selectRaw('
                posts.id,
                posts.title,
                posts.slug,
                posts.published_at,
                COUNT(behavior_events.id) as views,
                seo_analyses.overall_score as seo_score
            ')
            ->groupBy('posts.id', 'posts.title', 'posts.slug', 'posts.published_at', 'seo_analyses.overall_score')
            ->orderByDesc('views')
            ->limit($limit)
            ->get();

        return [
            'top_content' => $posts->map(function ($p) {
                return [
                    'id' => $p->id,
                    'title' => $p->title,
                    'slug' => $p->slug,
                    'views' => $p->views,
                    'seo_score' => round($p->seo_score ?? 0, 1),
                    'published_at' => $p->published_at,
                ];
            }),
            'total_posts' => DB::table('posts')->where('status', 'published')->count(),
            'total_views' => $posts->sum('views'),
            'avg_seo_score' => round($posts->avg('seo_score') ?? 0, 1),
            'period' => $period,
        ];
    }

    // ========================================
    // HELPER METHODS
    // ========================================

    /**
     * Get indexed pages count
     */
    private function getIndexedPagesCount(): int
    {
        // This would integrate with Google Search Console
        return DB::table('posts')->where('status', 'published')->count() - 3;
    }

    /**
     * Get average keyword position
     */
    private function getAvgKeywordPosition(): float
    {
        $avg = DB::table('rank_tracking')
            ->whereIn('id', function ($query) {
                $query->selectRaw('MAX(id)')
                    ->from('rank_tracking')
                    ->groupBy('keyword');
            })
            ->avg('position');

        return round($avg ?? 0, 1);
    }

    /**
     * Get organic traffic count
     */
    private function getOrganicTraffic(int $days): int
    {
        return DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->where(function ($q) {
                $q->where('utm_source', 'organic')
                    ->orWhere('referrer', 'LIKE', '%google%')
                    ->orWhere('referrer', 'LIKE', '%bing%')
                    ->orWhere('referrer', 'LIKE', '%duckduckgo%');
            })
            ->count();
    }

    /**
     * Get total backlinks count
     */
    private function getTotalBacklinks(): int
    {
        return DB::table('backlinks')->where('status', 'active')->count();
    }

    /**
     * Calculate bounce rate
     */
    private function calculateBounceRate(int $days): float
    {
        $total = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->count();

        $bounced = DB::table('behavior_sessions')
            ->where('started_at', '>=', now()->subDays($days))
            ->where('page_count', 1)
            ->count();

        return $total > 0 ? round(($bounced / $total) * 100, 2) : 0;
    }

    /**
     * Get SEO recommendations based on scores
     */
    private function getSeoRecommendations(array $scores): array
    {
        $recommendations = [];

        if (($scores['title'] ?? 0) < 70) {
            $recommendations[] = [
                'priority' => 'high',
                'category' => 'title',
                'message' => 'Improve title tags - ensure they contain target keywords and are 50-60 characters',
            ];
        }

        if (($scores['content'] ?? 0) < 70) {
            $recommendations[] = [
                'priority' => 'high',
                'category' => 'content',
                'message' => 'Enhance content quality - add more relevant keywords and improve content depth',
            ];
        }

        if (($scores['readability'] ?? 0) < 70) {
            $recommendations[] = [
                'priority' => 'medium',
                'category' => 'readability',
                'message' => 'Improve readability - use shorter sentences and simpler words',
            ];
        }

        if (($scores['technical'] ?? 0) < 70) {
            $recommendations[] = [
                'priority' => 'high',
                'category' => 'technical',
                'message' => 'Fix technical SEO issues - check meta tags, schema markup, and site speed',
            ];
        }

        return $recommendations;
    }
}
