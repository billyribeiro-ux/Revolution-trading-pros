<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\Facades\Gate;
use Laravel\Horizon\Horizon;
use Laravel\Horizon\HorizonApplicationServiceProvider;

/**
 * Horizon Service Provider
 *
 * Configures Laravel Horizon dashboard access and notifications.
 *
 * @see https://laravel.com/docs/12.x/horizon
 */
final class HorizonServiceProvider extends HorizonApplicationServiceProvider
{
    /**
     * Bootstrap any application services.
     */
    public function boot(): void
    {
        parent::boot();

        // Configure notification settings
        // Horizon::routeSmsNotificationsTo('15556667777');
        // Horizon::routeMailNotificationsTo('admin@example.com');
        // Horizon::routeSlackNotificationsTo('slack-webhook-url', '#channel');

        // Night mode by default (easier on the eyes)
        Horizon::night();
    }

    /**
     * Register the Horizon gate.
     *
     * This gate determines who can access Horizon in non-local environments.
     */
    protected function gate(): void
    {
        Gate::define('viewHorizon', function ($user): bool {
            // Allow access for users with admin or superadmin role
            if (method_exists($user, 'hasRole')) {
                return $user->hasRole(['admin', 'superadmin']);
            }

            // Fallback: Check email domain for admins
            $adminEmails = [
                'admin@revolutiontradingpros.com',
                // Add other admin emails here
            ];

            return in_array($user->email, $adminEmails, true);
        });
    }
}
