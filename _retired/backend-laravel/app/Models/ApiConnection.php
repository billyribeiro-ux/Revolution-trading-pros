<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Support\Facades\Crypt;
use Carbon\Carbon;

/**
 * API Connection Model
 *
 * Manages third-party API connections with encrypted credentials,
 * health monitoring, and comprehensive status tracking.
 *
 * @property string $id
 * @property string $service_key
 * @property string $service_name
 * @property string $category
 * @property string $status
 * @property Carbon|null $connected_at
 * @property Carbon|null $last_verified_at
 * @property Carbon|null $expires_at
 * @property array|null $credentials
 * @property array|null $config
 * @property array|null $metadata
 * @property int $health_score
 * @property Carbon|null $last_health_check
 * @property string|null $last_error
 * @property int $error_count
 * @property int $success_count
 * @property int $api_calls_today
 * @property int $api_calls_total
 * @property Carbon|null $last_api_call
 * @property bool $is_oauth
 * @property string|null $access_token
 * @property string|null $refresh_token
 * @property string|null $oauth_scopes
 * @property string|null $webhook_url
 * @property string|null $webhook_secret
 * @property bool $webhooks_enabled
 * @property string $environment
 * @property string|null $api_version
 *
 * @level L11 Principal Engineer - Apple-grade implementation
 */
class ApiConnection extends Model
{
    use HasFactory, HasUuids, SoftDeletes;

    protected $fillable = [
        'service_key',
        'service_name',
        'category',
        'status',
        'connected_at',
        'last_verified_at',
        'expires_at',
        'credentials',
        'config',
        'metadata',
        'health_score',
        'last_health_check',
        'last_error',
        'error_count',
        'success_count',
        'api_calls_today',
        'api_calls_total',
        'last_api_call',
        'is_oauth',
        'access_token',
        'refresh_token',
        'oauth_scopes',
        'webhook_url',
        'webhook_secret',
        'webhooks_enabled',
        'environment',
        'api_version',
    ];

    protected $casts = [
        'connected_at' => 'datetime',
        'last_verified_at' => 'datetime',
        'expires_at' => 'datetime',
        'last_health_check' => 'datetime',
        'last_api_call' => 'datetime',
        'config' => 'array',
        'metadata' => 'array',
        'is_oauth' => 'boolean',
        'webhooks_enabled' => 'boolean',
        'health_score' => 'integer',
        'error_count' => 'integer',
        'success_count' => 'integer',
        'api_calls_today' => 'integer',
        'api_calls_total' => 'integer',
    ];

    protected $hidden = [
        'credentials',
        'access_token',
        'refresh_token',
        'webhook_secret',
    ];

    // =========================================================================
    // CREDENTIAL ENCRYPTION
    // =========================================================================

    /**
     * Set credentials with encryption
     */
    public function setCredentialsAttribute(?array $value): void
    {
        $this->attributes['credentials'] = $value
            ? Crypt::encryptString(json_encode($value))
            : null;
    }

    /**
     * Get decrypted credentials
     */
    public function getCredentialsAttribute(?string $value): ?array
    {
        if (!$value) {
            return null;
        }

        try {
            return json_decode(Crypt::decryptString($value), true);
        } catch (\Exception $e) {
            return null;
        }
    }

    /**
     * Set access token with encryption
     */
    public function setAccessTokenAttribute(?string $value): void
    {
        $this->attributes['access_token'] = $value
            ? Crypt::encryptString($value)
            : null;
    }

    /**
     * Get decrypted access token
     */
    public function getAccessTokenAttribute(?string $value): ?string
    {
        if (!$value) {
            return null;
        }

        try {
            return Crypt::decryptString($value);
        } catch (\Exception $e) {
            return null;
        }
    }

    /**
     * Set refresh token with encryption
     */
    public function setRefreshTokenAttribute(?string $value): void
    {
        $this->attributes['refresh_token'] = $value
            ? Crypt::encryptString($value)
            : null;
    }

    /**
     * Get decrypted refresh token
     */
    public function getRefreshTokenAttribute(?string $value): ?string
    {
        if (!$value) {
            return null;
        }

        try {
            return Crypt::decryptString($value);
        } catch (\Exception $e) {
            return null;
        }
    }

    /**
     * Set webhook secret with encryption
     */
    public function setWebhookSecretAttribute(?string $value): void
    {
        $this->attributes['webhook_secret'] = $value
            ? Crypt::encryptString($value)
            : null;
    }

    /**
     * Get decrypted webhook secret
     */
    public function getWebhookSecretAttribute(?string $value): ?string
    {
        if (!$value) {
            return null;
        }

        try {
            return Crypt::decryptString($value);
        } catch (\Exception $e) {
            return null;
        }
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get connection logs
     */
    public function logs(): HasMany
    {
        return $this->hasMany(ApiConnectionLog::class, 'connection_id');
    }

    // =========================================================================
    // STATUS METHODS
    // =========================================================================

    /**
     * Check if connected
     */
    public function isConnected(): bool
    {
        return $this->status === 'connected';
    }

    /**
     * Check if connection has expired
     */
    public function isExpired(): bool
    {
        if (!$this->expires_at) {
            return false;
        }

        return $this->expires_at->isPast();
    }

    /**
     * Check if token needs refresh (within 5 minutes of expiry)
     */
    public function needsRefresh(): bool
    {
        if (!$this->is_oauth || !$this->expires_at) {
            return false;
        }

        return $this->expires_at->subMinutes(5)->isPast();
    }

    /**
     * Check if connection is healthy
     */
    public function isHealthy(): bool
    {
        return $this->health_score >= 80 && $this->status === 'connected';
    }

    /**
     * Get health status label
     */
    public function getHealthStatus(): string
    {
        if ($this->health_score >= 90) return 'excellent';
        if ($this->health_score >= 70) return 'good';
        if ($this->health_score >= 50) return 'fair';
        if ($this->health_score >= 30) return 'poor';
        return 'critical';
    }

    // =========================================================================
    // CREDENTIAL HELPERS
    // =========================================================================

    /**
     * Get a specific credential value
     */
    public function getCredential(string $key, $default = null)
    {
        return $this->credentials[$key] ?? $default;
    }

    /**
     * Get API key from credentials
     */
    public function getApiKey(): ?string
    {
        return $this->getCredential('api_key')
            ?? $this->getCredential('apiKey')
            ?? $this->getCredential('key');
    }

    /**
     * Get secret key from credentials
     */
    public function getSecretKey(): ?string
    {
        return $this->getCredential('secret_key')
            ?? $this->getCredential('secretKey')
            ?? $this->getCredential('secret');
    }

    // =========================================================================
    // API CALL TRACKING
    // =========================================================================

    /**
     * Record an API call
     */
    public function recordApiCall(bool $success = true): void
    {
        $this->increment('api_calls_today');
        $this->increment('api_calls_total');
        $this->last_api_call = now();

        if ($success) {
            $this->increment('success_count');
            $this->recalculateHealthScore();
        } else {
            $this->increment('error_count');
            $this->recalculateHealthScore();
        }

        $this->save();
    }

    /**
     * Record an error
     */
    public function recordError(string $error): void
    {
        $this->last_error = $error;
        $this->increment('error_count');
        $this->recalculateHealthScore();

        if ($this->error_count >= 10 && $this->health_score < 30) {
            $this->status = 'error';
        }

        $this->save();
    }

    /**
     * Recalculate health score based on success/error ratio
     */
    public function recalculateHealthScore(): void
    {
        $total = $this->success_count + $this->error_count;

        if ($total === 0) {
            $this->health_score = 100;
            return;
        }

        $successRate = ($this->success_count / $total) * 100;

        // Weight recent errors more heavily
        $recentErrorPenalty = min($this->error_count * 2, 30);

        $this->health_score = max(0, min(100, (int) ($successRate - $recentErrorPenalty)));
    }

    /**
     * Reset daily API call counter
     */
    public function resetDailyCounter(): void
    {
        $this->api_calls_today = 0;
        $this->save();
    }

    // =========================================================================
    // CONNECTION LIFECYCLE
    // =========================================================================

    /**
     * Mark as connected
     */
    public function markConnected(): void
    {
        $this->status = 'connected';
        $this->connected_at = now();
        $this->last_verified_at = now();
        $this->last_error = null;
        $this->error_count = 0;
        $this->health_score = 100;
        $this->save();
    }

    /**
     * Mark as disconnected
     */
    public function markDisconnected(): void
    {
        $this->status = 'disconnected';
        $this->credentials = null;
        $this->access_token = null;
        $this->refresh_token = null;
        $this->connected_at = null;
        $this->save();
    }

    /**
     * Mark as verified
     */
    public function markVerified(): void
    {
        $this->last_verified_at = now();
        $this->last_health_check = now();

        if ($this->status === 'error') {
            $this->status = 'connected';
        }

        $this->save();
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to connected services
     */
    public function scopeConnected($query)
    {
        return $query->where('status', 'connected');
    }

    /**
     * Scope to disconnected services
     */
    public function scopeDisconnected($query)
    {
        return $query->where('status', 'disconnected');
    }

    /**
     * Scope by category
     */
    public function scopeCategory($query, string $category)
    {
        return $query->where('category', $category);
    }

    /**
     * Scope to services needing attention
     */
    public function scopeNeedsAttention($query)
    {
        return $query->where(function ($q) {
            $q->where('status', 'error')
                ->orWhere('health_score', '<', 50)
                ->orWhere('expires_at', '<', now()->addDays(7));
        });
    }

    /**
     * Scope by environment
     */
    public function scopeEnvironment($query, string $env)
    {
        return $query->where('environment', $env);
    }

    // =========================================================================
    // SERIALIZATION
    // =========================================================================

    /**
     * Get public data for API response (without sensitive fields)
     */
    public function toPublicArray(): array
    {
        return [
            'id' => $this->id,
            'service_key' => $this->service_key,
            'service_name' => $this->service_name,
            'category' => $this->category,
            'status' => $this->status,
            'is_connected' => $this->isConnected(),
            'is_healthy' => $this->isHealthy(),
            'health_score' => $this->health_score,
            'health_status' => $this->getHealthStatus(),
            'connected_at' => $this->connected_at?->toIso8601String(),
            'last_verified_at' => $this->last_verified_at?->toIso8601String(),
            'expires_at' => $this->expires_at?->toIso8601String(),
            'is_expired' => $this->isExpired(),
            'needs_refresh' => $this->needsRefresh(),
            'config' => $this->config,
            'metadata' => $this->metadata,
            'is_oauth' => $this->is_oauth,
            'oauth_scopes' => $this->oauth_scopes,
            'webhooks_enabled' => $this->webhooks_enabled,
            'environment' => $this->environment,
            'api_version' => $this->api_version,
            'api_calls_today' => $this->api_calls_today,
            'api_calls_total' => $this->api_calls_total,
            'last_api_call' => $this->last_api_call?->toIso8601String(),
            'last_error' => $this->last_error,
            'error_count' => $this->error_count,
            'success_count' => $this->success_count,
            'created_at' => $this->created_at?->toIso8601String(),
            'updated_at' => $this->updated_at?->toIso8601String(),
        ];
    }
}
