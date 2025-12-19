<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Enums\ConversationPriority;
use App\Enums\ConversationStatus;
use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\EmailThreadMapping;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * EmailThreadingService
 *
 * Enterprise-grade email threading service implementing RFC 5256 threading algorithms.
 * Handles conversation detection, thread matching, and message correlation.
 *
 * Threading Strategy (in order of priority):
 * 1. In-Reply-To header - Direct reply reference
 * 2. References header - Thread chain reference
 * 3. Mailbox hash - Reply-to address routing
 * 4. Subject + Contact - Fallback subject-based matching
 *
 * @version 1.0.0
 */
final class EmailThreadingService
{
    /**
     * Cache TTL for thread lookups (5 minutes).
     */
    private const CACHE_TTL = 300;

    /**
     * Maximum age for subject-based thread matching (7 days).
     */
    private const SUBJECT_MATCH_MAX_AGE_DAYS = 7;

    /**
     * Find or create a conversation for an incoming email.
     *
     * @param Contact $contact The contact who sent/received the email
     * @param string $messageId The RFC 2822 Message-ID
     * @param string|null $inReplyTo The In-Reply-To header value
     * @param array<string> $references The References header values
     * @param string $subject The email subject
     * @param string|null $mailboxHash The mailbox hash from address parsing
     */
    public function findOrCreateConversation(
        Contact $contact,
        string $messageId,
        ?string $inReplyTo,
        array $references,
        string $subject,
        ?string $mailboxHash = null,
    ): EmailConversation {
        // Strategy 1: Check In-Reply-To header
        if ($inReplyTo) {
            $conversation = $this->findByMessageId($inReplyTo);
            if ($conversation) {
                return $this->updateConversationForNewMessage($conversation);
            }
        }

        // Strategy 2: Check References header
        if (!empty($references)) {
            foreach ($references as $reference) {
                $conversation = $this->findByMessageId($reference);
                if ($conversation) {
                    return $this->updateConversationForNewMessage($conversation);
                }
            }
        }

        // Strategy 3: Check mailbox hash (reply+{hash}@domain.com)
        if ($mailboxHash) {
            $conversation = $this->findByMailboxHash($mailboxHash);
            if ($conversation) {
                return $this->updateConversationForNewMessage($conversation);
            }
        }

        // Strategy 4: Subject-based matching with same contact
        $normalizedSubject = EmailThreadMapping::normalizeSubject($subject);
        $conversation = $this->findBySubjectAndContact($normalizedSubject, $contact->id);
        if ($conversation) {
            return $this->updateConversationForNewMessage($conversation);
        }

        // No existing conversation found - create new one
        return $this->createConversation($contact, $subject, $mailboxHash);
    }

    /**
     * Find conversation by Message-ID header.
     */
    public function findByMessageId(string $messageId): ?EmailConversation
    {
        $cacheKey = "thread:message_id:" . md5($messageId);

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($messageId) {
            $mapping = EmailThreadMapping::where('message_id_header', $messageId)
                ->orWhere('in_reply_to_header', $messageId)
                ->first();

            return $mapping?->conversation;
        });
    }

    /**
     * Find conversation by mailbox hash.
     */
    public function findByMailboxHash(string $mailboxHash): ?EmailConversation
    {
        $cacheKey = "thread:mailbox_hash:" . md5($mailboxHash);

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($mailboxHash) {
            return EmailConversation::where('mailbox_hash', $mailboxHash)
                ->whereIn('status', ConversationStatus::activeStatuses())
                ->first();
        });
    }

    /**
     * Find conversation by normalized subject and contact within time window.
     */
    public function findBySubjectAndContact(
        string $normalizedSubject,
        int $contactId
    ): ?EmailConversation {
        $cacheKey = "thread:subject:" . md5($normalizedSubject . ":" . $contactId);

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($normalizedSubject, $contactId) {
            // Look for recent conversations with same subject and contact
            $mapping = EmailThreadMapping::where('subject_normalized', $normalizedSubject)
                ->whereHas('conversation', function ($query) use ($contactId) {
                    $query->where('contact_id', $contactId)
                        ->whereIn('status', ConversationStatus::activeStatuses())
                        ->where('created_at', '>=', now()->subDays(self::SUBJECT_MATCH_MAX_AGE_DAYS));
                })
                ->orderByDesc('created_at')
                ->first();

            return $mapping?->conversation;
        });
    }

    /**
     * Create a new conversation.
     */
    public function createConversation(
        Contact $contact,
        string $subject,
        ?string $mailboxHash = null,
    ): EmailConversation {
        $threadId = $this->generateThreadId();

        return EmailConversation::create([
            'contact_id' => $contact->id,
            'subject' => $subject,
            'thread_id' => $threadId,
            'mailbox_hash' => $mailboxHash,
            'status' => ConversationStatus::OPEN,
            'priority' => $this->detectPriority($subject),
            'channel' => 'email',
            'message_count' => 0,
            'unread_count' => 0,
            'first_message_at' => now(),
            'latest_message_at' => now(),
            'participants' => [
                [
                    'email' => $contact->email,
                    'name' => $contact->full_name,
                    'role' => 'initiator',
                    'added_at' => now()->toIso8601String(),
                ],
            ],
        ]);
    }

    /**
     * Add thread mapping for future thread detection.
     */
    public function addThreadMapping(
        EmailConversation $conversation,
        string $messageId,
        ?string $inReplyTo,
        array $references,
        string $subject,
        ?string $mailboxHash = null,
    ): EmailThreadMapping {
        $normalizedSubject = EmailThreadMapping::normalizeSubject($subject);

        $mapping = EmailThreadMapping::create([
            'conversation_id' => $conversation->id,
            'message_id_header' => $messageId,
            'in_reply_to_header' => $inReplyTo,
            'references_first' => $references[0] ?? null,
            'subject_normalized' => $normalizedSubject,
            'mailbox_hash' => $mailboxHash,
        ]);

        // Clear caches
        $this->clearThreadCaches($messageId, $mailboxHash, $normalizedSubject);

        return $mapping;
    }

    /**
     * Update conversation metadata when new message arrives.
     */
    private function updateConversationForNewMessage(
        EmailConversation $conversation
    ): EmailConversation {
        // Reopen if resolved
        if ($conversation->status === ConversationStatus::RESOLVED) {
            $conversation->update([
                'status' => $conversation->assigned_to
                    ? ConversationStatus::PENDING
                    : ConversationStatus::OPEN,
                'resolved_at' => null,
            ]);
        }

        // Update latest message timestamp
        $conversation->update([
            'latest_message_at' => now(),
        ]);

        return $conversation->fresh();
    }

    /**
     * Generate a unique thread ID.
     */
    private function generateThreadId(): string
    {
        return 'thread_' . Str::uuid()->toString();
    }

    /**
     * Detect conversation priority from subject keywords.
     */
    private function detectPriority(string $subject): ConversationPriority
    {
        $lowerSubject = strtolower($subject);

        // Urgent keywords
        $urgentKeywords = ['urgent', 'emergency', 'critical', 'asap', 'immediate'];
        foreach ($urgentKeywords as $keyword) {
            if (str_contains($lowerSubject, $keyword)) {
                return ConversationPriority::URGENT;
            }
        }

        // High priority keywords
        $highKeywords = ['important', 'priority', 'time-sensitive'];
        foreach ($highKeywords as $keyword) {
            if (str_contains($lowerSubject, $keyword)) {
                return ConversationPriority::HIGH;
            }
        }

        return ConversationPriority::NORMAL;
    }

    /**
     * Clear thread-related caches.
     */
    private function clearThreadCaches(
        string $messageId,
        ?string $mailboxHash,
        string $normalizedSubject
    ): void {
        Cache::forget("thread:message_id:" . md5($messageId));

        if ($mailboxHash) {
            Cache::forget("thread:mailbox_hash:" . md5($mailboxHash));
        }

        // Note: Subject cache is keyed by subject+contact, so we can't clear it precisely here
        // It will naturally expire after CACHE_TTL
    }

    /**
     * Merge two conversations into one.
     *
     * Useful when we later discover two conversations are actually the same thread.
     */
    public function mergeConversations(
        EmailConversation $target,
        EmailConversation $source
    ): EmailConversation {
        return DB::transaction(function () use ($target, $source) {
            // Move all messages to target conversation
            $source->messages()->update(['conversation_id' => $target->id]);

            // Move thread mappings
            $source->threadMappings()->update(['conversation_id' => $target->id]);

            // Merge participants
            $targetParticipants = $target->participants ?? [];
            $sourceParticipants = $source->participants ?? [];
            $existingEmails = array_column($targetParticipants, 'email');

            foreach ($sourceParticipants as $participant) {
                if (!in_array($participant['email'], $existingEmails, true)) {
                    $targetParticipants[] = $participant;
                }
            }

            // Merge tags
            $mergedTags = array_unique(array_merge(
                $target->tags ?? [],
                $source->tags ?? []
            ));

            // Update target conversation stats
            $target->update([
                'participants' => $targetParticipants,
                'tags' => array_values($mergedTags),
                'message_count' => $target->messages()->count(),
                'unread_count' => $target->messages()->where('is_read', false)->count(),
                'first_message_at' => min(
                    $target->first_message_at,
                    $source->first_message_at
                ),
                'latest_message_at' => max(
                    $target->latest_message_at,
                    $source->latest_message_at
                ),
            ]);

            // Delete source conversation
            $source->delete();

            return $target->fresh();
        });
    }

    /**
     * Split a message into a new conversation.
     *
     * Useful when a message was incorrectly threaded.
     */
    public function splitToNewConversation(
        EmailConversation $sourceConversation,
        string $messageId
    ): ?EmailConversation {
        $message = $sourceConversation->messages()
            ->where('id', $messageId)
            ->first();

        if (!$message) {
            return null;
        }

        return DB::transaction(function () use ($sourceConversation, $message) {
            // Create new conversation
            $newConversation = EmailConversation::create([
                'contact_id' => $sourceConversation->contact_id,
                'subject' => $message->subject,
                'thread_id' => $this->generateThreadId(),
                'status' => ConversationStatus::OPEN,
                'priority' => ConversationPriority::NORMAL,
                'channel' => 'email',
                'message_count' => 1,
                'unread_count' => $message->is_read ? 0 : 1,
                'first_message_at' => $message->created_at,
                'latest_message_at' => $message->created_at,
            ]);

            // Move message to new conversation
            $message->update(['conversation_id' => $newConversation->id]);

            // Update thread mapping
            EmailThreadMapping::where('message_id_header', $message->message_id)
                ->update(['conversation_id' => $newConversation->id]);

            // Update source conversation stats
            $sourceConversation->update([
                'message_count' => $sourceConversation->messages()->count(),
                'unread_count' => $sourceConversation->messages()->where('is_read', false)->count(),
            ]);

            return $newConversation;
        });
    }

    /**
     * Generate a reply-to address with hash for thread tracking.
     */
    public function generateReplyToAddress(EmailConversation $conversation): string
    {
        $domain = config('mail.from.address', 'noreply@example.com');
        $domain = explode('@', $domain)[1] ?? 'example.com';

        $hash = $conversation->mailbox_hash ?? $this->generateMailboxHash($conversation);

        // Update conversation with hash if not set
        if (!$conversation->mailbox_hash) {
            $conversation->update(['mailbox_hash' => $hash]);
        }

        return "reply+{$hash}@{$domain}";
    }

    /**
     * Generate a unique mailbox hash for conversation routing.
     */
    private function generateMailboxHash(EmailConversation $conversation): string
    {
        // Generate a short, URL-safe hash
        return Str::lower(Str::random(12));
    }

    /**
     * Parse mailbox hash from a reply-to address.
     */
    public function parseMailboxHash(string $email): ?string
    {
        if (preg_match('/^reply\+([a-z0-9]+)@/i', $email, $matches)) {
            return $matches[1];
        }

        return null;
    }
}
