<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Database\Eloquent\Relations\MorphMany;

/**
 * HasMedia Trait
 *
 * Handle media attachments for models.
 */
trait HasMedia
{
    /**
     * Get all media for the model.
     */
    public function media(): MorphMany
    {
        return $this->morphMany(\App\Models\Media::class, 'mediable');
    }

    /**
     * Get media by collection.
     */
    public function getMedia(string $collection = 'default'): \Illuminate\Database\Eloquent\Collection
    {
        return $this->media()->where('collection', $collection)->get();
    }

    /**
     * Get first media item.
     */
    public function getFirstMedia(string $collection = 'default'): ?\App\Models\Media
    {
        return $this->media()->where('collection', $collection)->first();
    }

    /**
     * Check if model has media.
     */
    public function hasMedia(string $collection = 'default'): bool
    {
        return $this->media()->where('collection', $collection)->exists();
    }
}
