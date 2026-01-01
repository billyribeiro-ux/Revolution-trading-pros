<?php

declare(strict_types=1);

namespace App\Contracts\Email;

use App\DataTransferObjects\Email\InboundEmailPayload;

/**
 * Contract for spam analysis.
 *
 * Implementations analyze emails for spam indicators.
 *
 * @version 1.0.0
 */
interface SpamAnalyzerInterface
{
    /**
     * Analyze an email for spam.
     *
     * @param InboundEmailPayload $payload The email to analyze
     * @return SpamAnalysisResult The analysis result
     */
    public function analyze(InboundEmailPayload $payload): SpamAnalysisResult;

    /**
     * Get the spam threshold.
     */
    public function getThreshold(): float;

    /**
     * Check if an email should be marked as spam.
     *
     * @param InboundEmailPayload $payload The email to check
     */
    public function isSpam(InboundEmailPayload $payload): bool;
}

/**
 * Value object for spam analysis results.
 */
readonly class SpamAnalysisResult
{
    /**
     * @param float $score Spam score (0-10)
     * @param bool $isSpam Whether classified as spam
     * @param array<string, float> $factors Contributing factors and their scores
     * @param string|null $verdict Provider's spam verdict
     * @param array<string, mixed> $metadata Additional analysis metadata
     */
    public function __construct(
        public float $score,
        public bool $isSpam,
        public array $factors = [],
        public ?string $verdict = null,
        public array $metadata = [],
    ) {}

    /**
     * Create a result for clean email.
     */
    public static function clean(float $score = 0.0): self
    {
        return new self(
            score: $score,
            isSpam: false,
            verdict: 'CLEAN',
        );
    }

    /**
     * Create a result for spam email.
     *
     * @param array<string, float> $factors
     */
    public static function spam(float $score, array $factors = []): self
    {
        return new self(
            score: $score,
            isSpam: true,
            factors: $factors,
            verdict: 'SPAM',
        );
    }

    /**
     * Get primary spam reason.
     */
    public function getPrimaryReason(): ?string
    {
        if (empty($this->factors)) {
            return null;
        }

        arsort($this->factors);

        return array_key_first($this->factors);
    }

    /**
     * Convert to array.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'score' => $this->score,
            'is_spam' => $this->isSpam,
            'factors' => $this->factors,
            'verdict' => $this->verdict,
            'primary_reason' => $this->getPrimaryReason(),
        ];
    }
}
