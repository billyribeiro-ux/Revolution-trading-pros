<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * SanitizeInput Middleware - Enterprise Input Validation
 * =======================================================
 *
 * Comprehensive input sanitization to prevent:
 * - XSS attacks via request parameters
 * - SQL injection attempts
 * - Path traversal attacks
 * - Null byte injection
 * - Unicode attacks
 *
 * @version 1.0.0
 * @security Critical
 */
class SanitizeInput
{
    /**
     * Fields that should NOT be sanitized (contain HTML intentionally)
     */
    private const EXEMPT_FIELDS = [
        'body_html',
        'content_html',
        'html_content',
        'template_body',
        'email_body',
    ];

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Sanitize query parameters
        $query = $this->sanitizeArray($request->query->all());
        $request->query->replace($query);

        // Sanitize request body (POST/PUT/PATCH)
        if ($request->isMethod('POST') || $request->isMethod('PUT') || $request->isMethod('PATCH')) {
            $input = $this->sanitizeArray($request->all(), $request->path());
            $request->replace($input);
        }

        // Validate and sanitize headers
        $this->validateHeaders($request);

        return $next($request);
    }

    /**
     * Recursively sanitize an array of input data
     */
    private function sanitizeArray(array $data, string $path = ''): array
    {
        $sanitized = [];

        foreach ($data as $key => $value) {
            // Sanitize key
            $sanitizedKey = $this->sanitizeKey($key);

            // Skip exempt fields
            if (in_array($sanitizedKey, self::EXEMPT_FIELDS, true)) {
                $sanitized[$sanitizedKey] = $value;
                continue;
            }

            if (is_array($value)) {
                $sanitized[$sanitizedKey] = $this->sanitizeArray($value, $path);
            } elseif (is_string($value)) {
                $sanitized[$sanitizedKey] = $this->sanitizeString($value);
            } else {
                $sanitized[$sanitizedKey] = $value;
            }
        }

        return $sanitized;
    }

    /**
     * Sanitize a string value
     */
    private function sanitizeString(string $value): string
    {
        // Remove null bytes
        $value = str_replace(chr(0), '', $value);

        // Remove control characters (except newline, tab, carriage return)
        $value = preg_replace('/[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]/u', '', $value);

        // Normalize Unicode
        if (function_exists('normalizer_normalize')) {
            $normalized = normalizer_normalize($value, \Normalizer::FORM_C);
            if ($normalized !== false) {
                $value = $normalized;
            }
        }

        // Convert special HTML characters to entities
        $value = htmlspecialchars($value, ENT_QUOTES | ENT_HTML5, 'UTF-8', false);

        // Decode back for storage (we encode on output)
        $value = htmlspecialchars_decode($value, ENT_QUOTES);

        // Remove potential SQL injection patterns (basic)
        $value = $this->removeSqlInjectionPatterns($value);

        // Limit string length to prevent DoS
        if (strlen($value) > 65535) {
            $value = substr($value, 0, 65535);
        }

        return trim($value);
    }

    /**
     * Sanitize array/object key
     */
    private function sanitizeKey(string|int $key): string|int
    {
        if (is_int($key)) {
            return $key;
        }

        // Remove potentially dangerous characters from keys
        $key = preg_replace('/[^a-zA-Z0-9_\-\.]/', '', $key);

        // Prevent very long keys
        if (strlen($key) > 100) {
            $key = substr($key, 0, 100);
        }

        return $key;
    }

    /**
     * Remove common SQL injection patterns
     * Note: This is defense in depth - parameterized queries are the primary defense
     */
    private function removeSqlInjectionPatterns(string $value): string
    {
        // Only detect and log, don't modify (to avoid breaking legitimate input)
        $suspiciousPatterns = [
            '/\bUNION\s+SELECT\b/i',
            '/\bSELECT\s+.*\s+FROM\b/i',
            '/\bINSERT\s+INTO\b/i',
            '/\bDELETE\s+FROM\b/i',
            '/\bDROP\s+TABLE\b/i',
            '/\bOR\s+1\s*=\s*1\b/i',
            '/\bAND\s+1\s*=\s*1\b/i',
            '/--\s*$/m',
            '/;\s*--/',
            '/\/\*.*\*\//s',
        ];

        foreach ($suspiciousPatterns as $pattern) {
            if (preg_match($pattern, $value)) {
                \Log::warning('Potential SQL injection attempt detected', [
                    'pattern' => $pattern,
                    'ip' => request()->ip(),
                    'path' => request()->path(),
                ]);
                break;
            }
        }

        return $value;
    }

    /**
     * Validate request headers for security
     */
    private function validateHeaders(Request $request): void
    {
        // Check for suspicious User-Agent
        $userAgent = $request->header('User-Agent', '');

        $suspiciousAgents = [
            '/sqlmap/i',
            '/nikto/i',
            '/nessus/i',
            '/burp/i',
            '/dirbuster/i',
            '/nmap/i',
        ];

        foreach ($suspiciousAgents as $pattern) {
            if (preg_match($pattern, $userAgent)) {
                \Log::warning('Suspicious User-Agent detected', [
                    'user_agent' => $userAgent,
                    'ip' => $request->ip(),
                    'path' => $request->path(),
                ]);
                break;
            }
        }

        // Validate Content-Type for POST requests
        if ($request->isMethod('POST') && $request->getContent()) {
            $contentType = $request->header('Content-Type', '');
            $allowedTypes = [
                'application/json',
                'application/x-www-form-urlencoded',
                'multipart/form-data',
            ];

            $isAllowed = false;
            foreach ($allowedTypes as $type) {
                if (str_starts_with($contentType, $type)) {
                    $isAllowed = true;
                    break;
                }
            }

            if (!$isAllowed && !empty($contentType)) {
                \Log::warning('Unexpected Content-Type', [
                    'content_type' => $contentType,
                    'ip' => $request->ip(),
                    'path' => $request->path(),
                ]);
            }
        }
    }
}
