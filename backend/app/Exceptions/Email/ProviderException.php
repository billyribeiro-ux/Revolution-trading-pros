<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

use Throwable;

/**
 * Exception for email provider errors.
 *
 * Used when external email services fail or return errors.
 *
 * @version 1.0.0
 */
class ProviderException extends InboundEmailException
{
    protected string $category = 'email.provider';

    protected bool $retryable = true;

    /**
     * Provider name.
     */
    protected string $provider;

    /**
     * Provider error code.
     */
    protected ?string $providerCode;

    /**
     * Create a new provider exception.
     *
     * @param array<string, mixed> $context
     */
    public function __construct(
        string $provider,
        string $message,
        ?string $providerCode = null,
        int $code = 0,
        ?Throwable $previous = null,
        array $context = [],
    ) {
        $this->provider = $provider;
        $this->providerCode = $providerCode;
        $this->category = "email.provider.{$provider}";

        parent::__construct(
            "[{$provider}] {$message}",
            $code,
            $previous,
            array_merge($context, [
                'provider' => $provider,
                'provider_code' => $providerCode,
            ]),
        );
    }

    /**
     * Create exception for Postmark errors.
     *
     * @param array<string, mixed> $context
     */
    public static function postmark(
        string $message,
        ?string $errorCode = null,
        ?Throwable $previous = null,
        array $context = [],
    ): self {
        return new self('postmark', $message, $errorCode, 0, $previous, $context);
    }

    /**
     * Create exception for AWS SES errors.
     *
     * @param array<string, mixed> $context
     */
    public static function ses(
        string $message,
        ?string $errorCode = null,
        ?Throwable $previous = null,
        array $context = [],
    ): self {
        return new self('ses', $message, $errorCode, 0, $previous, $context);
    }

    /**
     * Create exception for SendGrid errors.
     *
     * @param array<string, mixed> $context
     */
    public static function sendgrid(
        string $message,
        ?string $errorCode = null,
        ?Throwable $previous = null,
        array $context = [],
    ): self {
        return new self('sendgrid', $message, $errorCode, 0, $previous, $context);
    }

    /**
     * Create exception for connection timeout.
     *
     * @param array<string, mixed> $context
     */
    public static function timeout(
        string $provider,
        int $timeout,
        ?Throwable $previous = null,
        array $context = [],
    ): self {
        return new self(
            $provider,
            "Connection timeout after {$timeout}s",
            'TIMEOUT',
            0,
            $previous,
            array_merge($context, ['timeout' => $timeout]),
        );
    }

    /**
     * Create exception for circuit breaker open.
     *
     * @param array<string, mixed> $context
     */
    public static function circuitOpen(
        string $provider,
        int $retryAfter,
        array $context = [],
    ): self {
        $exception = new self(
            $provider,
            "Circuit breaker open, retry after {$retryAfter}s",
            'CIRCUIT_OPEN',
            503,
            null,
            array_merge($context, ['retry_after' => $retryAfter]),
        );

        return $exception;
    }

    /**
     * Get the provider name.
     */
    public function getProvider(): string
    {
        return $this->provider;
    }

    /**
     * Get the provider error code.
     */
    public function getProviderCode(): ?string
    {
        return $this->providerCode;
    }

    public function getSafeMessage(): string
    {
        return 'Email provider error';
    }
}
