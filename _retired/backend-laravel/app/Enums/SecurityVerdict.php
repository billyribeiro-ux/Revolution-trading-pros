<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Security Verdict Enum
 *
 * Defines email authentication and security check results.
 * Used for SPF, DKIM, DMARC, spam, and virus verdicts.
 *
 * @version 1.0.0
 */
enum SecurityVerdict: string
{
    case PASS = 'pass';
    case FAIL = 'fail';
    case NEUTRAL = 'neutral';
    case NONE = 'none';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::PASS => 'Pass',
            self::FAIL => 'Fail',
            self::NEUTRAL => 'Neutral',
            self::NONE => 'None',
        };
    }

    /**
     * Get verdict color for UI.
     */
    public function color(): string
    {
        return match ($this) {
            self::PASS => 'green',
            self::FAIL => 'red',
            self::NEUTRAL => 'yellow',
            self::NONE => 'gray',
        };
    }

    /**
     * Check if verdict is positive.
     */
    public function isPassing(): bool
    {
        return $this === self::PASS;
    }

    /**
     * Check if verdict indicates a problem.
     */
    public function isFailing(): bool
    {
        return $this === self::FAIL;
    }

    /**
     * Get numeric score for aggregate calculations.
     */
    public function score(): int
    {
        return match ($this) {
            self::PASS => 100,
            self::NEUTRAL => 50,
            self::NONE => 0,
            self::FAIL => -100,
        };
    }
}
