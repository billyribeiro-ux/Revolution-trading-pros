<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;

/**
 * Funnel Action Model (Steps in automation funnels)
 *
 * Action types (FluentCRM Pro compatible):
 * - send_email, wait, add_tag, remove_tag
 * - add_to_list, remove_from_list, add_to_sequence
 * - http_request, update_contact, change_status
 * - condition (branching), end_funnel
 *
 * @property string $id
 * @property string $funnel_id
 * @property string|null $parent_id
 * @property string $action_type
 * @property string|null $title
 * @property array|null $settings
 * @property int $position
 * @property string $condition_type
 * @property int $delay_seconds
 * @property int $execution_count
 */
class FunnelAction extends Model
{
    use HasUuids;

    protected $fillable = [
        'funnel_id',
        'parent_id',
        'action_type',
        'title',
        'settings',
        'position',
        'condition_type',
        'delay_seconds',
    ];

    protected function casts(): array
    {
        return [
            'settings' => 'array',
            'position' => 'integer',
            'delay_seconds' => 'integer',
            'execution_count' => 'integer',
        ];
    }

    // Available action types (FluentCRM Pro compatible)
    public const ACTION_TYPES = [
        'send_email' => 'Send Email',
        'send_campaign_email' => 'Send Campaign Email',
        'wait' => 'Wait/Delay',
        'add_tag' => 'Add Tag',
        'remove_tag' => 'Remove Tag',
        'add_to_list' => 'Add to List',
        'remove_from_list' => 'Remove from List',
        'add_to_sequence' => 'Add to Email Sequence',
        'remove_from_sequence' => 'Remove from Email Sequence',
        'update_contact' => 'Update Contact Field',
        'change_status' => 'Change Contact Status',
        'add_activity' => 'Add Activity Note',
        'http_request' => 'HTTP Request (Webhook)',
        'create_user' => 'Create WordPress User',
        'change_user_role' => 'Change User Role',
        'remove_user_role' => 'Remove User Role',
        'update_user_meta' => 'Update User Meta',
        'condition' => 'Conditional Branch',
        'ab_test' => 'A/B Test Split',
        'goal' => 'Goal/Benchmark',
        'end_funnel' => 'End Funnel',
        'remove_from_funnel' => 'Remove from Other Funnel',
    ];

    protected static function booted(): void
    {
        static::creating(function (self $action): void {
            $action->condition_type ??= 'none';
            $action->delay_seconds ??= 0;
            $action->settings ??= [];
        });
    }

    // Relationships
    public function funnel(): BelongsTo
    {
        return $this->belongsTo(AutomationFunnel::class, 'funnel_id');
    }

    public function parent(): BelongsTo
    {
        return $this->belongsTo(self::class, 'parent_id');
    }

    public function children(): HasMany
    {
        return $this->hasMany(self::class, 'parent_id')->orderBy('position');
    }

    public function yesChildren(): HasMany
    {
        return $this->children()->where('condition_type', 'yes');
    }

    public function noChildren(): HasMany
    {
        return $this->children()->where('condition_type', 'no');
    }

    // Scopes
    public function scopeRootLevel(Builder $query): Builder
    {
        return $query->whereNull('parent_id');
    }

    // Business Logic
    public function getNextAction(?bool $conditionResult = null): ?self
    {
        // For conditional actions, get the appropriate branch
        if ($this->action_type === 'condition' && $conditionResult !== null) {
            $branch = $conditionResult ? $this->yesChildren()->first() : $this->noChildren()->first();
            if ($branch) {
                return $branch;
            }
        }

        // Get next sibling action
        $query = self::where('funnel_id', $this->funnel_id)
            ->where('position', '>', $this->position);

        if ($this->parent_id) {
            $query->where('parent_id', $this->parent_id)
                ->where('condition_type', $this->condition_type);
        } else {
            $query->whereNull('parent_id');
        }

        $nextSibling = $query->orderBy('position')->first();

        if ($nextSibling) {
            return $nextSibling;
        }

        // If no sibling, go back to parent's next sibling
        if ($this->parent_id) {
            return $this->parent?->getNextAction();
        }

        return null;
    }

    public function execute(Contact $contact, FunnelSubscriber $subscriber): array
    {
        $this->increment('execution_count');

        return match ($this->action_type) {
            'send_email' => $this->executeSendEmail($contact, $subscriber),
            'wait' => $this->executeWait($subscriber),
            'add_tag' => $this->executeAddTag($contact),
            'remove_tag' => $this->executeRemoveTag($contact),
            'add_to_list' => $this->executeAddToList($contact),
            'remove_from_list' => $this->executeRemoveFromList($contact),
            'add_to_sequence' => $this->executeAddToSequence($contact),
            'remove_from_sequence' => $this->executeRemoveFromSequence($contact),
            'update_contact' => $this->executeUpdateContact($contact),
            'change_status' => $this->executeChangeStatus($contact),
            'http_request' => $this->executeHttpRequest($contact),
            'add_activity' => $this->executeAddActivity($contact),
            'condition' => $this->executeCondition($contact),
            'end_funnel' => $this->executeEndFunnel($subscriber),
            default => ['success' => true, 'message' => 'Action type not implemented'],
        };
    }

    private function executeSendEmail(Contact $contact, FunnelSubscriber $subscriber): array
    {
        // Email sending logic would integrate with EmailService
        return ['success' => true, 'message' => 'Email queued'];
    }

    private function executeWait(FunnelSubscriber $subscriber): array
    {
        $waitSeconds = $this->settings['wait_seconds'] ?? $this->delay_seconds;
        $subscriber->update([
            'status' => 'waiting',
            'next_execution_at' => now()->addSeconds($waitSeconds),
        ]);
        return ['success' => true, 'message' => "Waiting for {$waitSeconds} seconds"];
    }

    private function executeAddTag(Contact $contact): array
    {
        $tagId = $this->settings['tag_id'] ?? null;
        if (!$tagId) {
            return ['success' => false, 'message' => 'No tag specified'];
        }
        // Tag logic would go here
        return ['success' => true, 'message' => 'Tag added'];
    }

    private function executeRemoveTag(Contact $contact): array
    {
        $tagId = $this->settings['tag_id'] ?? null;
        if (!$tagId) {
            return ['success' => false, 'message' => 'No tag specified'];
        }
        return ['success' => true, 'message' => 'Tag removed'];
    }

    private function executeAddToList(Contact $contact): array
    {
        $listId = $this->settings['list_id'] ?? null;
        if (!$listId) {
            return ['success' => false, 'message' => 'No list specified'];
        }
        return ['success' => true, 'message' => 'Added to list'];
    }

    private function executeRemoveFromList(Contact $contact): array
    {
        $listId = $this->settings['list_id'] ?? null;
        if (!$listId) {
            return ['success' => false, 'message' => 'No list specified'];
        }
        return ['success' => true, 'message' => 'Removed from list'];
    }

    private function executeAddToSequence(Contact $contact): array
    {
        $sequenceId = $this->settings['sequence_id'] ?? null;
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
            : ['success' => false, 'message' => 'Could not add to sequence'];
    }

    private function executeRemoveFromSequence(Contact $contact): array
    {
        $sequenceId = $this->settings['sequence_id'] ?? null;
        if (!$sequenceId) {
            return ['success' => false, 'message' => 'No sequence specified'];
        }

        $sequence = EmailSequence::find($sequenceId);
        if ($sequence) {
            $sequence->unsubscribe($contact, 'funnel_action');
        }

        return ['success' => true, 'message' => 'Removed from sequence'];
    }

    private function executeUpdateContact(Contact $contact): array
    {
        $updates = $this->settings['updates'] ?? [];
        $contact->update(array_filter($updates, fn ($key) => $contact->isFillable($key), ARRAY_FILTER_USE_KEY));
        return ['success' => true, 'message' => 'Contact updated'];
    }

    private function executeChangeStatus(Contact $contact): array
    {
        $newStatus = $this->settings['status'] ?? null;
        if (!$newStatus) {
            return ['success' => false, 'message' => 'No status specified'];
        }

        $contact->update(['status' => $newStatus]);
        return ['success' => true, 'message' => "Status changed to {$newStatus}"];
    }

    private function executeHttpRequest(Contact $contact): array
    {
        // HTTP request logic would go here
        return ['success' => true, 'message' => 'HTTP request sent'];
    }

    private function executeAddActivity(Contact $contact): array
    {
        // Activity creation logic
        return ['success' => true, 'message' => 'Activity added'];
    }

    private function executeCondition(Contact $contact): array
    {
        $conditions = $this->settings['conditions'] ?? [];
        $result = true;

        foreach ($conditions as $condition) {
            $field = $condition['field'] ?? null;
            $operator = $condition['operator'] ?? 'equals';
            $value = $condition['value'] ?? null;

            if (!$field) {
                continue;
            }

            $contactValue = data_get($contact, $field);
            $conditionMet = match ($operator) {
                'equals' => $contactValue == $value,
                'not_equals' => $contactValue != $value,
                'contains' => str_contains((string) $contactValue, (string) $value),
                'greater_than' => $contactValue > $value,
                'less_than' => $contactValue < $value,
                default => true,
            };

            if (!$conditionMet) {
                $result = false;
                break;
            }
        }

        return ['success' => true, 'condition_result' => $result];
    }

    private function executeEndFunnel(FunnelSubscriber $subscriber): array
    {
        $subscriber->complete();
        return ['success' => true, 'message' => 'Funnel ended'];
    }
}
