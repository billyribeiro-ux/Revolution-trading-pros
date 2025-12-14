<?php

declare(strict_types=1);

namespace App\Providers;

use App\Services\Seo\SeoDataSourceManager;
use App\Services\Seo\KeywordIntelligenceService;
use App\Services\Seo\PredictiveSeoAnalyticsService;
use App\Services\Seo\CompetitorAnalysisService;
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
use App\Services\Seo\Providers\SerpApiProvider;
use App\Services\CacheService;
use App\Services\GoogleSearchConsoleService;
use Illuminate\Support\ServiceProvider;
use Illuminate\Contracts\Foundation\Application;

/**
 * SEO Service Provider
 *
 * Apple Principal Engineer ICT11+ Architecture - MAXIMUM POWER MODE
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Wires up ALL SEO services with proper dependency injection.
 * Implements multi-source data architecture with Google + third-party providers.
 *
 * Services Registered:
 * - SeoDataSourceManager (orchestration layer)
 * - KeywordIntelligenceService (keyword research & analysis)
 * - PredictiveSeoAnalyticsService (traffic prediction & ROI)
 * - CompetitorAnalysisService (competitor intelligence)
 * - GoogleSearchConsoleProvider (GSC data)
 * - GoogleKeywordPlannerProvider (GKP data)
 * - GoogleTrendsProvider (Trends data)
 * - SerpApiProvider (Third-party SERP data)
 *
 * @version 3.0.0
 */
class SeoServiceProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register(): void
    {
        // ═══════════════════════════════════════════════════════════════════════
        // GOOGLE PROVIDERS (Always Enabled)
        // ═══════════════════════════════════════════════════════════════════════

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

        // ═══════════════════════════════════════════════════════════════════════
        // THIRD-PARTY PROVIDERS (Enabled by Default)
        // ═══════════════════════════════════════════════════════════════════════

        // Register SerpAPI Provider
        $this->app->singleton(SerpApiProvider::class, function (Application $app) {
            return new SerpApiProvider(
                $app->make(CacheService::class)
            );
        });

        // ═══════════════════════════════════════════════════════════════════════
        // ORCHESTRATION LAYER
        // ═══════════════════════════════════════════════════════════════════════

        // Register SEO Data Source Manager (orchestration layer)
        $this->app->singleton(SeoDataSourceManager::class, function (Application $app) {
            $serpApiProvider = null;

            // Try to get SerpAPI provider if enabled
            if (config('seo.third_party.serpapi.enabled', true)) {
                try {
                    $serpApiProvider = $app->make(SerpApiProvider::class);
                } catch (\Exception $e) {
                    // SerpAPI not available, continue without it
                }
            }

            return new SeoDataSourceManager(
                $app->make(GoogleSearchConsoleProvider::class),
                $app->make(GoogleKeywordPlannerProvider::class),
                $app->make(GoogleTrendsProvider::class),
                $app->make(CacheService::class),
                $serpApiProvider
            );
        });

        // ═══════════════════════════════════════════════════════════════════════
        // INTELLIGENCE SERVICES
        // ═══════════════════════════════════════════════════════════════════════

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

        // Register Competitor Analysis Service
        $this->app->singleton(CompetitorAnalysisService::class, function (Application $app) {
            $serpApiProvider = null;

            if (config('seo.third_party.serpapi.enabled', true)) {
                try {
                    $serpApiProvider = $app->make(SerpApiProvider::class);
                } catch (\Exception $e) {
                    // Continue without SerpAPI
                }
            }

            return new CompetitorAnalysisService(
                $app->make(SeoDataSourceManager::class),
                $app->make(CacheService::class),
                $serpApiProvider
            );
        });

        // ═══════════════════════════════════════════════════════════════════════
        // ALIASES FOR CONVENIENCE
        // ═══════════════════════════════════════════════════════════════════════

        $this->app->alias(SeoDataSourceManager::class, 'seo.data_sources');
        $this->app->alias(KeywordIntelligenceService::class, 'seo.keywords');
        $this->app->alias(PredictiveSeoAnalyticsService::class, 'seo.analytics');
        $this->app->alias(CompetitorAnalysisService::class, 'seo.competitors');
        $this->app->alias(SerpApiProvider::class, 'seo.serpapi');
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
            // Core Services
            SeoDataSourceManager::class,
            KeywordIntelligenceService::class,
            PredictiveSeoAnalyticsService::class,
            CompetitorAnalysisService::class,

            // Google Providers
            GoogleSearchConsoleProvider::class,
            GoogleKeywordPlannerProvider::class,
            GoogleTrendsProvider::class,

            // Third-Party Providers
            SerpApiProvider::class,

            // Aliases
            'seo.data_sources',
            'seo.keywords',
            'seo.analytics',
            'seo.competitors',
            'seo.serpapi',
        ];
    }
}
