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
 * PayPal Payment Provider (ICT9+ Enterprise Grade)
 *
 * Full PayPal REST API integration with:
 * - Orders and Payments
 * - Subscriptions (Billing Agreements)
 * - Webhooks
 *
 * @version 1.0.0
 */
class PayPalProvider implements PaymentProviderContract
{
    private const SANDBOX_URL = 'https://api-m.sandbox.paypal.com';
    private const PRODUCTION_URL = 'https://api-m.paypal.com';

    private string $clientId;
    private string $clientSecret;
    private string $webhookId;
    private bool $testMode;
    private ?string $accessToken = null;
    private ?int $tokenExpiry = null;

    private const SUPPORTED_CURRENCIES = [
        'USD', 'EUR', 'GBP', 'CAD', 'AUD', 'JPY', 'CHF', 'HKD',
        'SGD', 'NZD', 'SEK', 'DKK', 'NOK', 'MXN', 'BRL', 'PLN'
    ];

    public function __construct()
    {
        $this->testMode = config('services.paypal.sandbox', true);
        $this->clientId = config('services.paypal.client_id', '');
        $this->clientSecret = config('services.paypal.client_secret', '');
        $this->webhookId = config('services.paypal.webhook_id', '');
    }

    public function getName(): string
    {
        return 'paypal';
    }

    public function isAvailable(): bool
    {
        return !empty($this->clientId) && !empty($this->clientSecret);
    }

    public function isTestMode(): bool
    {
        return $this->testMode;
    }

    private function getBaseUrl(): string
    {
        return $this->testMode ? self::SANDBOX_URL : self::PRODUCTION_URL;
    }

    // =========================================================================
    // AUTHENTICATION
    // =========================================================================

    private function getAccessToken(): string
    {
        // Check cached token
        if ($this->accessToken && $this->tokenExpiry && time() < $this->tokenExpiry) {
            return $this->accessToken;
        }

        // Check cache
        $cached = Cache::get('paypal:access_token');
        if ($cached) {
            $this->accessToken = $cached['token'];
            $this->tokenExpiry = $cached['expiry'];
            if (time() < $this->tokenExpiry) {
                return $this->accessToken;
            }
        }

        // Get new token
        $response = Http::withBasicAuth($this->clientId, $this->clientSecret)
            ->asForm()
            ->post($this->getBaseUrl() . '/v1/oauth2/token', [
                'grant_type' => 'client_credentials',
            ]);

        if ($response->failed()) {
            throw new \Exception('Failed to get PayPal access token');
        }

        $data = $response->json();
        $this->accessToken = $data['access_token'];
        $this->tokenExpiry = time() + $data['expires_in'] - 60; // 1 minute buffer

        Cache::put('paypal:access_token', [
            'token' => $this->accessToken,
            'expiry' => $this->tokenExpiry,
        ], $data['expires_in'] - 60);

        return $this->accessToken;
    }

    // =========================================================================
    // CUSTOMER MANAGEMENT (PayPal doesn't have traditional customers)
    // =========================================================================

    public function createCustomer(User $user, array $options = []): array
    {
        // PayPal doesn't have a customer object like Stripe
        // We return user data as a reference
        return [
            'id' => 'user_' . $user->id,
            'email' => $user->email,
            'name' => $user->name ?? "{$user->first_name} {$user->last_name}",
            'provider' => 'paypal',
        ];
    }

    public function getCustomer(string $customerId): ?array
    {
        // PayPal doesn't have customers
        return null;
    }

    public function updateCustomer(string $customerId, array $data): array
    {
        return ['id' => $customerId];
    }

    public function deleteCustomer(string $customerId): bool
    {
        return true;
    }

    // =========================================================================
    // PAYMENT METHODS (PayPal handles this internally)
    // =========================================================================

    public function attachPaymentMethod(string $customerId, string $paymentMethodId): array
    {
        return ['id' => $paymentMethodId];
    }

    public function detachPaymentMethod(string $paymentMethodId): bool
    {
        return true;
    }

    public function listPaymentMethods(string $customerId, string $type = 'card'): array
    {
        return ['data' => []];
    }

    public function setDefaultPaymentMethod(string $customerId, string $paymentMethodId): array
    {
        return ['id' => $customerId];
    }

    // =========================================================================
    // ORDERS (PayPal's equivalent to Payment Intents)
    // =========================================================================

    public function createPaymentIntent(Order $order, array $options = []): array
    {
        return $this->createOrder($order, $options);
    }

    public function createOrder(Order $order, array $options = []): array
    {
        $items = [];
        foreach ($order->items as $item) {
            $items[] = [
                'name' => $item->name,
                'quantity' => (string) $item->quantity,
                'unit_amount' => [
                    'currency_code' => $order->currency ?? 'USD',
                    'value' => number_format($item->price, 2, '.', ''),
                ],
            ];
        }

        $data = [
            'intent' => 'CAPTURE',
            'purchase_units' => [
                [
                    'reference_id' => (string) $order->id,
                    'amount' => [
                        'currency_code' => $order->currency ?? 'USD',
                        'value' => number_format($order->total, 2, '.', ''),
                        'breakdown' => [
                            'item_total' => [
                                'currency_code' => $order->currency ?? 'USD',
                                'value' => number_format($order->subtotal ?? $order->total, 2, '.', ''),
                            ],
                        ],
                    ],
                    'items' => $items,
                ],
            ],
            'application_context' => [
                'brand_name' => config('app.name'),
                'user_action' => 'PAY_NOW',
                'return_url' => $options['return_url'] ?? config('app.url') . '/checkout/success',
                'cancel_url' => $options['cancel_url'] ?? config('app.url') . '/checkout/cancel',
            ],
        ];

        return $this->request('POST', '/v2/checkout/orders', $data);
    }

    public function confirmPaymentIntent(string $paymentIntentId, array $options = []): array
    {
        return $this->captureOrder($paymentIntentId);
    }

    public function captureOrder(string $orderId): array
    {
        return $this->request('POST', "/v2/checkout/orders/{$orderId}/capture");
    }

    public function cancelPaymentIntent(string $paymentIntentId): array
    {
        // PayPal orders expire automatically
        return ['id' => $paymentIntentId, 'status' => 'CANCELLED'];
    }

    public function getPaymentIntent(string $paymentIntentId): ?array
    {
        try {
            return $this->request('GET', "/v2/checkout/orders/{$paymentIntentId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // CHECKOUT SESSIONS
    // =========================================================================

    public function createCheckoutSession(Order $order, array $urls, array $options = []): array
    {
        $paypalOrder = $this->createOrder($order, array_merge($options, [
            'return_url' => $urls['success'] ?? config('app.url') . '/checkout/success',
            'cancel_url' => $urls['cancel'] ?? config('app.url') . '/checkout/cancel',
        ]));

        // Find approval URL
        $approvalUrl = null;
        foreach ($paypalOrder['links'] as $link) {
            if ($link['rel'] === 'approve') {
                $approvalUrl = $link['href'];
                break;
            }
        }

        return [
            'id' => $paypalOrder['id'],
            'url' => $approvalUrl,
            'order' => $paypalOrder,
        ];
    }

    public function getCheckoutSession(string $sessionId): ?array
    {
        return $this->getPaymentIntent($sessionId);
    }

    // =========================================================================
    // SUBSCRIPTIONS (Billing Agreements)
    // =========================================================================

    public function createSubscription(string $customerId, string $priceId, array $options = []): array
    {
        $data = [
            'plan_id' => $priceId,
            'application_context' => [
                'brand_name' => config('app.name'),
                'user_action' => 'SUBSCRIBE_NOW',
                'return_url' => $options['return_url'] ?? config('app.url') . '/subscription/success',
                'cancel_url' => $options['cancel_url'] ?? config('app.url') . '/subscription/cancel',
            ],
        ];

        if (!empty($options['start_time'])) {
            $data['start_time'] = $options['start_time'];
        }

        return $this->request('POST', '/v1/billing/subscriptions', $data);
    }

    public function getSubscription(string $subscriptionId): ?array
    {
        try {
            return $this->request('GET', "/v1/billing/subscriptions/{$subscriptionId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    public function updateSubscription(string $subscriptionId, array $data): array
    {
        $patches = [];
        foreach ($data as $key => $value) {
            $patches[] = [
                'op' => 'replace',
                'path' => "/{$key}",
                'value' => $value,
            ];
        }

        return $this->request('PATCH', "/v1/billing/subscriptions/{$subscriptionId}", $patches);
    }

    public function cancelSubscription(string $subscriptionId, bool $immediately = false): array
    {
        $this->request('POST', "/v1/billing/subscriptions/{$subscriptionId}/cancel", [
            'reason' => 'Customer requested cancellation',
        ]);

        return ['id' => $subscriptionId, 'status' => 'CANCELLED'];
    }

    public function pauseSubscription(string $subscriptionId): array
    {
        $this->request('POST', "/v1/billing/subscriptions/{$subscriptionId}/suspend", [
            'reason' => 'Customer requested pause',
        ]);

        return ['id' => $subscriptionId, 'status' => 'SUSPENDED'];
    }

    public function resumeSubscription(string $subscriptionId): array
    {
        $this->request('POST', "/v1/billing/subscriptions/{$subscriptionId}/activate", [
            'reason' => 'Customer requested resume',
        ]);

        return ['id' => $subscriptionId, 'status' => 'ACTIVE'];
    }

    // =========================================================================
    // BILLING PLANS (Products/Prices)
    // =========================================================================

    public function createProduct(array $data): array
    {
        return $this->request('POST', '/v1/catalogs/products', [
            'name' => $data['name'],
            'description' => $data['description'] ?? '',
            'type' => $data['type'] ?? 'SERVICE',
            'category' => $data['category'] ?? 'SOFTWARE',
        ]);
    }

    public function createPrice(string $productId, array $data): array
    {
        return $this->createBillingPlan($productId, $data);
    }

    public function createBillingPlan(string $productId, array $data): array
    {
        $planData = [
            'product_id' => $productId,
            'name' => $data['name'],
            'description' => $data['description'] ?? '',
            'billing_cycles' => [
                [
                    'frequency' => [
                        'interval_unit' => strtoupper($data['interval'] ?? 'MONTH'),
                        'interval_count' => $data['interval_count'] ?? 1,
                    ],
                    'tenure_type' => 'REGULAR',
                    'sequence' => 1,
                    'total_cycles' => 0, // Infinite
                    'pricing_scheme' => [
                        'fixed_price' => [
                            'value' => number_format($data['amount'], 2, '.', ''),
                            'currency_code' => $data['currency'] ?? 'USD',
                        ],
                    ],
                ],
            ],
            'payment_preferences' => [
                'auto_bill_outstanding' => true,
                'payment_failure_threshold' => 3,
            ],
        ];

        return $this->request('POST', '/v1/billing/plans', $planData);
    }

    public function updatePrice(string $priceId, array $data): array
    {
        $patches = [];
        foreach ($data as $key => $value) {
            $patches[] = [
                'op' => 'replace',
                'path' => "/{$key}",
                'value' => $value,
            ];
        }

        return $this->request('PATCH', "/v1/billing/plans/{$priceId}", $patches);
    }

    // =========================================================================
    // INVOICES
    // =========================================================================

    public function createInvoice(string $customerId, array $items, array $options = []): array
    {
        $invoiceItems = [];
        foreach ($items as $item) {
            $invoiceItems[] = [
                'name' => $item['name'] ?? $item['description'],
                'quantity' => (string) ($item['quantity'] ?? 1),
                'unit_amount' => [
                    'currency_code' => $item['currency'] ?? 'USD',
                    'value' => number_format($item['amount'], 2, '.', ''),
                ],
            ];
        }

        $invoice = $this->request('POST', '/v2/invoicing/invoices', [
            'detail' => [
                'currency_code' => $options['currency'] ?? 'USD',
            ],
            'items' => $invoiceItems,
        ]);

        // Send the invoice
        $this->request('POST', "/v2/invoicing/invoices/{$invoice['id']}/send");

        return $invoice;
    }

    public function getInvoice(string $invoiceId): ?array
    {
        try {
            return $this->request('GET', "/v2/invoicing/invoices/{$invoiceId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    public function listInvoices(string $customerId, array $options = []): array
    {
        return $this->request('GET', '/v2/invoicing/invoices', [
            'page_size' => $options['limit'] ?? 20,
        ]);
    }

    public function payInvoice(string $invoiceId): array
    {
        return $this->request('POST', "/v2/invoicing/invoices/{$invoiceId}/payments", [
            'method' => 'PAYPAL',
        ]);
    }

    public function voidInvoice(string $invoiceId): array
    {
        return $this->request('POST', "/v2/invoicing/invoices/{$invoiceId}/cancel");
    }

    // =========================================================================
    // REFUNDS
    // =========================================================================

    public function createRefund(string $paymentId, ?int $amount = null, string $reason = ''): array
    {
        $data = [];

        if ($amount !== null) {
            $data['amount'] = [
                'value' => number_format($amount / 100, 2, '.', ''),
                'currency_code' => 'USD',
            ];
        }

        if ($reason) {
            $data['note_to_payer'] = $reason;
        }

        return $this->request('POST', "/v2/payments/captures/{$paymentId}/refund", $data);
    }

    public function getRefund(string $refundId): ?array
    {
        try {
            return $this->request('GET', "/v2/payments/refunds/{$refundId}");
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // WEBHOOKS
    // =========================================================================

    public function verifyWebhookSignature(string $payload, string $signature): bool
    {
        if (empty($this->webhookId)) {
            return false;
        }

        try {
            $response = $this->request('POST', '/v1/notifications/verify-webhook-signature', [
                'webhook_id' => $this->webhookId,
                'webhook_event' => json_decode($payload, true),
                'cert_url' => request()->header('PAYPAL-CERT-URL'),
                'transmission_id' => request()->header('PAYPAL-TRANSMISSION-ID'),
                'transmission_sig' => request()->header('PAYPAL-TRANSMISSION-SIG'),
                'transmission_time' => request()->header('PAYPAL-TRANSMISSION-TIME'),
                'auth_algo' => request()->header('PAYPAL-AUTH-ALGO'),
            ]);

            return ($response['verification_status'] ?? '') === 'SUCCESS';
        } catch (\Exception $e) {
            Log::error('PayPal webhook verification failed', ['error' => $e->getMessage()]);
            return false;
        }
    }

    public function parseWebhookEvent(Request $request): array
    {
        $payload = $request->getContent();

        if (!$this->verifyWebhookSignature($payload, '')) {
            throw new \Exception('Invalid webhook signature');
        }

        return json_decode($payload, true);
    }

    // =========================================================================
    // UTILITIES
    // =========================================================================

    public function getPublishableKey(): string
    {
        return $this->clientId;
    }

    public function formatAmount(float $amount, string $currency = 'USD'): int
    {
        return (int) round($amount * 100);
    }

    public function parseAmount(int $amount, string $currency = 'USD'): float
    {
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
        $url = $this->getBaseUrl() . $endpoint;

        $http = Http::withToken($this->getAccessToken())
            ->withHeaders([
                'Content-Type' => 'application/json',
                'PayPal-Request-Id' => uniqid('pp_', true),
            ]);

        $response = match (strtoupper($method)) {
            'GET' => $http->get($url, $data),
            'POST' => $http->post($url, $data),
            'PATCH' => $http->patch($url, $data),
            'PUT' => $http->put($url, $data),
            'DELETE' => $http->delete($url),
            default => throw new \Exception("Unsupported HTTP method: {$method}"),
        };

        if ($response->failed()) {
            $error = $response->json();
            Log::error('PayPal API error', [
                'endpoint' => $endpoint,
                'error' => $error,
            ]);
            throw new \Exception($error['message'] ?? $error['error_description'] ?? 'PayPal API error');
        }

        return $response->json() ?? [];
    }
}
