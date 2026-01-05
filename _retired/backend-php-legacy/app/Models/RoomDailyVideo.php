<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Room Daily Video Model
 *
 * Represents daily video content for each trading room/alert service
 *
 * @property int $id
 * @property int $trading_room_id
 * @property int|null $trader_id
 * @property string $title
 * @property string|null $description
 * @property string $video_url
 * @property string $video_platform
 * @property string|null $video_id
 * @property string|null $thumbnail_url
 * @property int|null $duration
 * @property string $video_date
 * @property bool $is_featured
 * @property bool $is_published
 * @property int $views_count
 * @property array|null $tags
 * @property array|null $metadata
 */
class RoomDailyVideo extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'trading_room_id',
        'trader_id',
        'title',
        'description',
        'video_url',
        'video_platform',
        'video_id',
        'thumbnail_url',
        'duration',
        'video_date',
        'is_featured',
        'is_published',
        'views_count',
        'tags',
        'metadata',
    ];

    protected $casts = [
        'trading_room_id' => 'integer',
        'trader_id' => 'integer',
        'duration' => 'integer',
        'video_date' => 'date',
        'is_featured' => 'boolean',
        'is_published' => 'boolean',
        'views_count' => 'integer',
        'tags' => 'array',
        'metadata' => 'array',
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // RELATIONSHIPS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get the trading room this video belongs to
     */
    public function tradingRoom(): BelongsTo
    {
        return $this->belongsTo(TradingRoom::class);
    }

    /**
     * Get the trader who created this video
     */
    public function trader(): BelongsTo
    {
        return $this->belongsTo(RoomTrader::class, 'trader_id');
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SCOPES
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Only published videos
     */
    public function scopePublished($query)
    {
        return $query->where('is_published', true);
    }

    /**
     * Only featured videos
     */
    public function scopeFeatured($query)
    {
        return $query->where('is_featured', true);
    }

    /**
     * Filter by trading room slug
     */
    public function scopeForRoom($query, string $slug)
    {
        return $query->whereHas('tradingRoom', function ($q) use ($slug) {
            $q->where('slug', $slug);
        });
    }

    /**
     * Filter by trader
     */
    public function scopeByTrader($query, int $traderId)
    {
        return $query->where('trader_id', $traderId);
    }

    /**
     * Order by video date descending
     */
    public function scopeLatest($query)
    {
        return $query->orderByDesc('video_date');
    }

    /**
     * Search by title or description
     */
    public function scopeSearch($query, string $term)
    {
        return $query->where(function ($q) use ($term) {
            $q->where('title', 'like', "%{$term}%")
              ->orWhere('description', 'like', "%{$term}%");
        });
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ACCESSORS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get formatted duration (e.g., "12:34" or "1:23:45")
     */
    public function getFormattedDurationAttribute(): string
    {
        if (!$this->duration) {
            return '';
        }

        $hours = floor($this->duration / 3600);
        $minutes = floor(($this->duration % 3600) / 60);
        $seconds = $this->duration % 60;

        if ($hours > 0) {
            return sprintf('%d:%02d:%02d', $hours, $minutes, $seconds);
        }

        return sprintf('%d:%02d', $minutes, $seconds);
    }

    /**
     * Get formatted video date
     */
    public function getFormattedDateAttribute(): string
    {
        return $this->video_date?->format('F d, Y') ?? '';
    }

    /**
     * Get embed URL based on platform
     */
    public function getEmbedUrlAttribute(): string
    {
        switch ($this->video_platform) {
            case 'vimeo':
                return "https://player.vimeo.com/video/{$this->video_id}";
            case 'youtube':
                return "https://www.youtube.com/embed/{$this->video_id}";
            case 'bunny':
                return $this->video_url; // Bunny uses direct URLs
            case 'wistia':
                return "https://fast.wistia.net/embed/iframe/{$this->video_id}";
            default:
                return $this->video_url;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Increment view count
     */
    public function incrementViews(): void
    {
        $this->increment('views_count');
    }

    /**
     * Extract video ID from URL based on platform
     */
    public static function extractVideoId(string $url, string $platform): ?string
    {
        switch ($platform) {
            case 'vimeo':
                preg_match('/vimeo\.com\/(\d+)/', $url, $matches);
                return $matches[1] ?? null;

            case 'youtube':
                preg_match('/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&\s]+)/', $url, $matches);
                return $matches[1] ?? null;

            case 'wistia':
                preg_match('/wistia\.com\/medias\/(\w+)/', $url, $matches);
                return $matches[1] ?? null;

            default:
                return null;
        }
    }
}
