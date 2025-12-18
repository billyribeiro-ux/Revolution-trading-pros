<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use App\Support\Logging\CorrelationContext;
use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * Correlation ID Middleware
 *
 * Initializes correlation context from request headers and
 * adds correlation headers to responses for distributed tracing.
 *
 * @version 1.0.0
 */
class CorrelationIdMiddleware
{
    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Reset context for new request (important for Octane)
        CorrelationContext::reset();

        // Initialize from incoming headers
        CorrelationContext::initFromHeaders($request->headers->all());

        // Add request metadata to context
        CorrelationContext::addContext([
            'http_method' => $request->method(),
            'http_path' => $request->path(),
            'client_ip' => $request->ip(),
            'user_agent' => substr($request->userAgent() ?? '', 0, 100),
        ]);

        // If authenticated, add user context
        if ($request->user()) {
            CorrelationContext::set('user_id', $request->user()->id);
        }

        /** @var Response $response */
        $response = $next($request);

        // Add correlation headers to response
        $response->headers->set('X-Correlation-ID', CorrelationContext::getCorrelationId());
        $response->headers->set('X-Request-ID', CorrelationContext::getRequestId());

        return $response;
    }

    /**
     * Clean up after request (for Octane).
     */
    public function terminate(Request $request, Response $response): void
    {
        CorrelationContext::reset();
    }
}
