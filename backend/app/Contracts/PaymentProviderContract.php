<?php

declare(strict_types=1);

namespace App\Contracts;

use App\Models\User;
use App\Models\Order;
use App\Models\UserSubscription;
use App\Models\SubscriptionPayment;
use Illuminate\Http\Request;

/**
 * Payment Provider Contract (ICT9+ Enterprise Grade)
 *
 * Unified interface for all payment providers (Stripe, PayPal, Paddle, etc.)
 *
 * @version 1.0.0
 */
interface PaymentProviderContract
{
    /**
     * Get the provider name
     */
    public function getName(): string;

    /**
     * Check if provider is configured and available
     */
    public function isAvailable(): bool;

    /**
     * Check if in test/sandbox mode
     */
    public function isTestMode(): bool;

    // =========================================================================
    // CUSTOMER MANAGEMENT
    // =========================================================================

    /**
     * Create a customer in the payment provider
     */
    public function createCustomer(User $user, array $options = []): array;

    /**
     * Get customer from payment provider
     */
    public function getCustomer(string $customerId): ?array;

    /**
     * Update customer in payment provider
     */
    public function updateCustomer(string $customerId, array $data): array;

    /**
     * Delete customer from payment provider
     */
    public function deleteCustomer(string $customerId): bool;

    // =========================================================================
    // PAYMENT METHODS
    // =========================================================================

    /**
     * Attach a payment method to customer
     */
    public function attachPaymentMethod(string $customerId, string $paymentMethodId): array;

    /**
     * Detach a payment method from customer
     */
    public function detachPaymentMethod(string $paymentMethodId): bool;

    /**
     * List customer's payment methods
     */
    public function listPaymentMethods(string $customerId, string $type = 'card'): array;

    /**
     * Set default payment method for customer
     */
    public function setDefaultPaymentMethod(string $customerId, string $paymentMethodId): array;

    // =========================================================================
    // ONE-TIME PAYMENTS
    // =========================================================================

    /**
     * Create a payment intent for one-time payment
     */
    public function createPaymentIntent(Order $order, array $options = []): array;

    /**
     * Confirm a payment intent
     */
    public function confirmPaymentIntent(string $paymentIntentId, array $options = []): array;

    /**
     * Cancel a payment intent
     */
    public function cancelPaymentIntent(string $paymentIntentId): array;

    /**
     * Get payment intent details
     */
    public function getPaymentIntent(string $paymentIntentId): ?array;

    // =========================================================================
    // CHECKOUT SESSIONS
    // =========================================================================

    /**
     * Create a checkout session
     */
    public function createCheckoutSession(Order $order, array $urls, array $options = []): array;

    /**
     * Get checkout session details
     */
    public function getCheckoutSession(string $sessionId): ?array;

    // =========================================================================
    // SUBSCRIPTIONS
    // =========================================================================

    /**
     * Create a subscription in the payment provider
     */
    public function createSubscription(
        string $customerId,
        string $priceId,
        array $options = []
    ): array;

    /**
     * Get subscription from payment provider
     */
    public function getSubscription(string $subscriptionId): ?array;

    /**
     * Update subscription in payment provider
     */
    public function updateSubscription(string $subscriptionId, array $data): array;

    /**
     * Cancel subscription in payment provider
     */
    public function cancelSubscription(string $subscriptionId, bool $immediately = false): array;

    /**
     * Pause subscription (if supported)
     */
    public function pauseSubscription(string $subscriptionId): array;

    /**
     * Resume subscription
     */
    public function resumeSubscription(string $subscriptionId): array;

    // =========================================================================
    // INVOICES
    // =========================================================================

    /**
     * Create an invoice
     */
    public function createInvoice(string $customerId, array $items, array $options = []): array;

    /**
     * Get invoice details
     */
    public function getInvoice(string $invoiceId): ?array;

    /**
     * List customer invoices
     */
    public function listInvoices(string $customerId, array $options = []): array;

    /**
     * Pay an invoice
     */
    public function payInvoice(string $invoiceId): array;

    /**
     * Void an invoice
     */
    public function voidInvoice(string $invoiceId): array;

    // =========================================================================
    // REFUNDS
    // =========================================================================

    /**
     * Create a refund
     */
    public function createRefund(string $paymentId, ?int $amount = null, string $reason = ''): array;

    /**
     * Get refund details
     */
    public function getRefund(string $refundId): ?array;

    // =========================================================================
    // PRODUCTS & PRICES
    // =========================================================================

    /**
     * Create a product
     */
    public function createProduct(array $data): array;

    /**
     * Create a price for a product
     */
    public function createPrice(string $productId, array $data): array;

    /**
     * Update a price
     */
    public function updatePrice(string $priceId, array $data): array;

    // =========================================================================
    // WEBHOOKS
    // =========================================================================

    /**
     * Verify webhook signature
     */
    public function verifyWebhookSignature(string $payload, string $signature): bool;

    /**
     * Parse webhook event
     */
    public function parseWebhookEvent(Request $request): array;

    // =========================================================================
    // UTILITIES
    // =========================================================================

    /**
     * Get publishable/public key
     */
    public function getPublishableKey(): string;

    /**
     * Format amount for the provider (handle decimals)
     */
    public function formatAmount(float $amount, string $currency = 'USD'): int;

    /**
     * Parse amount from provider format
     */
    public function parseAmount(int $amount, string $currency = 'USD'): float;

    /**
     * Get supported currencies
     */
    public function getSupportedCurrencies(): array;
}
