<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Cache;

/**
 * ConsentSettings Model
 *
 * Manages consent management configuration settings.
 * Provides a key-value store with type casting and caching.
 *
 * @property int $id
 * @property string $key
 * @property string|null $value
 * @property string $type
 * @property string $group
 * @property string|null $description
 * @property bool $is_public
 */
class ConsentSettings extends Model
{
    protected $table = 'consent_settings';

    protected $fillable = [
        'key',
        'value',
        'type',
        'group',
        'description',
        'is_public',
    ];

    protected $casts = [
        'is_public' => 'boolean',
    ];

    /**
     * Setting groups
     */
    public const GROUP_GENERAL = 'general';
    public const GROUP_BANNER = 'banner';
    public const GROUP_DESIGN = 'design';
    public const GROUP_BEHAVIOR = 'behavior';
    public const GROUP_SCRIPTS = 'scripts';
    public const GROUP_GEOLOCATION = 'geolocation';
    public const GROUP_INTEGRATIONS = 'integrations';
    public const GROUP_PROOF = 'proof';

    /**
     * Setting types
     */
    public const TYPE_STRING = 'string';
    public const TYPE_INTEGER = 'integer';
    public const TYPE_BOOLEAN = 'boolean';
    public const TYPE_JSON = 'json';
    public const TYPE_ARRAY = 'array';

    /**
     * Cache key prefix
     */
    protected const CACHE_PREFIX = 'consent_settings:';
    protected const CACHE_TTL = 3600; // 1 hour

    /**
     * Get a setting value by key
     */
    public static function getValue(string $key, mixed $default = null): mixed
    {
        $cacheKey = self::CACHE_PREFIX . $key;

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($key, $default) {
            $setting = static::where('key', $key)->first();

            if (!$setting) {
                return $default;
            }

            return $setting->getCastedValue();
        });
    }

    /**
     * Set a setting value
     */
    public static function setValue(string $key, mixed $value, array $attributes = []): static
    {
        $type = $attributes['type'] ?? self::determineType($value);
        $storedValue = self::serializeValue($value, $type);

        $setting = static::updateOrCreate(
            ['key' => $key],
            array_merge([
                'value' => $storedValue,
                'type' => $type,
            ], $attributes)
        );

        // Clear cache
        Cache::forget(self::CACHE_PREFIX . $key);
        Cache::forget(self::CACHE_PREFIX . 'all');

        return $setting;
    }

    /**
     * Get all settings as an array
     */
    public static function getAllSettings(?string $group = null): array
    {
        $cacheKey = self::CACHE_PREFIX . 'all' . ($group ? ':' . $group : '');

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($group) {
            $query = static::query();

            if ($group) {
                $query->where('group', $group);
            }

            return $query->get()
                ->mapWithKeys(fn($setting) => [$setting->key => $setting->getCastedValue()])
                ->toArray();
        });
    }

    /**
     * Get all public settings
     */
    public static function getPublicSettings(): array
    {
        return Cache::remember(self::CACHE_PREFIX . 'public', self::CACHE_TTL, function () {
            return static::where('is_public', true)
                ->get()
                ->mapWithKeys(fn($setting) => [$setting->key => $setting->getCastedValue()])
                ->toArray();
        });
    }

    /**
     * Get settings by group
     */
    public static function getByGroup(string $group): array
    {
        return static::getAllSettings($group);
    }

    /**
     * Bulk update settings
     */
    public static function bulkUpdate(array $settings): void
    {
        foreach ($settings as $key => $value) {
            if (is_array($value) && isset($value['value'])) {
                static::setValue($key, $value['value'], $value);
            } else {
                static::setValue($key, $value);
            }
        }

        // Clear all cache
        Cache::forget(self::CACHE_PREFIX . 'all');
    }

    /**
     * Clear all settings cache
     */
    public static function clearCache(): void
    {
        $settings = static::all();
        foreach ($settings as $setting) {
            Cache::forget(self::CACHE_PREFIX . $setting->key);
        }
        Cache::forget(self::CACHE_PREFIX . 'all');
        Cache::forget(self::CACHE_PREFIX . 'public');
    }

    /**
     * Get the casted value based on type
     */
    public function getCastedValue(): mixed
    {
        if ($this->value === null) {
            return null;
        }

        return match ($this->type) {
            self::TYPE_INTEGER => (int) $this->value,
            self::TYPE_BOOLEAN => filter_var($this->value, FILTER_VALIDATE_BOOLEAN),
            self::TYPE_JSON, self::TYPE_ARRAY => json_decode($this->value, true),
            default => $this->value,
        };
    }

    /**
     * Determine the type of a value
     */
    protected static function determineType(mixed $value): string
    {
        return match (true) {
            is_bool($value) => self::TYPE_BOOLEAN,
            is_int($value) => self::TYPE_INTEGER,
            is_array($value) => self::TYPE_JSON,
            default => self::TYPE_STRING,
        };
    }

    /**
     * Serialize value for storage
     */
    protected static function serializeValue(mixed $value, string $type): ?string
    {
        if ($value === null) {
            return null;
        }

        return match ($type) {
            self::TYPE_BOOLEAN => $value ? '1' : '0',
            self::TYPE_JSON, self::TYPE_ARRAY => json_encode($value),
            default => (string) $value,
        };
    }

    /**
     * Get default settings configuration
     */
    public static function getDefaults(): array
    {
        return [
            // General Settings
            'consent_enabled' => ['value' => true, 'type' => 'boolean', 'group' => 'general', 'is_public' => true],
            'test_mode' => ['value' => false, 'type' => 'boolean', 'group' => 'general'],
            'expire_days' => ['value' => 365, 'type' => 'integer', 'group' => 'general', 'is_public' => true],
            'consent_version' => ['value' => 1, 'type' => 'integer', 'group' => 'general', 'is_public' => true],
            'policy_version' => ['value' => '1.0.0', 'type' => 'string', 'group' => 'general', 'is_public' => true],

            // Banner Settings
            'banner_enabled' => ['value' => true, 'type' => 'boolean', 'group' => 'banner', 'is_public' => true],
            'banner_position' => ['value' => 'bottom', 'type' => 'string', 'group' => 'banner', 'is_public' => true],
            'banner_layout' => ['value' => 'bar', 'type' => 'string', 'group' => 'banner', 'is_public' => true],
            'show_reject_button' => ['value' => true, 'type' => 'boolean', 'group' => 'banner', 'is_public' => true],
            'show_settings_button' => ['value' => true, 'type' => 'boolean', 'group' => 'banner', 'is_public' => true],
            'close_on_scroll' => ['value' => false, 'type' => 'boolean', 'group' => 'banner', 'is_public' => true],
            'close_on_scroll_distance' => ['value' => 60, 'type' => 'integer', 'group' => 'banner', 'is_public' => true],

            // Script Blocking
            'script_blocking_enabled' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_google_analytics' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_google_tag_manager' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_facebook_pixel' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_tiktok_pixel' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_twitter_pixel' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_linkedin_pixel' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_pinterest_tag' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_reddit_pixel' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_hotjar' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_youtube_embeds' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_vimeo_embeds' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],
            'block_google_maps' => ['value' => true, 'type' => 'boolean', 'group' => 'scripts'],

            // Google Consent Mode
            'google_consent_mode_enabled' => ['value' => true, 'type' => 'boolean', 'group' => 'integrations', 'is_public' => true],
            'bing_consent_mode_enabled' => ['value' => false, 'type' => 'boolean', 'group' => 'integrations', 'is_public' => true],

            // Geolocation
            'geolocation_enabled' => ['value' => false, 'type' => 'boolean', 'group' => 'geolocation'],
            'geo_default_strict' => ['value' => true, 'type' => 'boolean', 'group' => 'geolocation'],

            // Proof of Consent
            'proof_consent_enabled' => ['value' => true, 'type' => 'boolean', 'group' => 'proof'],
            'proof_retention_days' => ['value' => 365, 'type' => 'integer', 'group' => 'proof'],
            'proof_auto_delete' => ['value' => true, 'type' => 'boolean', 'group' => 'proof'],
        ];
    }

    /**
     * Initialize default settings
     */
    public static function initializeDefaults(): void
    {
        $defaults = self::getDefaults();

        foreach ($defaults as $key => $config) {
            if (!static::where('key', $key)->exists()) {
                static::create([
                    'key' => $key,
                    'value' => self::serializeValue($config['value'], $config['type']),
                    'type' => $config['type'],
                    'group' => $config['group'] ?? 'general',
                    'is_public' => $config['is_public'] ?? false,
                ]);
            }
        }
    }
}
