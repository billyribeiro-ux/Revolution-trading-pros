<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * Versionable Trait
 *
 * Track model versions for audit and rollback.
 */
trait Versionable
{
    /**
     * Boot the trait.
     */
    protected static function bootVersionable(): void
    {
        static::updating(function ($model) {
            if (method_exists($model, 'incrementVersion')) {
                $model->incrementVersion();
            }
        });
    }

    /**
     * Increment the version number.
     */
    public function incrementVersion(): void
    {
        if (isset($this->version)) {
            $this->version = ($this->version ?? 0) + 1;
        }
    }

    /**
     * Get current version.
     */
    public function getVersion(): int
    {
        return $this->version ?? 1;
    }
}
