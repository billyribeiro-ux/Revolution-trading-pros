<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\RateLimiter;
use Symfony\Component\HttpFoundation\Response;

/**
 * Plan-Based Rate Limiter Middleware (ICT9+ Enterprise Grade)
 *
 * Applies rate limiting based on subscription plan:
 * - Different limits per plan tier
 * - Endpoint-specific limits
 * - Graceful limit headers
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class PlanRateLimiter
{
    /**
     * Default rate limits per plan (requests per minute)
     */
    private const PLAN_LIMITS = [
        'free' => [
            'default' => 60,
            'api' => 30,
            'export' => 5,
            'ai' => 10,
        ],
        'starter' => [
            'default' => 120,
            'api' => 100,
            'export' => 20,
            'ai' => 50,
        ],
        'professional' => [
            'default' => 300,
            'api' => 300,
            'export' => 50,
            'ai' => 100,
        ],
        'business' => [
            'default' => 600,
            'api' => 600,
            'export' => 100,
            'ai' => 200,
        ],
        'enterprise' => [
            'default' => 1000,
            'api' => 1000,
            'export' => 500,
            'ai' => 500,
        ],
    ];

    /**
     * Handle an incoming request
     *
     * @param Request $request
     * @param Closure $next
     * @param string $limiterType Type of rate limit to apply
     */
    public function handle(Request $request, Closure $next, string $limiterType = 'default'): Response
    {
        $user = $request->user();
        $key = $this->resolveRequestKey($request, $user);
        $limit = $this->getLimit($user, $limiterType);

        $executed = RateLimiter::attempt(
            $key,
            $limit,
            function () {},
            60 // Decay in seconds (1 minute)
        );

        if (!$executed) {
            return $this->buildRateLimitResponse($key, $limit);
        }

        $response = $next($request);

        return $this->addRateLimitHeaders($response, $key, $limit);
    }

    /**
     * Get rate limit for user's plan
     */
    private function getLimit($user, string $limiterType): int
    {
        $planSlug = $this->getUserPlanSlug($user);
        $planLimits = self::PLAN_LIMITS[$planSlug] ?? self::PLAN_LIMITS['free'];

        return $planLimits[$limiterType] ?? $planLimits['default'];
    }

    /**
     * Get user's plan slug
     */
    private function getUserPlanSlug($user): string
    {
        if (!$user) {
            return 'free';
        }

        $subscription = $user->activeSubscription();

        if (!$subscription || !$subscription->plan) {
            return 'free';
        }

        return $subscription->plan->slug ?? 'free';
    }

    /**
     * Resolve rate limiter key
     */
    private function resolveRequestKey(Request $request, $user): string
    {
        $identifier = $user?->id ?? $request->ip();
        $route = $request->route()?->getName() ?? $request->path();

        return "rate_limit:{$identifier}:{$route}";
    }

    /**
     * Build rate limit exceeded response
     */
    private function buildRateLimitResponse(string $key, int $limit): Response
    {
        $retryAfter = RateLimiter::availableIn($key);

        return response()->json([
            'error' => 'Too many requests',
            'code' => 'rate_limit_exceeded',
            'message' => 'You have exceeded the rate limit. Please try again later.',
            'retry_after' => $retryAfter,
        ], 429, [
            'Retry-After' => $retryAfter,
            'X-RateLimit-Limit' => $limit,
            'X-RateLimit-Remaining' => 0,
            'X-RateLimit-Reset' => now()->addSeconds($retryAfter)->timestamp,
        ]);
    }

    /**
     * Add rate limit headers to response
     */
    private function addRateLimitHeaders(Response $response, string $key, int $limit): Response
    {
        $remaining = RateLimiter::remaining($key, $limit);
        $resetAt = now()->addMinute()->timestamp;

        $response->headers->set('X-RateLimit-Limit', (string) $limit);
        $response->headers->set('X-RateLimit-Remaining', (string) max(0, $remaining));
        $response->headers->set('X-RateLimit-Reset', (string) $resetAt);

        return $response;
    }

    /**
     * Configure rate limiters for routes
     */
    public static function configureRateLimiters(): void
    {
        // API rate limiter
        RateLimiter::for('api', function (Request $request) {
            $user = $request->user();
            $limit = (new self())->getLimit($user, 'api');

            return \Illuminate\Cache\RateLimiting\Limit::perMinute($limit)
                ->by($user?->id ?: $request->ip());
        });

        // Export rate limiter
        RateLimiter::for('export', function (Request $request) {
            $user = $request->user();
            $limit = (new self())->getLimit($user, 'export');

            return \Illuminate\Cache\RateLimiting\Limit::perMinute($limit)
                ->by($user?->id ?: $request->ip());
        });

        // AI rate limiter
        RateLimiter::for('ai', function (Request $request) {
            $user = $request->user();
            $limit = (new self())->getLimit($user, 'ai');

            return \Illuminate\Cache\RateLimiting\Limit::perMinute($limit)
                ->by($user?->id ?: $request->ip());
        });
    }
}
