<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\PerformanceOptimizationService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;

/**
 * PerformanceController - Website Performance Optimization API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Endpoints for monitoring and optimizing website performance.
 *
 * @version 1.0.0
 */
class PerformanceController extends Controller
{
    public function __construct(
        private readonly PerformanceOptimizationService $performanceService
    ) {}

    /**
     * Get comprehensive performance dashboard
     */
    public function dashboard(): JsonResponse
    {
        $metrics = $this->performanceService->getPerformanceMetrics();

        return response()->json([
            'success' => true,
            'data' => $metrics,
        ]);
    }

    /**
     * Get server metrics
     */
    public function serverMetrics(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => $this->performanceService->getServerMetrics(),
        ]);
    }

    /**
     * Get database metrics
     */
    public function databaseMetrics(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => $this->performanceService->getDatabaseMetrics(),
        ]);
    }

    /**
     * Get cache metrics
     */
    public function cacheMetrics(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => $this->performanceService->getCacheMetrics(),
        ]);
    }

    /**
     * Get optimization recommendations
     */
    public function recommendations(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => [
                'recommendations' => $this->performanceService->getOptimizationRecommendations(),
                'cdn' => $this->performanceService->getCdnRecommendations(),
                'core_web_vitals' => $this->performanceService->getCoreWebVitalsOptimization(),
            ],
        ]);
    }

    /**
     * Warm caches for optimal performance
     */
    public function warmCaches(): JsonResponse
    {
        $result = $this->performanceService->warmCaches();

        return response()->json([
            'success' => true,
            'data' => $result,
            'message' => 'Caches warmed successfully',
        ]);
    }

    /**
     * Analyze database queries
     */
    public function analyzeQueries(): JsonResponse
    {
        $this->performanceService->analyzeQueries();

        return response()->json([
            'success' => true,
            'message' => 'Query analysis enabled. Make requests then call /performance/queries/results',
        ]);
    }

    /**
     * Get query analysis results
     */
    public function queryResults(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => $this->performanceService->getQueryAnalysis(),
        ]);
    }

    /**
     * Get Core Web Vitals optimization guide
     */
    public function coreWebVitals(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => $this->performanceService->getCoreWebVitalsOptimization(),
        ]);
    }
}
