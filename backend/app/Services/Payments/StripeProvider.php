<?php

declare(strict_types=1);

namespace App\Services\Payments;

use App\Contracts\PaymentProviderContract;
use App\Models\User;
use App\Models\Order;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;

/**
 * Stripe Payment Provider (ICT9+ Enterprise Grade)
 *
 * Full Stripe API integration with:
 * - Customers, Payment Methods, Payment Intents
 * - Subscriptions with full lifecycle management
 * - Invoices and Refunds
 * - Webhook handling with signature verification
 *
 * @version 1.0.0
 */
class StripeProvider implements PaymentProviderContract
{
    private const API_URL = 'https://api.stripe.com/v1';
    private const API_VERSION = '2024-11-20.acacia';

    private string $secretKey;
    private string $publishableKey;
    private string $webhookSecret;
    private bool $testMode;

    /**
     * Zero-decimal currencies (no cents)
     */
    private const ZERO_DECIMAL_CURRENCIES = [
        'BIF', 'CLP', 'DJF', 'GNF', 'JPY', 'KMF', 'KRW', 'MGA',
        'PYG', 'RWF', 'UGX', 'VND', 'VUV', 'XAF', 'XOF', 'XPF'
    ];

    private const SUPPORTED_CURRENCIES = [
        'USD', 'EUR', 'GBP', 'CAD', 'AUD', 'JPY', 'CHF', 'HKD',
        'SGD', 'NZD', 'SEK', 'DKK', 'NOK', 'MXN', 'BRL', 'INR'
    ];

    public function __construct()
    {
        $this->testMode = config('services.stripe.test_mode', true);
        $this->secretKey = $this->testMode
            ? config('services.stripe.test_secret')
            : config('services.stripe.secret');
        $this->publishableKey = $this->testMode
            ? config('services.stripe.test_publishable')
            : config('services.stripe.publishable');
        $this->webhookSecret = config('services.stripe.webhook_secret', '');
    }

    public function getName(): string
    {
        return 'stripe';
    }

    public function isAvailable(): bool
    {
        return !empty($this->secretKey) && !empty($this->publishableKey);
    }

    public function isTestMode(): bool
    {
        return $this->testMode;
    }

    // =========================================================================
    // CUSTOMER MANAGEMENT
    // =========================================================================

    public function createCustomer(User $user, array $options = []): array
    {
        return $this->request('POST', '/customers', array_merge([
            'email' => $user->email,
            'name' => $user->name ?? "{$user->first_name} {$user->last_name}",
            'metadata' => [
                'user_id' => $user->id,
                'source' => 'revolution_trading_pros',
            ],
        ], $options));
    }

    public function getCustomer(string $customerId): ?array
    {
        try {
            return $this->request('GET', "/customers/{$customerId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    public function updateCustomer(string $customerId, array $data): array
    {
        return $this->request('POST', "/customers/{$customerId}", $data);
    }

    public function deleteCustomer(string $customerId): bool
    {
        try {
            $this->request('DELETE', "/customers/{$customerId}");
            return true;
        } catch (\Exception $e) {
            return false;
        }
    }

    /**
     * Get or create customer for user
     */
    public function getOrCreateCustomer(User $user): array
    {
        // Check if user has Stripe customer ID
        if ($user->stripe_customer_id) {
            $customer = $this->getCustomer($user->stripe_customer_id);
            if ($customer) {
                return $customer;
            }
        }

        // Search by email
        $existing = $this->request('GET', '/customers', [
            'email' => $user->email,
            'limit' => 1,
        ]);

        if (!empty($existing['data'])) {
            $customer = $existing['data'][0];
            $user->update(['stripe_customer_id' => $customer['id']]);
            return $customer;
        }

        // Create new customer
        $customer = $this->createCustomer($user);
        $user->update(['stripe_customer_id' => $customer['id']]);

        return $customer;
    }

    // =========================================================================
    // PAYMENT METHODS
    // =========================================================================

    public function attachPaymentMethod(string $customerId, string $paymentMethodId): array
    {
        return $this->request('POST', "/payment_methods/{$paymentMethodId}/attach", [
            'customer' => $customerId,
        ]);
    }

    public function detachPaymentMethod(string $paymentMethodId): bool
    {
        try {
            $this->request('POST', "/payment_methods/{$paymentMethodId}/detach");
            return true;
        } catch (\Exception $e) {
            return false;
        }
    }

    public function listPaymentMethods(string $customerId, string $type = 'card'): array
    {
        return $this->request('GET', '/payment_methods', [
            'customer' => $customerId,
            'type' => $type,
        ]);
    }

    public function setDefaultPaymentMethod(string $customerId, string $paymentMethodId): array
    {
        return $this->updateCustomer($customerId, [
            'invoice_settings' => [
                'default_payment_method' => $paymentMethodId,
            ],
        ]);
    }

    // =========================================================================
    // ONE-TIME PAYMENTS
    // =========================================================================

    public function createPaymentIntent(Order $order, array $options = []): array
    {
        $customer = $this->getOrCreateCustomer($order->user);

        $data = [
            'amount' => $this->formatAmount($order->total, $order->currency ?? 'USD'),
            'currency' => strtolower($order->currency ?? 'USD'),
            'customer' => $customer['id'],
            'metadata' => [
                'order_id' => $order->id,
                'user_id' => $order->user_id,
            ],
            'automatic_payment_methods' => ['enabled' => true],
            'description' => "Order #{$order->id}",
        ];

        if (!empty($options['payment_method'])) {
            $data['payment_method'] = $options['payment_method'];
        }

        if (!empty($options['receipt_email'])) {
            $data['receipt_email'] = $options['receipt_email'];
        }

        if (!empty($options['statement_descriptor'])) {
            $data['statement_descriptor'] = substr($options['statement_descriptor'], 0, 22);
        }

        return $this->request('POST', '/payment_intents', array_merge($data, $options));
    }

    public function confirmPaymentIntent(string $paymentIntentId, array $options = []): array
    {
        return $this->request('POST', "/payment_intents/{$paymentIntentId}/confirm", $options);
    }

    public function cancelPaymentIntent(string $paymentIntentId): array
    {
        return $this->request('POST', "/payment_intents/{$paymentIntentId}/cancel");
    }

    public function getPaymentIntent(string $paymentIntentId): ?array
    {
        try {
            return $this->request('GET', "/payment_intents/{$paymentIntentId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // CHECKOUT SESSIONS
    // =========================================================================

    public function createCheckoutSession(Order $order, array $urls, array $options = []): array
    {
        $customer = $this->getOrCreateCustomer($order->user);

        $lineItems = [];
        foreach ($order->items as $item) {
            $lineItems[] = [
                'price_data' => [
                    'currency' => strtolower($order->currency ?? 'USD'),
                    'product_data' => [
                        'name' => $item->name,
                        'description' => $item->description ?? null,
                    ],
                    'unit_amount' => $this->formatAmount($item->price, $order->currency ?? 'USD'),
                ],
                'quantity' => $item->quantity,
            ];
        }

        $data = [
            'customer' => $customer['id'],
            'line_items' => $lineItems,
            'mode' => $options['mode'] ?? 'payment',
            'success_url' => $urls['success'] ?? config('app.url') . '/checkout/success?session_id={CHECKOUT_SESSION_ID}',
            'cancel_url' => $urls['cancel'] ?? config('app.url') . '/checkout/cancel',
            'metadata' => [
                'order_id' => $order->id,
                'user_id' => $order->user_id,
            ],
        ];

        if (!empty($options['allow_promotion_codes'])) {
            $data['allow_promotion_codes'] = true;
        }

        return $this->request('POST', '/checkout/sessions', array_merge($data, $options));
    }

    public function getCheckoutSession(string $sessionId): ?array
    {
        try {
            return $this->request('GET', "/checkout/sessions/{$sessionId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // SUBSCRIPTIONS
    // =========================================================================

    public function createSubscription(string $customerId, string $priceId, array $options = []): array
    {
        $data = [
            'customer' => $customerId,
            'items' => [['price' => $priceId]],
            'payment_behavior' => 'default_incomplete',
            'payment_settings' => [
                'save_default_payment_method' => 'on_subscription',
            ],
            'expand' => ['latest_invoice.payment_intent'],
        ];

        if (!empty($options['trial_period_days'])) {
            $data['trial_period_days'] = $options['trial_period_days'];
        }

        if (!empty($options['metadata'])) {
            $data['metadata'] = $options['metadata'];
        }

        if (!empty($options['default_payment_method'])) {
            $data['default_payment_method'] = $options['default_payment_method'];
        }

        if (!empty($options['coupon'])) {
            $data['coupon'] = $options['coupon'];
        }

        return $this->request('POST', '/subscriptions', array_merge($data, $options));
    }

    public function getSubscription(string $subscriptionId): ?array
    {
        try {
            return $this->request('GET', "/subscriptions/{$subscriptionId}", [
                'expand' => ['customer', 'default_payment_method', 'latest_invoice'],
            ]);
        } catch (\Exception $e) {
            return null;
        }
    }

    public function updateSubscription(string $subscriptionId, array $data): array
    {
        return $this->request('POST', "/subscriptions/{$subscriptionId}", $data);
    }

    public function cancelSubscription(string $subscriptionId, bool $immediately = false): array
    {
        if ($immediately) {
            return $this->request('DELETE', "/subscriptions/{$subscriptionId}");
        }

        return $this->updateSubscription($subscriptionId, [
            'cancel_at_period_end' => true,
        ]);
    }

    public function pauseSubscription(string $subscriptionId): array
    {
        return $this->updateSubscription($subscriptionId, [
            'pause_collection' => [
                'behavior' => 'mark_uncollectible',
            ],
        ]);
    }

    public function resumeSubscription(string $subscriptionId): array
    {
        return $this->updateSubscription($subscriptionId, [
            'pause_collection' => '',
        ]);
    }

    /**
     * Change subscription plan
     */
    public function changeSubscriptionPlan(string $subscriptionId, string $newPriceId, bool $prorate = true): array
    {
        $subscription = $this->getSubscription($subscriptionId);

        return $this->updateSubscription($subscriptionId, [
            'items' => [
                [
                    'id' => $subscription['items']['data'][0]['id'],
                    'price' => $newPriceId,
                ],
            ],
            'proration_behavior' => $prorate ? 'create_prorations' : 'none',
        ]);
    }

    // =========================================================================
    // INVOICES
    // =========================================================================

    public function createInvoice(string $customerId, array $items, array $options = []): array
    {
        // Add invoice items first
        foreach ($items as $item) {
            $this->request('POST', '/invoiceitems', [
                'customer' => $customerId,
                'amount' => $this->formatAmount($item['amount'], $item['currency'] ?? 'USD'),
                'currency' => strtolower($item['currency'] ?? 'USD'),
                'description' => $item['description'] ?? '',
            ]);
        }

        // Create and finalize invoice
        $invoice = $this->request('POST', '/invoices', array_merge([
            'customer' => $customerId,
            'auto_advance' => true,
        ], $options));

        return $this->request('POST', "/invoices/{$invoice['id']}/finalize");
    }

    public function getInvoice(string $invoiceId): ?array
    {
        try {
            return $this->request('GET', "/invoices/{$invoiceId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    public function listInvoices(string $customerId, array $options = []): array
    {
        return $this->request('GET', '/invoices', array_merge([
            'customer' => $customerId,
            'limit' => 100,
        ], $options));
    }

    public function payInvoice(string $invoiceId): array
    {
        return $this->request('POST', "/invoices/{$invoiceId}/pay");
    }

    public function voidInvoice(string $invoiceId): array
    {
        return $this->request('POST', "/invoices/{$invoiceId}/void");
    }

    /**
     * Get upcoming invoice for subscription
     */
    public function getUpcomingInvoice(string $customerId, ?string $subscriptionId = null): ?array
    {
        try {
            $params = ['customer' => $customerId];
            if ($subscriptionId) {
                $params['subscription'] = $subscriptionId;
            }
            return $this->request('GET', '/invoices/upcoming', $params);
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // REFUNDS
    // =========================================================================

    public function createRefund(string $paymentId, ?int $amount = null, string $reason = ''): array
    {
        $data = ['payment_intent' => $paymentId];

        if ($amount !== null) {
            $data['amount'] = $amount;
        }

        if ($reason) {
            $data['reason'] = $reason;
        }

        return $this->request('POST', '/refunds', $data);
    }

    public function getRefund(string $refundId): ?array
    {
        try {
            return $this->request('GET', "/refunds/{$refundId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // PRODUCTS & PRICES
    // =========================================================================

    public function createProduct(array $data): array
    {
        return $this->request('POST', '/products', $data);
    }

    public function createPrice(string $productId, array $data): array
    {
        return $this->request('POST', '/prices', array_merge([
            'product' => $productId,
        ], $data));
    }

    public function updatePrice(string $priceId, array $data): array
    {
        return $this->request('POST', "/prices/{$priceId}", $data);
    }

    /**
     * Create product with price in one call
     */
    public function createProductWithPrice(array $productData, array $priceData): array
    {
        $product = $this->createProduct($productData);
        $price = $this->createPrice($product['id'], $priceData);

        return [
            'product' => $product,
            'price' => $price,
        ];
    }

    // =========================================================================
    // WEBHOOKS
    // =========================================================================

    public function verifyWebhookSignature(string $payload, string $signature): bool
    {
        if (empty($this->webhookSecret)) {
            Log::warning('Stripe webhook secret not configured');
            return false;
        }

        try {
            $elements = explode(',', $signature);
            $timestamp = null;
            $signatures = [];

            foreach ($elements as $element) {
                $parts = explode('=', $element, 2);
                if (count($parts) === 2) {
                    if ($parts[0] === 't') {
                        $timestamp = $parts[1];
                    } elseif ($parts[0] === 'v1') {
                        $signatures[] = $parts[1];
                    }
                }
            }

            if ($timestamp === null || empty($signatures)) {
                return false;
            }

            // Check timestamp (5 minute tolerance)
            if (abs(time() - (int)$timestamp) > 300) {
                return false;
            }

            $signedPayload = $timestamp . '.' . $payload;
            $expectedSignature = hash_hmac('sha256', $signedPayload, $this->webhookSecret);

            foreach ($signatures as $sig) {
                if (hash_equals($expectedSignature, $sig)) {
                    return true;
                }
            }

            return false;
        } catch (\Exception $e) {
            Log::error('Stripe webhook verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    public function parseWebhookEvent(Request $request): array
    {
        $payload = $request->getContent();
        $signature = $request->header('Stripe-Signature', '');

        if (!$this->verifyWebhookSignature($payload, $signature)) {
            throw new \Exception('Invalid webhook signature');
        }

        return json_decode($payload, true);
    }

    // =========================================================================
    // UTILITIES
    // =========================================================================

    public function getPublishableKey(): string
    {
        return $this->publishableKey;
    }

    public function formatAmount(float $amount, string $currency = 'USD'): int
    {
        $currency = strtoupper($currency);

        if (in_array($currency, self::ZERO_DECIMAL_CURRENCIES)) {
            return (int) round($amount);
        }

        return (int) round($amount * 100);
    }

    public function parseAmount(int $amount, string $currency = 'USD'): float
    {
        $currency = strtoupper($currency);

        if (in_array($currency, self::ZERO_DECIMAL_CURRENCIES)) {
            return (float) $amount;
        }

        return $amount / 100;
    }

    public function getSupportedCurrencies(): array
    {
        return self::SUPPORTED_CURRENCIES;
    }

    // =========================================================================
    // HTTP CLIENT
    // =========================================================================

    private function request(string $method, string $endpoint, array $data = []): array
    {
        $url = self::API_URL . $endpoint;

        $response = Http::withBasicAuth($this->secretKey, '')
            ->withHeaders([
                'Stripe-Version' => self::API_VERSION,
            ])
            ->asForm()
            ->{strtolower($method)}($url, $data);

        if ($response->failed()) {
            $error = $response->json();
            Log::error('Stripe API error', [
                'endpoint' => $endpoint,
                'error' => $error,
            ]);
            throw new \Exception($error['error']['message'] ?? 'Stripe API error');
        }

        return $response->json();
    }
}
