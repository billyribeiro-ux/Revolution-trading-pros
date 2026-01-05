<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Integration Credential Model
 *
 * Securely stores OAuth credentials for external integrations
 * (Google Search Console, Google Analytics, etc.)
 *
 * @property string $id
 * @property int $user_id
 * @property string $provider
 * @property string $access_token
 * @property string|null $refresh_token
 * @property \Carbon\Carbon|null $expires_at
 * @property string|null $scopes
 * @property array|null $metadata
 * @property bool $is_active
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class IntegrationCredential extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'user_id',
        'provider',
        'access_token',
        'refresh_token',
        'expires_at',
        'scopes',
        'metadata',
        'is_active',
    ];

    protected $casts = [
        'expires_at' => 'datetime',
        'metadata' => 'array',
        'is_active' => 'boolean',
    ];

    protected $hidden = [
        'access_token',
        'refresh_token',
    ];

    /**
     * Get the user that owns the credential
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Check if the token is expired
     */
    public function isExpired(): bool
    {
        if (!$this->expires_at) {
            return false;
        }

        return $this->expires_at->isPast();
    }

    /**
     * Check if the token needs refresh (expires within 5 minutes)
     */
    public function needsRefresh(): bool
    {
        if (!$this->expires_at) {
            return false;
        }

        return $this->expires_at->subMinutes(5)->isPast();
    }

    /**
     * Scope to get credentials by provider
     */
    public function scopeProvider($query, string $provider)
    {
        return $query->where('provider', $provider);
    }

    /**
     * Scope to get active credentials
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    /**
     * Get property ID from metadata (for GA4)
     */
    public function getPropertyId(): ?string
    {
        return $this->metadata['property_id'] ?? null;
    }

    /**
     * Get site URL from metadata (for GSC)
     */
    public function getSiteUrl(): ?string
    {
        return $this->metadata['site_url'] ?? null;
    }
}
