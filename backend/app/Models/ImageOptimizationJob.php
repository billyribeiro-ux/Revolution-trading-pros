<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * ImageOptimizationJob Model
 *
 * Tracks image optimization job status and results.
 *
 * @property int $id
 * @property int $media_id
 * @property int|null $preset_id
 * @property string $status
 * @property int $priority
 * @property int $attempts
 * @property int $max_attempts
 * @property array|null $operations
 * @property array|null $options
 * @property int $progress
 * @property string|null $current_operation
 * @property int|null $processing_time_ms
 * @property int|null $memory_peak_bytes
 * @property array|null $results
 * @property int|null $original_size
 * @property int|null $optimized_size
 * @property float|null $total_savings_percent
 * @property string|null $error_message
 * @property string|null $error_trace
 * @property \Carbon\Carbon|null $started_at
 * @property \Carbon\Carbon|null $completed_at
 * @property \Carbon\Carbon|null $scheduled_at
 */
class ImageOptimizationJob extends Model
{
    use HasFactory;

    // Job statuses
    public const STATUS_PENDING = 'pending';
    public const STATUS_PROCESSING = 'processing';
    public const STATUS_COMPLETED = 'completed';
    public const STATUS_FAILED = 'failed';
    public const STATUS_CANCELLED = 'cancelled';

    // Operations
    public const OP_CONVERT_WEBP = 'convert_webp';
    public const OP_CONVERT_AVIF = 'convert_avif';
    public const OP_GENERATE_THUMBNAIL = 'generate_thumbnail';
    public const OP_GENERATE_RESPONSIVE = 'generate_responsive';
    public const OP_GENERATE_RETINA = 'generate_retina';
    public const OP_COMPRESS = 'compress';
    public const OP_STRIP_METADATA = 'strip_metadata';
    public const OP_GENERATE_LQIP = 'generate_lqip';
    public const OP_GENERATE_BLURHASH = 'generate_blurhash';
    public const OP_RESIZE = 'resize';

    protected $table = 'image_optimization_jobs';

    protected $fillable = [
        'media_id',
        'preset_id',
        'status',
        'priority',
        'attempts',
        'max_attempts',
        'operations',
        'options',
        'progress',
        'current_operation',
        'processing_time_ms',
        'memory_peak_bytes',
        'results',
        'original_size',
        'optimized_size',
        'total_savings_percent',
        'error_message',
        'error_trace',
        'started_at',
        'completed_at',
        'scheduled_at',
    ];

    protected $casts = [
        'operations' => 'array',
        'options' => 'array',
        'results' => 'array',
        'priority' => 'integer',
        'attempts' => 'integer',
        'max_attempts' => 'integer',
        'progress' => 'integer',
        'processing_time_ms' => 'integer',
        'memory_peak_bytes' => 'integer',
        'original_size' => 'integer',
        'optimized_size' => 'integer',
        'total_savings_percent' => 'decimal:2',
        'started_at' => 'datetime',
        'completed_at' => 'datetime',
        'scheduled_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_PENDING,
        'priority' => 5,
        'attempts' => 0,
        'max_attempts' => 3,
        'progress' => 0,
    ];

    /**
     * Get the media being optimized
     */
    public function media(): BelongsTo
    {
        return $this->belongsTo(Media::class);
    }

    /**
     * Get the preset used
     */
    public function preset(): BelongsTo
    {
        return $this->belongsTo(ImageOptimizationPreset::class, 'preset_id');
    }

    /**
     * Check if job is pending
     */
    public function isPending(): bool
    {
        return $this->status === self::STATUS_PENDING;
    }

    /**
     * Check if job is processing
     */
    public function isProcessing(): bool
    {
        return $this->status === self::STATUS_PROCESSING;
    }

    /**
     * Check if job completed
     */
    public function isCompleted(): bool
    {
        return $this->status === self::STATUS_COMPLETED;
    }

    /**
     * Check if job failed
     */
    public function isFailed(): bool
    {
        return $this->status === self::STATUS_FAILED;
    }

    /**
     * Check if can retry
     */
    public function canRetry(): bool
    {
        return $this->isFailed() && $this->attempts < $this->max_attempts;
    }

    /**
     * Mark as processing
     */
    public function markAsProcessing(): self
    {
        $this->update([
            'status' => self::STATUS_PROCESSING,
            'started_at' => now(),
            'attempts' => $this->attempts + 1,
        ]);
        return $this;
    }

    /**
     * Update progress
     */
    public function updateProgress(int $progress, ?string $operation = null): self
    {
        $data = ['progress' => min(100, max(0, $progress))];
        if ($operation) {
            $data['current_operation'] = $operation;
        }
        $this->update($data);
        return $this;
    }

    /**
     * Mark as completed
     */
    public function markAsCompleted(array $results = []): self
    {
        $processingTime = $this->started_at
            ? now()->diffInMilliseconds($this->started_at)
            : null;

        $savings = 0;
        if ($this->original_size && $this->optimized_size) {
            $savings = round((1 - ($this->optimized_size / $this->original_size)) * 100, 2);
        }

        $this->update([
            'status' => self::STATUS_COMPLETED,
            'progress' => 100,
            'completed_at' => now(),
            'processing_time_ms' => $processingTime,
            'memory_peak_bytes' => memory_get_peak_usage(true),
            'results' => $results,
            'total_savings_percent' => $savings,
        ]);

        return $this;
    }

    /**
     * Mark as failed
     */
    public function markAsFailed(string $error, ?string $trace = null): self
    {
        $this->update([
            'status' => self::STATUS_FAILED,
            'error_message' => $error,
            'error_trace' => $trace,
            'completed_at' => now(),
        ]);
        return $this;
    }

    /**
     * Cancel job
     */
    public function cancel(): self
    {
        $this->update([
            'status' => self::STATUS_CANCELLED,
            'completed_at' => now(),
        ]);
        return $this;
    }

    /**
     * Get savings in bytes
     */
    public function getSavingsBytesAttribute(): int
    {
        if (!$this->original_size || !$this->optimized_size) {
            return 0;
        }
        return max(0, $this->original_size - $this->optimized_size);
    }

    /**
     * Get formatted processing time
     */
    public function getFormattedProcessingTimeAttribute(): string
    {
        if (!$this->processing_time_ms) {
            return 'N/A';
        }

        if ($this->processing_time_ms < 1000) {
            return $this->processing_time_ms . 'ms';
        }

        return round($this->processing_time_ms / 1000, 2) . 's';
    }

    /**
     * Scope: Pending jobs
     */
    public function scopePending($query)
    {
        return $query->where('status', self::STATUS_PENDING);
    }

    /**
     * Scope: Processing jobs
     */
    public function scopeProcessing($query)
    {
        return $query->where('status', self::STATUS_PROCESSING);
    }

    /**
     * Scope: Completed jobs
     */
    public function scopeCompleted($query)
    {
        return $query->where('status', self::STATUS_COMPLETED);
    }

    /**
     * Scope: Failed jobs
     */
    public function scopeFailed($query)
    {
        return $query->where('status', self::STATUS_FAILED);
    }

    /**
     * Scope: By priority (lower first)
     */
    public function scopeOrderByPriority($query)
    {
        return $query->orderBy('priority', 'asc')->orderBy('created_at', 'asc');
    }

    /**
     * Scope: Ready to process
     */
    public function scopeReadyToProcess($query)
    {
        return $query->pending()
            ->whereRaw('attempts < max_attempts')
            ->where(function ($q) {
                $q->whereNull('scheduled_at')
                    ->orWhere('scheduled_at', '<=', now());
            })
            ->orderByPriority();
    }

    /**
     * Get next job to process
     */
    public static function getNextJob(): ?self
    {
        return static::readyToProcess()->first();
    }

    /**
     * Get statistics
     */
    public static function getStats(): array
    {
        return [
            'pending' => static::pending()->count(),
            'processing' => static::processing()->count(),
            'completed' => static::completed()->count(),
            'failed' => static::failed()->count(),
            'total_savings_bytes' => static::completed()->sum(\DB::raw('original_size - optimized_size')),
            'avg_processing_time_ms' => static::completed()->avg('processing_time_ms'),
        ];
    }

    /**
     * Export to array
     */
    public function toJobArray(): array
    {
        return [
            'id' => $this->id,
            'media_id' => $this->media_id,
            'status' => $this->status,
            'progress' => $this->progress,
            'current_operation' => $this->current_operation,
            'attempts' => $this->attempts,
            'max_attempts' => $this->max_attempts,
            'original_size' => $this->original_size,
            'optimized_size' => $this->optimized_size,
            'savings_percent' => $this->total_savings_percent,
            'savings_bytes' => $this->savings_bytes,
            'processing_time' => $this->formatted_processing_time,
            'error' => $this->error_message,
            'started_at' => $this->started_at?->toIso8601String(),
            'completed_at' => $this->completed_at?->toIso8601String(),
        ];
    }
}
