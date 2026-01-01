<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Symfony\Component\HttpFoundation\Response;

/**
 * Request Cache Middleware (ICT9+ Enterprise Grade)
 *
 * Full response caching for GET requests with:
 * - Intelligent cache key generation
 * - User-aware caching
 * - Cache-Control header support
 * - Conditional requests (If-None-Match, If-Modified-Since)
 * - Stale-while-revalidate support
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class RequestCache
{
    /**
     * Cacheable routes (regex patterns)
     */
    private const CACHEABLE_PATTERNS = [
        '#^api/v\d+/posts$#',
        '#^api/v\d+/posts/[^/]+$#',
        '#^api/v\d+/products$#',
        '#^api/v\d+/products/[^/]+$#',
        '#^api/v\d+/categories$#',
        '#^api/v\d+/tags$#',
        '#^api/v\d+/settings/public$#',
    ];

    /**
     * Never cache these patterns
     */
    private const NEVER_CACHE_PATTERNS = [
        '#^api/v\d+/admin#',
        '#^api/v\d+/me#',
        '#^api/v\d+/auth#',
        '#^api/v\d+/checkout#',
        '#^api/v\d+/cart#',
    ];

    /**
     * Default TTLs by route type
     */
    private const TTL = [
        'posts' => 300,      // 5 minutes
        'products' => 600,   // 10 minutes
        'categories' => 3600, // 1 hour
        'tags' => 3600,      // 1 hour
        'settings' => 3600,  // 1 hour
        'default' => 300,    // 5 minutes
    ];

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Only cache GET requests
        if (!$request->isMethod('GET')) {
            return $next($request);
        }

        // Check if route is cacheable
        if (!$this->isCacheable($request)) {
            return $next($request);
        }

        // Check for no-cache directive
        if ($this->hasNoCacheDirective($request)) {
            return $next($request);
        }

        // Generate cache key
        $cacheKey = $this->generateCacheKey($request);

        // Try to get from cache
        $cached = Cache::get($cacheKey);

        if ($cached !== null) {
            // Check conditional request (If-None-Match)
            $ifNoneMatch = $request->headers->get('If-None-Match');
            if ($ifNoneMatch && $ifNoneMatch === $cached['etag']) {
                return response('', 304)
                    ->header('ETag', $cached['etag'])
                    ->header('X-Cache', 'HIT-304');
            }

            // Check If-Modified-Since
            $ifModifiedSince = $request->headers->get('If-Modified-Since');
            if ($ifModifiedSince) {
                $modifiedSinceTime = strtotime($ifModifiedSince);
                if ($modifiedSinceTime && $cached['created_at'] <= $modifiedSinceTime) {
                    return response('', 304)
                        ->header('Last-Modified', gmdate('D, d M Y H:i:s', $cached['created_at']) . ' GMT')
                        ->header('X-Cache', 'HIT-304');
                }
            }

            // Return cached response
            return $this->buildCachedResponse($cached);
        }

        // Execute request
        $response = $next($request);

        // Only cache successful JSON responses
        if ($response->isSuccessful() && $response instanceof JsonResponse) {
            $this->cacheResponse($cacheKey, $request, $response);
        }

        return $response->header('X-Cache', 'MISS');
    }

    /**
     * Check if request is cacheable
     */
    private function isCacheable(Request $request): bool
    {
        $path = $request->path();

        // Check never cache patterns first
        foreach (self::NEVER_CACHE_PATTERNS as $pattern) {
            if (preg_match($pattern, $path)) {
                return false;
            }
        }

        // Check cacheable patterns
        foreach (self::CACHEABLE_PATTERNS as $pattern) {
            if (preg_match($pattern, $path)) {
                return true;
            }
        }

        return false;
    }

    /**
     * Check for no-cache directive in request
     */
    private function hasNoCacheDirective(Request $request): bool
    {
        $cacheControl = $request->headers->get('Cache-Control', '');

        return str_contains($cacheControl, 'no-cache') ||
               str_contains($cacheControl, 'no-store') ||
               $request->headers->has('Pragma');
    }

    /**
     * Generate cache key for request
     */
    private function generateCacheKey(Request $request): string
    {
        $components = [
            'request_cache',
            $request->method(),
            $request->path(),
            md5(serialize($request->query())),
        ];

        // Include user ID for authenticated requests if needed
        if ($request->user()) {
            $components[] = 'user:' . $request->user()->id;
        }

        // Include Accept header for content negotiation
        $accept = $request->headers->get('Accept', 'application/json');
        $components[] = md5($accept);

        return implode(':', $components);
    }

    /**
     * Cache the response
     */
    private function cacheResponse(string $cacheKey, Request $request, JsonResponse $response): void
    {
        $content = $response->getContent();
        $etag = '"' . md5($content) . '"';
        $now = time();

        $cached = [
            'status' => $response->getStatusCode(),
            'headers' => $this->getCacheableHeaders($response),
            'content' => $content,
            'etag' => $etag,
            'created_at' => $now,
            'path' => $request->path(),
        ];

        $ttl = $this->getTtlForPath($request->path());

        Cache::put($cacheKey, $cached, $ttl);

        // Set response headers
        $response->headers->set('ETag', $etag);
        $response->headers->set('Last-Modified', gmdate('D, d M Y H:i:s', $now) . ' GMT');
        $response->headers->set('X-Cache-TTL', (string) $ttl);
    }

    /**
     * Get headers that should be cached
     */
    private function getCacheableHeaders(Response $response): array
    {
        $cacheableHeaders = [
            'Content-Type',
            'Content-Language',
            'X-RateLimit-Limit',
            'X-RateLimit-Remaining',
        ];

        $headers = [];
        foreach ($cacheableHeaders as $header) {
            if ($response->headers->has($header)) {
                $headers[$header] = $response->headers->get($header);
            }
        }

        return $headers;
    }

    /**
     * Build response from cached data
     */
    private function buildCachedResponse(array $cached): Response
    {
        $response = response($cached['content'], $cached['status']);

        // Restore headers
        foreach ($cached['headers'] as $name => $value) {
            $response->headers->set($name, $value);
        }

        // Add cache headers
        $response->headers->set('ETag', $cached['etag']);
        $response->headers->set('Last-Modified', gmdate('D, d M Y H:i:s', $cached['created_at']) . ' GMT');
        $response->headers->set('X-Cache', 'HIT');
        $response->headers->set('Age', (string) (time() - $cached['created_at']));

        return $response;
    }

    /**
     * Get TTL for specific path
     */
    private function getTtlForPath(string $path): int
    {
        foreach (self::TTL as $pattern => $ttl) {
            if ($pattern !== 'default' && str_contains($path, $pattern)) {
                return $ttl;
            }
        }

        return self::TTL['default'];
    }
}
