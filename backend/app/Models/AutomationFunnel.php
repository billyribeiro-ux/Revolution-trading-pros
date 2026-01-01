<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\Cache;

/**
 * Automation Funnel Model (FluentCRM Pro Automation Workflows)
 *
 * Funnels are automation workflows triggered by events:
 * - contact_created, tag_applied, list_applied
 * - form_submitted, order_completed
 * - user_login, birthday, custom_event
 *
 * @property string $id
 * @property string $title
 * @property string $slug
 * @property string|null $description
 * @property string $status
 * @property string $trigger_type
 * @property array|null $trigger_settings
 * @property array|null $conditions
 * @property int $subscribers_count
 * @property int $completed_count
 * @property float $total_revenue
 * @property string|null $created_by
 */
class AutomationFunnel extends Model
{
    use HasUuids, SoftDeletes;

    // Cache configuration
    private const CACHE_PREFIX = 'crm:funnel:';
    private const CACHE_TTL = 3600; // 1 hour
    private const CACHE_STATS_TTL = 300; // 5 minutes for frequently changing stats
    private const CACHE_TAG = 'crm_funnels';

    protected $fillable = [
        'title',
        'slug',
        'description',
        'status',
        'trigger_type',
        'trigger_settings',
        'conditions',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'trigger_settings' => 'array',
            'conditions' => 'array',
            'subscribers_count' => 'integer',
            'completed_count' => 'integer',
            'total_revenue' => 'decimal:2',
        ];
    }

    // Available trigger types (FluentCRM Pro compatible)
    public const TRIGGER_TYPES = [
        'contact_created' => 'When a contact is created',
        'tag_applied' => 'When a tag is applied',
        'tag_removed' => 'When a tag is removed',
        'list_applied' => 'When added to a list',
        'list_removed' => 'When removed from a list',
        'contact_status_changed' => 'When contact status changes',
        'form_submitted' => 'When a form is submitted',
        'order_completed' => 'When an order is completed',
        'order_refunded' => 'When an order is refunded',
        'subscription_started' => 'When subscription starts',
        'subscription_cancelled' => 'When subscription is cancelled',
        'user_login' => 'When user logs in',
        'user_registered' => 'When user registers',
        'birthday' => 'On contact birthday',
        'sequence_completed' => 'When email sequence is completed',
        'link_clicked' => 'When a link is clicked',
        'email_opened' => 'When an email is opened',
        'custom_event' => 'Custom event trigger',
    ];

    protected static function booted(): void
    {
        static::creating(function (self $funnel): void {
            $funnel->slug ??= str($funnel->title)->slug()->toString();
            $funnel->status ??= 'draft';
            $funnel->trigger_settings ??= [];
            $funnel->conditions ??= [];
        });

        // Cache invalidation on model events
        static::saved(fn (self $funnel) => $funnel->clearCache());
        static::deleted(fn (self $funnel) => $funnel->clearCache());
    }

    // =====================================================
    // CACHE METHODS
    // =====================================================

    /**
     * Find funnel by ID with caching
     */
    public static function findCached(string $id): ?self
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . $id,
            self::CACHE_TTL,
            fn () => self::with(['actions', 'creator'])->find($id)
        );
    }

    /**
     * Get all active funnels with caching
     */
    public static function getActiveCached(): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'active:all',
            self::CACHE_TTL,
            fn () => self::active()->with('actions')->get()
        );
    }

    /**
     * Get funnels by trigger type with caching
     */
    public static function getByTriggerCached(string $triggerType): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'trigger:' . $triggerType,
            self::CACHE_TTL,
            fn () => self::active()->byTrigger($triggerType)->with('actions')->get()
        );
    }

    /**
     * Get funnel stats with short TTL caching
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
     * Clear all cache for this funnel
     */
    public function clearCache(): void
    {
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id);
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id . ':stats');
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'active:all');
        if ($this->trigger_type) {
            Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'trigger:' . $this->trigger_type);
        }
    }

    /**
     * Clear all funnel cache (static)
     */
    public static function clearAllCache(): void
    {
        Cache::tags([self::CACHE_TAG])->flush();
    }

    // Relationships
    public function actions(): HasMany
    {
        return $this->hasMany(FunnelAction::class, 'funnel_id')->orderBy('position');
    }

    public function subscribers(): HasMany
    {
        return $this->hasMany(FunnelSubscriber::class, 'funnel_id');
    }

    public function activeSubscribers(): HasMany
    {
        return $this->subscribers()->whereIn('status', ['active', 'waiting']);
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

    public function scopeByTrigger(Builder $query, string $triggerType): Builder
    {
        return $query->where('trigger_type', $triggerType);
    }

    // Business Logic
    public function canEnroll(Contact $contact): bool
    {
        // Check if already enrolled
        if ($this->subscribers()->where('contact_id', $contact->id)->whereIn('status', ['active', 'waiting'])->exists()) {
            return false;
        }

        // Check entry conditions
        return $this->evaluateConditions($contact);
    }

    public function evaluateConditions(Contact $contact): bool
    {
        foreach ($this->conditions ?? [] as $condition) {
            if (!$this->evaluateCondition($contact, $condition)) {
                return false;
            }
        }
        return true;
    }

    private function evaluateCondition(Contact $contact, array $condition): bool
    {
        $field = $condition['field'] ?? null;
        $operator = $condition['operator'] ?? 'equals';
        $value = $condition['value'] ?? null;

        if (!$field) {
            return true;
        }

        $contactValue = data_get($contact, $field);

        return match ($operator) {
            'equals' => $contactValue == $value,
            'not_equals' => $contactValue != $value,
            'contains' => str_contains((string) $contactValue, (string) $value),
            'not_contains' => !str_contains((string) $contactValue, (string) $value),
            'greater_than' => $contactValue > $value,
            'less_than' => $contactValue < $value,
            'is_empty' => empty($contactValue),
            'is_not_empty' => !empty($contactValue),
            'in' => in_array($contactValue, (array) $value),
            'not_in' => !in_array($contactValue, (array) $value),
            default => true,
        };
    }

    public function enroll(Contact $contact): ?FunnelSubscriber
    {
        if (!$this->canEnroll($contact)) {
            return null;
        }

        $firstAction = $this->actions()->first();

        $subscriber = $this->subscribers()->create([
            'contact_id' => $contact->id,
            'current_action_id' => $firstAction?->id,
            'status' => 'active',
            'entered_at' => now(),
            'next_execution_at' => $firstAction ? now()->addSeconds($firstAction->delay_seconds) : null,
        ]);

        $this->increment('subscribers_count');

        return $subscriber;
    }

    public function removeSubscriber(Contact $contact, string $reason = 'manual'): bool
    {
        $subscriber = $this->subscribers()
            ->where('contact_id', $contact->id)
            ->whereIn('status', ['active', 'waiting'])
            ->first();

        if (!$subscriber) {
            return false;
        }

        $subscriber->cancel($reason);
        return true;
    }

    public function activate(): void
    {
        $this->update(['status' => 'active']);
    }

    public function pause(): void
    {
        $this->update(['status' => 'paused']);
    }

    public function getStats(): array
    {
        $subscribers = $this->subscribers();

        return [
            'total_subscribers' => $this->subscribers_count,
            'active_subscribers' => $subscribers->clone()->where('status', 'active')->count(),
            'waiting_subscribers' => $subscribers->clone()->where('status', 'waiting')->count(),
            'completed' => $this->completed_count,
            'cancelled' => $subscribers->clone()->where('status', 'cancelled')->count(),
            'failed' => $subscribers->clone()->where('status', 'failed')->count(),
            'completion_rate' => $this->subscribers_count > 0
                ? round(($this->completed_count / $this->subscribers_count) * 100, 2)
                : 0,
            'total_revenue' => $this->total_revenue,
        ];
    }

    public static function getTriggersForEvent(string $triggerType): \Illuminate\Database\Eloquent\Collection
    {
        return self::active()->byTrigger($triggerType)->get();
    }
}
