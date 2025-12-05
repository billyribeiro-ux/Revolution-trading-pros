<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Support\Str;

/**
 * HasUuid Trait
 *
 * Automatically generates UUID for models.
 */
trait HasUuid
{
    /**
     * Boot the trait.
     */
    protected static function bootHasUuid(): void
    {
        static::creating(function ($model) {
            if (empty($model->{$model->getKeyName()}) && $model->getKeyType() === 'string') {
                $model->{$model->getKeyName()} = (string) Str::uuid();
            }

            if (empty($model->uuid) && in_array('uuid', $model->getFillable())) {
                $model->uuid = (string) Str::uuid();
            }
        });
    }

    /**
     * Get the route key for the model.
     */
    public function getRouteKeyName(): string
    {
        return property_exists($this, 'routeKeyName') ? $this->routeKeyName : 'uuid';
    }
}
