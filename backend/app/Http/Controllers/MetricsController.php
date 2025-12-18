<?php

declare(strict_types=1);

namespace App\Http\Controllers;

use App\Support\Metrics\MetricsCollector;
use Illuminate\Http\Response;

/**
 * Metrics Controller
 *
 * Exposes Prometheus metrics endpoint for monitoring.
 *
 * @version 1.0.0
 */
class MetricsController extends Controller
{
    public function __construct(
        private readonly MetricsCollector $metrics,
    ) {}

    /**
     * Export metrics in Prometheus format.
     *
     * @route GET /metrics
     */
    public function __invoke(): Response
    {
        $output = $this->metrics->export();

        return response($output, 200, [
            'Content-Type' => 'text/plain; version=0.0.4; charset=utf-8',
            'Cache-Control' => 'no-cache, no-store, must-revalidate',
        ]);
    }
}
