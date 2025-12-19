<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Product;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\Channel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

class InventoryReserved implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Product $product,
        public readonly int $quantity,
        public readonly string $reservationId,
        public readonly \Carbon\Carbon $expiresAt
    ) {}

    public function broadcastOn(): array
    {
        return [
            new Channel('inventory.' . $this->product->id),
        ];
    }

    public function broadcastAs(): string
    {
        return 'inventory.reserved';
    }

    public function broadcastWith(): array
    {
        return [
            'product_id' => $this->product->id,
            'quantity_reserved' => $this->quantity,
            'available_stock' => $this->product->available_stock,
            'reservation_id' => $this->reservationId,
            'expires_at' => $this->expiresAt->toIso8601String(),
        ];
    }
}
