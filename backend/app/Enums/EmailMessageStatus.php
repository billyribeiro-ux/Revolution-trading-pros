<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Email Message Status Enum
 *
 * Defines processing states for individual email messages.
 *
 * @version 1.0.0
 */
enum EmailMessageStatus: string
{
    case PENDING = 'pending';
    case PROCESSING = 'processing';
    case PROCESSED = 'processed';
    case FAILED = 'failed';
    case SPAM = 'spam';
    case QUARANTINED = 'quarantined';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::PENDING => 'Pending',
            self::PROCESSING => 'Processing',
            self::PROCESSED => 'Processed',
            self::FAILED => 'Failed',
            self::SPAM => 'Spam',
            self::QUARANTINED => 'Quarantined',
        };
    }

    /**
     * Get status color for UI.
     */
    public function color(): string
    {
        return match ($this) {
            self::PENDING => 'yellow',
            self::PROCESSING => 'blue',
            self::PROCESSED => 'green',
            self::FAILED => 'red',
            self::SPAM => 'orange',
            self::QUARANTINED => 'purple',
        };
    }

    /**
     * Check if message requires attention.
     */
    public function requiresAttention(): bool
    {
        return in_array($this, [self::FAILED, self::QUARANTINED], true);
    }

    /**
     * Check if message is successfully processed.
     */
    public function isSuccess(): bool
    {
        return $this === self::PROCESSED;
    }

    /**
     * Check if message can be retried.
     */
    public function canRetry(): bool
    {
        return in_array($this, [self::FAILED, self::QUARANTINED], true);
    }
}
