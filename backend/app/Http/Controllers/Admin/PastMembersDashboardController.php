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
use Carbon\Carbon;

/**
 * PastMembersDashboardController - Enterprise-grade past member management
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Features:
 * - Time-based filtering (30d, 60d, 90d, 6mo, 1yr)
 * - Bulk email campaigns with progress tracking
 * - Service-based filtering
 * - Win-back automation
 * - Performance optimized with caching
 *
 * @version 2.0.0
 */
class PastMembersDashboardController extends Controller
{
    /**
     * Time period configurations
     */
    private const TIME_PERIODS = [
        '30d'  => ['days' => 30, 'label' => 'Last 30 Days'],
        '60d'  => ['days' => 60, 'label' => 'Last 60 Days'],
        '90d'  => ['days' => 90, 'label' => 'Last 90 Days'],
        '6mo'  => ['days' => 180, 'label' => 'Last 6 Months'],
        '1yr'  => ['days' => 365, 'label' => 'Last Year'],
        'all'  => ['days' => null, 'label' => 'All Time'],
    ];

    /**
     * Cache TTL in seconds
     */
    private const CACHE_TTL = 300; // 5 minutes

    public function __construct(
        protected MembershipService $membershipService
    ) {}

    /**
     * Get dashboard overview with all time periods
     */
    public function overview(Request $request): JsonResponse
    {
        $cacheKey = 'past_members_overview_' . md5(json_encode($request->all()));

        $data = Cache::remember($cacheKey, self::CACHE_TTL, function () {
            $overview = [];

            foreach (self::TIME_PERIODS as $key => $config) {
                $overview[$key] = $this->getTimePeriodStats($config['days']);
                $overview[$key]['label'] = $config['label'];
            }

            return $overview;
        });

        return response()->json([
            'success' => true,
            'data' => $data,
            'generated_at' => now()->toISOString(),
        ]);
    }

    /**
     * Get past members for a specific time period
     */
    public function byTimePeriod(Request $request, string $period): JsonResponse
    {
        if (!isset(self::TIME_PERIODS[$period])) {
            return response()->json([
                'success' => false,
                'error' => 'Invalid time period. Use: 30d, 60d, 90d, 6mo, 1yr, all',
            ], 400);
        }

        $config = self::TIME_PERIODS[$period];
        $perPage = min($request->input('per_page', 25), 100);

        $query = $this->buildPastMembersQuery($config['days']);

        // Apply filters
        if ($request->filled('service_id')) {
            $query->whereHas('subscriptions', fn($q) =>
                $q->where('product_id', $request->service_id)
            );
        }

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%");
            });
        }

        if ($request->filled('min_spent')) {
            $query->having('total_spent', '>=', $request->min_spent);
        }

        // Sorting
        $sortBy = $request->input('sort_by', 'churned_at');
        $sortDir = $request->input('sort_dir', 'desc');

        if ($sortBy === 'churned_at') {
            $query->orderByRaw('MAX(user_subscriptions.cancelled_at) ' . $sortDir);
        } elseif ($sortBy === 'total_spent') {
            $query->orderBy('total_spent', $sortDir);
        } else {
            $query->orderBy('created_at', $sortDir);
        }

        $members = $query->paginate($perPage);

        // Transform data
        $members->getCollection()->transform(function ($member) {
            return $this->transformPastMember($member);
        });

        return response()->json([
            'success' => true,
            'period' => $period,
            'period_label' => $config['label'],
            'stats' => $this->getTimePeriodStats($config['days']),
            'members' => $members->items(),
            'pagination' => [
                'total' => $members->total(),
                'per_page' => $members->perPage(),
                'current_page' => $members->currentPage(),
                'last_page' => $members->lastPage(),
            ],
        ]);
    }

    /**
     * Get available services for filtering
     */
    public function services(): JsonResponse
    {
        $services = Cache::remember('past_members_services', self::CACHE_TTL, function () {
            return Product::withCount(['subscriptions as churned_count' => function ($q) {
                $q->whereIn('status', ['cancelled', 'expired']);
            }])
            ->withCount(['subscriptions as active_count' => function ($q) {
                $q->where('status', 'active');
            }])
            ->orderBy('name')
            ->get(['id', 'name', 'type', 'price'])
            ->map(fn($s) => [
                'id' => $s->id,
                'name' => $s->name,
                'type' => $s->type,
                'price' => $s->price,
                'churned_count' => $s->churned_count,
                'active_count' => $s->active_count,
            ]);
        });

        return response()->json([
            'success' => true,
            'services' => $services,
        ]);
    }

    /**
     * Send bulk win-back emails for a time period
     */
    public function sendBulkWinBack(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'period' => 'required|string|in:30d,60d,90d,6mo,1yr,all',
            'service_id' => 'nullable|integer|exists:products,id',
            'template' => 'required|string|in:30_free,discount,missed,custom',
            'custom_subject' => 'required_if:template,custom|string|max:255',
            'custom_body' => 'required_if:template,custom|string',
            'offer_code' => 'nullable|string|max:20',
            'discount_percent' => 'nullable|integer|min:1|max:100',
            'expires_in_days' => 'nullable|integer|min:1|max:30',
            'limit' => 'nullable|integer|min:1|max:1000',
            'exclude_contacted_within_days' => 'nullable|integer|min:1|max:90',
        ]);

        $config = self::TIME_PERIODS[$validated['period']];
        $query = $this->buildPastMembersQuery($config['days']);

        // Apply service filter
        if (!empty($validated['service_id'])) {
            $query->whereHas('subscriptions', fn($q) =>
                $q->where('product_id', $validated['service_id'])
            );
        }

        // Exclude recently contacted
        if (!empty($validated['exclude_contacted_within_days'])) {
            $cutoff = now()->subDays($validated['exclude_contacted_within_days']);
            $query->whereDoesntHave('emailLogs', fn($q) =>
                $q->where('campaign_type', 'winback')
                  ->where('sent_at', '>=', $cutoff)
            );
        }

        // Apply limit
        if (!empty($validated['limit'])) {
            $query->limit($validated['limit']);
        }

        $members = $query->get();
        $totalToSend = $members->count();

        if ($totalToSend === 0) {
            return response()->json([
                'success' => true,
                'message' => 'No members found matching criteria',
                'sent' => 0,
                'failed' => 0,
            ]);
        }

        // Create a campaign tracking record
        $campaignId = 'WINBACK-' . strtoupper(substr(md5(now()->toString()), 0, 8));

        // Queue emails in batches
        $batchSize = 50;
        $sent = 0;
        $failed = 0;
        $emailLogs = [];
        $now = now();

        foreach ($members->chunk($batchSize) as $batch) {
            foreach ($batch as $member) {
                try {
                    // Get template content
                    $template = $this->getWinBackTemplate(
                        $validated['template'],
                        $member,
                        $validated['offer_code'] ?? null,
                        $validated['discount_percent'] ?? null,
                        $validated['custom_subject'] ?? null,
                        $validated['custom_body'] ?? null
                    );

                    // Queue email with delay to prevent rate limiting
                    Mail::to($member->email, $member->name)
                        ->later($now->addSeconds($sent), new \App\Mail\GenericEmail(
                            $template['subject'],
                            $template['body']
                        ));

                    $emailLogs[] = [
                        'user_id' => $member->id,
                        'campaign_id' => $campaignId,
                        'campaign_type' => 'winback',
                        'template' => $validated['template'],
                        'subject' => $template['subject'],
                        'offer_code' => $validated['offer_code'] ?? null,
                        'sent_at' => $now,
                        'created_at' => $now,
                    ];

                    $sent++;
                } catch (\Exception $e) {
                    Log::error('Failed to queue win-back email', [
                        'user_id' => $member->id,
                        'error' => $e->getMessage(),
                    ]);
                    $failed++;
                }
            }

            // Batch insert logs
            if (count($emailLogs) >= $batchSize) {
                DB::table('email_logs')->insert($emailLogs);
                $emailLogs = [];
            }
        }

        // Insert remaining logs
        if (!empty($emailLogs)) {
            DB::table('email_logs')->insert($emailLogs);
        }

        // Clear cache
        Cache::tags(['past_members'])->flush();

        return response()->json([
            'success' => true,
            'message' => "Queued {$sent} win-back emails for delivery",
            'campaign_id' => $campaignId,
            'period' => $validated['period'],
            'sent' => $sent,
            'failed' => $failed,
            'total_eligible' => $totalToSend,
        ]);
    }

    /**
     * Send feedback survey to a time period
     */
    public function sendBulkSurvey(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'period' => 'required|string|in:30d,60d,90d,6mo,1yr,all',
            'service_id' => 'nullable|integer|exists:products,id',
            'incentive' => 'nullable|string|max:255',
            'limit' => 'nullable|integer|min:1|max:500',
        ]);

        $config = self::TIME_PERIODS[$validated['period']];
        $query = $this->buildPastMembersQuery($config['days']);

        if (!empty($validated['service_id'])) {
            $query->whereHas('subscriptions', fn($q) =>
                $q->where('product_id', $validated['service_id'])
            );
        }

        if (!empty($validated['limit'])) {
            $query->limit($validated['limit']);
        }

        $members = $query->get();
        $sent = 0;
        $failed = 0;

        foreach ($members as $member) {
            try {
                $incentive = $validated['incentive'] ? (object) ['description' => $validated['incentive']] : null;

                if ($this->membershipService->sendFeedbackSurvey($member, $incentive)) {
                    $sent++;
                } else {
                    $failed++;
                }
            } catch (\Exception $e) {
                $failed++;
            }
        }

        return response()->json([
            'success' => true,
            'message' => "Sent {$sent} feedback surveys",
            'sent' => $sent,
            'failed' => $failed,
        ]);
    }

    /**
     * Get email campaign history
     */
    public function campaignHistory(Request $request): JsonResponse
    {
        $campaigns = DB::table('email_logs')
            ->select([
                'campaign_id',
                'campaign_type',
                'template',
                DB::raw('COUNT(*) as total_sent'),
                DB::raw('MIN(sent_at) as sent_at'),
                DB::raw('COUNT(DISTINCT offer_code) as offers_used'),
            ])
            ->whereNotNull('campaign_id')
            ->groupBy('campaign_id', 'campaign_type', 'template')
            ->orderByDesc('sent_at')
            ->limit(50)
            ->get();

        return response()->json([
            'success' => true,
            'campaigns' => $campaigns,
        ]);
    }

    /**
     * Get churn reasons breakdown
     */
    public function churnReasons(Request $request): JsonResponse
    {
        $period = $request->input('period', '90d');
        $config = self::TIME_PERIODS[$period] ?? self::TIME_PERIODS['90d'];

        $query = UserSubscription::whereIn('status', ['cancelled', 'expired']);

        if ($config['days']) {
            $query->where('cancelled_at', '>=', now()->subDays($config['days']));
        }

        $reasons = $query
            ->whereNotNull('cancellation_reason')
            ->select('cancellation_reason', DB::raw('COUNT(*) as count'))
            ->groupBy('cancellation_reason')
            ->orderByDesc('count')
            ->limit(10)
            ->get();

        return response()->json([
            'success' => true,
            'period' => $period,
            'reasons' => $reasons,
        ]);
    }

    /**
     * Build optimized past members query
     */
    private function buildPastMembersQuery(?int $days): \Illuminate\Database\Eloquent\Builder
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
            ->selectRaw('MAX(user_subscriptions.cancellation_reason) as churn_reason')
            ->leftJoin('user_subscriptions', 'users.id', '=', 'user_subscriptions.user_id')
            ->whereExists(function ($q) {
                $q->select(DB::raw(1))
                  ->from('user_subscriptions as us')
                  ->whereColumn('us.user_id', 'users.id')
                  ->whereIn('us.status', ['cancelled', 'expired']);
            })
            ->whereNotExists(function ($q) {
                $q->select(DB::raw(1))
                  ->from('user_subscriptions as us')
                  ->whereColumn('us.user_id', 'users.id')
                  ->where('us.status', 'active');
            })
            ->groupBy('users.id', 'users.name', 'users.first_name', 'users.last_name', 'users.email', 'users.avatar', 'users.created_at');

        if ($days) {
            $cutoffDate = now()->subDays($days);
            $query->whereExists(function ($q) use ($cutoffDate) {
                $q->select(DB::raw(1))
                  ->from('user_subscriptions as us')
                  ->whereColumn('us.user_id', 'users.id')
                  ->whereIn('us.status', ['cancelled', 'expired'])
                  ->where('us.cancelled_at', '>=', $cutoffDate);
            });
        }

        return $query;
    }

    /**
     * Get statistics for a time period
     */
    private function getTimePeriodStats(?int $days): array
    {
        $cacheKey = 'past_members_stats_' . ($days ?? 'all');

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($days) {
            $query = UserSubscription::whereIn('status', ['cancelled', 'expired']);

            if ($days) {
                $query->where('cancelled_at', '>=', now()->subDays($days));
            }

            $totalChurned = $query->distinct('user_id')->count('user_id');
            $potentialRevenue = $query->avg('price') * $totalChurned;

            // Previous period for comparison
            $previousQuery = UserSubscription::whereIn('status', ['cancelled', 'expired']);
            if ($days) {
                $previousQuery->whereBetween('cancelled_at', [
                    now()->subDays($days * 2),
                    now()->subDays($days),
                ]);
            }
            $previousChurned = $previousQuery->distinct('user_id')->count('user_id');

            $changePercent = $previousChurned > 0
                ? round((($totalChurned - $previousChurned) / $previousChurned) * 100, 1)
                : 0;

            return [
                'total' => $totalChurned,
                'potential_recovery_revenue' => round($potentialRevenue, 2),
                'previous_period' => $previousChurned,
                'change_percent' => $changePercent,
                'trend' => $changePercent > 0 ? 'up' : ($changePercent < 0 ? 'down' : 'stable'),
            ];
        });
    }

    /**
     * Transform past member for API response
     */
    private function transformPastMember($member): array
    {
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
            'days_since_churn' => $member->churned_at
                ? Carbon::parse($member->churned_at)->diffInDays(now())
                : null,
            'joined_at' => $member->created_at,
            'win_back_potential' => $this->calculateWinBackPotential($member),
        ];
    }

    /**
     * Calculate win-back potential score (0-100)
     */
    private function calculateWinBackPotential($member): array
    {
        $score = 0;
        $factors = [];

        // Total spent (0-40 points)
        $spent = (float) $member->total_spent;
        if ($spent >= 500) {
            $score += 40;
            $factors[] = 'High lifetime value';
        } elseif ($spent >= 100) {
            $score += 25;
            $factors[] = 'Medium lifetime value';
        } elseif ($spent > 0) {
            $score += 10;
        }

        // Recency (0-30 points)
        $daysSinceChurn = $member->churned_at
            ? Carbon::parse($member->churned_at)->diffInDays(now())
            : 365;

        if ($daysSinceChurn <= 30) {
            $score += 30;
            $factors[] = 'Recently churned';
        } elseif ($daysSinceChurn <= 90) {
            $score += 20;
            $factors[] = 'Churned within 90 days';
        } elseif ($daysSinceChurn <= 180) {
            $score += 10;
        }

        // Tenure (0-30 points)
        $tenure = Carbon::parse($member->created_at)->diffInDays(now());
        if ($tenure >= 365) {
            $score += 30;
            $factors[] = 'Long-term customer';
        } elseif ($tenure >= 180) {
            $score += 20;
            $factors[] = 'Established customer';
        } elseif ($tenure >= 30) {
            $score += 10;
        }

        $level = match(true) {
            $score >= 70 => 'high',
            $score >= 40 => 'medium',
            default => 'low',
        };

        return [
            'score' => $score,
            'level' => $level,
            'factors' => $factors,
        ];
    }

    /**
     * Get win-back email template
     */
    private function getWinBackTemplate(
        string $template,
        $member,
        ?string $offerCode,
        ?int $discountPercent,
        ?string $customSubject,
        ?string $customBody
    ): array {
        $name = $member->first_name ?? $member->name ?? 'there';
        $code = $offerCode ?? 'COMEBACK' . rand(100, 999);
        $discount = $discountPercent ?? 30;

        $templates = [
            '30_free' => [
                'subject' => "We Miss You, {$name}! Come Back for 30 Days FREE",
                'body' => $this->render30FreeTemplate($name, $code),
            ],
            'discount' => [
                'subject' => "Exclusive {$discount}% Off - Just for You, {$name}!",
                'body' => $this->renderDiscountTemplate($name, $code, $discount),
            ],
            'missed' => [
                'subject' => "We've Missed You at Revolution Trading Pros",
                'body' => $this->renderMissedTemplate($name),
            ],
            'custom' => [
                'subject' => str_replace(['{{name}}', '{{code}}'], [$name, $code], $customSubject ?? ''),
                'body' => str_replace(['{{name}}', '{{code}}', '{{discount}}'], [$name, $code, $discount], $customBody ?? ''),
            ],
        ];

        return $templates[$template] ?? $templates['missed'];
    }

    private function render30FreeTemplate(string $name, string $code): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0;">We Miss You, {$name}!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">It's been a while since we've seen you at Revolution Trading Pros, and we want you back!</p>
            <div style="background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); border-radius: 16px; padding: 32px; margin: 32px 0; text-align: center;">
                <p style="color: white; font-size: 24px; font-weight: 600; margin: 0 0 8px 0;">30 Days FREE</p>
                <p style="color: rgba(255,255,255,0.9); font-size: 15px; margin: 0;">Use code: <strong>{$code}</strong></p>
            </div>
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Claim Your Free Month</a>
            </div>
            <p style="color: #86868b; font-size: 14px; text-align: center;">No strings attached. Your satisfaction is our priority.</p>
        </div>
        HTML;
    }

    private function renderDiscountTemplate(string $name, string $code, int $discount): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0;">Exclusive Offer for You!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Hi {$name},</p>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">We noticed you're no longer with us, and we'd love to have you back. Here's a special offer just for you:</p>
            <div style="background: #f5f5f7; border-radius: 16px; padding: 32px; margin: 32px 0; text-align: center;">
                <p style="color: #6366f1; font-size: 48px; font-weight: 700; margin: 0;">{$discount}% OFF</p>
                <p style="color: #424245; font-size: 15px; margin: 8px 0 0 0;">Your next month with code: <strong>{$code}</strong></p>
            </div>
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: #1d1d1f; color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Reactivate Now</a>
            </div>
        </div>
        HTML;
    }

    private function renderMissedTemplate(string $name): string
    {
        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="color: #1d1d1f; font-size: 32px; font-weight: 600; margin: 0;">We've Missed You!</h1>
            </div>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Hi {$name},</p>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">The trading world keeps moving, and so do we. Since you've been away, we've added:</p>
            <ul style="color: #424245; font-size: 17px; line-height: 1.8; padding-left: 24px;">
                <li>New trading signals and alerts</li>
                <li>Enhanced live trading rooms</li>
                <li>Exclusive member-only content</li>
                <li>Improved educational resources</li>
            </ul>
            <p style="color: #424245; font-size: 17px; line-height: 1.6;">Come back and see what you've been missing!</p>
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Explore What's New</a>
            </div>
        </div>
        HTML;
    }
}
