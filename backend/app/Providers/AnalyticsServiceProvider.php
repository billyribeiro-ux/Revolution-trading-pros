<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use App\Services\Analytics\AnalyticsCacheManager;
use App\Services\Analytics\EventIngestionService;
use App\Services\Analytics\KPIComputationEngine;
use App\Services\Analytics\CohortAnalyticsEngine;
use App\Services\Analytics\FunnelAnalyticsEngine;
use App\Services\Analytics\AttributionEngine;
use App\Services\Analytics\ForecastingEngine;
use App\Services\Analytics\AnomalyDetectionEngine;

/**
 * AnalyticsServiceProvider - Enterprise Analytics Engine Registration
 *
 * Registers all analytics services as singletons for dependency injection.
 */
class AnalyticsServiceProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register(): void
    {
        // Register cache manager first (no dependencies)
        $this->app->singleton(AnalyticsCacheManager::class, function ($app) {
            return new AnalyticsCacheManager();
        });

        // Register attribution engine
        $this->app->singleton(AttributionEngine::class, function ($app) {
            return new AttributionEngine(
                $app->make(AnalyticsCacheManager::class)
            );
        });

        // Register event ingestion service
        $this->app->singleton(EventIngestionService::class, function ($app) {
            return new EventIngestionService(
                $app->make(AnalyticsCacheManager::class),
                $app->make(AttributionEngine::class)
            );
        });

        // Register anomaly detection engine
        $this->app->singleton(AnomalyDetectionEngine::class, function ($app) {
            return new AnomalyDetectionEngine(
                $app->make(AnalyticsCacheManager::class)
            );
        });

        // Register KPI computation engine
        $this->app->singleton(KPIComputationEngine::class, function ($app) {
            return new KPIComputationEngine(
                $app->make(AnomalyDetectionEngine::class),
                $app->make(AnalyticsCacheManager::class)
            );
        });

        // Register cohort analytics engine
        $this->app->singleton(CohortAnalyticsEngine::class, function ($app) {
            return new CohortAnalyticsEngine(
                $app->make(AnalyticsCacheManager::class)
            );
        });

        // Register funnel analytics engine
        $this->app->singleton(FunnelAnalyticsEngine::class, function ($app) {
            return new FunnelAnalyticsEngine(
                $app->make(AnalyticsCacheManager::class)
            );
        });

        // Register forecasting engine
        $this->app->singleton(ForecastingEngine::class, function ($app) {
            return new ForecastingEngine(
                $app->make(AnalyticsCacheManager::class)
            );
        });
    }

    /**
     * Bootstrap services.
     */
    public function boot(): void
    {
        // Seed default KPIs, Cohorts, Funnels, and Segments on first run
        // This would typically be done via a seeder or command
    }
}
