<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Email Direction Enum
 *
 * Defines the direction of email flow.
 *
 * @version 1.0.0
 */
enum EmailDirection: string
{
    case INBOUND = 'inbound';
    case OUTBOUND = 'outbound';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::INBOUND => 'Inbound',
            self::OUTBOUND => 'Outbound',
        };
    }

    /**
     * Get icon for UI.
     */
    public function icon(): string
    {
        return match ($this) {
            self::INBOUND => 'arrow-down-left',
            self::OUTBOUND => 'arrow-up-right',
        };
    }

    /**
     * Check if message is from external sender.
     */
    public function isFromExternal(): bool
    {
        return $this === self::INBOUND;
    }
}
