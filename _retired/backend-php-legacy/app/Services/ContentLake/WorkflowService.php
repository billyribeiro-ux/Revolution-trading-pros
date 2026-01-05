<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Content Workflow Service
 *
 * Provides comprehensive workflow and approval features:
 * - Configurable workflow stages
 * - Role-based approval chains
 * - Automatic transitions
 * - Deadline management
 * - Notifications
 * - Audit trail
 * - Parallel and sequential approvals
 * - Conditional workflows
 */
class WorkflowService
{
    private const CACHE_TTL = 3600;

    /**
     * Get or create workflow instance for a document
     */
    public function getWorkflow(string $documentId, string $documentType = 'post'): array
    {
        $workflow = DB::table('content_workflows')
            ->where('document_id', $documentId)
            ->first();

        if (!$workflow) {
            // Create default workflow
            return $this->createWorkflow($documentId, $documentType);
        }

        return $this->formatWorkflow($workflow);
    }

    /**
     * Create a new workflow instance
     */
    public function createWorkflow(string $documentId, string $documentType, ?string $templateId = null): array
    {
        $template = $templateId
            ? $this->getWorkflowTemplate($templateId)
            : $this->getDefaultTemplate($documentType);

        $workflowId = Str::uuid()->toString();

        DB::table('content_workflows')->insert([
            'id' => $workflowId,
            'document_id' => $documentId,
            'document_type' => $documentType,
            'template_id' => $template['id'],
            'current_stage' => $template['stages'][0]['id'],
            'status' => 'active',
            'started_at' => now(),
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        // Initialize first stage
        $this->initializeStage($workflowId, $template['stages'][0], $documentId);

        // Record in audit log
        $this->recordAuditEntry($workflowId, 'workflow_started', null, [
            'template' => $template['name'],
            'firstStage' => $template['stages'][0]['name'],
        ]);

        return $this->getWorkflow($documentId);
    }

    /**
     * Submit document for review
     */
    public function submitForReview(string $documentId, int $userId, ?string $message = null): array
    {
        $workflow = $this->getWorkflow($documentId);

        if ($workflow['status'] !== 'active') {
            throw new \InvalidArgumentException('Workflow is not active');
        }

        $currentStage = $this->getCurrentStage($workflow);

        // Check if user can submit
        if (!$this->canUserSubmit($currentStage, $userId)) {
            throw new \InvalidArgumentException('User cannot submit at this stage');
        }

        // Move to next stage
        return $this->transitionToNextStage($workflow, $userId, 'submit', $message);
    }

    /**
     * Approve current stage
     */
    public function approve(string $documentId, int $userId, ?string $comment = null): array
    {
        $workflow = $this->getWorkflow($documentId);

        if ($workflow['status'] !== 'active') {
            throw new \InvalidArgumentException('Workflow is not active');
        }

        $currentStage = $this->getCurrentStage($workflow);

        // Check if user can approve
        if (!$this->canUserApprove($currentStage, $userId)) {
            throw new \InvalidArgumentException('User is not authorized to approve at this stage');
        }

        // Record approval
        $this->recordApproval($workflow['id'], $currentStage['id'], $userId, true, $comment);

        // Check if stage is complete (all required approvals received)
        if ($this->isStageComplete($workflow['id'], $currentStage)) {
            return $this->transitionToNextStage($workflow, $userId, 'approve', $comment);
        }

        return $this->getWorkflow($documentId);
    }

    /**
     * Reject current stage
     */
    public function reject(string $documentId, int $userId, string $reason): array
    {
        $workflow = $this->getWorkflow($documentId);

        if ($workflow['status'] !== 'active') {
            throw new \InvalidArgumentException('Workflow is not active');
        }

        $currentStage = $this->getCurrentStage($workflow);

        // Check if user can reject
        if (!$this->canUserApprove($currentStage, $userId)) {
            throw new \InvalidArgumentException('User is not authorized to reject at this stage');
        }

        // Record rejection
        $this->recordApproval($workflow['id'], $currentStage['id'], $userId, false, $reason);

        // Handle rejection based on template configuration
        $template = $this->getWorkflowTemplate($workflow['templateId']);
        $rejectionAction = $template['onRejection'] ?? 'return_to_author';

        return match ($rejectionAction) {
            'return_to_author' => $this->returnToAuthor($workflow, $userId, $reason),
            'return_to_previous' => $this->returnToPreviousStage($workflow, $userId, $reason),
            'cancel' => $this->cancelWorkflow($documentId, $userId, $reason),
            default => $this->returnToAuthor($workflow, $userId, $reason),
        };
    }

    /**
     * Request changes
     */
    public function requestChanges(string $documentId, int $userId, array $changes): array
    {
        $workflow = $this->getWorkflow($documentId);

        $this->recordAuditEntry($workflow['id'], 'changes_requested', $userId, [
            'changes' => $changes,
        ]);

        // Create change request record
        DB::table('workflow_change_requests')->insert([
            'id' => Str::uuid()->toString(),
            'workflow_id' => $workflow['id'],
            'stage_id' => $workflow['currentStage'],
            'requested_by' => $userId,
            'changes' => json_encode($changes),
            'status' => 'pending',
            'created_at' => now(),
        ]);

        // Return to author
        return $this->returnToAuthor($workflow, $userId, 'Changes requested');
    }

    /**
     * Get pending approvals for a user
     */
    public function getPendingApprovals(int $userId): array
    {
        $userRoles = $this->getUserRoles($userId);

        $pending = DB::table('content_workflows')
            ->join('workflow_stages', 'content_workflows.current_stage', '=', 'workflow_stages.id')
            ->where('content_workflows.status', 'active')
            ->where(function ($query) use ($userId, $userRoles) {
                $query->whereJsonContains('workflow_stages.approvers', ['userId' => $userId])
                    ->orWhere(function ($q) use ($userRoles) {
                        foreach ($userRoles as $role) {
                            $q->orWhereJsonContains('workflow_stages.approvers', ['role' => $role]);
                        }
                    });
            })
            ->select('content_workflows.*', 'workflow_stages.name as stage_name')
            ->get();

        return $pending->map(fn($w) => $this->formatWorkflow($w))->toArray();
    }

    /**
     * Get workflow history/audit trail
     */
    public function getHistory(string $documentId): array
    {
        $workflow = $this->getWorkflow($documentId);

        $history = DB::table('workflow_audit_log')
            ->where('workflow_id', $workflow['id'])
            ->orderBy('created_at', 'desc')
            ->get();

        return $history->map(fn($entry) => [
            'id' => $entry->id,
            'action' => $entry->action,
            'userId' => $entry->user_id,
            'metadata' => json_decode($entry->metadata, true),
            'createdAt' => $entry->created_at,
        ])->toArray();
    }

    /**
     * Cancel a workflow
     */
    public function cancelWorkflow(string $documentId, int $userId, ?string $reason = null): array
    {
        $workflow = $this->getWorkflow($documentId);

        DB::table('content_workflows')
            ->where('id', $workflow['id'])
            ->update([
                'status' => 'cancelled',
                'completed_at' => now(),
                'updated_at' => now(),
            ]);

        $this->recordAuditEntry($workflow['id'], 'workflow_cancelled', $userId, [
            'reason' => $reason,
        ]);

        return $this->getWorkflow($documentId);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // WORKFLOW TEMPLATE MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Create a workflow template
     */
    public function createTemplate(array $template): array
    {
        $templateId = Str::uuid()->toString();

        DB::table('workflow_templates')->insert([
            'id' => $templateId,
            'name' => $template['name'],
            'description' => $template['description'] ?? null,
            'document_type' => $template['documentType'] ?? 'post',
            'stages' => json_encode($template['stages']),
            'on_rejection' => $template['onRejection'] ?? 'return_to_author',
            'on_completion' => $template['onCompletion'] ?? 'publish',
            'is_default' => $template['isDefault'] ?? false,
            'is_active' => true,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        // Store stages
        foreach ($template['stages'] as $index => $stage) {
            $stageId = Str::uuid()->toString();

            DB::table('workflow_stages')->insert([
                'id' => $stageId,
                'template_id' => $templateId,
                'name' => $stage['name'],
                'description' => $stage['description'] ?? null,
                'order' => $index,
                'approvers' => json_encode($stage['approvers'] ?? []),
                'approval_type' => $stage['approvalType'] ?? 'any', // any, all, threshold
                'approval_threshold' => $stage['approvalThreshold'] ?? 1,
                'deadline_hours' => $stage['deadlineHours'] ?? null,
                'auto_approve' => $stage['autoApprove'] ?? false,
                'notifications' => json_encode($stage['notifications'] ?? []),
                'created_at' => now(),
            ]);
        }

        Cache::forget('workflow_templates');

        return $this->getWorkflowTemplate($templateId);
    }

    /**
     * Get workflow template
     */
    public function getWorkflowTemplate(string $templateId): array
    {
        $template = DB::table('workflow_templates')->find($templateId);

        if (!$template) {
            throw new \InvalidArgumentException("Template not found: {$templateId}");
        }

        $stages = DB::table('workflow_stages')
            ->where('template_id', $templateId)
            ->orderBy('order')
            ->get();

        return [
            'id' => $template->id,
            'name' => $template->name,
            'description' => $template->description,
            'documentType' => $template->document_type,
            'stages' => $stages->map(fn($s) => $this->formatStage($s))->toArray(),
            'onRejection' => $template->on_rejection,
            'onCompletion' => $template->on_completion,
            'isDefault' => (bool) $template->is_default,
            'isActive' => (bool) $template->is_active,
        ];
    }

    /**
     * List workflow templates
     */
    public function listTemplates(?string $documentType = null): array
    {
        $query = DB::table('workflow_templates')->where('is_active', true);

        if ($documentType) {
            $query->where('document_type', $documentType);
        }

        return $query->get()->map(fn($t) => [
            'id' => $t->id,
            'name' => $t->name,
            'description' => $t->description,
            'documentType' => $t->document_type,
            'isDefault' => (bool) $t->is_default,
        ])->toArray();
    }

    /**
     * Get default template for document type
     */
    public function getDefaultTemplate(string $documentType): array
    {
        $template = DB::table('workflow_templates')
            ->where('document_type', $documentType)
            ->where('is_default', true)
            ->where('is_active', true)
            ->first();

        if ($template) {
            return $this->getWorkflowTemplate($template->id);
        }

        // Create basic default template
        return $this->createTemplate([
            'name' => "Default {$documentType} Workflow",
            'documentType' => $documentType,
            'isDefault' => true,
            'stages' => [
                [
                    'name' => 'Draft',
                    'description' => 'Initial draft stage',
                    'approvers' => [],
                    'approvalType' => 'any',
                ],
                [
                    'name' => 'Review',
                    'description' => 'Editorial review',
                    'approvers' => [['role' => 'editor']],
                    'approvalType' => 'any',
                    'deadlineHours' => 48,
                ],
                [
                    'name' => 'Approved',
                    'description' => 'Ready for publication',
                    'approvers' => [['role' => 'admin']],
                    'approvalType' => 'any',
                ],
            ],
            'onRejection' => 'return_to_author',
            'onCompletion' => 'publish',
        ]);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function formatWorkflow(object $workflow): array
    {
        $template = $this->getWorkflowTemplate($workflow->template_id);
        $currentStage = collect($template['stages'])->firstWhere('id', $workflow->current_stage);

        return [
            'id' => $workflow->id,
            'documentId' => $workflow->document_id,
            'documentType' => $workflow->document_type,
            'templateId' => $workflow->template_id,
            'templateName' => $template['name'],
            'currentStage' => $workflow->current_stage,
            'currentStageName' => $currentStage['name'] ?? 'Unknown',
            'status' => $workflow->status,
            'startedAt' => $workflow->started_at,
            'completedAt' => $workflow->completed_at,
            'stages' => $template['stages'],
            'progress' => $this->calculateProgress($workflow, $template),
        ];
    }

    private function formatStage(object $stage): array
    {
        return [
            'id' => $stage->id,
            'name' => $stage->name,
            'description' => $stage->description,
            'order' => $stage->order,
            'approvers' => json_decode($stage->approvers, true),
            'approvalType' => $stage->approval_type,
            'approvalThreshold' => $stage->approval_threshold,
            'deadlineHours' => $stage->deadline_hours,
            'autoApprove' => (bool) $stage->auto_approve,
            'notifications' => json_decode($stage->notifications, true),
        ];
    }

    private function getCurrentStage(array $workflow): array
    {
        return collect($workflow['stages'])->firstWhere('id', $workflow['currentStage']) ?? [];
    }

    private function initializeStage(string $workflowId, array $stage, string $documentId): void
    {
        // Check for auto-approve
        if ($stage['autoApprove'] ?? false) {
            $this->transitionToNextStage(
                $this->getWorkflowByWorkflowId($workflowId),
                0,
                'auto_approve',
                'Automatically approved'
            );
            return;
        }

        // Set deadline if configured
        if ($stage['deadlineHours'] ?? null) {
            $deadline = now()->addHours($stage['deadlineHours']);

            DB::table('workflow_deadlines')->insert([
                'id' => Str::uuid()->toString(),
                'workflow_id' => $workflowId,
                'stage_id' => $stage['id'],
                'deadline' => $deadline,
                'status' => 'pending',
                'created_at' => now(),
            ]);
        }

        // Send notifications
        $this->sendStageNotifications($workflowId, $stage, 'stage_entered');
    }

    private function getWorkflowByWorkflowId(string $workflowId): array
    {
        $workflow = DB::table('content_workflows')->find($workflowId);
        return $this->formatWorkflow($workflow);
    }

    private function canUserSubmit(array $stage, int $userId): bool
    {
        // By default, the author can always submit from draft stage
        return $stage['order'] === 0;
    }

    private function canUserApprove(array $stage, int $userId): bool
    {
        $approvers = $stage['approvers'] ?? [];
        $userRoles = $this->getUserRoles($userId);

        foreach ($approvers as $approver) {
            if (isset($approver['userId']) && $approver['userId'] === $userId) {
                return true;
            }

            if (isset($approver['role']) && in_array($approver['role'], $userRoles, true)) {
                return true;
            }
        }

        return false;
    }

    private function getUserRoles(int $userId): array
    {
        // Get user roles from database or auth system
        $roles = DB::table('model_has_roles')
            ->join('roles', 'model_has_roles.role_id', '=', 'roles.id')
            ->where('model_has_roles.model_id', $userId)
            ->where('model_has_roles.model_type', 'App\\Models\\User')
            ->pluck('roles.name')
            ->toArray();

        return $roles;
    }

    private function recordApproval(string $workflowId, string $stageId, int $userId, bool $approved, ?string $comment): void
    {
        DB::table('workflow_approvals')->insert([
            'id' => Str::uuid()->toString(),
            'workflow_id' => $workflowId,
            'stage_id' => $stageId,
            'user_id' => $userId,
            'approved' => $approved,
            'comment' => $comment,
            'created_at' => now(),
        ]);

        $this->recordAuditEntry($workflowId, $approved ? 'approved' : 'rejected', $userId, [
            'stageId' => $stageId,
            'comment' => $comment,
        ]);
    }

    private function isStageComplete(string $workflowId, array $stage): bool
    {
        $approvals = DB::table('workflow_approvals')
            ->where('workflow_id', $workflowId)
            ->where('stage_id', $stage['id'])
            ->where('approved', true)
            ->count();

        return match ($stage['approvalType']) {
            'any' => $approvals >= 1,
            'all' => $approvals >= count($stage['approvers']),
            'threshold' => $approvals >= ($stage['approvalThreshold'] ?? 1),
            default => $approvals >= 1,
        };
    }

    private function transitionToNextStage(array $workflow, int $userId, string $action, ?string $comment): array
    {
        $currentIndex = collect($workflow['stages'])->search(fn($s) => $s['id'] === $workflow['currentStage']);
        $nextIndex = $currentIndex + 1;

        if ($nextIndex >= count($workflow['stages'])) {
            // Workflow complete
            return $this->completeWorkflow($workflow, $userId);
        }

        $nextStage = $workflow['stages'][$nextIndex];

        DB::table('content_workflows')
            ->where('id', $workflow['id'])
            ->update([
                'current_stage' => $nextStage['id'],
                'updated_at' => now(),
            ]);

        $this->recordAuditEntry($workflow['id'], 'stage_transition', $userId, [
            'from' => $workflow['currentStageName'],
            'to' => $nextStage['name'],
            'action' => $action,
            'comment' => $comment,
        ]);

        // Initialize next stage
        $this->initializeStage($workflow['id'], $nextStage, $workflow['documentId']);

        return $this->getWorkflow($workflow['documentId']);
    }

    private function returnToAuthor(array $workflow, int $userId, string $reason): array
    {
        $firstStage = $workflow['stages'][0];

        DB::table('content_workflows')
            ->where('id', $workflow['id'])
            ->update([
                'current_stage' => $firstStage['id'],
                'updated_at' => now(),
            ]);

        $this->recordAuditEntry($workflow['id'], 'returned_to_author', $userId, [
            'reason' => $reason,
        ]);

        return $this->getWorkflow($workflow['documentId']);
    }

    private function returnToPreviousStage(array $workflow, int $userId, string $reason): array
    {
        $currentIndex = collect($workflow['stages'])->search(fn($s) => $s['id'] === $workflow['currentStage']);
        $prevIndex = max(0, $currentIndex - 1);
        $prevStage = $workflow['stages'][$prevIndex];

        DB::table('content_workflows')
            ->where('id', $workflow['id'])
            ->update([
                'current_stage' => $prevStage['id'],
                'updated_at' => now(),
            ]);

        $this->recordAuditEntry($workflow['id'], 'returned_to_previous', $userId, [
            'reason' => $reason,
            'stage' => $prevStage['name'],
        ]);

        return $this->getWorkflow($workflow['documentId']);
    }

    private function completeWorkflow(array $workflow, int $userId): array
    {
        $template = $this->getWorkflowTemplate($workflow['templateId']);

        DB::table('content_workflows')
            ->where('id', $workflow['id'])
            ->update([
                'status' => 'completed',
                'completed_at' => now(),
                'updated_at' => now(),
            ]);

        $this->recordAuditEntry($workflow['id'], 'workflow_completed', $userId, []);

        // Handle completion action
        $completionAction = $template['onCompletion'] ?? 'publish';

        if ($completionAction === 'publish') {
            // Trigger publish
            $this->triggerPublish($workflow['documentId']);
        }

        return $this->getWorkflow($workflow['documentId']);
    }

    private function triggerPublish(string $documentId): void
    {
        // Publish the document
        DB::table('posts')
            ->where('id', $documentId)
            ->update([
                'status' => 'published',
                'published_at' => now(),
                'updated_at' => now(),
            ]);
    }

    private function calculateProgress(object $workflow, array $template): float
    {
        $currentIndex = collect($template['stages'])->search(fn($s) => $s['id'] === $workflow->current_stage);
        return ($currentIndex + 1) / count($template['stages']);
    }

    private function recordAuditEntry(string $workflowId, string $action, ?int $userId, array $metadata): void
    {
        DB::table('workflow_audit_log')->insert([
            'id' => Str::uuid()->toString(),
            'workflow_id' => $workflowId,
            'action' => $action,
            'user_id' => $userId,
            'metadata' => json_encode($metadata),
            'created_at' => now(),
        ]);
    }

    private function sendStageNotifications(string $workflowId, array $stage, string $event): void
    {
        // Implementation for sending notifications
        // Would integrate with notification system
    }
}
