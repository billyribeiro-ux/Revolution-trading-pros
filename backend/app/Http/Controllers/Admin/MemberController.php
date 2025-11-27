<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\User;
use App\Models\UserSubscription;
use App\Models\Product;
use App\Models\SubscriptionPlan;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Mail;
use Carbon\Carbon;

class MemberController extends Controller
{
    /**
     * Allowed columns for sorting (SQL injection prevention)
     */
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'name', 'email', 'first_name', 'last_name', 'id'
    ];
    /**
     * Display comprehensive list of all members with advanced filtering
     */
    public function index(Request $request)
    {
        $query = User::query()
            ->with(['subscriptions.subscriptionPlan', 'subscriptions.product', 'orders'])
            ->withCount(['subscriptions as active_subscriptions_count' => function ($q) {
                $q->where('status', 'active');
            }])
            ->withCount(['orders as total_orders'])
            ->withSum('orders as total_spent', 'total');

        // Search filter
        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%")
                  ->orWhere('first_name', 'like', "%{$search}%")
                  ->orWhere('last_name', 'like', "%{$search}%");
            });
        }

        // Status filter
        if ($request->filled('status')) {
            switch ($request->status) {
                case 'active':
                    $query->whereHas('subscriptions', fn($q) => $q->where('status', 'active'));
                    break;
                case 'trial':
                    $query->whereHas('subscriptions', fn($q) => $q->where('status', 'trial'));
                    break;
                case 'churned':
                    $query->whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
                          ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'));
                    break;
                case 'never_subscribed':
                    $query->whereDoesntHave('subscriptions');
                    break;
            }
        }

        // Date range filter
        if ($request->filled('date_from')) {
            $query->where('created_at', '>=', Carbon::parse($request->date_from)->startOfDay());
        }
        if ($request->filled('date_to')) {
            $query->where('created_at', '<=', Carbon::parse($request->date_to)->endOfDay());
        }

        // Product/Service filter
        if ($request->filled('product_id')) {
            $query->whereHas('subscriptions', fn($q) => $q->where('product_id', $request->product_id));
        }

        // Subscription plan filter
        if ($request->filled('plan_id')) {
            $query->whereHas('subscriptions', fn($q) => $q->where('subscription_plan_id', $request->plan_id));
        }

        // Spending tier filter
        if ($request->filled('spending_tier')) {
            switch ($request->spending_tier) {
                case 'whale': // $5000+
                    $query->having('total_spent', '>=', 5000);
                    break;
                case 'high': // $1000-4999
                    $query->having('total_spent', '>=', 1000)->having('total_spent', '<', 5000);
                    break;
                case 'medium': // $100-999
                    $query->having('total_spent', '>=', 100)->having('total_spent', '<', 1000);
                    break;
                case 'low': // <$100
                    $query->having('total_spent', '<', 100);
                    break;
            }
        }

        // Sorting with whitelist validation (SQL injection prevention)
        $sortBy = $request->input('sort_by', 'created_at');
        $sortDir = $request->input('sort_dir', 'desc');

        if (!in_array(strtolower($sortDir), ['asc', 'desc'], true)) {
            $sortDir = 'desc';
        }

        if ($sortBy === 'total_spent') {
            $query->orderBy(DB::raw('COALESCE(total_spent, 0)'), $sortDir);
        } elseif (in_array($sortBy, self::ALLOWED_SORT_COLUMNS, true)) {
            $query->orderBy($sortBy, $sortDir);
        } else {
            $query->orderBy('created_at', $sortDir);
        }

        $members = $query->paginate($request->input('per_page', 25));

        // Transform data for frontend
        $members->getCollection()->transform(function ($member) {
            return $this->transformMember($member);
        });

        return response()->json([
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
     * Get comprehensive member statistics
     */
    public function stats(Request $request)
    {
        $now = Carbon::now();
        $startOfMonth = $now->copy()->startOfMonth();
        $startOfLastMonth = $now->copy()->subMonth()->startOfMonth();
        $endOfLastMonth = $now->copy()->subMonth()->endOfMonth();

        // Total members
        $totalMembers = User::count();
        $newThisMonth = User::where('created_at', '>=', $startOfMonth)->count();
        $newLastMonth = User::whereBetween('created_at', [$startOfLastMonth, $endOfLastMonth])->count();

        // Active subscribers
        $activeSubscribers = User::whereHas('subscriptions', fn($q) => $q->where('status', 'active'))->count();
        $activeLastMonth = User::whereHas('subscriptions', function ($q) use ($endOfLastMonth) {
            $q->where('status', 'active')->where('created_at', '<=', $endOfLastMonth);
        })->count();

        // Trial members
        $trialMembers = User::whereHas('subscriptions', fn($q) => $q->where('status', 'trial'))->count();

        // Churned members (had subscription, now cancelled/expired, no active subs)
        $churnedMembers = User::whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
            ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'))
            ->count();

        // Churn rate calculation
        $churnedThisMonth = UserSubscription::where('status', 'cancelled')
            ->where('cancelled_at', '>=', $startOfMonth)
            ->count();
        $churnRate = $activeLastMonth > 0 ? round(($churnedThisMonth / $activeLastMonth) * 100, 2) : 0;

        // Revenue metrics
        $mrr = UserSubscription::where('status', 'active')
            ->sum(DB::raw("CASE
                WHEN interval = 'yearly' THEN price / 12
                WHEN interval = 'quarterly' THEN price / 3
                ELSE price
            END"));

        $totalRevenue = UserSubscription::sum('total_paid');

        // Average LTV
        $avgLtv = User::whereHas('subscriptions')
            ->withSum('subscriptions as lifetime_value', 'total_paid')
            ->avg('lifetime_value') ?? 0;

        // Top services by members
        $topServices = Product::withCount(['users' => function ($q) {
            $q->whereHas('subscriptions', fn($sq) => $sq->where('status', 'active'));
        }])
        ->orderByDesc('users_count')
        ->take(5)
        ->get(['id', 'name', 'type'])
        ->map(fn($p) => [
            'id' => $p->id,
            'name' => $p->name,
            'type' => $p->type,
            'members_count' => $p->users_count,
        ]);

        // Member growth trend (last 12 months)
        $growthTrend = collect(range(11, 0))->map(function ($monthsAgo) {
            $date = Carbon::now()->subMonths($monthsAgo);
            return [
                'month' => $date->format('M Y'),
                'members' => User::where('created_at', '<=', $date->endOfMonth())->count(),
                'new' => User::whereBetween('created_at', [
                    $date->copy()->startOfMonth(),
                    $date->copy()->endOfMonth()
                ])->count(),
            ];
        });

        return response()->json([
            'overview' => [
                'total_members' => $totalMembers,
                'new_this_month' => $newThisMonth,
                'new_last_month' => $newLastMonth,
                'growth_rate' => $newLastMonth > 0
                    ? round((($newThisMonth - $newLastMonth) / $newLastMonth) * 100, 1)
                    : 0,
            ],
            'subscriptions' => [
                'active' => $activeSubscribers,
                'trial' => $trialMembers,
                'churned' => $churnedMembers,
                'churn_rate' => $churnRate,
            ],
            'revenue' => [
                'mrr' => round($mrr, 2),
                'total' => round($totalRevenue, 2),
                'avg_ltv' => round($avgLtv, 2),
            ],
            'top_services' => $topServices,
            'growth_trend' => $growthTrend,
        ]);
    }

    /**
     * Get members by service/product
     */
    public function byService(Request $request, $productId)
    {
        $product = Product::findOrFail($productId);

        $query = User::query()
            ->whereHas('subscriptions', fn($q) => $q->where('product_id', $productId))
            ->with(['subscriptions' => fn($q) => $q->where('product_id', $productId)->with('subscriptionPlan')])
            ->withSum(['subscriptions as service_revenue' => fn($q) => $q->where('product_id', $productId)], 'total_paid');

        // Status filter
        if ($request->filled('status')) {
            $query->whereHas('subscriptions', function ($q) use ($request, $productId) {
                $q->where('product_id', $productId)->where('status', $request->status);
            });
        }

        // Search
        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%");
            });
        }

        $members = $query->orderBy('created_at', 'desc')
            ->paginate($request->input('per_page', 25));

        // Service stats
        $stats = [
            'total_members' => User::whereHas('subscriptions', fn($q) => $q->where('product_id', $productId))->count(),
            'active_members' => User::whereHas('subscriptions', fn($q) => $q->where('product_id', $productId)->where('status', 'active'))->count(),
            'trial_members' => User::whereHas('subscriptions', fn($q) => $q->where('product_id', $productId)->where('status', 'trial'))->count(),
            'churned_members' => User::whereHas('subscriptions', fn($q) => $q->where('product_id', $productId)->whereIn('status', ['cancelled', 'expired']))
                ->whereDoesntHave('subscriptions', fn($q) => $q->where('product_id', $productId)->where('status', 'active'))
                ->count(),
            'total_revenue' => UserSubscription::where('product_id', $productId)->sum('total_paid'),
        ];

        return response()->json([
            'service' => [
                'id' => $product->id,
                'name' => $product->name,
                'type' => $product->type,
            ],
            'stats' => $stats,
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
     * Get all services/products for filtering
     */
    public function services()
    {
        $services = Product::withCount(['users as members_count'])
            ->orderBy('name')
            ->get(['id', 'name', 'type', 'price', 'is_active'])
            ->map(fn($s) => [
                'id' => $s->id,
                'name' => $s->name,
                'type' => $s->type,
                'price' => $s->price,
                'is_active' => $s->is_active,
                'members_count' => $s->members_count,
            ]);

        return response()->json(['services' => $services]);
    }

    /**
     * Get churned/past members for win-back campaigns
     */
    public function churned(Request $request)
    {
        $query = User::query()
            ->whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
            ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'))
            ->with(['subscriptions' => function ($q) {
                $q->whereIn('status', ['cancelled', 'expired'])
                  ->orderByDesc('cancelled_at')
                  ->with('subscriptionPlan', 'product');
            }])
            ->withSum('subscriptions as total_spent', 'total_paid');

        // Churn reason filter
        if ($request->filled('churn_reason')) {
            $query->whereHas('subscriptions', fn($q) =>
                $q->where('metadata->cancel_reason', 'like', "%{$request->churn_reason}%")
            );
        }

        // Days since churned filter
        if ($request->filled('churned_within_days')) {
            $daysAgo = Carbon::now()->subDays($request->churned_within_days);
            $query->whereHas('subscriptions', fn($q) =>
                $q->whereIn('status', ['cancelled', 'expired'])
                  ->where('cancelled_at', '>=', $daysAgo)
            );
        }

        // Search
        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%");
            });
        }

        // Win-back potential scoring (high value churned members)
        if ($request->filled('winback_potential')) {
            switch ($request->winback_potential) {
                case 'high':
                    $query->having('total_spent', '>=', 500);
                    break;
                case 'medium':
                    $query->having('total_spent', '>=', 100)->having('total_spent', '<', 500);
                    break;
                case 'low':
                    $query->having('total_spent', '<', 100);
                    break;
            }
        }

        $members = $query->orderByDesc('total_spent')
            ->paginate($request->input('per_page', 25));

        // Transform with churn details
        $members->getCollection()->transform(function ($member) {
            $lastSubscription = $member->subscriptions->first();
            return array_merge($this->transformMember($member), [
                'churned_at' => $lastSubscription?->cancelled_at ?? $lastSubscription?->expires_at,
                'last_product' => $lastSubscription?->product?->name ?? 'Unknown',
                'last_plan' => $lastSubscription?->subscriptionPlan?->name ?? 'Unknown',
                'churn_reason' => $lastSubscription?->metadata['cancel_reason'] ?? null,
                'days_since_churn' => $lastSubscription?->cancelled_at
                    ? Carbon::parse($lastSubscription->cancelled_at)->diffInDays(Carbon::now())
                    : null,
            ]);
        });

        // Churned stats
        $stats = [
            'total_churned' => User::whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
                ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'))
                ->count(),
            'churned_this_month' => UserSubscription::where('status', 'cancelled')
                ->where('cancelled_at', '>=', Carbon::now()->startOfMonth())
                ->distinct('user_id')
                ->count('user_id'),
            'potential_recovery_revenue' => UserSubscription::whereIn('status', ['cancelled', 'expired'])
                ->avg('price') * User::whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
                    ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'))
                    ->count(),
            'top_churn_reasons' => DB::table('user_subscriptions')
                ->whereIn('status', ['cancelled', 'expired'])
                ->whereNotNull('metadata')
                ->selectRaw("JSON_EXTRACT(metadata, '$.cancel_reason') as reason, COUNT(*) as count")
                ->groupBy('reason')
                ->orderByDesc('count')
                ->take(5)
                ->get(),
        ];

        return response()->json([
            'stats' => $stats,
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
     * Get single member details
     */
    public function show($id)
    {
        $member = User::with([
            'subscriptions.subscriptionPlan',
            'subscriptions.product',
            'orders.items',
        ])
        ->withSum('subscriptions as total_spent', 'total_paid')
        ->withCount('subscriptions')
        ->findOrFail($id);

        // Calculate engagement score
        $engagementScore = $this->calculateEngagementScore($member);

        // Get activity timeline
        $timeline = $this->getMemberTimeline($member);

        return response()->json([
            'member' => $this->transformMember($member, true),
            'engagement_score' => $engagementScore,
            'timeline' => $timeline,
        ]);
    }

    /**
     * Send email to single member (queued for async delivery)
     */
    public function sendEmail(Request $request, $id)
    {
        $validated = $request->validate([
            'template_id' => 'nullable|exists:email_templates,id',
            'subject' => 'required|string|max:255',
            'body' => 'required|string',
            'campaign_type' => 'nullable|string|in:winback,promo,general,reminder',
        ]);

        $member = User::findOrFail($id);

        // Queue email for async delivery (non-blocking)
        try {
            Mail::to($member->email, $member->name)
                ->queue(new \App\Mail\GenericEmail(
                    $validated['subject'],
                    $validated['body']
                ));

            // Log email queued
            DB::table('email_logs')->insert([
                'user_id' => $member->id,
                'template_id' => $validated['template_id'] ?? null,
                'subject' => $validated['subject'],
                'campaign_type' => $validated['campaign_type'] ?? 'general',
                'sent_at' => now(),
                'created_at' => now(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Email queued for delivery',
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to queue email: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Send bulk email to multiple members (batch queued for async delivery)
     */
    public function bulkEmail(Request $request)
    {
        $validated = $request->validate([
            'member_ids' => 'required|array|max:500', // Limit bulk operations
            'member_ids.*' => 'exists:users,id',
            'template_id' => 'nullable|exists:email_templates,id',
            'subject' => 'required|string|max:255',
            'body' => 'required|string',
            'campaign_type' => 'nullable|string|in:winback,promo,general,reminder,free_trial',
            'personalize' => 'boolean',
        ]);

        $members = User::whereIn('id', $validated['member_ids'])->get();
        $queued = 0;

        // Prepare batch email logs for single insert
        $emailLogs = [];
        $now = now();

        foreach ($members as $member) {
            // Personalize body if requested
            $body = $validated['body'];
            if ($validated['personalize'] ?? false) {
                $body = str_replace(
                    ['{{name}}', '{{first_name}}', '{{email}}'],
                    [$member->name, $member->first_name ?? $member->name, $member->email],
                    $body
                );
            }

            // Queue email asynchronously (non-blocking)
            Mail::to($member->email, $member->name)
                ->later(now()->addSeconds($queued), new \App\Mail\GenericEmail(
                    $validated['subject'],
                    $body
                ));

            $emailLogs[] = [
                'user_id' => $member->id,
                'template_id' => $validated['template_id'] ?? null,
                'subject' => $validated['subject'],
                'campaign_type' => $validated['campaign_type'] ?? 'general',
                'sent_at' => $now,
                'created_at' => $now,
            ];

            $queued++;
        }

        // Batch insert email logs (single query instead of N queries)
        if (!empty($emailLogs)) {
            DB::table('email_logs')->insert($emailLogs);
        }

        return response()->json([
            'success' => true,
            'message' => "Queued {$queued} emails for delivery",
            'queued' => $queued,
        ]);
    }

    /**
     * Get email templates for campaigns
     */
    public function emailTemplates()
    {
        $templates = DB::table('email_templates')
            ->select('id', 'name', 'subject', 'category', 'is_active')
            ->orderBy('name')
            ->get();

        // Add pre-built win-back templates
        $winbackTemplates = [
            [
                'id' => 'winback_30_free',
                'name' => 'Win-Back: 30 Days Free',
                'subject' => 'We Miss You! Come Back for 30 Days FREE',
                'category' => 'winback',
                'is_active' => true,
                'is_preset' => true,
                'body' => $this->getWinbackTemplate('30_free'),
            ],
            [
                'id' => 'winback_discount',
                'name' => 'Win-Back: 50% Discount',
                'subject' => 'Exclusive Offer: 50% Off Your Next Month',
                'category' => 'winback',
                'is_active' => true,
                'is_preset' => true,
                'body' => $this->getWinbackTemplate('discount'),
            ],
            [
                'id' => 'winback_missed',
                'name' => 'Win-Back: We Miss You',
                'subject' => 'We\'ve Missed You at Revolution Trading Pros',
                'category' => 'winback',
                'is_active' => true,
                'is_preset' => true,
                'body' => $this->getWinbackTemplate('missed'),
            ],
        ];

        return response()->json([
            'templates' => $templates,
            'preset_templates' => $winbackTemplates,
        ]);
    }

    /**
     * Export members to CSV
     */
    public function export(Request $request)
    {
        $query = User::query()
            ->with('subscriptions.subscriptionPlan')
            ->withSum('subscriptions as total_spent', 'total_paid');

        // Apply same filters as index
        if ($request->filled('status')) {
            switch ($request->status) {
                case 'active':
                    $query->whereHas('subscriptions', fn($q) => $q->where('status', 'active'));
                    break;
                case 'churned':
                    $query->whereHas('subscriptions', fn($q) => $q->whereIn('status', ['cancelled', 'expired']))
                          ->whereDoesntHave('subscriptions', fn($q) => $q->where('status', 'active'));
                    break;
            }
        }

        $members = $query->get();

        $csv = "Name,Email,Status,Subscriptions,Total Spent,Joined\n";
        foreach ($members as $member) {
            $status = $member->subscriptions->where('status', 'active')->count() > 0 ? 'Active' : 'Inactive';
            $subscriptions = $member->subscriptions->pluck('subscriptionPlan.name')->filter()->implode('; ');
            $csv .= "\"{$member->name}\",{$member->email},{$status},\"{$subscriptions}\",{$member->total_spent},{$member->created_at}\n";
        }

        return response($csv)
            ->header('Content-Type', 'text/csv')
            ->header('Content-Disposition', 'attachment; filename=members_export_' . date('Y-m-d') . '.csv');
    }

    /**
     * Transform member model to API response
     */
    private function transformMember($member, $detailed = false)
    {
        $activeSubscription = $member->subscriptions?->firstWhere('status', 'active');

        $data = [
            'id' => $member->id,
            'name' => $member->name,
            'first_name' => $member->first_name,
            'last_name' => $member->last_name,
            'email' => $member->email,
            'avatar' => $member->avatar ?? null,
            'status' => $this->getMemberStatus($member),
            'status_label' => $this->getMemberStatusLabel($member),
            'joined_at' => $member->created_at,
            'total_spent' => round($member->total_spent ?? 0, 2),
            'active_subscriptions_count' => $member->active_subscriptions_count ?? 0,
            'current_plan' => $activeSubscription?->subscriptionPlan?->name ?? null,
            'current_product' => $activeSubscription?->product?->name ?? null,
        ];

        if ($detailed) {
            $data['subscriptions'] = $member->subscriptions->map(fn($sub) => [
                'id' => $sub->id,
                'plan' => $sub->subscriptionPlan?->name,
                'product' => $sub->product?->name,
                'status' => $sub->status,
                'price' => $sub->price,
                'interval' => $sub->interval,
                'start_date' => $sub->start_date,
                'next_payment' => $sub->next_payment_date,
                'cancelled_at' => $sub->cancelled_at,
                'total_paid' => $sub->total_paid,
            ]);
            $data['orders'] = $member->orders?->map(fn($order) => [
                'id' => $order->id,
                'number' => $order->order_number,
                'total' => $order->total,
                'status' => $order->status,
                'created_at' => $order->created_at,
            ]);
            $data['email_verified'] = !is_null($member->email_verified_at);
            $data['last_login'] = $member->last_login_at ?? null;
        }

        return $data;
    }

    /**
     * Get member status code
     */
    private function getMemberStatus($member): string
    {
        $activeCount = $member->subscriptions?->where('status', 'active')->count() ?? 0;
        $trialCount = $member->subscriptions?->where('status', 'trial')->count() ?? 0;
        $hasChurned = $member->subscriptions?->whereIn('status', ['cancelled', 'expired'])->count() > 0;

        if ($activeCount > 0) return 'active';
        if ($trialCount > 0) return 'trial';
        if ($hasChurned) return 'churned';
        return 'never_subscribed';
    }

    /**
     * Get member status display label
     */
    private function getMemberStatusLabel($member): string
    {
        return match($this->getMemberStatus($member)) {
            'active' => 'Active Subscriber',
            'trial' => 'Trial Member',
            'churned' => 'Past Member',
            'never_subscribed' => 'Registered User',
        };
    }

    /**
     * Calculate member engagement score (0-100)
     */
    private function calculateEngagementScore($member): int
    {
        $score = 0;

        // Tenure (up to 20 points)
        $daysSinceJoined = Carbon::parse($member->created_at)->diffInDays(Carbon::now());
        $score += min(20, $daysSinceJoined / 30 * 5);

        // Spending (up to 30 points)
        $totalSpent = $member->total_spent ?? 0;
        $score += min(30, $totalSpent / 100 * 5);

        // Active subscriptions (up to 25 points)
        $activeSubs = $member->subscriptions?->where('status', 'active')->count() ?? 0;
        $score += min(25, $activeSubs * 12.5);

        // Order frequency (up to 15 points)
        $orderCount = $member->subscriptions_count ?? 0;
        $score += min(15, $orderCount * 5);

        // Email verified (10 points)
        if ($member->email_verified_at) {
            $score += 10;
        }

        return (int) min(100, $score);
    }

    /**
     * Get member activity timeline
     */
    private function getMemberTimeline($member): array
    {
        $timeline = [];

        // Registration
        $timeline[] = [
            'type' => 'registration',
            'title' => 'Account Created',
            'date' => $member->created_at,
            'icon' => 'user-plus',
        ];

        // Subscriptions
        foreach ($member->subscriptions as $sub) {
            $timeline[] = [
                'type' => 'subscription_start',
                'title' => "Started {$sub->subscriptionPlan?->name}",
                'date' => $sub->start_date ?? $sub->created_at,
                'icon' => 'credit-card',
                'meta' => ['plan' => $sub->subscriptionPlan?->name, 'price' => $sub->price],
            ];

            if ($sub->cancelled_at) {
                $timeline[] = [
                    'type' => 'subscription_cancel',
                    'title' => "Cancelled {$sub->subscriptionPlan?->name}",
                    'date' => $sub->cancelled_at,
                    'icon' => 'x-circle',
                ];
            }
        }

        // Sort by date descending
        usort($timeline, fn($a, $b) => strtotime($b['date']) - strtotime($a['date']));

        return array_slice($timeline, 0, 20);
    }

    /**
     * Get win-back email template content
     */
    private function getWinbackTemplate($type): string
    {
        return match($type) {
            '30_free' => '
                <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                    <h1 style="color: #6366f1;">We Miss You, {{name}}!</h1>
                    <p>It\'s been a while since we\'ve seen you at Revolution Trading Pros, and we want you back!</p>
                    <p><strong>As a special welcome back offer, we\'re giving you 30 days completely FREE.</strong></p>
                    <p>No strings attached. Just use code <strong>COMEBACK30</strong> when you reactivate.</p>
                    <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 12px 24px; border-radius: 8px; text-decoration: none; font-weight: bold;">Claim Your 30 Free Days</a>
                </div>
            ',
            'discount' => '
                <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                    <h1 style="color: #6366f1;">Exclusive 50% Off Offer!</h1>
                    <p>Hi {{name}},</p>
                    <p>We noticed you\'re no longer with us, and we\'d love to have you back.</p>
                    <p><strong>For a limited time, get 50% off your next month!</strong></p>
                    <p>Use code <strong>RETURN50</strong> at checkout.</p>
                    <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 12px 24px; border-radius: 8px; text-decoration: none; font-weight: bold;">Reactivate Now</a>
                </div>
            ',
            'missed' => '
                <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                    <h1 style="color: #6366f1;">We\'ve Missed You!</h1>
                    <p>Hi {{name}},</p>
                    <p>The trading world keeps moving, and so do we. Since you\'ve been away, we\'ve added:</p>
                    <ul>
                        <li>New trading signals and alerts</li>
                        <li>Enhanced live trading rooms</li>
                        <li>Exclusive member-only content</li>
                    </ul>
                    <p>Come back and see what you\'ve been missing!</p>
                    <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 12px 24px; border-radius: 8px; text-decoration: none; font-weight: bold;">Explore What\'s New</a>
                </div>
            ',
            default => '',
        };
    }
}
