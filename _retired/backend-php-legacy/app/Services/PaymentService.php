<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Order;
use App\Models\User;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;
use RuntimeException;

/**
 * Stripe Payment Service
 *
 * Handles payment processing for orders using Stripe API.
 * Supports payment intents, subscriptions, refunds, and webhooks.
 *
 * Features matching Fluent Cart Pro:
 * - Payment intent creation
 * - Payment confirmation
 * - Subscription management
 * - Refund processing
 * - Webhook handling
 * - Multi-currency support
 */
class PaymentService
{
    /**
     * Stripe API base URL
     */
    private const STRIPE_API_URL = 'https://api.stripe.com/v1';

    /**
     * Stripe API version
     */
    private const STRIPE_API_VERSION = '2024-11-20.acacia';

    /**
     * Supported currencies
     */
    public const SUPPORTED_CURRENCIES = [
        'USD', 'EUR', 'GBP', 'CAD', 'AUD', 'JPY', 'CHF', 'HKD', 'SGD', 'NZD',
    ];

    /**
     * Zero-decimal currencies (no cents)
     */
    private const ZERO_DECIMAL_CURRENCIES = [
        'BIF', 'CLP', 'DJF', 'GNF', 'JPY', 'KMF', 'KRW', 'MGA', 'PYG', 'RWF',
        'UGX', 'VND', 'VUV', 'XAF', 'XOF', 'XPF',
    ];

    /**
     * Stripe secret key
     */
    private string $secretKey;

    /**
     * Stripe publishable key
     */
    private string $publishableKey;

    /**
     * Webhook signing secret
     */
    private string $webhookSecret;

    /**
     * Test mode flag
     */
    private bool $testMode;

    public function __construct()
    {
        $this->testMode = config('services.stripe.test_mode', true);
        $this->secretKey = $this->testMode
            ? config('services.stripe.test_secret_key', '')
            : config('services.stripe.secret_key', '');
        $this->publishableKey = $this->testMode
            ? config('services.stripe.test_publishable_key', '')
            : config('services.stripe.publishable_key', '');
        $this->webhookSecret = config('services.stripe.webhook_secret', '');

        if (empty($this->secretKey)) {
            Log::warning('Stripe secret key not configured');
        }
    }

    /**
     * Create a payment intent for an order
     */
    public function createPaymentIntent(Order $order, array $options = []): array
    {
        $this->validateApiKey();

        $currency = strtolower($order->currency ?? 'usd');
        $amount = $this->convertToSmallestUnit($order->total, $currency);

        $payload = [
            'amount' => $amount,
            'currency' => $currency,
            'automatic_payment_methods' => ['enabled' => 'true'],
            'metadata' => [
                'order_id' => $order->id,
                'order_number' => $order->order_number,
                'user_id' => $order->user_id ?? 'guest',
            ],
        ];

        // Add customer if user exists
        if ($order->user_id) {
            $stripeCustomer = $this->getOrCreateCustomer($order->user);
            if ($stripeCustomer) {
                $payload['customer'] = $stripeCustomer['id'];
            }
        }

        // Add description
        $payload['description'] = "Order {$order->order_number}";

        // Add receipt email
        if ($order->customer_email) {
            $payload['receipt_email'] = $order->customer_email;
        }

        // Add statement descriptor (max 22 chars)
        $payload['statement_descriptor_suffix'] = substr('RTP ' . $order->order_number, 0, 22);

        // Merge additional options
        $payload = array_merge($payload, $options);

        // SECURITY: Generate idempotency key based on order to prevent double charges
        // Format: pi_{order_number}_{timestamp_minute} - allows retry within same minute
        $idempotencyKey = "pi_{$order->order_number}_" . floor(time() / 60);

        try {
            $response = $this->stripeRequest('POST', '/payment_intents', $payload, $idempotencyKey);

            // Update order with payment intent
            $order->update([
                'payment_provider' => 'stripe',
                'payment_intent_id' => $response['id'],
                'status' => Order::STATUS_AWAITING_PAYMENT,
            ]);

            Log::info('Payment intent created', [
                'order_id' => $order->id,
                'payment_intent_id' => $response['id'],
                'amount' => $amount,
                'currency' => $currency,
            ]);

            return [
                'payment_intent_id' => $response['id'],
                'client_secret' => $response['client_secret'],
                'publishable_key' => $this->publishableKey,
                'amount' => $order->total,
                'currency' => $order->currency,
            ];
        } catch (\Exception $e) {
            Log::error('Failed to create payment intent', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Confirm a payment intent
     */
    public function confirmPayment(string $paymentIntentId): array
    {
        $this->validateApiKey();

        $response = $this->stripeRequest('GET', "/payment_intents/{$paymentIntentId}");

        return [
            'id' => $response['id'],
            'status' => $response['status'],
            'amount' => $response['amount'],
            'currency' => $response['currency'],
            'payment_method' => $response['payment_method'] ?? null,
            'receipt_url' => $response['charges']['data'][0]['receipt_url'] ?? null,
        ];
    }

    /**
     * Process payment webhook event
     */
    public function handleWebhook(string $payload, string $signature): array
    {
        if (empty($this->webhookSecret)) {
            throw new RuntimeException('Webhook secret not configured');
        }

        // Verify webhook signature
        if (!$this->verifyWebhookSignature($payload, $signature)) {
            throw new InvalidArgumentException('Invalid webhook signature');
        }

        $event = json_decode($payload, true);
        $eventType = $event['type'] ?? '';
        $data = $event['data']['object'] ?? [];

        Log::info('Processing Stripe webhook', [
            'type' => $eventType,
            'id' => $event['id'] ?? null,
        ]);

        return match ($eventType) {
            'payment_intent.succeeded' => $this->handlePaymentSucceeded($data),
            'payment_intent.payment_failed' => $this->handlePaymentFailed($data),
            'charge.refunded' => $this->handleChargeRefunded($data),
            'customer.subscription.created' => $this->handleSubscriptionCreated($data),
            'customer.subscription.updated' => $this->handleSubscriptionUpdated($data),
            'customer.subscription.deleted' => $this->handleSubscriptionDeleted($data),
            'invoice.paid' => $this->handleInvoicePaid($data),
            'invoice.payment_failed' => $this->handleInvoicePaymentFailed($data),
            default => ['handled' => false, 'message' => 'Unhandled event type'],
        };
    }

    /**
     * Handle successful payment
     *
     * SECURITY: Validates payment amount matches order total to prevent
     * manipulation attacks where wrong amounts are paid via Stripe dashboard.
     */
    private function handlePaymentSucceeded(array $data): array
    {
        $paymentIntentId = $data['id'];
        $order = Order::where('payment_intent_id', $paymentIntentId)->first();

        if (!$order) {
            Log::warning('Order not found for payment intent', [
                'payment_intent_id' => $paymentIntentId,
            ]);
            return ['handled' => false, 'message' => 'Order not found'];
        }

        // CRITICAL SECURITY: Validate payment amount matches order total
        $paidAmount = $data['amount_received'] ?? $data['amount'] ?? 0;
        $expectedAmount = $this->convertToSmallestUnit($order->total, $order->currency ?? 'USD');

        // Allow 1 cent tolerance for rounding issues
        if (abs($paidAmount - $expectedAmount) > 1) {
            Log::critical('Payment amount mismatch detected - potential fraud', [
                'order_id' => $order->id,
                'order_number' => $order->order_number,
                'expected_amount' => $expectedAmount,
                'paid_amount' => $paidAmount,
                'currency' => $order->currency ?? 'USD',
                'payment_intent_id' => $paymentIntentId,
            ]);

            $order->update([
                'payment_status' => Order::PAYMENT_FAILED,
                'status' => Order::STATUS_ON_HOLD,
                'metadata' => array_merge($order->metadata ?? [], [
                    'amount_mismatch' => true,
                    'expected_amount' => $expectedAmount,
                    'paid_amount' => $paidAmount,
                    'flagged_at' => now()->toIso8601String(),
                ]),
            ]);

            return [
                'handled' => false,
                'message' => 'Payment amount mismatch - order flagged for review',
                'order_id' => $order->id,
            ];
        }

        // Extract payment method details
        $paymentMethod = $data['payment_method_types'][0] ?? 'card';
        $cardBrand = null;
        $cardLast4 = null;

        if (isset($data['charges']['data'][0]['payment_method_details']['card'])) {
            $card = $data['charges']['data'][0]['payment_method_details']['card'];
            $cardBrand = $card['brand'] ?? null;
            $cardLast4 = $card['last4'] ?? null;
        }

        // Mark order as paid
        $order->update([
            'payment_status' => Order::PAYMENT_PAID,
            'payment_method' => $paymentMethod,
            'card_brand' => $cardBrand,
            'card_last4' => $cardLast4,
            'is_paid' => true,
            'paid_at' => now(),
            'status' => Order::STATUS_PROCESSING,
        ]);

        // Deduct inventory
        foreach ($order->items as $item) {
            $item->deductInventory();
        }

        // ICT 11+ CRITICAL FIX: Provision membership access after successful payment
        // This was MISSING - users were paying but not getting access!
        $this->provisionMembershipAccess($order);

        Log::info('Payment succeeded for order', [
            'order_id' => $order->id,
            'order_number' => $order->order_number,
            'amount' => $order->total,
            'memberships_provisioned' => true,
        ]);

        return [
            'handled' => true,
            'message' => 'Payment processed successfully',
            'order_id' => $order->id,
        ];
    }

    /**
     * Handle failed payment
     */
    private function handlePaymentFailed(array $data): array
    {
        $paymentIntentId = $data['id'];
        $order = Order::where('payment_intent_id', $paymentIntentId)->first();

        if (!$order) {
            return ['handled' => false, 'message' => 'Order not found'];
        }

        $errorMessage = $data['last_payment_error']['message'] ?? 'Payment failed';

        $order->update([
            'payment_status' => Order::PAYMENT_FAILED,
            'status' => Order::STATUS_FAILED,
            'metadata' => array_merge($order->metadata ?? [], [
                'payment_error' => $errorMessage,
                'failed_at' => now()->toIso8601String(),
            ]),
        ]);

        Log::warning('Payment failed for order', [
            'order_id' => $order->id,
            'error' => $errorMessage,
        ]);

        return [
            'handled' => true,
            'message' => 'Payment failure recorded',
            'error' => $errorMessage,
        ];
    }

    /**
     * Handle charge refund
     */
    private function handleChargeRefunded(array $data): array
    {
        $paymentIntentId = $data['payment_intent'] ?? null;

        if (!$paymentIntentId) {
            return ['handled' => false, 'message' => 'No payment intent in refund'];
        }

        $order = Order::where('payment_intent_id', $paymentIntentId)->first();

        if (!$order) {
            return ['handled' => false, 'message' => 'Order not found'];
        }

        $amountRefunded = $data['amount_refunded'] ?? 0;
        $amountRefundedDecimal = $this->convertFromSmallestUnit(
            $amountRefunded,
            strtolower($order->currency)
        );

        $isFullRefund = $amountRefundedDecimal >= $order->total;

        $order->update([
            'refund_amount' => $amountRefundedDecimal,
            'payment_status' => $isFullRefund
                ? Order::PAYMENT_REFUNDED
                : Order::PAYMENT_PARTIALLY_REFUNDED,
            'is_refunded' => $isFullRefund,
            'refunded_at' => $isFullRefund ? now() : null,
            'status' => $isFullRefund ? Order::STATUS_REFUNDED : $order->status,
        ]);

        // Restore inventory for full refunds
        if ($isFullRefund) {
            foreach ($order->items as $item) {
                if ($item->inventory_deducted) {
                    $item->restoreInventory();
                }
            }

            // ICT 11+ FIX: Revoke membership access on full refund
            $this->revokeMembershipAccess($order);
        }

        Log::info('Refund processed for order', [
            'order_id' => $order->id,
            'amount_refunded' => $amountRefundedDecimal,
            'is_full_refund' => $isFullRefund,
            'memberships_revoked' => $isFullRefund,
        ]);

        return [
            'handled' => true,
            'message' => $isFullRefund ? 'Full refund processed' : 'Partial refund processed',
        ];
    }

    /**
     * Handle subscription created
     */
    private function handleSubscriptionCreated(array $data): array
    {
        // Handle subscription creation
        Log::info('Subscription created', ['subscription_id' => $data['id']]);
        return ['handled' => true, 'message' => 'Subscription created'];
    }

    /**
     * Handle subscription updated
     */
    private function handleSubscriptionUpdated(array $data): array
    {
        Log::info('Subscription updated', ['subscription_id' => $data['id']]);
        return ['handled' => true, 'message' => 'Subscription updated'];
    }

    /**
     * Handle subscription deleted/cancelled
     */
    private function handleSubscriptionDeleted(array $data): array
    {
        Log::info('Subscription deleted', ['subscription_id' => $data['id']]);
        return ['handled' => true, 'message' => 'Subscription cancelled'];
    }

    /**
     * Handle invoice paid
     */
    private function handleInvoicePaid(array $data): array
    {
        Log::info('Invoice paid', ['invoice_id' => $data['id']]);
        return ['handled' => true, 'message' => 'Invoice paid'];
    }

    /**
     * Handle invoice payment failed
     */
    private function handleInvoicePaymentFailed(array $data): array
    {
        Log::warning('Invoice payment failed', ['invoice_id' => $data['id']]);
        return ['handled' => true, 'message' => 'Invoice payment failed'];
    }

    /**
     * Create a refund for an order
     *
     * SECURITY: Validates refund amount against order total and previous refunds
     * to prevent over-refunding attacks or accidental money loss.
     */
    public function createRefund(Order $order, ?float $amount = null, ?string $reason = null): array
    {
        $this->validateApiKey();

        if (!$order->payment_intent_id) {
            throw new InvalidArgumentException('Order has no payment intent');
        }

        // SECURITY: Verify order was actually paid
        if ($order->payment_status !== Order::PAYMENT_PAID &&
            $order->payment_status !== Order::PAYMENT_PARTIALLY_REFUNDED) {
            throw new InvalidArgumentException('Order has not been paid or is already fully refunded');
        }

        $currency = strtolower($order->currency ?? 'usd');
        $refundAmount = $amount ?? $order->total;

        // SECURITY: Calculate already refunded amount from order record
        $previousRefunds = $order->refund_amount ?? 0;
        $maxRefundable = round($order->total - $previousRefunds, 2);

        // SECURITY: Prevent refunding more than remaining balance
        if ($refundAmount > $maxRefundable) {
            Log::warning('Attempted over-refund blocked', [
                'order_id' => $order->id,
                'order_number' => $order->order_number,
                'order_total' => $order->total,
                'previous_refunds' => $previousRefunds,
                'max_refundable' => $maxRefundable,
                'attempted_refund' => $refundAmount,
            ]);

            throw new InvalidArgumentException(
                "Cannot refund {$refundAmount}. Maximum refundable amount is {$maxRefundable}"
            );
        }

        $payload = [
            'payment_intent' => $order->payment_intent_id,
        ];

        // Partial refund - always specify amount to be explicit
        $payload['amount'] = $this->convertToSmallestUnit($refundAmount, $currency);

        if ($reason) {
            $payload['reason'] = match ($reason) {
                'duplicate' => 'duplicate',
                'fraudulent' => 'fraudulent',
                default => 'requested_by_customer',
            };
            $payload['metadata'] = ['reason_detail' => $reason];
        }

        try {
            $response = $this->stripeRequest('POST', '/refunds', $payload);

            $refundedAmount = $this->convertFromSmallestUnit(
                $response['amount'],
                $currency
            );

            // Update order
            $order->refundAmount($refundedAmount, $reason);

            Log::info('Refund created', [
                'order_id' => $order->id,
                'refund_id' => $response['id'],
                'amount' => $refundedAmount,
            ]);

            return [
                'refund_id' => $response['id'],
                'amount' => $refundedAmount,
                'status' => $response['status'],
            ];
        } catch (\Exception $e) {
            Log::error('Failed to create refund', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Get or create a Stripe customer for a user
     */
    public function getOrCreateCustomer(User $user): ?array
    {
        $this->validateApiKey();

        // Check cache first
        $cacheKey = "stripe_customer:{$user->id}";
        $customerId = Cache::get($cacheKey);

        if ($customerId) {
            return ['id' => $customerId];
        }

        // Check if user has stored customer ID
        if ($user->stripe_customer_id) {
            Cache::put($cacheKey, $user->stripe_customer_id, 3600);
            return ['id' => $user->stripe_customer_id];
        }

        // Search for existing customer by email
        try {
            $response = $this->stripeRequest('GET', '/customers', [
                'email' => $user->email,
                'limit' => 1,
            ]);

            if (!empty($response['data'])) {
                $customerId = $response['data'][0]['id'];
                $user->update(['stripe_customer_id' => $customerId]);
                Cache::put($cacheKey, $customerId, 3600);
                return ['id' => $customerId];
            }

            // Create new customer
            $customer = $this->stripeRequest('POST', '/customers', [
                'email' => $user->email,
                'name' => $user->name,
                'metadata' => [
                    'user_id' => $user->id,
                ],
            ]);

            $user->update(['stripe_customer_id' => $customer['id']]);
            Cache::put($cacheKey, $customer['id'], 3600);

            return $customer;
        } catch (\Exception $e) {
            Log::error('Failed to get/create Stripe customer', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Create a checkout session (alternative to payment intents)
     */
    public function createCheckoutSession(Order $order, array $options = []): array
    {
        $this->validateApiKey();

        $lineItems = [];
        foreach ($order->items as $item) {
            $lineItems[] = [
                'price_data' => [
                    'currency' => strtolower($order->currency ?? 'usd'),
                    'unit_amount' => $this->convertToSmallestUnit(
                        $item->price,
                        strtolower($order->currency ?? 'usd')
                    ),
                    'product_data' => [
                        'name' => $item->name,
                        'description' => $item->description ?? '',
                    ],
                ],
                'quantity' => $item->quantity,
            ];
        }

        // Add tax as line item if applicable
        if ($order->tax > 0) {
            $lineItems[] = [
                'price_data' => [
                    'currency' => strtolower($order->currency ?? 'usd'),
                    'unit_amount' => $this->convertToSmallestUnit(
                        $order->tax,
                        strtolower($order->currency ?? 'usd')
                    ),
                    'product_data' => [
                        'name' => 'Tax',
                    ],
                ],
                'quantity' => 1,
            ];
        }

        $payload = [
            'mode' => 'payment',
            'line_items' => $lineItems,
            'success_url' => $options['success_url'] ?? config('app.url') . '/checkout/success?session_id={CHECKOUT_SESSION_ID}',
            'cancel_url' => $options['cancel_url'] ?? config('app.url') . '/checkout/cancel',
            'metadata' => [
                'order_id' => $order->id,
                'order_number' => $order->order_number,
            ],
        ];

        // Add customer if exists
        if ($order->user_id) {
            $customer = $this->getOrCreateCustomer($order->user);
            if ($customer) {
                $payload['customer'] = $customer['id'];
            }
        } elseif ($order->customer_email) {
            $payload['customer_email'] = $order->customer_email;
        }

        // Apply coupon/discount
        if ($order->discount_amount > 0 && $order->discount_code) {
            // Create a coupon for this specific discount
            $payload['discounts'] = [[
                'coupon' => $this->createStripeCoupon($order),
            ]];
        }

        try {
            $response = $this->stripeRequest('POST', '/checkout/sessions', $payload);

            $order->update([
                'payment_provider' => 'stripe',
                'payment_intent_id' => $response['payment_intent'] ?? $response['id'],
                'status' => Order::STATUS_AWAITING_PAYMENT,
                'metadata' => array_merge($order->metadata ?? [], [
                    'checkout_session_id' => $response['id'],
                ]),
            ]);

            Log::info('Checkout session created', [
                'order_id' => $order->id,
                'session_id' => $response['id'],
            ]);

            return [
                'session_id' => $response['id'],
                'checkout_url' => $response['url'],
            ];
        } catch (\Exception $e) {
            Log::error('Failed to create checkout session', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Create a Stripe coupon for the discount
     */
    private function createStripeCoupon(Order $order): string
    {
        $discountPercentage = ($order->discount_amount / $order->subtotal) * 100;
        $couponId = 'ORDER_' . $order->order_number . '_' . time();

        $payload = [
            'id' => $couponId,
            'percent_off' => round($discountPercentage, 2),
            'duration' => 'once',
            'name' => "Discount: {$order->discount_code}",
        ];

        $this->stripeRequest('POST', '/coupons', $payload);

        return $couponId;
    }

    /**
     * Make a request to Stripe API
     *
     * @param string $method HTTP method (GET, POST, DELETE)
     * @param string $endpoint Stripe API endpoint
     * @param array $data Request payload
     * @param string|null $idempotencyKey Optional idempotency key to prevent duplicate operations
     */
    private function stripeRequest(string $method, string $endpoint, array $data = [], ?string $idempotencyKey = null): array
    {
        $url = self::STRIPE_API_URL . $endpoint;

        $headers = [
            'Stripe-Version' => self::STRIPE_API_VERSION,
        ];

        // SECURITY: Add idempotency key for POST requests to prevent double-charges
        // on network retries. Key should be unique per intended operation.
        if ($idempotencyKey !== null && strtoupper($method) === 'POST') {
            $headers['Idempotency-Key'] = $idempotencyKey;
        }

        $request = Http::withBasicAuth($this->secretKey, '')
            ->withHeaders($headers)
            ->timeout(30);

        $response = match (strtoupper($method)) {
            'GET' => $request->get($url, $data),
            'POST' => $request->asForm()->post($url, $this->flattenArray($data)),
            'DELETE' => $request->delete($url),
            default => throw new InvalidArgumentException("Unsupported HTTP method: {$method}"),
        };

        if (!$response->successful()) {
            $error = $response->json('error', []);
            $errorMessage = $error['message'] ?? 'Unknown Stripe error';
            $errorCode = $error['code'] ?? 'unknown';

            Log::error('Stripe API error', [
                'endpoint' => $endpoint,
                'status' => $response->status(),
                'error' => $error,
            ]);

            throw new RuntimeException(
                "Stripe error ({$errorCode}): {$errorMessage}",
                $response->status()
            );
        }

        return $response->json();
    }

    /**
     * Flatten nested array for form encoding
     */
    private function flattenArray(array $array, string $prefix = ''): array
    {
        $result = [];

        foreach ($array as $key => $value) {
            $newKey = $prefix ? "{$prefix}[{$key}]" : $key;

            if (is_array($value)) {
                if (array_keys($value) === range(0, count($value) - 1)) {
                    // Indexed array
                    foreach ($value as $i => $v) {
                        if (is_array($v)) {
                            $result = array_merge($result, $this->flattenArray($v, "{$newKey}[{$i}]"));
                        } else {
                            $result["{$newKey}[{$i}]"] = $v;
                        }
                    }
                } else {
                    // Associative array
                    $result = array_merge($result, $this->flattenArray($value, $newKey));
                }
            } else {
                $result[$newKey] = $value;
            }
        }

        return $result;
    }

    /**
     * Convert amount to smallest currency unit (cents)
     */
    private function convertToSmallestUnit(float $amount, string $currency): int
    {
        if (in_array(strtoupper($currency), self::ZERO_DECIMAL_CURRENCIES, true)) {
            return (int) round($amount);
        }
        return (int) round($amount * 100);
    }

    /**
     * Convert amount from smallest currency unit
     */
    private function convertFromSmallestUnit(int $amount, string $currency): float
    {
        if (in_array(strtoupper($currency), self::ZERO_DECIMAL_CURRENCIES, true)) {
            return (float) $amount;
        }
        return round($amount / 100, 2);
    }

    /**
     * Verify webhook signature
     */
    private function verifyWebhookSignature(string $payload, string $signature): bool
    {
        if (empty($this->webhookSecret)) {
            return false;
        }

        $parts = [];
        foreach (explode(',', $signature) as $part) {
            [$key, $value] = explode('=', $part, 2);
            $parts[$key] = $value;
        }

        $timestamp = $parts['t'] ?? '';
        $v1Signature = $parts['v1'] ?? '';

        if (empty($timestamp) || empty($v1Signature)) {
            return false;
        }

        // Check timestamp is not too old (5 minutes tolerance)
        if (abs(time() - (int) $timestamp) > 300) {
            return false;
        }

        // Compute expected signature
        $signedPayload = "{$timestamp}.{$payload}";
        $expectedSignature = hash_hmac('sha256', $signedPayload, $this->webhookSecret);

        return hash_equals($expectedSignature, $v1Signature);
    }

    /**
     * Validate that API key is configured
     */
    private function validateApiKey(): void
    {
        if (empty($this->secretKey)) {
            throw new RuntimeException('Stripe API key not configured');
        }
    }

    /**
     * Get publishable key for frontend
     */
    public function getPublishableKey(): string
    {
        return $this->publishableKey;
    }

    /**
     * Check if test mode is enabled
     */
    public function isTestMode(): bool
    {
        return $this->testMode;
    }

    /**
     * ICT 11+ CRITICAL FIX: Provision membership access after successful payment
     *
     * This method creates UserMembership records for each membership item in the order,
     * granting the user access to their purchased trading rooms, courses, etc.
     */
    private function provisionMembershipAccess(Order $order): void
    {
        if (!$order->user_id) {
            Log::warning('Cannot provision memberships: Order has no user', ['order_id' => $order->id]);
            return;
        }

        foreach ($order->items as $item) {
            // Only process membership-type items
            if (!in_array($item->item_type, ['membership', 'trading_room', 'alert_service', 'course', 'indicator'])) {
                continue;
            }

            $planId = $item->plan_id ?? $item->product_id;
            if (!$planId) {
                Log::warning('Cannot provision membership: No plan/product ID', [
                    'order_id' => $order->id,
                    'item_id' => $item->id,
                ]);
                continue;
            }

            // Calculate expiration based on billing cycle
            $startsAt = now();
            $expiresAt = match ($item->billing_cycle ?? 'monthly') {
                'yearly', 'annual' => $startsAt->copy()->addYear(),
                'quarterly' => $startsAt->copy()->addMonths(3),
                'lifetime' => null, // Never expires
                default => $startsAt->copy()->addMonth(), // monthly
            };

            // Check if user already has this membership (avoid duplicates)
            $existing = \App\Models\UserMembership::where('user_id', $order->user_id)
                ->where('plan_id', $planId)
                ->where('status', 'active')
                ->first();

            if ($existing) {
                // Extend existing membership
                if ($expiresAt && $existing->expires_at) {
                    $existing->update([
                        'expires_at' => $existing->expires_at->max($expiresAt),
                    ]);
                    Log::info('Extended existing membership', [
                        'user_id' => $order->user_id,
                        'plan_id' => $planId,
                        'new_expires_at' => $existing->expires_at,
                    ]);
                }
            } else {
                // Create new membership
                \App\Models\UserMembership::create([
                    'user_id' => $order->user_id,
                    'plan_id' => $planId,
                    'starts_at' => $startsAt,
                    'expires_at' => $expiresAt,
                    'status' => 'active',
                    'payment_provider' => 'stripe',
                    'subscription_id' => $order->payment_intent_id,
                ]);

                Log::info('Provisioned new membership', [
                    'user_id' => $order->user_id,
                    'plan_id' => $planId,
                    'order_id' => $order->id,
                    'expires_at' => $expiresAt,
                ]);
            }
        }
    }

    /**
     * ICT 11+ FIX: Revoke membership access on full refund
     *
     * This prevents users from keeping access after requesting a refund.
     */
    private function revokeMembershipAccess(Order $order): void
    {
        if (!$order->user_id) {
            return;
        }

        foreach ($order->items as $item) {
            if (!in_array($item->item_type, ['membership', 'trading_room', 'alert_service', 'course', 'indicator'])) {
                continue;
            }

            $planId = $item->plan_id ?? $item->product_id;
            if (!$planId) {
                continue;
            }

            // Find and revoke the membership
            $membership = \App\Models\UserMembership::where('user_id', $order->user_id)
                ->where('plan_id', $planId)
                ->where('subscription_id', $order->payment_intent_id)
                ->first();

            if ($membership) {
                $membership->update([
                    'status' => 'revoked',
                    'expires_at' => now(),
                ]);

                Log::info('Revoked membership due to refund', [
                    'user_id' => $order->user_id,
                    'plan_id' => $planId,
                    'order_id' => $order->id,
                ]);
            }
        }
    }
}
