<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Casts\Attribute;
use InvalidArgumentException;

/**
 * SEO Settings Model
 * 
 * Manages SEO configuration with type-safe value handling.
 * Supports multiple data types: string, json, boolean, integer, float.
 *
 * @property int $id
 * @property string $key
 * @property mixed $value
 * @property string $type
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 */
class SeoSetting extends Model
{
    use HasFactory;

    /**
     * Valid setting types
     */
    public const TYPE_STRING = 'string';
    public const TYPE_JSON = 'json';
    public const TYPE_BOOLEAN = 'boolean';
    public const TYPE_INTEGER = 'integer';
    public const TYPE_FLOAT = 'float';

    /**
     * All valid types
     */
    public const VALID_TYPES = [
        self::TYPE_STRING,
        self::TYPE_JSON,
        self::TYPE_BOOLEAN,
        self::TYPE_INTEGER,
        self::TYPE_FLOAT,
    ];

    protected $fillable = [
        'key',
        'value',
        'type',
    ];

    protected $casts = [
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::saving(function (self $model): void {
            $model->validateType();
            $model->validateKey();
        });
    }

    /**
     * Get the setting value with proper type casting
     */
    protected function value(): Attribute
    {
        return Attribute::make(
            get: fn (?string $value): mixed => $this->castValue($value),
            set: fn (mixed $value): string => $this->prepareValueForStorage($value)
        );
    }

    /**
     * Cast stored value to appropriate type
     */
    protected function castValue(?string $value): mixed
    {
        if ($value === null) {
            return null;
        }

        return match($this->type) {
            self::TYPE_JSON => $this->decodeJson($value),
            self::TYPE_BOOLEAN => $this->castToBoolean($value),
            self::TYPE_INTEGER => (int) $value,
            self::TYPE_FLOAT => (float) $value,
            default => $value,
        };
    }

    /**
     * Prepare value for database storage
     */
    protected function prepareValueForStorage(mixed $value): string
    {
        return match($this->type) {
            self::TYPE_JSON => $this->encodeJson($value),
            self::TYPE_BOOLEAN => $value ? '1' : '0',
            self::TYPE_INTEGER => (string) ((int) $value),
            self::TYPE_FLOAT => (string) ((float) $value),
            default => (string) $value,
        };
    }

    /**
     * Safely decode JSON with error handling
     */
    protected function decodeJson(string $value): mixed
    {
        try {
            $decoded = json_decode($value, true, 512, JSON_THROW_ON_ERROR);
            return $decoded;
        } catch (\JsonException $e) {
            logger()->error('Failed to decode SEO setting JSON', [
                'key' => $this->key,
                'value' => $value,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Safely encode JSON with error handling
     */
    protected function encodeJson(mixed $value): string
    {
        try {
            return json_encode($value, JSON_THROW_ON_ERROR | JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES);
        } catch (\JsonException $e) {
            logger()->error('Failed to encode SEO setting JSON', [
                'key' => $this->key,
                'value' => $value,
                'error' => $e->getMessage(),
            ]);
            throw new InvalidArgumentException('Invalid JSON data for SEO setting');
        }
    }

    /**
     * Cast string to boolean with multiple formats support
     */
    protected function castToBoolean(string $value): bool
    {
        return match(strtolower($value)) {
            '1', 'true', 'yes', 'on' => true,
            '0', 'false', 'no', 'off', '' => false,
            default => (bool) $value,
        };
    }

    /**
     * Validate the type field
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf(
                    'Invalid SEO setting type "%s". Valid types: %s',
                    $this->type,
                    implode(', ', self::VALID_TYPES)
                )
            );
        }
    }

    /**
     * Validate the key field
     */
    protected function validateKey(): void
    {
        if (empty($this->key) || !is_string($this->key)) {
            throw new InvalidArgumentException('SEO setting key must be a non-empty string');
        }

        // Enforce snake_case convention
        if (!preg_match('/^[a-z][a-z0-9_]*$/', $this->key)) {
            throw new InvalidArgumentException(
                'SEO setting key must be snake_case (lowercase letters, numbers, underscores only)'
            );
        }
    }

    /**
     * Scope: Get setting by key
     */
    public function scopeByKey($query, string $key)
    {
        return $query->where('key', $key);
    }

    /**
     * Scope: Get settings by type
     */
    public function scopeByType($query, string $type)
    {
        return $query->where('type', $type);
    }

    /**
     * Helper: Get setting value by key
     */
    public static function getValue(string $key, mixed $default = null): mixed
    {
        $setting = static::byKey($key)->first();
        return $setting?->value ?? $default;
    }

    /**
     * Helper: Set or update setting value
     */
    public static function setValue(string $key, mixed $value, string $type = self::TYPE_STRING): self
    {
        return static::updateOrCreate(
            ['key' => $key],
            ['value' => $value, 'type' => $type]
        );
    }

    /**
     * Helper: Check if setting exists
     */
    public static function hasKey(string $key): bool
    {
        return static::byKey($key)->exists();
    }

    /**
     * Helper: Delete setting by key
     */
    public static function deleteByKey(string $key): bool
    {
        return static::byKey($key)->delete() > 0;
    }

    /**
     * Helper: Get all settings as key-value array
     */
    public static function getAllSettings(): array
    {
        return static::all()->pluck('value', 'key')->toArray();
    }
}