<?php

declare(strict_types=1);

namespace App\Services\Payments;

use App\Contracts\PaymentProviderContract;
use App\Models\User;
use App\Models\Order;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;

/**
 * Paddle Payment Provider (ICT9+ Enterprise Grade)
 *
 * Full integration with Paddle Billing API:
 * - Customer management
 * - Subscription lifecycle
 * - One-time payments
 * - Webhook verification
 * - Price/Product management
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class PaddleProvider implements PaymentProviderContract
{
    private string $apiKey;
    private string $vendorId;
    private string $webhookSecret;
    private string $baseUrl;
    private bool $sandbox;

    public function __construct()
    {
        $this->sandbox = config('services.paddle.sandbox', true);
        $this->apiKey = config('services.paddle.api_key', '');
        $this->vendorId = config('services.paddle.vendor_id', '');
        $this->webhookSecret = config('services.paddle.webhook_secret', '');
        $this->baseUrl = $this->sandbox
            ? 'https://sandbox-api.paddle.com'
            : 'https://api.paddle.com';
    }

    /**
     * Get provider name
     */
    public function getName(): string
    {
        return 'paddle';
    }

    /**
     * Check if provider is available
     */
    public function isAvailable(): bool
    {
        return !empty($this->apiKey) && !empty($this->vendorId);
    }

    /**
     * Create or retrieve a customer
     */
    public function createCustomer(User $user, array $options = []): array
    {
        try {
            // Check if customer already exists
            $existingCustomerId = $user->paddle_customer_id;
            if ($existingCustomerId) {
                return $this->getCustomer($existingCustomerId);
            }

            $response = $this->request('POST', '/customers', [
                'email' => $user->email,
                'name' => $user->name,
                'custom_data' => [
                    'user_id' => (string) $user->id,
                ],
                ...$options,
            ]);

            // Store customer ID on user
            $user->update(['paddle_customer_id' => $response['data']['id']]);

            Log::info('Paddle customer created', [
                'user_id' => $user->id,
                'customer_id' => $response['data']['id'],
            ]);

            return $response['data'];
        } catch (\Throwable $e) {
            Log::error('Paddle customer creation failed', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Get customer by ID
     */
    public function getCustomer(string $customerId): array
    {
        $response = $this->request('GET', "/customers/{$customerId}");
        return $response['data'];
    }

    /**
     * Update customer
     */
    public function updateCustomer(string $customerId, array $data): array
    {
        $response = $this->request('PATCH', "/customers/{$customerId}", $data);
        return $response['data'];
    }

    /**
     * Create a transaction (payment intent equivalent)
     */
    public function createPaymentIntent(Order $order, array $options = []): array
    {
        try {
            $user = $order->user;
            $customerId = $user->paddle_customer_id;

            if (!$customerId) {
                $customer = $this->createCustomer($user);
                $customerId = $customer['id'];
            }

            $items = [];
            foreach ($order->items as $item) {
                $items[] = [
                    'price_id' => $item->paddle_price_id,
                    'quantity' => $item->quantity,
                ];
            }

            $response = $this->request('POST', '/transactions', [
                'customer_id' => $customerId,
                'items' => $items,
                'currency_code' => strtoupper($order->currency ?? 'USD'),
                'custom_data' => [
                    'order_id' => (string) $order->id,
                ],
                'collection_mode' => 'automatic',
                ...$options,
            ]);

            Log::info('Paddle transaction created', [
                'order_id' => $order->id,
                'transaction_id' => $response['data']['id'],
            ]);

            return [
                'transaction_id' => $response['data']['id'],
                'checkout_url' => $response['data']['checkout']['url'] ?? null,
                'status' => $response['data']['status'],
                'raw' => $response['data'],
            ];
        } catch (\Throwable $e) {
            Log::error('Paddle transaction creation failed', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Confirm payment (process transaction)
     */
    public function confirmPayment(string $paymentIntentId, array $options = []): array
    {
        // Paddle transactions are auto-confirmed
        $response = $this->request('GET', "/transactions/{$paymentIntentId}");
        return [
            'success' => in_array($response['data']['status'], ['completed', 'billed']),
            'status' => $response['data']['status'],
            'raw' => $response['data'],
        ];
    }

    /**
     * Create a subscription
     */
    public function createSubscription(string $customerId, string $priceId, array $options = []): array
    {
        try {
            $response = $this->request('POST', '/subscriptions', [
                'customer_id' => $customerId,
                'items' => [
                    ['price_id' => $priceId, 'quantity' => $options['quantity'] ?? 1],
                ],
                'collection_mode' => 'automatic',
                'billing_cycle' => $options['billing_cycle'] ?? null,
                'custom_data' => $options['metadata'] ?? [],
                ...$options,
            ]);

            Log::info('Paddle subscription created', [
                'customer_id' => $customerId,
                'subscription_id' => $response['data']['id'],
            ]);

            return [
                'subscription_id' => $response['data']['id'],
                'status' => $response['data']['status'],
                'current_billing_period' => $response['data']['current_billing_period'] ?? null,
                'next_billed_at' => $response['data']['next_billed_at'] ?? null,
                'raw' => $response['data'],
            ];
        } catch (\Throwable $e) {
            Log::error('Paddle subscription creation failed', [
                'customer_id' => $customerId,
                'price_id' => $priceId,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Get subscription details
     */
    public function getSubscription(string $subscriptionId): array
    {
        $response = $this->request('GET', "/subscriptions/{$subscriptionId}");
        return $response['data'];
    }

    /**
     * Update subscription
     */
    public function updateSubscription(string $subscriptionId, array $data): array
    {
        $response = $this->request('PATCH', "/subscriptions/{$subscriptionId}", $data);
        return $response['data'];
    }

    /**
     * Cancel subscription
     */
    public function cancelSubscription(string $subscriptionId, bool $immediately = false): array
    {
        try {
            $response = $this->request('POST', "/subscriptions/{$subscriptionId}/cancel", [
                'effective_from' => $immediately ? 'immediately' : 'next_billing_period',
            ]);

            Log::info('Paddle subscription canceled', [
                'subscription_id' => $subscriptionId,
                'immediately' => $immediately,
            ]);

            return [
                'success' => true,
                'status' => $response['data']['status'],
                'canceled_at' => $response['data']['canceled_at'] ?? null,
                'raw' => $response['data'],
            ];
        } catch (\Throwable $e) {
            Log::error('Paddle subscription cancellation failed', [
                'subscription_id' => $subscriptionId,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Pause subscription
     */
    public function pauseSubscription(string $subscriptionId, ?string $resumeAt = null): array
    {
        $response = $this->request('POST', "/subscriptions/{$subscriptionId}/pause", [
            'effective_from' => 'next_billing_period',
            'resume_at' => $resumeAt,
        ]);

        return [
            'success' => true,
            'status' => $response['data']['status'],
            'paused_at' => $response['data']['paused_at'] ?? null,
            'raw' => $response['data'],
        ];
    }

    /**
     * Resume subscription
     */
    public function resumeSubscription(string $subscriptionId): array
    {
        $response = $this->request('POST', "/subscriptions/{$subscriptionId}/resume", [
            'effective_from' => 'immediately',
        ]);

        return [
            'success' => true,
            'status' => $response['data']['status'],
            'raw' => $response['data'],
        ];
    }

    /**
     * Change subscription plan
     */
    public function changeSubscriptionPlan(string $subscriptionId, string $newPriceId, bool $prorate = true): array
    {
        $response = $this->request('PATCH', "/subscriptions/{$subscriptionId}", [
            'items' => [
                ['price_id' => $newPriceId, 'quantity' => 1],
            ],
            'proration_billing_mode' => $prorate ? 'prorated_immediately' : 'full_immediately',
        ]);

        Log::info('Paddle subscription plan changed', [
            'subscription_id' => $subscriptionId,
            'new_price_id' => $newPriceId,
        ]);

        return [
            'success' => true,
            'raw' => $response['data'],
        ];
    }

    /**
     * Create refund
     */
    public function createRefund(string $transactionId, ?int $amount = null, string $reason = ''): array
    {
        try {
            $payload = [
                'transaction_id' => $transactionId,
                'reason' => $reason ?: 'Requested by customer',
            ];

            if ($amount !== null) {
                $payload['amount'] = $amount;
            }

            // Get transaction first to find adjustment
            $response = $this->request('POST', '/adjustments', [
                'action' => 'refund',
                'transaction_id' => $transactionId,
                'reason' => $reason ?: 'Customer request',
                'items' => $amount ? [['amount' => $amount]] : [],
            ]);

            Log::info('Paddle refund created', [
                'transaction_id' => $transactionId,
                'adjustment_id' => $response['data']['id'],
            ]);

            return [
                'success' => true,
                'refund_id' => $response['data']['id'],
                'status' => $response['data']['status'],
                'raw' => $response['data'],
            ];
        } catch (\Throwable $e) {
            Log::error('Paddle refund failed', [
                'transaction_id' => $transactionId,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Get invoice/transaction
     */
    public function getInvoice(string $transactionId): array
    {
        $response = $this->request('GET', "/transactions/{$transactionId}");
        return $response['data'];
    }

    /**
     * Get invoice PDF URL
     */
    public function getInvoicePdf(string $transactionId): string
    {
        $response = $this->request('GET', "/transactions/{$transactionId}/invoice");
        return $response['data']['url'] ?? '';
    }

    /**
     * List customer transactions
     */
    public function listInvoices(string $customerId, int $limit = 10): array
    {
        $response = $this->request('GET', '/transactions', [
            'customer_id' => $customerId,
            'per_page' => $limit,
            'order_by' => 'created_at[DESC]',
        ]);

        return $response['data'];
    }

    /**
     * Create a price
     */
    public function createPrice(array $data): array
    {
        $response = $this->request('POST', '/prices', [
            'description' => $data['description'] ?? $data['nickname'] ?? 'Price',
            'product_id' => $data['product_id'],
            'unit_price' => [
                'amount' => (string) $data['unit_amount'],
                'currency_code' => strtoupper($data['currency'] ?? 'USD'),
            ],
            'billing_cycle' => isset($data['interval']) ? [
                'interval' => $data['interval'],
                'frequency' => $data['interval_count'] ?? 1,
            ] : null,
            'trial_period' => isset($data['trial_days']) ? [
                'interval' => 'day',
                'frequency' => $data['trial_days'],
            ] : null,
            'custom_data' => $data['metadata'] ?? [],
        ]);

        return $response['data'];
    }

    /**
     * Get price
     */
    public function getPrice(string $priceId): array
    {
        $response = $this->request('GET', "/prices/{$priceId}");
        return $response['data'];
    }

    /**
     * Create a product
     */
    public function createProduct(array $data): array
    {
        $response = $this->request('POST', '/products', [
            'name' => $data['name'],
            'description' => $data['description'] ?? null,
            'tax_category' => $data['tax_category'] ?? 'standard',
            'image_url' => $data['image_url'] ?? null,
            'custom_data' => $data['metadata'] ?? [],
        ]);

        return $response['data'];
    }

    /**
     * Get product
     */
    public function getProduct(string $productId): array
    {
        $response = $this->request('GET', "/products/{$productId}");
        return $response['data'];
    }

    /**
     * Verify webhook signature
     */
    public function verifyWebhookSignature(string $payload, string $signature): bool
    {
        if (empty($this->webhookSecret)) {
            Log::warning('Paddle webhook secret not configured');
            return false;
        }

        try {
            // Parse the signature header
            $parts = [];
            foreach (explode(';', $signature) as $part) {
                [$key, $value] = explode('=', $part, 2);
                $parts[$key] = $value;
            }

            $timestamp = $parts['ts'] ?? '';
            $hash = $parts['h1'] ?? '';

            // Build signed payload
            $signedPayload = $timestamp . ':' . $payload;

            // Calculate expected signature
            $expectedSignature = hash_hmac('sha256', $signedPayload, $this->webhookSecret);

            // Verify (timing-safe comparison)
            return hash_equals($expectedSignature, $hash);
        } catch (\Throwable $e) {
            Log::error('Paddle webhook signature verification failed', [
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Process webhook event
     */
    public function processWebhook(array $payload): array
    {
        $eventType = $payload['event_type'] ?? 'unknown';

        return match ($eventType) {
            'subscription.created' => $this->handleSubscriptionCreated($payload),
            'subscription.updated' => $this->handleSubscriptionUpdated($payload),
            'subscription.canceled' => $this->handleSubscriptionCanceled($payload),
            'subscription.paused' => $this->handleSubscriptionPaused($payload),
            'subscription.resumed' => $this->handleSubscriptionResumed($payload),
            'subscription.past_due' => $this->handleSubscriptionPastDue($payload),
            'transaction.completed' => $this->handleTransactionCompleted($payload),
            'transaction.payment_failed' => $this->handleTransactionPaymentFailed($payload),
            'customer.created' => $this->handleCustomerCreated($payload),
            'customer.updated' => $this->handleCustomerUpdated($payload),
            default => ['handled' => false, 'event' => $eventType],
        };
    }

    /**
     * Handle subscription created
     */
    protected function handleSubscriptionCreated(array $payload): array
    {
        $data = $payload['data'] ?? [];
        $customerId = $data['customer_id'] ?? null;

        Log::info('Paddle subscription created webhook', [
            'subscription_id' => $data['id'] ?? null,
            'customer_id' => $customerId,
        ]);

        return ['handled' => true, 'action' => 'subscription_created'];
    }

    /**
     * Handle subscription updated
     */
    protected function handleSubscriptionUpdated(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle subscription updated webhook', [
            'subscription_id' => $data['id'] ?? null,
            'status' => $data['status'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'subscription_updated'];
    }

    /**
     * Handle subscription canceled
     */
    protected function handleSubscriptionCanceled(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle subscription canceled webhook', [
            'subscription_id' => $data['id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'subscription_canceled'];
    }

    /**
     * Handle subscription paused
     */
    protected function handleSubscriptionPaused(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle subscription paused webhook', [
            'subscription_id' => $data['id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'subscription_paused'];
    }

    /**
     * Handle subscription resumed
     */
    protected function handleSubscriptionResumed(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle subscription resumed webhook', [
            'subscription_id' => $data['id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'subscription_resumed'];
    }

    /**
     * Handle subscription past due
     */
    protected function handleSubscriptionPastDue(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::warning('Paddle subscription past due webhook', [
            'subscription_id' => $data['id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'subscription_past_due'];
    }

    /**
     * Handle transaction completed
     */
    protected function handleTransactionCompleted(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle transaction completed webhook', [
            'transaction_id' => $data['id'] ?? null,
            'customer_id' => $data['customer_id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'transaction_completed'];
    }

    /**
     * Handle transaction payment failed
     */
    protected function handleTransactionPaymentFailed(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::warning('Paddle transaction payment failed webhook', [
            'transaction_id' => $data['id'] ?? null,
            'error' => $data['payments'][0]['error_code'] ?? 'unknown',
        ]);

        return ['handled' => true, 'action' => 'payment_failed'];
    }

    /**
     * Handle customer created
     */
    protected function handleCustomerCreated(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle customer created webhook', [
            'customer_id' => $data['id'] ?? null,
            'email' => $data['email'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'customer_created'];
    }

    /**
     * Handle customer updated
     */
    protected function handleCustomerUpdated(array $payload): array
    {
        $data = $payload['data'] ?? [];

        Log::info('Paddle customer updated webhook', [
            'customer_id' => $data['id'] ?? null,
        ]);

        return ['handled' => true, 'action' => 'customer_updated'];
    }

    /**
     * Get portal session URL (customer portal)
     */
    public function createPortalSession(string $customerId, string $returnUrl): array
    {
        // Paddle uses a different approach - generate update payment method URL
        $subscriptions = $this->listSubscriptions($customerId);

        $urls = [];
        foreach ($subscriptions as $subscription) {
            $response = $this->request('GET', "/subscriptions/{$subscription['id']}/update-payment-method-transaction");
            $urls[$subscription['id']] = $response['data']['checkout']['url'] ?? null;
        }

        return [
            'update_urls' => $urls,
            'return_url' => $returnUrl,
        ];
    }

    /**
     * List customer subscriptions
     */
    public function listSubscriptions(string $customerId): array
    {
        $response = $this->request('GET', '/subscriptions', [
            'customer_id' => $customerId,
            'status' => 'active,trialing,past_due,paused',
        ]);

        return $response['data'];
    }

    /**
     * Preview subscription update (for proration preview)
     */
    public function previewSubscriptionChange(string $subscriptionId, string $newPriceId): array
    {
        $response = $this->request('POST', "/subscriptions/{$subscriptionId}/preview", [
            'items' => [
                ['price_id' => $newPriceId, 'quantity' => 1],
            ],
            'proration_billing_mode' => 'prorated_immediately',
        ]);

        return [
            'immediate_transaction' => $response['data']['immediate_transaction'] ?? null,
            'next_transaction' => $response['data']['next_transaction'] ?? null,
            'recurring_transaction_details' => $response['data']['recurring_transaction_details'] ?? null,
        ];
    }

    /**
     * Get payment method update URL
     */
    public function getPaymentMethodUpdateUrl(string $subscriptionId): string
    {
        $response = $this->request('GET', "/subscriptions/{$subscriptionId}/update-payment-method-transaction");
        return $response['data']['checkout']['url'] ?? '';
    }

    /**
     * Make API request
     */
    private function request(string $method, string $endpoint, array $data = []): array
    {
        $url = $this->baseUrl . $endpoint;

        $response = Http::withHeaders([
            'Authorization' => 'Bearer ' . $this->apiKey,
            'Content-Type' => 'application/json',
        ])
            ->timeout(30)
            ->retry(3, 100, function ($exception) {
                return $exception instanceof \Illuminate\Http\Client\ConnectionException;
            });

        $response = match (strtoupper($method)) {
            'GET' => $response->get($url, $data),
            'POST' => $response->post($url, $data),
            'PATCH' => $response->patch($url, $data),
            'PUT' => $response->put($url, $data),
            'DELETE' => $response->delete($url, $data),
            default => throw new \InvalidArgumentException("Invalid HTTP method: {$method}"),
        };

        if ($response->failed()) {
            $error = $response->json();
            throw new \Exception(
                'Paddle API error: ' . ($error['error']['detail'] ?? $error['error']['message'] ?? 'Unknown error'),
                $response->status()
            );
        }

        return $response->json();
    }
}
