<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailCampaign;
use App\Models\NewsletterSubscription;
use App\Models\EmailDomain;
use App\Models\EmailWebhook;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;

/**
 * EmailMetricsController
 *
 * Enterprise analytics dashboard for email marketing.
 *
 * Features:
 * - Dashboard KPIs
 * - Trend analysis
 * - Performance comparison
 * - Engagement metrics
 * - Deliverability stats
 * - Revenue attribution
 *
 * @version 1.0.0
 */
class EmailMetricsController extends Controller
{
    private const CACHE_TTL = 300; // 5 minutes

    /**
     * Get main dashboard data
     */
    public function dashboard(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $cacheKey = "email_metrics:dashboard:{$period}";

        $data = Cache::remember($cacheKey, self::CACHE_TTL, function () use ($period) {
            $dateRange = $this->getDateRange($period);

            return [
                'overview' => $this->getOverviewStats($dateRange),
                'campaigns' => $this->getCampaignStats($dateRange),
                'subscribers' => $this->getSubscriberStats($dateRange),
                'engagement' => $this->getEngagementStats($dateRange),
                'deliverability' => $this->getDeliverabilityStats($dateRange),
                'trends' => $this->getTrends($dateRange),
            ];
        });

        return response()->json([
            'success' => true,
            'data' => $data,
            'period' => $period,
        ]);
    }

    /**
     * Get overview statistics
     */
    public function overview(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        $stats = $this->getOverviewStats($dateRange);

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get campaign performance metrics
     */
    public function campaigns(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        $stats = $this->getCampaignStats($dateRange);

        // Get top performing campaigns
        $topCampaigns = EmailCampaign::where('sent_at', '>=', $dateRange['start'])
            ->where('status', 'sent')
            ->orderByRaw('(opened_count / NULLIF(sent_count, 0)) DESC')
            ->limit(10)
            ->get(['id', 'name', 'sent_count', 'opened_count', 'clicked_count', 'sent_at']);

        $stats['top_campaigns'] = $topCampaigns;

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get subscriber growth metrics
     */
    public function subscribers(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        $stats = $this->getSubscriberStats($dateRange);

        // Daily growth
        $dailyGrowth = NewsletterSubscription::where('created_at', '>=', $dateRange['start'])
            ->selectRaw('DATE(created_at) as date, COUNT(*) as count')
            ->groupBy('date')
            ->orderBy('date')
            ->pluck('count', 'date');

        $stats['daily_growth'] = $dailyGrowth;

        // Source breakdown
        $stats['by_source'] = NewsletterSubscription::where('created_at', '>=', $dateRange['start'])
            ->groupBy('source')
            ->selectRaw('source, COUNT(*) as count')
            ->pluck('count', 'source');

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get engagement metrics
     */
    public function engagement(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        $stats = $this->getEngagementStats($dateRange);

        // Hourly engagement distribution
        $hourlyEngagement = DB::table('email_logs')
            ->where('created_at', '>=', $dateRange['start'])
            ->whereIn('event_type', ['opened', 'clicked'])
            ->selectRaw('HOUR(created_at) as hour, event_type, COUNT(*) as count')
            ->groupBy('hour', 'event_type')
            ->get()
            ->groupBy('hour');

        $stats['hourly_distribution'] = $hourlyEngagement;

        // Day of week engagement
        $dayOfWeekEngagement = DB::table('email_logs')
            ->where('created_at', '>=', $dateRange['start'])
            ->whereIn('event_type', ['opened', 'clicked'])
            ->selectRaw('DAYOFWEEK(created_at) as day, COUNT(*) as count')
            ->groupBy('day')
            ->pluck('count', 'day');

        $stats['day_of_week'] = $dayOfWeekEngagement;

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get deliverability report
     */
    public function deliverability(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        $stats = $this->getDeliverabilityStats($dateRange);

        // Domain health
        $domains = EmailDomain::where('verified', true)->get();
        $stats['domains'] = $domains->map(fn($d) => [
            'domain' => $d->domain,
            'health_score' => $d->health_score,
            'spf' => $d->spf_verified,
            'dkim' => $d->dkim_verified,
            'dmarc' => $d->dmarc_verified,
        ]);

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get real-time metrics
     */
    public function realtime(): JsonResponse
    {
        // Emails sent in last hour
        $lastHour = DB::table('email_logs')
            ->where('created_at', '>=', now()->subHour())
            ->selectRaw('
                COUNT(*) as total,
                SUM(CASE WHEN event_type = "sent" THEN 1 ELSE 0 END) as sent,
                SUM(CASE WHEN event_type = "opened" THEN 1 ELSE 0 END) as opened,
                SUM(CASE WHEN event_type = "clicked" THEN 1 ELSE 0 END) as clicked,
                SUM(CASE WHEN event_type = "bounced" THEN 1 ELSE 0 END) as bounced
            ')
            ->first();

        // Active campaigns
        $activeCampaigns = EmailCampaign::where('status', 'sending')->count();

        // Queue status
        $queuedEmails = DB::table('jobs')
            ->where('queue', 'like', '%email%')
            ->count();

        // Recent subscribers
        $recentSubscribers = NewsletterSubscription::where('created_at', '>=', now()->subHour())->count();

        return response()->json([
            'success' => true,
            'data' => [
                'last_hour' => $lastHour,
                'active_campaigns' => $activeCampaigns,
                'queued_emails' => $queuedEmails,
                'recent_subscribers' => $recentSubscribers,
                'timestamp' => now()->toIso8601String(),
            ],
        ]);
    }

    /**
     * Get comparison metrics
     */
    public function compare(Request $request): JsonResponse
    {
        $period1 = $request->get('period1', 'this_month');
        $period2 = $request->get('period2', 'last_month');

        $range1 = $this->getDateRange($period1);
        $range2 = $this->getDateRange($period2);

        $stats1 = $this->getOverviewStats($range1);
        $stats2 = $this->getOverviewStats($range2);

        $comparison = [];
        foreach ($stats1 as $key => $value1) {
            $value2 = $stats2[$key] ?? 0;
            $change = $value2 > 0 ? (($value1 - $value2) / $value2) * 100 : 0;

            $comparison[$key] = [
                'current' => $value1,
                'previous' => $value2,
                'change' => round($change, 2),
                'trend' => $change > 0 ? 'up' : ($change < 0 ? 'down' : 'stable'),
            ];
        }

        return response()->json([
            'success' => true,
            'data' => $comparison,
            'periods' => [
                'current' => $period1,
                'previous' => $period2,
            ],
        ]);
    }

    /**
     * Export metrics as CSV/JSON
     */
    public function export(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $dateRange = $this->getDateRange($period);

        // Gather all metrics
        $metrics = [
            'overview' => $this->getOverviewStats($dateRange),
            'campaigns' => $this->getCampaignStats($dateRange),
            'subscribers' => $this->getSubscriberStats($dateRange),
            'engagement' => $this->getEngagementStats($dateRange),
            'deliverability' => $this->getDeliverabilityStats($dateRange),
        ];

        // Campaign details
        $campaigns = EmailCampaign::where('sent_at', '>=', $dateRange['start'])
            ->get(['id', 'name', 'subject', 'sent_count', 'opened_count', 'clicked_count',
                   'bounced_count', 'unsubscribed_count', 'sent_at']);

        $metrics['campaign_details'] = $campaigns;

        return response()->json([
            'success' => true,
            'data' => $metrics,
            'exported_at' => now()->toIso8601String(),
            'period' => $period,
        ]);
    }

    /**
     * Get overview stats
     */
    private function getOverviewStats(array $dateRange): array
    {
        return [
            'total_subscribers' => NewsletterSubscription::where('status', 'active')->count(),
            'new_subscribers' => NewsletterSubscription::where('created_at', '>=', $dateRange['start'])->count(),
            'total_campaigns' => EmailCampaign::where('sent_at', '>=', $dateRange['start'])->count(),
            'total_emails_sent' => EmailCampaign::where('sent_at', '>=', $dateRange['start'])->sum('sent_count'),
            'total_opens' => EmailCampaign::where('sent_at', '>=', $dateRange['start'])->sum('opened_count'),
            'total_clicks' => EmailCampaign::where('sent_at', '>=', $dateRange['start'])->sum('clicked_count'),
            'unsubscribes' => NewsletterSubscription::where('unsubscribed_at', '>=', $dateRange['start'])->count(),
        ];
    }

    /**
     * Get campaign stats
     */
    private function getCampaignStats(array $dateRange): array
    {
        $campaigns = EmailCampaign::where('sent_at', '>=', $dateRange['start'])
            ->where('status', 'sent')
            ->get();

        $totalSent = $campaigns->sum('sent_count') ?: 1;
        $totalOpened = $campaigns->sum('opened_count');
        $totalClicked = $campaigns->sum('clicked_count');
        $totalBounced = $campaigns->sum('bounced_count');

        return [
            'total' => $campaigns->count(),
            'sent' => $totalSent,
            'opened' => $totalOpened,
            'clicked' => $totalClicked,
            'bounced' => $totalBounced,
            'avg_open_rate' => round(($totalOpened / $totalSent) * 100, 2),
            'avg_click_rate' => round(($totalClicked / $totalSent) * 100, 2),
            'avg_bounce_rate' => round(($totalBounced / $totalSent) * 100, 2),
        ];
    }

    /**
     * Get subscriber stats
     */
    private function getSubscriberStats(array $dateRange): array
    {
        return [
            'total_active' => NewsletterSubscription::where('status', 'active')->count(),
            'new' => NewsletterSubscription::where('created_at', '>=', $dateRange['start'])->count(),
            'verified' => NewsletterSubscription::where('verified_at', '>=', $dateRange['start'])->count(),
            'unsubscribed' => NewsletterSubscription::where('unsubscribed_at', '>=', $dateRange['start'])->count(),
            'bounced' => NewsletterSubscription::where('status', 'bounced')
                ->where('updated_at', '>=', $dateRange['start'])->count(),
            'growth_rate' => $this->calculateGrowthRate($dateRange),
        ];
    }

    /**
     * Get engagement stats
     */
    private function getEngagementStats(array $dateRange): array
    {
        $logs = DB::table('email_logs')
            ->where('created_at', '>=', $dateRange['start'])
            ->selectRaw('
                SUM(CASE WHEN event_type = "opened" THEN 1 ELSE 0 END) as opens,
                SUM(CASE WHEN event_type = "clicked" THEN 1 ELSE 0 END) as clicks,
                COUNT(DISTINCT subscriber_id) as unique_engaged
            ')
            ->first();

        $totalSubscribers = NewsletterSubscription::where('status', 'active')->count() ?: 1;

        return [
            'total_opens' => $logs->opens ?? 0,
            'total_clicks' => $logs->clicks ?? 0,
            'unique_engaged' => $logs->unique_engaged ?? 0,
            'engagement_rate' => round((($logs->unique_engaged ?? 0) / $totalSubscribers) * 100, 2),
            'click_to_open_rate' => ($logs->opens ?? 0) > 0
                ? round((($logs->clicks ?? 0) / ($logs->opens ?? 1)) * 100, 2)
                : 0,
        ];
    }

    /**
     * Get deliverability stats
     */
    private function getDeliverabilityStats(array $dateRange): array
    {
        $logs = DB::table('email_logs')
            ->where('created_at', '>=', $dateRange['start'])
            ->selectRaw('
                SUM(CASE WHEN event_type = "sent" THEN 1 ELSE 0 END) as sent,
                SUM(CASE WHEN event_type = "delivered" THEN 1 ELSE 0 END) as delivered,
                SUM(CASE WHEN event_type = "bounced" THEN 1 ELSE 0 END) as bounced,
                SUM(CASE WHEN event_type = "complained" THEN 1 ELSE 0 END) as complaints
            ')
            ->first();

        $sent = $logs->sent ?? 1;

        return [
            'sent' => $sent,
            'delivered' => $logs->delivered ?? 0,
            'bounced' => $logs->bounced ?? 0,
            'complaints' => $logs->complaints ?? 0,
            'delivery_rate' => round((($logs->delivered ?? 0) / $sent) * 100, 2),
            'bounce_rate' => round((($logs->bounced ?? 0) / $sent) * 100, 2),
            'complaint_rate' => round((($logs->complaints ?? 0) / $sent) * 100, 4),
        ];
    }

    /**
     * Get trend data
     */
    private function getTrends(array $dateRange): array
    {
        // Daily email stats
        $dailyStats = EmailCampaign::where('sent_at', '>=', $dateRange['start'])
            ->selectRaw('DATE(sent_at) as date, SUM(sent_count) as sent, SUM(opened_count) as opened, SUM(clicked_count) as clicked')
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        // Daily subscriber growth
        $subscriberGrowth = NewsletterSubscription::where('created_at', '>=', $dateRange['start'])
            ->selectRaw('DATE(created_at) as date, COUNT(*) as new_subscribers')
            ->groupBy('date')
            ->orderBy('date')
            ->pluck('new_subscribers', 'date');

        return [
            'daily_emails' => $dailyStats,
            'subscriber_growth' => $subscriberGrowth,
        ];
    }

    /**
     * Calculate subscriber growth rate
     */
    private function calculateGrowthRate(array $dateRange): float
    {
        $daysDiff = $dateRange['start']->diffInDays($dateRange['end']);
        $previousStart = $dateRange['start']->copy()->subDays($daysDiff);

        $currentNew = NewsletterSubscription::whereBetween('created_at', [$dateRange['start'], $dateRange['end']])->count();
        $previousNew = NewsletterSubscription::whereBetween('created_at', [$previousStart, $dateRange['start']])->count();

        if ($previousNew === 0) {
            return $currentNew > 0 ? 100.0 : 0.0;
        }

        return round((($currentNew - $previousNew) / $previousNew) * 100, 2);
    }

    /**
     * Get date range from period string
     */
    private function getDateRange(string $period): array
    {
        return match ($period) {
            '7d' => ['start' => now()->subDays(7), 'end' => now()],
            '30d' => ['start' => now()->subDays(30), 'end' => now()],
            '90d' => ['start' => now()->subDays(90), 'end' => now()],
            'this_week' => ['start' => now()->startOfWeek(), 'end' => now()->endOfWeek()],
            'last_week' => ['start' => now()->subWeek()->startOfWeek(), 'end' => now()->subWeek()->endOfWeek()],
            'this_month' => ['start' => now()->startOfMonth(), 'end' => now()->endOfMonth()],
            'last_month' => ['start' => now()->subMonth()->startOfMonth(), 'end' => now()->subMonth()->endOfMonth()],
            'this_year' => ['start' => now()->startOfYear(), 'end' => now()],
            default => ['start' => now()->subDays(30), 'end' => now()],
        };
    }
}
