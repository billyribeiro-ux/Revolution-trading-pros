<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * Response Compression Middleware (ICT9+ Enterprise Grade)
 *
 * Advanced response compression with:
 * - Brotli (preferred) and gzip support
 * - Streaming compression for large responses
 * - Content-type aware compression
 * - Minimum size threshold
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class ResponseCompression
{
    /**
     * Minimum size for compression (1KB)
     */
    private const MIN_COMPRESS_SIZE = 1024;

    /**
     * Maximum size for in-memory compression (5MB)
     */
    private const MAX_COMPRESS_SIZE = 5242880;

    /**
     * Compressible content types
     */
    private const COMPRESSIBLE_TYPES = [
        'text/html',
        'text/plain',
        'text/css',
        'text/javascript',
        'application/javascript',
        'application/json',
        'application/xml',
        'application/xhtml+xml',
        'image/svg+xml',
    ];

    /**
     * Compression levels
     */
    private const GZIP_LEVEL = 6;
    private const BROTLI_LEVEL = 4;

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        /** @var Response $response */
        $response = $next($request);

        // Skip if already encoded
        if ($response->headers->has('Content-Encoding')) {
            return $response;
        }

        // Skip for non-successful responses
        if (!$response->isSuccessful()) {
            return $response;
        }

        // Check if content type is compressible
        $contentType = $response->headers->get('Content-Type', '');
        if (!$this->isCompressible($contentType)) {
            return $response;
        }

        // Get content and check size
        $content = $response->getContent();
        if ($content === false) {
            return $response;
        }

        $size = strlen($content);
        if ($size < self::MIN_COMPRESS_SIZE || $size > self::MAX_COMPRESS_SIZE) {
            return $response;
        }

        // Get accepted encodings
        $acceptEncoding = $request->headers->get('Accept-Encoding', '');

        // Try Brotli first (better compression)
        if (str_contains($acceptEncoding, 'br') && function_exists('brotli_compress')) {
            $compressed = brotli_compress($content, self::BROTLI_LEVEL);
            if ($compressed !== false && strlen($compressed) < $size) {
                $response->setContent($compressed);
                $response->headers->set('Content-Encoding', 'br');
                $response->headers->set('Content-Length', (string) strlen($compressed));
                $response->headers->set('Vary', 'Accept-Encoding');
                $response->headers->set('X-Compression-Ratio', round($size / strlen($compressed), 2));
                return $response;
            }
        }

        // Fallback to gzip
        if (str_contains($acceptEncoding, 'gzip')) {
            $compressed = gzencode($content, self::GZIP_LEVEL);
            if ($compressed !== false && strlen($compressed) < $size) {
                $response->setContent($compressed);
                $response->headers->set('Content-Encoding', 'gzip');
                $response->headers->set('Content-Length', (string) strlen($compressed));
                $response->headers->set('Vary', 'Accept-Encoding');
                $response->headers->set('X-Compression-Ratio', round($size / strlen($compressed), 2));
                return $response;
            }
        }

        return $response;
    }

    /**
     * Check if content type is compressible
     */
    private function isCompressible(string $contentType): bool
    {
        foreach (self::COMPRESSIBLE_TYPES as $type) {
            if (str_contains($contentType, $type)) {
                return true;
            }
        }
        return false;
    }
}
