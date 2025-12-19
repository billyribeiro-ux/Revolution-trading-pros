<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\User;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;

/**
 * WebSocketService - Real-time Communication Infrastructure
 *
 * Provides real-time updates matching Fluent Cart Pro features:
 * - Cart updates (add, remove, quantity change)
 * - Inventory alerts (low stock, reservation expiry)
 * - Order status changes
 * - Notification delivery
 *
 * Works with Laravel Reverb/Pusher/Soketi for WebSocket transport.
 *
 * @author Revolution Trading Pros
 * @version 2.0.0
 */
class WebSocketService
{
    private const CHANNEL_PREFIX = 'revolution.';
    private const PRESENCE_CACHE_TTL = 300; // 5 minutes

    /**
     * Channel types
     */
    public const CHANNEL_PUBLIC = 'public';
    public const CHANNEL_PRIVATE = 'private';
    public const CHANNEL_PRESENCE = 'presence';

    /**
     * Get channel name for a specific entity
     */
    public function getChannelName(string $type, string $entityType, mixed $entityId = null): string
    {
        $baseName = self::CHANNEL_PREFIX . $entityType;

        if ($entityId !== null) {
            $baseName .= '.' . $entityId;
        }

        return match ($type) {
            self::CHANNEL_PRIVATE => 'private-' . $baseName,
            self::CHANNEL_PRESENCE => 'presence-' . $baseName,
            default => $baseName,
        };
    }

    /**
     * Get user's cart channel
     */
    public function getUserCartChannel(?int $userId, ?string $sessionId = null): string
    {
        if ($userId) {
            return $this->getChannelName(self::CHANNEL_PRIVATE, 'user.' . $userId . '.cart');
        }

        if ($sessionId) {
            return $this->getChannelName(self::CHANNEL_PRIVATE, 'session.' . $sessionId . '.cart');
        }

        return $this->getChannelName(self::CHANNEL_PUBLIC, 'cart');
    }

    /**
     * Get user's notification channel
     */
    public function getUserNotificationChannel(int $userId): string
    {
        return $this->getChannelName(self::CHANNEL_PRIVATE, 'user.' . $userId . '.notifications');
    }

    /**
     * Get order updates channel
     */
    public function getOrderChannel(int $orderId): string
    {
        return $this->getChannelName(self::CHANNEL_PRIVATE, 'order.' . $orderId);
    }

    /**
     * Get product inventory channel (public for stock updates)
     */
    public function getInventoryChannel(int $productId): string
    {
        return $this->getChannelName(self::CHANNEL_PUBLIC, 'inventory.' . $productId);
    }

    /**
     * Get form submissions channel (for admin dashboard)
     */
    public function getFormChannel(int $formId): string
    {
        return $this->getChannelName(self::CHANNEL_PRIVATE, 'form.' . $formId);
    }

    /**
     * Get CRM contact channel
     */
    public function getContactChannel(int $contactId): string
    {
        return $this->getChannelName(self::CHANNEL_PRIVATE, 'contact.' . $contactId);
    }

    /**
     * Broadcast cart update
     */
    public function broadcastCartUpdate(
        array $cartData,
        string $action,
        ?int $userId = null,
        ?string $sessionId = null
    ): void {
        $channel = $this->getUserCartChannel($userId, $sessionId);

        $this->broadcast($channel, 'cart.updated', [
            'action' => $action,
            'cart' => $cartData,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Broadcast inventory update
     */
    public function broadcastInventoryUpdate(int $productId, int $availableStock, ?array $reservation = null): void
    {
        $channel = $this->getInventoryChannel($productId);

        $data = [
            'product_id' => $productId,
            'available_stock' => $availableStock,
            'timestamp' => now()->toIso8601String(),
        ];

        if ($reservation) {
            $data['reservation'] = $reservation;
        }

        $this->broadcast($channel, 'inventory.updated', $data);
    }

    /**
     * Broadcast low stock alert
     */
    public function broadcastLowStockAlert(int $productId, int $currentStock, int $threshold): void
    {
        // Broadcast to admin channel
        $adminChannel = $this->getChannelName(self::CHANNEL_PRIVATE, 'admin.inventory');

        $this->broadcast($adminChannel, 'inventory.low_stock', [
            'product_id' => $productId,
            'current_stock' => $currentStock,
            'threshold' => $threshold,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Broadcast order status change
     */
    public function broadcastOrderStatus(int $orderId, int $userId, string $status, ?string $message = null): void
    {
        // Broadcast to order channel
        $orderChannel = $this->getOrderChannel($orderId);
        $this->broadcast($orderChannel, 'order.status_changed', [
            'order_id' => $orderId,
            'status' => $status,
            'message' => $message,
            'timestamp' => now()->toIso8601String(),
        ]);

        // Also send to user's notification channel
        $userChannel = $this->getUserNotificationChannel($userId);
        $this->broadcast($userChannel, 'notification', [
            'type' => 'order_status',
            'title' => 'Order Update',
            'message' => $message ?? "Your order status has been updated to: {$status}",
            'data' => [
                'order_id' => $orderId,
                'status' => $status,
            ],
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Broadcast form submission notification
     */
    public function broadcastFormSubmission(int $formId, array $submissionData): void
    {
        $channel = $this->getFormChannel($formId);

        $this->broadcast($channel, 'form.submission', [
            'form_id' => $formId,
            'submission' => $submissionData,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Broadcast CRM contact update
     */
    public function broadcastContactUpdate(int $contactId, string $eventType, array $data = []): void
    {
        $channel = $this->getContactChannel($contactId);

        $this->broadcast($channel, 'contact.' . $eventType, array_merge($data, [
            'contact_id' => $contactId,
            'timestamp' => now()->toIso8601String(),
        ]));
    }

    /**
     * Send notification to user
     */
    public function sendNotification(
        int $userId,
        string $type,
        string $title,
        string $message,
        array $data = []
    ): void {
        $channel = $this->getUserNotificationChannel($userId);

        $this->broadcast($channel, 'notification', [
            'id' => Str::uuid()->toString(),
            'type' => $type,
            'title' => $title,
            'message' => $message,
            'data' => $data,
            'read' => false,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Broadcast to a channel
     */
    public function broadcast(string $channel, string $event, array $data): void
    {
        try {
            // Use Laravel's broadcasting
            broadcast(new \App\Events\GenericBroadcast($channel, $event, $data));

            Log::debug('WebSocketService: Broadcast sent', [
                'channel' => $channel,
                'event' => $event,
            ]);
        } catch (\Exception $e) {
            Log::error('WebSocketService: Broadcast failed', [
                'channel' => $channel,
                'event' => $event,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Track user presence (for presence channels)
     */
    public function trackPresence(int $userId, string $channel): void
    {
        $cacheKey = "presence:{$channel}";
        $presence = Cache::get($cacheKey, []);

        $presence[$userId] = [
            'user_id' => $userId,
            'joined_at' => now()->toIso8601String(),
        ];

        Cache::put($cacheKey, $presence, self::PRESENCE_CACHE_TTL);
    }

    /**
     * Remove user presence
     */
    public function removePresence(int $userId, string $channel): void
    {
        $cacheKey = "presence:{$channel}";
        $presence = Cache::get($cacheKey, []);

        unset($presence[$userId]);

        Cache::put($cacheKey, $presence, self::PRESENCE_CACHE_TTL);
    }

    /**
     * Get current presence for a channel
     */
    public function getPresence(string $channel): array
    {
        return Cache::get("presence:{$channel}", []);
    }

    /**
     * Authorize channel access
     */
    public function authorizeChannel(User $user, string $channel): bool|array
    {
        // Parse channel name
        $channelName = str_replace(['private-', 'presence-'], '', $channel);

        // User-specific channels
        if (preg_match('/user\.(\d+)\./', $channelName, $matches)) {
            $targetUserId = (int) $matches[1];
            if ($user->id !== $targetUserId && !$user->hasRole(['admin', 'super-admin'])) {
                return false;
            }
        }

        // Session channels (allow if session matches)
        if (str_contains($channelName, 'session.')) {
            // Sessions are validated by middleware
            return true;
        }

        // Order channels
        if (preg_match('/order\.(\d+)/', $channelName, $matches)) {
            $orderId = (int) $matches[1];
            $order = \App\Models\Order::find($orderId);
            if ($order && $order->user_id !== $user->id && !$user->hasRole(['admin', 'super-admin'])) {
                return false;
            }
        }

        // Contact channels (admin only)
        if (str_contains($channelName, 'contact.') && !$user->hasRole(['admin', 'super-admin'])) {
            return false;
        }

        // Form channels (admin only)
        if (str_contains($channelName, 'form.') && !$user->hasRole(['admin', 'super-admin'])) {
            return false;
        }

        // Admin channels
        if (str_contains($channelName, 'admin.') && !$user->hasRole(['admin', 'super-admin'])) {
            return false;
        }

        // For presence channels, return user data
        if (str_starts_with($channel, 'presence-')) {
            return [
                'id' => $user->id,
                'name' => $user->name,
            ];
        }

        return true;
    }

    /**
     * Get connection configuration for frontend
     */
    public function getConnectionConfig(): array
    {
        return [
            'broadcaster' => config('broadcasting.default'),
            'key' => config('broadcasting.connections.' . config('broadcasting.default') . '.key'),
            'wsHost' => config('broadcasting.connections.' . config('broadcasting.default') . '.options.host'),
            'wsPort' => config('broadcasting.connections.' . config('broadcasting.default') . '.options.port', 6001),
            'wssPort' => config('broadcasting.connections.' . config('broadcasting.default') . '.options.port', 6001),
            'forceTLS' => config('app.env') === 'production',
            'enabledTransports' => ['ws', 'wss'],
            'cluster' => config('broadcasting.connections.' . config('broadcasting.default') . '.options.cluster', 'mt1'),
        ];
    }
}
