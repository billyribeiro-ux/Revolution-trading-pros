<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Rank Alert Model
 * 
 * @property int $id
 * @property int $rank_tracking_id
 * @property string $type
 * @property string $severity
 * @property int|null $previous_position
 * @property int|null $current_position
 * @property int|null $threshold
 * @property array|null $metadata
 * @property bool $is_read
 * @property \Carbon\Carbon|null $read_at
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class RankAlert extends Model
{
    use HasFactory;

    protected $fillable = [
        'rank_tracking_id',
        'type',
        'severity',
        'previous_position',
        'current_position',
        'threshold',
        'metadata',
        'is_read',
        'read_at',
    ];

    protected $casts = [
        'metadata' => 'array',
        'is_read' => 'boolean',
        'read_at' => 'datetime',
        'previous_position' => 'integer',
        'current_position' => 'integer',
        'threshold' => 'integer',
    ];

    /**
     * Alert types.
     */
    public const TYPE_DROP = 'ranking_drop';
    public const TYPE_RISE = 'ranking_rise';
    public const TYPE_ENTERED_TOP = 'entered_top';
    public const TYPE_EXITED_TOP = 'exited_top';
    public const TYPE_NOT_FOUND = 'not_found';

    /**
     * Alert severities.
     */
    public const SEVERITY_INFO = 'info';
    public const SEVERITY_WARNING = 'warning';
    public const SEVERITY_CRITICAL = 'critical';

    /**
     * Get the rank tracking this alert belongs to.
     */
    public function rankTracking(): BelongsTo
    {
        return $this->belongsTo(RankTracking::class);
    }

    /**
     * Scope to unread alerts.
     */
    public function scopeUnread($query)
    {
        return $query->where('is_read', false);
    }

    /**
     * Scope to critical alerts.
     */
    public function scopeCritical($query)
    {
        return $query->where('severity', self::SEVERITY_CRITICAL);
    }

    /**
     * Mark alert as read.
     */
    public function markAsRead(): void
    {
        $this->update([
            'is_read' => true,
            'read_at' => now(),
        ]);
    }
}
