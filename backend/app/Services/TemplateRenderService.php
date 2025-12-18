<?php

namespace App\Services;

/**
 * Simple template rendering service that replaces {{variable}} placeholders
 * with values from a data array. Supports dot notation for nested data
 * (e.g. {{user.name}}). Uses PHP's preg_replace_callback.
 */
class TemplateRenderService
{
    /**
     * Render a template string with the provided data.
     *
     * @param  string  $template  The raw template containing {{placeholders}}
     * @param  array  $data  Associative array of data to replace
     * @return string Rendered output
     */
    public function render(string $template, array $data): string
    {
        // Callback to replace each placeholder
        $callback = function ($matches) use ($data) {
            $key = trim($matches[1]); // e.g. user.name
            $value = $this->resolveKey($key, $data);

            return $value ?? $matches[0]; // keep original if not found
        };

        return preg_replace_callback('/{{\s*([^}]+)\s*}}/', $callback, $template);
    }

    /**
     * Resolve a dot‑notation key against the data array.
     *
     * @param  string  $key  e.g. "user.name"
     */
    protected function resolveKey(string $key, array $data): ?string
    {
        $segments = explode('.', $key);
        $value = $data;
        foreach ($segments as $segment) {
            if (is_array($value) && array_key_exists($segment, $value)) {
                $value = $value[$segment];
            } else {
                return null;
            }
        }

        // Cast scalar values to string, ignore objects/arrays
        return is_scalar($value) ? (string) $value : null;
    }
}
