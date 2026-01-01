<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Room Learning Content Model
 *
 * Represents learning center content for each trading room/alert service
 */
class RoomLearningContent extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'room_learning_content';

    protected $fillable = [
        'trading_room_id',
        'trader_id',
        'title',
        'description',
        'content_type',
        'content_url',
        'thumbnail_url',
        'duration',
        'difficulty_level',
        'category',
        'sort_order',
        'is_published',
        'views_count',
        'metadata',
    ];

    protected $casts = [
        'trading_room_id' => 'integer',
        'trader_id' => 'integer',
        'duration' => 'integer',
        'sort_order' => 'integer',
        'is_published' => 'boolean',
        'views_count' => 'integer',
        'metadata' => 'array',
    ];

    // Relationships
    public function tradingRoom(): BelongsTo
    {
        return $this->belongsTo(TradingRoom::class);
    }

    public function trader(): BelongsTo
    {
        return $this->belongsTo(RoomTrader::class, 'trader_id');
    }

    // Scopes
    public function scopePublished($query)
    {
        return $query->where('is_published', true);
    }

    public function scopeForRoom($query, string $slug)
    {
        return $query->whereHas('tradingRoom', fn($q) => $q->where('slug', $slug));
    }

    public function scopeByCategory($query, string $category)
    {
        return $query->where('category', $category);
    }

    public function scopeByType($query, string $type)
    {
        return $query->where('content_type', $type);
    }

    public function scopeOrdered($query)
    {
        return $query->orderBy('sort_order');
    }
}
