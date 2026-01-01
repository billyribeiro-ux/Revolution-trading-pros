<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Email;

use App\Exceptions\Email\InboundEmailException;
use Carbon\CarbonImmutable;
use DateTimeInterface;
use Illuminate\Support\Arr;

/**
 * Base DTO for inbound email payloads.
 *
 * Provides type-safe access to normalized email data across providers.
 *
 * @version 1.0.0
 */
readonly class InboundEmailPayload
{
    /**
     * @param string $messageId Unique message identifier
     * @param string $fromEmail Sender email address
     * @param string|null $fromName Sender display name
     * @param string $toEmail Recipient email address
     * @param string|null $toName Recipient display name
     * @param string $subject Email subject line
     * @param string|null $textBody Plain text body
     * @param string|null $htmlBody HTML body
     * @param CarbonImmutable $receivedAt When the email was received
     * @param string|null $inReplyTo In-Reply-To header value
     * @param array<string> $references References header values
     * @param array<EmailHeader> $headers All email headers
     * @param array<EmailAttachmentPayload> $attachments Attachment payloads
     * @param string|null $mailboxHash Reply-to hash for routing
     * @param float $spamScore Spam score (0-10)
     * @param string|null $spamStatus Spam status from provider
     * @param array<string, mixed> $rawPayload Original provider payload
     * @param string $provider Provider name (postmark, ses, sendgrid)
     */
    public function __construct(
        public string $messageId,
        public string $fromEmail,
        public ?string $fromName,
        public string $toEmail,
        public ?string $toName,
        public string $subject,
        public ?string $textBody,
        public ?string $htmlBody,
        public CarbonImmutable $receivedAt,
        public ?string $inReplyTo,
        public array $references,
        public array $headers,
        public array $attachments,
        public ?string $mailboxHash,
        public float $spamScore,
        public ?string $spamStatus,
        public array $rawPayload,
        public string $provider,
    ) {}

    /**
     * Get body preview (first 200 characters of text body).
     */
    public function getBodyPreview(int $length = 200): string
    {
        $body = $this->textBody ?? strip_tags($this->htmlBody ?? '');
        $body = preg_replace('/\s+/', ' ', trim($body)) ?? '';

        if (strlen($body) > $length) {
            return substr($body, 0, $length - 3) . '...';
        }

        return $body;
    }

    /**
     * Check if email has attachments.
     */
    public function hasAttachments(): bool
    {
        return count($this->attachments) > 0;
    }

    /**
     * Get attachment count.
     */
    public function attachmentCount(): int
    {
        return count($this->attachments);
    }

    /**
     * Get total attachment size in bytes.
     */
    public function totalAttachmentSize(): int
    {
        return array_reduce(
            $this->attachments,
            fn (int $total, EmailAttachmentPayload $att) => $total + $att->size,
            0,
        );
    }

    /**
     * Check if this appears to be a reply.
     */
    public function isReply(): bool
    {
        return $this->inReplyTo !== null
            || count($this->references) > 0
            || $this->mailboxHash !== null
            || str_starts_with(strtolower($this->subject), 're:');
    }

    /**
     * Get header value by name (case-insensitive).
     */
    public function getHeader(string $name): ?string
    {
        $nameLower = strtolower($name);

        foreach ($this->headers as $header) {
            if (strtolower($header->name) === $nameLower) {
                return $header->value;
            }
        }

        return null;
    }

    /**
     * Check if spam score exceeds threshold.
     */
    public function isAboveSpamThreshold(float $threshold): bool
    {
        return $this->spamScore >= $threshold;
    }

    /**
     * Get normalized subject (without Re:/Fwd: prefixes).
     */
    public function getNormalizedSubject(): string
    {
        $subject = preg_replace('/^(Re|Fwd|Fw):\s*/i', '', $this->subject) ?? $this->subject;

        return trim($subject);
    }

    /**
     * Convert to array for storage/logging.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'message_id' => $this->messageId,
            'from_email' => $this->fromEmail,
            'from_name' => $this->fromName,
            'to_email' => $this->toEmail,
            'to_name' => $this->toName,
            'subject' => $this->subject,
            'text_body' => $this->textBody,
            'html_body' => $this->htmlBody,
            'received_at' => $this->receivedAt->toIso8601String(),
            'in_reply_to' => $this->inReplyTo,
            'references' => $this->references,
            'attachment_count' => $this->attachmentCount(),
            'mailbox_hash' => $this->mailboxHash,
            'spam_score' => $this->spamScore,
            'spam_status' => $this->spamStatus,
            'provider' => $this->provider,
        ];
    }

    /**
     * Get loggable representation (sensitive data redacted).
     *
     * @return array<string, mixed>
     */
    public function toLoggable(): array
    {
        return [
            'message_id' => $this->messageId,
            'from_email' => $this->fromEmail,
            'to_email' => $this->toEmail,
            'subject' => $this->subject,
            'received_at' => $this->receivedAt->toIso8601String(),
            'is_reply' => $this->isReply(),
            'has_attachments' => $this->hasAttachments(),
            'attachment_count' => $this->attachmentCount(),
            'spam_score' => $this->spamScore,
            'provider' => $this->provider,
        ];
    }
}
