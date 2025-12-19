<?php

declare(strict_types=1);

namespace App\Services\Forms;

use App\Models\FormSubmission;
use App\Models\FormSubmissionApprovalLog;
use App\Models\FormApprovalSettings;
use App\Models\User;
use App\Services\EmailService;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Collection;

/**
 * FormSubmissionApprovalService - FluentForms 6.1.8 (December 2025)
 *
 * Enterprise-grade admin approval workflow for form submissions.
 * Features:
 * - Multi-status approval workflow (pending, approved, rejected, needs_revision, on_hold)
 * - Conditional approval based on submission data
 * - Email notifications to approvers and submitters
 * - Audit trail logging
 * - Auto-approval after timeout
 * - Bulk operations support
 */
class FormSubmissionApprovalService
{
    public function __construct(
        private readonly EmailService $emailService
    ) {}

    /**
     * Check if submission requires approval and set initial status
     */
    public function initializeApproval(FormSubmission $submission, array $submissionData): void
    {
        $settings = FormApprovalSettings::where('form_id', $submission->form_id)->first();

        if (!$settings || !$settings->approval_required) {
            // Check conditional rules
            if ($settings && $settings->evaluateConditionalApproval($submissionData)) {
                $this->setApprovalRequired($submission, $settings);
            }
            return;
        }

        $this->setApprovalRequired($submission, $settings);
    }

    /**
     * Set approval required status
     */
    private function setApprovalRequired(FormSubmission $submission, FormApprovalSettings $settings): void
    {
        $submission->update([
            'approval_status' => FormSubmissionApprovalLog::STATUS_PENDING,
            'approval_required_at' => now(),
        ]);

        // Notify approvers
        if ($settings->notify_approvers) {
            $this->notifyApprovers($submission, $settings);
        }

        Log::info('FormSubmissionApprovalService: Approval required', [
            'submission_id' => $submission->id,
            'form_id' => $submission->form_id,
        ]);
    }

    /**
     * Approve a submission
     */
    public function approve(FormSubmission $submission, User $admin, ?string $note = null): array
    {
        if (!$this->canApprove($submission, $admin)) {
            return [
                'success' => false,
                'message' => 'You do not have permission to approve this submission.',
            ];
        }

        $previousStatus = $submission->approval_status;

        DB::transaction(function () use ($submission, $admin, $note, $previousStatus) {
            $submission->update([
                'approval_status' => FormSubmissionApprovalLog::STATUS_APPROVED,
                'approved_at' => now(),
                'approved_by' => $admin->id,
                'approval_note' => $note,
            ]);

            FormSubmissionApprovalLog::logAction(
                $submission->id,
                $admin->id,
                FormSubmissionApprovalLog::STATUS_APPROVED,
                $previousStatus,
                $note
            );
        });

        // Notify submitter
        $this->notifySubmitterOfDecision($submission, 'approved');

        Log::info('FormSubmissionApprovalService: Submission approved', [
            'submission_id' => $submission->id,
            'admin_id' => $admin->id,
        ]);

        return [
            'success' => true,
            'message' => 'Submission has been approved.',
            'status' => FormSubmissionApprovalLog::STATUS_APPROVED,
        ];
    }

    /**
     * Reject a submission
     */
    public function reject(FormSubmission $submission, User $admin, ?string $note = null): array
    {
        if (!$this->canApprove($submission, $admin)) {
            return [
                'success' => false,
                'message' => 'You do not have permission to reject this submission.',
            ];
        }

        $previousStatus = $submission->approval_status;

        DB::transaction(function () use ($submission, $admin, $note, $previousStatus) {
            $submission->update([
                'approval_status' => FormSubmissionApprovalLog::STATUS_REJECTED,
                'approved_at' => now(),
                'approved_by' => $admin->id,
                'approval_note' => $note,
            ]);

            FormSubmissionApprovalLog::logAction(
                $submission->id,
                $admin->id,
                FormSubmissionApprovalLog::STATUS_REJECTED,
                $previousStatus,
                $note
            );
        });

        // Notify submitter
        $this->notifySubmitterOfDecision($submission, 'rejected');

        Log::info('FormSubmissionApprovalService: Submission rejected', [
            'submission_id' => $submission->id,
            'admin_id' => $admin->id,
        ]);

        return [
            'success' => true,
            'message' => 'Submission has been rejected.',
            'status' => FormSubmissionApprovalLog::STATUS_REJECTED,
        ];
    }

    /**
     * Request revision from submitter
     */
    public function requestRevision(FormSubmission $submission, User $admin, ?string $note = null): array
    {
        if (!$this->canApprove($submission, $admin)) {
            return [
                'success' => false,
                'message' => 'You do not have permission to request revision.',
            ];
        }

        $previousStatus = $submission->approval_status;

        DB::transaction(function () use ($submission, $admin, $note, $previousStatus) {
            $submission->update([
                'approval_status' => FormSubmissionApprovalLog::STATUS_NEEDS_REVISION,
                'approval_note' => $note,
            ]);

            FormSubmissionApprovalLog::logAction(
                $submission->id,
                $admin->id,
                FormSubmissionApprovalLog::STATUS_NEEDS_REVISION,
                $previousStatus,
                $note
            );
        });

        // Notify submitter
        $this->notifySubmitterOfDecision($submission, 'needs_revision');

        Log::info('FormSubmissionApprovalService: Revision requested', [
            'submission_id' => $submission->id,
            'admin_id' => $admin->id,
        ]);

        return [
            'success' => true,
            'message' => 'Revision has been requested.',
            'status' => FormSubmissionApprovalLog::STATUS_NEEDS_REVISION,
        ];
    }

    /**
     * Put submission on hold
     */
    public function putOnHold(FormSubmission $submission, User $admin, ?string $note = null): array
    {
        if (!$this->canApprove($submission, $admin)) {
            return [
                'success' => false,
                'message' => 'You do not have permission to put this submission on hold.',
            ];
        }

        $previousStatus = $submission->approval_status;

        DB::transaction(function () use ($submission, $admin, $note, $previousStatus) {
            $submission->update([
                'approval_status' => FormSubmissionApprovalLog::STATUS_ON_HOLD,
                'approval_note' => $note,
            ]);

            FormSubmissionApprovalLog::logAction(
                $submission->id,
                $admin->id,
                FormSubmissionApprovalLog::STATUS_ON_HOLD,
                $previousStatus,
                $note
            );
        });

        Log::info('FormSubmissionApprovalService: Submission put on hold', [
            'submission_id' => $submission->id,
            'admin_id' => $admin->id,
        ]);

        return [
            'success' => true,
            'message' => 'Submission has been put on hold.',
            'status' => FormSubmissionApprovalLog::STATUS_ON_HOLD,
        ];
    }

    /**
     * Reset approval status to pending
     */
    public function resetToPending(FormSubmission $submission, User $admin, ?string $note = null): array
    {
        if (!$this->canApprove($submission, $admin)) {
            return [
                'success' => false,
                'message' => 'You do not have permission to reset this submission.',
            ];
        }

        $previousStatus = $submission->approval_status;

        DB::transaction(function () use ($submission, $admin, $note, $previousStatus) {
            $submission->update([
                'approval_status' => FormSubmissionApprovalLog::STATUS_PENDING,
                'approved_at' => null,
                'approved_by' => null,
                'approval_note' => $note,
            ]);

            FormSubmissionApprovalLog::logAction(
                $submission->id,
                $admin->id,
                FormSubmissionApprovalLog::STATUS_PENDING,
                $previousStatus,
                $note
            );
        });

        return [
            'success' => true,
            'message' => 'Submission has been reset to pending.',
            'status' => FormSubmissionApprovalLog::STATUS_PENDING,
        ];
    }

    /**
     * Get approval history for a submission
     */
    public function getApprovalHistory(FormSubmission $submission): Collection
    {
        return FormSubmissionApprovalLog::forSubmission($submission->id)
            ->with('admin:id,name,email')
            ->orderBy('created_at', 'desc')
            ->get()
            ->map(function ($log) {
                return [
                    'id' => (string) $log->id,
                    'status' => $log->status,
                    'previous_status' => $log->previous_status,
                    'note' => $log->note,
                    'admin_name' => $log->admin_name ?? $log->admin?->name ?? 'Unknown',
                    'admin_email' => $log->admin_email ?? $log->admin?->email,
                    'created_at' => $log->created_at->toIso8601String(),
                ];
            });
    }

    /**
     * Get pending submissions for a form
     */
    public function getPendingSubmissions(int $formId): Collection
    {
        return FormSubmission::where('form_id', $formId)
            ->where('approval_status', FormSubmissionApprovalLog::STATUS_PENDING)
            ->with(['data', 'user:id,name,email'])
            ->orderBy('created_at', 'asc')
            ->get();
    }

    /**
     * Get submissions by approval status
     */
    public function getSubmissionsByStatus(int $formId, string $status): Collection
    {
        return FormSubmission::where('form_id', $formId)
            ->where('approval_status', $status)
            ->with(['data', 'user:id,name,email'])
            ->orderBy('created_at', 'desc')
            ->get();
    }

    /**
     * Bulk approve submissions
     */
    public function bulkApprove(array $submissionIds, User $admin, ?string $note = null): array
    {
        $results = ['approved' => 0, 'failed' => 0, 'errors' => []];

        foreach ($submissionIds as $id) {
            $submission = FormSubmission::find($id);
            if (!$submission) {
                $results['failed']++;
                $results['errors'][] = "Submission {$id} not found";
                continue;
            }

            $result = $this->approve($submission, $admin, $note);
            if ($result['success']) {
                $results['approved']++;
            } else {
                $results['failed']++;
                $results['errors'][] = $result['message'];
            }
        }

        return $results;
    }

    /**
     * Bulk reject submissions
     */
    public function bulkReject(array $submissionIds, User $admin, ?string $note = null): array
    {
        $results = ['rejected' => 0, 'failed' => 0, 'errors' => []];

        foreach ($submissionIds as $id) {
            $submission = FormSubmission::find($id);
            if (!$submission) {
                $results['failed']++;
                $results['errors'][] = "Submission {$id} not found";
                continue;
            }

            $result = $this->reject($submission, $admin, $note);
            if ($result['success']) {
                $results['rejected']++;
            } else {
                $results['failed']++;
                $results['errors'][] = $result['message'];
            }
        }

        return $results;
    }

    /**
     * Process auto-approvals for forms with timeout
     */
    public function processAutoApprovals(): int
    {
        $count = 0;
        $settings = FormApprovalSettings::whereNotNull('auto_approve_after_hours')
            ->where('auto_approve_after_hours', '>', 0)
            ->get();

        foreach ($settings as $setting) {
            $cutoff = now()->subHours($setting->auto_approve_after_hours);

            $submissions = FormSubmission::where('form_id', $setting->form_id)
                ->where('approval_status', FormSubmissionApprovalLog::STATUS_PENDING)
                ->where('approval_required_at', '<=', $cutoff)
                ->get();

            foreach ($submissions as $submission) {
                $submission->update([
                    'approval_status' => FormSubmissionApprovalLog::STATUS_APPROVED,
                    'approved_at' => now(),
                    'approval_note' => 'Auto-approved after timeout',
                ]);

                FormSubmissionApprovalLog::create([
                    'submission_id' => $submission->id,
                    'admin_id' => 0, // System
                    'status' => FormSubmissionApprovalLog::STATUS_APPROVED,
                    'previous_status' => FormSubmissionApprovalLog::STATUS_PENDING,
                    'note' => "Auto-approved after {$setting->auto_approve_after_hours} hours",
                    'admin_name' => 'System',
                ]);

                $count++;
            }
        }

        if ($count > 0) {
            Log::info("FormSubmissionApprovalService: Auto-approved {$count} submissions");
        }

        return $count;
    }

    /**
     * Check if user can approve submissions
     */
    private function canApprove(FormSubmission $submission, User $user): bool
    {
        // Super admins can always approve
        if ($user->hasRole('super-admin') || $user->hasRole('admin')) {
            return true;
        }

        // Check form-specific approvers
        $settings = FormApprovalSettings::where('form_id', $submission->form_id)->first();
        if ($settings && $settings->isApprover($user->id)) {
            return true;
        }

        return false;
    }

    /**
     * Notify approvers of new pending submission
     */
    private function notifyApprovers(FormSubmission $submission, FormApprovalSettings $settings): void
    {
        $approvers = $settings->approvers();
        if ($approvers->isEmpty()) {
            return;
        }

        $form = $submission->form;
        $reviewUrl = config('app.url') . "/admin/forms/{$form->id}/submissions/{$submission->id}";

        foreach ($approvers as $approver) {
            try {
                $this->emailService->send([
                    'to' => $approver->email,
                    'subject' => "New submission requires approval: {$form->name}",
                    'template' => $settings->approval_email_template ?? 'emails.forms.approval-required',
                    'data' => [
                        'approver_name' => $approver->name,
                        'form_name' => $form->name,
                        'submission_id' => $submission->submission_id,
                        'review_url' => $reviewUrl,
                        'submitted_at' => $submission->created_at->format('M d, Y \a\t g:i A'),
                    ],
                ]);
            } catch (\Exception $e) {
                Log::warning('Failed to notify approver', [
                    'approver_id' => $approver->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Notify submitter of approval decision
     */
    private function notifySubmitterOfDecision(FormSubmission $submission, string $decision): void
    {
        $settings = FormApprovalSettings::where('form_id', $submission->form_id)->first();

        if (!$settings || !$settings->notify_submitter) {
            return;
        }

        // Get submitter email from submission data
        $submitterEmail = $submission->user?->email ?? $submission->getFieldValue('email');
        if (!$submitterEmail) {
            return;
        }

        $form = $submission->form;
        $template = match ($decision) {
            'approved' => $settings->approval_email_template ?? 'emails.forms.submission-approved',
            'rejected' => $settings->rejection_email_template ?? 'emails.forms.submission-rejected',
            'needs_revision' => $settings->revision_email_template ?? 'emails.forms.submission-revision',
            default => null,
        };

        if (!$template) {
            return;
        }

        $subject = match ($decision) {
            'approved' => "Your submission has been approved: {$form->name}",
            'rejected' => "Your submission has been declined: {$form->name}",
            'needs_revision' => "Your submission needs revision: {$form->name}",
            default => "Update on your submission: {$form->name}",
        };

        try {
            $this->emailService->send([
                'to' => $submitterEmail,
                'subject' => $subject,
                'template' => $template,
                'data' => [
                    'form_name' => $form->name,
                    'submission_id' => $submission->submission_id,
                    'decision' => $decision,
                    'note' => $submission->approval_note,
                    'decided_at' => $submission->approved_at?->format('M d, Y \a\t g:i A'),
                ],
            ]);
        } catch (\Exception $e) {
            Log::warning('Failed to notify submitter', [
                'submission_id' => $submission->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Get approval statistics for a form
     */
    public function getApprovalStats(int $formId): array
    {
        $stats = FormSubmission::where('form_id', $formId)
            ->selectRaw('approval_status, COUNT(*) as count')
            ->groupBy('approval_status')
            ->pluck('count', 'approval_status')
            ->toArray();

        return [
            'none' => $stats['none'] ?? 0,
            'pending' => $stats[FormSubmissionApprovalLog::STATUS_PENDING] ?? 0,
            'approved' => $stats[FormSubmissionApprovalLog::STATUS_APPROVED] ?? 0,
            'rejected' => $stats[FormSubmissionApprovalLog::STATUS_REJECTED] ?? 0,
            'needs_revision' => $stats[FormSubmissionApprovalLog::STATUS_NEEDS_REVISION] ?? 0,
            'on_hold' => $stats[FormSubmissionApprovalLog::STATUS_ON_HOLD] ?? 0,
        ];
    }
}
