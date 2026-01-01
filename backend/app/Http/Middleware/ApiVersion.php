<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Symfony\Component\HttpFoundation\Response;

/**
 * API Version Negotiation Middleware
 *
 * L11 Enterprise Pattern - Supports multiple versioning strategies:
 * 1. Accept header: Accept: application/vnd.api+json; version=1
 * 2. Custom header: X-API-Version: 1
 * 3. URL prefix: /api/v1/resource (handled by route groups)
 *
 * Features:
 * - Version sunset warnings
 * - Version negotiation
 * - Backward compatibility
 * - Metrics tracking
 */
class ApiVersion
{
    /**
     * Supported API versions
     */
    private const SUPPORTED_VERSIONS = ['1', '2'];

    /**
     * Current stable version
     */
    private const CURRENT_VERSION = '1';

    /**
     * Deprecated versions (will return warning header)
     */
    private const DEPRECATED_VERSIONS = [];

    /**
     * Sunset dates for deprecated versions (ISO 8601)
     */
    private const SUNSET_DATES = [
        // '1' => '2025-06-01',
    ];

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next, ?string $routeVersion = null): Response
    {
        // Determine requested API version
        $requestedVersion = $this->resolveVersion($request, $routeVersion);

        // Validate version
        if (!$this->isVersionSupported($requestedVersion)) {
            return response()->json([
                'error' => 'Unsupported API version',
                'message' => "API version '{$requestedVersion}' is not supported",
                'supported_versions' => self::SUPPORTED_VERSIONS,
                'current_version' => self::CURRENT_VERSION,
            ], 400);
        }

        // Store version in request for controllers
        $request->attributes->set('api_version', $requestedVersion);
        $request->attributes->set('api_version_int', (int) $requestedVersion);

        // Process request
        $response = $next($request);

        // Add version headers to response
        $response->headers->set('X-API-Version', $requestedVersion);
        $response->headers->set('X-API-Version-Current', self::CURRENT_VERSION);
        $response->headers->set('X-API-Supported-Versions', implode(',', self::SUPPORTED_VERSIONS));

        // Add deprecation warning if applicable
        if ($this->isVersionDeprecated($requestedVersion)) {
            $sunsetDate = self::SUNSET_DATES[$requestedVersion] ?? null;

            $response->headers->set('X-API-Deprecated', 'true');
            $response->headers->set('X-API-Deprecation-Warning',
                "API version {$requestedVersion} is deprecated. Please migrate to version " . self::CURRENT_VERSION
            );

            if ($sunsetDate) {
                $response->headers->set('Sunset', $sunsetDate);
                $response->headers->set('X-API-Sunset-Date', $sunsetDate);
            }

            // Log deprecation usage for monitoring
            Log::info('Deprecated API version used', [
                'version' => $requestedVersion,
                'path' => $request->path(),
                'method' => $request->method(),
                'user_id' => $request->user()?->id,
                'ip' => $request->ip(),
            ]);
        }

        return $response;
    }

    /**
     * Resolve API version from request
     */
    private function resolveVersion(Request $request, ?string $routeVersion): string
    {
        // Priority 1: Route parameter version (from URL like /api/v2/resource)
        if ($routeVersion !== null) {
            return ltrim($routeVersion, 'v');
        }

        // Priority 2: Custom X-API-Version header
        $headerVersion = $request->header('X-API-Version');
        if ($headerVersion !== null) {
            return ltrim($headerVersion, 'v');
        }

        // Priority 3: Accept header with version parameter
        // Format: application/vnd.api+json; version=1
        $accept = $request->header('Accept', '');
        if (preg_match('/version\s*=\s*(\d+)/', $accept, $matches)) {
            return $matches[1];
        }

        // Priority 4: Query parameter (for debugging/testing only)
        $queryVersion = $request->query('api_version');
        if ($queryVersion !== null && app()->environment('local', 'testing')) {
            return ltrim($queryVersion, 'v');
        }

        // Default to current version
        return self::CURRENT_VERSION;
    }

    /**
     * Check if version is supported
     */
    private function isVersionSupported(string $version): bool
    {
        return in_array($version, self::SUPPORTED_VERSIONS, true);
    }

    /**
     * Check if version is deprecated
     */
    private function isVersionDeprecated(string $version): bool
    {
        return in_array($version, self::DEPRECATED_VERSIONS, true);
    }

    /**
     * Get controller namespace for version
     */
    public static function getControllerNamespace(string $version): string
    {
        return "App\\Http\\Controllers\\Api\\V{$version}";
    }

    /**
     * Get current API version
     */
    public static function getCurrentVersion(): string
    {
        return self::CURRENT_VERSION;
    }

    /**
     * Get all supported versions
     */
    public static function getSupportedVersions(): array
    {
        return self::SUPPORTED_VERSIONS;
    }
}
