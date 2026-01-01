<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Room Archive Model
 *
 * Represents trading room session recordings
 */
class RoomArchive extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'trading_room_id',
        'title',
        'description',
        'video_url',
        'thumbnail_url',
        'duration',
        'recording_date',
        'session_type',
        'is_published',
        'views_count',
        'timestamps',
        'metadata',
    ];

    protected $casts = [
        'trading_room_id' => 'integer',
        'duration' => 'integer',
        'recording_date' => 'date',
        'is_published' => 'boolean',
        'views_count' => 'integer',
        'timestamps' => 'array',
        'metadata' => 'array',
    ];

    // Relationships
    public function tradingRoom(): BelongsTo
    {
        return $this->belongsTo(TradingRoom::class);
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

    public function scopeLatest($query)
    {
        return $query->orderByDesc('recording_date');
    }

    public function scopeBySessionType($query, string $type)
    {
        return $query->where('session_type', $type);
    }

    // Accessors
    public function getFormattedDurationAttribute(): string
    {
        if (!$this->duration) return '';
        $hours = floor($this->duration / 3600);
        $minutes = floor(($this->duration % 3600) / 60);
        $seconds = $this->duration % 60;
        return $hours > 0
            ? sprintf('%d:%02d:%02d', $hours, $minutes, $seconds)
            : sprintf('%d:%02d', $minutes, $seconds);
    }
}
