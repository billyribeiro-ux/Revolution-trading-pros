<?php

declare(strict_types=1);

namespace App\Observers;

use App\Models\Order;
use App\Services\Integration\IntegrationHub;
use App\Services\Integration\WebSocketService;
use App\Services\Integration\InventoryReservationService;
use Illuminate\Support\Facades\Log;

/**
 * OrderObserver - Wires orders to the ecosystem
 *
 * Triggers:
 * - CRM contact updates (LTV, tags)
 * - Deal creation/updates
 * - Inventory commitment
 * - WebSocket notifications
 * - Automation workflows
 */
class OrderObserver
{
    public function __construct(
        private readonly IntegrationHub $integrationHub,
        private readonly WebSocketService $webSocketService,
        private readonly InventoryReservationService $inventoryService
    ) {}

    /**
     * Handle the Order "created" event.
     */
    public function created(Order $order): void
    {
        try {
            // Process through integration hub
            $this->integrationHub->processOrder($order, 'created');

            // Notify user
            if ($order->user_id) {
                $this->webSocketService->broadcastOrderStatus(
                    $order->id,
                    $order->user_id,
                    $order->status,
                    "Your order #{$order->order_number} has been placed successfully!"
                );
            }

            Log::info('OrderObserver: Order created', [
                'order_id' => $order->id,
                'order_number' => $order->order_number,
            ]);
        } catch (\Exception $e) {
            Log::error('OrderObserver: Failed to process order creation', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Handle the Order "updated" event.
     */
    public function updated(Order $order): void
    {
        try {
            // Check for status change
            if ($order->isDirty('status')) {
                $oldStatus = $order->getOriginal('status');
                $newStatus = $order->status;

                // Process status-specific logic
                $this->handleStatusChange($order, $oldStatus, $newStatus);

                // Notify user
                if ($order->user_id) {
                    $this->webSocketService->broadcastOrderStatus(
                        $order->id,
                        $order->user_id,
                        $newStatus,
                        $this->getStatusMessage($order, $newStatus)
                    );
                }
            }
        } catch (\Exception $e) {
            Log::error('OrderObserver: Failed to process order update', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Handle status-specific business logic
     */
    private function handleStatusChange(Order $order, string $oldStatus, string $newStatus): void
    {
        switch ($newStatus) {
            case 'completed':
            case 'processing':
                $this->integrationHub->processOrder($order, 'completed');

                // Commit inventory reservations
                foreach ($order->items as $item) {
                    if ($item->reservation_id) {
                        $this->inventoryService->commit($item->reservation_id, $order->id);
                    }
                }
                break;

            case 'cancelled':
                $this->integrationHub->processOrder($order, 'cancelled');

                // Release inventory reservations
                foreach ($order->items as $item) {
                    if ($item->reservation_id) {
                        $this->inventoryService->release($item->reservation_id);
                    }
                }
                break;

            case 'refunded':
                $this->integrationHub->processOrder($order, 'refunded');
                break;

            case 'shipped':
                $this->integrationHub->processOrder($order, 'shipped');
                break;

            case 'delivered':
                $this->integrationHub->processOrder($order, 'delivered');
                break;
        }
    }

    /**
     * Get human-readable status message
     */
    private function getStatusMessage(Order $order, string $status): string
    {
        return match ($status) {
            'pending' => "Your order #{$order->order_number} is pending payment.",
            'processing' => "Your order #{$order->order_number} is being processed.",
            'completed' => "Your order #{$order->order_number} has been completed!",
            'shipped' => "Great news! Your order #{$order->order_number} has been shipped.",
            'delivered' => "Your order #{$order->order_number} has been delivered!",
            'cancelled' => "Your order #{$order->order_number} has been cancelled.",
            'refunded' => "Your order #{$order->order_number} has been refunded.",
            'on_hold' => "Your order #{$order->order_number} is on hold.",
            default => "Your order #{$order->order_number} status has been updated to: {$status}",
        };
    }
}
