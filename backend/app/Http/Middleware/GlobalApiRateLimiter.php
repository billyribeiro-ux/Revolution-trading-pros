<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Symfony\Component\HttpFoundation\Response;

/**
 * GlobalApiRateLimiter - Enterprise IP-Based Rate Limiting
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Provides global IP-based rate limiting for all API endpoints with:
 * - Configurable limits per endpoint type
 * - Sliding window algorithm
 * - Automatic IP reputation tracking
 * - Suspicious activity detection
 * - Whitelist/blacklist support
 * - Detailed metrics collection
 *
 * @version 1.0.0
 */
class GlobalApiRateLimiter
{
    /**
     * Rate limit configurations by endpoint type
     */
    private const RATE_LIMITS = [
        'default' => ['requests' => 100, 'window' => 60],      // 100 requests per minute
        'auth' => ['requests' => 10, 'window' => 60],          // 10 requests per minute for auth
        'newsletter' => ['requests' => 5, 'window' => 60],     // 5 requests per minute for newsletter
        'email' => ['requests' => 30, 'window' => 60],         // 30 requests per minute for email ops
        'webhook' => ['requests' => 200, 'window' => 60],      // 200 requests per minute for webhooks
        'admin' => ['requests' => 200, 'window' => 60],        // 200 requests per minute for admin
        'public' => ['requests' => 60, 'window' => 60],        // 60 requests per minute for public
    ];

    /**
     * Burst limits (short-term spike protection)
     */
    private const BURST_LIMITS = [
        'default' => ['requests' => 20, 'window' => 5],        // 20 requests per 5 seconds
        'auth' => ['requests' => 5, 'window' => 10],           // 5 requests per 10 seconds
        'newsletter' => ['requests' => 3, 'window' => 10],     // 3 requests per 10 seconds
    ];

    /**
     * Global daily limits per IP
     */
    private const DAILY_LIMIT = 10000;

    /**
     * Suspicious activity thresholds
     */
    private const SUSPICIOUS_THRESHOLD = 5;  // Failed attempts before flagging
    private const BAN_DURATION = 3600;       // 1 hour ban for suspicious IPs

    /**
     * Cache key prefixes
     */
    private const CACHE_PREFIX = 'rate_limit:';
    private const BURST_PREFIX = 'burst_limit:';
    private const DAILY_PREFIX = 'daily_limit:';
    private const SUSPICIOUS_PREFIX = 'suspicious:';
    private const BANNED_PREFIX = 'banned:';

    /**
     * Whitelisted IPs (internal services, monitoring, etc.)
     */
    private array $whitelist = [];

    /**
     * Handle an incoming request
     */
    public function handle(Request $request, Closure $next, string $type = 'default'): Response
    {
        $ip = $this->getClientIp($request);

        // Check whitelist
        if ($this->isWhitelisted($ip)) {
            return $next($request);
        }

        // Check if IP is banned
        if ($this->isBanned($ip)) {
            return $this->bannedResponse($ip);
        }

        // Get rate limit config
        $config = self::RATE_LIMITS[$type] ?? self::RATE_LIMITS['default'];
        $burstConfig = self::BURST_LIMITS[$type] ?? self::BURST_LIMITS['default'];

        // Check burst limit first (short-term)
        $burstResult = $this->checkBurstLimit($ip, $type, $burstConfig);
        if (!$burstResult['allowed']) {
            $this->recordSuspiciousActivity($ip, 'burst_exceeded');
            return $this->rateLimitResponse($burstResult, 'burst');
        }

        // Check standard rate limit
        $rateResult = $this->checkRateLimit($ip, $type, $config);
        if (!$rateResult['allowed']) {
            $this->recordSuspiciousActivity($ip, 'rate_exceeded');
            return $this->rateLimitResponse($rateResult, 'standard');
        }

        // Check daily limit
        $dailyResult = $this->checkDailyLimit($ip);
        if (!$dailyResult['allowed']) {
            $this->recordSuspiciousActivity($ip, 'daily_exceeded');
            return $this->rateLimitResponse($dailyResult, 'daily');
        }

        // Process request
        $response = $next($request);

        // Add rate limit headers
        return $this->addRateLimitHeaders($response, $rateResult, $config);
    }

    /**
     * Get the real client IP address
     */
    private function getClientIp(Request $request): string
    {
        // Check for forwarded IPs (behind proxy/load balancer)
        $forwardedFor = $request->header('X-Forwarded-For');
        if ($forwardedFor) {
            $ips = explode(',', $forwardedFor);
            return trim($ips[0]);
        }

        $realIp = $request->header('X-Real-IP');
        if ($realIp) {
            return $realIp;
        }

        return $request->ip() ?? '0.0.0.0';
    }

    /**
     * Check if IP is whitelisted
     */
    private function isWhitelisted(string $ip): bool
    {
        // Load whitelist from config/cache
        $this->whitelist = Cache::remember('rate_limit:whitelist', 3600, function () {
            return config('rate_limiting.whitelist', [
                '127.0.0.1',
                '::1',
            ]);
        });

        return in_array($ip, $this->whitelist, true);
    }

    /**
     * Check if IP is banned
     */
    private function isBanned(string $ip): bool
    {
        return Cache::has(self::BANNED_PREFIX . $ip);
    }

    /**
     * Check burst rate limit (sliding window)
     */
    private function checkBurstLimit(string $ip, string $type, array $config): array
    {
        $key = self::BURST_PREFIX . $type . ':' . $ip;
        return $this->slidingWindowCheck($key, $config['requests'], $config['window']);
    }

    /**
     * Check standard rate limit (sliding window)
     */
    private function checkRateLimit(string $ip, string $type, array $config): array
    {
        $key = self::CACHE_PREFIX . $type . ':' . $ip;
        return $this->slidingWindowCheck($key, $config['requests'], $config['window']);
    }

    /**
     * Check daily limit
     */
    private function checkDailyLimit(string $ip): array
    {
        $key = self::DAILY_PREFIX . $ip . ':' . date('Y-m-d');
        $count = Cache::get($key, 0);

        if ($count >= self::DAILY_LIMIT) {
            return [
                'allowed' => false,
                'remaining' => 0,
                'reset' => strtotime('tomorrow'),
                'limit' => self::DAILY_LIMIT,
            ];
        }

        Cache::put($key, $count + 1, now()->endOfDay());

        return [
            'allowed' => true,
            'remaining' => self::DAILY_LIMIT - $count - 1,
            'reset' => strtotime('tomorrow'),
            'limit' => self::DAILY_LIMIT,
        ];
    }

    /**
     * Sliding window rate limit algorithm
     */
    private function slidingWindowCheck(string $key, int $limit, int $window): array
    {
        $now = microtime(true);
        $windowStart = $now - $window;

        // Get existing timestamps
        $timestamps = Cache::get($key, []);

        // Remove expired timestamps
        $timestamps = array_filter($timestamps, fn($ts) => $ts > $windowStart);

        // Check if limit exceeded
        if (count($timestamps) >= $limit) {
            $oldestTimestamp = min($timestamps);
            $retryAfter = (int) ceil($oldestTimestamp + $window - $now);

            return [
                'allowed' => false,
                'remaining' => 0,
                'reset' => (int) ($oldestTimestamp + $window),
                'retry_after' => max(1, $retryAfter),
                'limit' => $limit,
            ];
        }

        // Add current request timestamp
        $timestamps[] = $now;
        Cache::put($key, $timestamps, $window);

        return [
            'allowed' => true,
            'remaining' => $limit - count($timestamps),
            'reset' => (int) ($now + $window),
            'limit' => $limit,
        ];
    }

    /**
     * Record suspicious activity
     */
    private function recordSuspiciousActivity(string $ip, string $reason): void
    {
        $key = self::SUSPICIOUS_PREFIX . $ip;
        $count = Cache::increment($key);

        if ($count === 1) {
            Cache::put($key, 1, 3600); // 1 hour window
        }

        Log::warning('[GlobalApiRateLimiter] Suspicious activity detected', [
            'ip' => $ip,
            'reason' => $reason,
            'count' => $count,
        ]);

        // Ban IP if threshold exceeded
        if ($count >= self::SUSPICIOUS_THRESHOLD) {
            $this->banIp($ip, $reason);
        }
    }

    /**
     * Ban an IP address
     */
    private function banIp(string $ip, string $reason): void
    {
        Cache::put(self::BANNED_PREFIX . $ip, [
            'reason' => $reason,
            'banned_at' => now()->toIso8601String(),
        ], self::BAN_DURATION);

        Log::warning('[GlobalApiRateLimiter] IP banned', [
            'ip' => $ip,
            'reason' => $reason,
            'duration' => self::BAN_DURATION,
        ]);
    }

    /**
     * Generate rate limit exceeded response
     */
    private function rateLimitResponse(array $result, string $type): Response
    {
        $message = match ($type) {
            'burst' => 'Too many requests in a short period. Please slow down.',
            'daily' => 'Daily request limit exceeded. Please try again tomorrow.',
            default => 'Rate limit exceeded. Please try again later.',
        };

        return response()->json([
            'error' => 'rate_limit_exceeded',
            'message' => $message,
            'retry_after' => $result['retry_after'] ?? 60,
        ], Response::HTTP_TOO_MANY_REQUESTS, [
            'Retry-After' => $result['retry_after'] ?? 60,
            'X-RateLimit-Limit' => $result['limit'],
            'X-RateLimit-Remaining' => 0,
            'X-RateLimit-Reset' => $result['reset'],
        ]);
    }

    /**
     * Generate banned IP response
     */
    private function bannedResponse(string $ip): Response
    {
        $banData = Cache::get(self::BANNED_PREFIX . $ip);

        return response()->json([
            'error' => 'ip_banned',
            'message' => 'Your IP has been temporarily banned due to suspicious activity.',
            'banned_at' => $banData['banned_at'] ?? null,
        ], Response::HTTP_FORBIDDEN);
    }

    /**
     * Add rate limit headers to response
     */
    private function addRateLimitHeaders(Response $response, array $result, array $config): Response
    {
        $response->headers->set('X-RateLimit-Limit', (string) $config['requests']);
        $response->headers->set('X-RateLimit-Remaining', (string) $result['remaining']);
        $response->headers->set('X-RateLimit-Reset', (string) $result['reset']);
        $response->headers->set('X-RateLimit-Window', (string) $config['window']);

        return $response;
    }

    /**
     * Get rate limit statistics for an IP
     */
    public static function getStatistics(string $ip): array
    {
        $stats = [];

        foreach (array_keys(self::RATE_LIMITS) as $type) {
            $key = self::CACHE_PREFIX . $type . ':' . $ip;
            $timestamps = Cache::get($key, []);
            $stats[$type] = [
                'requests_in_window' => count($timestamps),
                'limit' => self::RATE_LIMITS[$type]['requests'],
            ];
        }

        $dailyKey = self::DAILY_PREFIX . $ip . ':' . date('Y-m-d');
        $stats['daily'] = [
            'requests_today' => Cache::get($dailyKey, 0),
            'limit' => self::DAILY_LIMIT,
        ];

        $stats['suspicious_count'] = Cache::get(self::SUSPICIOUS_PREFIX . $ip, 0);
        $stats['is_banned'] = Cache::has(self::BANNED_PREFIX . $ip);

        return $stats;
    }

    /**
     * Manually unban an IP
     */
    public static function unbanIp(string $ip): bool
    {
        Cache::forget(self::BANNED_PREFIX . $ip);
        Cache::forget(self::SUSPICIOUS_PREFIX . $ip);

        Log::info('[GlobalApiRateLimiter] IP manually unbanned', ['ip' => $ip]);

        return true;
    }

    /**
     * Add IP to whitelist
     */
    public static function addToWhitelist(string $ip): bool
    {
        $whitelist = Cache::get('rate_limit:whitelist', []);
        if (!in_array($ip, $whitelist, true)) {
            $whitelist[] = $ip;
            Cache::put('rate_limit:whitelist', $whitelist, 86400 * 365);
        }

        return true;
    }

    /**
     * Remove IP from whitelist
     */
    public static function removeFromWhitelist(string $ip): bool
    {
        $whitelist = Cache::get('rate_limit:whitelist', []);
        $whitelist = array_filter($whitelist, fn($i) => $i !== $ip);
        Cache::put('rate_limit:whitelist', array_values($whitelist), 86400 * 365);

        return true;
    }
}
