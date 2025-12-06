<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Support\Facades\Cache;

/**
 * Contact List Model (FluentCRM mailing lists)
 *
 * @property string $id
 * @property string $title
 * @property string $slug
 * @property string|null $description
 * @property bool $is_public
 * @property int $contacts_count
 * @property string|null $created_by
 */
class ContactList extends Model
{
    use HasUuids;

    // Cache configuration
    private const CACHE_PREFIX = 'crm:list:';
    private const CACHE_TTL = 3600; // 1 hour
    private const CACHE_STATS_TTL = 300; // 5 minutes for frequently changing stats
    private const CACHE_TAG = 'crm_lists';

    protected $fillable = [
        'title',
        'slug',
        'description',
        'is_public',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'is_public' => 'boolean',
            'contacts_count' => 'integer',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $list): void {
            $list->slug ??= str($list->title)->slug()->toString();
            $list->is_public ??= false;
        });

        // Cache invalidation on model events
        static::saved(fn (self $list) => $list->clearCache());
        static::deleted(fn (self $list) => $list->clearCache());
    }

    // =====================================================
    // CACHE METHODS
    // =====================================================

    /**
     * Find list by ID with caching
     */
    public static function findCached(string $id): ?self
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . $id,
            self::CACHE_TTL,
            fn () => self::with('creator')->find($id)
        );
    }

    /**
     * Get all lists with caching
     */
    public static function getAllCached(): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'all',
            self::CACHE_TTL,
            fn () => self::orderBy('title')->get()
        );
    }

    /**
     * Get public lists with caching
     */
    public static function getPublicCached(): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'public:all',
            self::CACHE_TTL,
            fn () => self::where('is_public', true)->orderBy('title')->get()
        );
    }

    /**
     * Clear all cache for this list
     */
    public function clearCache(): void
    {
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id);
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'all');
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'public:all');
    }

    /**
     * Clear all list cache (static)
     */
    public static function clearAllCache(): void
    {
        Cache::tags([self::CACHE_TAG])->flush();
    }

    // Relationships
    public function contacts(): BelongsToMany
    {
        return $this->belongsToMany(Contact::class, 'contact_list_members', 'list_id', 'contact_id')
            ->withPivot(['status', 'subscribed_at', 'unsubscribed_at'])
            ->withTimestamps();
    }

    public function subscribedContacts(): BelongsToMany
    {
        return $this->contacts()->wherePivot('status', 'subscribed');
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    // Business Logic
    public function addContact(Contact $contact, string $status = 'subscribed'): void
    {
        if (!$this->contacts()->where('contacts.id', $contact->id)->exists()) {
            $this->contacts()->attach($contact->id, [
                'status' => $status,
                'subscribed_at' => now(),
            ]);
            $this->increment('contacts_count');
        }
    }

    public function removeContact(Contact $contact): void
    {
        if ($this->contacts()->where('contacts.id', $contact->id)->exists()) {
            $this->contacts()->updateExistingPivot($contact->id, [
                'status' => 'unsubscribed',
                'unsubscribed_at' => now(),
            ]);
            $this->decrement('contacts_count');
        }
    }

    public function hasContact(Contact $contact): bool
    {
        return $this->subscribedContacts()->where('contacts.id', $contact->id)->exists();
    }

    public function recalculateCount(): void
    {
        $this->update([
            'contacts_count' => $this->subscribedContacts()->count(),
        ]);
    }
}
