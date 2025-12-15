<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

use Exception;
use Throwable;

/**
 * Base exception for inbound email processing errors.
 *
 * Provides structured error categorization for enterprise-grade
 * error handling, logging, and alerting.
 *
 * @version 1.0.0
 */
class InboundEmailException extends Exception
{
    /**
     * Error category for classification.
     */
    protected string $category = 'email.inbound';

    /**
     * Whether this error is retryable.
     */
    protected bool $retryable = false;

    /**
     * Context data for structured logging.
     *
     * @var array<string, mixed>
     */
    protected array $context = [];

    /**
     * Correlation ID for request tracing.
     */
    protected ?string $correlationId = null;

    /**
     * Create a new exception instance.
     *
     * @param array<string, mixed> $context
     */
    public function __construct(
        string $message,
        int $code = 0,
        ?Throwable $previous = null,
        array $context = [],
    ) {
        parent::__construct($message, $code, $previous);
        $this->context = $context;
        $this->correlationId = $context['correlation_id'] ?? null;
    }

    /**
     * Get the error category for classification.
     */
    public function getCategory(): string
    {
        return $this->category;
    }

    /**
     * Check if this error is retryable.
     */
    public function isRetryable(): bool
    {
        return $this->retryable;
    }

    /**
     * Get structured context for logging.
     *
     * @return array<string, mixed>
     */
    public function getContext(): array
    {
        return array_merge($this->context, [
            'exception_class' => static::class,
            'category' => $this->category,
            'retryable' => $this->retryable,
            'correlation_id' => $this->correlationId,
        ]);
    }

    /**
     * Get correlation ID for request tracing.
     */
    public function getCorrelationId(): ?string
    {
        return $this->correlationId;
    }

    /**
     * Set correlation ID for request tracing.
     */
    public function setCorrelationId(string $correlationId): self
    {
        $this->correlationId = $correlationId;
        $this->context['correlation_id'] = $correlationId;

        return $this;
    }

    /**
     * Get HTTP status code for API responses.
     */
    public function getHttpStatusCode(): int
    {
        return 500;
    }

    /**
     * Get safe message for external API responses.
     */
    public function getSafeMessage(): string
    {
        return 'An error occurred processing the email';
    }

    /**
     * Convert to array for JSON responses.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'error' => $this->getSafeMessage(),
            'code' => $this->getCode(),
            'category' => $this->category,
            'retryable' => $this->retryable,
            'correlation_id' => $this->correlationId,
        ];
    }
}
