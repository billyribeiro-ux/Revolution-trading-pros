<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Support\Facades\Cache;

/**
 * Contact Tag Model (FluentCRM contact tags)
 *
 * @property string $id
 * @property string $title
 * @property string $slug
 * @property string|null $description
 * @property string $color
 * @property int $contacts_count
 * @property string|null $created_by
 */
class ContactTag extends Model
{
    use HasUuids;

    // Cache configuration
    private const CACHE_PREFIX = 'crm:tag:';
    private const CACHE_TTL = 3600; // 1 hour
    private const CACHE_TAG = 'crm_tags';

    protected $fillable = [
        'title',
        'slug',
        'description',
        'color',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'contacts_count' => 'integer',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $tag): void {
            $tag->slug ??= str($tag->title)->slug()->toString();
            $tag->color ??= '#6366F1';
        });

        // Cache invalidation on model events
        static::saved(fn (self $tag) => $tag->clearCache());
        static::deleted(fn (self $tag) => $tag->clearCache());
    }

    // =====================================================
    // CACHE METHODS
    // =====================================================

    /**
     * Find tag by ID with caching
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
     * Find tag by slug with caching
     */
    public static function findBySlugCached(string $slug): ?self
    {
        return Cache::tags([self::CACHE_TAG])->remember(
            self::CACHE_PREFIX . 'slug:' . $slug,
            self::CACHE_TTL,
            fn () => self::where('slug', $slug)->first()
        );
    }

    /**
     * Get all tags with caching
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
     * Clear all cache for this tag
     */
    public function clearCache(): void
    {
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . $this->id);
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'slug:' . $this->slug);
        Cache::tags([self::CACHE_TAG])->forget(self::CACHE_PREFIX . 'all');
    }

    /**
     * Clear all tag cache (static)
     */
    public static function clearAllCache(): void
    {
        Cache::tags([self::CACHE_TAG])->flush();
    }

    // Relationships
    public function contacts(): BelongsToMany
    {
        return $this->belongsToMany(Contact::class, 'contact_tag_pivot', 'tag_id', 'contact_id')
            ->withPivot(['applied_at', 'applied_by'])
            ->withTimestamps();
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    // Business Logic
    public function applyToContact(Contact $contact, ?string $appliedBy = null): bool
    {
        if ($this->contacts()->where('contacts.id', $contact->id)->exists()) {
            return false;
        }

        $this->contacts()->attach($contact->id, [
            'applied_at' => now(),
            'applied_by' => $appliedBy,
        ]);

        $this->increment('contacts_count');

        // Trigger tag applied event for automations
        event('crm.tag_applied', [
            'contact' => $contact,
            'tag' => $this,
        ]);

        return true;
    }

    public function removeFromContact(Contact $contact): bool
    {
        if (!$this->contacts()->where('contacts.id', $contact->id)->exists()) {
            return false;
        }

        $this->contacts()->detach($contact->id);
        $this->decrement('contacts_count');

        // Trigger tag removed event for automations
        event('crm.tag_removed', [
            'contact' => $contact,
            'tag' => $this,
        ]);

        return true;
    }

    public function hasContact(Contact $contact): bool
    {
        return $this->contacts()->where('contacts.id', $contact->id)->exists();
    }

    public function recalculateCount(): void
    {
        $this->update([
            'contacts_count' => $this->contacts()->count(),
        ]);
    }
}
