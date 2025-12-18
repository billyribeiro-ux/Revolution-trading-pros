<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

/**
 * Exception for attachment processing errors.
 *
 * @version 1.0.0
 */
class AttachmentException extends InboundEmailException
{
    protected string $category = 'email.attachment';

    protected bool $retryable = false;

    /**
     * Create exception for file too large.
     *
     * @param array<string, mixed> $context
     */
    public static function tooLarge(
        string $filename,
        int $size,
        int $maxSize,
        array $context = [],
    ): self {
        return new self(
            "Attachment '{$filename}' exceeds maximum size ({$size} > {$maxSize} bytes)",
            413,
            null,
            array_merge($context, [
                'filename' => $filename,
                'size' => $size,
                'max_size' => $maxSize,
                'reason' => 'too_large',
            ]),
        );
    }

    /**
     * Create exception for blocked file type.
     *
     * @param array<string, mixed> $context
     */
    public static function blockedType(
        string $filename,
        string $extension,
        array $context = [],
    ): self {
        return new self(
            "Attachment '{$filename}' has blocked file type: {$extension}",
            415,
            null,
            array_merge($context, [
                'filename' => $filename,
                'extension' => $extension,
                'reason' => 'blocked_type',
            ]),
        );
    }

    /**
     * Create exception for virus detected.
     *
     * @param array<string, mixed> $context
     */
    public static function virusDetected(
        string $filename,
        string $virusName,
        array $context = [],
    ): self {
        $exception = new self(
            "Virus detected in attachment '{$filename}': {$virusName}",
            422,
            null,
            array_merge($context, [
                'filename' => $filename,
                'virus_name' => $virusName,
                'reason' => 'virus_detected',
            ]),
        );
        $exception->category = 'email.security.virus';

        return $exception;
    }

    /**
     * Create exception for storage failure.
     *
     * @param array<string, mixed> $context
     */
    public static function storageFailed(
        string $filename,
        string $reason,
        array $context = [],
    ): self {
        $exception = new self(
            "Failed to store attachment '{$filename}': {$reason}",
            500,
            null,
            array_merge($context, [
                'filename' => $filename,
                'storage_error' => $reason,
                'reason' => 'storage_failed',
            ]),
        );
        $exception->retryable = true;

        return $exception;
    }

    public function getSafeMessage(): string
    {
        $context = $this->getContext();
        $reason = $context['reason'] ?? 'unknown';

        return match ($reason) {
            'too_large' => 'Attachment exceeds maximum size',
            'blocked_type' => 'File type not allowed',
            'virus_detected' => 'Security threat detected in attachment',
            'storage_failed' => 'Failed to process attachment',
            default => 'Attachment processing error',
        };
    }
}
