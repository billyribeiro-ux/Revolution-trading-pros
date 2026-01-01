<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\Facades\Gate;
use Illuminate\Support\ServiceProvider;
use Laravel\Pulse\Facades\Pulse;

/**
 * Pulse Service Provider
 *
 * Configures Laravel Pulse dashboard access and customizations.
 *
 * @see https://laravel.com/docs/12.x/pulse
 */
final class PulseServiceProvider extends ServiceProvider
{
    /**
     * Register any application services.
     */
    public function register(): void
    {
        //
    }

    /**
     * Bootstrap any application services.
     */
    public function boot(): void
    {
        $this->configureGate();
        $this->configureUserResolution();
        $this->configureFiltering();
    }

    /**
     * Configure the Pulse authorization gate.
     */
    private function configureGate(): void
    {
        Gate::define('viewPulse', function ($user): bool {
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

    /**
     * Configure how Pulse resolves user information.
     */
    private function configureUserResolution(): void
    {
        Pulse::users(function ($ids) {
            return \App\Models\User::whereIn('id', $ids)
                ->get()
                ->mapWithKeys(fn ($user) => [
                    $user->id => [
                        'name' => $user->name,
                        'email' => $user->email,
                        'avatar' => $user->avatar_url ?? null,
                    ],
                ]);
        });
    }

    /**
     * Configure request filtering.
     */
    private function configureFiltering(): void
    {
        // Filter out requests that shouldn't be recorded
        Pulse::filter(function ($entry) {
            // Skip health check endpoints
            if (str_starts_with($entry->key ?? '', '/api/health')) {
                return false;
            }

            // Skip Horizon and Pulse requests
            if (str_contains($entry->key ?? '', '/horizon') ||
                str_contains($entry->key ?? '', '/pulse')) {
                return false;
            }

            return true;
        });
    }
}
