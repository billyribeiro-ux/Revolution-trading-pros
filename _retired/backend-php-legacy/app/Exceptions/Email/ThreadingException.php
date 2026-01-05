<?php

declare(strict_types=1);

namespace App\Exceptions\Email;

/**
 * Exception for email threading errors.
 *
 * @version 1.0.0
 */
class ThreadingException extends InboundEmailException
{
    protected string $category = 'email.threading';

    protected bool $retryable = true;

    /**
     * Create exception for conversation not found.
     *
     * @param array<string, mixed> $context
     */
    public static function conversationNotFound(string $identifier, array $context = []): self
    {
        return new self(
            "Conversation not found: {$identifier}",
            404,
            null,
            array_merge($context, [
                'identifier' => $identifier,
                'reason' => 'not_found',
            ]),
        );
    }

    /**
     * Create exception for circular reference.
     *
     * @param array<string, mixed> $context
     */
    public static function circularReference(string $messageId, array $context = []): self
    {
        $exception = new self(
            "Circular reference detected in email thread: {$messageId}",
            422,
            null,
            array_merge($context, [
                'message_id' => $messageId,
                'reason' => 'circular_reference',
            ]),
        );
        $exception->retryable = false;

        return $exception;
    }

    /**
     * Create exception for thread depth exceeded.
     *
     * @param array<string, mixed> $context
     */
    public static function depthExceeded(int $depth, int $maxDepth, array $context = []): self
    {
        $exception = new self(
            "Thread depth exceeded ({$depth} > {$maxDepth})",
            422,
            null,
            array_merge($context, [
                'depth' => $depth,
                'max_depth' => $maxDepth,
                'reason' => 'depth_exceeded',
            ]),
        );
        $exception->retryable = false;

        return $exception;
    }

    /**
     * Create exception for merge conflict.
     *
     * @param array<string, mixed> $context
     */
    public static function mergeConflict(
        string $targetId,
        string $sourceId,
        string $reason,
        array $context = [],
    ): self {
        $exception = new self(
            "Cannot merge conversations: {$reason}",
            422,
            null,
            array_merge($context, [
                'target_id' => $targetId,
                'source_id' => $sourceId,
                'merge_error' => $reason,
                'reason' => 'merge_conflict',
            ]),
        );
        $exception->retryable = false;

        return $exception;
    }

    public function getSafeMessage(): string
    {
        $context = $this->getContext();
        $reason = $context['reason'] ?? 'unknown';

        return match ($reason) {
            'not_found' => 'Conversation not found',
            'circular_reference' => 'Invalid email thread structure',
            'depth_exceeded' => 'Thread too deep',
            'merge_conflict' => 'Cannot merge conversations',
            default => 'Email threading error',
        };
    }
}
