<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * Immutable Trait
 *
 * Prevent updates to model after creation.
 */
trait Immutable
{
    /**
     * Boot the trait.
     */
    protected static function bootImmutable(): void
    {
        static::updating(function ($model) {
            if (!$model->allowUpdates()) {
                return false;
            }
        });

        static::deleting(function ($model) {
            if (!$model->allowDeletion()) {
                return false;
            }
        });
    }

    /**
     * Check if updates are allowed.
     */
    protected function allowUpdates(): bool
    {
        return property_exists($this, 'allowUpdates') && $this->allowUpdates;
    }

    /**
     * Check if deletion is allowed.
     */
    protected function allowDeletion(): bool
    {
        return property_exists($this, 'allowDeletion') && $this->allowDeletion;
    }
}
