<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Email;

use App\Exceptions\Email\InboundEmailException;
use Carbon\CarbonImmutable;
use Illuminate\Support\Arr;

/**
 * DTO for Postmark inbound email webhook payloads.
 *
 * Provides type-safe parsing and validation of Postmark webhook data.
 *
 * @see https://postmarkapp.com/developer/webhooks/inbound-webhook
 * @version 1.0.0
 */
final readonly class PostmarkInboundPayload
{
    private function __construct(
        private InboundEmailPayload $payload,
    ) {}

    /**
     * Parse raw Postmark webhook payload.
     *
     * @param array<string, mixed> $data Raw webhook payload
     * @throws InboundEmailException If required fields are missing
     */
    public static function fromWebhook(array $data): self
    {
        self::validate($data);

        // Parse From address
        $from = self::parseEmailAddress($data['From'] ?? $data['FromFull']['Email'] ?? '');

        // Parse To address
        $to = self::parseEmailAddress($data['To'] ?? $data['ToFull'][0]['Email'] ?? '');

        // Parse headers
        $headers = self::parseHeaders($data['Headers'] ?? []);

        // Parse attachments
        $attachments = self::parseAttachments($data['Attachments'] ?? []);

        // Extract references from header
        $references = [];
        $inReplyTo = $data['Headers'] ?? [];
        foreach ($inReplyTo as $header) {
            if (($header['Name'] ?? '') === 'References') {
                $references = array_filter(
                    preg_split('/\s+/', $header['Value'] ?? '') ?: [],
                );
            }
        }

        // Get In-Reply-To from headers if not provided directly
        $inReplyToValue = null;
        foreach ($data['Headers'] ?? [] as $header) {
            if (($header['Name'] ?? '') === 'In-Reply-To') {
                $inReplyToValue = trim($header['Value'] ?? '');
                break;
            }
        }

        // Parse date
        $receivedAt = CarbonImmutable::parse($data['Date'] ?? now());

        // Create the base payload
        $payload = new InboundEmailPayload(
            messageId: $data['MessageID'] ?? self::generateMessageId(),
            fromEmail: $from['email'],
            fromName: $from['name'],
            toEmail: $to['email'],
            toName: $to['name'],
            subject: $data['Subject'] ?? '(No Subject)',
            textBody: $data['TextBody'] ?? null,
            htmlBody: $data['HtmlBody'] ?? null,
            receivedAt: $receivedAt,
            inReplyTo: $inReplyToValue,
            references: $references,
            headers: $headers,
            attachments: $attachments,
            mailboxHash: $data['MailboxHash'] ?? null,
            spamScore: (float) ($data['SpamScore'] ?? 0),
            spamStatus: $data['SpamStatus'] ?? null,
            rawPayload: $data,
            provider: 'postmark',
        );

        return new self($payload);
    }

    /**
     * Validate required fields in payload.
     *
     * @param array<string, mixed> $data
     * @throws InboundEmailException
     */
    private static function validate(array $data): void
    {
        $requiredFields = ['From', 'Subject'];

        foreach ($requiredFields as $field) {
            // Also check alternative formats
            if (!isset($data[$field]) && !isset($data["{$field}Full"])) {
                throw new InboundEmailException(
                    "Missing required field: {$field}",
                    422,
                    null,
                    ['field' => $field, 'provider' => 'postmark'],
                );
            }
        }

        // Must have either To or OriginalRecipient
        if (!isset($data['To']) && !isset($data['ToFull']) && !isset($data['OriginalRecipient'])) {
            throw new InboundEmailException(
                'Missing recipient information',
                422,
                null,
                ['provider' => 'postmark'],
            );
        }
    }

    /**
     * Parse email address string into components.
     *
     * @return array{email: string, name: string|null}
     */
    private static function parseEmailAddress(string $address): array
    {
        // Handle format: "Name" <email@example.com>
        if (preg_match('/^"?([^"<]*)"?\s*<([^>]+)>/', $address, $matches)) {
            return [
                'email' => strtolower(trim($matches[2])),
                'name' => trim($matches[1]) ?: null,
            ];
        }

        // Handle plain email format
        return [
            'email' => strtolower(trim($address)),
            'name' => null,
        ];
    }

    /**
     * Parse headers array.
     *
     * @param array<array{Name: string, Value: string}> $data
     * @return array<EmailHeader>
     */
    private static function parseHeaders(array $data): array
    {
        return array_map(
            fn (array $header) => EmailHeader::fromArray($header),
            $data,
        );
    }

    /**
     * Parse attachments array.
     *
     * @param array<array<string, mixed>> $data
     * @return array<EmailAttachmentPayload>
     */
    private static function parseAttachments(array $data): array
    {
        return array_map(
            fn (array $attachment) => EmailAttachmentPayload::fromPostmark($attachment),
            $data,
        );
    }

    /**
     * Generate a fallback message ID.
     */
    private static function generateMessageId(): string
    {
        return sprintf(
            '<%s.%s@%s>',
            now()->timestamp,
            bin2hex(random_bytes(8)),
            config('app.domain', 'local'),
        );
    }

    /**
     * Get the normalized payload.
     */
    public function getPayload(): InboundEmailPayload
    {
        return $this->payload;
    }

    /**
     * Get original recipient (before forwarding/alias expansion).
     */
    public function getOriginalRecipient(): ?string
    {
        return $this->payload->rawPayload['OriginalRecipient'] ?? null;
    }

    /**
     * Get CC recipients.
     *
     * @return array<array{email: string, name: string|null}>
     */
    public function getCcRecipients(): array
    {
        $cc = $this->payload->rawPayload['CcFull'] ?? [];

        return array_map(
            fn (array $recipient) => [
                'email' => $recipient['Email'] ?? '',
                'name' => $recipient['Name'] ?? null,
            ],
            $cc,
        );
    }

    /**
     * Get BCC recipients.
     *
     * @return array<array{email: string, name: string|null}>
     */
    public function getBccRecipients(): array
    {
        $bcc = $this->payload->rawPayload['BccFull'] ?? [];

        return array_map(
            fn (array $recipient) => [
                'email' => $recipient['Email'] ?? '',
                'name' => $recipient['Name'] ?? null,
            ],
            $bcc,
        );
    }

    /**
     * Get Reply-To address.
     */
    public function getReplyTo(): ?string
    {
        return $this->payload->rawPayload['ReplyTo'] ?? null;
    }

    /**
     * Get stripped text reply (without quoted text).
     */
    public function getStrippedTextReply(): ?string
    {
        return $this->payload->rawPayload['StrippedTextReply'] ?? null;
    }

    /**
     * Check if email is an auto-reply.
     */
    public function isAutoReply(): bool
    {
        // Check Auto-Submitted header
        foreach ($this->payload->headers as $header) {
            if ($header->nameIs('Auto-Submitted') && $header->value !== 'no') {
                return true;
            }
            if ($header->nameIs('X-Auto-Response-Suppress')) {
                return true;
            }
        }

        return false;
    }
}
