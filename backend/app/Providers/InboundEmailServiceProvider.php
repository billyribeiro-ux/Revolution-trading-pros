<?php

declare(strict_types=1);

namespace App\Providers;

use App\Contracts\Email\EmailAttachmentStorageInterface;
use App\Contracts\Email\EmailThreadingInterface;
use App\Contracts\Email\InboundEmailProcessorInterface;
use App\Services\Email\EmailAttachmentService;
use App\Services\Email\EmailThreadingService;
use App\Services\Email\InboundEmailService;
use App\Support\CircuitBreaker\CircuitBreakerFactory;
use App\Support\Logging\StructuredLogger;
use App\Support\Metrics\MetricsCollector;
use Illuminate\Contracts\Cache\Repository as CacheRepository;
use Illuminate\Support\ServiceProvider;

/**
 * Inbound Email Service Provider
 *
 * Registers inbound email services, contracts, and dependencies.
 *
 * @version 1.0.0
 */
class InboundEmailServiceProvider extends ServiceProvider
{
    /**
     * All of the container bindings that should be registered.
     *
     * @var array<string, string>
     */
    public array $bindings = [
        EmailThreadingInterface::class => EmailThreadingService::class,
        EmailAttachmentStorageInterface::class => EmailAttachmentService::class,
    ];

    /**
     * All of the container singletons that should be registered.
     *
     * @var array<string, string>
     */
    public array $singletons = [
        CircuitBreakerFactory::class => CircuitBreakerFactory::class,
        MetricsCollector::class => MetricsCollector::class,
    ];

    /**
     * Register services.
     */
    public function register(): void
    {
        // Register circuit breaker factory
        $this->app->singleton(CircuitBreakerFactory::class, function ($app) {
            return new CircuitBreakerFactory(
                $app->make(CacheRepository::class),
            );
        });

        // Register metrics collector
        $this->app->singleton(MetricsCollector::class, function ($app) {
            return new MetricsCollector(
                $app->make(CacheRepository::class),
            );
        });

        // Register inbound email processor
        $this->app->bind(InboundEmailProcessorInterface::class, function ($app) {
            return $app->make(InboundEmailService::class);
        });

        // Register structured loggers
        $this->app->bind('logger.email', function () {
            return StructuredLogger::email();
        });

        $this->app->bind('logger.webhook', function () {
            return StructuredLogger::webhook();
        });

        $this->app->bind('logger.security', function () {
            return StructuredLogger::security();
        });
    }

    /**
     * Bootstrap services.
     */
    public function boot(): void
    {
        // Publish configuration
        $this->publishes([
            __DIR__ . '/../../config/inbound-email.php' => config_path('inbound-email.php'),
        ], 'inbound-email-config');

        // Merge configuration
        $this->mergeConfigFrom(
            __DIR__ . '/../../config/inbound-email.php',
            'inbound-email',
        );
    }
}
