<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Symfony\Component\HttpFoundation\Response;

/**
 * SecurityHeaders Middleware - Google L11 Principal Engineer Standard
 * ====================================================================
 *
 * Implements comprehensive security headers following OWASP guidelines:
 * - Content Security Policy (CSP) with nonce-based script loading
 * - XSS Protection
 * - Content Type Options
 * - Frame Options
 * - Referrer Policy
 * - Permissions Policy
 * - HSTS (for production)
 *
 * @version 2.0.0
 * @security Critical
 */
class SecurityHeaders
{
    /**
     * CSP nonce for this request
     */
    private string $nonce;

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Generate cryptographically secure nonce for this request
        $this->nonce = $this->generateNonce();

        // Store nonce in request for views to access
        $request->attributes->set('csp_nonce', $this->nonce);

        // Also store in app container for global access
        app()->instance('csp.nonce', $this->nonce);

        $response = $next($request);

        // Content Security Policy - Strict with nonce-based scripts
        $csp = $this->buildContentSecurityPolicy($request);
        $response->headers->set('Content-Security-Policy', $csp);

        // Also set Report-Only header in development for testing
        if (config('app.env') !== 'production') {
            $response->headers->set('Content-Security-Policy-Report-Only', $this->buildReportOnlyCSP());
        }

        // Prevent XSS attacks (deprecated but still useful for older browsers)
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

        // Cross-Origin policies (relaxed for API endpoints that need CORS)
        if (!$this->isApiEndpoint($request)) {
            $response->headers->set('Cross-Origin-Opener-Policy', 'same-origin');
            $response->headers->set('Cross-Origin-Resource-Policy', 'same-site');
        }

        return $response;
    }

    /**
     * Generate a cryptographically secure nonce
     */
    private function generateNonce(): string
    {
        return base64_encode(random_bytes(16));
    }

    /**
     * Get the current CSP nonce
     */
    public function getNonce(): string
    {
        return $this->nonce;
    }

    /**
     * Build Content Security Policy header with nonce-based scripts
     */
    private function buildContentSecurityPolicy(Request $request): string
    {
        $nonce = $this->nonce;

        // Trusted domains for scripts
        $trustedScriptDomains = [
            'https://www.google.com',
            'https://www.gstatic.com',
            'https://js.stripe.com',
            'https://www.googletagmanager.com',
            'https://www.google-analytics.com',
        ];

        // Trusted domains for styles
        $trustedStyleDomains = [
            'https://fonts.googleapis.com',
        ];

        $directives = [
            "default-src 'self'",
            // Use nonce for inline scripts, strict-dynamic for dynamically loaded scripts
            "script-src 'self' 'nonce-{$nonce}' 'strict-dynamic' " . implode(' ', $trustedScriptDomains),
            // Use nonce for inline styles
            "style-src 'self' 'nonce-{$nonce}' " . implode(' ', $trustedStyleDomains),
            "font-src 'self' https://fonts.gstatic.com data:",
            "img-src 'self' data: https: blob:",
            "connect-src 'self' https://api.stripe.com https://www.google-analytics.com wss: https:",
            "frame-src 'self' https://www.google.com https://js.stripe.com https://player.vimeo.com https://www.youtube.com",
            "frame-ancestors 'self'",
            "form-action 'self'",
            "base-uri 'self'",
            "object-src 'none'",
            "worker-src 'self' blob:",
            "manifest-src 'self'",
            "upgrade-insecure-requests",
        ];

        // Add report-uri for CSP violation reporting
        if (config('app.env') === 'production' && config('security.csp_report_uri')) {
            $directives[] = "report-uri " . config('security.csp_report_uri');
        }

        return implode('; ', $directives);
    }

    /**
     * Build Report-Only CSP for testing stricter policies
     */
    private function buildReportOnlyCSP(): string
    {
        $directives = [
            "default-src 'self'",
            "script-src 'self' 'nonce-{$this->nonce}'",
            "style-src 'self' 'nonce-{$this->nonce}'",
            "object-src 'none'",
            "base-uri 'self'",
        ];

        return implode('; ', $directives);
    }

    /**
     * Check if this is an API endpoint
     */
    private function isApiEndpoint(Request $request): bool
    {
        return str_starts_with($request->path(), 'api/');
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
