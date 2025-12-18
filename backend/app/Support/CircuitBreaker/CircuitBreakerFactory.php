<?php

declare(strict_types=1);

namespace App\Support\CircuitBreaker;

use Illuminate\Contracts\Cache\Repository as CacheRepository;

/**
 * Factory for creating circuit breaker instances.
 *
 * Manages circuit breaker instances and provides default configurations.
 *
 * @version 1.0.0
 */
class CircuitBreakerFactory
{
    /**
     * Circuit breaker instances.
     *
     * @var array<string, CircuitBreaker>
     */
    private array $circuits = [];

    /**
     * Default configurations per service type.
     *
     * @var array<string, array{failure_threshold: int, success_threshold: int, timeout: int}>
     */
    private array $defaults = [
        'email' => [
            'failure_threshold' => 5,
            'success_threshold' => 2,
            'timeout' => 60,
        ],
        'storage' => [
            'failure_threshold' => 3,
            'success_threshold' => 1,
            'timeout' => 30,
        ],
        'external' => [
            'failure_threshold' => 10,
            'success_threshold' => 3,
            'timeout' => 120,
        ],
    ];

    public function __construct(
        private readonly CacheRepository $cache,
    ) {}

    /**
     * Get or create a circuit breaker for a service.
     *
     * @param string $name Unique circuit name
     * @param string $type Service type for default config (email, storage, external)
     */
    public function for(string $name, string $type = 'external'): CircuitBreaker
    {
        if (isset($this->circuits[$name])) {
            return $this->circuits[$name];
        }

        $config = $this->defaults[$type] ?? $this->defaults['external'];

        $this->circuits[$name] = new CircuitBreaker(
            cache: $this->cache,
            name: $name,
            failureThreshold: $config['failure_threshold'],
            successThreshold: $config['success_threshold'],
            timeout: $config['timeout'],
        );

        return $this->circuits[$name];
    }

    /**
     * Create a circuit breaker with custom configuration.
     *
     * @param string $name Unique circuit name
     * @param int $failureThreshold Failures before opening
     * @param int $successThreshold Successes before closing
     * @param int $timeout Seconds before recovery attempt
     */
    public function create(
        string $name,
        int $failureThreshold = 5,
        int $successThreshold = 2,
        int $timeout = 60,
    ): CircuitBreaker {
        $this->circuits[$name] = new CircuitBreaker(
            cache: $this->cache,
            name: $name,
            failureThreshold: $failureThreshold,
            successThreshold: $successThreshold,
            timeout: $timeout,
        );

        return $this->circuits[$name];
    }

    /**
     * Get all circuit breakers.
     *
     * @return array<string, CircuitBreaker>
     */
    public function all(): array
    {
        return $this->circuits;
    }

    /**
     * Get statistics for all circuits.
     *
     * @return array<string, array<string, mixed>>
     */
    public function getAllStats(): array
    {
        $stats = [];

        foreach ($this->circuits as $name => $circuit) {
            $stats[$name] = $circuit->getStats();
        }

        return $stats;
    }

    /**
     * Reset all circuit breakers.
     */
    public function resetAll(): void
    {
        foreach ($this->circuits as $circuit) {
            $circuit->reset();
        }
    }

    /**
     * Circuit breaker for Postmark email service.
     */
    public function postmark(): CircuitBreaker
    {
        return $this->for('email.postmark', 'email');
    }

    /**
     * Circuit breaker for AWS SES email service.
     */
    public function ses(): CircuitBreaker
    {
        return $this->for('email.ses', 'email');
    }

    /**
     * Circuit breaker for SendGrid email service.
     */
    public function sendgrid(): CircuitBreaker
    {
        return $this->for('email.sendgrid', 'email');
    }

    /**
     * Circuit breaker for R2/S3 storage service.
     */
    public function storage(): CircuitBreaker
    {
        return $this->for('storage.r2', 'storage');
    }

    /**
     * Circuit breaker for ClamAV virus scanner.
     */
    public function virusScanner(): CircuitBreaker
    {
        return $this->for('security.clamav', 'external');
    }
}
