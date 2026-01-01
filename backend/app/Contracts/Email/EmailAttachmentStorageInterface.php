<?php

declare(strict_types=1);

namespace App\Contracts\Email;

use App\DataTransferObjects\Email\EmailAttachmentPayload;
use App\Models\EmailAttachment;
use App\Models\EmailMessage;

/**
 * Contract for email attachment storage.
 *
 * Handles secure storage, validation, and retrieval of email attachments.
 *
 * @version 1.0.0
 */
interface EmailAttachmentStorageInterface
{
    /**
     * Store an attachment for an email message.
     *
     * @param EmailMessage $message The parent message
     * @param EmailAttachmentPayload $payload The attachment payload
     * @return EmailAttachment The stored attachment record
     * @throws \App\Exceptions\Email\AttachmentException On storage failure
     */
    public function store(EmailMessage $message, EmailAttachmentPayload $payload): EmailAttachment;

    /**
     * Store multiple attachments for a message.
     *
     * @param EmailMessage $message The parent message
     * @param array<EmailAttachmentPayload> $payloads The attachment payloads
     * @return array<EmailAttachment> The stored attachment records
     * @throws \App\Exceptions\Email\AttachmentException On storage failure
     */
    public function storeMany(EmailMessage $message, array $payloads): array;

    /**
     * Validate an attachment before storage.
     *
     * @param EmailAttachmentPayload $payload The attachment to validate
     * @return bool True if valid
     * @throws \App\Exceptions\Email\AttachmentException If validation fails
     */
    public function validate(EmailAttachmentPayload $payload): bool;

    /**
     * Generate a signed download URL for an attachment.
     *
     * @param EmailAttachment $attachment The attachment
     * @param int $expiresIn Expiration time in seconds
     * @return string|null The signed URL, or null if not available
     */
    public function getDownloadUrl(EmailAttachment $attachment, int $expiresIn = 3600): ?string;

    /**
     * Delete an attachment from storage.
     *
     * @param EmailAttachment $attachment The attachment to delete
     * @return bool True if deleted
     */
    public function delete(EmailAttachment $attachment): bool;

    /**
     * Scan an attachment for viruses.
     *
     * @param EmailAttachment $attachment The attachment to scan
     * @return bool True if clean, false if infected
     */
    public function scanForVirus(EmailAttachment $attachment): bool;
}
