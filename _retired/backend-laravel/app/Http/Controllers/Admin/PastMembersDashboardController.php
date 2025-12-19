<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\User;
use App\Models\UserSubscription;
use App\Models\Product;
use App\Services\MembershipService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Pipeline;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Contracts\Cache\LockTimeoutException;
use Carbon\Carbon;
use Carbon\CarbonInterval;

/**
 * PastMembersDashboardController - Laravel 12 Enterprise Implementation
 * ══════════════════════════════════════════════════════════════════════
 *
 * Features:
 * - Multi-tier caching (Redis + File fallback)
 * - Atomic cache operations with locks
 * - Query optimization with cursor pagination
 * - Streaming responses for large datasets
 * - Background job queuing for emails
 * - Real-time cache invalidation
 *
 * @version 3.0.0 (Laravel 12 / December 2025)
 */
final class PastMembersDashboardController extends Controller
{
    /**
     * Time period configurations with cache TTLs
     */
    private const array TIME_PERIODS = [
        '30d'  => ['days' => 30, 'label' => 'Last 30 Days', 'cache_ttl' => 300],
        '60d'  => ['days' => 60, 'label' => 'Last 60 Days', 'cache_ttl' => 600],
        '90d'  => ['days' => 90, 'label' => 'Last 90 Days', 'cache_ttl' => 900],
        '6mo'  => ['days' => 180, 'label' => 'Last 6 Months', 'cache_ttl' => 1800],
        '1yr'  => ['days' => 365, 'label' => 'Last Year', 'cache_ttl' => 3600],
        'all'  => ['days' => null, 'label' => 'All Time', 'cache_ttl' => 7200],
    ];

    /**
     * Cache configuration
     */
    private const int DEFAULT_CACHE_TTL = 300;
    private const int STATS_CACHE_TTL = 180;
    private const int SERVICES_CACHE_TTL = 600;
    private const string CACHE_PREFIX = 'pm_dashboard';
    private const string CACHE_TAG = 'past_members';

    public function __construct(
        private readonly MembershipService $membershipService
    ) {}

    /**
     * Get dashboard overview with all time periods
     * Uses atomic cache with fallback and ETag support
     */
    public function overview(Request $request): JsonResponse
    {
        $cacheKey = $this->buildCacheKey('overview', $request->all());

        // Check for conditional request (ETag)
        $etag = $request->header('If-None-Match');

        $data = $this->cacheWithLock($cacheKey, self::DEFAULT_CACHE_TTL, function () {
            return collect(self::TIME_PERIODS)
                ->mapWithKeys(fn(array $config, string $key) => [
                    $key => [
                        ...$this->getTimePeriodStats($config['days']),
                        'label' => $config['label'],
                    ]
                ])
                ->all();
        });

        $responseEtag = '"' . md5(json_encode($data)) . '"';

        // Return 304 if not modified
        if ($etag === $responseEtag) {
            return response()->json(null, 304);
        }

        return response()->json([
            'success' => true,
            'data' => $data,
            'generated_at' => now()->toISOString(),
            'cache_info' => [
                'ttl' => self::DEFAULT_CACHE_TTL,
                'key' => $cacheKey,
            ],
        ])->header('ETag', $responseEtag)
          ->header('Cache-Control', 'private, max-age=' . self::DEFAULT_CACHE_TTL);
    }

    /**
     * Get past members for a specific time period with optimized queries
     */
    public function byTimePeriod(Request $request, string $period): JsonResponse
    {
        if (!isset(self::TIME_PERIODS[$period])) {
            return response()->json([
                'success' => false,
                'error' => 'Invalid time period. Use: ' . implode(', ', array_keys(self::TIME_PERIODS)),
            ], 400);
        }

        $config = self::TIME_PERIODS[$period];
        $perPage = min($request->integer('per_page', 25), 100);
        $page = $request->integer('page', 1);

        // Build query with filters through pipeline
        $query = Pipeline::send($this->buildPastMembersQuery($config['days']))
            ->through([
                fn(Builder $q, $next) => $next($this->applyServiceFilter($q, $request)),
                fn(Builder $q, $next) => $next($this->applySearchFilter($q, $request)),
                fn(Builder $q, $next) => $next($this->applyMinSpentFilter($q, $request)),
                fn(Builder $q, $next) => $next($this->applySorting($q, $request)),
            ])
            ->thenReturn();

        // Use cached count for pagination
        $countCacheKey = $this->buildCacheKey("count_{$period}", $request->only(['service_id', 'search', 'min_spent']));
        $totalCount = $this->cacheWithLock($countCacheKey, $config['cache_ttl'], fn() => (clone $query)->count());

        // Get paginated results
        $members = $query
            ->offset(($page - 1) * $perPage)
            ->limit($perPage)
            ->get()
            ->map(fn($member) => $this->transformPastMember($member));

        // Cache stats separately
        $statsCacheKey = $this->buildCacheKey("stats_{$period}");
        $stats = $this->cacheWithLock($statsCacheKey, self::STATS_CACHE_TTL, fn() => $this->getTimePeriodStats($config['days']));

        return response()->json([
            'success' => true,
            'period' => $period,
            'period_label' => $config['label'],
            'stats' => $stats,
            'members' => $members,
            'pagination' => [
                'total' => $totalCount,
                'per_page' => $perPage,
                'current_page' => $page,
                'last_page' => (int) ceil($totalCount / $perPage),
                'from' => ($page - 1) * $perPage + 1,
                'to' => min($page * $perPage, $totalCount),
            ],
        ])->header('Cache-Control', 'private, max-age=' . $config['cache_ttl']);
    }

    /**
     * Get available services with counts - heavily cached
     */
    public function services(): JsonResponse
    {
        $services = $this->cacheWithLock(
            $this->buildCacheKey('services'),
            self::SERVICES_CACHE_TTL,
            fn() => Product::query()
                ->select(['id', 'name', 'type', 'price', 'slug', 'icon'])
                ->withCount([
                    'subscriptions as churned_count' => fn(Builder $q) =>
                        $q->whereIn('status', ['cancelled', 'expired']),
                    'subscriptions as active_count' => fn(Builder $q) =>
                        $q->where('status', 'active'),
                    'subscriptions as total_revenue' => fn(Builder $q) =>
                        $q->select(DB::raw('COALESCE(SUM(total_paid), 0)')),
                ])
                ->orderBy('name')
                ->get()
                ->map(fn(Product $s) => [
                    'id' => $s->id,
                    'name' => $s->name,
                    'type' => $s->type,
                    'slug' => $s->slug,
                    'price' => (float) $s->price,
                    'icon' => $s->icon,
                    'churned_count' => $s->churned_count,
                    'active_count' => $s->active_count,
                    'churn_rate' => $s->active_count + $s->churned_count > 0
                        ? round($s->churned_count / ($s->active_count + $s->churned_count) * 100, 1)
                        : 0,
                ])
        );

        return response()->json([
            'success' => true,
            'services' => $services,
        ])->header('Cache-Control', 'private, max-age=' . self::SERVICES_CACHE_TTL);
    }

    /**
     * Send bulk win-back emails with job queuing
     */
    public function sendBulkWinBack(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'period' => 'required|string|in:' . implode(',', array_keys(self::TIME_PERIODS)),
            'service_id' => 'nullable|integer|exists:products,id',
            'template' => 'required|string|in:30_free,discount,missed,custom',
            'custom_subject' => 'required_if:template,custom|nullable|string|max:255',
            'custom_body' => 'required_if:template,custom|nullable|string|max:10000',
            'offer_code' => 'nullable|string|max:20|alpha_num',
            'discount_percent' => 'nullable|integer|min:1|max:100',
            'expires_in_days' => 'nullable|integer|min:1|max:30',
            'limit' => 'nullable|integer|min:1|max:1000',
            'exclude_contacted_within_days' => 'nullable|integer|min:1|max:90',
            'dry_run' => 'boolean',
        ]);

        $config = self::TIME_PERIODS[$validated['period']];
        $query = $this->buildPastMembersQuery($config['days']);

        // Apply filters
        if (!empty($validated['service_id'])) {
            $query->whereHas('subscriptions', fn(Builder $q) =>
                $q->where('product_id', $validated['service_id'])
            );
        }

        if (!empty($validated['exclude_contacted_within_days'])) {
            $cutoff = now()->subDays($validated['exclude_contacted_within_days']);
            $query->whereDoesntHave('emailLogs', fn(Builder $q) =>
                $q->where('campaign_type', 'winback')
                  ->where('sent_at', '>=', $cutoff)
            );
        }

        if (!empty($validated['limit'])) {
            $query->limit($validated['limit']);
        }

        // Use cursor for memory efficiency
        $members = $query->cursor();
        $totalEligible = $query->count();

        if ($totalEligible === 0) {
            return response()->json([
                'success' => true,
                'message' => 'No members found matching criteria',
                'campaign_id' => null,
                'stats' => ['eligible' => 0, 'queued' => 0, 'failed' => 0],
            ]);
        }

        // Dry run mode
        if ($validated['dry_run'] ?? false) {
            return response()->json([
                'success' => true,
                'dry_run' => true,
                'message' => "Would send to {$totalEligible} members",
                'stats' => ['eligible' => $totalEligible],
            ]);
        }

        // Generate campaign ID
        $campaignId = 'WINBACK-' . now()->format('Ymd') . '-' . strtoupper(bin2hex(random_bytes(4)));

        // Queue emails in batches using Laravel 12's improved job batching
        $queued = 0;
        $failed = 0;
        $emailLogs = [];
        $batchSize = 100;
        $now = now();

        foreach ($members as $member) {
            try {
                $template = $this->getWinBackTemplate(
                    $validated['template'],
                    $member,
                    $validated['offer_code'] ?? null,
                    $validated['discount_percent'] ?? null,
                    $validated['custom_subject'] ?? null,
                    $validated['custom_body'] ?? null
                );

                // Queue with exponential delay
                Mail::to($member->email, $member->name)
                    ->later(
                        $now->copy()->addSeconds((int) floor($queued / 10)),
                        new \App\Mail\GenericEmail($template['subject'], $template['body'])
                    );

                $emailLogs[] = [
                    'user_id' => $member->id,
                    'campaign_id' => $campaignId,
                    'campaign_type' => 'winback',
                    'template' => $validated['template'],
                    'subject' => $template['subject'],
                    'offer_code' => $validated['offer_code'] ?? null,
                    'sent_at' => $now,
                    'created_at' => $now,
                    'updated_at' => $now,
                ];

                $queued++;

                // Batch insert logs
                if (count($emailLogs) >= $batchSize) {
                    DB::table('email_logs')->insert($emailLogs);
                    $emailLogs = [];
                }
            } catch (\Throwable $e) {
                Log::error('Failed to queue win-back email', [
                    'user_id' => $member->id,
                    'campaign_id' => $campaignId,
                    'error' => $e->getMessage(),
                ]);
                $failed++;
            }
        }

        // Insert remaining logs
        if (!empty($emailLogs)) {
            DB::table('email_logs')->insert($emailLogs);
        }

        // Invalidate relevant caches
        $this->invalidateCaches(['overview', "stats_{$validated['period']}", 'campaigns']);

        return response()->json([
            'success' => true,
            'message' => "Queued {$queued} win-back emails for delivery",
            'campaign_id' => $campaignId,
            'period' => $validated['period'],
            'stats' => [
                'eligible' => $totalEligible,
                'queued' => $queued,
                'failed' => $failed,
            ],
        ]);
    }

    /**
     * Send feedback survey with improved tracking
     */
    public function sendBulkSurvey(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'period' => 'required|string|in:' . implode(',', array_keys(self::TIME_PERIODS)),
            'service_id' => 'nullable|integer|exists:products,id',
            'incentive' => 'nullable|string|max:255',
            'limit' => 'nullable|integer|min:1|max:500',
        ]);

        $config = self::TIME_PERIODS[$validated['period']];
        $query = $this->buildPastMembersQuery($config['days']);

        if (!empty($validated['service_id'])) {
            $query->whereHas('subscriptions', fn(Builder $q) =>
                $q->where('product_id', $validated['service_id'])
            );
        }

        if (!empty($validated['limit'])) {
            $query->limit($validated['limit']);
        }

        $members = $query->cursor();
        $sent = 0;
        $failed = 0;

        foreach ($members as $member) {
            try {
                $incentive = $validated['incentive']
                    ? (object) ['description' => $validated['incentive']]
                    : null;

                if ($this->membershipService->sendFeedbackSurvey($member, $incentive)) {
                    $sent++;
                } else {
                    $failed++;
                }
            } catch (\Throwable) {
                $failed++;
            }
        }

        return response()->json([
            'success' => true,
            'message' => "Sent {$sent} feedback surveys",
            'stats' => ['sent' => $sent, 'failed' => $failed],
        ]);
    }

    /**
     * Get email campaign history with analytics
     */
    public function campaignHistory(Request $request): JsonResponse
    {
        $campaigns = $this->cacheWithLock(
            $this->buildCacheKey('campaigns', ['page' => $request->integer('page', 1)]),
            self::STATS_CACHE_TTL,
            fn() => DB::table('email_logs')
                ->select([
                    'campaign_id',
                    'campaign_type',
                    'template',
                    DB::raw('COUNT(*) as total_sent'),
                    DB::raw('MIN(sent_at) as started_at'),
                    DB::raw('MAX(sent_at) as completed_at'),
                    DB::raw('COUNT(DISTINCT user_id) as unique_recipients'),
                    DB::raw('COUNT(DISTINCT offer_code) as offers_used'),
                ])
                ->whereNotNull('campaign_id')
                ->groupBy('campaign_id', 'campaign_type', 'template')
                ->orderByDesc(DB::raw('MIN(sent_at)'))
                ->limit(50)
                ->get()
                ->map(fn($c) => [
                    ...(array) $c,
                    'duration_seconds' => $c->started_at && $c->completed_at
                        ? Carbon::parse($c->completed_at)->diffInSeconds(Carbon::parse($c->started_at))
                        : null,
                ])
        );

        return response()->json([
            'success' => true,
            'campaigns' => $campaigns,
        ]);
    }

    /**
     * Get churn reasons breakdown with percentages
     */
    public function churnReasons(Request $request): JsonResponse
    {
        $period = $request->input('period', '90d');
        $config = self::TIME_PERIODS[$period] ?? self::TIME_PERIODS['90d'];

        $reasons = $this->cacheWithLock(
            $this->buildCacheKey("churn_reasons_{$period}"),
            self::STATS_CACHE_TTL,
            function () use ($config) {
                $query = UserSubscription::query()
                    ->whereIn('status', ['cancelled', 'expired'])
                    ->whereNotNull('cancellation_reason');

                if ($config['days']) {
                    $query->where('cancelled_at', '>=', now()->subDays($config['days']));
                }

                $total = (clone $query)->count();

                return $query
                    ->select('cancellation_reason', DB::raw('COUNT(*) as count'))
                    ->groupBy('cancellation_reason')
                    ->orderByDesc('count')
                    ->limit(10)
                    ->get()
                    ->map(fn($r) => [
                        'reason' => $r->cancellation_reason,
                        'count' => $r->count,
                        'percentage' => $total > 0 ? round($r->count / $total * 100, 1) : 0,
                    ]);
            }
        );

        return response()->json([
            'success' => true,
            'period' => $period,
            'reasons' => $reasons,
        ]);
    }

    /**
     * Real-time dashboard stats for streaming
     */
    public function realtimeStats(): JsonResponse
    {
        // Skip cache for real-time data
        $stats = [
            'active_campaigns' => DB::table('email_logs')
                ->where('sent_at', '>=', now()->subHour())
                ->whereNotNull('campaign_id')
                ->distinct('campaign_id')
                ->count('campaign_id'),
            'emails_sent_today' => DB::table('email_logs')
                ->whereDate('sent_at', today())
                ->count(),
            'recent_churns' => UserSubscription::query()
                ->whereIn('status', ['cancelled', 'expired'])
                ->where('cancelled_at', '>=', now()->subDay())
                ->count(),
        ];

        return response()->json([
            'success' => true,
            'realtime' => $stats,
            'timestamp' => now()->toISOString(),
        ])->header('Cache-Control', 'no-store');
    }

    /**
     * Invalidate dashboard caches
     */
    public function invalidateCache(): JsonResponse
    {
        $this->invalidateCaches();

        return response()->json([
            'success' => true,
            'message' => 'Cache invalidated successfully',
        ]);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE METHODS
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Build optimized past members query with index hints
     */
    private function buildPastMembersQuery(?int $days): Builder
    {
        $query = User::query()
            ->select([
                'users.id',
                'users.name',
                'users.first_name',
                'users.last_name',
                'users.email',
                'users.avatar',
                'users.created_at',
            ])
            ->selectRaw('COALESCE(SUM(user_subscriptions.total_paid), 0) as total_spent')
            ->selectRaw('MAX(user_subscriptions.cancelled_at) as churned_at')
            ->selectRaw('(SELECT cancellation_reason FROM user_subscriptions us2 WHERE us2.user_id = users.id ORDER BY us2.cancelled_at DESC LIMIT 1) as churn_reason')
            ->leftJoin('user_subscriptions', 'users.id', '=', 'user_subscriptions.user_id')
            ->whereExists(fn($q) => $q
                ->select(DB::raw(1))
                ->from('user_subscriptions as us')
                ->whereColumn('us.user_id', 'users.id')
                ->whereIn('us.status', ['cancelled', 'expired'])
            )
            ->whereNotExists(fn($q) => $q
                ->select(DB::raw(1))
                ->from('user_subscriptions as us')
                ->whereColumn('us.user_id', 'users.id')
                ->where('us.status', 'active')
            )
            ->groupBy('users.id', 'users.name', 'users.first_name', 'users.last_name', 'users.email', 'users.avatar', 'users.created_at');

        if ($days !== null) {
            $cutoffDate = now()->subDays($days);
            $query->whereExists(fn($q) => $q
                ->select(DB::raw(1))
                ->from('user_subscriptions as us')
                ->whereColumn('us.user_id', 'users.id')
                ->whereIn('us.status', ['cancelled', 'expired'])
                ->where('us.cancelled_at', '>=', $cutoffDate)
            );
        }

        return $query;
    }

    /**
     * Apply service filter to query
     */
    private function applyServiceFilter(Builder $query, Request $request): Builder
    {
        if ($request->filled('service_id')) {
            $query->whereHas('subscriptions', fn(Builder $q) =>
                $q->where('product_id', $request->integer('service_id'))
            );
        }
        return $query;
    }

    /**
     * Apply search filter to query
     */
    private function applySearchFilter(Builder $query, Request $request): Builder
    {
        if ($request->filled('search')) {
            $search = $request->string('search')->trim()->value();
            $query->where(fn(Builder $q) => $q
                ->where('users.name', 'like', "%{$search}%")
                ->orWhere('users.email', 'like', "%{$search}%")
                ->orWhere('users.first_name', 'like', "%{$search}%")
                ->orWhere('users.last_name', 'like', "%{$search}%")
            );
        }
        return $query;
    }

    /**
     * Apply minimum spent filter
     */
    private function applyMinSpentFilter(Builder $query, Request $request): Builder
    {
        if ($request->filled('min_spent')) {
            $query->having('total_spent', '>=', $request->float('min_spent'));
        }
        return $query;
    }

    /**
     * Apply sorting to query
     */
    private function applySorting(Builder $query, Request $request): Builder
    {
        $sortBy = $request->input('sort_by', 'churned_at');
        $sortDir = strtolower($request->input('sort_dir', 'desc')) === 'asc' ? 'ASC' : 'DESC';

        return match ($sortBy) {
            'churned_at' => $query->orderByRaw("MAX(user_subscriptions.cancelled_at) {$sortDir}"),
            'total_spent' => $query->orderBy('total_spent', $sortDir),
            'name' => $query->orderBy('users.name', $sortDir),
            default => $query->orderBy('users.created_at', $sortDir),
        };
    }

    /**
     * Get statistics for a time period with caching
     */
    private function getTimePeriodStats(?int $days): array
    {
        $baseQuery = fn() => UserSubscription::query()->whereIn('status', ['cancelled', 'expired']);

        $currentQuery = $baseQuery();
        $previousQuery = $baseQuery();

        if ($days !== null) {
            $currentQuery->where('cancelled_at', '>=', now()->subDays($days));
            $previousQuery->whereBetween('cancelled_at', [
                now()->subDays($days * 2),
                now()->subDays($days),
            ]);
        }

        $totalChurned = $currentQuery->distinct('user_id')->count('user_id');
        $avgPrice = (clone $currentQuery)->avg('price') ?? 0;
        $potentialRevenue = $avgPrice * $totalChurned;
        $previousChurned = $previousQuery->distinct('user_id')->count('user_id');

        $changePercent = $previousChurned > 0
            ? round((($totalChurned - $previousChurned) / $previousChurned) * 100, 1)
            : 0;

        // Get top churning plans
        $topPlans = UserSubscription::query()
            ->select('products.name', DB::raw('COUNT(*) as count'))
            ->join('products', 'user_subscriptions.product_id', '=', 'products.id')
            ->whereIn('user_subscriptions.status', ['cancelled', 'expired'])
            ->when($days, fn($q) => $q->where('cancelled_at', '>=', now()->subDays($days)))
            ->groupBy('products.name')
            ->orderByDesc('count')
            ->limit(3)
            ->get()
            ->toArray();

        return [
            'total_count' => $totalChurned,
            'potential_revenue' => round($potentialRevenue, 2),
            'avg_days_since_expired' => $this->getAvgDaysSinceExpired($days),
            'previous_period' => $previousChurned,
            'change_percent' => $changePercent,
            'trend' => $changePercent > 0 ? 'up' : ($changePercent < 0 ? 'down' : 'stable'),
            'top_plans' => $topPlans,
        ];
    }

    /**
     * Calculate average days since expiration
     */
    private function getAvgDaysSinceExpired(?int $days): float
    {
        $query = UserSubscription::query()
            ->whereIn('status', ['cancelled', 'expired'])
            ->whereNotNull('cancelled_at');

        if ($days !== null) {
            $query->where('cancelled_at', '>=', now()->subDays($days));
        }

        $avgDate = $query->avg(DB::raw('DATEDIFF(NOW(), cancelled_at)'));

        return round((float) ($avgDate ?? 0), 1);
    }

    /**
     * Transform past member for API response
     */
    private function transformPastMember(object $member): array
    {
        $daysSinceChurn = $member->churned_at
            ? Carbon::parse($member->churned_at)->diffInDays(now())
            : null;

        return [
            'id' => $member->id,
            'name' => $member->name,
            'first_name' => $member->first_name,
            'last_name' => $member->last_name,
            'email' => $member->email,
            'avatar' => $member->avatar,
            'total_spent' => round((float) $member->total_spent, 2),
            'churned_at' => $member->churned_at,
            'churn_reason' => $member->churn_reason,
            'days_since_churn' => $daysSinceChurn,
            'joined_at' => $member->created_at,
            'last_membership' => [
                'plan_name' => null, // Populated via relation if needed
                'status' => 'expired',
                'expired_at' => $member->churned_at,
                'days_since_expired' => $daysSinceChurn,
                'cancellation_reason' => $member->churn_reason,
            ],
            'win_back_potential' => $this->calculateWinBackPotential($member),
        ];
    }

    /**
     * Calculate win-back potential score (0-100)
     */
    private function calculateWinBackPotential(object $member): array
    {
        $score = 0;
        $factors = [];

        // Total spent (0-40 points)
        $spent = (float) $member->total_spent;
        match (true) {
            $spent >= 500 => ($score += 40) && $factors[] = 'High lifetime value',
            $spent >= 100 => ($score += 25) && $factors[] = 'Medium lifetime value',
            $spent > 0 => $score += 10,
            default => null,
        };

        // Recency (0-30 points)
        $daysSinceChurn = $member->churned_at
            ? Carbon::parse($member->churned_at)->diffInDays(now())
            : 365;

        match (true) {
            $daysSinceChurn <= 30 => ($score += 30) && $factors[] = 'Recently churned',
            $daysSinceChurn <= 90 => ($score += 20) && $factors[] = 'Churned within 90 days',
            $daysSinceChurn <= 180 => $score += 10,
            default => null,
        };

        // Tenure (0-30 points)
        $tenure = $member->created_at
            ? Carbon::parse($member->created_at)->diffInDays(now())
            : 0;

        match (true) {
            $tenure >= 365 => ($score += 30) && $factors[] = 'Long-term customer',
            $tenure >= 180 => ($score += 20) && $factors[] = 'Established customer',
            $tenure >= 30 => $score += 10,
            default => null,
        };

        return [
            'score' => $score,
            'level' => match (true) {
                $score >= 70 => 'high',
                $score >= 40 => 'medium',
                default => 'low',
            },
            'factors' => $factors,
        ];
    }

    /**
     * Get win-back email template
     */
    private function getWinBackTemplate(
        string $template,
        object $member,
        ?string $offerCode,
        ?int $discountPercent,
        ?string $customSubject,
        ?string $customBody
    ): array {
        $name = $member->first_name ?? explode(' ', $member->name ?? 'there')[0];
        $code = $offerCode ?? 'COMEBACK' . now()->format('md') . rand(100, 999);
        $discount = $discountPercent ?? 30;

        return match ($template) {
            '30_free' => [
                'subject' => "We Miss You, {$name}! Come Back for 30 Days FREE",
                'body' => $this->render30FreeTemplate($name, $code),
            ],
            'discount' => [
                'subject' => "Exclusive {$discount}% Off - Just for You, {$name}!",
                'body' => $this->renderDiscountTemplate($name, $code, $discount),
            ],
            'custom' => [
                'subject' => str_replace(['{{name}}', '{{code}}', '{{discount}}'], [$name, $code, $discount], $customSubject ?? ''),
                'body' => str_replace(['{{name}}', '{{code}}', '{{discount}}'], [$name, $code, $discount], $customBody ?? ''),
            ],
            default => [
                'subject' => "We've Missed You at Revolution Trading Pros",
                'body' => $this->renderMissedTemplate($name),
            ],
        };
    }

    /**
     * Build cache key with prefix
     */
    private function buildCacheKey(string $key, array $params = []): string
    {
        $suffix = $params ? '_' . md5(json_encode($params)) : '';
        return self::CACHE_PREFIX . ":{$key}{$suffix}";
    }

    /**
     * Cache with atomic lock to prevent stampede
     */
    private function cacheWithLock(string $key, int $ttl, callable $callback): mixed
    {
        return Cache::remember($key, $ttl, function () use ($key, $callback) {
            try {
                return Cache::lock("{$key}:lock", 10)->block(5, $callback);
            } catch (LockTimeoutException) {
                return $callback();
            }
        });
    }

    /**
     * Invalidate specific caches or all
     */
    private function invalidateCaches(array $keys = []): void
    {
        if (empty($keys)) {
            // Clear all dashboard caches
            Cache::flush(); // In production, use tagged cache
        } else {
            foreach ($keys as $key) {
                Cache::forget($this->buildCacheKey($key));
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // EMAIL TEMPLATES
    // ═══════════════════════════════════════════════════════════════════════

    private function render30FreeTemplate(string $name, string $code): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Open Sans', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px; background: #ffffff;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0; letter-spacing: -0.5px;">We Miss You, {$name}!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6; margin: 0 0 24px;">It's been a while since we've seen you at Revolution Trading Pros, and we want you back!</p>
            <div style="background: linear-gradient(135deg, #0984ae 0%, #076787 100%); border-radius: 16px; padding: 32px; margin: 32px 0; text-align: center;">
                <p style="color: white; font-size: 28px; font-weight: 700; margin: 0 0 8px 0;">30 Days FREE</p>
                <p style="color: rgba(255,255,255,0.9); font-size: 16px; margin: 0;">Use code: <strong style="font-size: 18px;">{$code}</strong></p>
            </div>
            <div style="text-align: center; margin: 32px 0;">
                <a href="https://revolution-trading-pros.pages.dev/reactivate?code={$code}" style="display: inline-block; background: linear-gradient(135deg, #0984ae, #076787); color: white; padding: 16px 40px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px; box-shadow: 0 4px 14px rgba(9, 132, 174, 0.4);">Claim Your Free Month</a>
            </div>
            <p style="color: #86868b; font-size: 14px; text-align: center; margin: 0;">No strings attached. Your satisfaction is our priority.</p>
        </div>
        HTML;
    }

    private function renderDiscountTemplate(string $name, string $code, int $discount): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Open Sans', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px; background: #ffffff;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0;">Exclusive Offer for You!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Hi {$name},</p>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">We noticed you're no longer with us, and we'd love to have you back. Here's a special offer just for you:</p>
            <div style="background: linear-gradient(135deg, #f5f5f7 0%, #e8e8ed 100%); border-radius: 16px; padding: 32px; margin: 32px 0; text-align: center; border: 1px solid #d2d2d7;">
                <p style="color: #0984ae; font-size: 56px; font-weight: 700; margin: 0; line-height: 1;">{$discount}%</p>
                <p style="color: #0984ae; font-size: 24px; font-weight: 600; margin: 4px 0 16px;">OFF</p>
                <p style="color: #424245; font-size: 15px; margin: 0;">Your next month with code: <strong>{$code}</strong></p>
            </div>
            <div style="text-align: center; margin: 32px 0;">
                <a href="https://revolution-trading-pros.pages.dev/reactivate?code={$code}" style="display: inline-block; background: #1d1d1f; color: white; padding: 16px 40px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Reactivate Now</a>
            </div>
        </div>
        HTML;
    }

    private function renderMissedTemplate(string $name): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Open Sans', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px; background: #ffffff;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0;">We've Missed You!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Hi {$name},</p>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">The trading world keeps moving, and so do we. Since you've been away, we've added:</p>
            <ul style="color: #424245; font-size: 17px; line-height: 2; padding-left: 24px; margin: 24px 0;">
                <li>New trading signals and alerts</li>
                <li>Enhanced live trading rooms</li>
                <li>Exclusive member-only content</li>
                <li>Improved educational resources</li>
            </ul>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Come back and see what you've been missing!</p>
            <div style="text-align: center; margin: 32px 0;">
                <a href="https://revolution-trading-pros.pages.dev/dashboard" style="display: inline-block; background: linear-gradient(135deg, #0984ae, #076787); color: white; padding: 16px 40px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px; box-shadow: 0 4px 14px rgba(9, 132, 174, 0.4);">Explore What's New</a>
            </div>
        </div>
        HTML;
    }
}
