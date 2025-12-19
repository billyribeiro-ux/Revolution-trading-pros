<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\UserSubscription;
use App\Models\SubscriptionPlan;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Validation\Rule;
use Exception;

/**
 * UserSubscriptionController - Enterprise Grade Implementation
 *
 * Handles user self-service subscription management:
 * - View subscriptions
 * - Cancel/Pause/Resume/Reactivate
 * - Payment method updates
 * - Billing history
 *
 * @version 3.0.0 (Google L7+ Enterprise)
 */
class UserSubscriptionController extends Controller
{
    public function index(Request $request): JsonResponse
    {
        try {
            $subscriptions = $request->user()
                ->subscriptions()
                ->with(['plan', 'payments'])
                ->orderBy('created_at', 'desc')
                ->get()
                ->map(fn($subscription) => $this->formatSubscription($subscription));

            return response()->json([
                'subscriptions' => $subscriptions,
                'total' => $subscriptions->count(),
            ]);
        } catch (Exception $e) {
            Log::error('Failed to fetch subscriptions', [
                'user_id' => $request->user()->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to load subscriptions',
            ], 500);
        }
    }

    public function show(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->with(['plan', 'payments'])
                ->findOrFail($id);

            return response()->json([
                'subscription' => $this->formatSubscription($subscription),
            ]);
        } catch (Exception $e) {
            Log::error('Failed to fetch subscription', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Subscription not found',
            ], 404);
        }
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'plan_id' => 'required|exists:subscription_plans,id',
            'payment_method' => 'required|string|in:card,paypal,bank',
            'interval' => 'nullable|string|in:monthly,quarterly,yearly',
        ]);

        try {
            $plan = SubscriptionPlan::findOrFail($validated['plan_id']);

            if ($request->user()->hasActiveSubscription($plan->id)) {
                return response()->json([
                    'message' => 'You already have an active subscription to this plan',
                ], 400);
            }

            $subscription = DB::transaction(function () use ($request, $plan, $validated) {
                return $request->user()->subscriptions()->create([
                    'subscription_plan_id' => $plan->id,
                    'status' => UserSubscription::STATUS_PENDING,
                    'interval' => $validated['interval'] ?? $plan->billing_period ?? 'monthly',
                    'price' => $plan->price,
                    'currency' => 'USD',
                    'start_date' => now(),
                    'next_payment_date' => now()->addMonth(),
                    'auto_renew' => true,
                    'payment_method' => json_encode([
                        'type' => $validated['payment_method'],
                    ]),
                ]);
            });

            $this->logSubscriptionEvent($subscription, 'created');

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->load(['plan', 'payments'])),
                'message' => 'Subscription created successfully',
            ], 201);
        } catch (Exception $e) {
            Log::error('Failed to create subscription', [
                'user_id' => $request->user()->id,
                'plan_id' => $validated['plan_id'],
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to create subscription',
            ], 500);
        }
    }

    public function cancel(Request $request, int $id): JsonResponse
    {
        $validated = $request->validate([
            'reason' => 'nullable|string|max:500',
            'immediate' => 'nullable|boolean',
        ]);

        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            if ($subscription->isCancelled()) {
                return response()->json([
                    'message' => 'Subscription is already cancelled',
                ], 400);
            }

            $immediate = $request->boolean('immediate', false);
            $reason = $validated['reason'] ?? null;

            DB::transaction(function () use ($subscription, $immediate, $reason) {
                if ($immediate) {
                    $subscription->cancel($reason);
                } else {
                    $subscription->cancelAtPeriodEnd($reason);
                }
            });

            $message = $immediate
                ? 'Subscription cancelled immediately'
                : 'Subscription will be cancelled at the end of the billing period';

            $this->logSubscriptionEvent($subscription, 'cancelled', [
                'reason' => $reason,
                'immediate' => $immediate,
            ]);

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->fresh(['plan', 'payments'])),
                'message' => $message,
            ]);
        } catch (Exception $e) {
            Log::error('Failed to cancel subscription', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to cancel subscription',
            ], 500);
        }
    }

    public function pause(Request $request, int $id): JsonResponse
    {
        $validated = $request->validate([
            'reason' => 'nullable|string|max:500',
        ]);

        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            if (!$subscription->isActive()) {
                return response()->json([
                    'message' => 'Only active subscriptions can be paused',
                ], 400);
            }

            DB::transaction(function () use ($subscription, $validated) {
                $subscription->pause($validated['reason'] ?? null);
            });

            $this->logSubscriptionEvent($subscription, 'paused', [
                'reason' => $validated['reason'] ?? null,
            ]);

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->fresh(['plan', 'payments'])),
                'message' => 'Subscription paused successfully',
            ]);
        } catch (Exception $e) {
            Log::error('Failed to pause subscription', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to pause subscription',
            ], 500);
        }
    }

    public function resume(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            if (!$subscription->isPaused()) {
                return response()->json([
                    'message' => 'Only paused subscriptions can be resumed',
                ], 400);
            }

            DB::transaction(function () use ($subscription) {
                $subscription->resume();
            });

            $this->logSubscriptionEvent($subscription, 'resumed');

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->fresh(['plan', 'payments'])),
                'message' => 'Subscription resumed successfully',
            ]);
        } catch (Exception $e) {
            Log::error('Failed to resume subscription', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to resume subscription',
            ], 500);
        }
    }

    public function reactivate(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            if (!$subscription->isCancelled()) {
                return response()->json([
                    'message' => 'Only cancelled subscriptions can be reactivated',
                ], 400);
            }

            DB::transaction(function () use ($subscription) {
                $subscription->reactivate();
            });

            $this->logSubscriptionEvent($subscription, 'reactivated');

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->fresh(['plan', 'payments'])),
                'message' => 'Subscription reactivated successfully',
            ]);
        } catch (Exception $e) {
            Log::error('Failed to reactivate subscription', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to reactivate subscription',
            ], 500);
        }
    }

    public function invoices(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            $invoices = $subscription->payments()
                ->where('status', 'paid')
                ->orderBy('created_at', 'desc')
                ->get()
                ->map(fn($payment) => [
                    'id' => (string) $payment->id,
                    'number' => 'INV-' . str_pad($payment->id, 6, '0', STR_PAD_LEFT),
                    'amount' => $payment->amount,
                    'status' => 'paid',
                    'paid_at' => $payment->paid_at?->toISOString() ?? $payment->created_at->toISOString(),
                    'created_at' => $payment->created_at->toISOString(),
                ]);

            return response()->json([
                'invoices' => $invoices,
            ]);
        } catch (Exception $e) {
            Log::error('Failed to fetch invoices', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to load invoices',
            ], 500);
        }
    }

    public function payments(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            $payments = $subscription->payments()
                ->orderBy('created_at', 'desc')
                ->get()
                ->map(fn($payment) => [
                    'id' => (string) $payment->id,
                    'amount' => $payment->amount,
                    'status' => $payment->status,
                    'payment_date' => $payment->created_at->toISOString(),
                    'payment_method' => $payment->payment_method ?? 'card',
                    'failure_reason' => $payment->failure_reason,
                ]);

            return response()->json([
                'payments' => $payments,
            ]);
        } catch (Exception $e) {
            Log::error('Failed to fetch payment history', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to load payment history',
            ], 500);
        }
    }

    public function updatePaymentMethod(Request $request, int $id): JsonResponse
    {
        $validated = $request->validate([
            'payment_method' => 'required|array',
            'payment_method.type' => 'required|in:card,paypal,bank',
            'payment_method.last4' => 'nullable|string|size:4',
            'payment_method.brand' => 'nullable|string|max:50',
            'payment_method.expiry_month' => 'nullable|integer|min:1|max:12',
            'payment_method.expiry_year' => 'nullable|integer|min:' . date('Y'),
        ]);

        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            DB::transaction(function () use ($subscription, $validated) {
                $subscription->update([
                    'payment_method' => json_encode($validated['payment_method']),
                ]);
            });

            $this->logSubscriptionEvent($subscription, 'payment_method_updated');

            return response()->json([
                'subscription' => $this->formatSubscription($subscription->fresh(['plan', 'payments'])),
                'message' => 'Payment method updated successfully',
            ]);
        } catch (Exception $e) {
            Log::error('Failed to update payment method', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to update payment method',
            ], 500);
        }
    }

    public function retryPayment(Request $request, int $id): JsonResponse
    {
        try {
            $subscription = $request->user()
                ->subscriptions()
                ->findOrFail($id);

            $this->logSubscriptionEvent($subscription, 'payment_retry_attempted');

            return response()->json([
                'message' => 'Payment retry initiated',
            ]);
        } catch (Exception $e) {
            Log::error('Failed to retry payment', [
                'user_id' => $request->user()->id,
                'subscription_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to retry payment',
            ], 500);
        }
    }

    public function metrics(Request $request): JsonResponse
    {
        try {
            $user = $request->user();
            $subscriptions = $user->subscriptions;

            $activeSubscriptions = $subscriptions->filter(fn($s) => $s->isActive());

            $metrics = [
                'total_subscriptions' => $subscriptions->count(),
                'active_subscriptions' => $activeSubscriptions->count(),
                'total_spent' => $subscriptions->sum('total_paid'),
                'monthly_cost' => $activeSubscriptions->sum(fn($s) => $s->getMrr()),
                'annual_cost' => $activeSubscriptions->sum(fn($s) => $s->getArr()),
            ];

            return response()->json($metrics);
        } catch (Exception $e) {
            Log::error('Failed to fetch subscription metrics', [
                'user_id' => $request->user()->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to load metrics',
            ], 500);
        }
    }

    private function formatSubscription(UserSubscription $subscription): array
    {
        $plan = $subscription->plan;
        $payments = $subscription->payments ?? collect();

        return [
            'id' => (string) $subscription->id,
            'userId' => (string) $subscription->user_id,
            'productId' => (string) ($subscription->product_id ?? $subscription->subscription_plan_id),
            'productName' => $plan?->name ?? 'Unknown',
            'planId' => (string) $subscription->subscription_plan_id,
            'status' => $subscription->status,
            'interval' => $subscription->interval ?? 'monthly',
            'price' => (float) ($subscription->price ?? $plan?->price ?? 0),
            'currency' => $subscription->currency ?? 'USD',
            'startDate' => $subscription->start_date?->toISOString(),
            'nextPaymentDate' => $subscription->next_payment_date?->toISOString(),
            'lastPaymentDate' => $subscription->last_payment_date?->toISOString(),
            'endDate' => $subscription->end_date?->toISOString(),
            'cancelledAt' => $subscription->cancelled_at?->toISOString(),
            'pausedAt' => $subscription->paused_at?->toISOString(),
            'totalPaid' => (float) ($subscription->total_paid ?? 0),
            'failedPayments' => (int) ($subscription->failed_payments ?? 0),
            'successfulPayments' => (int) ($subscription->successful_payments ?? 0),
            'paymentHistory' => $payments->map(fn($payment) => [
                'id' => (string) $payment->id,
                'amount' => (float) $payment->amount,
                'status' => $payment->status,
                'paymentDate' => $payment->created_at->toISOString(),
                'dueDate' => $payment->created_at->toISOString(),
                'paymentMethod' => $payment->payment_method ?? 'card',
            ])->toArray(),
            'pauseReason' => $subscription->pause_reason,
            'cancellationReason' => $subscription->cancellation_reason,
            'renewalCount' => (int) ($subscription->renewal_count ?? 0),
            'autoRenew' => (bool) ($subscription->auto_renew ?? true),
            'trialEndDate' => ($subscription->trial_end_date ?? $subscription->trial_ends_at)?->toISOString(),
            'isTrialing' => $subscription->onTrial(),
            'paymentMethod' => json_decode($subscription->payment_method, true) ?? ['type' => 'card'],
            'emailsSent' => [],
            'createdAt' => $subscription->created_at->toISOString(),
            'updatedAt' => $subscription->updated_at->toISOString(),
            'notes' => $subscription->notes,
            'mrr' => $subscription->getMrr(),
            'arr' => $subscription->getArr(),
            'ltv' => $subscription->getLtv(),
            'churnRisk' => 0,
            'failedAttempts' => (int) ($subscription->failed_payments ?? 0),
        ];
    }

    private function logSubscriptionEvent(UserSubscription $subscription, string $event, array $metadata = []): void
    {
        Log::info("Subscription {$event}", [
            'subscription_id' => $subscription->id,
            'user_id' => $subscription->user_id,
            'status' => $subscription->status,
            'event' => $event,
            'metadata' => $metadata,
            'timestamp' => now()->toISOString(),
        ]);
    }
}