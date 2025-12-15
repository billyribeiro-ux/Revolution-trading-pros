<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Conversation Priority Enum
 *
 * Defines priority levels for email conversations.
 *
 * @version 1.0.0
 */
enum ConversationPriority: string
{
    case LOW = 'low';
    case NORMAL = 'normal';
    case HIGH = 'high';
    case URGENT = 'urgent';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::LOW => 'Low',
            self::NORMAL => 'Normal',
            self::HIGH => 'High',
            self::URGENT => 'Urgent',
        };
    }

    /**
     * Get priority color for UI.
     */
    public function color(): string
    {
        return match ($this) {
            self::LOW => 'gray',
            self::NORMAL => 'blue',
            self::HIGH => 'orange',
            self::URGENT => 'red',
        };
    }

    /**
     * Get numeric weight for sorting.
     */
    public function weight(): int
    {
        return match ($this) {
            self::LOW => 1,
            self::NORMAL => 2,
            self::HIGH => 3,
            self::URGENT => 4,
        };
    }

    /**
     * Get SLA response time in minutes.
     */
    public function slaMinutes(): int
    {
        return match ($this) {
            self::LOW => 1440,      // 24 hours
            self::NORMAL => 480,    // 8 hours
            self::HIGH => 120,      // 2 hours
            self::URGENT => 30,     // 30 minutes
        };
    }
}
