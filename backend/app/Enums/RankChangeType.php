<?php

namespace App\Enums;

/**
 * Enum for rank change types.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
enum RankChangeType: string
{
    case IMPROVED = 'improved';
    case DECLINED = 'declined';
    case DROPPED = 'dropped';
    case STABLE = 'stable';
    case NEW_ENTRY = 'new_entry';
    case LOST = 'lost';
    case RECOVERED = 'recovered';

    /**
     * Get display label.
     */
    public function label(): string
    {
        return match($this) {
            self::IMPROVED => 'Improved',
            self::DECLINED => 'Declined',
            self::DROPPED => 'Dropped',
            self::STABLE => 'Stable',
            self::NEW_ENTRY => 'New Entry',
            self::LOST => 'Lost',
            self::RECOVERED => 'Recovered',
        };
    }

    /**
     * Get CSS color class.
     */
    public function colorClass(): string
    {
        return match($this) {
            self::IMPROVED => 'text-green-500',
            self::DECLINED => 'text-red-500',
            self::DROPPED => 'text-red-500',
            self::STABLE => 'text-gray-500',
            self::NEW_ENTRY => 'text-blue-500',
            self::LOST => 'text-orange-500',
            self::RECOVERED => 'text-green-400',
        };
    }

    /**
     * Determine change type from positions.
     */
    public static function fromPositions(?int $previous, ?int $current): self
    {
        if ($previous === null && $current !== null) {
            return self::NEW_ENTRY;
        }

        if ($previous !== null && $current === null) {
            return self::LOST;
        }

        if ($previous === null || $current === null) {
            return self::STABLE;
        }

        if ($current < $previous) {
            return self::IMPROVED;
        }

        if ($current > $previous) {
            return self::DECLINED;
        }

        return self::STABLE;
    }
}
