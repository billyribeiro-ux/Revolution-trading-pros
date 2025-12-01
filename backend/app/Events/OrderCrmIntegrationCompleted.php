<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Contact;
use App\Models\Order;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

class OrderCrmIntegrationCompleted implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Order $order,
        public readonly Contact $contact,
        public readonly array $result
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('contact.' . $this->contact->id),
            new PrivateChannel('order.' . $this->order->id),
        ];
    }

    public function broadcastAs(): string
    {
        return 'order.crm.integration.completed';
    }

    public function broadcastWith(): array
    {
        return [
            'order_id' => $this->order->id,
            'order_number' => $this->order->order_number,
            'contact_id' => $this->contact->id,
            'deal_created' => $this->result['deal_created'] ?? false,
            'deal_id' => $this->result['deal_id'] ?? null,
            'ltv_updated' => $this->result['ltv_updated'] ?? false,
            'tags_applied' => $this->result['tags_applied'] ?? [],
        ];
    }
}
