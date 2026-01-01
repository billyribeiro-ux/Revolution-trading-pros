<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use App\Models\FormSubmission;
use App\Models\Order;
use App\Models\NewsletterSubscription;
use App\Models\Contact;
use App\Observers\FormSubmissionObserver;
use App\Observers\OrderObserver;
use App\Observers\NewsletterSubscriptionObserver;
use App\Observers\ContactObserver;
use App\Services\Integration\IntegrationHub;
use App\Services\Integration\ConsentService;
use App\Services\Integration\WebSocketService;
use App\Services\Integration\InventoryReservationService;

/**
 * IntegrationServiceProvider
 *
 * Registers all integration services and observers for the unified ecosystem.
 */
class IntegrationServiceProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register(): void
    {
        // Register singleton services
        $this->app->singleton(ConsentService::class);
        $this->app->singleton(WebSocketService::class);
        $this->app->singleton(InventoryReservationService::class);
        $this->app->singleton(IntegrationHub::class);
    }

    /**
     * Bootstrap services.
     */
    public function boot(): void
    {
        // Register model observers
        FormSubmission::observe(FormSubmissionObserver::class);
        Order::observe(OrderObserver::class);
        NewsletterSubscription::observe(NewsletterSubscriptionObserver::class);
        Contact::observe(ContactObserver::class);

        // Schedule inventory reservation cleanup
        $this->app->booted(function () {
            $schedule = $this->app->make(\Illuminate\Console\Scheduling\Schedule::class);

            // Release expired inventory reservations every 5 minutes
            $schedule->command('inventory:release-expired')
                ->everyFiveMinutes()
                ->withoutOverlapping()
                ->runInBackground();
        });
    }
}
