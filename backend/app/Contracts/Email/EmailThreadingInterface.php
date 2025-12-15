<?php

declare(strict_types=1);

namespace App\Contracts\Email;

use App\Models\Contact;
use App\Models\EmailConversation;

/**
 * Contract for email threading service.
 *
 * Provides conversation threading based on email headers
 * and various fallback strategies.
 *
 * @version 1.0.0
 */
interface EmailThreadingInterface
{
    /**
     * Find or create a conversation for an email.
     *
     * @param Contact $contact The sender contact
     * @param string $messageId RFC 5322 Message-ID
     * @param string|null $inReplyTo In-Reply-To header value
     * @param array<string> $references References header values
     * @param string $subject Email subject
     * @param string|null $mailboxHash Reply-to mailbox hash
     * @return EmailConversation The existing or new conversation
     */
    public function findOrCreateConversation(
        Contact $contact,
        string $messageId,
        ?string $inReplyTo,
        array $references,
        string $subject,
        ?string $mailboxHash = null,
    ): EmailConversation;

    /**
     * Merge two conversations into one.
     *
     * @param EmailConversation $target The target conversation (kept)
     * @param EmailConversation $source The source conversation (merged/deleted)
     * @return EmailConversation The merged conversation
     * @throws \App\Exceptions\Email\ThreadingException On merge failure
     */
    public function mergeConversations(
        EmailConversation $target,
        EmailConversation $source,
    ): EmailConversation;

    /**
     * Split a conversation at a specific message.
     *
     * @param EmailConversation $conversation The conversation to split
     * @param string $messageId Message ID to split at
     * @return EmailConversation The new conversation containing the split messages
     * @throws \App\Exceptions\Email\ThreadingException On split failure
     */
    public function splitConversation(
        EmailConversation $conversation,
        string $messageId,
    ): EmailConversation;

    /**
     * Get the threading strategy used for a message.
     *
     * @param string $messageId The message ID
     * @return string The strategy name (in-reply-to, references, mailbox-hash, subject, new)
     */
    public function getThreadingStrategy(string $messageId): string;
}
