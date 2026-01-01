<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

/**
 * Exception thrown when webhook signature verification fails.
 *
 * This is a security exception and should trigger security alerts.
 *
 * @version 1.0.0
 */
class InvalidSignatureException extends InboundEmailException
{
    protected string $category = 'email.security.signature';

    protected bool $retryable = false;

    /**
     * Create exception for missing signature.
     *
     * @param array<string, mixed> $context
     */
    public static function missing(array $context = []): self
    {
        return new self(
            'Webhook signature header is missing',
            401,
            null,
            array_merge($context, ['reason' => 'missing_signature']),
        );
    }

    /**
     * Create exception for invalid signature.
     *
     * @param array<string, mixed> $context
     */
    public static function invalid(array $context = []): self
    {
        return new self(
            'Webhook signature verification failed',
            401,
            null,
            array_merge($context, ['reason' => 'invalid_signature']),
        );
    }

    /**
     * Create exception for expired signature.
     *
     * @param array<string, mixed> $context
     */
    public static function expired(array $context = []): self
    {
        return new self(
            'Webhook signature has expired',
            401,
            null,
            array_merge($context, ['reason' => 'expired_signature']),
        );
    }

    public function getHttpStatusCode(): int
    {
        return 401;
    }

    public function getSafeMessage(): string
    {
        return 'Invalid signature';
    }
}
