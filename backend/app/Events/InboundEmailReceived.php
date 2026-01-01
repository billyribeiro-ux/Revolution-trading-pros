<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * InboundEmailReceived Event
 *
 * Fired when an inbound email is successfully processed.
 * Can be used to trigger automation workflows, notifications, etc.
 *
 * @version 1.0.0
 */
class InboundEmailReceived implements ShouldBroadcast
{
    use Dispatchable;
    use InteractsWithSockets;
    use SerializesModels;

    /**
     * Create a new event instance.
     */
    public function __construct(
        public readonly EmailMessage $message,
        public readonly Contact $contact,
        public readonly EmailConversation $conversation,
    ) {}

    /**
     * Get the channels the event should broadcast on.
     *
     * @return array<\Illuminate\Broadcasting\Channel>
     */
    public function broadcastOn(): array
    {
        $channels = [
            new PrivateChannel("contact.{$this->contact->id}"),
            new PrivateChannel("conversation.{$this->conversation->id}"),
        ];

        // Broadcast to assigned user if any
        if ($this->conversation->assigned_to) {
            $channels[] = new PrivateChannel("user.{$this->conversation->assigned_to}");
        }

        return $channels;
    }

    /**
     * Get the data to broadcast.
     *
     * @return array<string, mixed>
     */
    public function broadcastWith(): array
    {
        return [
            'message' => [
                'id' => $this->message->id,
                'subject' => $this->message->subject,
                'from_email' => $this->message->from_email,
                'from_name' => $this->message->from_name,
                'body_preview' => $this->message->body_preview,
                'attachment_count' => $this->message->attachment_count,
                'received_at' => $this->message->received_at?->toIso8601String(),
            ],
            'contact' => [
                'id' => $this->contact->id,
                'email' => $this->contact->email,
                'full_name' => $this->contact->full_name,
            ],
            'conversation' => [
                'id' => $this->conversation->id,
                'subject' => $this->conversation->subject,
                'status' => $this->conversation->status->value,
                'message_count' => $this->conversation->message_count,
                'unread_count' => $this->conversation->unread_count,
            ],
        ];
    }

    /**
     * The event's broadcast name.
     */
    public function broadcastAs(): string
    {
        return 'inbound.email.received';
    }
}
