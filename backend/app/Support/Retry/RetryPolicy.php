<?php

declare(strict_types=1);

namespace App\Support\Retry;

use Closure;
use Illuminate\Support\Facades\Log;
use Throwable;

/**
 * Retry Policy with Exponential Backoff
 *
 * Provides configurable retry logic with exponential backoff,
 * jitter, and exception filtering for resilient operations.
 *
 * @version 1.0.0
 */
class RetryPolicy
{
    /**
     * Maximum retry attempts.
     */
    private int $maxAttempts = 3;

    /**
     * Base delay in milliseconds.
     */
    private int $baseDelayMs = 100;

    /**
     * Maximum delay in milliseconds.
     */
    private int $maxDelayMs = 30000;

    /**
     * Exponential base (multiplier).
     */
    private float $multiplier = 2.0;

    /**
     * Whether to add random jitter.
     */
    private bool $useJitter = true;

    /**
     * Exceptions that should trigger retry.
     *
     * @var array<class-string<Throwable>>
     */
    private array $retryableExceptions = [];

    /**
     * Exceptions that should NOT trigger retry.
     *
     * @var array<class-string<Throwable>>
     */
    private array $nonRetryableExceptions = [];

    /**
     * Custom retry predicate.
     *
     * @var Closure|null
     */
    private ?Closure $retryPredicate = null;

    /**
     * Callback executed before each retry.
     *
     * @var Closure|null
     */
    private ?Closure $onRetry = null;

    /**
     * Create a new retry policy.
     */
    public static function create(): self
    {
        return new self();
    }

    /**
     * Create a policy with default settings for HTTP requests.
     */
    public static function forHttp(): self
    {
        return self::create()
            ->withMaxAttempts(3)
            ->withBaseDelay(1000)
            ->withMaxDelay(10000)
            ->withJitter();
    }

    /**
     * Create a policy with default settings for database operations.
     */
    public static function forDatabase(): self
    {
        return self::create()
            ->withMaxAttempts(5)
            ->withBaseDelay(50)
            ->withMaxDelay(5000)
            ->withJitter();
    }

    /**
     * Create a policy with default settings for email operations.
     */
    public static function forEmail(): self
    {
        return self::create()
            ->withMaxAttempts(3)
            ->withBaseDelay(500)
            ->withMaxDelay(30000)
            ->withJitter();
    }

    /**
     * Set maximum retry attempts.
     */
    public function withMaxAttempts(int $attempts): self
    {
        $this->maxAttempts = max(1, $attempts);

        return $this;
    }

    /**
     * Set base delay in milliseconds.
     */
    public function withBaseDelay(int $milliseconds): self
    {
        $this->baseDelayMs = max(0, $milliseconds);

        return $this;
    }

    /**
     * Set maximum delay in milliseconds.
     */
    public function withMaxDelay(int $milliseconds): self
    {
        $this->maxDelayMs = max($this->baseDelayMs, $milliseconds);

        return $this;
    }

    /**
     * Set exponential multiplier.
     */
    public function withMultiplier(float $multiplier): self
    {
        $this->multiplier = max(1.0, $multiplier);

        return $this;
    }

    /**
     * Enable jitter (randomization).
     */
    public function withJitter(bool $enabled = true): self
    {
        $this->useJitter = $enabled;

        return $this;
    }

    /**
     * Set exceptions that should trigger retry.
     *
     * @param array<class-string<Throwable>> $exceptions
     */
    public function retryOn(array $exceptions): self
    {
        $this->retryableExceptions = $exceptions;

        return $this;
    }

    /**
     * Set exceptions that should NOT trigger retry.
     *
     * @param array<class-string<Throwable>> $exceptions
     */
    public function dontRetryOn(array $exceptions): self
    {
        $this->nonRetryableExceptions = $exceptions;

        return $this;
    }

    /**
     * Set custom retry predicate.
     *
     * @param Closure(Throwable, int): bool $predicate
     */
    public function retryIf(Closure $predicate): self
    {
        $this->retryPredicate = $predicate;

        return $this;
    }

    /**
     * Set callback to execute before each retry.
     *
     * @param Closure(Throwable, int, int): void $callback (exception, attempt, delayMs)
     */
    public function onRetry(Closure $callback): self
    {
        $this->onRetry = $callback;

        return $this;
    }

    /**
     * Execute an operation with retry policy.
     *
     * @template T
     * @param Closure(): T $operation
     * @return T
     * @throws Throwable
     */
    public function execute(Closure $operation): mixed
    {
        $attempt = 1;
        $lastException = null;

        while ($attempt <= $this->maxAttempts) {
            try {
                return $operation();
            } catch (Throwable $e) {
                $lastException = $e;

                // Check if we should retry
                if (!$this->shouldRetry($e, $attempt)) {
                    throw $e;
                }

                // Check if we've exhausted attempts
                if ($attempt >= $this->maxAttempts) {
                    throw $e;
                }

                // Calculate delay
                $delayMs = $this->calculateDelay($attempt);

                // Execute retry callback
                if ($this->onRetry !== null) {
                    ($this->onRetry)($e, $attempt, $delayMs);
                }

                // Wait before retrying
                usleep($delayMs * 1000);

                $attempt++;
            }
        }

        // Should never reach here, but just in case
        throw $lastException ?? new \RuntimeException('Retry policy exhausted');
    }

    /**
     * Determine if operation should be retried.
     */
    private function shouldRetry(Throwable $exception, int $attempt): bool
    {
        // Check custom predicate first
        if ($this->retryPredicate !== null) {
            return ($this->retryPredicate)($exception, $attempt);
        }

        // Check non-retryable exceptions
        foreach ($this->nonRetryableExceptions as $nonRetryable) {
            if ($exception instanceof $nonRetryable) {
                return false;
            }
        }

        // Check retryable exceptions (if specified)
        if (!empty($this->retryableExceptions)) {
            foreach ($this->retryableExceptions as $retryable) {
                if ($exception instanceof $retryable) {
                    return true;
                }
            }

            return false;
        }

        // Default: retry all exceptions
        return true;
    }

    /**
     * Calculate delay for a given attempt.
     */
    private function calculateDelay(int $attempt): int
    {
        // Exponential backoff: baseDelay * multiplier^(attempt-1)
        $delay = (int) ($this->baseDelayMs * pow($this->multiplier, $attempt - 1));

        // Apply maximum cap
        $delay = min($delay, $this->maxDelayMs);

        // Apply jitter (±25%)
        if ($this->useJitter && $delay > 0) {
            $jitterRange = (int) ($delay * 0.25);
            $delay += random_int(-$jitterRange, $jitterRange);
            $delay = max(0, $delay);
        }

        return $delay;
    }

    /**
     * Get configuration for logging/debugging.
     *
     * @return array<string, mixed>
     */
    public function getConfig(): array
    {
        return [
            'max_attempts' => $this->maxAttempts,
            'base_delay_ms' => $this->baseDelayMs,
            'max_delay_ms' => $this->maxDelayMs,
            'multiplier' => $this->multiplier,
            'use_jitter' => $this->useJitter,
            'retryable_exceptions' => $this->retryableExceptions,
            'non_retryable_exceptions' => $this->nonRetryableExceptions,
        ];
    }
}
