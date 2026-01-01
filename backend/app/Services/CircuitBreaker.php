<?php

declare(strict_types=1);

namespace App\Services;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Closure;
use Throwable;

/**
 * Circuit Breaker Service
 *
 * Google Enterprise Grade circuit breaker pattern for:
 * - External service protection
 * - Automatic failure detection
 * - Gradual recovery (half-open state)
 * - Configurable thresholds and timeouts
 *
 * States:
 * - CLOSED: Normal operation, requests flow through
 * - OPEN: Circuit tripped, requests fail fast
 * - HALF_OPEN: Testing if service recovered
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class CircuitBreaker
{
    public const STATE_CLOSED = 'closed';
    public const STATE_OPEN = 'open';
    public const STATE_HALF_OPEN = 'half_open';

    protected string $service;
    protected int $failureThreshold;
    protected int $successThreshold;
    protected int $timeout;
    protected int $halfOpenRequests;

    /**
     * Create a new circuit breaker instance
     */
    public function __construct(
        string $service,
        int $failureThreshold = 5,
        int $successThreshold = 3,
        int $timeout = 60,
        int $halfOpenRequests = 3
    ) {
        $this->service = $service;
        $this->failureThreshold = $failureThreshold;
        $this->successThreshold = $successThreshold;
        $this->timeout = $timeout;
        $this->halfOpenRequests = $halfOpenRequests;
    }

    /**
     * Execute operation through circuit breaker
     *
     * @template T
     * @param Closure(): T $operation
     * @param Closure(): T|null $fallback
     * @return T
     * @throws CircuitBreakerOpenException
     */
    public function execute(Closure $operation, ?Closure $fallback = null): mixed
    {
        $state = $this->getState();

        // Fast fail if circuit is open
        if ($state === self::STATE_OPEN) {
            Log::warning('Circuit breaker OPEN, request blocked', [
                'service' => $this->service,
                'state' => $state,
            ]);

            if ($fallback) {
                return $fallback();
            }

            throw new CircuitBreakerOpenException(
                "Circuit breaker is OPEN for service: {$this->service}"
            );
        }

        // Check half-open request limit
        if ($state === self::STATE_HALF_OPEN) {
            $halfOpenCount = $this->getHalfOpenCount();
            if ($halfOpenCount >= $this->halfOpenRequests) {
                Log::debug('Circuit breaker HALF_OPEN limit reached', [
                    'service' => $this->service,
                    'count' => $halfOpenCount,
                ]);

                if ($fallback) {
                    return $fallback();
                }

                throw new CircuitBreakerOpenException(
                    "Circuit breaker HALF_OPEN limit reached for service: {$this->service}"
                );
            }
            $this->incrementHalfOpenCount();
        }

        try {
            $result = $operation();
            $this->recordSuccess();
            return $result;
        } catch (Throwable $e) {
            $this->recordFailure($e);

            if ($fallback) {
                return $fallback();
            }

            throw $e;
        }
    }

    /**
     * Record a successful operation
     */
    protected function recordSuccess(): void
    {
        $state = $this->getState();

        if ($state === self::STATE_HALF_OPEN) {
            $successCount = $this->incrementSuccessCount();

            if ($successCount >= $this->successThreshold) {
                $this->closeCircuit();
                Log::info('Circuit breaker CLOSED after recovery', [
                    'service' => $this->service,
                    'success_count' => $successCount,
                ]);
            }
        } elseif ($state === self::STATE_CLOSED) {
            // Reset failure count on success
            $this->resetFailureCount();
        }
    }

    /**
     * Record a failed operation
     */
    protected function recordFailure(Throwable $e): void
    {
        $failureCount = $this->incrementFailureCount();

        Log::warning('Circuit breaker failure recorded', [
            'service' => $this->service,
            'failure_count' => $failureCount,
            'threshold' => $this->failureThreshold,
            'error' => $e->getMessage(),
        ]);

        $state = $this->getState();

        if ($state === self::STATE_HALF_OPEN) {
            // Immediately open on failure in half-open state
            $this->openCircuit();
            Log::warning('Circuit breaker OPENED from HALF_OPEN state', [
                'service' => $this->service,
            ]);
        } elseif ($failureCount >= $this->failureThreshold) {
            $this->openCircuit();
            Log::error('Circuit breaker OPENED after threshold exceeded', [
                'service' => $this->service,
                'failure_count' => $failureCount,
            ]);
        }
    }

    /**
     * Get current circuit state
     */
    public function getState(): string
    {
        $state = Cache::get($this->stateKey());

        if ($state === self::STATE_OPEN) {
            // Check if timeout has passed for half-open transition
            $openedAt = Cache::get($this->openedAtKey());
            if ($openedAt && (time() - $openedAt) >= $this->timeout) {
                $this->halfOpenCircuit();
                return self::STATE_HALF_OPEN;
            }
        }

        return $state ?? self::STATE_CLOSED;
    }

    /**
     * Open the circuit
     */
    protected function openCircuit(): void
    {
        Cache::put($this->stateKey(), self::STATE_OPEN, now()->addMinutes(10));
        Cache::put($this->openedAtKey(), time(), now()->addMinutes(10));
        $this->resetSuccessCount();
        $this->resetHalfOpenCount();
    }

    /**
     * Transition to half-open state
     */
    protected function halfOpenCircuit(): void
    {
        Cache::put($this->stateKey(), self::STATE_HALF_OPEN, now()->addMinutes(10));
        $this->resetSuccessCount();
        $this->resetHalfOpenCount();
    }

    /**
     * Close the circuit
     */
    protected function closeCircuit(): void
    {
        Cache::forget($this->stateKey());
        Cache::forget($this->openedAtKey());
        $this->resetFailureCount();
        $this->resetSuccessCount();
        $this->resetHalfOpenCount();
    }

    /**
     * Get circuit breaker metrics
     */
    public function getMetrics(): array
    {
        return [
            'service' => $this->service,
            'state' => $this->getState(),
            'failure_count' => Cache::get($this->failureCountKey(), 0),
            'success_count' => Cache::get($this->successCountKey(), 0),
            'half_open_count' => Cache::get($this->halfOpenCountKey(), 0),
            'failure_threshold' => $this->failureThreshold,
            'success_threshold' => $this->successThreshold,
            'timeout_seconds' => $this->timeout,
        ];
    }

    /**
     * Manually reset the circuit breaker
     */
    public function reset(): void
    {
        $this->closeCircuit();
        Log::info('Circuit breaker manually reset', [
            'service' => $this->service,
        ]);
    }

    // Counter methods
    protected function incrementFailureCount(): int
    {
        return Cache::increment($this->failureCountKey());
    }

    protected function resetFailureCount(): void
    {
        Cache::forget($this->failureCountKey());
    }

    protected function incrementSuccessCount(): int
    {
        return Cache::increment($this->successCountKey());
    }

    protected function resetSuccessCount(): void
    {
        Cache::forget($this->successCountKey());
    }

    protected function getHalfOpenCount(): int
    {
        return (int) Cache::get($this->halfOpenCountKey(), 0);
    }

    protected function incrementHalfOpenCount(): int
    {
        return Cache::increment($this->halfOpenCountKey());
    }

    protected function resetHalfOpenCount(): void
    {
        Cache::forget($this->halfOpenCountKey());
    }

    // Cache key helpers
    protected function stateKey(): string
    {
        return "circuit_breaker:{$this->service}:state";
    }

    protected function openedAtKey(): string
    {
        return "circuit_breaker:{$this->service}:opened_at";
    }

    protected function failureCountKey(): string
    {
        return "circuit_breaker:{$this->service}:failures";
    }

    protected function successCountKey(): string
    {
        return "circuit_breaker:{$this->service}:successes";
    }

    protected function halfOpenCountKey(): string
    {
        return "circuit_breaker:{$this->service}:half_open";
    }
}

/**
 * Exception thrown when circuit is open
 */
class CircuitBreakerOpenException extends \Exception
{
}
