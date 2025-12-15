<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Enums\ConversationPriority;
use App\Enums\ConversationStatus;
use App\Enums\EmailDirection;
use App\Enums\EmailMessageStatus;
use App\Enums\SecurityVerdict;
use App\Events\InboundEmailReceived;
use App\Models\Contact;
use App\Models\CrmActivity;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use App\Models\EmailThreadMapping;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Throwable;

/**
 * InboundEmailService
 *
 * Enterprise-grade service for processing inbound emails from webhook providers.
 * Handles contact resolution, conversation threading, spam analysis, and CRM integration.
 *
 * Features:
 * - Multi-provider support (Postmark, SES, SendGrid)
 * - Automatic contact resolution and creation
 * - Intelligent conversation threading
 * - Spam and security analysis
 * - Attachment processing
 * - CRM activity logging
 * - Event-driven automation triggers
 *
 * @version 1.0.0
 */
final class InboundEmailService
{
    /**
     * Spam score threshold for auto-marking as spam.
     */
    private const SPAM_THRESHOLD = 5.0;

    /**
     * Maximum processing time in seconds before timeout.
     */
    private const PROCESSING_TIMEOUT = 30;

    public function __construct(
        private readonly EmailThreadingService $threadingService,
        private readonly EmailAttachmentService $attachmentService,
        private readonly AuditService $auditService,
    ) {}

    /**
     * Process inbound email from Postmark webhook.
     *
     * @param array{
     *     From: string,
     *     FromName?: string,
     *     FromFull?: array{Email: string, Name: string},
     *     To: string,
     *     ToFull?: array<array{Email: string, Name: string}>,
     *     Cc?: string,
     *     CcFull?: array<array{Email: string, Name: string}>,
     *     Bcc?: string,
     *     BccFull?: array<array{Email: string, Name: string}>,
     *     ReplyTo?: string,
     *     Subject: string,
     *     MessageID: string,
     *     Date: string,
     *     MailboxHash?: string,
     *     TextBody?: string,
     *     HtmlBody?: string,
     *     StrippedTextReply?: string,
     *     Tag?: string,
     *     Headers?: array<array{Name: string, Value: string}>,
     *     Attachments?: array<array{Name: string, Content: string, ContentType: string, ContentLength: int, ContentID?: string}>,
     * } $payload
     *
     * @throws \App\Exceptions\InboundEmailProcessingException
     */
    public function processPostmarkWebhook(array $payload): EmailMessage
    {
        $startTime = microtime(true);

        $this->auditService->startTiming();

        try {
            return DB::transaction(function () use ($payload, $startTime): EmailMessage {
                // 1. Extract sender information
                $senderEmail = $this->extractEmail($payload['From']);
                $senderName = $payload['FromName']
                    ?? $payload['FromFull']['Name']
                    ?? $this->extractName($payload['From']);

                // 2. Find or create contact
                $contact = $this->resolveContact($senderEmail, $senderName);

                // 3. Extract headers for threading
                $headers = $this->normalizeHeaders($payload['Headers'] ?? []);
                $messageId = $payload['MessageID'];
                $inReplyTo = $headers['in-reply-to'] ?? null;
                $references = $this->parseReferences($headers['references'] ?? null);
                $mailboxHash = $payload['MailboxHash'] ?? null;

                // 4. Find or create conversation
                $conversation = $this->threadingService->findOrCreateConversation(
                    contact: $contact,
                    messageId: $messageId,
                    inReplyTo: $inReplyTo,
                    references: $references,
                    subject: $payload['Subject'],
                    mailboxHash: $mailboxHash,
                );

                // 5. Extract authentication results
                $authResults = $this->extractAuthenticationResults($headers);

                // 6. Calculate spam score
                $spamScore = $this->calculateSpamScore($payload, $headers);

                // 7. Create the email message
                $message = EmailMessage::create([
                    'conversation_id' => $conversation->id,
                    'message_id' => $messageId,
                    'provider_message_id' => $payload['MessageID'],
                    'in_reply_to' => $inReplyTo,
                    'references' => $references,
                    'from_email' => $senderEmail,
                    'from_name' => $senderName,
                    'to_email' => $this->extractEmail($payload['To']),
                    'to_name' => $this->extractName($payload['To']),
                    'cc' => $this->parseRecipientList($payload['CcFull'] ?? []),
                    'bcc' => $this->parseRecipientList($payload['BccFull'] ?? []),
                    'reply_to_email' => $payload['ReplyTo'] ?? null,
                    'subject' => $payload['Subject'],
                    'body_text' => $payload['TextBody'] ?? null,
                    'body_html' => $payload['HtmlBody'] ?? null,
                    'direction' => EmailDirection::INBOUND,
                    'status' => $this->determineInitialStatus($spamScore, $authResults),
                    'spam_score' => $spamScore,
                    'spam_verdict' => $spamScore >= self::SPAM_THRESHOLD
                        ? SecurityVerdict::FAIL
                        : SecurityVerdict::PASS,
                    'spf_verdict' => SecurityVerdict::tryFrom($authResults['spf'] ?? 'none'),
                    'dkim_verdict' => SecurityVerdict::tryFrom($authResults['dkim'] ?? 'none'),
                    'dmarc_verdict' => SecurityVerdict::tryFrom($authResults['dmarc'] ?? 'none'),
                    'authentication_results' => $authResults,
                    'headers' => $payload['Headers'] ?? [],
                    'size_bytes' => $this->calculateEmailSize($payload),
                    'attachment_count' => count($payload['Attachments'] ?? []),
                    'provider' => 'postmark',
                    'provider_data' => [
                        'tag' => $payload['Tag'] ?? null,
                        'mailbox_hash' => $mailboxHash,
                        'date' => $payload['Date'] ?? null,
                    ],
                    'received_at' => now(),
                ]);

                // 8. Process attachments
                if (!empty($payload['Attachments'])) {
                    $this->attachmentService->processAttachments(
                        $message,
                        $payload['Attachments']
                    );
                }

                // 9. Update conversation participants
                $this->updateConversationParticipants($conversation, $payload);

                // 10. Create CRM activity
                $this->createCrmActivity($contact, $message, $conversation);

                // 11. Update contact activity timestamps
                $this->updateContactActivity($contact);

                // 12. Calculate processing time
                $processingTimeMs = (int) ((microtime(true) - $startTime) * 1000);
                $message->update([
                    'processing_time_ms' => $processingTimeMs,
                    'processed_at' => now(),
                    'status' => $message->status === EmailMessageStatus::PENDING
                        ? EmailMessageStatus::PROCESSED
                        : $message->status,
                ]);

                // 13. Add thread mapping for future thread detection
                $this->threadingService->addThreadMapping(
                    conversation: $conversation,
                    messageId: $messageId,
                    inReplyTo: $inReplyTo,
                    references: $references,
                    subject: $payload['Subject'],
                    mailboxHash: $mailboxHash,
                );

                // 14. Log audit event
                $this->auditService->log(
                    action: 'inbound_email_received',
                    entityType: 'email_message',
                    entityId: $message->id,
                    entityName: $payload['Subject'],
                    metadata: [
                        'from' => $senderEmail,
                        'to' => $payload['To'],
                        'conversation_id' => $conversation->id,
                        'contact_id' => $contact->id,
                        'spam_score' => $spamScore,
                        'attachment_count' => count($payload['Attachments'] ?? []),
                        'processing_time_ms' => $processingTimeMs,
                    ]
                );

                // 15. Fire event for automation triggers
                event(new InboundEmailReceived($message, $contact, $conversation));

                return $message;
            });
        } catch (Throwable $e) {
            Log::error('Inbound email processing failed', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
                'from' => $payload['From'] ?? 'unknown',
                'subject' => $payload['Subject'] ?? 'unknown',
                'message_id' => $payload['MessageID'] ?? 'unknown',
            ]);

            throw $e;
        }
    }

    /**
     * Process inbound email from AWS SES webhook.
     *
     * @param array $payload SES notification payload
     */
    public function processSesWebhook(array $payload): EmailMessage
    {
        // SES sends notifications via SNS
        $notification = $payload['Message'] ?? $payload;

        if (is_string($notification)) {
            $notification = json_decode($notification, true);
        }

        // Extract receipt info
        $mail = $notification['mail'] ?? [];
        $receipt = $notification['receipt'] ?? [];

        // Transform to normalized format
        $normalized = [
            'From' => $mail['source'] ?? '',
            'To' => implode(', ', $mail['destination'] ?? []),
            'Subject' => $mail['commonHeaders']['subject'] ?? '',
            'MessageID' => $mail['messageId'] ?? Str::uuid()->toString(),
            'Date' => $mail['timestamp'] ?? now()->toIso8601String(),
            'Headers' => $this->transformSesHeaders($mail['headers'] ?? []),
            'TextBody' => $notification['content'] ?? null,
            'Attachments' => [], // SES requires S3 for attachments
        ];

        // Add authentication results from SES receipt
        if (isset($receipt['spfVerdict'])) {
            $normalized['Headers'][] = [
                'Name' => 'X-SES-Spam-Verdict',
                'Value' => $receipt['spamVerdict']['status'] ?? 'GRAY',
            ];
            $normalized['Headers'][] = [
                'Name' => 'X-SES-Virus-Verdict',
                'Value' => $receipt['virusVerdict']['status'] ?? 'GRAY',
            ];
        }

        return $this->processPostmarkWebhook($normalized);
    }

    /**
     * Resolve or create a contact from email address.
     */
    private function resolveContact(string $email, ?string $name): Contact
    {
        $contact = Contact::where('email', strtolower($email))->first();

        if ($contact) {
            return $contact;
        }

        // Check if auto-creation is enabled
        $autoCreate = DB::table('inbound_email_settings')
            ->where('key', 'auto_create_contacts')
            ->value('value') ?? 'true';

        if ($autoCreate !== 'true') {
            // Create a minimal contact record
            $contact = Contact::create([
                'email' => strtolower($email),
                'first_name' => $this->extractFirstName($name),
                'last_name' => $this->extractLastName($name),
                'source' => 'inbound_email',
                'lifecycle_stage' => 'subscriber',
            ]);
        } else {
            // Get default lifecycle stage
            $lifecycleStage = DB::table('inbound_email_settings')
                ->where('key', 'default_contact_lifecycle_stage')
                ->value('value') ?? 'lead';

            $contact = Contact::create([
                'email' => strtolower($email),
                'first_name' => $this->extractFirstName($name),
                'last_name' => $this->extractLastName($name),
                'source' => 'inbound_email',
                'source_details' => json_encode(['created_from' => 'inbound_email']),
                'lifecycle_stage' => $lifecycleStage,
                'first_touch_channel' => 'email',
                'last_activity_at' => now(),
            ]);
        }

        return $contact;
    }

    /**
     * Extract email address from "Name <email@example.com>" format.
     */
    private function extractEmail(string $from): string
    {
        if (preg_match('/<([^>]+)>/', $from, $matches)) {
            return strtolower(trim($matches[1]));
        }

        return strtolower(trim($from));
    }

    /**
     * Extract name from "Name <email@example.com>" format.
     */
    private function extractName(string $from): ?string
    {
        if (preg_match('/^([^<]+)</', $from, $matches)) {
            $name = trim($matches[1], ' "\'');
            return $name ?: null;
        }

        return null;
    }

    /**
     * Extract first name from full name.
     */
    private function extractFirstName(?string $fullName): ?string
    {
        if (empty($fullName)) {
            return null;
        }

        $parts = explode(' ', trim($fullName), 2);

        return $parts[0] ?? null;
    }

    /**
     * Extract last name from full name.
     */
    private function extractLastName(?string $fullName): ?string
    {
        if (empty($fullName)) {
            return null;
        }

        $parts = explode(' ', trim($fullName), 2);

        return $parts[1] ?? null;
    }

    /**
     * Normalize email headers to lowercase keys.
     *
     * @param array<array{Name: string, Value: string}> $headers
     * @return array<string, string>
     */
    private function normalizeHeaders(array $headers): array
    {
        $normalized = [];

        foreach ($headers as $header) {
            if (isset($header['Name'], $header['Value'])) {
                $normalized[strtolower($header['Name'])] = $header['Value'];
            }
        }

        return $normalized;
    }

    /**
     * Parse References header into array of message IDs.
     *
     * @return array<string>
     */
    private function parseReferences(?string $references): array
    {
        if (empty($references)) {
            return [];
        }

        // References header contains space-separated message IDs
        preg_match_all('/<[^>]+>/', $references, $matches);

        return $matches[0] ?? [];
    }

    /**
     * Parse recipient list from full recipient array.
     *
     * @param array<array{Email: string, Name?: string}> $recipients
     * @return array<array{email: string, name: string|null}>
     */
    private function parseRecipientList(array $recipients): array
    {
        return array_map(
            fn (array $r) => [
                'email' => strtolower($r['Email'] ?? $r['email'] ?? ''),
                'name' => $r['Name'] ?? $r['name'] ?? null,
            ],
            $recipients
        );
    }

    /**
     * Extract authentication results from headers.
     *
     * @return array{spf: string, dkim: string, dmarc: string}
     */
    private function extractAuthenticationResults(array $headers): array
    {
        $results = [
            'spf' => 'none',
            'dkim' => 'none',
            'dmarc' => 'none',
        ];

        // Parse Authentication-Results header
        $authResults = $headers['authentication-results'] ?? '';

        if (stripos($authResults, 'spf=pass') !== false) {
            $results['spf'] = 'pass';
        } elseif (stripos($authResults, 'spf=fail') !== false) {
            $results['spf'] = 'fail';
        } elseif (stripos($authResults, 'spf=') !== false) {
            $results['spf'] = 'neutral';
        }

        if (stripos($authResults, 'dkim=pass') !== false) {
            $results['dkim'] = 'pass';
        } elseif (stripos($authResults, 'dkim=fail') !== false) {
            $results['dkim'] = 'fail';
        } elseif (stripos($authResults, 'dkim=') !== false) {
            $results['dkim'] = 'neutral';
        }

        if (stripos($authResults, 'dmarc=pass') !== false) {
            $results['dmarc'] = 'pass';
        } elseif (stripos($authResults, 'dmarc=fail') !== false) {
            $results['dmarc'] = 'fail';
        } elseif (stripos($authResults, 'dmarc=') !== false) {
            $results['dmarc'] = 'neutral';
        }

        // Also check Postmark-specific headers
        if (isset($headers['x-spam-score'])) {
            $results['spam_score'] = (float) $headers['x-spam-score'];
        }

        return $results;
    }

    /**
     * Calculate spam score from payload and headers.
     */
    private function calculateSpamScore(array $payload, array $headers): float
    {
        $score = 0.0;

        // Use provider's spam score if available
        if (isset($headers['x-spam-score'])) {
            return (float) $headers['x-spam-score'];
        }

        // Basic heuristic scoring
        $subject = $payload['Subject'] ?? '';
        $body = ($payload['TextBody'] ?? '') . ($payload['HtmlBody'] ?? '');

        // Check for spam keywords
        $spamKeywords = [
            'viagra', 'cialis', 'lottery', 'winner', 'prince',
            'million dollars', 'free money', 'act now', 'limited time',
            'click here', 'unsubscribe', 'opt out',
        ];

        foreach ($spamKeywords as $keyword) {
            if (stripos($subject . $body, $keyword) !== false) {
                $score += 1.0;
            }
        }

        // Check for excessive caps
        $capsRatio = strlen(preg_replace('/[^A-Z]/', '', $subject)) / max(1, strlen($subject));
        if ($capsRatio > 0.5) {
            $score += 2.0;
        }

        // Check for many exclamation marks
        $exclamations = substr_count($subject, '!');
        if ($exclamations > 2) {
            $score += 1.0 * min(3, $exclamations - 2);
        }

        // Check authentication
        $authResults = $this->extractAuthenticationResults($headers);
        if ($authResults['spf'] === 'fail') {
            $score += 2.0;
        }
        if ($authResults['dkim'] === 'fail') {
            $score += 2.0;
        }

        return min(10.0, $score);
    }

    /**
     * Determine initial message status based on spam score and auth results.
     */
    private function determineInitialStatus(float $spamScore, array $authResults): EmailMessageStatus
    {
        if ($spamScore >= self::SPAM_THRESHOLD) {
            return EmailMessageStatus::SPAM;
        }

        if ($authResults['spf'] === 'fail' && $authResults['dkim'] === 'fail') {
            return EmailMessageStatus::QUARANTINED;
        }

        return EmailMessageStatus::PENDING;
    }

    /**
     * Calculate approximate email size in bytes.
     */
    private function calculateEmailSize(array $payload): int
    {
        $size = 0;

        $size += strlen($payload['Subject'] ?? '');
        $size += strlen($payload['TextBody'] ?? '');
        $size += strlen($payload['HtmlBody'] ?? '');

        foreach ($payload['Attachments'] ?? [] as $attachment) {
            $size += $attachment['ContentLength'] ?? 0;
        }

        return $size;
    }

    /**
     * Update conversation participants from email headers.
     */
    private function updateConversationParticipants(
        EmailConversation $conversation,
        array $payload
    ): void {
        $participants = $conversation->participants ?? [];
        $existingEmails = array_column($participants, 'email');

        // Add sender
        $senderEmail = $this->extractEmail($payload['From']);
        if (!in_array($senderEmail, $existingEmails, true)) {
            $participants[] = [
                'email' => $senderEmail,
                'name' => $this->extractName($payload['From']),
                'role' => 'sender',
                'added_at' => now()->toIso8601String(),
            ];
        }

        // Add CC recipients
        foreach ($payload['CcFull'] ?? [] as $cc) {
            $ccEmail = strtolower($cc['Email'] ?? '');
            if ($ccEmail && !in_array($ccEmail, $existingEmails, true)) {
                $participants[] = [
                    'email' => $ccEmail,
                    'name' => $cc['Name'] ?? null,
                    'role' => 'cc',
                    'added_at' => now()->toIso8601String(),
                ];
            }
        }

        $conversation->update(['participants' => $participants]);
    }

    /**
     * Create CRM activity for the inbound email.
     */
    private function createCrmActivity(
        Contact $contact,
        EmailMessage $message,
        EmailConversation $conversation
    ): void {
        CrmActivity::create([
            'subject_type' => 'contact',
            'subject_id' => $contact->id,
            'type' => 'email_received',
            'title' => "Email received: {$message->subject}",
            'description' => $message->body_preview,
            'properties' => [
                'message_id' => $message->id,
                'conversation_id' => $conversation->id,
                'from' => $message->from_email,
                'subject' => $message->subject,
                'attachment_count' => $message->attachment_count,
            ],
            'occurred_at' => $message->received_at,
        ]);
    }

    /**
     * Update contact activity timestamps.
     */
    private function updateContactActivity(Contact $contact): void
    {
        $contact->update([
            'last_activity_at' => now(),
            'last_email_received_at' => now(),
        ]);

        $contact->increment('activities_count');
    }

    /**
     * Transform SES headers to Postmark format.
     *
     * @return array<array{Name: string, Value: string}>
     */
    private function transformSesHeaders(array $headers): array
    {
        return array_map(
            fn (array $h) => [
                'Name' => $h['name'] ?? '',
                'Value' => $h['value'] ?? '',
            ],
            $headers
        );
    }
}
