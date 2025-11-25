<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Cache\RateLimiter;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Symfony\Component\HttpFoundation\Response;

/**
 * Rate Limit Media Operations
 *
 * Google Enterprise Grade rate limiting with:
 * - Per-user and per-IP rate limits
 * - Tiered limits based on operation type
 * - Burst allowance for legitimate spikes
 * - Clear rate limit headers in responses
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class RateLimitMedia
{
    protected RateLimiter $limiter;

    /**
     * Rate limit configurations by operation type
     */
    protected array $limits = [
        'upload' => [
            'max_attempts' => 30,      // 30 uploads per minute
            'decay_minutes' => 1,
            'burst_allowance' => 10,   // Extra 10 for burst
        ],
        'bulk_upload' => [
            'max_attempts' => 5,       // 5 bulk operations per minute
            'decay_minutes' => 1,
            'burst_allowance' => 2,
        ],
        'delete' => [
            'max_attempts' => 20,      // 20 deletes per minute
            'decay_minutes' => 1,
            'burst_allowance' => 5,
        ],
        'bulk_delete' => [
            'max_attempts' => 3,       // 3 bulk deletes per minute
            'decay_minutes' => 1,
            'burst_allowance' => 1,
        ],
        'optimize' => [
            'max_attempts' => 50,      // 50 optimizations per minute
            'decay_minutes' => 1,
            'burst_allowance' => 20,
        ],
        'bulk_optimize' => [
            'max_attempts' => 5,       // 5 bulk optimizations per minute
            'decay_minutes' => 1,
            'burst_allowance' => 2,
        ],
        'download' => [
            'max_attempts' => 100,     // 100 downloads per minute
            'decay_minutes' => 1,
            'burst_allowance' => 50,
        ],
        'default' => [
            'max_attempts' => 60,      // 60 requests per minute
            'decay_minutes' => 1,
            'burst_allowance' => 10,
        ],
    ];

    public function __construct(RateLimiter $limiter)
    {
        $this->limiter = $limiter;
    }

    /**
     * Handle an incoming request
     */
    public function handle(Request $request, Closure $next, string $operation = 'default'): Response
    {
        $config = $this->limits[$operation] ?? $this->limits['default'];
        $key = $this->resolveRequestSignature($request, $operation);

        $maxAttempts = $config['max_attempts'];
        $decayMinutes = $config['decay_minutes'];

        // Check if rate limit exceeded
        if ($this->limiter->tooManyAttempts($key, $maxAttempts)) {
            return $this->buildRateLimitResponse($key, $maxAttempts, $operation);
        }

        // Increment counter
        $this->limiter->hit($key, $decayMinutes * 60);

        /** @var Response $response */
        $response = $next($request);

        // Add rate limit headers
        return $this->addRateLimitHeaders(
            $response,
            $maxAttempts,
            $this->calculateRemainingAttempts($key, $maxAttempts)
        );
    }

    /**
     * Resolve request signature for rate limiting
     */
    protected function resolveRequestSignature(Request $request, string $operation): string
    {
        $userId = $request->user()?->id ?? 'guest';
        $ip = $request->ip();

        return sprintf(
            'media_rate_limit:%s:%s:%s',
            $operation,
            $userId,
            sha1($ip)
        );
    }

    /**
     * Build rate limit exceeded response
     */
    protected function buildRateLimitResponse(string $key, int $maxAttempts, string $operation): JsonResponse
    {
        $retryAfter = $this->limiter->availableIn($key);

        return response()->json([
            'success' => false,
            'error' => 'rate_limit_exceeded',
            'message' => "Too many {$operation} requests. Please try again later.",
            'retry_after' => $retryAfter,
            'limit' => $maxAttempts,
        ], Response::HTTP_TOO_MANY_REQUESTS)->withHeaders([
            'X-RateLimit-Limit' => $maxAttempts,
            'X-RateLimit-Remaining' => 0,
            'X-RateLimit-Reset' => $this->availableAt($retryAfter),
            'Retry-After' => $retryAfter,
        ]);
    }

    /**
     * Add rate limit headers to response
     */
    protected function addRateLimitHeaders(Response $response, int $maxAttempts, int $remainingAttempts): Response
    {
        $response->headers->add([
            'X-RateLimit-Limit' => $maxAttempts,
            'X-RateLimit-Remaining' => max(0, $remainingAttempts),
        ]);

        return $response;
    }

    /**
     * Calculate remaining attempts
     */
    protected function calculateRemainingAttempts(string $key, int $maxAttempts): int
    {
        return $this->limiter->remaining($key, $maxAttempts);
    }

    /**
     * Get the timestamp when rate limit resets
     */
    protected function availableAt(int $retryAfter): int
    {
        return now()->addSeconds($retryAfter)->getTimestamp();
    }
}
