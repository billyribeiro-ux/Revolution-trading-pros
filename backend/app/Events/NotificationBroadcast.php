<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\User;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcastNow;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * NotificationBroadcast - Real-time user notification event
 *
 * Enterprise-grade notification broadcasting with:
 * - Priority-based delivery
 * - Notification categorization
 * - Action links support
 * - Read/dismiss tracking
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class NotificationBroadcast implements ShouldBroadcastNow
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public string $id;

    public function __construct(
        public readonly int $userId,
        public readonly string $type,
        public readonly string $title,
        public readonly string $message,
        public readonly string $priority = 'normal',
        public readonly ?array $action = null,
        public readonly ?array $metadata = null
    ) {
        $this->id = uniqid('notif_', true);
    }

    /**
     * Get the channels the event should broadcast on.
     */
    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('user.' . $this->userId),
        ];
    }

    /**
     * The event's broadcast name.
     */
    public function broadcastAs(): string
    {
        return 'notification';
    }

    /**
     * Get the data to broadcast.
     */
    public function broadcastWith(): array
    {
        return [
            'id' => $this->id,
            'type' => $this->type,
            'priority' => $this->priority,
            'title' => $this->title,
            'message' => $this->message,
            'action' => $this->action,
            'metadata' => $this->metadata,
            'timestamp' => now()->toIso8601String(),
            'read' => false,
            'dismissed' => false,
        ];
    }

    /**
     * Static factory methods for common notification types
     */
    public static function info(int $userId, string $title, string $message, ?array $action = null): self
    {
        return new self($userId, 'info', $title, $message, 'normal', $action);
    }

    public static function success(int $userId, string $title, string $message, ?array $action = null): self
    {
        return new self($userId, 'success', $title, $message, 'normal', $action);
    }

    public static function warning(int $userId, string $title, string $message, ?array $action = null): self
    {
        return new self($userId, 'warning', $title, $message, 'high', $action);
    }

    public static function error(int $userId, string $title, string $message, ?array $action = null): self
    {
        return new self($userId, 'error', $title, $message, 'urgent', $action);
    }

    public static function system(int $userId, string $title, string $message, ?array $action = null): self
    {
        return new self($userId, 'system', $title, $message, 'normal', $action);
    }

    /**
     * Send notification to multiple users
     */
    public static function toUsers(array $userIds, string $type, string $title, string $message, string $priority = 'normal', ?array $action = null): void
    {
        foreach ($userIds as $userId) {
            broadcast(new self($userId, $type, $title, $message, $priority, $action));
        }
    }

    /**
     * Send notification to all admin users
     */
    public static function toAdmins(string $type, string $title, string $message, string $priority = 'normal', ?array $action = null): void
    {
        $adminIds = User::role(['admin', 'super-admin'])->pluck('id')->toArray();
        self::toUsers($adminIds, $type, $title, $message, $priority, $action);
    }
}
