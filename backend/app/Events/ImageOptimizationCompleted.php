<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Media;
use Illuminate\Broadcasting\Channel;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Broadcast when image optimization is completed
 */
class ImageOptimizationCompleted implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public Media $media,
        public array $results,
    ) {}

    /**
     * Get the channels the event should broadcast on.
     */
    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('image-optimization.' . $this->media->uploaded_by),
            new Channel('image-optimization'),
        ];
    }

    /**
     * Get the data to broadcast.
     */
    public function broadcastWith(): array
    {
        return [
            'media_id' => $this->media->id,
            'filename' => $this->media->filename,
            'thumbnail_url' => $this->media->thumbnail_url,
            'original_size' => $this->results['original_size'] ?? $this->media->size,
            'optimized_size' => $this->results['optimized_size'] ?? null,
            'savings_percent' => $this->results['savings_percent'] ?? 0,
            'variants_count' => $this->results['variants_count'] ?? 0,
            'formats' => $this->results['formats'] ?? [],
            'timestamp' => now()->toIso8601String(),
        ];
    }

    /**
     * The event's broadcast name.
     */
    public function broadcastAs(): string
    {
        return 'optimization.completed';
    }
}
