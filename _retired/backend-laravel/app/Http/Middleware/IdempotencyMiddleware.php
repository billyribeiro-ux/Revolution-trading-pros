<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Symfony\Component\HttpFoundation\Response;

/**
 * Idempotency Middleware
 *
 * Ensures webhook requests are processed exactly once using
 * idempotency keys. Prevents duplicate processing on retries.
 *
 * Usage:
 * - Client sends X-Idempotency-Key header
 * - First request is processed and response cached
 * - Subsequent requests with same key return cached response
 *
 * @version 1.0.0
 */
class IdempotencyMiddleware
{
    /**
     * Cache TTL for idempotency keys (24 hours).
     */
    private const CACHE_TTL = 86400;

    /**
     * Header name for idempotency key.
     */
    private const HEADER_NAME = 'X-Idempotency-Key';

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Only apply to non-GET requests
        if ($request->isMethod('GET')) {
            return $next($request);
        }

        $idempotencyKey = $this->getIdempotencyKey($request);

        // If no key provided, process normally
        if ($idempotencyKey === null) {
            return $next($request);
        }

        $cacheKey = $this->buildCacheKey($idempotencyKey, $request);

        // Check for existing response
        $cachedResponse = Cache::get($cacheKey);

        if ($cachedResponse !== null) {
            return $this->buildResponseFromCache($cachedResponse);
        }

        // Mark as processing to prevent race conditions
        $lockKey = "{$cacheKey}:lock";
        $acquired = Cache::add($lockKey, true, 30);

        if (!$acquired) {
            // Another request is processing this key
            return response()->json([
                'error' => 'Request is being processed',
                'idempotency_key' => $idempotencyKey,
            ], 409);
        }

        try {
            /** @var Response $response */
            $response = $next($request);

            // Only cache successful responses
            if ($response->isSuccessful()) {
                $this->cacheResponse($cacheKey, $response);
            }

            // Add header to indicate this was the original request
            $response->headers->set('X-Idempotency-Processed', 'true');

            return $response;
        } finally {
            Cache::forget($lockKey);
        }
    }

    /**
     * Get idempotency key from request.
     */
    private function getIdempotencyKey(Request $request): ?string
    {
        // Check header first
        $key = $request->header(self::HEADER_NAME);

        if ($key !== null) {
            return $key;
        }

        // For webhooks, use message ID if available
        $messageId = $request->input('MessageID')
            ?? $request->input('messageId')
            ?? $request->input('mail.messageId');

        return $messageId;
    }

    /**
     * Build cache key including request path for isolation.
     */
    private function buildCacheKey(string $idempotencyKey, Request $request): string
    {
        $path = $request->path();

        return "idempotency:{$path}:{$idempotencyKey}";
    }

    /**
     * Cache the response.
     */
    private function cacheResponse(string $cacheKey, Response $response): void
    {
        $data = [
            'status' => $response->getStatusCode(),
            'headers' => $response->headers->all(),
            'content' => $response->getContent(),
        ];

        Cache::put($cacheKey, $data, self::CACHE_TTL);
    }

    /**
     * Build response from cached data.
     *
     * @param array{status: int, headers: array<string, array<string>>, content: string} $cached
     */
    private function buildResponseFromCache(array $cached): Response
    {
        $response = response($cached['content'], $cached['status']);

        // Restore important headers
        foreach (['Content-Type', 'X-Correlation-ID', 'X-Request-ID'] as $header) {
            if (isset($cached['headers'][strtolower($header)][0])) {
                $response->headers->set($header, $cached['headers'][strtolower($header)][0]);
            }
        }

        // Indicate this is a cached/replayed response
        $response->headers->set('X-Idempotency-Replayed', 'true');

        return $response;
    }
}
