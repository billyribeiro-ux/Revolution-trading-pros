<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Builder;

/**
 * ConsentRecord Model
 *
 * Stores consent records for GDPR/CCPA compliance.
 * Each record represents a single consent action (grant or withdrawal).
 *
 * @property int $id
 * @property string $identifier
 * @property string $identifier_type
 * @property string $consent_type
 * @property bool $granted
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property string|null $source
 * @property string|null $region
 * @property array|null $metadata
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 * @property \Carbon\Carbon|null $deleted_at
 */
class ConsentRecord extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'consent_records';

    protected $fillable = [
        'identifier',
        'identifier_type',
        'consent_type',
        'granted',
        'ip_address',
        'user_agent',
        'source',
        'region',
        'metadata',
    ];

    protected $casts = [
        'granted' => 'boolean',
        'metadata' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * Identifier types
     */
    public const IDENTIFIER_EMAIL = 'email';
    public const IDENTIFIER_COOKIE = 'cookie_id';
    public const IDENTIFIER_USER = 'user_id';
    public const IDENTIFIER_DELETED = 'deleted';

    /**
     * Consent sources
     */
    public const SOURCE_BANNER = 'consent_banner';
    public const SOURCE_FORM = 'form';
    public const SOURCE_SETTINGS = 'user_settings';
    public const SOURCE_API = 'api';
    public const SOURCE_WITHDRAWAL = 'user_withdrawal';

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to filter by identifier
     */
    public function scopeForIdentifier(Builder $query, string $identifier): Builder
    {
        return $query->where('identifier', $identifier);
    }

    /**
     * Scope to get granted consents
     */
    public function scopeGranted(Builder $query): Builder
    {
        return $query->where('granted', true);
    }

    /**
     * Scope to get withdrawn consents
     */
    public function scopeWithdrawn(Builder $query): Builder
    {
        return $query->where('granted', false);
    }

    /**
     * Scope to filter by consent type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('consent_type', $type);
    }

    /**
     * Scope to filter by region
     */
    public function scopeFromRegion(Builder $query, string $region): Builder
    {
        return $query->where('region', $region);
    }

    /**
     * Scope to get recent records
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope to get latest consent for each type
     */
    public function scopeLatestPerType(Builder $query): Builder
    {
        return $query->orderBy('consent_type')
            ->orderByDesc('created_at');
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the user if identifier is a user_id
     */
    public function user()
    {
        if ($this->identifier_type !== self::IDENTIFIER_USER) {
            return null;
        }

        return $this->belongsTo(User::class, 'identifier', 'id');
    }

    /**
     * Get the contact if identifier is an email
     */
    public function contact()
    {
        if ($this->identifier_type !== self::IDENTIFIER_EMAIL) {
            return null;
        }

        return $this->belongsTo(Contact::class, 'identifier', 'email');
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    /**
     * Get the human-readable consent type name
     */
    public function getConsentTypeNameAttribute(): string
    {
        $names = [
            'necessary' => 'Strictly Necessary',
            'functional' => 'Functional',
            'analytics' => 'Analytics',
            'marketing' => 'Marketing',
            'personalization' => 'Personalization',
            'email_marketing' => 'Email Marketing',
            'sms_marketing' => 'SMS Marketing',
            'newsletter' => 'Newsletter',
            'third_party_sharing' => 'Third Party Sharing',
            'profiling' => 'Profiling',
            'analytics_tracking' => 'Analytics Tracking',
            'personalized_ads' => 'Personalized Ads',
            'transactional' => 'Transactional',
        ];

        return $names[$this->consent_type] ?? ucfirst(str_replace('_', ' ', $this->consent_type));
    }

    /**
     * Get the status label
     */
    public function getStatusLabelAttribute(): string
    {
        return $this->granted ? 'Granted' : 'Withdrawn';
    }

    /**
     * Check if this is a category consent
     */
    public function getIsCategoryAttribute(): bool
    {
        $categories = ['necessary', 'functional', 'analytics', 'marketing', 'personalization'];
        return in_array($this->consent_type, $categories);
    }

    /**
     * Check if this is a purpose consent
     */
    public function getIsPurposeAttribute(): bool
    {
        return !$this->is_category;
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Check if consent is currently valid
     */
    public function isValid(): bool
    {
        // Check if consent hasn't been superseded by a newer record
        $newerRecord = static::where('identifier', $this->identifier)
            ->where('consent_type', $this->consent_type)
            ->where('created_at', '>', $this->created_at)
            ->exists();

        return !$newerRecord && $this->granted;
    }

    /**
     * Get the audit trail for this consent type
     */
    public function getAuditTrail(): \Illuminate\Database\Eloquent\Collection
    {
        return static::where('identifier', $this->identifier)
            ->where('consent_type', $this->consent_type)
            ->orderBy('created_at')
            ->get();
    }

    /**
     * Create a withdrawal record
     */
    public function withdraw(array $metadata = []): static
    {
        return static::create([
            'identifier' => $this->identifier,
            'identifier_type' => $this->identifier_type,
            'consent_type' => $this->consent_type,
            'granted' => false,
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
            'source' => self::SOURCE_WITHDRAWAL,
            'region' => $this->region,
            'metadata' => array_merge($metadata, [
                'withdrawn_from' => $this->id,
                'withdrawal_date' => now()->toIso8601String(),
            ]),
        ]);
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Get current consent status for an identifier
     */
    public static function getCurrentStatus(string $identifier): array
    {
        $records = static::where('identifier', $identifier)
            ->latest()
            ->get()
            ->groupBy('consent_type')
            ->map(fn($group) => $group->first());

        return $records->mapWithKeys(fn($record, $type) => [
            $type => [
                'granted' => $record->granted,
                'date' => $record->created_at,
                'source' => $record->source,
            ]
        ])->toArray();
    }

    /**
     * Check if any consent exists for identifier
     */
    public static function hasAnyConsent(string $identifier): bool
    {
        return static::where('identifier', $identifier)
            ->where('granted', true)
            ->exists();
    }

    /**
     * Get consent statistics
     */
    public static function getStatistics(int $days = 30): array
    {
        $records = static::recent($days)->get();

        return [
            'total_records' => $records->count(),
            'grants' => $records->where('granted', true)->count(),
            'withdrawals' => $records->where('granted', false)->count(),
            'by_type' => $records->groupBy('consent_type')->map->count(),
            'by_source' => $records->groupBy('source')->map->count(),
            'by_region' => $records->groupBy('region')->map->count(),
            'grant_rate' => $records->count() > 0
                ? round($records->where('granted', true)->count() / $records->count() * 100, 2)
                : 0,
        ];
    }
}
