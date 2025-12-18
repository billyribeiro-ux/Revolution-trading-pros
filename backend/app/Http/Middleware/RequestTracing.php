<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Symfony\Component\HttpFoundation\Response;

/**
 * Request Tracing Middleware
 *
 * Google Enterprise Grade request tracing with:
 * - Unique request ID generation
 * - Structured logging for all requests
 * - Performance timing metrics
 * - Error correlation
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class RequestTracing
{
    /**
     * Handle an incoming request
     */
    public function handle(Request $request, Closure $next): Response
    {
        $startTime = microtime(true);

        // Generate or use existing request ID
        $requestId = $request->header('X-Request-ID') ?? Str::uuid()->toString();

        // Store in request for downstream access
        $request->attributes->set('request_id', $requestId);

        // Add to log context for all log calls in this request
        Log::shareContext([
            'request_id' => $requestId,
            'method' => $request->method(),
            'path' => $request->path(),
            'user_id' => $request->user()?->id,
            'ip' => $request->ip(),
            'user_agent' => Str::limit($request->userAgent() ?? '', 100),
        ]);

        // Log request start (debug level for non-production)
        if (config('app.debug')) {
            Log::debug('Request started', [
                'query' => $request->query(),
            ]);
        }

        /** @var Response $response */
        $response = $next($request);

        // Calculate timing
        $duration = (microtime(true) - $startTime) * 1000;

        // Add tracing headers to response
        $response->headers->set('X-Request-ID', $requestId);
        $response->headers->set('X-Response-Time', round($duration, 2) . 'ms');

        // Log request completion
        $logLevel = $response->getStatusCode() >= 500 ? 'error' :
                   ($response->getStatusCode() >= 400 ? 'warning' : 'info');

        Log::$logLevel('Request completed', [
            'status' => $response->getStatusCode(),
            'duration_ms' => round($duration, 2),
            'memory_peak_mb' => round(memory_get_peak_usage(true) / 1024 / 1024, 2),
        ]);

        // Clear shared context
        Log::withoutContext();

        return $response;
    }
}
