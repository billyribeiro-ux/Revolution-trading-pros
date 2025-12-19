<?php

namespace App\Services\Dashboard;

use App\Models\DashboardWidget;
use Illuminate\Support\Facades\Redis;
use Illuminate\Support\Facades\Log;

class WebSocketService
{
    private const CHANNEL_PREFIX = 'dashboard:';
    private const WIDGET_UPDATE_EVENT = 'widget:update';
    private const DASHBOARD_UPDATE_EVENT = 'dashboard:update';

    /**
     * Broadcast widget data update to all connected clients
     */
    public function broadcastWidgetUpdate(string $widgetId, array $data): void
    {
        $channel = self::CHANNEL_PREFIX . "widget:{$widgetId}";
        
        $payload = [
            'event' => self::WIDGET_UPDATE_EVENT,
            'widget_id' => $widgetId,
            'data' => $data,
            'timestamp' => now()->toIso8601String(),
        ];

        $this->publish($channel, $payload);
        
        Log::info("WebSocket: Widget update broadcasted", [
            'widget_id' => $widgetId,
            'channel' => $channel
        ]);
    }

    /**
     * Broadcast dashboard update to all users viewing it
     */
    public function broadcastDashboardUpdate(string $dashboardId, array $changes): void
    {
        $channel = self::CHANNEL_PREFIX . "dashboard:{$dashboardId}";
        
        $payload = [
            'event' => self::DASHBOARD_UPDATE_EVENT,
            'dashboard_id' => $dashboardId,
            'changes' => $changes,
            'timestamp' => now()->toIso8601String(),
        ];

        $this->publish($channel, $payload);
        
        Log::info("WebSocket: Dashboard update broadcasted", [
            'dashboard_id' => $dashboardId,
            'channel' => $channel
        ]);
    }

    /**
     * Subscribe user to dashboard updates
     */
    public function subscribeToDashboard(string $dashboardId, int $userId): string
    {
        $channel = self::CHANNEL_PREFIX . "dashboard:{$dashboardId}";
        
        // Store subscription in Redis
        Redis::sadd("dashboard:{$dashboardId}:subscribers", $userId);
        Redis::expire("dashboard:{$dashboardId}:subscribers", 3600); // 1 hour TTL
        
        return $channel;
    }

    /**
     * Unsubscribe user from dashboard updates
     */
    public function unsubscribeFromDashboard(string $dashboardId, int $userId): void
    {
        Redis::srem("dashboard:{$dashboardId}:subscribers", $userId);
    }

    /**
     * Get all subscribers for a dashboard
     */
    public function getDashboardSubscribers(string $dashboardId): array
    {
        $subscribers = Redis::smembers("dashboard:{$dashboardId}:subscribers");
        return array_map('intval', $subscribers);
    }

    /**
     * Broadcast system-wide notification
     */
    public function broadcastSystemNotification(string $message, string $type = 'info'): void
    {
        $channel = self::CHANNEL_PREFIX . 'system';
        
        $payload = [
            'event' => 'system:notification',
            'message' => $message,
            'type' => $type,
            'timestamp' => now()->toIso8601String(),
        ];

        $this->publish($channel, $payload);
    }

    /**
     * Publish message to Redis channel
     */
    private function publish(string $channel, array $payload): void
    {
        try {
            Redis::publish($channel, json_encode($payload));
        } catch (\Exception $e) {
            Log::error("WebSocket: Failed to publish message", [
                'channel' => $channel,
                'error' => $e->getMessage()
            ]);
        }
    }

    /**
     * Get channel name for widget
     */
    public function getWidgetChannel(string $widgetId): string
    {
        return self::CHANNEL_PREFIX . "widget:{$widgetId}";
    }

    /**
     * Get channel name for dashboard
     */
    public function getDashboardChannel(string $dashboardId): string
    {
        return self::CHANNEL_PREFIX . "dashboard:{$dashboardId}";
    }

    /**
     * Broadcast widget refresh to trigger data reload
     */
    public function triggerWidgetRefresh(string $widgetId): void
    {
        $channel = $this->getWidgetChannel($widgetId);
        
        $payload = [
            'event' => 'widget:refresh',
            'widget_id' => $widgetId,
            'timestamp' => now()->toIso8601String(),
        ];

        $this->publish($channel, $payload);
    }

    /**
     * Broadcast multiple widget updates in batch
     */
    public function broadcastBatchUpdate(array $widgetUpdates): void
    {
        foreach ($widgetUpdates as $widgetId => $data) {
            $this->broadcastWidgetUpdate($widgetId, $data);
        }
    }

    /**
     * Check if user is subscribed to dashboard
     */
    public function isUserSubscribed(string $dashboardId, int $userId): bool
    {
        return Redis::sismember("dashboard:{$dashboardId}:subscribers", $userId);
    }

    /**
     * Get active connections count for dashboard
     */
    public function getActiveConnectionsCount(string $dashboardId): int
    {
        return Redis::scard("dashboard:{$dashboardId}:subscribers");
    }
}
