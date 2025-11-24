<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

/**
 * AnalyticsSession Model - Session-level Analytics Aggregation
 *
 * Tracks user sessions with aggregated metrics for journey analysis.
 *
 * @property int $id
 * @property string $session_id
 * @property int|null $user_id
 * @property string|null $anonymous_id
 * @property Carbon $started_at
 * @property Carbon|null $ended_at
 * @property int $duration_seconds
 * @property bool $is_active
 * @property int $page_views
 * @property int $events_count
 * @property bool $had_conversion
 */
class AnalyticsSession extends Model
{
    protected $table = 'analytics_sessions';

    /**
     * Session timeout in minutes
     */
    public const SESSION_TIMEOUT_MINUTES = 30;

    /**
     * Bounce threshold in seconds
     */
    public const BOUNCE_THRESHOLD_SECONDS = 10;

    protected $fillable = [
        'session_id',
        'user_id',
        'anonymous_id',
        'started_at',
        'ended_at',
        'duration_seconds',
        'is_active',
        'page_views',
        'events_count',
        'interactions_count',
        'total_value',
        'landing_page',
        'exit_page',
        'entry_source',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'channel',
        'referrer',
        'device_type',
        'browser',
        'os',
        'country_code',
        'city',
        'engagement_score',
        'is_bounce',
        'had_conversion',
        'goals_completed',
        'journey_stage',
        'pages_visited',
        'events_summary',
    ];

    protected $casts = [
        'user_id' => 'integer',
        'duration_seconds' => 'integer',
        'is_active' => 'boolean',
        'page_views' => 'integer',
        'events_count' => 'integer',
        'interactions_count' => 'integer',
        'total_value' => 'decimal:4',
        'engagement_score' => 'decimal:2',
        'is_bounce' => 'boolean',
        'had_conversion' => 'boolean',
        'goals_completed' => 'integer',
        'pages_visited' => 'array',
        'events_summary' => 'array',
        'started_at' => 'datetime',
        'ended_at' => 'datetime',
    ];

    protected $attributes = [
        'is_active' => true,
        'page_views' => 0,
        'events_count' => 0,
        'interactions_count' => 0,
        'total_value' => 0,
        'engagement_score' => 0,
        'is_bounce' => false,
        'had_conversion' => false,
        'goals_completed' => 0,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function events(): HasMany
    {
        return $this->hasMany(AnalyticsEvent::class, 'session_id', 'session_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    public function scopeCompleted(Builder $query): Builder
    {
        return $query->where('is_active', false);
    }

    public function scopeWithConversion(Builder $query): Builder
    {
        return $query->where('had_conversion', true);
    }

    public function scopeBounces(Builder $query): Builder
    {
        return $query->where('is_bounce', true);
    }

    public function scopeEngaged(Builder $query): Builder
    {
        return $query->where('is_bounce', false);
    }

    public function scopeByChannel(Builder $query, string $channel): Builder
    {
        return $query->where('channel', $channel);
    }

    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('started_at', [$start, $end]);
    }

    public function scopeLastNDays(Builder $query, int $days): Builder
    {
        return $query->where('started_at', '>=', now()->subDays($days));
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Find or create session for tracking
     */
    public static function findOrCreateSession(
        string $sessionId,
        ?int $userId = null,
        ?string $anonymousId = null,
        array $attributes = []
    ): self {
        $session = static::where('session_id', $sessionId)->first();

        if ($session) {
            // Update user_id if now authenticated
            if ($userId && !$session->user_id) {
                $session->update(['user_id' => $userId]);
            }

            return $session;
        }

        return static::create(array_merge([
            'session_id' => $sessionId,
            'user_id' => $userId,
            'anonymous_id' => $anonymousId,
            'started_at' => now(),
            'is_active' => true,
        ], $attributes));
    }

    /**
     * Get session metrics aggregates
     */
    public static function getMetrics(Carbon $startDate, Carbon $endDate): array
    {
        $baseQuery = static::inDateRange($startDate, $endDate);

        $totalSessions = (clone $baseQuery)->count();
        $bouncedSessions = (clone $baseQuery)->bounces()->count();
        $convertedSessions = (clone $baseQuery)->withConversion()->count();

        return [
            'total_sessions' => $totalSessions,
            'unique_users' => (clone $baseQuery)->whereNotNull('user_id')->distinct('user_id')->count('user_id'),
            'avg_duration' => (clone $baseQuery)->avg('duration_seconds') ?? 0,
            'avg_page_views' => (clone $baseQuery)->avg('page_views') ?? 0,
            'bounce_rate' => $totalSessions > 0 ? round(($bouncedSessions / $totalSessions) * 100, 2) : 0,
            'conversion_rate' => $totalSessions > 0 ? round(($convertedSessions / $totalSessions) * 100, 2) : 0,
            'total_revenue' => (clone $baseQuery)->sum('total_value') ?? 0,
            'avg_engagement_score' => (clone $baseQuery)->avg('engagement_score') ?? 0,
        ];
    }

    /**
     * Get sessions by channel
     */
    public static function getByChannel(Carbon $startDate, Carbon $endDate): Collection
    {
        return static::inDateRange($startDate, $endDate)
            ->select(
                'channel',
                DB::raw('COUNT(*) as sessions'),
                DB::raw('COUNT(DISTINCT user_id) as users'),
                DB::raw('AVG(duration_seconds) as avg_duration'),
                DB::raw('AVG(page_views) as avg_page_views'),
                DB::raw('SUM(CASE WHEN is_bounce = 1 THEN 1 ELSE 0 END) as bounces'),
                DB::raw('SUM(CASE WHEN had_conversion = 1 THEN 1 ELSE 0 END) as conversions'),
                DB::raw('SUM(total_value) as revenue')
            )
            ->groupBy('channel')
            ->orderByDesc('sessions')
            ->get()
            ->map(function ($row) {
                $row->bounce_rate = $row->sessions > 0
                    ? round(($row->bounces / $row->sessions) * 100, 2)
                    : 0;
                $row->conversion_rate = $row->sessions > 0
                    ? round(($row->conversions / $row->sessions) * 100, 2)
                    : 0;
                return $row;
            });
    }

    /**
     * Get time-series session data
     */
    public static function getTimeSeries(
        string $granularity,
        Carbon $startDate,
        Carbon $endDate
    ): Collection {
        $dateFormat = match ($granularity) {
            'hour' => '%Y-%m-%d %H:00:00',
            'day' => '%Y-%m-%d',
            'week' => '%Y-%u',
            'month' => '%Y-%m',
            default => '%Y-%m-%d',
        };

        return static::inDateRange($startDate, $endDate)
            ->select(
                DB::raw("DATE_FORMAT(started_at, '{$dateFormat}') as period"),
                DB::raw('COUNT(*) as sessions'),
                DB::raw('COUNT(DISTINCT user_id) as unique_users'),
                DB::raw('AVG(duration_seconds) as avg_duration'),
                DB::raw('SUM(CASE WHEN is_bounce = 1 THEN 1 ELSE 0 END) as bounces'),
                DB::raw('SUM(CASE WHEN had_conversion = 1 THEN 1 ELSE 0 END) as conversions'),
                DB::raw('SUM(total_value) as revenue')
            )
            ->groupBy('period')
            ->orderBy('period')
            ->get();
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Record a new event in this session
     */
    public function recordEvent(AnalyticsEvent $event): void
    {
        $this->events_count++;

        if ($event->event_category === AnalyticsEvent::CATEGORY_PAGE_VIEW) {
            $this->page_views++;

            // Track pages visited
            $pages = $this->pages_visited ?? [];
            if ($event->page_path && !in_array($event->page_path, $pages)) {
                $pages[] = $event->page_path;
                $this->pages_visited = $pages;
            }

            // Update exit page
            $this->exit_page = $event->page_url;
        }

        if ($event->event_type === AnalyticsEvent::TYPE_CLICK) {
            $this->interactions_count++;
        }

        if ($event->is_conversion) {
            $this->had_conversion = true;
        }

        if ($event->revenue) {
            $this->total_value += $event->revenue;
        }

        // Update last activity
        $this->ended_at = $event->event_timestamp;
        $this->duration_seconds = $this->started_at->diffInSeconds($this->ended_at);

        // Check for bounce (single page view, short duration)
        $this->is_bounce = $this->page_views <= 1 && $this->duration_seconds < self::BOUNCE_THRESHOLD_SECONDS;

        // Update engagement score
        $this->engagement_score = $this->calculateEngagementScore();

        $this->save();
    }

    /**
     * End the session
     */
    public function endSession(): void
    {
        $this->is_active = false;
        $this->ended_at = $this->ended_at ?? now();
        $this->duration_seconds = $this->started_at->diffInSeconds($this->ended_at);

        // Final bounce check
        $this->is_bounce = $this->page_views <= 1 && $this->duration_seconds < self::BOUNCE_THRESHOLD_SECONDS;

        // Finalize engagement score
        $this->engagement_score = $this->calculateEngagementScore();

        // Create events summary
        $this->events_summary = $this->generateEventsSummary();

        $this->save();
    }

    /**
     * Calculate session engagement score
     */
    public function calculateEngagementScore(): float
    {
        $score = 0;

        // Page views (up to 20 points)
        $score += min(20, $this->page_views * 4);

        // Duration (up to 30 points)
        $durationMinutes = $this->duration_seconds / 60;
        $score += min(30, $durationMinutes * 3);

        // Interactions (up to 20 points)
        $score += min(20, $this->interactions_count * 2);

        // Conversion bonus (30 points)
        if ($this->had_conversion) {
            $score += 30;
        }

        // Goals bonus (up to 20 points)
        $score += min(20, $this->goals_completed * 5);

        // Not a bounce bonus
        if (!$this->is_bounce && $this->page_views > 1) {
            $score += 10;
        }

        return round(min(100, $score), 2);
    }

    /**
     * Generate events summary for the session
     */
    protected function generateEventsSummary(): array
    {
        return $this->events()
            ->select('event_name', DB::raw('COUNT(*) as count'))
            ->groupBy('event_name')
            ->pluck('count', 'event_name')
            ->toArray();
    }

    /**
     * Get user journey for this session
     */
    public function getJourney(): Collection
    {
        return $this->events()
            ->select('event_name', 'event_type', 'page_path', 'event_timestamp', 'properties')
            ->orderBy('event_timestamp')
            ->get();
    }

    /**
     * Check if session has timed out
     */
    public function hasTimedOut(): bool
    {
        if (!$this->is_active) {
            return false;
        }

        $lastActivity = $this->ended_at ?? $this->started_at;

        return $lastActivity->diffInMinutes(now()) >= self::SESSION_TIMEOUT_MINUTES;
    }
}
