<?php

declare(strict_types=1);

namespace App\Enums;

/**
 * Attachment Scan Status Enum
 *
 * Defines virus/malware scan states for email attachments.
 *
 * @version 1.0.0
 */
enum AttachmentScanStatus: string
{
    case PENDING = 'pending';
    case CLEAN = 'clean';
    case INFECTED = 'infected';
    case ERROR = 'error';
    case SKIPPED = 'skipped';

    /**
     * Get human-readable label.
     */
    public function label(): string
    {
        return match ($this) {
            self::PENDING => 'Pending Scan',
            self::CLEAN => 'Clean',
            self::INFECTED => 'Infected',
            self::ERROR => 'Scan Error',
            self::SKIPPED => 'Skipped',
        };
    }

    /**
     * Get status color for UI.
     */
    public function color(): string
    {
        return match ($this) {
            self::PENDING => 'yellow',
            self::CLEAN => 'green',
            self::INFECTED => 'red',
            self::ERROR => 'orange',
            self::SKIPPED => 'gray',
        };
    }

    /**
     * Check if attachment is safe to download.
     */
    public function isSafeToDownload(): bool
    {
        return in_array($this, [self::CLEAN, self::SKIPPED], true);
    }

    /**
     * Check if attachment should be blocked.
     */
    public function isBlocked(): bool
    {
        return $this === self::INFECTED;
    }

    /**
     * Check if scan needs retry.
     */
    public function needsRetry(): bool
    {
        return $this === self::ERROR;
    }
}
