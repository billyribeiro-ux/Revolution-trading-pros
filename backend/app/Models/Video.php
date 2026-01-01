<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;

/**
 * Video Model - Enterprise-grade video content management
 * 
 * Comprehensive video management with platform integration, analytics tracking,
 * engagement metrics, and content optimization capabilities.
 *
 * @property int $id
 * @property string $title Video title
 * @property string|null $description Video description
 * @property string|null $slug URL-friendly slug
 * @property string $url Video URL
 * @property string $platform Platform name (youtube, vimeo, etc.)
 * @property string $video_id Platform-specific video identifier
 * @property string|null $thumbnail_url Thumbnail image URL
 * @property int $duration Video duration in seconds
 * @property string|null $quality Video quality level
 * @property bool $is_active Active status
 * @property bool $is_featured Featured status
 * @property bool $allow_comments Allow comments flag
 * @property int|null $user_id Owner user ID
 * @property int|null $category_id Category ID
 * @property array|null $metadata Additional video metadata
 * @property array|null $settings Video-specific settings
 * @property int $view_count Cached view count
 * @property int $like_count Cached like count
 * @property Carbon|null $published_at Publication timestamp
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read User $user
 * @property-read Collection<int, VideoAnalytic> $analytics
 * @property-read int $analytics_count
 * @property-read int $total_views
 * @property-read float $average_completion_rate
 * @property-read int $total_watch_time
 * @property-read string $formatted_duration
 * @property-read float $engagement_rate
 * @property-read bool $is_published
 * 
 * @method static Builder active()
 * @method static Builder inactive()
 * @method static Builder featured()
 * @method static Builder published()
 * @method static Builder byPlatform(string $platform)
 * @method static Builder byUser(int $userId)
 * @method static Builder popular(int $minViews = 100)
 * @method static Builder recent(int $days = 7)
 * @method static Builder longForm(int $minDuration = 600)
 * @method static Builder shortForm(int $maxDuration = 60)
 */
class Video extends Model
{
    use SoftDeletes, HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'videos';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'title',
        'description',
        'slug',
        'url',
        'platform',
        'video_id',
        'thumbnail_url',
        'duration',
        'quality',
        'is_active',
        'is_featured',
        'allow_comments',
        'user_id',
        'category_id',
        'metadata',
        'settings',
        'view_count',
        'like_count',
        'published_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'user_id' => 'integer',
        'category_id' => 'integer',
        'metadata' => 'array',
        'settings' => 'array',
        'is_active' => 'boolean',
        'is_featured' => 'boolean',
        'allow_comments' => 'boolean',
        'duration' => 'integer',
        'view_count' => 'integer',
        'like_count' => 'integer',
        'published_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'is_active' => true,
        'is_featured' => false,
        'allow_comments' => true,
        'view_count' => 0,
        'like_count' => 0,
        'duration' => 0,
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'total_views',
        'average_completion_rate',
        'total_watch_time',
        'formatted_duration',
        'engagement_rate',
        'is_published',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const PLATFORM_YOUTUBE = 'youtube';
    public const PLATFORM_VIMEO = 'vimeo';
    public const PLATFORM_DAILYMOTION = 'dailymotion';
    public const PLATFORM_WISTIA = 'wistia';
    public const PLATFORM_CUSTOM = 'custom';
    public const PLATFORM_S3 = 's3';
    public const PLATFORM_BUNNY = 'bunny';

    /**
     * Supported video platforms.
     *
     * @var array<int, string>
     */
    public const PLATFORMS = [
        self::PLATFORM_YOUTUBE,
        self::PLATFORM_VIMEO,
        self::PLATFORM_DAILYMOTION,
        self::PLATFORM_WISTIA,
        self::PLATFORM_CUSTOM,
        self::PLATFORM_S3,
        self::PLATFORM_BUNNY,
    ];

    /**
     * Platform labels.
     *
     * @var array<string, string>
     */
    public const PLATFORM_LABELS = [
        self::PLATFORM_YOUTUBE => 'YouTube',
        self::PLATFORM_VIMEO => 'Vimeo',
        self::PLATFORM_DAILYMOTION => 'Dailymotion',
        self::PLATFORM_WISTIA => 'Wistia',
        self::PLATFORM_CUSTOM => 'Custom Player',
        self::PLATFORM_S3 => 'Amazon S3',
        self::PLATFORM_BUNNY => 'Bunny CDN',
    ];

    /**
     * Video quality levels.
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
        '4k',
        'auto',
    ];

    /**
     * Duration thresholds (seconds).
     */
    public const SHORT_FORM_MAX = 60; // 1 minute
    public const MEDIUM_FORM_MAX = 600; // 10 minutes
    public const LONG_FORM_MIN = 600; // 10 minutes

    /**
     * Popularity thresholds.
     */
    public const POPULAR_MIN_VIEWS = 1000;
    public const VIRAL_MIN_VIEWS = 10000;
    public const TRENDING_MIN_VIEWS = 5000;

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Auto-generate slug on creation
        static::creating(function (Video $video) {
            if (empty($video->slug) && !empty($video->title)) {
                $video->slug = $video->generateUniqueSlug($video->title);
            }

            // Validate platform
            if (!in_array($video->platform, self::PLATFORMS, true)) {
                throw new \InvalidArgumentException(
                    "Invalid platform: {$video->platform}"
                );
            }

            // Set published_at if not set and is_active
            if ($video->is_active && empty($video->published_at)) {
                $video->published_at = now();
            }
        });

        // Update slug if title changes
        static::updating(function (Video $video) {
            if ($video->isDirty('title') && $video->getSetting('auto_update_slug', true)) {
                $video->slug = $video->generateUniqueSlug($video->title);
            }

            // Set/unset published_at based on is_active changes
            if ($video->isDirty('is_active')) {
                if ($video->is_active && empty($video->published_at)) {
                    $video->published_at = now();
                } elseif (!$video->is_active) {
                    $video->published_at = null;
                }
            }

            // Log status changes
            if ($video->isDirty('is_active')) {
                Log::info('Video status changed', [
                    'id' => $video->id,
                    'title' => $video->title,
                    'old_status' => $video->getOriginal('is_active') ? 'active' : 'inactive',
                    'new_status' => $video->is_active ? 'active' : 'inactive',
                ]);
            }
        });

        // Cascade delete analytics
        static::deleting(function (Video $video) {
            if ($video->isForceDeleting()) {
                $video->analytics()->forceDelete();
            }
            
            Log::info('Video deleted', [
                'id' => $video->id,
                'title' => $video->title,
                'platform' => $video->platform,
                'force' => $video->isForceDeleting(),
            ]);
        });

        // Clear cache on changes
        static::saved(function (Video $video) {
            Cache::forget("video_stats_{$video->id}");
            Cache::forget("video_analytics_{$video->id}");
        });

        static::deleted(function (Video $video) {
            Cache::forget("video_stats_{$video->id}");
            Cache::forget("video_analytics_{$video->id}");
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the user that owns the video.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class)
            ->withDefault([
                'name' => 'Unknown User',
            ]);
    }

    /**
     * Get the category for the video.
     */
    public function category(): BelongsTo
    {
        return $this->belongsTo(VideoCategory::class, 'category_id')
            ->withDefault([
                'name' => 'Uncategorized',
            ]);
    }

    /**
     * Get the analytics for the video.
     */
    public function analytics(): HasMany
    {
        return $this->hasMany(VideoAnalytic::class);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include active videos.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    /**
     * Scope a query to only include inactive videos.
     */
    public function scopeInactive(Builder $query): Builder
    {
        return $query->where('is_active', false);
    }

    /**
     * Scope a query to only include featured videos.
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true);
    }

    /**
     * Scope a query to only include published videos.
     */
    public function scopePublished(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->whereNotNull('published_at')
            ->where('published_at', '<=', now());
    }

    /**
     * Scope a query to filter by platform.
     */
    public function scopeByPlatform(Builder $query, string $platform): Builder
    {
        return $query->where('platform', $platform);
    }

    /**
     * Scope a query to filter by user.
     */
    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope a query to filter by category.
     */
    public function scopeByCategory(Builder $query, int $categoryId): Builder
    {
        return $query->where('category_id', $categoryId);
    }

    /**
     * Scope a query to popular videos.
     */
    public function scopePopular(Builder $query, int $minViews = self::POPULAR_MIN_VIEWS): Builder
    {
        return $query->where('view_count', '>=', $minViews)
            ->orderBy('view_count', 'desc');
    }

    /**
     * Scope a query to viral videos.
     */
    public function scopeViral(Builder $query, int $minViews = self::VIRAL_MIN_VIEWS): Builder
    {
        return $query->where('view_count', '>=', $minViews)
            ->orderBy('view_count', 'desc');
    }

    /**
     * Scope a query to trending videos.
     */
    public function scopeTrending(Builder $query, int $days = 7, int $minViews = self::TRENDING_MIN_VIEWS): Builder
    {
        return $query->where('published_at', '>=', now()->subDays($days))
            ->where('view_count', '>=', $minViews)
            ->orderBy('view_count', 'desc');
    }

    /**
     * Scope a query to recent videos.
     */
    public function scopeRecent(Builder $query, int $days = 7): Builder
    {
        return $query->where('published_at', '>=', now()->subDays($days))
            ->orderBy('published_at', 'desc');
    }

    /**
     * Scope a query to long-form videos.
     */
    public function scopeLongForm(Builder $query, int $minDuration = self::LONG_FORM_MIN): Builder
    {
        return $query->where('duration', '>=', $minDuration);
    }

    /**
     * Scope a query to short-form videos.
     */
    public function scopeShortForm(Builder $query, int $maxDuration = self::SHORT_FORM_MAX): Builder
    {
        return $query->where('duration', '<=', $maxDuration);
    }

    /**
     * Scope a query to medium-form videos.
     */
    public function scopeMediumForm(Builder $query): Builder
    {
        return $query->where('duration', '>', self::SHORT_FORM_MAX)
            ->where('duration', '<=', self::MEDIUM_FORM_MAX);
    }

    /**
     * Scope a query to search videos.
     */
    public function scopeSearch(Builder $query, string $searchTerm): Builder
    {
        return $query->where(function (Builder $q) use ($searchTerm) {
            $q->where('title', 'LIKE', "%{$searchTerm}%")
                ->orWhere('description', 'LIKE', "%{$searchTerm}%")
                ->orWhere('slug', 'LIKE', "%{$searchTerm}%");
        });
    }

    /**
     * Scope to order by most viewed.
     */
    public function scopeMostViewed(Builder $query): Builder
    {
        return $query->orderBy('view_count', 'desc');
    }

    /**
     * Scope to order by most liked.
     */
    public function scopeMostLiked(Builder $query): Builder
    {
        return $query->orderBy('like_count', 'desc');
    }

    /**
     * Scope to order by newest.
     */
    public function scopeNewest(Builder $query): Builder
    {
        return $query->orderBy('published_at', 'desc');
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Get total views from analytics.
     */
    public function getTotalViewsAttribute(): int
    {
        return Cache::remember("video_{$this->id}_total_views", 300, function () {
            return $this->analytics()
                ->distinct('session_id')
                ->count('session_id');
        });
    }

    /**
     * Get average completion rate.
     */
    public function getAverageCompletionRateAttribute(): float
    {
        return Cache::remember("video_{$this->id}_avg_completion", 300, function () {
            return round($this->analytics()->avg('completion_rate') ?? 0, 2);
        });
    }

    /**
     * Get total watch time.
     */
    public function getTotalWatchTimeAttribute(): int
    {
        return Cache::remember("video_{$this->id}_total_watch_time", 300, function () {
            return (int) $this->analytics()->sum('watch_time');
        });
    }

    /**
     * Get formatted duration.
     */
    public function getFormattedDurationAttribute(): string
    {
        return $this->formatDuration($this->duration);
    }

    /**
     * Get engagement rate.
     */
    public function getEngagementRateAttribute(): float
    {
        if ($this->view_count === 0) {
            return 0.0;
        }

        $engagedViews = $this->analytics()
            ->where('watch_time', '>=', VideoAnalytic::ENGAGEMENT_THRESHOLD_SECONDS)
            ->distinct('session_id')
            ->count('session_id');

        return round(($engagedViews / max(1, $this->view_count)) * 100, 2);
    }

    /**
     * Check if video is published.
     */
    public function getIsPublishedAttribute(): bool
    {
        return $this->isPublished();
    }

    /**
     * Get platform label.
     */
    public function getPlatformLabelAttribute(): string
    {
        return self::PLATFORM_LABELS[$this->platform] ?? ucfirst($this->platform);
    }

    /**
     * Get video form type (short, medium, long).
     */
    public function getFormTypeAttribute(): string
    {
        if ($this->duration <= self::SHORT_FORM_MAX) {
            return 'short';
        } elseif ($this->duration <= self::MEDIUM_FORM_MAX) {
            return 'medium';
        }
        return 'long';
    }

    /**
     * Ensure slug is unique and URL-safe.
     */
    public function setSlugAttribute(?string $value): void
    {
        $this->attributes['slug'] = $value ? Str::slug($value) : null;
    }

    // =========================================================================
    // STATUS & VALIDATION METHODS
    // =========================================================================

    /**
     * Check if video is published.
     */
    public function isPublished(): bool
    {
        return $this->is_active 
            && $this->published_at !== null 
            && $this->published_at->isPast();
    }

    /**
     * Check if video is featured.
     */
    public function isFeatured(): bool
    {
        return $this->is_featured;
    }

    /**
     * Check if video is popular.
     */
    public function isPopular(int $threshold = self::POPULAR_MIN_VIEWS): bool
    {
        return $this->view_count >= $threshold;
    }

    /**
     * Check if video is viral.
     */
    public function isViral(int $threshold = self::VIRAL_MIN_VIEWS): bool
    {
        return $this->view_count >= $threshold;
    }

    /**
     * Check if video is short-form.
     */
    public function isShortForm(): bool
    {
        return $this->duration <= self::SHORT_FORM_MAX;
    }

    /**
     * Check if video is long-form.
     */
    public function isLongForm(): bool
    {
        return $this->duration >= self::LONG_FORM_MIN;
    }

    /**
     * Check if platform is valid.
     */
    public static function isValidPlatform(string $platform): bool
    {
        return in_array($platform, self::PLATFORMS, true);
    }

    // =========================================================================
    // ACTIVATION & STATUS MANAGEMENT
    // =========================================================================

    /**
     * Activate the video.
     */
    public function activate(): bool
    {
        $this->is_active = true;
        if (empty($this->published_at)) {
            $this->published_at = now();
        }
        return $this->save();
    }

    /**
     * Deactivate the video.
     */
    public function deactivate(): bool
    {
        $this->is_active = false;
        return $this->save();
    }

    /**
     * Toggle active status.
     */
    public function toggleActive(): bool
    {
        if ($this->is_active) {
            return $this->deactivate();
        }
        return $this->activate();
    }

    /**
     * Feature the video.
     */
    public function feature(): bool
    {
        $this->is_featured = true;
        return $this->save();
    }

    /**
     * Unfeature the video.
     */
    public function unfeature(): bool
    {
        $this->is_featured = false;
        return $this->save();
    }

    /**
     * Toggle featured status.
     */
    public function toggleFeatured(): bool
    {
        $this->is_featured = !$this->is_featured;
        return $this->save();
    }

    /**
     * Publish the video.
     */
    public function publish(?Carbon $publishedAt = null): bool
    {
        $this->is_active = true;
        $this->published_at = $publishedAt ?? now();
        return $this->save();
    }

    /**
     * Unpublish the video.
     */
    public function unpublish(): bool
    {
        $this->is_active = false;
        $this->published_at = null;
        return $this->save();
    }

    /**
     * Schedule publication.
     */
    public function schedulePublication(Carbon $publishAt): bool
    {
        $this->is_active = true;
        $this->published_at = $publishAt;
        return $this->save();
    }

    // =========================================================================
    // VIEW & ENGAGEMENT TRACKING
    // =========================================================================

    /**
     * Increment view count.
     */
    public function incrementViews(int $count = 1): void
    {
        $this->increment('view_count', $count);
        Cache::forget("video_{$this->id}_total_views");
    }

    /**
     * Increment like count.
     */
    public function incrementLikes(int $count = 1): void
    {
        $this->increment('like_count', $count);
    }

    /**
     * Decrement like count.
     */
    public function decrementLikes(int $count = 1): void
    {
        $this->decrement('like_count', max(0, $count));
    }

    /**
     * Update cached view count from analytics.
     */
    public function updateViewCount(): bool
    {
        $actualViews = $this->analytics()
            ->distinct('session_id')
            ->count('session_id');

        $this->view_count = $actualViews;
        return $this->save();
    }

    /**
     * Get engagement metrics.
     */
    public function getEngagementMetrics(int $days = 30): array
    {
        return VideoAnalytic::getEngagementMetrics($this->id, $days);
    }

    /**
     * Get quality analytics.
     */
    public function getQualityAnalytics(int $days = 30): array
    {
        return VideoAnalytic::getQualityAnalytics($this->id, $days);
    }

    /**
     * Get device analytics.
     */
    public function getDeviceAnalytics(int $days = 30): array
    {
        return VideoAnalytic::getDeviceAnalytics($this->id, $days);
    }

    /**
     * Get viewing trend.
     */
    public function getViewingTrend(int $days = 30): array
    {
        return VideoAnalytic::getViewingTrend($this->id, $days);
    }

    // =========================================================================
    // SETTINGS MANAGEMENT
    // =========================================================================

    /**
     * Get setting value by key.
     */
    public function getSetting(string $key, mixed $default = null): mixed
    {
        return $this->settings[$key] ?? $default;
    }

    /**
     * Set setting value.
     */
    public function setSetting(string $key, mixed $value): bool
    {
        $settings = $this->settings ?? [];
        $settings[$key] = $value;
        $this->settings = $settings;
        return $this->save();
    }

    /**
     * Remove setting.
     */
    public function removeSetting(string $key): bool
    {
        $settings = $this->settings ?? [];
        unset($settings[$key]);
        $this->settings = $settings;
        return $this->save();
    }

    // =========================================================================
    // METADATA MANAGEMENT
    // =========================================================================

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
     * Generate unique slug.
     */
    protected function generateUniqueSlug(string $title): string
    {
        $slug = Str::slug($title);
        $originalSlug = $slug;
        $counter = 1;

        while (self::where('slug', $slug)->where('id', '!=', $this->id ?? 0)->exists()) {
            $slug = $originalSlug . '-' . $counter;
            $counter++;
        }

        return $slug;
    }

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
     * Get embed URL for the video.
     */
    public function getEmbedUrl(): string
    {
        return match ($this->platform) {
            self::PLATFORM_YOUTUBE => "https://www.youtube.com/embed/{$this->video_id}",
            self::PLATFORM_VIMEO => "https://player.vimeo.com/video/{$this->video_id}",
            self::PLATFORM_DAILYMOTION => "https://www.dailymotion.com/embed/video/{$this->video_id}",
            self::PLATFORM_WISTIA => "https://fast.wistia.net/embed/iframe/{$this->video_id}",
            default => $this->url,
        };
    }

    /**
     * Get video statistics.
     */
    public function getStats(): array
    {
        return Cache::remember("video_stats_{$this->id}", 300, function () {
            return [
                'views' => $this->total_views,
                'likes' => $this->like_count,
                'watch_time' => $this->total_watch_time,
                'avg_completion' => $this->average_completion_rate,
                'engagement_rate' => $this->engagement_rate,
                'duration' => $this->duration,
                'formatted_duration' => $this->formatted_duration,
                'form_type' => $this->form_type,
            ];
        });
    }

    /**
     * Get video summary for display.
     */
    public function toSummary(): array
    {
        return [
            'id' => $this->id,
            'title' => $this->title,
            'slug' => $this->slug,
            'platform' => $this->platform_label,
            'duration' => $this->formatted_duration,
            'views' => $this->view_count,
            'likes' => $this->like_count,
            'is_active' => $this->is_active,
            'is_featured' => $this->is_featured,
            'is_published' => $this->is_published,
            'published_at' => $this->published_at?->toDateTimeString(),
            'thumbnail_url' => $this->thumbnail_url,
            'embed_url' => $this->getEmbedUrl(),
        ];
    }

    // =========================================================================
    // BULK OPERATIONS
    // =========================================================================

    /**
     * Bulk activate videos.
     *
     * @param array<int> $ids
     */
    public static function bulkActivate(array $ids): int
    {
        return self::whereIn('id', $ids)->update([
            'is_active' => true,
            'published_at' => now(),
        ]);
    }

    /**
     * Bulk deactivate videos.
     *
     * @param array<int> $ids
     */
    public static function bulkDeactivate(array $ids): int
    {
        return self::whereIn('id', $ids)->update(['is_active' => false]);
    }

    /**
     * Bulk feature videos.
     *
     * @param array<int> $ids
     */
    public static function bulkFeature(array $ids): int
    {
        return self::whereIn('id', $ids)->update(['is_featured' => true]);
    }

    /**
     * Bulk unfeature videos.
     *
     * @param array<int> $ids
     */
    public static function bulkUnfeature(array $ids): int
    {
        return self::whereIn('id', $ids)->update(['is_featured' => false]);
    }

    /**
     * Bulk delete videos.
     *
     * @param array<int> $ids
     */
    public static function bulkDelete(array $ids): int
    {
        return self::whereIn('id', $ids)->delete();
    }

    /**
     * Clean up old inactive videos.
     */
    public static function cleanupInactive(int $daysInactive = 180): int
    {
        return self::where('is_active', false)
            ->where('updated_at', '<', now()->subDays($daysInactive))
            ->delete();
    }
}