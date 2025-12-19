<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * Configurable Trait
 *
 * Handle configuration settings for models.
 */
trait Configurable
{
    /**
     * Get configuration value.
     */
    public function getConfig(string $key, mixed $default = null): mixed
    {
        $config = $this->config ?? [];

        return data_get($config, $key, $default);
    }

    /**
     * Set configuration value.
     */
    public function setConfig(string $key, mixed $value): self
    {
        $config = $this->config ?? [];
        data_set($config, $key, $value);
        $this->config = $config;

        return $this;
    }

    /**
     * Merge configuration values.
     */
    public function mergeConfig(array $values): self
    {
        $config = $this->config ?? [];
        $this->config = array_merge($config, $values);

        return $this;
    }
}
