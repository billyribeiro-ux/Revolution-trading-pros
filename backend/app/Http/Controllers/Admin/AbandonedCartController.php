<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\AbandonedCart;
use App\Models\Order;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Carbon\Carbon;

/**
 * AbandonedCartController - Manage cart recovery campaigns
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Features:
 * - Dashboard with recovery stats
 * - Manual recovery email sending
 * - Bulk recovery campaigns
 * - Recovery tracking
 *
 * @version 1.0.0
 */
class AbandonedCartController extends Controller
{
    private const CACHE_TTL = 300;

    /**
     * Get abandoned cart dashboard stats
     */
    public function dashboard(): JsonResponse
    {
        $stats = Cache::remember('abandoned_cart_dashboard', self::CACHE_TTL, function () {
            $now = now();

            // Today's stats
            $today = AbandonedCart::whereDate('abandoned_at', $now->toDateString());
            $todayTotal = $today->count();
            $todayValue = $today->sum('total');
            $todayRecovered = $today->recovered()->count();

            // Last 7 days
            $week = AbandonedCart::where('abandoned_at', '>=', $now->subDays(7));
            $weekTotal = $week->count();
            $weekValue = $week->sum('total');
            $weekRecovered = $week->recovered()->count();
            $weekRecoveredValue = $week->recovered()->sum('total');

            // Last 30 days
            $month = AbandonedCart::where('abandoned_at', '>=', $now->subDays(30));
            $monthTotal = $month->count();
            $monthValue = $month->sum('total');
            $monthRecovered = $month->recovered()->count();
            $monthRecoveredValue = $month->recovered()->sum('total');

            // Recovery rates
            $weekRecoveryRate = $weekTotal > 0 ? round(($weekRecovered / $weekTotal) * 100, 1) : 0;
            $monthRecoveryRate = $monthTotal > 0 ? round(($monthRecovered / $monthTotal) * 100, 1) : 0;

            // Pending recovery
            $pendingRecovery = AbandonedCart::pendingRecovery()->count();
            $pendingValue = AbandonedCart::pendingRecovery()->sum('total');

            // By recovery stage
            $byStage = AbandonedCart::pendingRecovery()
                ->selectRaw('recovery_attempts, COUNT(*) as count, SUM(total) as value')
                ->groupBy('recovery_attempts')
                ->get()
                ->keyBy('recovery_attempts');

            // Hourly breakdown (last 24 hours)
            $hourly = AbandonedCart::where('abandoned_at', '>=', now()->subHours(24))
                ->selectRaw('HOUR(abandoned_at) as hour, COUNT(*) as count, SUM(total) as value')
                ->groupBy('hour')
                ->orderBy('hour')
                ->get();

            return [
                'today' => [
                    'total' => $todayTotal,
                    'value' => round($todayValue, 2),
                    'recovered' => $todayRecovered,
                ],
                'week' => [
                    'total' => $weekTotal,
                    'value' => round($weekValue, 2),
                    'recovered' => $weekRecovered,
                    'recovered_value' => round($weekRecoveredValue, 2),
                    'recovery_rate' => $weekRecoveryRate,
                ],
                'month' => [
                    'total' => $monthTotal,
                    'value' => round($monthValue, 2),
                    'recovered' => $monthRecovered,
                    'recovered_value' => round($monthRecoveredValue, 2),
                    'recovery_rate' => $monthRecoveryRate,
                ],
                'pending' => [
                    'total' => $pendingRecovery,
                    'value' => round($pendingValue, 2),
                    'by_stage' => [
                        'stage_1' => $byStage->get(0)?->count ?? 0,
                        'stage_2' => $byStage->get(1)?->count ?? 0,
                        'stage_3' => $byStage->get(2)?->count ?? 0,
                    ],
                ],
                'hourly' => $hourly,
            ];
        });

        return response()->json([
            'success' => true,
            'data' => $stats,
            'generated_at' => now()->toISOString(),
        ]);
    }

    /**
     * List abandoned carts with filtering
     */
    public function index(Request $request): JsonResponse
    {
        $query = AbandonedCart::query()
            ->with('user:id,name,email,avatar');

        // Status filter
        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        // Period filter
        if ($request->filled('period')) {
            $query->byPeriod($request->period);
        }

        // Value filter
        if ($request->filled('min_value')) {
            $query->where('total', '>=', $request->min_value);
        }

        // Recovery stage filter
        if ($request->filled('recovery_stage')) {
            $query->where('recovery_attempts', $request->recovery_stage);
        }

        // Search
        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('email', 'like', "%{$search}%")
                  ->orWhereHas('user', fn($uq) => $uq->where('name', 'like', "%{$search}%"));
            });
        }

        // Sorting
        $sortBy = $request->input('sort_by', 'abandoned_at');
        $sortDir = $request->input('sort_dir', 'desc');
        $query->orderBy($sortBy, $sortDir);

        $perPage = min($request->input('per_page', 25), 100);
        $carts = $query->paginate($perPage);

        // Transform data
        $carts->getCollection()->transform(fn($cart) => $this->transformCart($cart));

        return response()->json([
            'success' => true,
            'carts' => $carts->items(),
            'pagination' => [
                'total' => $carts->total(),
                'per_page' => $carts->perPage(),
                'current_page' => $carts->currentPage(),
                'last_page' => $carts->lastPage(),
            ],
        ]);
    }

    /**
     * Get single abandoned cart details
     */
    public function show(int $id): JsonResponse
    {
        $cart = AbandonedCart::with('user')->findOrFail($id);

        return response()->json([
            'success' => true,
            'cart' => $this->transformCart($cart, true),
        ]);
    }

    /**
     * Send recovery email to single cart
     */
    public function sendRecoveryEmail(Request $request, int $id): JsonResponse
    {
        $cart = AbandonedCart::findOrFail($id);

        if (!$cart->canReceiveRecoveryEmail()) {
            return response()->json([
                'success' => false,
                'error' => 'Cart cannot receive recovery email',
            ], 400);
        }

        $validated = $request->validate([
            'template' => 'nullable|string|in:reminder,urgency,discount',
            'discount_percent' => 'nullable|integer|min:5|max:50',
        ]);

        try {
            $template = $validated['template'] ?? $this->getTemplateForStage($cart->recovery_attempts);
            $discountPercent = $validated['discount_percent'] ?? ($cart->recovery_attempts >= 2 ? 15 : null);

            // Generate recovery code if discount
            if ($discountPercent) {
                $cart->generateRecoveryCode($discountPercent);
            }

            // Send email
            $emailContent = $this->buildRecoveryEmail($cart, $template, $discountPercent);

            Mail::to($cart->email)->send(new \App\Mail\GenericEmail(
                $emailContent['subject'],
                $emailContent['body']
            ));

            $cart->recordRecoveryAttempt();

            // Log
            DB::table('email_logs')->insert([
                'user_id' => $cart->user_id,
                'campaign_type' => 'abandoned_cart',
                'template' => $template,
                'subject' => $emailContent['subject'],
                'offer_code' => $cart->recovery_code,
                'sent_at' => now(),
                'created_at' => now(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Recovery email sent',
                'recovery_attempt' => $cart->recovery_attempts,
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to send abandoned cart recovery email', [
                'cart_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'error' => 'Failed to send email',
            ], 500);
        }
    }

    /**
     * Send bulk recovery emails
     */
    public function sendBulkRecovery(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'stage' => 'required|integer|in:1,2,3',
            'limit' => 'nullable|integer|min:1|max:500',
            'min_value' => 'nullable|numeric|min:0',
        ]);

        $query = match((int) $validated['stage']) {
            1 => AbandonedCart::readyForFirstEmail(),
            2 => AbandonedCart::readyForSecondEmail(),
            3 => AbandonedCart::readyForThirdEmail(),
        };

        if (!empty($validated['min_value'])) {
            $query->where('total', '>=', $validated['min_value']);
        }

        $limit = $validated['limit'] ?? 100;
        $carts = $query->limit($limit)->get();

        $sent = 0;
        $failed = 0;
        $template = $this->getTemplateForStage($validated['stage'] - 1);
        $discountPercent = $validated['stage'] >= 3 ? 15 : null;

        foreach ($carts as $cart) {
            try {
                if ($discountPercent) {
                    $cart->generateRecoveryCode($discountPercent);
                }

                $emailContent = $this->buildRecoveryEmail($cart, $template, $discountPercent);

                Mail::to($cart->email)->later(
                    now()->addSeconds($sent), // Stagger emails
                    new \App\Mail\GenericEmail($emailContent['subject'], $emailContent['body'])
                );

                $cart->recordRecoveryAttempt();
                $sent++;
            } catch (\Exception $e) {
                $failed++;
            }
        }

        Cache::forget('abandoned_cart_dashboard');

        return response()->json([
            'success' => true,
            'message' => "Sent {$sent} recovery emails, {$failed} failed",
            'sent' => $sent,
            'failed' => $failed,
            'stage' => $validated['stage'],
        ]);
    }

    /**
     * Track recovery link click
     */
    public function trackClick(string $code): JsonResponse
    {
        $cart = AbandonedCart::where('recovery_code', $code)->first();

        if (!$cart) {
            return response()->json([
                'success' => false,
                'error' => 'Invalid recovery code',
            ], 404);
        }

        $cart->markAsClicked();

        return response()->json([
            'success' => true,
            'cart_id' => $cart->id,
            'discount' => $cart->recovery_discount,
        ]);
    }

    /**
     * Mark cart as recovered
     */
    public function markRecovered(Request $request, int $id): JsonResponse
    {
        $cart = AbandonedCart::findOrFail($id);

        $orderId = $request->input('order_id');
        $cart->markAsRecovered($orderId);

        Cache::forget('abandoned_cart_dashboard');

        return response()->json([
            'success' => true,
            'message' => 'Cart marked as recovered',
        ]);
    }

    /**
     * Get recovery email templates
     */
    public function templates(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'templates' => [
                [
                    'id' => 'reminder',
                    'name' => 'Gentle Reminder',
                    'description' => 'Friendly reminder about items left in cart',
                    'recommended_stage' => 1,
                ],
                [
                    'id' => 'urgency',
                    'name' => 'Urgency',
                    'description' => 'Create urgency with stock/time warnings',
                    'recommended_stage' => 2,
                ],
                [
                    'id' => 'discount',
                    'name' => 'Special Offer',
                    'description' => 'Offer discount to complete purchase',
                    'recommended_stage' => 3,
                ],
            ],
        ]);
    }

    /**
     * Report a cart as abandoned (called from frontend)
     */
    public function reportAbandoned(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'session_id' => 'required|string',
            'email' => 'nullable|email',
            'items' => 'required|array|min:1',
            'items.*.id' => 'required',
            'items.*.name' => 'required|string',
            'items.*.price' => 'required|numeric',
            'items.*.quantity' => 'required|integer|min:1',
            'subtotal' => 'required|numeric',
            'total' => 'required|numeric',
            'source' => 'nullable|string',
            'utm_params' => 'nullable|array',
        ]);

        $user = $request->user();
        $email = $validated['email'] ?? $user?->email;

        if (!$email) {
            return response()->json([
                'success' => false,
                'error' => 'Email required for cart recovery',
            ], 400);
        }

        // Check for existing abandoned cart with same session
        $existingCart = AbandonedCart::where('session_id', $validated['session_id'])
            ->where('status', AbandonedCart::STATUS_PENDING)
            ->first();

        if ($existingCart) {
            // Update existing cart
            $existingCart->update([
                'items' => $validated['items'],
                'subtotal' => $validated['subtotal'],
                'total' => $validated['total'],
                'abandoned_at' => now(),
            ]);

            return response()->json([
                'success' => true,
                'cart_id' => $existingCart->id,
                'message' => 'Cart updated',
            ]);
        }

        // Create new abandoned cart
        $cart = AbandonedCart::create([
            'user_id' => $user?->id,
            'session_id' => $validated['session_id'],
            'email' => $email,
            'items' => $validated['items'],
            'subtotal' => $validated['subtotal'],
            'total' => $validated['total'],
            'source' => $validated['source'] ?? 'web',
            'utm_params' => $validated['utm_params'] ?? null,
            'abandoned_at' => now(),
        ]);

        return response()->json([
            'success' => true,
            'cart_id' => $cart->id,
            'message' => 'Cart abandonment recorded',
        ]);
    }

    /**
     * Transform cart for API response
     */
    private function transformCart(AbandonedCart $cart, bool $detailed = false): array
    {
        $data = [
            'id' => $cart->id,
            'email' => $cart->email,
            'user' => $cart->user ? [
                'id' => $cart->user->id,
                'name' => $cart->user->name,
                'avatar' => $cart->user->avatar,
            ] : null,
            'item_count' => $cart->item_count,
            'total' => $cart->total,
            'currency' => $cart->currency,
            'status' => $cart->status,
            'recovery_attempts' => $cart->recovery_attempts,
            'hours_since_abandonment' => $cart->hours_since_abandonment,
            'abandoned_at' => $cart->abandoned_at->toISOString(),
            'can_recover' => $cart->canReceiveRecoveryEmail(),
        ];

        if ($detailed) {
            $data['items'] = $cart->items;
            $data['subtotal'] = $cart->subtotal;
            $data['source'] = $cart->source;
            $data['utm_params'] = $cart->utm_params;
            $data['recovery_code'] = $cart->recovery_code;
            $data['recovery_discount'] = $cart->recovery_discount;
            $data['last_recovery_at'] = $cart->last_recovery_at?->toISOString();
            $data['recovered_at'] = $cart->recovered_at?->toISOString();
            $data['metadata'] = $cart->metadata;
        }

        return $data;
    }

    /**
     * Get template for recovery stage
     */
    private function getTemplateForStage(int $attempts): string
    {
        return match($attempts) {
            0 => 'reminder',
            1 => 'urgency',
            default => 'discount',
        };
    }

    /**
     * Build recovery email content
     */
    private function buildRecoveryEmail(AbandonedCart $cart, string $template, ?int $discount): array
    {
        $name = $cart->user?->first_name ?? $cart->user?->name ?? 'there';
        $itemCount = $cart->item_count;
        $total = number_format($cart->total, 2);

        $templates = [
            'reminder' => [
                'subject' => "Did you forget something? Your cart is waiting!",
                'body' => $this->renderReminderTemplate($name, $itemCount, $total, $cart->items),
            ],
            'urgency' => [
                'subject' => "⏰ Your cart items are selling fast!",
                'body' => $this->renderUrgencyTemplate($name, $itemCount, $total, $cart->items),
            ],
            'discount' => [
                'subject' => "🎁 {$discount}% off to complete your order!",
                'body' => $this->renderDiscountTemplate($name, $itemCount, $total, $cart->items, $discount, $cart->recovery_code),
            ],
        ];

        return $templates[$template] ?? $templates['reminder'];
    }

    private function renderReminderTemplate(string $name, int $itemCount, string $total, array $items): string
    {
        $itemsHtml = $this->renderItemsList($items);

        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <h1 style="color: #1d1d1f; font-size: 28px; font-weight: 600; text-align: center; margin-bottom: 24px;">
                Your cart misses you, {$name}!
            </h1>
            <p style="color: #424245; font-size: 17px; line-height: 1.6; text-align: center;">
                You left {$itemCount} item(s) in your cart worth <strong>\${$total}</strong>
            </p>
            {$itemsHtml}
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: #1d1d1f; color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Complete Your Order</a>
            </div>
        </div>
        HTML;
    }

    private function renderUrgencyTemplate(string $name, int $itemCount, string $total, array $items): string
    {
        $itemsHtml = $this->renderItemsList($items);

        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <div style="background: #fff3cd; border-radius: 12px; padding: 16px; margin-bottom: 24px; text-align: center;">
                <p style="color: #856404; font-size: 15px; margin: 0;">⚡ Items in your cart are in high demand!</p>
            </div>
            <h1 style="color: #1d1d1f; font-size: 28px; font-weight: 600; text-align: center; margin-bottom: 24px;">
                Don't miss out, {$name}!
            </h1>
            <p style="color: #424245; font-size: 17px; line-height: 1.6; text-align: center;">
                Your {$itemCount} item(s) worth <strong>\${$total}</strong> are waiting
            </p>
            {$itemsHtml}
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: linear-gradient(135deg, #f59e0b, #d97706); color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Secure Your Items Now</a>
            </div>
        </div>
        HTML;
    }

    private function renderDiscountTemplate(string $name, int $itemCount, string $total, array $items, int $discount, string $code): string
    {
        $itemsHtml = $this->renderItemsList($items);

        return <<<HTML
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif; max-width: 600px; margin: 0 auto; padding: 40px 20px;">
            <div style="background: linear-gradient(135deg, #6366f1, #8b5cf6); border-radius: 16px; padding: 32px; margin-bottom: 24px; text-align: center;">
                <p style="color: rgba(255,255,255,0.9); font-size: 15px; margin: 0 0 8px 0;">SPECIAL OFFER</p>
                <p style="color: white; font-size: 48px; font-weight: 700; margin: 0;">{$discount}% OFF</p>
                <p style="color: rgba(255,255,255,0.9); font-size: 15px; margin: 8px 0 0 0;">Use code: <strong>{$code}</strong></p>
            </div>
            <h1 style="color: #1d1d1f; font-size: 28px; font-weight: 600; text-align: center; margin-bottom: 24px;">
                Complete your order, {$name}!
            </h1>
            {$itemsHtml}
            <div style="text-align: center; margin: 32px 0;">
                <a href="#" style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6); color: white; padding: 16px 32px; border-radius: 12px; text-decoration: none; font-weight: 600; font-size: 17px;">Claim {$discount}% Off Now</a>
            </div>
            <p style="color: #86868b; font-size: 13px; text-align: center;">Offer expires in 24 hours</p>
        </div>
        HTML;
    }

    private function renderItemsList(array $items): string
    {
        $html = '<div style="background: #f5f5f7; border-radius: 12px; padding: 20px; margin: 24px 0;">';

        foreach (array_slice($items, 0, 3) as $item) {
            $name = htmlspecialchars($item['name'] ?? 'Item');
            $price = number_format($item['price'] ?? 0, 2);
            $qty = $item['quantity'] ?? 1;

            $html .= <<<ITEM
            <div style="display: flex; justify-content: space-between; padding: 12px 0; border-bottom: 1px solid #e5e5e5;">
                <span style="color: #1d1d1f; font-size: 15px;">{$name} × {$qty}</span>
                <span style="color: #424245; font-size: 15px;">\${$price}</span>
            </div>
            ITEM;
        }

        if (count($items) > 3) {
            $remaining = count($items) - 3;
            $html .= "<p style='color: #86868b; font-size: 14px; margin: 12px 0 0 0; text-align: center;'>+ {$remaining} more item(s)</p>";
        }

        $html .= '</div>';

        return $html;
    }
}
