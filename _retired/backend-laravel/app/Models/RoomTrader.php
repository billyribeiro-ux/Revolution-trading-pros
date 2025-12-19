<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;

/**
 * Room Trader Model
 *
 * Represents traders who create content for trading rooms and alert services
 *
 * @property int $id
 * @property string $name
 * @property string $slug
 * @property string|null $title
 * @property string|null $bio
 * @property string|null $photo_url
 * @property string|null $email
 * @property array|null $social_links
 * @property array|null $specialties
 * @property bool $is_active
 * @property int $sort_order
 */
class RoomTrader extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'name',
        'slug',
        'title',
        'bio',
        'photo_url',
        'email',
        'social_links',
        'specialties',
        'is_active',
        'sort_order',
    ];

    protected $casts = [
        'is_active' => 'boolean',
        'sort_order' => 'integer',
        'social_links' => 'array',
        'specialties' => 'array',
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // RELATIONSHIPS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get trading rooms this trader belongs to
     */
    public function tradingRooms(): BelongsToMany
    {
        return $this->belongsToMany(TradingRoom::class, 'trading_room_trader')
            ->withPivot('is_primary')
            ->withTimestamps();
    }

    /**
     * Get daily videos by this trader
     */
    public function dailyVideos(): HasMany
    {
        return $this->hasMany(RoomDailyVideo::class, 'trader_id');
    }

    /**
     * Get learning content by this trader
     */
    public function learningContent(): HasMany
    {
        return $this->hasMany(RoomLearningContent::class, 'trader_id');
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SCOPES
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Only active traders
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
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
     * Get total video count across all rooms
     */
    public function getVideoCountAttribute(): int
    {
        return $this->dailyVideos()->where('is_published', true)->count();
    }

    /**
     * Get initials for avatar fallback
     */
    public function getInitialsAttribute(): string
    {
        $words = explode(' ', $this->name);
        $initials = '';
        foreach ($words as $word) {
            $initials .= strtoupper(substr($word, 0, 1));
        }
        return substr($initials, 0, 2);
    }
}
