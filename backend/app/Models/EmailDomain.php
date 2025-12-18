<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * EmailDomain Model
 *
 * Represents a verified sending domain for email marketing.
 *
 * @property int $id
 * @property string $domain
 * @property string $verification_token
 * @property bool $verified
 * @property \Carbon\Carbon|null $verified_at
 * @property string|null $dkim_selector
 * @property bool $dkim_verified
 * @property bool $spf_verified
 * @property bool $dmarc_verified
 * @property \Carbon\Carbon|null $last_checked_at
 * @property int|null $created_by
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class EmailDomain extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'domain',
        'verification_token',
        'verified',
        'verified_at',
        'dkim_selector',
        'dkim_verified',
        'spf_verified',
        'dmarc_verified',
        'last_checked_at',
        'created_by',
    ];

    protected $casts = [
        'verified' => 'boolean',
        'verified_at' => 'datetime',
        'dkim_verified' => 'boolean',
        'spf_verified' => 'boolean',
        'dmarc_verified' => 'boolean',
        'last_checked_at' => 'datetime',
    ];

    /**
     * Get the creator of this domain
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Scope for verified domains
     */
    public function scopeVerified($query)
    {
        return $query->where('verified', true);
    }

    /**
     * Check if domain is fully authenticated
     */
    public function isFullyAuthenticated(): bool
    {
        return $this->spf_verified && $this->dkim_verified && $this->dmarc_verified;
    }

    /**
     * Get health score (0-100)
     */
    public function getHealthScoreAttribute(): int
    {
        $score = 0;

        if ($this->verified) {
            $score += 20;
        }
        if ($this->spf_verified) {
            $score += 25;
        }
        if ($this->dkim_verified) {
            $score += 30;
        }
        if ($this->dmarc_verified) {
            $score += 25;
        }

        return $score;
    }
}
