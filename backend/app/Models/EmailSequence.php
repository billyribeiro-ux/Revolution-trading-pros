<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Casts\AsCollection;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\Cache;

/**
 * Email Sequence Model (FluentCRM Pro Drip Campaigns)
 *
 * @property string $id
 * @property string $title
 * @property string $slug
 * @property string $status
 * @property string|null $description
 * @property string $design_template
 * @property array|null $settings
 * @property array|null $subscriber_settings
 * @property int $emails_count
 * @property int $subscribers_count
 * @property int $total_sent
 * @property int $total_opened
 * @property int $total_clicked
 * @property float $total_revenue
 * @property string $currency
 * @property string|null $created_by
 */
class EmailSequence extends Model
{
    use HasUuids, SoftDeletes;

    // Cache configuration
    private const CACHE_PREFIX = 'crm:sequence:';
    private const CACHE_TTL = 3600; // 1 hour
    private const CACHE_STATS_TTL = 300; // 5 minutes for frequently changing stats
    private const CACHE_TAG = 'crm_sequences';

    protected $fillable = [
        'title',
        'slug',
        'status',
        'description',
        'design_template',
        'settings',
        'subscriber_settings',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'settings' => 'array',
            'subscriber_settings' => 'array',
            'emails_count' => 'integer',
            'subscribers_count' => 'integer',
            'total_sent' => 'integer',
            'total_opened' => 'integer',
            'total_clicked' => 'integer',
            'total_revenue' => 'decimal:2',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $sequence): void {
            $sequence->slug ??= str($sequence->title)->slug()->toString();
            $sequence->status ??= 'draft';
            $sequence->design_template ??= 'simple';
            $sequence->settings ??= [
                'mailer_settings' => [
                    'from_name' => '',
                    'from_email' => '',
                    'reply_to_name' => '',
                    'reply_to_email' => '',
                    'is_custom' => false,
                ],
            ];
        });

        // Cache invalidation on model events
        static::saved(fn (self $sequence) => $sequence->clearCache());
        static::deleted(fn (self $sequence) => $sequence->clearCache());
    }

    // =====================================================
    // CACHE METHODS
    // =====================================================

    /**
     * Get sequence with caching
     */
    public static function findCached(string $id): ?self
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . $id,
            self::CACHE_TTL,
            fn () => self::with(['emails', 'creator'])->find($id)
        );
    }

    /**
     * Get all active sequences with caching
     */
    public static function getActiveCached(): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'active:all',
            self::CACHE_TTL,
            fn () => self::active()->with('emails')->get()
        );
    }

    /**
     * Get sequence stats with short TTL caching
     */
    public function getStatsCached(): array
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . $this->id . ':stats',
            self::CACHE_STATS_TTL,
            fn () => $this->getStats()
        );
    }

    /**
     * Clear all cache for this sequence
     */
    public function clearCache(): void
    {
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id);
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id . ':stats');
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'active:all');
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'list');
    }

    /**
     * Clear all sequence cache (static)
     */
    public static function clearAllCache(): void
    {
        Cache::tags([self::CACHE_TAG])->flush();
    }

    // Relationships
    public function emails(): HasMany
    {
        return $this->hasMany(SequenceMail::class, 'sequence_id')->orderBy('position');
    }

    public function trackers(): HasMany
    {
        return $this->hasMany(SequenceTracker::class, 'sequence_id');
    }

    public function activeTrackers(): HasMany
    {
        return $this->trackers()->where('status', 'active');
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

    public function scopeDraft(Builder $query): Builder
    {
        return $query->where('status', 'draft');
    }

    // Business Logic
    public function subscribe(Contact $contact, ?int $startFromPosition = 0): ?SequenceTracker
    {
        // Check if already subscribed
        if ($this->trackers()->where('contact_id', $contact->id)->exists()) {
            return null;
        }

        $firstEmail = $this->emails()
            ->where('status', 'active')
            ->where('position', '>=', $startFromPosition)
            ->orderBy('position')
            ->first();

        if (!$firstEmail) {
            return null;
        }

        return $this->trackers()->create([
            'contact_id' => $contact->id,
            'next_sequence_mail_id' => $firstEmail->id,
            'status' => 'active',
            'started_at' => now(),
            'next_execution_at' => now()->addSeconds($firstEmail->delay),
        ]);
    }

    public function unsubscribe(Contact $contact, string $reason = 'manual'): bool
    {
        return $this->trackers()
            ->where('contact_id', $contact->id)
            ->where('status', 'active')
            ->update([
                'status' => 'cancelled',
                'notes' => array_merge(
                    $this->trackers()->where('contact_id', $contact->id)->first()?->notes ?? [],
                    [['action' => 'unsubscribed', 'reason' => $reason, 'at' => now()->toIso8601String()]]
                ),
            ]) > 0;
    }

    public function getStats(): array
    {
        return [
            'emails' => $this->emails_count,
            'subscribers' => $this->subscribers_count,
            'active_subscribers' => $this->activeTrackers()->count(),
            'completed' => $this->trackers()->where('status', 'completed')->count(),
            'total_sent' => $this->total_sent,
            'total_opened' => $this->total_opened,
            'total_clicked' => $this->total_clicked,
            'open_rate' => $this->total_sent > 0 ? round(($this->total_opened / $this->total_sent) * 100, 2) : 0,
            'click_rate' => $this->total_opened > 0 ? round(($this->total_clicked / $this->total_opened) * 100, 2) : 0,
            'revenue' => [
                'amount' => number_format((float) $this->total_revenue, 2),
                'currency' => $this->currency,
            ],
        ];
    }

    public function recalculateCounts(): void
    {
        $this->update([
            'emails_count' => $this->emails()->count(),
            'subscribers_count' => $this->trackers()->count(),
            'total_sent' => $this->emails()->sum('sent_count'),
            'total_opened' => $this->emails()->sum('open_count'),
            'total_clicked' => $this->emails()->sum('click_count'),
            'total_revenue' => $this->emails()->sum('revenue'),
        ]);
    }
}
