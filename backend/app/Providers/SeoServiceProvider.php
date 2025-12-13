<?php

declare(strict_types=1);

namespace App\Providers;

use App\Services\Seo\SeoDataSourceManager;
use App\Services\Seo\KeywordIntelligenceService;
use App\Services\Seo\PredictiveSeoAnalyticsService;
use App\Services\Seo\NlpIntelligenceService;
use App\Services\Seo\SeoAnalyzerService;
use App\Services\Seo\InternalLinkIntelligenceService;
use App\Services\Seo\SchemaIntelligenceService;
use App\Services\Seo\AiContentOptimizerService;
use App\Services\Seo\BingSeoService;
use App\Services\Seo\SeoCacheService;
use App\Services\Seo\Providers\GoogleSearchConsoleProvider;
use App\Services\Seo\Providers\GoogleKeywordPlannerProvider;
use App\Services\Seo\Providers\GoogleTrendsProvider;
use App\Services\CacheService;
use App\Services\GoogleSearchConsoleService;
use Illuminate\Support\ServiceProvider;
use Illuminate\Contracts\Foundation\Application;

/**
 * SEO Service Provider
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Wires up all SEO services with proper dependency injection.
 * Implements Google-first data architecture with optional third-party fallbacks.
 *
 * Services Registered:
 * - SeoDataSourceManager (orchestration layer)
 * - KeywordIntelligenceService (keyword research & analysis)
 * - PredictiveSeoAnalyticsService (traffic prediction & ROI)
 * - GoogleSearchConsoleProvider (GSC data)
 * - GoogleKeywordPlannerProvider (GKP data)
 * - GoogleTrendsProvider (Trends data)
 *
 * @version 2.0.0
 */
class SeoServiceProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register(): void
    {
        // Register Google Search Console Provider
        $this->app->singleton(GoogleSearchConsoleProvider::class, function (Application $app) {
            return new GoogleSearchConsoleProvider(
                $app->make(GoogleSearchConsoleService::class),
                $app->make(CacheService::class)
            );
        });

        // Register Google Keyword Planner Provider
        $this->app->singleton(GoogleKeywordPlannerProvider::class, function (Application $app) {
            return new GoogleKeywordPlannerProvider(
                $app->make(CacheService::class)
            );
        });

        // Register Google Trends Provider
        $this->app->singleton(GoogleTrendsProvider::class, function (Application $app) {
            return new GoogleTrendsProvider(
                $app->make(CacheService::class)
            );
        });

        // Register SEO Data Source Manager (orchestration layer)
        $this->app->singleton(SeoDataSourceManager::class, function (Application $app) {
            return new SeoDataSourceManager(
                $app->make(GoogleSearchConsoleProvider::class),
                $app->make(GoogleKeywordPlannerProvider::class),
                $app->make(GoogleTrendsProvider::class),
                $app->make(CacheService::class)
            );
        });

        // Register Keyword Intelligence Service
        $this->app->singleton(KeywordIntelligenceService::class, function (Application $app) {
            return new KeywordIntelligenceService(
                $app->make(SeoDataSourceManager::class),
                $app->make(NlpIntelligenceService::class),
                $app->make(CacheService::class)
            );
        });

        // Register Predictive SEO Analytics Service
        $this->app->singleton(PredictiveSeoAnalyticsService::class, function (Application $app) {
            return new PredictiveSeoAnalyticsService(
                $app->make(SeoDataSourceManager::class),
                $app->make(CacheService::class)
            );
        });

        // Register aliases for backward compatibility
        $this->app->alias(SeoDataSourceManager::class, 'seo.data_sources');
        $this->app->alias(KeywordIntelligenceService::class, 'seo.keywords');
        $this->app->alias(PredictiveSeoAnalyticsService::class, 'seo.analytics');
    }

    /**
     * Bootstrap services.
     */
    public function boot(): void
    {
        // Publish config if needed
        if ($this->app->runningInConsole()) {
            $this->publishes([
                __DIR__ . '/../../config/seo.php' => config_path('seo.php'),
            ], 'seo-config');
        }
    }

    /**
     * Get the services provided by the provider.
     *
     * @return array<string>
     */
    public function provides(): array
    {
        return [
            SeoDataSourceManager::class,
            KeywordIntelligenceService::class,
            PredictiveSeoAnalyticsService::class,
            GoogleSearchConsoleProvider::class,
            GoogleKeywordPlannerProvider::class,
            GoogleTrendsProvider::class,
            'seo.data_sources',
            'seo.keywords',
            'seo.analytics',
        ];
    }
}
