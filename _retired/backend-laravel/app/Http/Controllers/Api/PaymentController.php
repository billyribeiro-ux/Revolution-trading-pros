<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Order;
use App\Services\PaymentService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;

/**
 * Payment Controller
 *
 * Handles payment processing endpoints including:
 * - Creating payment intents
 * - Creating checkout sessions
 * - Processing refunds
 * - Handling Stripe webhooks
 *
 * Matches Fluent Cart Pro payment gateway features.
 */
class PaymentController extends Controller
{
    public function __construct(
        private readonly PaymentService $paymentService
    ) {}

    /**
     * Get Stripe publishable key and configuration
     */
    public function config(): JsonResponse
    {
        return response()->json([
            'publishable_key' => $this->paymentService->getPublishableKey(),
            'test_mode' => $this->paymentService->isTestMode(),
            'supported_currencies' => PaymentService::SUPPORTED_CURRENCIES,
        ]);
    }

    /**
     * Create a payment intent for an order
     */
    public function createPaymentIntent(Request $request): JsonResponse
    {
        $request->validate([
            'order_id' => 'required|exists:orders,id',
        ]);

        $order = Order::findOrFail($request->order_id);

        // Verify order belongs to authenticated user
        if ($request->user() && $order->user_id !== $request->user()->id) {
            return response()->json([
                'error' => 'Unauthorized',
            ], 403);
        }

        // Check order is in payable state
        if (!in_array($order->status, [Order::STATUS_PENDING, Order::STATUS_AWAITING_PAYMENT])) {
            return response()->json([
                'error' => 'Order is not in a payable state',
                'current_status' => $order->status,
            ], 400);
        }

        try {
            $result = $this->paymentService->createPaymentIntent($order);

            return response()->json([
                'success' => true,
                'payment_intent' => $result,
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to create payment intent', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Failed to create payment intent',
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Create a Stripe Checkout session for an order
     */
    public function createCheckoutSession(Request $request): JsonResponse
    {
        $request->validate([
            'order_id' => 'required|exists:orders,id',
            'success_url' => 'nullable|url',
            'cancel_url' => 'nullable|url',
        ]);

        $order = Order::with('items')->findOrFail($request->order_id);

        // Verify order belongs to authenticated user
        if ($request->user() && $order->user_id !== $request->user()->id) {
            return response()->json([
                'error' => 'Unauthorized',
            ], 403);
        }

        // Check order is in payable state
        if (!in_array($order->status, [Order::STATUS_PENDING, Order::STATUS_AWAITING_PAYMENT])) {
            return response()->json([
                'error' => 'Order is not in a payable state',
                'current_status' => $order->status,
            ], 400);
        }

        try {
            $options = [];
            if ($request->success_url) {
                $options['success_url'] = $request->success_url;
            }
            if ($request->cancel_url) {
                $options['cancel_url'] = $request->cancel_url;
            }

            $result = $this->paymentService->createCheckoutSession($order, $options);

            return response()->json([
                'success' => true,
                'checkout' => $result,
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to create checkout session', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Failed to create checkout session',
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Confirm payment status for an order
     */
    public function confirmPayment(Request $request): JsonResponse
    {
        $request->validate([
            'order_id' => 'required|exists:orders,id',
        ]);

        $order = Order::findOrFail($request->order_id);

        // Verify order belongs to authenticated user
        if ($request->user() && $order->user_id !== $request->user()->id) {
            return response()->json([
                'error' => 'Unauthorized',
            ], 403);
        }

        if (!$order->payment_intent_id) {
            return response()->json([
                'error' => 'No payment intent for this order',
            ], 400);
        }

        try {
            $result = $this->paymentService->confirmPayment($order->payment_intent_id);

            // Update order if payment succeeded
            if ($result['status'] === 'succeeded' && !$order->is_paid) {
                $order->markAsPaid();
            }

            return response()->json([
                'success' => true,
                'payment_status' => $result['status'],
                'order' => $order->fresh()->toOrderArray(),
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to confirm payment', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Failed to confirm payment',
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Create a refund for an order
     */
    public function refund(Request $request): JsonResponse
    {
        $request->validate([
            'order_id' => 'required|exists:orders,id',
            'amount' => 'nullable|numeric|min:0.01',
            'reason' => 'nullable|string|max:500',
        ]);

        $order = Order::findOrFail($request->order_id);

        // Check if order can be refunded
        if (!$order->canBeRefunded()) {
            return response()->json([
                'error' => 'Order cannot be refunded',
                'is_paid' => $order->is_paid,
                'is_canceled' => $order->is_canceled,
                'refund_amount' => $order->refund_amount,
                'total' => $order->total,
            ], 400);
        }

        try {
            $result = $this->paymentService->createRefund(
                $order,
                $request->amount,
                $request->reason
            );

            return response()->json([
                'success' => true,
                'refund' => $result,
                'order' => $order->fresh()->toOrderArray(),
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to create refund', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Failed to create refund',
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Handle Stripe webhook events
     */
    public function webhook(Request $request): JsonResponse
    {
        $payload = $request->getContent();
        $signature = $request->header('Stripe-Signature', '');

        if (empty($signature)) {
            Log::warning('Stripe webhook missing signature');
            return response()->json(['error' => 'Missing signature'], 400);
        }

        try {
            $result = $this->paymentService->handleWebhook($payload, $signature);

            return response()->json([
                'received' => true,
                ...$result,
            ]);
        } catch (\InvalidArgumentException $e) {
            Log::warning('Invalid webhook signature', [
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Invalid signature',
            ], 400);
        } catch (\Exception $e) {
            Log::error('Webhook processing error', [
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'error' => 'Webhook processing failed',
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get order payment status
     */
    public function orderStatus(Request $request, Order $order): JsonResponse
    {
        // Verify order belongs to authenticated user
        if ($request->user() && $order->user_id !== $request->user()->id) {
            return response()->json([
                'error' => 'Unauthorized',
            ], 403);
        }

        return response()->json([
            'order_number' => $order->order_number,
            'status' => $order->status,
            'payment_status' => $order->payment_status,
            'is_paid' => $order->is_paid,
            'total' => $order->total,
            'currency' => $order->currency,
            'paid_at' => $order->paid_at?->toIso8601String(),
            'payment_method' => $order->payment_method,
            'card_brand' => $order->card_brand,
            'card_last4' => $order->card_last4,
        ]);
    }
}
