<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

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
