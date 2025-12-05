<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Support\Facades\App;

/**
 * Translatable Trait
 *
 * Handle multi-language content for models.
 */
trait Translatable
{
    /**
     * Get translatable attributes.
     */
    public function getTranslatableAttributes(): array
    {
        return $this->translatable ?? [];
    }

    /**
     * Get translated value for an attribute.
     */
    public function getTranslation(string $key, ?string $locale = null): mixed
    {
        $locale = $locale ?? App::getLocale();
        $translations = $this->translations ?? [];

        if (isset($translations[$locale][$key])) {
            return $translations[$locale][$key];
        }

        return $this->{$key} ?? null;
    }

    /**
     * Set translated value for an attribute.
     */
    public function setTranslation(string $key, string $value, ?string $locale = null): self
    {
        $locale = $locale ?? App::getLocale();
        $translations = $this->translations ?? [];
        $translations[$locale][$key] = $value;
        $this->translations = $translations;

        return $this;
    }
}
