<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;

/**
 * Sequence Mail Model (Individual emails in a sequence)
 *
 * @property string $id
 * @property string $sequence_id
 * @property string $title
 * @property string $email_subject
 * @property string|null $email_pre_header
 * @property string $email_body
 * @property int $delay
 * @property string $delay_unit
 * @property int $delay_value
 * @property string $status
 * @property array|null $settings
 * @property int $position
 * @property int $sent_count
 * @property int $open_count
 * @property int $click_count
 * @property int $unsubscribe_count
 * @property float $revenue
 */
class SequenceMail extends Model
{
    use HasUuids;

    protected $fillable = [
        'sequence_id',
        'title',
        'email_subject',
        'email_pre_header',
        'email_body',
        'delay',
        'delay_unit',
        'delay_value',
        'status',
        'settings',
        'position',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'settings' => 'array',
            'delay' => 'integer',
            'delay_value' => 'integer',
            'position' => 'integer',
            'sent_count' => 'integer',
            'open_count' => 'integer',
            'click_count' => 'integer',
            'unsubscribe_count' => 'integer',
            'revenue' => 'decimal:2',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $mail): void {
            $mail->status ??= 'draft';
            $mail->delay_unit ??= 'days';
            $mail->delay_value ??= 1;
            $mail->delay = self::calculateDelayInSeconds($mail->delay_value, $mail->delay_unit);
            $mail->settings ??= [
                'mailer_settings' => [
                    'from_name' => '',
                    'from_email' => '',
                    'reply_to_name' => '',
                    'reply_to_email' => '',
                    'is_custom' => false,
                ],
                'timings' => [
                    'selected_days_only' => false,
                    'allowed_days' => [],
                    'sending_time' => null,
                ],
            ];
        });

        static::saving(function (self $mail): void {
            $mail->delay = self::calculateDelayInSeconds($mail->delay_value, $mail->delay_unit);
        });

        static::saved(function (self $mail): void {
            $mail->sequence?->recalculateCounts();
        });

        static::deleted(function (self $mail): void {
            $mail->sequence?->recalculateCounts();
        });
    }

    // Relationships
    public function sequence(): BelongsTo
    {
        return $this->belongsTo(EmailSequence::class, 'sequence_id');
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    // Scopes
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', 'active');
    }

    public function scopeOrdered(Builder $query): Builder
    {
        return $query->orderBy('position')->orderBy('delay');
    }

    // Helpers
    public static function calculateDelayInSeconds(int $value, string $unit): int
    {
        return match ($unit) {
            'minutes' => $value * 60,
            'hours' => $value * 3600,
            'days' => $value * 86400,
            'weeks' => $value * 604800,
            default => $value * 86400,
        };
    }

    public function getDelayForHumans(): string
    {
        if ($this->delay_value === 0) {
            return 'Immediately';
        }

        $unit = $this->delay_value === 1
            ? rtrim($this->delay_unit, 's')
            : $this->delay_unit;

        return "{$this->delay_value} {$unit}";
    }

    public function getNextMail(): ?self
    {
        return self::where('sequence_id', $this->sequence_id)
            ->where('position', '>', $this->position)
            ->where('status', 'active')
            ->orderBy('position')
            ->first();
    }

    public function getOpenRate(): float
    {
        return $this->sent_count > 0
            ? round(($this->open_count / $this->sent_count) * 100, 2)
            : 0;
    }

    public function getClickRate(): float
    {
        return $this->open_count > 0
            ? round(($this->click_count / $this->open_count) * 100, 2)
            : 0;
    }
}
