<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

/**
 * Exception thrown when rate limit is exceeded.
 *
 * @version 1.0.0
 */
class RateLimitExceededException extends InboundEmailException
{
    protected string $category = 'email.rate_limit';

    protected bool $retryable = true;

    /**
     * Retry-After header value in seconds.
     */
    protected int $retryAfter;

    /**
     * Create a new rate limit exception.
     *
     * @param array<string, mixed> $context
     */
    public function __construct(
        string $message = 'Rate limit exceeded',
        int $retryAfter = 60,
        array $context = [],
    ) {
        parent::__construct($message, 429, null, $context);
        $this->retryAfter = $retryAfter;
    }

    /**
     * Create exception for IP-based rate limiting.
     */
    public static function forIp(string $ip, int $retryAfter = 60): self
    {
        return new self(
            "Rate limit exceeded for IP: {$ip}",
            $retryAfter,
            ['ip' => $ip, 'limit_type' => 'ip'],
        );
    }

    /**
     * Create exception for sender-based rate limiting.
     */
    public static function forSender(string $email, int $retryAfter = 60): self
    {
        return new self(
            "Rate limit exceeded for sender: {$email}",
            $retryAfter,
            ['sender' => $email, 'limit_type' => 'sender'],
        );
    }

    /**
     * Get the Retry-After value in seconds.
     */
    public function getRetryAfter(): int
    {
        return $this->retryAfter;
    }

    public function getHttpStatusCode(): int
    {
        return 429;
    }

    public function getSafeMessage(): string
    {
        return 'Rate limit exceeded';
    }
}
