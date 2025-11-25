<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * Revolution Trading Pros - UserSession Model
 * Enterprise-grade session management with Redis caching
 *
 * @property int $id
 * @property int $user_id
 * @property string $session_id
 * @property string $device_id
 * @property string $device_name
 * @property string $device_type
 * @property string|null $os_name
 * @property string|null $os_version
 * @property string|null $browser_name
 * @property string|null $browser_version
 * @property string $ip_address
 * @property string $user_agent
 * @property string|null $location_country
 * @property string|null $location_city
 * @property float|null $location_lat
 * @property float|null $location_lng
 * @property \Carbon\Carbon $last_activity_at
 * @property \Carbon\Carbon $expires_at
 * @property bool $is_active
 * @property string|null $logout_reason
 * @property array|null $metadata
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */
class UserSession extends Model
{
    use HasFactory;

    /**
     * Cache key prefix for session validation
     */
    const CACHE_PREFIX = 'user_session:';
    const CACHE_TTL = 300; // 5 minutes

    /**
     * The attributes that are mass assignable.
     */
    protected $fillable = [
        'user_id',
        'session_id',
        'device_id',
        'device_name',
        'device_type',
        'os_name',
        'os_version',
        'browser_name',
        'browser_version',
        'ip_address',
        'user_agent',
        'location_country',
        'location_city',
        'location_lat',
        'location_lng',
        'last_activity_at',
        'expires_at',
        'is_active',
        'logout_reason',
        'metadata',
    ];

    /**
     * The attributes that should be cast.
     */
    protected $casts = [
        'last_activity_at' => 'datetime',
        'expires_at' => 'datetime',
        'is_active' => 'boolean',
        'metadata' => 'array',
        'location_lat' => 'float',
        'location_lng' => 'float',
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (UserSession $session) {
            if (empty($session->session_id)) {
                $session->session_id = (string) Str::uuid();
            }
        });

        // Clear cache when session is updated
        static::updated(function (UserSession $session) {
            Cache::forget(self::CACHE_PREFIX . $session->session_id);
        });

        static::deleted(function (UserSession $session) {
            Cache::forget(self::CACHE_PREFIX . $session->session_id);
        });
    }

    /**
     * Get the user that owns the session.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Get the tokens associated with this session.
     */
    public function tokens(): HasMany
    {
        return $this->hasMany(\Laravel\Sanctum\PersonalAccessToken::class, 'user_session_id');
    }

    /**
     * Check if the session is valid (active and not expired).
     */
    public function isValid(): bool
    {
        return $this->is_active && $this->expires_at->isFuture();
    }

    /**
     * Validate session with Redis caching for performance.
     */
    public static function validateWithCache(string $sessionId): ?self
    {
        $cacheKey = self::CACHE_PREFIX . $sessionId;

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($sessionId) {
            return self::where('session_id', $sessionId)
                ->where('is_active', true)
                ->where('expires_at', '>', now())
                ->first();
        });
    }

    /**
     * Invalidate session and clear cache.
     */
    public function invalidate(string $reason = 'manual'): void
    {
        $this->update([
            'is_active' => false,
            'logout_reason' => $reason,
        ]);

        // Delete associated tokens
        $this->tokens()->delete();

        // Clear cache
        Cache::forget(self::CACHE_PREFIX . $this->session_id);
    }

    /**
     * Update last activity timestamp.
     */
    public function touch(): bool
    {
        $this->last_activity_at = now();
        return $this->save();
    }

    /**
     * Scope to get active sessions only.
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true)
            ->where('expires_at', '>', now());
    }

    /**
     * Scope to get sessions for a specific user.
     */
    public function scopeForUser($query, int $userId)
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Get a human-readable device description.
     */
    public function getDeviceDescriptionAttribute(): string
    {
        $parts = [];

        if ($this->browser_name) {
            $parts[] = $this->browser_name . ($this->browser_version ? " {$this->browser_version}" : '');
        }

        if ($this->os_name) {
            $parts[] = $this->os_name . ($this->os_version ? " {$this->os_version}" : '');
        }

        return implode(' on ', $parts) ?: $this->device_name;
    }

    /**
     * Get location as a string.
     */
    public function getLocationAttribute(): ?string
    {
        if ($this->location_city && $this->location_country) {
            return "{$this->location_city}, {$this->location_country}";
        }

        return $this->location_country;
    }

    /**
     * Check if this is the current device (same IP and user agent hash).
     */
    public function isCurrentDevice(string $ip, string $userAgent): bool
    {
        return $this->ip_address === $ip &&
               hash('sha256', $this->user_agent) === hash('sha256', $userAgent);
    }

    /**
     * Convert to array for API response.
     */
    public function toApiArray(): array
    {
        return [
            'id' => $this->id,
            'session_id' => $this->session_id,
            'device_name' => $this->device_name,
            'device_type' => $this->device_type,
            'device_description' => $this->device_description,
            'browser' => $this->browser_name,
            'os' => $this->os_name,
            'ip_address' => $this->ip_address,
            'location' => $this->location,
            'last_activity_at' => $this->last_activity_at->toISOString(),
            'created_at' => $this->created_at->toISOString(),
            'is_current' => false, // Set by controller
        ];
    }
}
