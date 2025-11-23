<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * Error404 Model - Enterprise-grade 404 error tracking and analytics
 * 
 * Comprehensive 404 error monitoring with hit tracking, resolution management,
 * analytics, and automated cleanup capabilities for maintaining site health.
 *
 * @property int $id
 * @property string $url The requested URL that returned 404
 * @property string|null $referrer HTTP referrer
 * @property string|null $user_agent User agent string
 * @property string|null $ip_address Client IP address
 * @property array|null $metadata Additional tracking metadata
 * @property int $hit_count Number of times this URL was hit
 * @property bool $is_resolved Resolution status
 * @property string|null $resolution_notes Notes about resolution
 * @property int|null $resolved_by_user_id User who resolved this
 * @property Carbon|null $resolved_at Resolution timestamp
 * @property Carbon $first_seen_at First occurrence timestamp
 * @property Carbon $last_seen_at Most recent occurrence timestamp
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read float $average_hits_per_day
 * @property-read int $days_since_first_seen
 * @property-read int $days_since_last_seen
 * @property-read bool $is_critical
 * @property-read bool $is_stale
 * @property-read string $severity_level
 * 
 * @method static Builder unresolved()
 * @method static Builder resolved()
 * @method static Builder critical(int $threshold = 50)
 * @method static Builder frequent(int $minHits = 10)
 * @method static Builder recent(int $days = 7)
 * @method static Builder stale(int $days = 30)
 * @method static Builder byUrl(string $url)
 * @method static Builder byPattern(string $pattern)
 * @method static Builder topErrors(int $limit = 10)
 * @method static Builder orderedByFrequency()
 * @method static Builder orderedByRecent()
 * @method static Builder orderedBySeverity()
 */
class Error404 extends Model
{
    use HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'error_404s';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'url',
        'referrer',
        'user_agent',
        'ip_address',
        'metadata',
        'hit_count',
        'is_resolved',
        'resolution_notes',
        'resolved_by_user_id',
        'resolved_at',
        'first_seen_at',
        'last_seen_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'metadata' => 'array',
        'hit_count' => 'integer',
        'is_resolved' => 'boolean',
        'resolved_by_user_id' => 'integer',
        'resolved_at' => 'datetime',
        'first_seen_at' => 'datetime',
        'last_seen_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'hit_count' => 0,
        'is_resolved' => false,
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<int, string>
     */
    protected $hidden = [
        'ip_address',
        'user_agent',
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'average_hits_per_day',
        'days_since_first_seen',
        'days_since_last_seen',
        'is_critical',
        'is_stale',
        'severity_level',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const SEVERITY_LOW = 'low';
    public const SEVERITY_MEDIUM = 'medium';
    public const SEVERITY_HIGH = 'high';
    public const SEVERITY_CRITICAL = 'critical';

    public const THRESHOLD_CRITICAL = 100;
    public const THRESHOLD_HIGH = 50;
    public const THRESHOLD_MEDIUM = 10;

    public const STALE_DAYS_DEFAULT = 30;
    public const CLEANUP_DAYS_DEFAULT = 90;

    /**
     * Severity levels with thresholds.
     *
     * @var array<string, int>
     */
    public const SEVERITY_THRESHOLDS = [
        self::SEVERITY_CRITICAL => self::THRESHOLD_CRITICAL,
        self::SEVERITY_HIGH => self::THRESHOLD_HIGH,
        self::SEVERITY_MEDIUM => self::THRESHOLD_MEDIUM,
        self::SEVERITY_LOW => 0,
    ];

    /**
     * Severity colors for UI.
     *
     * @var array<string, string>
     */
    public const SEVERITY_COLORS = [
        self::SEVERITY_CRITICAL => 'red',
        self::SEVERITY_HIGH => 'orange',
        self::SEVERITY_MEDIUM => 'yellow',
        self::SEVERITY_LOW => 'gray',
    ];

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Auto-set timestamps on creation
        static::creating(function (Error404 $error) {
            if (empty($error->first_seen_at)) {
                $error->first_seen_at = now();
            }
            if (empty($error->last_seen_at)) {
                $error->last_seen_at = now();
            }

            // Capture request metadata if available
            if (request() && empty($error->metadata)) {
                $error->captureRequestMetadata();
            }
        });

        // Log resolution changes
        static::updating(function (Error404 $error) {
            if ($error->isDirty('is_resolved')) {
                $status = $error->is_resolved ? 'resolved' : 'unresolved';
                
                Log::info("404 Error {$status}", [
                    'id' => $error->id,
                    'url' => $error->url,
                    'hit_count' => $error->hit_count,
                    'resolved_by' => $error->resolved_by_user_id,
                ]);

                if ($error->is_resolved && empty($error->resolved_at)) {
                    $error->resolved_at = now();
                }
            }
        });

        // Clear cache on changes
        static::saved(function (Error404 $error) {
            Cache::forget('error_404_stats');
            Cache::forget('error_404_top_errors');
        });

        static::deleted(function (Error404 $error) {
            Cache::forget('error_404_stats');
            Cache::forget('error_404_top_errors');
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the user who resolved this error.
     */
    public function resolvedBy(): \Illuminate\Database\Eloquent\Relations\BelongsTo
    {
        return $this->belongsTo(\App\Models\User::class, 'resolved_by_user_id')
            ->withDefault([
                'name' => 'System',
            ]);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include unresolved errors.
     */
    public function scopeUnresolved(Builder $query): Builder
    {
        return $query->where('is_resolved', false);
    }

    /**
     * Scope a query to only include resolved errors.
     */
    public function scopeResolved(Builder $query): Builder
    {
        return $query->where('is_resolved', true);
    }

    /**
     * Scope a query to critical errors (high hit count).
     */
    public function scopeCritical(Builder $query, int $threshold = self::THRESHOLD_CRITICAL): Builder
    {
        return $query->where('hit_count', '>=', $threshold)
            ->where('is_resolved', false);
    }

    /**
     * Scope a query to frequent errors.
     */
    public function scopeFrequent(Builder $query, int $minHits = self::THRESHOLD_MEDIUM): Builder
    {
        return $query->where('hit_count', '>=', $minHits);
    }

    /**
     * Scope a query to recent errors.
     */
    public function scopeRecent(Builder $query, int $days = 7): Builder
    {
        return $query->where('last_seen_at', '>=', now()->subDays($days));
    }

    /**
     * Scope a query to stale errors (not seen recently).
     */
    public function scopeStale(Builder $query, int $days = self::STALE_DAYS_DEFAULT): Builder
    {
        return $query->where('last_seen_at', '<', now()->subDays($days));
    }

    /**
     * Scope a query by exact URL.
     */
    public function scopeByUrl(Builder $query, string $url): Builder
    {
        return $query->where('url', $url);
    }

    /**
     * Scope a query by URL pattern.
     */
    public function scopeByPattern(Builder $query, string $pattern): Builder
    {
        return $query->where('url', 'LIKE', $pattern);
    }

    /**
     * Scope a query to top errors by hit count.
     */
    public function scopeTopErrors(Builder $query, int $limit = 10): Builder
    {
        return $query->orderBy('hit_count', 'desc')->limit($limit);
    }

    /**
     * Scope to order by frequency (hit count).
     */
    public function scopeOrderedByFrequency(Builder $query): Builder
    {
        return $query->orderBy('hit_count', 'desc');
    }

    /**
     * Scope to order by most recent.
     */
    public function scopeOrderedByRecent(Builder $query): Builder
    {
        return $query->orderBy('last_seen_at', 'desc');
    }

    /**
     * Scope to order by severity level.
     */
    public function scopeOrderedBySeverity(Builder $query): Builder
    {
        return $query->orderByRaw(
            "CASE 
                WHEN hit_count >= ? THEN 1
                WHEN hit_count >= ? THEN 2
                WHEN hit_count >= ? THEN 3
                ELSE 4
            END",
            [
                self::THRESHOLD_CRITICAL,
                self::THRESHOLD_HIGH,
                self::THRESHOLD_MEDIUM
            ]
        )->orderBy('hit_count', 'desc');
    }

    /**
     * Scope to errors by referrer.
     */
    public function scopeByReferrer(Builder $query, string $referrer): Builder
    {
        return $query->where('referrer', 'LIKE', "%{$referrer}%");
    }

    /**
     * Scope to errors from specific IP.
     */
    public function scopeByIp(Builder $query, string $ip): Builder
    {
        return $query->where('ip_address', $ip);
    }

    /**
     * Scope to errors within date range.
     */
    public function scopeBetweenDates(Builder $query, Carbon $startDate, Carbon $endDate): Builder
    {
        return $query->whereBetween('last_seen_at', [$startDate, $endDate]);
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Get average hits per day since first seen.
     */
    public function getAverageHitsPerDayAttribute(): float
    {
        $days = max(1, $this->getDaysSinceFirstSeen());
        return round($this->hit_count / $days, 2);
    }

    /**
     * Get days since first seen.
     */
    public function getDaysSinceFirstSeenAttribute(): int
    {
        return $this->getDaysSinceFirstSeen();
    }

    /**
     * Get days since last seen.
     */
    public function getDaysSinceLastSeenAttribute(): int
    {
        return $this->getDaysSinceLastSeen();
    }

    /**
     * Check if error is critical.
     */
    public function getIsCriticalAttribute(): bool
    {
        return $this->isCritical();
    }

    /**
     * Check if error is stale.
     */
    public function getIsStaleAttribute(): bool
    {
        return $this->isStale();
    }

    /**
     * Get severity level.
     */
    public function getSeverityLevelAttribute(): string
    {
        return $this->getSeverityLevel();
    }

    /**
     * Get severity color for UI.
     */
    public function getSeverityColorAttribute(): string
    {
        return self::SEVERITY_COLORS[$this->severity_level] ?? 'gray';
    }

    /**
     * Normalize URL on set.
     */
    public function setUrlAttribute(?string $value): void
    {
        $this->attributes['url'] = $value ? $this->normalizeUrl($value) : null;
    }

    // =========================================================================
    // HIT TRACKING METHODS
    // =========================================================================

    /**
     * Increment hit count and update last seen timestamp.
     */
    public function incrementHits(): void
    {
        $this->increment('hit_count');
        $this->update(['last_seen_at' => now()]);
    }

    /**
     * Record a hit with optional metadata update.
     */
    public function recordInstanceHit(array $additionalMetadata = []): void
    {
        $this->increment('hit_count');
        
        $updates = ['last_seen_at' => now()];

        if (!empty($additionalMetadata)) {
            $metadata = $this->metadata ?? [];
            $metadata = array_merge($metadata, $additionalMetadata);
            $updates['metadata'] = $metadata;
        }

        $this->update($updates);
    }

    /**
     * Reset hit count.
     */
    public function resetHits(): bool
    {
        $this->hit_count = 0;
        return $this->save();
    }

    // =========================================================================
    // RESOLUTION MANAGEMENT METHODS
    // =========================================================================

    /**
     * Mark error as resolved.
     */
    public function resolve(?string $notes = null, ?int $userId = null): bool
    {
        $this->is_resolved = true;
        $this->resolved_at = now();
        $this->resolution_notes = $notes;
        $this->resolved_by_user_id = $userId;

        return $this->save();
    }

    /**
     * Mark error as unresolved.
     */
    public function unresolve(): bool
    {
        $this->is_resolved = false;
        $this->resolved_at = null;
        $this->resolution_notes = null;
        $this->resolved_by_user_id = null;

        return $this->save();
    }

    /**
     * Alias for resolve().
     */
    public function markAsResolved(?string $notes = null, ?int $userId = null): bool
    {
        return $this->resolve($notes, $userId);
    }

    /**
     * Toggle resolution status.
     */
    public function toggleResolution(?int $userId = null): bool
    {
        if ($this->is_resolved) {
            return $this->unresolve();
        }

        return $this->resolve(null, $userId);
    }

    // =========================================================================
    // ANALYTICS & CALCULATIONS
    // =========================================================================

    /**
     * Get days since first seen.
     */
    public function getDaysSinceFirstSeen(): int
    {
        return (int) $this->first_seen_at->diffInDays(now());
    }

    /**
     * Get days since last seen.
     */
    public function getDaysSinceLastSeen(): int
    {
        return (int) $this->last_seen_at->diffInDays(now());
    }

    /**
     * Get hours since last seen.
     */
    public function getHoursSinceLastSeen(): int
    {
        return (int) $this->last_seen_at->diffInHours(now());
    }

    /**
     * Calculate average hits per day.
     */
    public function calculateAverageHitsPerDay(): float
    {
        return $this->average_hits_per_day;
    }

    /**
     * Check if error is stale (not seen recently).
     */
    public function isStale(int $days = self::STALE_DAYS_DEFAULT): bool
    {
        return $this->last_seen_at->lt(now()->subDays($days));
    }

    /**
     * Check if error is critical (high hit count and unresolved).
     */
    public function isCritical(int $threshold = self::THRESHOLD_CRITICAL): bool
    {
        return $this->hit_count >= $threshold && !$this->is_resolved;
    }

    /**
     * Check if error is high severity.
     */
    public function isHighSeverity(): bool
    {
        return $this->hit_count >= self::THRESHOLD_HIGH && !$this->is_resolved;
    }

    /**
     * Get severity level based on hit count.
     */
    public function getSeverityLevel(): string
    {
        if ($this->is_resolved) {
            return self::SEVERITY_LOW;
        }

        if ($this->hit_count >= self::THRESHOLD_CRITICAL) {
            return self::SEVERITY_CRITICAL;
        }

        if ($this->hit_count >= self::THRESHOLD_HIGH) {
            return self::SEVERITY_HIGH;
        }

        if ($this->hit_count >= self::THRESHOLD_MEDIUM) {
            return self::SEVERITY_MEDIUM;
        }

        return self::SEVERITY_LOW;
    }

    /**
     * Check if URL matches a pattern.
     */
    public function matchesPattern(string $pattern): bool
    {
        return fnmatch($pattern, $this->url);
    }

    /**
     * Get URL path segments.
     */
    public function getUrlSegments(): array
    {
        $path = parse_url($this->url, PHP_URL_PATH);
        return array_filter(explode('/', $path ?? ''));
    }

    /**
     * Get URL extension if present.
     */
    public function getUrlExtension(): ?string
    {
        return pathinfo($this->url, PATHINFO_EXTENSION) ?: null;
    }

    // =========================================================================
    // METADATA MANAGEMENT
    // =========================================================================

    /**
     * Capture request metadata automatically.
     */
    protected function captureRequestMetadata(): void
    {
        if (!request()) {
            return;
        }

        $this->metadata = [
            'method' => request()->method(),
            'query_string' => request()->getQueryString(),
            'timestamp' => now()->toIso8601String(),
        ];
    }

    /**
     * Add metadata entry.
     */
    public function addMetadata(string $key, mixed $value): bool
    {
        $metadata = $this->metadata ?? [];
        $metadata[$key] = $value;
        $this->metadata = $metadata;
        return $this->save();
    }

    /**
     * Get metadata value by key.
     */
    public function getMetadata(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Remove metadata entry.
     */
    public function removeMetadata(string $key): bool
    {
        $metadata = $this->metadata ?? [];
        unset($metadata[$key]);
        $this->metadata = $metadata;
        return $this->save();
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /**
     * Normalize URL for consistency.
     */
    protected function normalizeUrl(string $url): string
    {
        // Remove query parameters
        $url = strtok($url, '?');
        
        // Remove trailing slash
        $url = rtrim($url, '/');
        
        // Convert to lowercase
        $url = strtolower($url);

        return $url;
    }

    /**
     * Get error summary for display.
     */
    public function toSummary(): array
    {
        return [
            'id' => $this->id,
            'url' => $this->url,
            'hit_count' => $this->hit_count,
            'severity' => $this->severity_level,
            'is_resolved' => $this->is_resolved,
            'is_critical' => $this->is_critical,
            'is_stale' => $this->is_stale,
            'first_seen' => $this->first_seen_at->toDateTimeString(),
            'last_seen' => $this->last_seen_at->toDateTimeString(),
            'days_active' => $this->days_since_first_seen,
            'avg_hits_per_day' => $this->average_hits_per_day,
            'referrer' => $this->referrer,
            'resolved_at' => $this->resolved_at?->toDateTimeString(),
            'resolution_notes' => $this->resolution_notes,
        ];
    }

    /**
     * Export error details.
     */
    public function toExport(): array
    {
        return [
            'URL' => $this->url,
            'Hit Count' => $this->hit_count,
            'Severity' => ucfirst($this->severity_level),
            'Status' => $this->is_resolved ? 'Resolved' : 'Unresolved',
            'First Seen' => $this->first_seen_at->toDateString(),
            'Last Seen' => $this->last_seen_at->toDateString(),
            'Days Active' => $this->days_since_first_seen,
            'Avg Hits/Day' => $this->average_hits_per_day,
            'Referrer' => $this->referrer ?? 'Direct',
            'Resolved At' => $this->resolved_at?->toDateString() ?? 'N/A',
        ];
    }

    // =========================================================================
    // STATIC FACTORY & BULK METHODS
    // =========================================================================

    /**
     * Record a 404 hit (create or update).
     */
    public static function recordHit(
        string $url,
        ?string $referrer = null,
        ?string $userAgent = null,
        ?string $ipAddress = null,
        array $metadata = []
    ): self {
        $normalizedUrl = (new self())->normalizeUrl($url);

        $error = self::firstOrCreate(
            ['url' => $normalizedUrl],
            [
                'referrer' => $referrer,
                'user_agent' => $userAgent,
                'ip_address' => $ipAddress,
                'metadata' => $metadata,
                'hit_count' => 0,
                'first_seen_at' => now(),
                'last_seen_at' => now(),
            ]
        );

        if (!$error->wasRecentlyCreated) {
            $error->recordInstanceHit($metadata);
        } else {
            $error->increment('hit_count');
        }

        return $error;
    }

    /**
     * Bulk resolve errors.
     *
     * @param array<int> $ids
     */
    public static function bulkResolve(array $ids, ?string $notes = null, ?int $userId = null): int
    {
        return self::whereIn('id', $ids)->update([
            'is_resolved' => true,
            'resolved_at' => now(),
            'resolution_notes' => $notes,
            'resolved_by_user_id' => $userId,
        ]);
    }

    /**
     * Bulk unresolve errors.
     *
     * @param array<int> $ids
     */
    public static function bulkUnresolve(array $ids): int
    {
        return self::whereIn('id', $ids)->update([
            'is_resolved' => false,
            'resolved_at' => null,
            'resolution_notes' => null,
            'resolved_by_user_id' => null,
        ]);
    }

    /**
     * Bulk delete errors.
     *
     * @param array<int> $ids
     */
    public static function bulkDelete(array $ids): int
    {
        return self::whereIn('id', $ids)->delete();
    }

    /**
     * Clean up stale resolved errors.
     */
    public static function cleanupStale(int $days = self::CLEANUP_DAYS_DEFAULT): int
    {
        return self::where('last_seen_at', '<', now()->subDays($days))
            ->where('is_resolved', true)
            ->delete();
    }

    /**
     * Clean up old resolved errors.
     */
    public static function cleanupResolved(int $days = self::STALE_DAYS_DEFAULT): int
    {
        return self::where('is_resolved', true)
            ->where('resolved_at', '<', now()->subDays($days))
            ->delete();
    }

    /**
     * Clean up low-priority errors.
     */
    public static function cleanupLowPriority(int $maxHits = 5, int $days = 60): int
    {
        return self::where('hit_count', '<=', $maxHits)
            ->where('last_seen_at', '<', now()->subDays($days))
            ->delete();
    }

    /**
     * Get comprehensive statistics.
     */
    public static function getStats(): array
    {
        return Cache::remember('error_404_stats', 300, function () {
            return [
                'total' => self::count(),
                'unresolved' => self::unresolved()->count(),
                'resolved' => self::resolved()->count(),
                'critical' => self::critical()->count(),
                'high_severity' => self::unresolved()->frequent(self::THRESHOLD_HIGH)->count(),
                'medium_severity' => self::unresolved()
                    ->frequent(self::THRESHOLD_MEDIUM)
                    ->where('hit_count', '<', self::THRESHOLD_HIGH)
                    ->count(),
                'total_hits' => self::sum('hit_count'),
                'recent_7d' => self::recent(7)->count(),
                'recent_24h' => self::recent(1)->count(),
                'stale' => self::stale()->count(),
                'avg_hits_per_error' => round(self::avg('hit_count'), 2),
            ];
        });
    }

    /**
     * Get top errors with caching.
     */
    public static function getTopErrors(int $limit = 10): Collection
    {
        return Cache::remember("error_404_top_errors_{$limit}", 300, function () use ($limit) {
            return self::unresolved()
                ->topErrors($limit)
                ->get(['id', 'url', 'hit_count', 'last_seen_at', 'severity_level']);
        });
    }

    /**
     * Get errors by severity distribution.
     */
    public static function getSeverityDistribution(): array
    {
        $errors = self::unresolved()->get();

        return [
            self::SEVERITY_CRITICAL => $errors->filter(fn($e) => $e->isCritical())->count(),
            self::SEVERITY_HIGH => $errors->filter(fn($e) => $e->isHighSeverity() && !$e->isCritical())->count(),
            self::SEVERITY_MEDIUM => $errors->filter(fn($e) => $e->hit_count >= self::THRESHOLD_MEDIUM && $e->hit_count < self::THRESHOLD_HIGH)->count(),
            self::SEVERITY_LOW => $errors->filter(fn($e) => $e->hit_count < self::THRESHOLD_MEDIUM)->count(),
        ];
    }

    /**
     * Get errors grouped by URL pattern.
     */
    public static function getPatternAnalysis(): array
    {
        $errors = self::unresolved()->get();
        $patterns = [];

        foreach ($errors as $error) {
            $segments = $error->getUrlSegments();
            $firstSegment = $segments[0] ?? 'root';
            
            if (!isset($patterns[$firstSegment])) {
                $patterns[$firstSegment] = [
                    'count' => 0,
                    'total_hits' => 0,
                    'urls' => [],
                ];
            }

            $patterns[$firstSegment]['count']++;
            $patterns[$firstSegment]['total_hits'] += $error->hit_count;
            $patterns[$firstSegment]['urls'][] = $error->url;
        }

        return $patterns;
    }

    /**
     * Export all errors to array format.
     */
    public static function exportAll(?Builder $query = null): array
    {
        $errors = $query ? $query->get() : self::all();
        return $errors->map->toExport()->toArray();
    }
}