<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;

/**
 * Trading Room / Alert Service Model
 *
 * Represents trading rooms (Day Trading, Swing Trading, Small Account Mentorship)
 * and alert services (SPX Profit Pulse, Explosive Swing)
 *
 * @property int $id
 * @property string $name
 * @property string $slug
 * @property string $type
 * @property string|null $description
 * @property string|null $short_description
 * @property string|null $icon
 * @property string|null $color
 * @property string|null $image_url
 * @property string|null $logo_url
 * @property bool $is_active
 * @property bool $is_featured
 * @property int $sort_order
 * @property array|null $features
 * @property array|null $schedule
 * @property array|null $metadata
 */
class TradingRoom extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'name',
        'slug',
        'type',
        'description',
        'short_description',
        'icon',
        'color',
        'image_url',
        'logo_url',
        'is_active',
        'is_featured',
        'sort_order',
        'features',
        'schedule',
        'metadata',
    ];

    protected $casts = [
        'is_active' => 'boolean',
        'is_featured' => 'boolean',
        'sort_order' => 'integer',
        'features' => 'array',
        'schedule' => 'array',
        'metadata' => 'array',
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // RELATIONSHIPS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get traders associated with this room
     */
    public function traders(): BelongsToMany
    {
        return $this->belongsToMany(RoomTrader::class, 'trading_room_trader')
            ->withPivot('is_primary')
            ->withTimestamps()
            ->orderByPivot('is_primary', 'desc');
    }

    /**
     * Get daily videos for this room
     */
    public function dailyVideos(): HasMany
    {
        return $this->hasMany(RoomDailyVideo::class)
            ->orderByDesc('video_date');
    }

    /**
     * Get learning content for this room
     */
    public function learningContent(): HasMany
    {
        return $this->hasMany(RoomLearningContent::class)
            ->orderBy('sort_order');
    }

    /**
     * Get video archives for this room
     */
    public function archives(): HasMany
    {
        return $this->hasMany(RoomArchive::class)
            ->orderByDesc('recording_date');
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SCOPES
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Only trading rooms
     */
    public function scopeTradingRooms($query)
    {
        return $query->where('type', 'trading_room');
    }

    /**
     * Only alert services
     */
    public function scopeAlertServices($query)
    {
        return $query->where('type', 'alert_service');
    }

    /**
     * Only active rooms/services
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    /**
     * Only featured rooms/services
     */
    public function scopeFeatured($query)
    {
        return $query->where('is_featured', true);
    }

    /**
     * Order by sort order
     */
    public function scopeOrdered($query)
    {
        return $query->orderBy('sort_order');
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ACCESSORS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get the primary trader for this room
     */
    public function getPrimaryTraderAttribute()
    {
        return $this->traders()->wherePivot('is_primary', true)->first();
    }

    /**
     * Check if this is a trading room
     */
    public function getIsTradingRoomAttribute(): bool
    {
        return $this->type === 'trading_room';
    }

    /**
     * Check if this is an alert service
     */
    public function getIsAlertServiceAttribute(): bool
    {
        return $this->type === 'alert_service';
    }

    /**
     * Get video count
     */
    public function getVideoCountAttribute(): int
    {
        return $this->dailyVideos()->where('is_published', true)->count();
    }
}
