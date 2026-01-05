<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

/**
 * Exception thrown when email is classified as spam.
 *
 * @version 1.0.0
 */
class SpamDetectedException extends InboundEmailException
{
    protected string $category = 'email.spam';

    protected bool $retryable = false;

    /**
     * Spam score that triggered the detection.
     */
    protected float $spamScore;

    /**
     * Threshold that was exceeded.
     */
    protected float $threshold;

    /**
     * Create a new spam detected exception.
     *
     * @param array<string, mixed> $context
     */
    public function __construct(
        float $spamScore,
        float $threshold,
        array $context = [],
    ) {
        $this->spamScore = $spamScore;
        $this->threshold = $threshold;

        parent::__construct(
            "Email classified as spam (score: {$spamScore}, threshold: {$threshold})",
            422,
            null,
            array_merge($context, [
                'spam_score' => $spamScore,
                'threshold' => $threshold,
            ]),
        );
    }

    /**
     * Get the spam score.
     */
    public function getSpamScore(): float
    {
        return $this->spamScore;
    }

    /**
     * Get the threshold that was exceeded.
     */
    public function getThreshold(): float
    {
        return $this->threshold;
    }

    public function getHttpStatusCode(): int
    {
        return 200; // Still return 200 to prevent webhook retries
    }

    public function getSafeMessage(): string
    {
        return 'Email classified as spam';
    }
}
