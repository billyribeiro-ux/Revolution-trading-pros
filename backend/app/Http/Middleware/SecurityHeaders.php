<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * SecurityHeaders Middleware - Google L11 Principal Engineer Standard
 * ====================================================================
 *
 * Implements comprehensive security headers following OWASP guidelines:
 * - Content Security Policy (CSP)
 * - XSS Protection
 * - Content Type Options
 * - Frame Options
 * - Referrer Policy
 * - Permissions Policy
 * - HSTS (for production)
 *
 * @version 1.0.0
 * @security Critical
 */
class SecurityHeaders
{
    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        $response = $next($request);

        // Content Security Policy - Strict but practical
        $csp = $this->buildContentSecurityPolicy();
        $response->headers->set('Content-Security-Policy', $csp);

        // Prevent XSS attacks
        $response->headers->set('X-XSS-Protection', '1; mode=block');

        // Prevent MIME type sniffing
        $response->headers->set('X-Content-Type-Options', 'nosniff');

        // Prevent clickjacking
        $response->headers->set('X-Frame-Options', 'SAMEORIGIN');

        // Control referrer information
        $response->headers->set('Referrer-Policy', 'strict-origin-when-cross-origin');

        // Permissions Policy (formerly Feature-Policy)
        $response->headers->set('Permissions-Policy', $this->buildPermissionsPolicy());

        // HSTS - Force HTTPS (only in production)
        if (config('app.env') === 'production') {
            $response->headers->set(
                'Strict-Transport-Security',
                'max-age=31536000; includeSubDomains; preload'
            );
        }

        // Prevent caching of sensitive data
        if ($this->isSensitiveEndpoint($request)) {
            $response->headers->set('Cache-Control', 'no-store, no-cache, must-revalidate, max-age=0');
            $response->headers->set('Pragma', 'no-cache');
            $response->headers->set('Expires', '0');
        }

        // Cross-Origin policies
        $response->headers->set('Cross-Origin-Opener-Policy', 'same-origin');
        $response->headers->set('Cross-Origin-Embedder-Policy', 'require-corp');
        $response->headers->set('Cross-Origin-Resource-Policy', 'same-origin');

        return $response;
    }

    /**
     * Build Content Security Policy header
     */
    private function buildContentSecurityPolicy(): string
    {
        $directives = [
            "default-src 'self'",
            "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://www.google.com https://www.gstatic.com https://js.stripe.com",
            "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
            "font-src 'self' https://fonts.gstatic.com data:",
            "img-src 'self' data: https: blob:",
            "connect-src 'self' https://api.stripe.com wss: https:",
            "frame-src 'self' https://www.google.com https://js.stripe.com https://player.vimeo.com https://www.youtube.com",
            "frame-ancestors 'self'",
            "form-action 'self'",
            "base-uri 'self'",
            "object-src 'none'",
            "upgrade-insecure-requests",
        ];

        return implode('; ', $directives);
    }

    /**
     * Build Permissions Policy header
     */
    private function buildPermissionsPolicy(): string
    {
        $policies = [
            'accelerometer=()',
            'autoplay=(self)',
            'camera=()',
            'cross-origin-isolated=()',
            'display-capture=()',
            'encrypted-media=(self)',
            'fullscreen=(self)',
            'geolocation=()',
            'gyroscope=()',
            'keyboard-map=()',
            'magnetometer=()',
            'microphone=()',
            'midi=()',
            'payment=(self)',
            'picture-in-picture=(self)',
            'publickey-credentials-get=()',
            'screen-wake-lock=()',
            'sync-xhr=(self)',
            'usb=()',
            'web-share=(self)',
            'xr-spatial-tracking=()',
        ];

        return implode(', ', $policies);
    }

    /**
     * Check if endpoint contains sensitive data
     */
    private function isSensitiveEndpoint(Request $request): bool
    {
        $sensitivePatterns = [
            '/api/login',
            '/api/register',
            '/api/me',
            '/api/auth',
            '/api/admin',
            '/api/cart/checkout',
            '/api/subscriptions',
        ];

        $path = $request->path();

        foreach ($sensitivePatterns as $pattern) {
            if (str_starts_with('/' . $path, $pattern)) {
                return true;
            }
        }

        return false;
    }
}
