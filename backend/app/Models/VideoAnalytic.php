<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;

/**
 * VideoAnalytic Model - Enterprise-grade video analytics tracking
 * 
 * Comprehensive video engagement tracking with event logging, completion metrics,
 * quality analytics, and behavioral insights for video content optimization.
 *
 * @property int $id
 * @property int $video_id
 * @property int|null $user_id
 * @property string $session_id Unique viewing session identifier
 * @property string $event_type Event type (play, pause, complete, etc.)
 * @property int|null $timestamp_seconds Playback position in seconds
 * @property int $watch_time Total watch time in seconds
 * @property float $completion_rate Video completion percentage (0-100)
 * @property int $interactions User interaction count
 * @property string|null $quality Playback quality level
 * @property int $buffer_events Number of buffering events
 * @property array|null $event_data Additional event metadata
 * @property string|null $ip_address Client IP address
 * @property string|null $user_agent Client user agent
 * @property string|null $referrer HTTP referrer
 * @property string|null $device_type Device type (desktop, mobile, tablet)
 * @property string|null $browser Browser name
 * @property string|null $os Operating system
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read Video $video
 * @property-read User|null $user
 * @property-read string $event_label
 * @property-read bool $is_engaged
 * @property-read bool $is_completed
 * 
 * @method static Builder byEventType(string $eventType)
 * @method static Builder bySession(string $sessionId)
 * @method static Builder byVideo(int $videoId)
 * @method static Builder byUser(int $userId)
 * @method static Builder engaged(int $minWatchTime = 30)
 * @method static Builder completed()
 * @method static Builder recent(int $days = 7)
 * @method static Builder highQuality()
 * @method static Builder withBuffering()
 */
class VideoAnalytic extends Model
{
    use HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'video_analytics';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'video_id',
        'user_id',
        'session_id',
        'event_type',
        'timestamp_seconds',
        'watch_time',
        'completion_rate',
        'interactions',
        'quality',
        'buffer_events',
        'event_data',
        'ip_address',
        'user_agent',
        'referrer',
        'device_type',
        'browser',
        'os',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'video_id' => 'integer',
        'user_id' => 'integer',
        'event_data' => 'array',
        'timestamp_seconds' => 'integer',
        'watch_time' => 'integer',
        'completion_rate' => 'decimal:2',
        'interactions' => 'integer',
        'buffer_events' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'watch_time' => 0,
        'completion_rate' => 0,
        'interactions' => 0,
        'buffer_events' => 0,
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
        'event_label',
        'is_engaged',
        'is_completed',
    ];

    // =========================================================================
    // CONSTANTS - EVENT TYPES
    // =========================================================================

    public const EVENT_LOAD = 'load';
    public const EVENT_PLAY = 'play';
    public const EVENT_PAUSE = 'pause';
    public const EVENT_RESUME = 'resume';
    public const EVENT_SEEK = 'seek';
    public const EVENT_COMPLETE = 'complete';
    public const EVENT_BUFFER_START = 'buffer_start';
    public const EVENT_BUFFER_END = 'buffer_end';
    public const EVENT_QUALITY_CHANGE = 'quality_change';
    public const EVENT_VOLUME_CHANGE = 'volume_change';
    public const EVENT_FULLSCREEN = 'fullscreen';
    public const EVENT_EXIT_FULLSCREEN = 'exit_fullscreen';
    public const EVENT_MUTE = 'mute';
    public const EVENT_UNMUTE = 'unmute';
    public const EVENT_ERROR = 'error';
    public const EVENT_MILESTONE = 'milestone';

    /**
     * Available event types.
     *
     * @var array<int, string>
     */
    public const EVENT_TYPES = [
        self::EVENT_LOAD,
        self::EVENT_PLAY,
        self::EVENT_PAUSE,
        self::EVENT_RESUME,
        self::EVENT_SEEK,
        self::EVENT_COMPLETE,
        self::EVENT_BUFFER_START,
        self::EVENT_BUFFER_END,
        self::EVENT_QUALITY_CHANGE,
        self::EVENT_VOLUME_CHANGE,
        self::EVENT_FULLSCREEN,
        self::EVENT_EXIT_FULLSCREEN,
        self::EVENT_MUTE,
        self::EVENT_UNMUTE,
        self::EVENT_ERROR,
        self::EVENT_MILESTONE,
    ];

    /**
     * Event labels for display.
     *
     * @var array<string, string>
     */
    public const EVENT_LABELS = [
        self::EVENT_LOAD => 'Video Loaded',
        self::EVENT_PLAY => 'Started Playing',
        self::EVENT_PAUSE => 'Paused',
        self::EVENT_RESUME => 'Resumed',
        self::EVENT_SEEK => 'Seeked',
        self::EVENT_COMPLETE => 'Completed',
        self::EVENT_BUFFER_START => 'Buffering Started',
        self::EVENT_BUFFER_END => 'Buffering Ended',
        self::EVENT_QUALITY_CHANGE => 'Quality Changed',
        self::EVENT_VOLUME_CHANGE => 'Volume Changed',
        self::EVENT_FULLSCREEN => 'Fullscreen Enabled',
        self::EVENT_EXIT_FULLSCREEN => 'Fullscreen Disabled',
        self::EVENT_MUTE => 'Muted',
        self::EVENT_UNMUTE => 'Unmuted',
        self::EVENT_ERROR => 'Error Occurred',
        self::EVENT_MILESTONE => 'Milestone Reached',
    ];

    /**
     * Quality levels.
     *
     * @var array<int, string>
     */
    public const QUALITY_LEVELS = [
        '144p',
        '240p',
        '360p',
        '480p',
        '720p',
        '1080p',
        '1440p',
        '2160p',
        'auto',
    ];

    /**
     * Device types.
     *
     * @var array<int, string>
     */
    public const DEVICE_TYPES = [
        'desktop',
        'mobile',
        'tablet',
        'tv',
        'unknown',
    ];

    /**
     * Engagement thresholds.
     */
    public const ENGAGEMENT_THRESHOLD_SECONDS = 30;
    public const COMPLETION_THRESHOLD_PERCENT = 90.0;
    public const HIGH_INTERACTION_THRESHOLD = 5;

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Parse user agent on creation
        static::creating(function (VideoAnalytic $analytic) {
            if (request() && empty($analytic->user_agent)) {
                $analytic->captureRequestData();
            }

            if ($analytic->user_agent && empty($analytic->device_type)) {
                $analytic->parseUserAgent();
            }

            // Validate event type
            if (!in_array($analytic->event_type, self::EVENT_TYPES, true)) {
                throw new \InvalidArgumentException(
                    "Invalid event type: {$analytic->event_type}"
                );
            }
        });

        // Clear cache on changes
        static::saved(function (VideoAnalytic $analytic) {
            Cache::forget("video_analytics_{$analytic->video_id}");
            Cache::forget("video_session_analytics_{$analytic->session_id}");
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the video that owns the analytic.
     */
    public function video(): BelongsTo
    {
        return $this->belongsTo(Video::class)
            ->withDefault([
                'title' => 'Unknown Video',
            ]);
    }

    /**
     * Get the user that owns the analytic.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class)
            ->withDefault([
                'name' => 'Anonymous',
            ]);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to filter by event type.
     */
    public function scopeByEventType(Builder $query, string $eventType): Builder
    {
        return $query->where('event_type', $eventType);
    }

    /**
     * Scope a query to filter by session.
     */
    public function scopeBySession(Builder $query, string $sessionId): Builder
    {
        return $query->where('session_id', $sessionId);
    }

    /**
     * Scope a query to filter by video.
     */
    public function scopeByVideo(Builder $query, int $videoId): Builder
    {
        return $query->where('video_id', $videoId);
    }

    /**
     * Scope a query to filter by user.
     */
    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope to engaged viewers (minimum watch time).
     */
    public function scopeEngaged(Builder $query, int $minWatchTime = self::ENGAGEMENT_THRESHOLD_SECONDS): Builder
    {
        return $query->where('watch_time', '>=', $minWatchTime);
    }

    /**
     * Scope to completed views.
     */
    public function scopeCompleted(Builder $query, float $threshold = self::COMPLETION_THRESHOLD_PERCENT): Builder
    {
        return $query->where('completion_rate', '>=', $threshold);
    }

    /**
     * Scope to recent analytics.
     */
    public function scopeRecent(Builder $query, int $days = 7): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope to specific date range.
     */
    public function scopeBetweenDates(Builder $query, Carbon $startDate, Carbon $endDate): Builder
    {
        return $query->whereBetween('created_at', [$startDate, $endDate]);
    }

    /**
     * Scope to high quality playback.
     */
    public function scopeHighQuality(Builder $query): Builder
    {
        return $query->whereIn('quality', ['1080p', '1440p', '2160p']);
    }

    /**
     * Scope to analytics with buffering issues.
     */
    public function scopeWithBuffering(Builder $query, int $minBufferEvents = 1): Builder
    {
        return $query->where('buffer_events', '>=', $minBufferEvents);
    }

    /**
     * Scope to specific device type.
     */
    public function scopeByDeviceType(Builder $query, string $deviceType): Builder
    {
        return $query->where('device_type', $deviceType);
    }

    /**
     * Scope to mobile devices.
     */
    public function scopeMobile(Builder $query): Builder
    {
        return $query->whereIn('device_type', ['mobile', 'tablet']);
    }

    /**
     * Scope to desktop devices.
     */
    public function scopeDesktop(Builder $query): Builder
    {
        return $query->where('device_type', 'desktop');
    }

    /**
     * Scope to highly interactive sessions.
     */
    public function scopeHighlyInteractive(Builder $query, int $minInteractions = self::HIGH_INTERACTION_THRESHOLD): Builder
    {
        return $query->where('interactions', '>=', $minInteractions);
    }

    /**
     * Scope to error events.
     */
    public function scopeErrors(Builder $query): Builder
    {
        return $query->where('event_type', self::EVENT_ERROR);
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Get human-readable event label.
     */
    public function getEventLabelAttribute(): string
    {
        return self::EVENT_LABELS[$this->event_type] ?? ucfirst(str_replace('_', ' ', $this->event_type));
    }

    /**
     * Check if session is engaged.
     */
    public function getIsEngagedAttribute(): bool
    {
        return $this->isEngaged();
    }

    /**
     * Check if video is completed.
     */
    public function getIsCompletedAttribute(): bool
    {
        return $this->isCompleted();
    }

    /**
     * Get formatted watch time.
     */
    public function getFormattedWatchTimeAttribute(): string
    {
        return $this->formatDuration($this->watch_time);
    }

    /**
     * Get completion percentage formatted.
     */
    public function getCompletionPercentageAttribute(): string
    {
        return number_format($this->completion_rate, 1) . '%';
    }

    // =========================================================================
    // EVENT TRACKING METHODS
    // =========================================================================

    /**
     * Check if event type is valid.
     */
    public static function isValidEventType(string $eventType): bool
    {
        return in_array($eventType, self::EVENT_TYPES, true);
    }

    /**
     * Check if session is engaged.
     */
    public function isEngaged(int $threshold = self::ENGAGEMENT_THRESHOLD_SECONDS): bool
    {
        return $this->watch_time >= $threshold;
    }

    /**
     * Check if video is completed.
     */
    public function isCompleted(float $threshold = self::COMPLETION_THRESHOLD_PERCENT): bool
    {
        return $this->completion_rate >= $threshold;
    }

    /**
     * Check if session had buffering issues.
     */
    public function hadBufferingIssues(int $threshold = 3): bool
    {
        return $this->buffer_events >= $threshold;
    }

    /**
     * Check if session is highly interactive.
     */
    public function isHighlyInteractive(int $threshold = self::HIGH_INTERACTION_THRESHOLD): bool
    {
        return $this->interactions >= $threshold;
    }

    /**
     * Increment interaction count.
     */
    public function incrementInteractions(int $count = 1): void
    {
        $this->increment('interactions', $count);
    }

    /**
     * Increment buffer events.
     */
    public function incrementBufferEvents(int $count = 1): void
    {
        $this->increment('buffer_events', $count);
    }

    /**
     * Update watch time.
     */
    public function updateWatchTime(int $seconds): bool
    {
        $this->watch_time = $seconds;
        return $this->save();
    }

    /**
     * Update completion rate.
     */
    public function updateCompletionRate(float $rate): bool
    {
        $this->completion_rate = min(100, max(0, $rate));
        return $this->save();
    }

    // =========================================================================
    // METADATA MANAGEMENT
    // =========================================================================

    /**
     * Capture request data automatically.
     */
    protected function captureRequestData(): void
    {
        if (!request()) {
            return;
        }

        $this->ip_address = $this->ip_address ?? request()->ip();
        $this->user_agent = $this->user_agent ?? request()->userAgent();
        $this->referrer = $this->referrer ?? request()->header('referer');
    }

    /**
     * Parse user agent to extract device info.
     */
    protected function parseUserAgent(): void
    {
        if (empty($this->user_agent)) {
            return;
        }

        $userAgent = strtolower($this->user_agent);

        // Detect device type
        if (preg_match('/(mobile|android|iphone|ipad|ipod)/i', $userAgent)) {
            if (preg_match('/(ipad|tablet)/i', $userAgent)) {
                $this->device_type = 'tablet';
            } else {
                $this->device_type = 'mobile';
            }
        } elseif (preg_match('/tv|smarttv/i', $userAgent)) {
            $this->device_type = 'tv';
        } else {
            $this->device_type = 'desktop';
        }

        // Detect browser
        if (preg_match('/chrome/i', $userAgent)) {
            $this->browser = 'Chrome';
        } elseif (preg_match('/firefox/i', $userAgent)) {
            $this->browser = 'Firefox';
        } elseif (preg_match('/safari/i', $userAgent)) {
            $this->browser = 'Safari';
        } elseif (preg_match('/edge/i', $userAgent)) {
            $this->browser = 'Edge';
        } else {
            $this->browser = 'Other';
        }

        // Detect OS
        if (preg_match('/windows/i', $userAgent)) {
            $this->os = 'Windows';
        } elseif (preg_match('/mac/i', $userAgent)) {
            $this->os = 'macOS';
        } elseif (preg_match('/linux/i', $userAgent)) {
            $this->os = 'Linux';
        } elseif (preg_match('/android/i', $userAgent)) {
            $this->os = 'Android';
        } elseif (preg_match('/ios|iphone|ipad/i', $userAgent)) {
            $this->os = 'iOS';
        } else {
            $this->os = 'Other';
        }
    }

    /**
     * Add event metadata.
     */
    public function addEventData(string $key, mixed $value): bool
    {
        $data = $this->event_data ?? [];
        $data[$key] = $value;
        $this->event_data = $data;
        return $this->save();
    }

    /**
     * Get event metadata value.
     */
    public function getEventData(string $key, mixed $default = null): mixed
    {
        return $this->event_data[$key] ?? $default;
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /**
     * Format duration in seconds to human-readable string.
     */
    protected function formatDuration(int $seconds): string
    {
        $hours = floor($seconds / 3600);
        $minutes = floor(($seconds % 3600) / 60);
        $secs = $seconds % 60;

        if ($hours > 0) {
            return sprintf('%d:%02d:%02d', $hours, $minutes, $secs);
        }
        return sprintf('%d:%02d', $minutes, $secs);
    }

    /**
     * Get analytic summary.
     */
    public function toSummary(): array
    {
        return [
            'id' => $this->id,
            'video_id' => $this->video_id,
            'session_id' => $this->session_id,
            'event_type' => $this->event_type,
            'event_label' => $this->event_label,
            'watch_time' => $this->watch_time,
            'formatted_watch_time' => $this->formatted_watch_time,
            'completion_rate' => $this->completion_rate,
            'completion_percentage' => $this->completion_percentage,
            'interactions' => $this->interactions,
            'quality' => $this->quality,
            'buffer_events' => $this->buffer_events,
            'device_type' => $this->device_type,
            'browser' => $this->browser,
            'is_engaged' => $this->is_engaged,
            'is_completed' => $this->is_completed,
            'created_at' => $this->created_at->toIso8601String(),
        ];
    }

    // =========================================================================
    // STATIC FACTORY & TRACKING METHODS
    // =========================================================================

    /**
     * Track a video event.
     */
    public static function trackEvent(
        int $videoId,
        string $sessionId,
        string $eventType,
        array $data = []
    ): self {
        return self::create(array_merge([
            'video_id' => $videoId,
            'session_id' => $sessionId,
            'event_type' => $eventType,
            'user_id' => auth()->id(),
        ], $data));
    }

    /**
     * Update or create session analytics.
     */
    public static function updateSession(
        string $sessionId,
        int $videoId,
        array $data
    ): self {
        return self::updateOrCreate(
            [
                'session_id' => $sessionId,
                'video_id' => $videoId,
                'event_type' => self::EVENT_PLAY,
            ],
            array_merge([
                'user_id' => auth()->id(),
            ], $data)
        );
    }

    // =========================================================================
    // ANALYTICS & AGGREGATION METHODS
    // =========================================================================

    /**
     * Get video analytics summary.
     */
    public static function getVideoAnalytics(int $videoId, int $days = 30): array
    {
        return Cache::remember("video_analytics_{$videoId}_{$days}", 300, function () use ($videoId, $days) {
            $analytics = self::where('video_id', $videoId)
                ->where('created_at', '>=', now()->subDays($days))
                ->get();

            $uniqueSessions = $analytics->unique('session_id')->count();
            $completedViews = $analytics->where('completion_rate', '>=', self::COMPLETION_THRESHOLD_PERCENT)->count();

            return [
                'total_views' => $uniqueSessions,
                'total_watch_time' => $analytics->sum('watch_time'),
                'avg_watch_time' => $analytics->avg('watch_time'),
                'avg_completion_rate' => $analytics->avg('completion_rate'),
                'completed_views' => $completedViews,
                'completion_rate' => $uniqueSessions > 0 ? round(($completedViews / $uniqueSessions) * 100, 2) : 0,
                'engaged_viewers' => $analytics->where('watch_time', '>=', self::ENGAGEMENT_THRESHOLD_SECONDS)->unique('session_id')->count(),
                'total_interactions' => $analytics->sum('interactions'),
                'avg_interactions' => $analytics->avg('interactions'),
                'buffer_events' => $analytics->sum('buffer_events'),
                'avg_buffer_events' => $analytics->avg('buffer_events'),
                'quality_distribution' => $analytics->whereNotNull('quality')->groupBy('quality')->map->count(),
                'device_distribution' => $analytics->whereNotNull('device_type')->groupBy('device_type')->map->count(),
            ];
        });
    }

    /**
     * Get session analytics.
     */
    public static function getSessionAnalytics(string $sessionId): array
    {
        return Cache::remember("video_session_analytics_{$sessionId}", 300, function () use ($sessionId) {
            $events = self::where('session_id', $sessionId)->orderBy('created_at')->get();

            if ($events->isEmpty()) {
                return [];
            }

            $latest = $events->last();

            return [
                'session_id' => $sessionId,
                'video_id' => $latest->video_id,
                'user_id' => $latest->user_id,
                'total_events' => $events->count(),
                'watch_time' => $latest->watch_time,
                'completion_rate' => $latest->completion_rate,
                'interactions' => $latest->interactions,
                'buffer_events' => $latest->buffer_events,
                'quality' => $latest->quality,
                'device_type' => $latest->device_type,
                'browser' => $latest->browser,
                'os' => $latest->os,
                'is_engaged' => $latest->is_engaged,
                'is_completed' => $latest->is_completed,
                'events' => $events->map(fn($e) => [
                    'type' => $e->event_type,
                    'timestamp' => $e->timestamp_seconds,
                    'created_at' => $e->created_at->toIso8601String(),
                ]),
            ];
        });
    }

    /**
     * Get engagement metrics for a video.
     */
    public static function getEngagementMetrics(int $videoId, int $days = 30): array
    {
        $analytics = self::where('video_id', $videoId)
            ->where('created_at', '>=', now()->subDays($days))
            ->get();

        $uniqueSessions = $analytics->unique('session_id')->count();

        if ($uniqueSessions === 0) {
            return [
                'engagement_rate' => 0,
                'avg_watch_time' => 0,
                'drop_off_rate' => 0,
                'interaction_rate' => 0,
            ];
        }

        $engagedSessions = $analytics->filter->isEngaged()->unique('session_id')->count();
        $interactiveSessions = $analytics->filter->isHighlyInteractive()->unique('session_id')->count();
        $completedSessions = $analytics->filter->isCompleted()->unique('session_id')->count();

        return [
            'engagement_rate' => round(($engagedSessions / $uniqueSessions) * 100, 2),
            'avg_watch_time' => round($analytics->avg('watch_time'), 2),
            'completion_rate' => round(($completedSessions / $uniqueSessions) * 100, 2),
            'drop_off_rate' => round((1 - ($completedSessions / $uniqueSessions)) * 100, 2),
            'interaction_rate' => round(($interactiveSessions / $uniqueSessions) * 100, 2),
        ];
    }

    /**
     * Get quality analytics for a video.
     */
    public static function getQualityAnalytics(int $videoId, int $days = 30): array
    {
        $analytics = self::where('video_id', $videoId)
            ->where('created_at', '>=', now()->subDays($days))
            ->whereNotNull('quality')
            ->get();

        return [
            'quality_distribution' => $analytics->groupBy('quality')->map->count()->toArray(),
            'avg_buffer_events' => round($analytics->avg('buffer_events'), 2),
            'sessions_with_buffering' => $analytics->where('buffer_events', '>', 0)->unique('session_id')->count(),
            'high_quality_views' => $analytics->whereIn('quality', ['1080p', '1440p', '2160p'])->unique('session_id')->count(),
        ];
    }

    /**
     * Get device analytics for a video.
     */
    public static function getDeviceAnalytics(int $videoId, int $days = 30): array
    {
        $analytics = self::where('video_id', $videoId)
            ->where('created_at', '>=', now()->subDays($days))
            ->get();

        return [
            'device_distribution' => $analytics->whereNotNull('device_type')->groupBy('device_type')->map->count()->toArray(),
            'browser_distribution' => $analytics->whereNotNull('browser')->groupBy('browser')->map->count()->toArray(),
            'os_distribution' => $analytics->whereNotNull('os')->groupBy('os')->map->count()->toArray(),
            'mobile_views' => $analytics->whereIn('device_type', ['mobile', 'tablet'])->unique('session_id')->count(),
            'desktop_views' => $analytics->where('device_type', 'desktop')->unique('session_id')->count(),
        ];
    }

    /**
     * Get viewing trend data.
     */
    public static function getViewingTrend(int $videoId, int $days = 30): array
    {
        $trend = self::where('video_id', $videoId)
            ->where('created_at', '>=', now()->subDays($days))
            ->select(DB::raw('DATE(created_at) as date'), DB::raw('COUNT(DISTINCT session_id) as views'))
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return $trend->pluck('views', 'date')->toArray();
    }

    /**
     * Export analytics data.
     */
    public static function exportAnalytics(int $videoId, ?Carbon $startDate = null, ?Carbon $endDate = null): array
    {
        $query = self::where('video_id', $videoId);

        if ($startDate) {
            $query->where('created_at', '>=', $startDate);
        }

        if ($endDate) {
            $query->where('created_at', '<=', $endDate);
        }

        return $query->get()->map->toSummary()->toArray();
    }

    /**
     * Clean up old analytics data.
     */
    public static function cleanup(int $daysToKeep = 90): int
    {
        return self::where('created_at', '<', now()->subDays($daysToKeep))->delete();
    }
}