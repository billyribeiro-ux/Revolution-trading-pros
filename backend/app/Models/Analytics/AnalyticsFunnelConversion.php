<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * AnalyticsFunnelConversion Model - Individual User Funnel Progress
 *
 * @property int $id
 * @property int $funnel_id
 * @property int|null $user_id
 * @property int $current_step
 * @property bool $is_converted
 */
class AnalyticsFunnelConversion extends Model
{
    protected $table = 'analytics_funnel_conversions';

    protected $fillable = [
        'funnel_id',
        'user_id',
        'anonymous_id',
        'session_id',
        'current_step',
        'max_step_reached',
        'is_converted',
        'dropped_at_step',
        'started_at',
        'converted_at',
        'last_activity_at',
        'total_duration_seconds',
        'step_timestamps',
        'step_durations',
        'entry_channel',
        'utm_source',
        'utm_campaign',
        'conversion_value',
        'started_date',
    ];

    protected $casts = [
        'funnel_id' => 'integer',
        'user_id' => 'integer',
        'current_step' => 'integer',
        'max_step_reached' => 'integer',
        'is_converted' => 'boolean',
        'dropped_at_step' => 'integer',
        'total_duration_seconds' => 'integer',
        'step_timestamps' => 'array',
        'step_durations' => 'array',
        'conversion_value' => 'decimal:4',
        'started_at' => 'datetime',
        'converted_at' => 'datetime',
        'last_activity_at' => 'datetime',
        'started_date' => 'date',
    ];

    protected $attributes = [
        'current_step' => 0,
        'max_step_reached' => 0,
        'is_converted' => false,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function funnel(): BelongsTo
    {
        return $this->belongsTo(AnalyticsFunnel::class, 'funnel_id');
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeConverted(Builder $query): Builder
    {
        return $query->where('is_converted', true);
    }

    public function scopeDroppedOff(Builder $query): Builder
    {
        return $query->where('is_converted', false)->whereNotNull('dropped_at_step');
    }

    public function scopeInProgress(Builder $query): Builder
    {
        return $query->where('is_converted', false)->whereNull('dropped_at_step');
    }

    public function scopeByFunnel(Builder $query, int $funnelId): Builder
    {
        return $query->where('funnel_id', $funnelId);
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('started_at', [$start, $end]);
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Advance to next step
     */
    public function advanceToStep(int $stepNumber): void
    {
        $now = now();

        $this->current_step = $stepNumber;
        $this->max_step_reached = max($this->max_step_reached, $stepNumber);
        $this->last_activity_at = $now;

        // Record step timestamp
        $timestamps = $this->step_timestamps ?? [];
        $timestamps[$stepNumber] = $now->toIso8601String();
        $this->step_timestamps = $timestamps;

        // Calculate duration from previous step
        if ($stepNumber > 1 && isset($timestamps[$stepNumber - 1])) {
            $previousTime = Carbon::parse($timestamps[$stepNumber - 1]);
            $durations = $this->step_durations ?? [];
            $durations[$stepNumber] = $previousTime->diffInSeconds($now);
            $this->step_durations = $durations;
        }

        $this->save();
    }

    /**
     * Mark as converted
     */
    public function markConverted(?float $value = null): void
    {
        $this->is_converted = true;
        $this->converted_at = now();
        $this->total_duration_seconds = $this->started_at->diffInSeconds($this->converted_at);

        if ($value !== null) {
            $this->conversion_value = $value;
        }

        $this->save();
    }

    /**
     * Mark as dropped off
     */
    public function markDroppedOff(): void
    {
        $this->dropped_at_step = $this->current_step;
        $this->last_activity_at = now();
        $this->save();
    }

    /**
     * Get time to convert in hours
     */
    public function getTimeToConvertHours(): ?float
    {
        if (!$this->is_converted || !$this->converted_at) {
            return null;
        }

        return round($this->started_at->diffInMinutes($this->converted_at) / 60, 2);
    }

    /**
     * Get slowest step (longest duration)
     */
    public function getSlowestStep(): ?array
    {
        if (empty($this->step_durations)) {
            return null;
        }

        $maxStep = null;
        $maxDuration = 0;

        foreach ($this->step_durations as $step => $duration) {
            if ($duration > $maxDuration) {
                $maxDuration = $duration;
                $maxStep = $step;
            }
        }

        return [
            'step' => $maxStep,
            'duration_seconds' => $maxDuration,
        ];
    }
}
