<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * EmailThreadMapping Model
 *
 * Maps email headers (Message-ID, In-Reply-To, References) to conversations
 * for accurate thread detection.
 *
 * @property int $id
 * @property string $conversation_id
 * @property string $message_id_header RFC 2822 Message-ID
 * @property string|null $in_reply_to_header In-Reply-To header
 * @property string|null $references_first First message in References chain
 * @property string $subject_normalized Normalized subject for matching
 * @property string|null $mailbox_hash Mailbox hash for routing
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 *
 * @property-read EmailConversation $conversation
 *
 * @version 1.0.0
 */
class EmailThreadMapping extends Model
{
    /**
     * The table associated with the model.
     */
    protected $table = 'email_thread_mappings';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'conversation_id',
        'message_id_header',
        'in_reply_to_header',
        'references_first',
        'subject_normalized',
        'mailbox_hash',
    ];

    /**
     * Get the conversation.
     */
    public function conversation(): BelongsTo
    {
        return $this->belongsTo(EmailConversation::class);
    }

    /**
     * Normalize subject line for thread matching.
     *
     * Removes Re:, Fwd:, etc. prefixes and normalizes whitespace.
     */
    public static function normalizeSubject(string $subject): string
    {
        // Remove common reply/forward prefixes (case insensitive)
        $patterns = [
            '/^(re|fw|fwd|aw|wg|sv|vs|ref|odp|res|enc|tr):\s*/i',
            '/^\[.*?\]\s*/',  // Remove [list-name] prefixes
        ];

        $normalized = $subject;

        // Apply patterns repeatedly to handle nested prefixes
        do {
            $previous = $normalized;
            foreach ($patterns as $pattern) {
                $normalized = preg_replace($pattern, '', $normalized);
            }
        } while ($normalized !== $previous);

        // Normalize whitespace
        $normalized = preg_replace('/\s+/', ' ', $normalized);

        return trim($normalized);
    }
}
