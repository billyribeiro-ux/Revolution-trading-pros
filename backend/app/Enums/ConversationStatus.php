<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Conversation Status Enum
 *
 * Defines the lifecycle states for email conversations in the CRM.
 *
 * @version 1.0.0
 */
enum ConversationStatus: string
{
    case OPEN = 'open';
    case PENDING = 'pending';
    case RESOLVED = 'resolved';
    case ARCHIVED = 'archived';
    case SPAM = 'spam';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::OPEN => 'Open',
            self::PENDING => 'Pending',
            self::RESOLVED => 'Resolved',
            self::ARCHIVED => 'Archived',
            self::SPAM => 'Spam',
        };
    }

    /**
     * Get status color for UI.
     */
    public function color(): string
    {
        return match ($this) {
            self::OPEN => 'blue',
            self::PENDING => 'yellow',
            self::RESOLVED => 'green',
            self::ARCHIVED => 'gray',
            self::SPAM => 'red',
        };
    }

    /**
     * Check if conversation is active.
     */
    public function isActive(): bool
    {
        return in_array($this, [self::OPEN, self::PENDING], true);
    }

    /**
     * Check if conversation is closed.
     */
    public function isClosed(): bool
    {
        return in_array($this, [self::RESOLVED, self::ARCHIVED, self::SPAM], true);
    }

    /**
     * Get all active statuses.
     *
     * @return array<self>
     */
    public static function activeStatuses(): array
    {
        return [self::OPEN, self::PENDING];
    }

    /**
     * Get all closed statuses.
     *
     * @return array<self>
     */
    public static function closedStatuses(): array
    {
        return [self::RESOLVED, self::ARCHIVED, self::SPAM];
    }
}
