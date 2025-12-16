<?php

declare(strict_types=1);

namespace App\Support\CircuitBreaker;

use Closure;
use Illuminate\Contracts\Cache\Repository as CacheRepository;
use Illuminate\Support\Facades\Log;
use Throwable;

/**
 * Circuit Breaker Implementation
 *
 * Implements the circuit breaker pattern for resilient external service calls.
 * Prevents cascading failures by failing fast when services are unavailable.
 *
 * States:
 * - CLOSED: Normal operation, requests pass through
 * - OPEN: Circuit tripped, requests fail immediately
 * - HALF_OPEN: Testing if service recovered
 *
 * @version 1.0.0
 */
class CircuitBreaker
{
    /**
     * Circuit states.
     */
    public const STATE_CLOSED = 'closed';
    public const STATE_OPEN = 'open';
    public const STATE_HALF_OPEN = 'half_open';

    /**
     * Create a new circuit breaker instance.
     *
     * @param CacheRepository $cache Cache for state persistence
     * @param string $name Unique name for this circuit
     * @param int $failureThreshold Failures before opening circuit
     * @param int $successThreshold Successes before closing circuit
     * @param int $timeout Seconds before attempting recovery
     */
    public function __construct(
        private readonly CacheRepository $cache,
        private readonly string $name,
        private readonly int $failureThreshold = 5,
        private readonly int $successThreshold = 2,
        private readonly int $timeout = 60,
    ) {}

    /**
     * Execute a callable through the circuit breaker.
     *
     * @template T
     * @param Closure(): T $callback The operation to execute
     * @param Closure(): T|null $fallback Optional fallback when circuit is open
     * @return T The callback result
     * @throws CircuitBreakerOpenException When circuit is open and no fallback
     * @throws Throwable When callback fails and circuit allows retries
     */
    public function call(Closure $callback, ?Closure $fallback = null): mixed
    {
        $state = $this->getState();

        // If circuit is open, check if timeout has passed
        if ($state === self::STATE_OPEN) {
            if (!$this->shouldAttemptReset()) {
                Log::warning("Circuit breaker [{$this->name}] is OPEN, rejecting request", [
                    'circuit' => $this->name,
                    'state' => $state,
                    'retry_after' => $this->getRetryAfter(),
                ]);

                if ($fallback !== null) {
                    return $fallback();
                }

                throw new CircuitBreakerOpenException(
                    $this->name,
                    $this->getRetryAfter(),
                );
            }

            // Transition to half-open
            $this->transitionTo(self::STATE_HALF_OPEN);
            $state = self::STATE_HALF_OPEN;
        }

        try {
            $result = $callback();
            $this->recordSuccess();

            return $result;
        } catch (Throwable $e) {
            $this->recordFailure($e);

            throw $e;
        }
    }

    /**
     * Get current circuit state.
     */
    public function getState(): string
    {
        return $this->cache->get(
            $this->cacheKey('state'),
            self::STATE_CLOSED,
        );
    }

    /**
     * Get failure count.
     */
    public function getFailureCount(): int
    {
        return (int) $this->cache->get($this->cacheKey('failures'), 0);
    }

    /**
     * Get success count (for half-open state).
     */
    public function getSuccessCount(): int
    {
        return (int) $this->cache->get($this->cacheKey('successes'), 0);
    }

    /**
     * Get seconds until retry is allowed.
     */
    public function getRetryAfter(): int
    {
        $openedAt = $this->cache->get($this->cacheKey('opened_at'));

        if (!$openedAt) {
            return 0;
        }

        $retryAt = $openedAt + $this->timeout;
        $remaining = $retryAt - time();

        return max(0, $remaining);
    }

    /**
     * Force circuit to open state.
     */
    public function trip(): void
    {
        $this->transitionTo(self::STATE_OPEN);
    }

    /**
     * Force circuit to closed state (reset).
     */
    public function reset(): void
    {
        $this->transitionTo(self::STATE_CLOSED);
        $this->cache->forget($this->cacheKey('failures'));
        $this->cache->forget($this->cacheKey('successes'));
        $this->cache->forget($this->cacheKey('opened_at'));
        $this->cache->forget($this->cacheKey('last_failure'));

        Log::info("Circuit breaker [{$this->name}] reset manually", [
            'circuit' => $this->name,
        ]);
    }

    /**
     * Get circuit statistics.
     *
     * @return array<string, mixed>
     */
    public function getStats(): array
    {
        return [
            'name' => $this->name,
            'state' => $this->getState(),
            'failures' => $this->getFailureCount(),
            'successes' => $this->getSuccessCount(),
            'failure_threshold' => $this->failureThreshold,
            'success_threshold' => $this->successThreshold,
            'timeout' => $this->timeout,
            'retry_after' => $this->getRetryAfter(),
            'last_failure' => $this->cache->get($this->cacheKey('last_failure')),
        ];
    }

    /**
     * Check if circuit is available for requests.
     */
    public function isAvailable(): bool
    {
        $state = $this->getState();

        if ($state === self::STATE_CLOSED) {
            return true;
        }

        if ($state === self::STATE_HALF_OPEN) {
            return true;
        }

        return $this->shouldAttemptReset();
    }

    /**
     * Record a successful operation.
     */
    private function recordSuccess(): void
    {
        $state = $this->getState();

        if ($state === self::STATE_HALF_OPEN) {
            $successes = $this->cache->increment($this->cacheKey('successes'));

            if ($successes >= $this->successThreshold) {
                $this->transitionTo(self::STATE_CLOSED);
                $this->cache->forget($this->cacheKey('failures'));
                $this->cache->forget($this->cacheKey('successes'));
                $this->cache->forget($this->cacheKey('opened_at'));

                Log::info("Circuit breaker [{$this->name}] CLOSED after recovery", [
                    'circuit' => $this->name,
                    'successes' => $successes,
                ]);
            }
        } elseif ($state === self::STATE_CLOSED) {
            // Reset failure count on success
            $this->cache->forget($this->cacheKey('failures'));
        }
    }

    /**
     * Record a failed operation.
     */
    private function recordFailure(Throwable $e): void
    {
        $state = $this->getState();

        // Store last failure info
        $this->cache->put(
            $this->cacheKey('last_failure'),
            [
                'message' => $e->getMessage(),
                'class' => get_class($e),
                'time' => now()->toIso8601String(),
            ],
            now()->addHours(24),
        );

        if ($state === self::STATE_HALF_OPEN) {
            // Any failure in half-open state trips the circuit again
            $this->transitionTo(self::STATE_OPEN);
            $this->cache->forget($this->cacheKey('successes'));

            Log::warning("Circuit breaker [{$this->name}] re-opened after half-open failure", [
                'circuit' => $this->name,
                'error' => $e->getMessage(),
            ]);
        } elseif ($state === self::STATE_CLOSED) {
            $failures = $this->cache->increment($this->cacheKey('failures'));

            if ($failures >= $this->failureThreshold) {
                $this->transitionTo(self::STATE_OPEN);

                Log::error("Circuit breaker [{$this->name}] OPENED after threshold reached", [
                    'circuit' => $this->name,
                    'failures' => $failures,
                    'threshold' => $this->failureThreshold,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Check if we should attempt to reset the circuit.
     */
    private function shouldAttemptReset(): bool
    {
        return $this->getRetryAfter() === 0;
    }

    /**
     * Transition to a new state.
     */
    private function transitionTo(string $newState): void
    {
        $oldState = $this->getState();

        $this->cache->put(
            $this->cacheKey('state'),
            $newState,
            now()->addHours(24),
        );

        if ($newState === self::STATE_OPEN) {
            $this->cache->put(
                $this->cacheKey('opened_at'),
                time(),
                now()->addHours(24),
            );
        }

        Log::debug("Circuit breaker [{$this->name}] state transition", [
            'circuit' => $this->name,
            'from' => $oldState,
            'to' => $newState,
        ]);
    }

    /**
     * Generate cache key for this circuit.
     */
    private function cacheKey(string $suffix): string
    {
        return "circuit_breaker:{$this->name}:{$suffix}";
    }
}
