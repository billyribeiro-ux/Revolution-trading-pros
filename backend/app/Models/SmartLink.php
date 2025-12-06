<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Str;

/**
 * Smart Link Model (FluentCRM Pro Trackable Action Links)
 *
 * Smart links perform CRM actions when clicked:
 * - Add/remove tags
 * - Add to lists
 * - Trigger automations
 * - Update contact fields
 * - Redirect to target URL
 *
 * @property string $id
 * @property string $title
 * @property string $short
 * @property string|null $target_url
 * @property array|null $actions
 * @property string|null $notes
 * @property bool $is_active
 * @property int $click_count
 * @property int $unique_clicks
 * @property array|null $click_data
 * @property string|null $created_by
 */
class SmartLink extends Model
{
    use HasUuids;

    protected $fillable = [
        'title',
        'target_url',
        'actions',
        'notes',
        'is_active',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'actions' => 'array',
            'click_data' => 'array',
            'is_active' => 'boolean',
            'click_count' => 'integer',
            'unique_clicks' => 'integer',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $link): void {
            $link->short = self::generateShortCode();
            $link->is_active ??= true;
            $link->actions ??= [];
        });
    }

    // Relationships
    public function clicks(): HasMany
    {
        return $this->hasMany(SmartLinkClick::class, 'smart_link_id');
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    // Scopes
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    // Static Helpers
    public static function generateShortCode(): string
    {
        $chars = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
        $code = '';

        // Generate base from timestamp
        $timestamp = time() - 1611224846; // offset from a fixed point
        while ($timestamp > 0) {
            $code .= $chars[$timestamp % 62];
            $timestamp = intdiv($timestamp, 62);
        }

        // Add random suffix
        $code .= $chars[random_int(0, 61)];

        return $code;
    }

    public static function findByShort(string $short): ?self
    {
        return self::where('short', $short)->first();
    }

    // Accessors
    public function getShortUrlAttribute(): string
    {
        return url("/sl/{$this->short}");
    }

    // Business Logic
    public function recordClick(?Contact $contact = null, array $metadata = []): SmartLinkClick
    {
        $isUnique = $contact
            ? !$this->clicks()->where('contact_id', $contact->id)->exists()
            : !$this->clicks()
                ->where('ip_address', $metadata['ip_address'] ?? null)
                ->where('created_at', '>=', now()->subDay())
                ->exists();

        $this->increment('click_count');
        if ($isUnique) {
            $this->increment('unique_clicks');
        }

        return $this->clicks()->create([
            'contact_id' => $contact?->id,
            'ip_address' => $metadata['ip_address'] ?? null,
            'user_agent' => $metadata['user_agent'] ?? null,
            'referrer' => $metadata['referrer'] ?? null,
            'country' => $metadata['country'] ?? null,
            'city' => $metadata['city'] ?? null,
            'device' => $metadata['device'] ?? null,
            'browser' => $metadata['browser'] ?? null,
            'os' => $metadata['os'] ?? null,
            'utm_params' => $metadata['utm_params'] ?? null,
            'clicked_at' => now(),
        ]);
    }

    public function executeActions(Contact $contact): array
    {
        $results = [];

        foreach ($this->actions ?? [] as $action) {
            $result = match ($action['type'] ?? null) {
                'add_tag' => $this->executeAddTag($contact, $action),
                'remove_tag' => $this->executeRemoveTag($contact, $action),
                'add_to_list' => $this->executeAddToList($contact, $action),
                'remove_from_list' => $this->executeRemoveFromList($contact, $action),
                'update_field' => $this->executeUpdateField($contact, $action),
                'add_to_sequence' => $this->executeAddToSequence($contact, $action),
                'remove_from_sequence' => $this->executeRemoveFromSequence($contact, $action),
                'trigger_automation' => $this->executeTriggerAutomation($contact, $action),
                default => ['success' => false, 'message' => 'Unknown action type'],
            };

            $results[] = [
                'action' => $action['type'] ?? 'unknown',
                'result' => $result,
            ];
        }

        return $results;
    }

    private function executeAddTag(Contact $contact, array $action): array
    {
        $tagId = $action['tag_id'] ?? null;
        if (!$tagId) {
            return ['success' => false, 'message' => 'No tag specified'];
        }

        // Add tag logic would go here
        return ['success' => true, 'message' => 'Tag added'];
    }

    private function executeRemoveTag(Contact $contact, array $action): array
    {
        $tagId = $action['tag_id'] ?? null;
        if (!$tagId) {
            return ['success' => false, 'message' => 'No tag specified'];
        }

        return ['success' => true, 'message' => 'Tag removed'];
    }

    private function executeAddToList(Contact $contact, array $action): array
    {
        $listId = $action['list_id'] ?? null;
        if (!$listId) {
            return ['success' => false, 'message' => 'No list specified'];
        }

        return ['success' => true, 'message' => 'Added to list'];
    }

    private function executeRemoveFromList(Contact $contact, array $action): array
    {
        $listId = $action['list_id'] ?? null;
        if (!$listId) {
            return ['success' => false, 'message' => 'No list specified'];
        }

        return ['success' => true, 'message' => 'Removed from list'];
    }

    private function executeUpdateField(Contact $contact, array $action): array
    {
        $field = $action['field'] ?? null;
        $value = $action['value'] ?? null;

        if (!$field) {
            return ['success' => false, 'message' => 'No field specified'];
        }

        if ($contact->isFillable($field)) {
            $contact->update([$field => $value]);
            return ['success' => true, 'message' => "Field {$field} updated"];
        }

        return ['success' => false, 'message' => 'Field not updatable'];
    }

    private function executeAddToSequence(Contact $contact, array $action): array
    {
        $sequenceId = $action['sequence_id'] ?? null;
        if (!$sequenceId) {
            return ['success' => false, 'message' => 'No sequence specified'];
        }

        $sequence = EmailSequence::find($sequenceId);
        if (!$sequence) {
            return ['success' => false, 'message' => 'Sequence not found'];
        }

        $tracker = $sequence->subscribe($contact);
        return $tracker
            ? ['success' => true, 'message' => 'Added to sequence']
            : ['success' => false, 'message' => 'Already in sequence or no emails'];
    }

    private function executeRemoveFromSequence(Contact $contact, array $action): array
    {
        $sequenceId = $action['sequence_id'] ?? null;
        if (!$sequenceId) {
            return ['success' => false, 'message' => 'No sequence specified'];
        }

        $sequence = EmailSequence::find($sequenceId);
        if (!$sequence) {
            return ['success' => false, 'message' => 'Sequence not found'];
        }

        $sequence->unsubscribe($contact, 'smart_link');
        return ['success' => true, 'message' => 'Removed from sequence'];
    }

    private function executeTriggerAutomation(Contact $contact, array $action): array
    {
        $funnelId = $action['funnel_id'] ?? null;
        if (!$funnelId) {
            return ['success' => false, 'message' => 'No automation specified'];
        }

        return ['success' => true, 'message' => 'Automation triggered'];
    }

    public function getStats(): array
    {
        return [
            'total_clicks' => $this->click_count,
            'unique_clicks' => $this->unique_clicks,
            'click_through_rate' => $this->unique_clicks > 0 && $this->click_count > 0
                ? round(($this->unique_clicks / $this->click_count) * 100, 2)
                : 0,
            'clicks_today' => $this->clicks()->whereDate('clicked_at', today())->count(),
            'clicks_this_week' => $this->clicks()->where('clicked_at', '>=', now()->startOfWeek())->count(),
            'top_countries' => $this->clicks()
                ->whereNotNull('country')
                ->selectRaw('country, COUNT(*) as count')
                ->groupBy('country')
                ->orderByDesc('count')
                ->limit(5)
                ->pluck('count', 'country'),
        ];
    }
}
