<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Email;

use App\Exceptions\Email\InboundEmailException;
use Carbon\CarbonImmutable;
use Illuminate\Support\Arr;

/**
 * DTO for AWS SES inbound email webhook payloads.
 *
 * Handles both SNS notification format and raw SES receipt format.
 *
 * @see https://docs.aws.amazon.com/ses/latest/dg/receiving-email-notifications-contents.html
 * @version 1.0.0
 */
final readonly class SesInboundPayload
{
    private function __construct(
        private InboundEmailPayload $payload,
        private ?string $receiptId,
        private array $sesReceipt,
    ) {}

    /**
     * Parse raw SES/SNS webhook payload.
     *
     * @param array<string, mixed> $data Raw webhook payload (SNS Message content)
     * @throws InboundEmailException If required fields are missing
     */
    public static function fromWebhook(array $data): self
    {
        // Handle SNS wrapper if present
        if (isset($data['Message']) && is_string($data['Message'])) {
            $data = json_decode($data['Message'], true) ?? $data;
        }

        self::validate($data);

        $mail = $data['mail'] ?? [];
        $receipt = $data['receipt'] ?? [];
        $content = $data['content'] ?? '';

        // Parse email content if provided as raw MIME
        $parsed = self::parseContent($content, $mail);

        // Parse headers
        $headers = self::parseHeaders($mail['headers'] ?? []);

        // Extract threading headers
        $inReplyTo = self::findHeader($headers, 'In-Reply-To');
        $referencesHeader = self::findHeader($headers, 'References');
        $references = $referencesHeader
            ? array_filter(preg_split('/\s+/', $referencesHeader) ?: [])
            : [];

        // Parse date
        $timestamp = $mail['timestamp'] ?? null;
        $receivedAt = $timestamp
            ? CarbonImmutable::parse($timestamp)
            : CarbonImmutable::now();

        // Calculate spam score from SES verdict
        $spamScore = self::calculateSpamScore($receipt);

        // Create the base payload
        $payload = new InboundEmailPayload(
            messageId: $mail['messageId'] ?? self::generateMessageId(),
            fromEmail: $mail['source'] ?? $parsed['from_email'] ?? '',
            fromName: $parsed['from_name'] ?? null,
            toEmail: $receipt['recipients'][0] ?? $parsed['to_email'] ?? '',
            toName: $parsed['to_name'] ?? null,
            subject: $mail['commonHeaders']['subject'] ?? $parsed['subject'] ?? '(No Subject)',
            textBody: $parsed['text_body'] ?? null,
            htmlBody: $parsed['html_body'] ?? null,
            receivedAt: $receivedAt,
            inReplyTo: $inReplyTo,
            references: $references,
            headers: $headers,
            attachments: $parsed['attachments'] ?? [],
            mailboxHash: self::extractMailboxHash($receipt['recipients'] ?? []),
            spamScore: $spamScore,
            spamStatus: self::getSpamStatus($receipt),
            rawPayload: $data,
            provider: 'ses',
        );

        return new self(
            $payload,
            $receipt['action']['objectKey'] ?? null,
            $receipt,
        );
    }

    /**
     * Validate required fields in payload.
     *
     * @param array<string, mixed> $data
     * @throws InboundEmailException
     */
    private static function validate(array $data): void
    {
        if (!isset($data['mail'])) {
            throw new InboundEmailException(
                'Missing mail object in SES payload',
                422,
                null,
                ['provider' => 'ses'],
            );
        }
    }

    /**
     * Parse raw MIME content if provided.
     *
     * @return array<string, mixed>
     */
    private static function parseContent(string $content, array $mail): array
    {
        $result = [
            'from_email' => null,
            'from_name' => null,
            'to_email' => null,
            'to_name' => null,
            'subject' => null,
            'text_body' => null,
            'html_body' => null,
            'attachments' => [],
        ];

        // If content is empty, try to get from common headers
        if (empty($content)) {
            $commonHeaders = $mail['commonHeaders'] ?? [];
            $result['from_email'] = $mail['source'] ?? null;
            $result['subject'] = $commonHeaders['subject'] ?? null;

            if (isset($commonHeaders['from'][0])) {
                $parsed = self::parseEmailAddressString($commonHeaders['from'][0]);
                $result['from_email'] = $parsed['email'];
                $result['from_name'] = $parsed['name'];
            }

            if (isset($commonHeaders['to'][0])) {
                $parsed = self::parseEmailAddressString($commonHeaders['to'][0]);
                $result['to_email'] = $parsed['email'];
                $result['to_name'] = $parsed['name'];
            }

            return $result;
        }

        // Parse MIME content if PHP mailparse extension is available
        if (function_exists('mailparse_msg_create')) {
            return self::parseMimeContent($content);
        }

        // Fallback: basic extraction
        return self::parseContentBasic($content);
    }

    /**
     * Parse MIME content using mailparse extension.
     *
     * @return array<string, mixed>
     */
    private static function parseMimeContent(string $content): array
    {
        $result = [
            'from_email' => null,
            'from_name' => null,
            'to_email' => null,
            'to_name' => null,
            'subject' => null,
            'text_body' => null,
            'html_body' => null,
            'attachments' => [],
        ];

        $mime = mailparse_msg_create();
        mailparse_msg_parse($mime, $content);

        $structure = mailparse_msg_get_structure($mime);
        $root = mailparse_msg_get_part_data(mailparse_msg_get_part($mime, $structure[0]));

        $result['subject'] = $root['headers']['subject'] ?? null;

        if (isset($root['headers']['from'])) {
            $parsed = self::parseEmailAddressString($root['headers']['from']);
            $result['from_email'] = $parsed['email'];
            $result['from_name'] = $parsed['name'];
        }

        if (isset($root['headers']['to'])) {
            $parsed = self::parseEmailAddressString($root['headers']['to']);
            $result['to_email'] = $parsed['email'];
            $result['to_name'] = $parsed['name'];
        }

        foreach ($structure as $partId) {
            $part = mailparse_msg_get_part($mime, $partId);
            $partData = mailparse_msg_get_part_data($part);

            $contentType = $partData['content-type'] ?? 'text/plain';
            $disposition = $partData['content-disposition'] ?? '';

            if ($disposition === 'attachment' || str_contains($disposition, 'attachment')) {
                // Handle attachment
                $startPos = $partData['starting-pos-body'] ?? 0;
                $endPos = $partData['ending-pos-body'] ?? strlen($content);
                $attachmentContent = substr($content, $startPos, $endPos - $startPos);

                $result['attachments'][] = EmailAttachmentPayload::fromSes([
                    'name' => $partData['disposition-filename'] ?? 'attachment',
                    'contentType' => $contentType,
                    'content' => base64_encode($attachmentContent),
                    'size' => strlen($attachmentContent),
                ]);
            } elseif ($contentType === 'text/plain' && !$result['text_body']) {
                $startPos = $partData['starting-pos-body'] ?? 0;
                $endPos = $partData['ending-pos-body'] ?? strlen($content);
                $result['text_body'] = substr($content, $startPos, $endPos - $startPos);
            } elseif ($contentType === 'text/html' && !$result['html_body']) {
                $startPos = $partData['starting-pos-body'] ?? 0;
                $endPos = $partData['ending-pos-body'] ?? strlen($content);
                $result['html_body'] = substr($content, $startPos, $endPos - $startPos);
            }
        }

        mailparse_msg_free($mime);

        return $result;
    }

    /**
     * Basic content parsing fallback.
     *
     * @return array<string, mixed>
     */
    private static function parseContentBasic(string $content): array
    {
        $result = [
            'from_email' => null,
            'from_name' => null,
            'to_email' => null,
            'to_name' => null,
            'subject' => null,
            'text_body' => null,
            'html_body' => null,
            'attachments' => [],
        ];

        // Split headers and body
        $parts = preg_split('/\r?\n\r?\n/', $content, 2);
        $headers = $parts[0] ?? '';
        $body = $parts[1] ?? '';

        // Parse basic headers
        if (preg_match('/^Subject:\s*(.+)$/mi', $headers, $matches)) {
            $result['subject'] = trim($matches[1]);
        }
        if (preg_match('/^From:\s*(.+)$/mi', $headers, $matches)) {
            $parsed = self::parseEmailAddressString($matches[1]);
            $result['from_email'] = $parsed['email'];
            $result['from_name'] = $parsed['name'];
        }
        if (preg_match('/^To:\s*(.+)$/mi', $headers, $matches)) {
            $parsed = self::parseEmailAddressString($matches[1]);
            $result['to_email'] = $parsed['email'];
            $result['to_name'] = $parsed['name'];
        }

        // Assume plain text body for simple fallback
        $result['text_body'] = $body;

        return $result;
    }

    /**
     * Parse email address string.
     *
     * @return array{email: string, name: string|null}
     */
    private static function parseEmailAddressString(string $address): array
    {
        if (preg_match('/^"?([^"<]*)"?\s*<([^>]+)>/', $address, $matches)) {
            return [
                'email' => strtolower(trim($matches[2])),
                'name' => trim($matches[1]) ?: null,
            ];
        }

        return [
            'email' => strtolower(trim($address)),
            'name' => null,
        ];
    }

    /**
     * Parse headers array from SES format.
     *
     * @param array<array{name: string, value: string}> $data
     * @return array<EmailHeader>
     */
    private static function parseHeaders(array $data): array
    {
        return array_map(
            fn (array $header) => new EmailHeader(
                $header['name'] ?? '',
                $header['value'] ?? '',
            ),
            $data,
        );
    }

    /**
     * Find header value by name.
     *
     * @param array<EmailHeader> $headers
     */
    private static function findHeader(array $headers, string $name): ?string
    {
        foreach ($headers as $header) {
            if ($header->nameIs($name)) {
                return $header->value;
            }
        }

        return null;
    }

    /**
     * Extract mailbox hash from recipient address.
     *
     * @param array<string> $recipients
     */
    private static function extractMailboxHash(array $recipients): ?string
    {
        foreach ($recipients as $recipient) {
            // Look for reply+{hash}@ pattern
            if (preg_match('/reply\+([a-zA-Z0-9]+)@/', $recipient, $matches)) {
                return $matches[1];
            }
        }

        return null;
    }

    /**
     * Calculate spam score from SES verdicts.
     */
    private static function calculateSpamScore(array $receipt): float
    {
        $score = 0.0;

        $spamVerdict = $receipt['spamVerdict']['status'] ?? 'PASS';
        if ($spamVerdict === 'FAIL') {
            $score += 5.0;
        }

        $virusVerdict = $receipt['virusVerdict']['status'] ?? 'PASS';
        if ($virusVerdict === 'FAIL') {
            $score += 5.0;
        }

        $spfVerdict = $receipt['spfVerdict']['status'] ?? 'PASS';
        if ($spfVerdict === 'FAIL') {
            $score += 2.0;
        }

        $dkimVerdict = $receipt['dkimVerdict']['status'] ?? 'PASS';
        if ($dkimVerdict === 'FAIL') {
            $score += 2.0;
        }

        $dmarcVerdict = $receipt['dmarcVerdict']['status'] ?? 'PASS';
        if ($dmarcVerdict === 'FAIL') {
            $score += 1.0;
        }

        return min($score, 10.0);
    }

    /**
     * Get spam status string from SES verdicts.
     */
    private static function getSpamStatus(array $receipt): string
    {
        $spamVerdict = $receipt['spamVerdict']['status'] ?? 'PASS';
        $virusVerdict = $receipt['virusVerdict']['status'] ?? 'PASS';

        if ($virusVerdict === 'FAIL') {
            return 'VIRUS';
        }
        if ($spamVerdict === 'FAIL') {
            return 'SPAM';
        }

        return 'OK';
    }

    /**
     * Generate a fallback message ID.
     */
    private static function generateMessageId(): string
    {
        return sprintf(
            '<%s.%s@ses.amazonaws.com>',
            now()->timestamp,
            bin2hex(random_bytes(8)),
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
     * Get the SES receipt ID (S3 object key if stored).
     */
    public function getReceiptId(): ?string
    {
        return $this->receiptId;
    }

    /**
     * Get SPF verdict.
     */
    public function getSpfVerdict(): string
    {
        return $this->sesReceipt['spfVerdict']['status'] ?? 'GRAY';
    }

    /**
     * Get DKIM verdict.
     */
    public function getDkimVerdict(): string
    {
        return $this->sesReceipt['dkimVerdict']['status'] ?? 'GRAY';
    }

    /**
     * Get DMARC verdict.
     */
    public function getDmarcVerdict(): string
    {
        return $this->sesReceipt['dmarcVerdict']['status'] ?? 'GRAY';
    }

    /**
     * Get virus verdict.
     */
    public function getVirusVerdict(): string
    {
        return $this->sesReceipt['virusVerdict']['status'] ?? 'GRAY';
    }

    /**
     * Get spam verdict.
     */
    public function getSpamVerdict(): string
    {
        return $this->sesReceipt['spamVerdict']['status'] ?? 'GRAY';
    }

    /**
     * Check if email passed all security checks.
     */
    public function passedSecurityChecks(): bool
    {
        return $this->getSpfVerdict() === 'PASS'
            && $this->getDkimVerdict() === 'PASS'
            && $this->getVirusVerdict() === 'PASS'
            && $this->getSpamVerdict() === 'PASS';
    }
}
