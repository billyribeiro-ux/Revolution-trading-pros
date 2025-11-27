<?php

namespace App\Providers;

use App\Models\Form;
use App\Models\Popup;
use App\Policies\FormPolicy;
use App\Policies\PopupPolicy;
use Illuminate\Support\Facades\Gate;
use Illuminate\Support\ServiceProvider;

class AppServiceProvider extends ServiceProvider
{
    /**
     * The policy mappings for the application.
     *
     * @var array<class-string, class-string>
     */
    protected array $policies = [
        Form::class => FormPolicy::class,
        Popup::class => PopupPolicy::class,
    ];

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
        // Register policies
        foreach ($this->policies as $model => $policy) {
            Gate::policy($model, $policy);
        }

        // Register implicit model binding with authorization
        Gate::define('viewAny-form', [FormPolicy::class, 'viewAny']);
        Gate::define('viewAny-popup', [PopupPolicy::class, 'viewAny']);
    }
}
