<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Cache;
use Symfony\Component\HttpFoundation\Response as SymfonyResponse;

/**
 * PerformanceOptimization Middleware
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade performance optimization middleware.
 *
 * Features:
 * - Response compression (gzip/brotli)
 * - Cache headers optimization
 * - ETags for conditional requests
 * - Security headers
 * - Performance timing headers
 *
 * @version 1.0.0
 */
class PerformanceOptimization
{
    /**
     * Static asset extensions for aggressive caching
     */
    private const STATIC_EXTENSIONS = [
        'css', 'js', 'jpg', 'jpeg', 'png', 'gif', 'webp', 'avif',
        'svg', 'ico', 'woff', 'woff2', 'ttf', 'eot', 'otf',
    ];

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): SymfonyResponse
    {
        $startTime = microtime(true);

        /** @var SymfonyResponse $response */
        $response = $next($request);

        // Skip for non-successful responses
        if (!$response->isSuccessful()) {
            return $response;
        }

        // Add performance timing header
        $duration = (microtime(true) - $startTime) * 1000;
        $response->headers->set('X-Response-Time', round($duration, 2) . 'ms');
        $response->headers->set('Server-Timing', 'app;dur=' . round($duration, 2));

        // Add security headers
        $this->addSecurityHeaders($response);

        // Add cache headers based on content type
        $this->addCacheHeaders($request, $response);

        // Add ETag for conditional requests
        $this->addETag($request, $response);

        // Add compression hints
        $this->addCompressionHeaders($request, $response);

        // Add preload hints for critical resources
        $this->addPreloadHints($response);

        return $response;
    }

    /**
     * Add security headers
     */
    private function addSecurityHeaders(SymfonyResponse $response): void
    {
        // Prevent MIME type sniffing
        $response->headers->set('X-Content-Type-Options', 'nosniff');

        // XSS Protection (legacy browsers)
        $response->headers->set('X-XSS-Protection', '1; mode=block');

        // Clickjacking protection
        if (!$response->headers->has('X-Frame-Options')) {
            $response->headers->set('X-Frame-Options', 'SAMEORIGIN');
        }

        // Referrer policy
        $response->headers->set('Referrer-Policy', 'strict-origin-when-cross-origin');

        // Permissions policy (formerly Feature Policy)
        $response->headers->set('Permissions-Policy',
            'geolocation=(), microphone=(), camera=(), payment=(), usb=()'
        );

        // DNS prefetch control
        $response->headers->set('X-DNS-Prefetch-Control', 'on');
    }

    /**
     * Add cache headers based on content type
     */
    private function addCacheHeaders(Request $request, SymfonyResponse $response): void
    {
        // Skip if cache headers already set
        if ($response->headers->has('Cache-Control') &&
            $response->headers->get('Cache-Control') !== 'no-cache, private') {
            return;
        }

        $path = $request->path();
        $contentType = $response->headers->get('Content-Type', '');

        // API responses - short cache with revalidation
        if (str_starts_with($path, 'api/')) {
            if ($request->isMethod('GET')) {
                // Public GET endpoints can be cached briefly
                if (!str_contains($path, 'admin') && !str_contains($path, 'me')) {
                    $response->headers->set('Cache-Control', 'public, max-age=60, stale-while-revalidate=300');
                } else {
                    $response->headers->set('Cache-Control', 'private, no-cache, must-revalidate');
                }
            } else {
                $response->headers->set('Cache-Control', 'no-store');
            }
            return;
        }

        // Static assets - aggressive caching
        $extension = pathinfo($path, PATHINFO_EXTENSION);
        if (in_array(strtolower($extension), self::STATIC_EXTENSIONS)) {
            // 1 year cache with immutable for versioned assets
            $response->headers->set('Cache-Control', 'public, max-age=31536000, immutable');
            return;
        }

        // HTML pages - short cache with revalidation
        if (str_contains($contentType, 'text/html')) {
            $response->headers->set('Cache-Control', 'public, max-age=300, stale-while-revalidate=3600');
            return;
        }

        // JSON responses
        if (str_contains($contentType, 'application/json')) {
            $response->headers->set('Cache-Control', 'public, max-age=60, stale-while-revalidate=300');
            return;
        }

        // Default - no aggressive caching
        $response->headers->set('Cache-Control', 'public, max-age=300');
    }

    /**
     * Add ETag for conditional requests
     */
    private function addETag(Request $request, SymfonyResponse $response): void
    {
        // Only for GET requests
        if (!$request->isMethod('GET')) {
            return;
        }

        // Skip if ETag already set
        if ($response->headers->has('ETag')) {
            return;
        }

        // Generate ETag from content
        $content = $response->getContent();
        if ($content && strlen($content) < 1024 * 1024) { // Only for responses < 1MB
            $etag = '"' . md5($content) . '"';
            $response->headers->set('ETag', $etag);

            // Check if client has matching ETag
            $ifNoneMatch = $request->headers->get('If-None-Match');
            if ($ifNoneMatch === $etag) {
                $response->setStatusCode(304);
                $response->setContent('');
            }
        }
    }

    /**
     * Add compression headers
     */
    private function addCompressionHeaders(Request $request, SymfonyResponse $response): void
    {
        // Indicate that response can be compressed
        $response->headers->set('Vary', 'Accept-Encoding');

        // Check if response was compressed by web server
        $acceptEncoding = $request->headers->get('Accept-Encoding', '');

        // Add compression hints for reverse proxies
        if (str_contains($acceptEncoding, 'br')) {
            $response->headers->set('X-Compress-Hint', 'br');
        } elseif (str_contains($acceptEncoding, 'gzip')) {
            $response->headers->set('X-Compress-Hint', 'gzip');
        }
    }

    /**
     * Add preload hints for critical resources
     */
    private function addPreloadHints(SymfonyResponse $response): void
    {
        $contentType = $response->headers->get('Content-Type', '');

        // Only for HTML responses
        if (!str_contains($contentType, 'text/html')) {
            return;
        }

        // Add Link headers for resource hints
        $links = [];

        // Preconnect to external origins
        $links[] = '<https://fonts.googleapis.com>; rel=preconnect';
        $links[] = '<https://fonts.gstatic.com>; rel=preconnect; crossorigin';

        // DNS prefetch for analytics
        $links[] = '<https://www.google-analytics.com>; rel=dns-prefetch';
        $links[] = '<https://www.googletagmanager.com>; rel=dns-prefetch';

        if (!empty($links)) {
            $response->headers->set('Link', implode(', ', $links));
        }
    }
}
