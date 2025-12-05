<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * HasMetadata Trait
 *
 * Handle JSON metadata for models.
 */
trait HasMetadata
{
    /**
     * Get metadata value.
     */
    public function getMeta(string $key, mixed $default = null): mixed
    {
        $metadata = $this->metadata ?? [];

        return data_get($metadata, $key, $default);
    }

    /**
     * Set metadata value.
     */
    public function setMeta(string $key, mixed $value): self
    {
        $metadata = $this->metadata ?? [];
        data_set($metadata, $key, $value);
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Check if metadata key exists.
     */
    public function hasMeta(string $key): bool
    {
        $metadata = $this->metadata ?? [];

        return data_get($metadata, $key) !== null;
    }

    /**
     * Remove metadata key.
     */
    public function removeMeta(string $key): self
    {
        $metadata = $this->metadata ?? [];
        unset($metadata[$key]);
        $this->metadata = $metadata;

        return $this;
    }
}
