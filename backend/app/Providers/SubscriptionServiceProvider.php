<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use Illuminate\Console\Scheduling\Schedule;
use App\Contracts\PaymentProviderContract;
use App\Services\Payments\StripeProvider;
use App\Services\Payments\PayPalProvider;
use App\Services\Payments\PaddleProvider;
use App\Services\Subscription\UsageTracker;
use App\Services\Subscription\TaxCalculator;
use App\Services\Subscription\InvoicePdfGenerator;
use App\Jobs\Subscription\ProcessSubscriptionRenewals;
use App\Jobs\Subscription\ProcessTrialExpirations;
use App\Jobs\Subscription\ProcessDunning;
use App\Http\Middleware\PlanRateLimiter;

/**
 * Subscription Service Provider (ICT9+ Enterprise Grade)
 *
 * Registers all subscription-related services:
 * - Payment providers
 * - Usage tracking
 * - Tax calculation
 * - Scheduled jobs
 *
 * @version 1.0.0
 */
class SubscriptionServiceProvider extends ServiceProvider
{
    /**
     * Register services
     */
    public function register(): void
    {
        // Register payment providers
        $this->app->singleton(StripeProvider::class);
        $this->app->singleton(PayPalProvider::class);
        $this->app->singleton(PaddleProvider::class);

        // Default payment provider
        $this->app->bind(PaymentProviderContract::class, function ($app) {
            $default = config('subscription.default_provider', 'stripe');

            return match ($default) {
                'stripe' => $app->make(StripeProvider::class),
                'paypal' => $app->make(PayPalProvider::class),
                'paddle' => $app->make(PaddleProvider::class),
                default => $app->make(StripeProvider::class),
            };
        });

        // Register subscription services
        $this->app->singleton(UsageTracker::class);
        $this->app->singleton(TaxCalculator::class);
        $this->app->singleton(InvoicePdfGenerator::class);

        // Aliases
        $this->app->alias(UsageTracker::class, 'usage');
        $this->app->alias(TaxCalculator::class, 'tax');
        $this->app->alias(InvoicePdfGenerator::class, 'invoice.pdf');
    }

    /**
     * Bootstrap services
     */
    public function boot(): void
    {
        // Register event listeners
        $this->registerEventListeners();

        // Configure rate limiters
        PlanRateLimiter::configureRateLimiters();

        // Schedule subscription jobs
        $this->app->booted(function () {
            $this->scheduleJobs();
        });
    }

    /**
     * Register event listeners
     */
    private function registerEventListeners(): void
    {
        $events = $this->app['events'];

        // Subscription events
        $events->listen(
            \App\Events\Subscription\SubscriptionCreated::class,
            \App\Listeners\Subscription\SendSubscriptionCreatedNotification::class
        );

        $events->listen(
            \App\Events\Subscription\InvoicePaymentFailed::class,
            \App\Listeners\Subscription\HandlePaymentFailure::class
        );

        $events->listen(
            \App\Events\Subscription\TrialWillEnd::class,
            \App\Listeners\Subscription\SendTrialEndingNotification::class
        );

        // Usage events
        $events->listen(
            \App\Events\UsageLimitApproaching::class,
            function ($event) {
                // Send notification
                $event->user->notify(new \App\Notifications\UsageLimitNotification(
                    $event->feature,
                    $event->currentPercentage,
                    $event->threshold
                ));
            }
        );
    }

    /**
     * Schedule subscription jobs
     */
    private function scheduleJobs(): void
    {
        $schedule = $this->app->make(Schedule::class);

        // Process subscription renewals every hour
        $schedule->job(new ProcessSubscriptionRenewals())
            ->hourly()
            ->withoutOverlapping()
            ->onOneServer();

        // Process trial expirations daily
        $schedule->job(new ProcessTrialExpirations())
            ->dailyAt('06:00')
            ->withoutOverlapping()
            ->onOneServer();

        // Process dunning every 4 hours
        $schedule->job(new ProcessDunning())
            ->everyFourHours()
            ->withoutOverlapping()
            ->onOneServer();

        // Sync metered usage with Stripe daily
        $schedule->call(function () {
            $subscriptions = \App\Models\Subscription::where('status', 'active')
                ->whereNotNull('stripe_subscription_id')
                ->cursor();

            $usageTracker = app(UsageTracker::class);

            foreach ($subscriptions as $subscription) {
                $features = $subscription->plan?->limits ?? [];
                foreach (array_keys($features) as $feature) {
                    $usageTracker->syncWithStripe($subscription, $feature);
                }
            }
        })->dailyAt('00:00')->name('sync-stripe-usage')->onOneServer();
    }

    /**
     * Get payment provider by name
     */
    public static function getProvider(string $name): PaymentProviderContract
    {
        return match ($name) {
            'stripe' => app(StripeProvider::class),
            'paypal' => app(PayPalProvider::class),
            'paddle' => app(PaddleProvider::class),
            default => throw new \InvalidArgumentException("Unknown payment provider: {$name}"),
        };
    }
}
