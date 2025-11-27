<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * WebAuthn Credential Model
 *
 * Stores WebAuthn/Passkey credentials for biometric authentication
 *
 * @property string $id
 * @property int $user_id
 * @property string $credential_id
 * @property string $public_key
 * @property string $name
 * @property int $counter
 * @property array $transports
 * @property string|null $aaguid
 * @property \Carbon\Carbon|null $last_used_at
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class WebAuthnCredential extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'user_id',
        'credential_id',
        'public_key',
        'name',
        'counter',
        'transports',
        'aaguid',
        'last_used_at',
    ];

    protected $casts = [
        'counter' => 'integer',
        'transports' => 'array',
        'last_used_at' => 'datetime',
    ];

    protected $hidden = [
        'public_key', // Don't expose the public key in API responses
    ];

    /**
     * Get the user that owns this credential
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Check if the credential was recently used
     */
    public function wasRecentlyUsed(int $minutes = 30): bool
    {
        if (!$this->last_used_at) {
            return false;
        }

        return $this->last_used_at->isAfter(now()->subMinutes($minutes));
    }

    /**
     * Scope to get unused credentials (for cleanup)
     */
    public function scopeUnused($query, int $days = 365)
    {
        return $query->where(function ($q) use ($days) {
            $q->whereNull('last_used_at')
                ->orWhere('last_used_at', '<', now()->subDays($days));
        });
    }
}
