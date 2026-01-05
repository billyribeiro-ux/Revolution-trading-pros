<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;

/**
 * FormApprovalSettings Model - FluentForms 6.1.8 (December 2025)
 *
 * Configuration for form-level approval workflow settings.
 * Defines who can approve, notification preferences, and auto-approval rules.
 *
 * @property int $id
 * @property int $form_id
 * @property bool $approval_required
 * @property array|null $approver_ids
 * @property array|null $approval_rules
 * @property bool $notify_approvers
 * @property bool $notify_submitter
 * @property string|null $approval_email_template
 * @property string|null $rejection_email_template
 * @property string|null $revision_email_template
 * @property int|null $auto_approve_after_hours
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 *
 * @property-read Form $form
 */
class FormApprovalSettings extends Model
{
    use HasFactory;

    protected $table = 'form_approval_settings';

    protected $fillable = [
        'form_id',
        'approval_required',
        'approver_ids',
        'approval_rules',
        'notify_approvers',
        'notify_submitter',
        'approval_email_template',
        'rejection_email_template',
        'revision_email_template',
        'auto_approve_after_hours',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'approval_required' => 'boolean',
        'approver_ids' => 'array',
        'approval_rules' => 'array',
        'notify_approvers' => 'boolean',
        'notify_submitter' => 'boolean',
        'auto_approve_after_hours' => 'integer',
    ];

    protected $attributes = [
        'approval_required' => false,
        'notify_approvers' => true,
        'notify_submitter' => true,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    public function approvers()
    {
        if (empty($this->approver_ids)) {
            return collect();
        }

        return User::whereIn('id', $this->approver_ids)->get();
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    public function isApprover(int $userId): bool
    {
        if (empty($this->approver_ids)) {
            return false;
        }

        return in_array($userId, $this->approver_ids, true);
    }

    public function addApprover(int $userId): void
    {
        $approvers = $this->approver_ids ?? [];
        if (!in_array($userId, $approvers, true)) {
            $approvers[] = $userId;
            $this->approver_ids = $approvers;
            $this->save();
        }
    }

    public function removeApprover(int $userId): void
    {
        $approvers = $this->approver_ids ?? [];
        $approvers = array_values(array_filter($approvers, fn($id) => $id !== $userId));
        $this->approver_ids = $approvers;
        $this->save();
    }

    public function shouldAutoApprove(): bool
    {
        return $this->auto_approve_after_hours !== null && $this->auto_approve_after_hours > 0;
    }

    public function evaluateConditionalApproval(array $submissionData): bool
    {
        if (empty($this->approval_rules)) {
            return $this->approval_required;
        }

        // Evaluate rules to determine if approval is required
        foreach ($this->approval_rules as $rule) {
            $field = $rule['field'] ?? null;
            $operator = $rule['operator'] ?? 'equals';
            $value = $rule['value'] ?? null;
            $requiresApproval = $rule['requires_approval'] ?? true;

            if (!$field || !isset($submissionData[$field])) {
                continue;
            }

            $fieldValue = $submissionData[$field];
            $matches = $this->evaluateCondition($fieldValue, $operator, $value);

            if ($matches && $requiresApproval) {
                return true;
            }
        }

        return $this->approval_required;
    }

    private function evaluateCondition(mixed $fieldValue, string $operator, mixed $compareValue): bool
    {
        return match ($operator) {
            'equals' => $fieldValue == $compareValue,
            'not_equals' => $fieldValue != $compareValue,
            'contains' => str_contains((string) $fieldValue, (string) $compareValue),
            'not_contains' => !str_contains((string) $fieldValue, (string) $compareValue),
            'greater_than' => (float) $fieldValue > (float) $compareValue,
            'less_than' => (float) $fieldValue < (float) $compareValue,
            'in' => in_array($fieldValue, (array) $compareValue),
            'not_in' => !in_array($fieldValue, (array) $compareValue),
            'is_empty' => empty($fieldValue),
            'is_not_empty' => !empty($fieldValue),
            default => false,
        };
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    public static function getOrCreate(int $formId): self
    {
        return self::firstOrCreate(
            ['form_id' => $formId],
            [
                'approval_required' => false,
                'notify_approvers' => true,
                'notify_submitter' => true,
            ]
        );
    }
}
