<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Symfony\Component\HttpFoundation\Response;

/**
 * HTTP Cache Headers Middleware
 *
 * Adds proper HTTP caching headers (ETag, Cache-Control, Last-Modified)
 * to API responses for CDN and browser caching.
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class HttpCacheHeaders
{
    /**
     * Cache configuration by route pattern
     */
    private array $cacheConfig = [
        'api/posts' => [
            'max_age' => 300,           // 5 minutes
            'stale_while_revalidate' => 60,
            'stale_if_error' => 3600,   // 1 hour
            'public' => true,
        ],
        'api/posts/*' => [
            'max_age' => 300,
            'stale_while_revalidate' => 60,
            'stale_if_error' => 3600,
            'public' => true,
        ],
        'api/categories' => [
            'max_age' => 3600,          // 1 hour
            'stale_while_revalidate' => 300,
            'public' => true,
        ],
        'api/tags' => [
            'max_age' => 3600,
            'stale_while_revalidate' => 300,
            'public' => true,
        ],
        'api/sitemap' => [
            'max_age' => 3600,
            'public' => true,
        ],
    ];

    /**
     * Routes that should never be cached
     */
    private array $noCacheRoutes = [
        'api/auth/*',
        'api/user/*',
        'api/admin/*',
        'api/cart/*',
        'api/checkout/*',
    ];

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Skip non-GET requests
        if (!$request->isMethod('GET')) {
            return $next($request);
        }

        // Skip routes that shouldn't be cached
        if ($this->shouldNotCache($request)) {
            $response = $next($request);
            return $this->addNoCacheHeaders($response);
        }

        // Check for conditional request (If-None-Match, If-Modified-Since)
        $response = $next($request);

        // Only process successful JSON responses
        if (!$response instanceof JsonResponse || !$response->isSuccessful()) {
            return $response;
        }

        // Generate ETag from response content
        $content = $response->getContent();
        $etag = '"' . md5($content) . '"';

        // Check If-None-Match header
        $ifNoneMatch = $request->header('If-None-Match');
        if ($ifNoneMatch && $ifNoneMatch === $etag) {
            return response()->json(null, 304)
                ->header('ETag', $etag)
                ->header('Cache-Control', $this->getCacheControlHeader($request));
        }

        // Add caching headers
        $response->header('ETag', $etag);
        $response->header('Cache-Control', $this->getCacheControlHeader($request));
        $response->header('Vary', 'Accept, Accept-Encoding, Authorization');

        // Add Last-Modified if we can determine it
        $lastModified = $this->getLastModified($response);
        if ($lastModified) {
            $response->header('Last-Modified', $lastModified);
        }

        return $response;
    }

    /**
     * Check if route should not be cached
     */
    private function shouldNotCache(Request $request): bool
    {
        $path = $request->path();

        foreach ($this->noCacheRoutes as $pattern) {
            if (fnmatch($pattern, $path)) {
                return true;
            }
        }

        // Don't cache authenticated requests
        if ($request->bearerToken() || $request->user()) {
            return true;
        }

        return false;
    }

    /**
     * Get cache configuration for the current route
     */
    private function getCacheConfig(Request $request): ?array
    {
        $path = $request->path();

        // Check exact matches first
        if (isset($this->cacheConfig[$path])) {
            return $this->cacheConfig[$path];
        }

        // Check wildcard patterns
        foreach ($this->cacheConfig as $pattern => $config) {
            if (fnmatch($pattern, $path)) {
                return $config;
            }
        }

        // Default config for API routes
        if (str_starts_with($path, 'api/')) {
            return [
                'max_age' => 60,
                'stale_while_revalidate' => 30,
                'public' => true,
            ];
        }

        return null;
    }

    /**
     * Build Cache-Control header value
     */
    private function getCacheControlHeader(Request $request): string
    {
        $config = $this->getCacheConfig($request);

        if (!$config) {
            return 'no-store, no-cache, must-revalidate';
        }

        $directives = [];

        // Public or private
        $directives[] = $config['public'] ?? true ? 'public' : 'private';

        // Max-age
        if (isset($config['max_age'])) {
            $directives[] = 'max-age=' . $config['max_age'];
        }

        // Stale-while-revalidate (allows serving stale content while fetching fresh)
        if (isset($config['stale_while_revalidate'])) {
            $directives[] = 'stale-while-revalidate=' . $config['stale_while_revalidate'];
        }

        // Stale-if-error (allows serving stale content on error)
        if (isset($config['stale_if_error'])) {
            $directives[] = 'stale-if-error=' . $config['stale_if_error'];
        }

        return implode(', ', $directives);
    }

    /**
     * Add no-cache headers to response
     */
    private function addNoCacheHeaders(Response $response): Response
    {
        $response->headers->set('Cache-Control', 'no-store, no-cache, must-revalidate, max-age=0');
        $response->headers->set('Pragma', 'no-cache');
        $response->headers->set('Expires', 'Thu, 01 Jan 1970 00:00:00 GMT');

        return $response;
    }

    /**
     * Try to extract Last-Modified from response data
     */
    private function getLastModified(JsonResponse $response): ?string
    {
        $content = json_decode($response->getContent(), true);

        // Look for updated_at in data
        $updatedAt = $content['data']['updated_at']
            ?? $content['updated_at']
            ?? null;

        if ($updatedAt) {
            try {
                $date = new \DateTime($updatedAt);
                return $date->format('D, d M Y H:i:s') . ' GMT';
            } catch (\Exception) {
                return null;
            }
        }

        return null;
    }
}
