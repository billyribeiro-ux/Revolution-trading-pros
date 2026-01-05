<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Support\Collection;
use InvalidArgumentException;
use Carbon\Carbon;

/**
 * SEO Alert Model
 * 
 * Manages SEO-related alerts and notifications for monitoring website health.
 * Tracks issues like broken links, missing meta tags, ranking drops, and technical SEO problems.
 *
 * @property int $id
 * @property string $type Alert type (broken_link, missing_meta, ranking_drop, etc.)
 * @property string $severity Severity level (critical, high, medium, low, info)
 * @property string $title Alert title
 * @property string $message Detailed alert message
 * @property array $metadata Additional contextual data
 * @property bool $is_new Whether alert is unread
 * @property bool $is_resolved Whether issue has been resolved
 * @property string|null $resolved_by User who resolved the alert
 * @property \Illuminate\Support\Carbon|null $resolved_at When alert was resolved
 * @property string|null $resolution_notes Notes about resolution
 * @property string|null $alertable_type Related entity type (optional)
 * @property int|null $alertable_id Related entity ID (optional)
 * @property int $occurrence_count Number of times this alert occurred
 * @property \Illuminate\Support\Carbon|null $last_occurred_at Last occurrence timestamp
 * @property \Illuminate\Support\Carbon|null $acknowledged_at When alert was acknowledged
 * @property string|null $acknowledged_by User who acknowledged
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Model|null $alertable
 */
class SeoAlert extends Model
{
    use HasFactory;

    /**
     * Alert types
     */
    public const TYPE_BROKEN_LINK = 'broken_link';
    public const TYPE_MISSING_META = 'missing_meta';
    public const TYPE_DUPLICATE_CONTENT = 'duplicate_content';
    public const TYPE_RANKING_DROP = 'ranking_drop';
    public const TYPE_RANKING_GAIN = 'ranking_gain';
    public const TYPE_PAGE_ERROR = 'page_error';
    public const TYPE_SLOW_PAGE = 'slow_page';
    public const TYPE_MOBILE_ISSUE = 'mobile_issue';
    public const TYPE_CRAWL_ERROR = 'crawl_error';
    public const TYPE_INDEXING_ISSUE = 'indexing_issue';
    public const TYPE_SECURITY_ISSUE = 'security_issue';
    public const TYPE_KEYWORD_OPPORTUNITY = 'keyword_opportunity';
    public const TYPE_COMPETITOR_ALERT = 'competitor_alert';
    public const TYPE_BACKLINK_LOST = 'backlink_lost';
    public const TYPE_BACKLINK_GAINED = 'backlink_gained';
    public const TYPE_CONTENT_ISSUE = 'content_issue';
    public const TYPE_SCHEMA_ERROR = 'schema_error';
    public const TYPE_SITEMAP_ERROR = 'sitemap_error';
    public const TYPE_ROBOTS_TXT_ERROR = 'robots_txt_error';

    /**
     * Severity levels
     */
    public const SEVERITY_CRITICAL = 'critical';
    public const SEVERITY_HIGH = 'high';
    public const SEVERITY_MEDIUM = 'medium';
    public const SEVERITY_LOW = 'low';
    public const SEVERITY_INFO = 'info';

    /**
     * Valid alert types
     */
    public const VALID_TYPES = [
        self::TYPE_BROKEN_LINK,
        self::TYPE_MISSING_META,
        self::TYPE_DUPLICATE_CONTENT,
        self::TYPE_RANKING_DROP,
        self::TYPE_RANKING_GAIN,
        self::TYPE_PAGE_ERROR,
        self::TYPE_SLOW_PAGE,
        self::TYPE_MOBILE_ISSUE,
        self::TYPE_CRAWL_ERROR,
        self::TYPE_INDEXING_ISSUE,
        self::TYPE_SECURITY_ISSUE,
        self::TYPE_KEYWORD_OPPORTUNITY,
        self::TYPE_COMPETITOR_ALERT,
        self::TYPE_BACKLINK_LOST,
        self::TYPE_BACKLINK_GAINED,
        self::TYPE_CONTENT_ISSUE,
        self::TYPE_SCHEMA_ERROR,
        self::TYPE_SITEMAP_ERROR,
        self::TYPE_ROBOTS_TXT_ERROR,
    ];

    /**
     * Valid severity levels
     */
    public const VALID_SEVERITIES = [
        self::SEVERITY_CRITICAL,
        self::SEVERITY_HIGH,
        self::SEVERITY_MEDIUM,
        self::SEVERITY_LOW,
        self::SEVERITY_INFO,
    ];

    /**
     * Severity priority (for sorting)
     */
    public const SEVERITY_PRIORITY = [
        self::SEVERITY_CRITICAL => 5,
        self::SEVERITY_HIGH => 4,
        self::SEVERITY_MEDIUM => 3,
        self::SEVERITY_LOW => 2,
        self::SEVERITY_INFO => 1,
    ];

    protected $fillable = [
        'type',
        'severity',
        'title',
        'message',
        'metadata',
        'is_new',
        'is_resolved',
        'resolved_by',
        'resolved_at',
        'resolution_notes',
        'alertable_type',
        'alertable_id',
        'occurrence_count',
        'last_occurred_at',
        'acknowledged_at',
        'acknowledged_by',
    ];

    protected $casts = [
        'metadata' => 'array',
        'is_new' => 'boolean',
        'is_resolved' => 'boolean',
        'resolved_at' => 'datetime',
        'last_occurred_at' => 'datetime',
        'acknowledged_at' => 'datetime',
        'occurrence_count' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'is_new' => true,
        'is_resolved' => false,
        'occurrence_count' => 1,
        'metadata' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->validateType();
            $model->validateSeverity();
            $model->last_occurred_at = $model->last_occurred_at ?? now();
        });

        static::updating(function (self $model): void {
            $model->validateType();
            $model->validateSeverity();
            
            // Auto-set resolved_at when marking as resolved
            if ($model->isDirty('is_resolved') && $model->is_resolved && !$model->resolved_at) {
                $model->resolved_at = now();
            }
        });
    }

    /**
     * Get the parent alertable model (optional polymorphic relation)
     */
    public function alertable(): MorphTo
    {
        return $this->morphTo();
    }

    /**
     * Validate alert type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf(
                    'Invalid alert type "%s". Valid types: %s',
                    $this->type,
                    implode(', ', self::VALID_TYPES)
                )
            );
        }
    }

    /**
     * Validate severity level
     */
    protected function validateSeverity(): void
    {
        if (!in_array($this->severity, self::VALID_SEVERITIES, true)) {
            throw new InvalidArgumentException(
                sprintf(
                    'Invalid severity "%s". Valid severities: %s',
                    $this->severity,
                    implode(', ', self::VALID_SEVERITIES)
                )
            );
        }
    }

    /**
     * Mark alert as read
     */
    public function markAsRead(?string $userId = null): self
    {
        $this->update([
            'is_new' => false,
            'acknowledged_at' => now(),
            'acknowledged_by' => $userId,
        ]);

        return $this;
    }

    /**
     * Mark alert as unread
     */
    public function markAsUnread(): self
    {
        $this->update([
            'is_new' => true,
            'acknowledged_at' => null,
            'acknowledged_by' => null,
        ]);

        return $this;
    }

    /**
     * Acknowledge alert without resolving
     */
    public function acknowledge(?string $userId = null): self
    {
        return $this->markAsRead($userId);
    }

    /**
     * Resolve the alert
     */
    public function resolve(?string $resolvedBy = null, ?string $notes = null): self
    {
        $this->update([
            'is_resolved' => true,
            'is_new' => false,
            'resolved_at' => now(),
            'resolved_by' => $resolvedBy,
            'resolution_notes' => $notes,
        ]);

        return $this;
    }

    /**
     * Unresolve the alert
     */
    public function unresolve(): self
    {
        $this->update([
            'is_resolved' => false,
            'resolved_at' => null,
            'resolved_by' => null,
            'resolution_notes' => null,
        ]);

        return $this;
    }

    /**
     * Increment occurrence count
     */
    public function recordOccurrence(): self
    {
        $this->increment('occurrence_count');
        $this->update([
            'last_occurred_at' => now(),
            'is_new' => true, // Mark as new again if it reoccurs
        ]);

        return $this;
    }

    /**
     * Check if alert is critical
     */
    public function isCritical(): bool
    {
        return $this->severity === self::SEVERITY_CRITICAL;
    }

    /**
     * Check if alert is high priority
     */
    public function isHighPriority(): bool
    {
        return in_array($this->severity, [self::SEVERITY_CRITICAL, self::SEVERITY_HIGH], true);
    }

    /**
     * Check if alert is informational
     */
    public function isInformational(): bool
    {
        return $this->severity === self::SEVERITY_INFO;
    }

    /**
     * Check if alert is positive (opportunity/gain)
     */
    public function isPositive(): bool
    {
        return in_array($this->type, [
            self::TYPE_RANKING_GAIN,
            self::TYPE_BACKLINK_GAINED,
            self::TYPE_KEYWORD_OPPORTUNITY,
        ], true);
    }

    /**
     * Check if alert is negative (issue/problem)
     */
    public function isNegative(): bool
    {
        return !$this->isPositive() && !$this->isInformational();
    }

    /**
     * Check if alert is stale (old and unresolved)
     */
    public function isStale(int $days = 30): bool
    {
        return !$this->is_resolved 
            && $this->created_at->addDays($days)->isPast();
    }

    /**
     * Check if alert was recently created
     */
    public function isRecent(int $hours = 24): bool
    {
        return $this->created_at->isAfter(now()->subHours($hours));
    }

    /**
     * Check if alert is recurring (multiple occurrences)
     */
    public function isRecurring(): bool
    {
        return $this->occurrence_count > 1;
    }

    /**
     * Get severity priority value
     */
    public function getSeverityPriorityAttribute(): int
    {
        return self::SEVERITY_PRIORITY[$this->severity] ?? 0;
    }

    /**
     * Get severity color for UI
     */
    public function getSeverityColorAttribute(): string
    {
        return match($this->severity) {
            self::SEVERITY_CRITICAL => '#dc2626', // red-600
            self::SEVERITY_HIGH => '#ea580c', // orange-600
            self::SEVERITY_MEDIUM => '#f59e0b', // amber-500
            self::SEVERITY_LOW => '#3b82f6', // blue-500
            self::SEVERITY_INFO => '#6b7280', // gray-500
        };
    }

    /**
     * Get severity icon for UI
     */
    public function getSeverityIconAttribute(): string
    {
        return match($this->severity) {
            self::SEVERITY_CRITICAL => 'alert-circle',
            self::SEVERITY_HIGH => 'alert-triangle',
            self::SEVERITY_MEDIUM => 'info',
            self::SEVERITY_LOW => 'bell',
            self::SEVERITY_INFO => 'message-circle',
        };
    }

    /**
     * Get type icon for UI
     */
    public function getTypeIconAttribute(): string
    {
        return match($this->type) {
            self::TYPE_BROKEN_LINK => 'link-off',
            self::TYPE_MISSING_META => 'file-text',
            self::TYPE_DUPLICATE_CONTENT => 'copy',
            self::TYPE_RANKING_DROP => 'trending-down',
            self::TYPE_RANKING_GAIN => 'trending-up',
            self::TYPE_PAGE_ERROR => 'alert-octagon',
            self::TYPE_SLOW_PAGE => 'clock',
            self::TYPE_MOBILE_ISSUE => 'smartphone',
            self::TYPE_CRAWL_ERROR => 'search',
            self::TYPE_INDEXING_ISSUE => 'database',
            self::TYPE_SECURITY_ISSUE => 'shield-off',
            self::TYPE_KEYWORD_OPPORTUNITY => 'target',
            self::TYPE_COMPETITOR_ALERT => 'users',
            self::TYPE_BACKLINK_LOST => 'link-off',
            self::TYPE_BACKLINK_GAINED => 'link',
            self::TYPE_CONTENT_ISSUE => 'edit',
            self::TYPE_SCHEMA_ERROR => 'code',
            self::TYPE_SITEMAP_ERROR => 'map',
            self::TYPE_ROBOTS_TXT_ERROR => 'file-code',
            default => 'bell',
        };
    }

    /**
     * Get human-readable type label
     */
    public function getTypeLabelAttribute(): string
    {
        return match($this->type) {
            self::TYPE_BROKEN_LINK => 'Broken Link',
            self::TYPE_MISSING_META => 'Missing Meta Tags',
            self::TYPE_DUPLICATE_CONTENT => 'Duplicate Content',
            self::TYPE_RANKING_DROP => 'Ranking Drop',
            self::TYPE_RANKING_GAIN => 'Ranking Gain',
            self::TYPE_PAGE_ERROR => 'Page Error',
            self::TYPE_SLOW_PAGE => 'Slow Page Load',
            self::TYPE_MOBILE_ISSUE => 'Mobile Issue',
            self::TYPE_CRAWL_ERROR => 'Crawl Error',
            self::TYPE_INDEXING_ISSUE => 'Indexing Issue',
            self::TYPE_SECURITY_ISSUE => 'Security Issue',
            self::TYPE_KEYWORD_OPPORTUNITY => 'Keyword Opportunity',
            self::TYPE_COMPETITOR_ALERT => 'Competitor Alert',
            self::TYPE_BACKLINK_LOST => 'Backlink Lost',
            self::TYPE_BACKLINK_GAINED => 'Backlink Gained',
            self::TYPE_CONTENT_ISSUE => 'Content Issue',
            self::TYPE_SCHEMA_ERROR => 'Schema Error',
            self::TYPE_SITEMAP_ERROR => 'Sitemap Error',
            self::TYPE_ROBOTS_TXT_ERROR => 'Robots.txt Error',
            default => 'Unknown',
        };
    }

    /**
     * Get metadata value
     */
    public function getMetadataValue(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Set metadata value
     */
    public function setMetadataValue(string $key, mixed $value): self
    {
        $metadata = $this->metadata;
        $metadata[$key] = $value;
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Get time until alert is considered stale
     */
    public function getStaleInAttribute(): ?Carbon
    {
        if ($this->is_resolved) {
            return null;
        }

        return $this->created_at->addDays(30);
    }

    /**
     * Get resolution time duration
     */
    public function getResolutionDurationAttribute(): ?string
    {
        if (!$this->is_resolved || !$this->resolved_at) {
            return null;
        }

        return $this->created_at->diffForHumans($this->resolved_at, [
            'parts' => 2,
            'short' => true,
        ]);
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Filter by severity
     */
    public function scopeOfSeverity(Builder $query, string $severity): Builder
    {
        return $query->where('severity', $severity);
    }

    /**
     * Scope: Get new/unread alerts
     */
    public function scopeUnread(Builder $query): Builder
    {
        return $query->where('is_new', true);
    }

    /**
     * Scope: Get read alerts
     */
    public function scopeRead(Builder $query): Builder
    {
        return $query->where('is_new', false);
    }

    /**
     * Scope: Get unresolved alerts
     */
    public function scopeUnresolved(Builder $query): Builder
    {
        return $query->where('is_resolved', false);
    }

    /**
     * Scope: Get resolved alerts
     */
    public function scopeResolved(Builder $query): Builder
    {
        return $query->where('is_resolved', true);
    }

    /**
     * Scope: Get critical alerts
     */
    public function scopeCritical(Builder $query): Builder
    {
        return $query->where('severity', self::SEVERITY_CRITICAL);
    }

    /**
     * Scope: Get high priority alerts
     */
    public function scopeHighPriority(Builder $query): Builder
    {
        return $query->whereIn('severity', [self::SEVERITY_CRITICAL, self::SEVERITY_HIGH]);
    }

    /**
     * Scope: Get positive alerts (opportunities/gains)
     */
    public function scopePositive(Builder $query): Builder
    {
        return $query->whereIn('type', [
            self::TYPE_RANKING_GAIN,
            self::TYPE_BACKLINK_GAINED,
            self::TYPE_KEYWORD_OPPORTUNITY,
        ]);
    }

    /**
     * Scope: Get negative alerts (issues/problems)
     */
    public function scopeNegative(Builder $query): Builder
    {
        return $query->whereNotIn('type', [
            self::TYPE_RANKING_GAIN,
            self::TYPE_BACKLINK_GAINED,
            self::TYPE_KEYWORD_OPPORTUNITY,
        ])->where('severity', '!=', self::SEVERITY_INFO);
    }

    /**
     * Scope: Get stale alerts
     */
    public function scopeStale(Builder $query, int $days = 30): Builder
    {
        return $query->where('is_resolved', false)
            ->where('created_at', '<', now()->subDays($days));
    }

    /**
     * Scope: Get recent alerts
     */
    public function scopeRecent(Builder $query, int $hours = 24): Builder
    {
        return $query->where('created_at', '>=', now()->subHours($hours));
    }

    /**
     * Scope: Get recurring alerts
     */
    public function scopeRecurring(Builder $query): Builder
    {
        return $query->where('occurrence_count', '>', 1);
    }

    /**
     * Scope: Get acknowledged alerts
     */
    public function scopeAcknowledged(Builder $query): Builder
    {
        return $query->whereNotNull('acknowledged_at');
    }

    /**
     * Scope: Order by severity priority
     */
    public function scopeOrderBySeverity(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderByRaw(
            "CASE severity 
                WHEN 'critical' THEN 5 
                WHEN 'high' THEN 4 
                WHEN 'medium' THEN 3 
                WHEN 'low' THEN 2 
                WHEN 'info' THEN 1 
                ELSE 0 
            END {$direction}"
        );
    }

    /**
     * Scope: Order by occurrence count
     */
    public function scopeOrderByOccurrences(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('occurrence_count', $direction);
    }

    /**
     * Static: Create or increment alert
     */
    public static function createOrIncrement(array $attributes): self
    {
        $existing = static::where('type', $attributes['type'])
            ->where('is_resolved', false)
            ->when(
                isset($attributes['alertable_type']) && isset($attributes['alertable_id']),
                fn($q) => $q->where('alertable_type', $attributes['alertable_type'])
                    ->where('alertable_id', $attributes['alertable_id'])
            )
            ->first();

        if ($existing) {
            $existing->recordOccurrence();
            return $existing->fresh();
        }

        return static::create($attributes);
    }

    /**
     * Static: Get dashboard summary
     */
    public static function getDashboardSummary(): array
    {
        $unresolved = static::unresolved();

        return [
            'total_unresolved' => $unresolved->count(),
            'critical' => $unresolved->clone()->critical()->count(),
            'high_priority' => $unresolved->clone()->highPriority()->count(),
            'unread' => static::unread()->count(),
            'positive' => static::positive()->unresolved()->count(),
            'negative' => static::negative()->unresolved()->count(),
            'stale' => static::stale()->count(),
            'recurring' => static::recurring()->unresolved()->count(),
            'recent_24h' => static::recent(24)->count(),
        ];
    }

    /**
     * Static: Get alerts grouped by type
     */
    public static function getGroupedByType(): Collection
    {
        return static::unresolved()
            ->get()
            ->groupBy('type')
            ->map(fn($alerts) => [
                'count' => $alerts->count(),
                'critical_count' => $alerts->where('severity', self::SEVERITY_CRITICAL)->count(),
                'latest' => $alerts->sortByDesc('created_at')->first(),
            ]);
    }

    /**
     * Static: Get alerts grouped by severity
     */
    public static function getGroupedBySeverity(): Collection
    {
        return static::unresolved()
            ->get()
            ->groupBy('severity')
            ->map(fn($alerts) => [
                'count' => $alerts->count(),
                'types' => $alerts->pluck('type')->unique()->values(),
            ]);
    }

    /**
     * Static: Bulk resolve alerts
     */
    public static function bulkResolve(array $ids, ?string $resolvedBy = null, ?string $notes = null): int
    {
        return static::whereIn('id', $ids)
            ->update([
                'is_resolved' => true,
                'is_new' => false,
                'resolved_at' => now(),
                'resolved_by' => $resolvedBy,
                'resolution_notes' => $notes,
            ]);
    }

    /**
     * Static: Bulk mark as read
     */
    public static function bulkMarkAsRead(array $ids, ?string $userId = null): int
    {
        return static::whereIn('id', $ids)
            ->update([
                'is_new' => false,
                'acknowledged_at' => now(),
                'acknowledged_by' => $userId,
            ]);
    }

    /**
     * Export to array for API
     */
    public function toAlertArray(): array
    {
        return [
            'id' => $this->id,
            'type' => $this->type,
            'type_label' => $this->type_label,
            'type_icon' => $this->type_icon,
            'severity' => $this->severity,
            'severity_priority' => $this->severity_priority,
            'severity_color' => $this->severity_color,
            'severity_icon' => $this->severity_icon,
            'title' => $this->title,
            'message' => $this->message,
            'metadata' => $this->metadata,
            'is_new' => $this->is_new,
            'is_resolved' => $this->is_resolved,
            'is_positive' => $this->isPositive(),
            'is_recurring' => $this->isRecurring(),
            'is_stale' => $this->isStale(),
            'occurrence_count' => $this->occurrence_count,
            'resolution_duration' => $this->resolution_duration,
            'created_at' => $this->created_at->toISOString(),
            'last_occurred_at' => $this->last_occurred_at?->toISOString(),
            'resolved_at' => $this->resolved_at?->toISOString(),
            'acknowledged_at' => $this->acknowledged_at?->toISOString(),
        ];
    }
}