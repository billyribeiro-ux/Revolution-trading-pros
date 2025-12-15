<?php

declare(strict_types=1);

namespace App\Support\CircuitBreaker;

use RuntimeException;

/**
 * Exception thrown when circuit breaker is open.
 *
 * @version 1.0.0
 */
class CircuitBreakerOpenException extends RuntimeException
{
    public function __construct(
        private readonly string $circuitName,
        private readonly int $retryAfter,
    ) {
        parent::__construct(
            "Circuit breaker [{$circuitName}] is open. Retry after {$retryAfter} seconds.",
            503,
        );
    }

    /**
     * Get the circuit name.
     */
    public function getCircuitName(): string
    {
        return $this->circuitName;
    }

    /**
     * Get seconds until retry is allowed.
     */
    public function getRetryAfter(): int
    {
        return $this->retryAfter;
    }
}
