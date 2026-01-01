<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Cart;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

class CartUpdated implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Cart $cart,
        public readonly string $action,
        public readonly ?int $productId = null
    ) {}

    public function broadcastOn(): array
    {
        $channels = [];

        if ($this->cart->user_id) {
            $channels[] = new PrivateChannel('user.' . $this->cart->user_id . '.cart');
        }

        if ($this->cart->session_id) {
            $channels[] = new PrivateChannel('session.' . $this->cart->session_id . '.cart');
        }

        return $channels;
    }

    public function broadcastAs(): string
    {
        return 'cart.updated';
    }

    public function broadcastWith(): array
    {
        return [
            'cart_id' => $this->cart->id,
            'action' => $this->action,
            'product_id' => $this->productId,
            'items_count' => $this->cart->items()->count(),
            'subtotal' => $this->cart->subtotal,
            'total' => $this->cart->total,
            'items' => $this->cart->items->map(fn($item) => [
                'id' => $item->id,
                'product_id' => $item->product_id,
                'quantity' => $item->quantity,
                'price' => $item->price,
                'total' => $item->total,
                'reserved_until' => $item->reserved_until?->toIso8601String(),
            ])->toArray(),
        ];
    }
}
