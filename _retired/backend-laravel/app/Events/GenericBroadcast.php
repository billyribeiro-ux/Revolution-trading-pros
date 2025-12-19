<?php

declare(strict_types=1);

namespace App\Events;

use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\Channel;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Broadcasting\PresenceChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcastNow;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * GenericBroadcast - Flexible broadcast event for WebSocket service
 */
class GenericBroadcast implements ShouldBroadcastNow
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        private readonly string $channelName,
        private readonly string $eventName,
        private readonly array $data
    ) {}

    public function broadcastOn(): array
    {
        // Determine channel type from name
        if (str_starts_with($this->channelName, 'private-')) {
            return [new PrivateChannel(str_replace('private-', '', $this->channelName))];
        }

        if (str_starts_with($this->channelName, 'presence-')) {
            return [new PresenceChannel(str_replace('presence-', '', $this->channelName))];
        }

        return [new Channel($this->channelName)];
    }

    public function broadcastAs(): string
    {
        return $this->eventName;
    }

    public function broadcastWith(): array
    {
        return $this->data;
    }
}
