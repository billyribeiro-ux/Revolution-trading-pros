<?php

declare(strict_types=1);

namespace App\Http\Controllers\Webhooks;

use App\Http\Controllers\Controller;
use App\Events\Subscription\SubscriptionCreated;
use App\Events\Subscription\SubscriptionUpdated;
use App\Events\Subscription\SubscriptionCanceled;
use App\Events\Subscription\PaymentSucceeded;
use App\Events\Subscription\PaymentFailed;
use App\Events\Subscription\InvoicePaymentFailed;
use App\Events\Subscription\TrialWillEnd;
use App\Events\Subscription\CustomerUpdated;
use App\Models\User;
use App\Models\Subscription;
use App\Models\Payment;
use App\Models\Invoice;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Log;
use Stripe\Webhook;
use Stripe\Event;

/**
 * Stripe Webhook Controller (ICT9+ Enterprise Grade)
 *
 * Handles all Stripe webhook events:
 * - Subscription lifecycle events
 * - Payment events
 * - Invoice events
 * - Customer events
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class StripeWebhookController extends Controller
{
    /**
     * Handle incoming Stripe webhook
     */
    public function handle(Request $request): Response
    {
        $payload = $request->getContent();
        $signature = $request->header('Stripe-Signature');
        $secret = config('services.stripe.webhook_secret');

        try {
            $event = Webhook::constructEvent($payload, $signature, $secret);
        } catch (\UnexpectedValueException $e) {
            Log::error('Stripe webhook: Invalid payload', ['error' => $e->getMessage()]);
            return response('Invalid payload', 400);
        } catch (\Stripe\Exception\SignatureVerificationException $e) {
            Log::error('Stripe webhook: Invalid signature', ['error' => $e->getMessage()]);
            return response('Invalid signature', 400);
        }

        Log::info('Stripe webhook received', [
            'type' => $event->type,
            'id' => $event->id,
        ]);

        $method = $this->getHandlerMethod($event->type);

        if (method_exists($this, $method)) {
            try {
                $this->$method($event);
            } catch (\Throwable $e) {
                Log::error('Stripe webhook handler error', [
                    'type' => $event->type,
                    'error' => $e->getMessage(),
                    'trace' => $e->getTraceAsString(),
                ]);
                // Return 200 to prevent Stripe from retrying
                // We'll handle failures via logging/alerting
            }
        } else {
            Log::info('Stripe webhook: Unhandled event type', ['type' => $event->type]);
        }

        return response('Webhook handled', 200);
    }

    /**
     * Convert event type to handler method name
     */
    private function getHandlerMethod(string $eventType): string
    {
        return 'handle' . str_replace(' ', '', ucwords(str_replace(['.', '_'], ' ', $eventType)));
    }

    /**
     * Handle customer.subscription.created
     */
    protected function handleCustomerSubscriptionCreated(Event $event): void
    {
        $stripeSubscription = $event->data->object;

        $user = User::where('stripe_customer_id', $stripeSubscription->customer)->first();

        if (!$user) {
            Log::warning('Stripe webhook: User not found for customer', [
                'customer_id' => $stripeSubscription->customer,
            ]);
            return;
        }

        $subscription = Subscription::updateOrCreate(
            ['stripe_subscription_id' => $stripeSubscription->id],
            [
                'user_id' => $user->id,
                'plan_id' => $this->getPlanIdFromStripePrice($stripeSubscription->items->data[0]->price->id),
                'status' => $this->mapStripeStatus($stripeSubscription->status),
                'trial_ends_at' => $stripeSubscription->trial_end
                    ? \Carbon\Carbon::createFromTimestamp($stripeSubscription->trial_end)
                    : null,
                'current_period_start' => \Carbon\Carbon::createFromTimestamp($stripeSubscription->current_period_start),
                'current_period_end' => \Carbon\Carbon::createFromTimestamp($stripeSubscription->current_period_end),
                'cancel_at_period_end' => $stripeSubscription->cancel_at_period_end,
            ]
        );

        event(new SubscriptionCreated($subscription, $user));

        Log::info('Stripe subscription created', [
            'user_id' => $user->id,
            'subscription_id' => $subscription->id,
            'stripe_subscription_id' => $stripeSubscription->id,
        ]);
    }

    /**
     * Handle customer.subscription.updated
     */
    protected function handleCustomerSubscriptionUpdated(Event $event): void
    {
        $stripeSubscription = $event->data->object;
        $previousAttributes = $event->data->previous_attributes ?? [];

        $subscription = Subscription::where('stripe_subscription_id', $stripeSubscription->id)->first();

        if (!$subscription) {
            Log::warning('Stripe webhook: Subscription not found', [
                'stripe_subscription_id' => $stripeSubscription->id,
            ]);
            return;
        }

        $oldStatus = $subscription->status;

        $subscription->update([
            'status' => $this->mapStripeStatus($stripeSubscription->status),
            'plan_id' => $this->getPlanIdFromStripePrice($stripeSubscription->items->data[0]->price->id),
            'trial_ends_at' => $stripeSubscription->trial_end
                ? \Carbon\Carbon::createFromTimestamp($stripeSubscription->trial_end)
                : null,
            'current_period_start' => \Carbon\Carbon::createFromTimestamp($stripeSubscription->current_period_start),
            'current_period_end' => \Carbon\Carbon::createFromTimestamp($stripeSubscription->current_period_end),
            'cancel_at_period_end' => $stripeSubscription->cancel_at_period_end,
            'canceled_at' => $stripeSubscription->canceled_at
                ? \Carbon\Carbon::createFromTimestamp($stripeSubscription->canceled_at)
                : null,
        ]);

        event(new SubscriptionUpdated($subscription, $oldStatus, array_keys($previousAttributes)));

        Log::info('Stripe subscription updated', [
            'subscription_id' => $subscription->id,
            'old_status' => $oldStatus,
            'new_status' => $subscription->status,
            'changed_fields' => array_keys($previousAttributes),
        ]);
    }

    /**
     * Handle customer.subscription.deleted
     */
    protected function handleCustomerSubscriptionDeleted(Event $event): void
    {
        $stripeSubscription = $event->data->object;

        $subscription = Subscription::where('stripe_subscription_id', $stripeSubscription->id)->first();

        if (!$subscription) {
            return;
        }

        $subscription->update([
            'status' => 'canceled',
            'canceled_at' => now(),
            'ends_at' => now(),
        ]);

        event(new SubscriptionCanceled($subscription));

        Log::info('Stripe subscription deleted', [
            'subscription_id' => $subscription->id,
        ]);
    }

    /**
     * Handle customer.subscription.trial_will_end
     */
    protected function handleCustomerSubscriptionTrialWillEnd(Event $event): void
    {
        $stripeSubscription = $event->data->object;

        $subscription = Subscription::where('stripe_subscription_id', $stripeSubscription->id)->first();

        if (!$subscription) {
            return;
        }

        event(new TrialWillEnd($subscription, $subscription->trial_ends_at));

        Log::info('Stripe trial will end', [
            'subscription_id' => $subscription->id,
            'trial_ends_at' => $subscription->trial_ends_at,
        ]);
    }

    /**
     * Handle invoice.payment_succeeded
     */
    protected function handleInvoicePaymentSucceeded(Event $event): void
    {
        $stripeInvoice = $event->data->object;

        $user = User::where('stripe_customer_id', $stripeInvoice->customer)->first();

        if (!$user) {
            return;
        }

        // Create or update invoice record
        $invoice = Invoice::updateOrCreate(
            ['stripe_invoice_id' => $stripeInvoice->id],
            [
                'user_id' => $user->id,
                'subscription_id' => $this->getSubscriptionIdFromStripe($stripeInvoice->subscription),
                'amount' => $stripeInvoice->amount_paid,
                'currency' => $stripeInvoice->currency,
                'status' => 'paid',
                'paid_at' => now(),
                'invoice_pdf' => $stripeInvoice->invoice_pdf,
                'hosted_invoice_url' => $stripeInvoice->hosted_invoice_url,
                'billing_reason' => $stripeInvoice->billing_reason,
            ]
        );

        // Create payment record
        if ($stripeInvoice->payment_intent) {
            Payment::updateOrCreate(
                ['stripe_payment_intent_id' => $stripeInvoice->payment_intent],
                [
                    'user_id' => $user->id,
                    'invoice_id' => $invoice->id,
                    'amount' => $stripeInvoice->amount_paid,
                    'currency' => $stripeInvoice->currency,
                    'status' => 'succeeded',
                    'payment_method' => 'card',
                ]
            );
        }

        event(new PaymentSucceeded($invoice, $user));

        Log::info('Stripe invoice payment succeeded', [
            'user_id' => $user->id,
            'invoice_id' => $invoice->id,
            'amount' => $stripeInvoice->amount_paid,
        ]);
    }

    /**
     * Handle invoice.payment_failed
     */
    protected function handleInvoicePaymentFailed(Event $event): void
    {
        $stripeInvoice = $event->data->object;

        $user = User::where('stripe_customer_id', $stripeInvoice->customer)->first();

        if (!$user) {
            return;
        }

        $subscription = null;
        if ($stripeInvoice->subscription) {
            $subscription = Subscription::where('stripe_subscription_id', $stripeInvoice->subscription)->first();

            if ($subscription) {
                $subscription->increment('payment_attempts');

                // Update subscription status based on attempt count
                if ($subscription->payment_attempts >= 4) {
                    $subscription->update(['status' => 'past_due']);
                }
            }
        }

        $invoice = Invoice::updateOrCreate(
            ['stripe_invoice_id' => $stripeInvoice->id],
            [
                'user_id' => $user->id,
                'subscription_id' => $subscription?->id,
                'amount' => $stripeInvoice->amount_due,
                'currency' => $stripeInvoice->currency,
                'status' => 'failed',
                'attempt_count' => $stripeInvoice->attempt_count,
                'next_payment_attempt' => $stripeInvoice->next_payment_attempt
                    ? \Carbon\Carbon::createFromTimestamp($stripeInvoice->next_payment_attempt)
                    : null,
            ]
        );

        event(new InvoicePaymentFailed($invoice, $user, $stripeInvoice->attempt_count));

        Log::warning('Stripe invoice payment failed', [
            'user_id' => $user->id,
            'invoice_id' => $invoice->id,
            'attempt_count' => $stripeInvoice->attempt_count,
        ]);
    }

    /**
     * Handle charge.succeeded
     */
    protected function handleChargeSucceeded(Event $event): void
    {
        $charge = $event->data->object;

        Log::info('Stripe charge succeeded', [
            'charge_id' => $charge->id,
            'amount' => $charge->amount,
            'customer' => $charge->customer,
        ]);
    }

    /**
     * Handle charge.failed
     */
    protected function handleChargeFailed(Event $event): void
    {
        $charge = $event->data->object;

        $user = User::where('stripe_customer_id', $charge->customer)->first();

        if ($user) {
            event(new PaymentFailed($user, $charge->amount, $charge->failure_message));
        }

        Log::warning('Stripe charge failed', [
            'charge_id' => $charge->id,
            'amount' => $charge->amount,
            'customer' => $charge->customer,
            'failure_message' => $charge->failure_message,
            'failure_code' => $charge->failure_code,
        ]);
    }

    /**
     * Handle charge.refunded
     */
    protected function handleChargeRefunded(Event $event): void
    {
        $charge = $event->data->object;

        Payment::where('stripe_payment_intent_id', $charge->payment_intent)
            ->update([
                'status' => $charge->refunded ? 'refunded' : 'partially_refunded',
                'refunded_amount' => $charge->amount_refunded,
            ]);

        Log::info('Stripe charge refunded', [
            'charge_id' => $charge->id,
            'amount_refunded' => $charge->amount_refunded,
        ]);
    }

    /**
     * Handle customer.created
     */
    protected function handleCustomerCreated(Event $event): void
    {
        $customer = $event->data->object;

        Log::info('Stripe customer created', [
            'customer_id' => $customer->id,
            'email' => $customer->email,
        ]);
    }

    /**
     * Handle customer.updated
     */
    protected function handleCustomerUpdated(Event $event): void
    {
        $customer = $event->data->object;

        $user = User::where('stripe_customer_id', $customer->id)->first();

        if ($user) {
            event(new CustomerUpdated($user, $customer));
        }

        Log::info('Stripe customer updated', [
            'customer_id' => $customer->id,
        ]);
    }

    /**
     * Handle payment_intent.succeeded
     */
    protected function handlePaymentIntentSucceeded(Event $event): void
    {
        $paymentIntent = $event->data->object;

        Payment::updateOrCreate(
            ['stripe_payment_intent_id' => $paymentIntent->id],
            [
                'status' => 'succeeded',
                'amount' => $paymentIntent->amount,
                'currency' => $paymentIntent->currency,
            ]
        );

        Log::info('Stripe payment intent succeeded', [
            'payment_intent_id' => $paymentIntent->id,
            'amount' => $paymentIntent->amount,
        ]);
    }

    /**
     * Handle payment_intent.payment_failed
     */
    protected function handlePaymentIntentPaymentFailed(Event $event): void
    {
        $paymentIntent = $event->data->object;

        Payment::where('stripe_payment_intent_id', $paymentIntent->id)
            ->update([
                'status' => 'failed',
                'failure_reason' => $paymentIntent->last_payment_error?->message,
            ]);

        Log::warning('Stripe payment intent failed', [
            'payment_intent_id' => $paymentIntent->id,
            'error' => $paymentIntent->last_payment_error?->message,
        ]);
    }

    /**
     * Handle payment_method.attached
     */
    protected function handlePaymentMethodAttached(Event $event): void
    {
        $paymentMethod = $event->data->object;

        Log::info('Stripe payment method attached', [
            'payment_method_id' => $paymentMethod->id,
            'customer' => $paymentMethod->customer,
            'type' => $paymentMethod->type,
        ]);
    }

    /**
     * Handle checkout.session.completed
     */
    protected function handleCheckoutSessionCompleted(Event $event): void
    {
        $session = $event->data->object;

        Log::info('Stripe checkout session completed', [
            'session_id' => $session->id,
            'customer' => $session->customer,
            'subscription' => $session->subscription,
            'mode' => $session->mode,
        ]);

        // Handle based on mode
        if ($session->mode === 'subscription' && $session->subscription) {
            // Subscription already handled by subscription.created
        } elseif ($session->mode === 'payment' && $session->payment_intent) {
            // One-time payment - could trigger order fulfillment
        }
    }

    /**
     * Map Stripe subscription status to internal status
     */
    private function mapStripeStatus(string $stripeStatus): string
    {
        return match ($stripeStatus) {
            'trialing' => 'trialing',
            'active' => 'active',
            'past_due' => 'past_due',
            'canceled' => 'canceled',
            'unpaid' => 'unpaid',
            'incomplete' => 'incomplete',
            'incomplete_expired' => 'expired',
            'paused' => 'paused',
            default => 'unknown',
        };
    }

    /**
     * Get internal plan ID from Stripe price ID
     */
    private function getPlanIdFromStripePrice(string $stripePriceId): ?int
    {
        $plan = \App\Models\Plan::where('stripe_price_id', $stripePriceId)->first();
        return $plan?->id;
    }

    /**
     * Get internal subscription ID from Stripe subscription ID
     */
    private function getSubscriptionIdFromStripe(?string $stripeSubscriptionId): ?int
    {
        if (!$stripeSubscriptionId) {
            return null;
        }

        $subscription = Subscription::where('stripe_subscription_id', $stripeSubscriptionId)->first();
        return $subscription?->id;
    }
}
