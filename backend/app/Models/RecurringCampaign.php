<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * Recurring Campaign Model (FluentCRM Pro Scheduled Newsletters)
 *
 * @property string $id
 * @property string $title
 * @property string $slug
 * @property string $status
 * @property string|null $email_subject
 * @property string|null $email_pre_header
 * @property string|null $email_body
 * @property string $design_template
 * @property array|null $settings
 * @property array|null $scheduling_settings
 * @property array|null $subscriber_settings
 * @property array|null $template_config
 * @property array|null $labels
 * @property int $total_campaigns_sent
 * @property int $total_emails_sent
 * @property float $total_revenue
 * @property Carbon|null $last_sent_at
 * @property Carbon|null $next_scheduled_at
 * @property string|null $created_by
 */
class RecurringCampaign extends Model
{
    use HasUuids, SoftDeletes;

    protected $fillable = [
        'title',
        'slug',
        'status',
        'email_subject',
        'email_pre_header',
        'email_body',
        'design_template',
        'settings',
        'scheduling_settings',
        'subscriber_settings',
        'template_config',
        'labels',
        'next_scheduled_at',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'settings' => 'array',
            'scheduling_settings' => 'array',
            'subscriber_settings' => 'array',
            'template_config' => 'array',
            'labels' => 'array',
            'total_campaigns_sent' => 'integer',
            'total_emails_sent' => 'integer',
            'total_revenue' => 'decimal:2',
            'last_sent_at' => 'datetime',
            'next_scheduled_at' => 'datetime',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $campaign): void {
            $campaign->slug ??= str($campaign->title)->slug()->toString();
            $campaign->status ??= 'draft';
            $campaign->design_template ??= 'simple';
            $campaign->scheduling_settings ??= [
                'type' => 'weekly',
                'day' => 'mon',
                'time' => '09:00',
                'timezone' => config('app.timezone'),
                'send_automatically' => true,
            ];
            $campaign->subscriber_settings ??= [
                'subscribers' => [['list' => 'all', 'tag' => 'all']],
                'excluded_subscribers' => [],
                'sending_filter' => 'list_tag',
                'dynamic_segment' => null,
                'advanced_filters' => [],
            ];
            $campaign->settings ??= [
                'mailer_settings' => [
                    'from_name' => '',
                    'from_email' => '',
                    'reply_to_name' => '',
                    'reply_to_email' => '',
                    'is_custom' => false,
                ],
            ];
        });
    }

    // Relationships
    public function emails(): HasMany
    {
        return $this->hasMany(RecurringMail::class, 'recurring_campaign_id')->orderByDesc('scheduled_at');
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

    public function scopeDueForSending(Builder $query): Builder
    {
        return $query->where('status', 'active')
            ->whereNotNull('next_scheduled_at')
            ->where('next_scheduled_at', '<=', now());
    }

    // Business Logic
    public function calculateNextScheduledAt(): ?Carbon
    {
        $scheduling = $this->scheduling_settings;
        if (!$scheduling || !($scheduling['send_automatically'] ?? true)) {
            return null;
        }

        $timezone = $scheduling['timezone'] ?? config('app.timezone');
        $time = $scheduling['time'] ?? '09:00';
        [$hour, $minute] = explode(':', $time);

        $next = now($timezone)->setTime((int) $hour, (int) $minute, 0);

        return match ($scheduling['type'] ?? 'weekly') {
            'daily' => $next->isPast() ? $next->addDay() : $next,
            'weekly' => $this->calculateWeeklyNext($next, $scheduling['day'] ?? 'mon'),
            'monthly' => $this->calculateMonthlyNext($next, (int) ($scheduling['day_of_month'] ?? 1)),
            default => $next->addWeek(),
        };
    }

    private function calculateWeeklyNext(Carbon $base, string $day): Carbon
    {
        $dayMap = ['sun' => 0, 'mon' => 1, 'tue' => 2, 'wed' => 3, 'thu' => 4, 'fri' => 5, 'sat' => 6];
        $targetDay = $dayMap[$day] ?? 1;

        $next = $base->copy()->next($targetDay);
        if ($base->dayOfWeek === $targetDay && $base->isFuture()) {
            return $base;
        }

        return $next;
    }

    private function calculateMonthlyNext(Carbon $base, int $dayOfMonth): Carbon
    {
        $next = $base->copy()->day($dayOfMonth);
        if ($next->isPast()) {
            $next->addMonth();
        }
        return $next;
    }

    public function scheduleNextSend(): void
    {
        $nextAt = $this->calculateNextScheduledAt();
        $this->update(['next_scheduled_at' => $nextAt]);
    }

    public function createMailInstance(): RecurringMail
    {
        return $this->emails()->create([
            'email_subject' => $this->email_subject,
            'email_pre_header' => $this->email_pre_header,
            'email_body' => $this->email_body,
            'status' => 'scheduled',
            'scheduled_at' => now(),
        ]);
    }

    public function getStats(): array
    {
        $lastEmail = $this->emails()->latest('sent_at')->first();

        return [
            'total_campaigns_sent' => $this->total_campaigns_sent,
            'total_emails_sent' => $this->total_emails_sent,
            'total_revenue' => $this->total_revenue,
            'avg_open_rate' => $this->calculateAverageOpenRate(),
            'avg_click_rate' => $this->calculateAverageClickRate(),
            'last_sent_at' => $this->last_sent_at?->toIso8601String(),
            'next_scheduled_at' => $this->next_scheduled_at?->toIso8601String(),
            'last_email_stats' => $lastEmail ? [
                'sent' => $lastEmail->sent_count,
                'opened' => $lastEmail->open_count,
                'clicked' => $lastEmail->click_count,
            ] : null,
        ];
    }

    private function calculateAverageOpenRate(): float
    {
        $emails = $this->emails()->where('sent_count', '>', 0)->get();
        if ($emails->isEmpty()) {
            return 0;
        }

        $totalRate = $emails->sum(fn ($e) => ($e->open_count / $e->sent_count) * 100);
        return round($totalRate / $emails->count(), 2);
    }

    private function calculateAverageClickRate(): float
    {
        $emails = $this->emails()->where('open_count', '>', 0)->get();
        if ($emails->isEmpty()) {
            return 0;
        }

        $totalRate = $emails->sum(fn ($e) => ($e->click_count / $e->open_count) * 100);
        return round($totalRate / $emails->count(), 2);
    }
}
